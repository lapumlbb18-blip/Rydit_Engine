//! demo_gpu_instancing.rs — v0.15.0 FIX FINAL
//! SDL2 puro + OpenGL directo (sin Canvas, sin raylib)
//! Mismo patrón que demo_50k_particulas pero con GPU instancing
//!
//! USO: cargo run --bin demo_gpu_instancing --release

use ry_gfx::gpu_instancing::{GPUInstancer, ParticleData};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use std::time::Instant;

const GPU_INST_VS: &str = include_str!("../../../ry-gfx/shaders/vertex.glsl");
const GPU_INST_FS: &str = include_str!("../../../ry-gfx/shaders/fragment.glsl");

fn main() {
    println!("🛡️ GPU Instancing — SDL2+OpenGL puro (sin Canvas)\n");

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let ga = video.gl_attr();
    ga.set_context_profile(GLProfile::Core);
    ga.set_context_version(3, 3);

    let win = video
        .window("GPU Instancing — RyDit v0.15.0", 1280, 720)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let ctx = win.gl_create_context().unwrap();
    win.gl_make_current(&ctx).unwrap();

    // Cargar GL con SDL2 (NO con raylib, NO con glfw)
    gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

    unsafe {
        let v = gl::GetString(gl::VERSION);
        let r = gl::GetString(gl::RENDERER);
        println!("[GL] {} / {}",
            std::ffi::CStr::from_ptr(v as *const _).to_string_lossy(),
            std::ffi::CStr::from_ptr(r as *const _).to_string_lossy());

        // Viewport explícito
        gl::Viewport(0, 0, 1280, 720);
    }

    // GPU Instancer
    let mut inst = GPUInstancer::new();
    inst.set_resolution(1280.0, 720.0);

    // Shaders temporales (GPUInstancer::load_shaders lee de disco)
    let vs_p = "/data/data/com.termux/files/usr/tmp/gpu_inst_vs.glsl";
    let fs_p = "/data/data/com.termux/files/usr/tmp/gpu_inst_fs.glsl";
    std::fs::write(vs_p, GPU_INST_VS).unwrap();
    std::fs::write(fs_p, GPU_INST_FS).unwrap();
    inst.load_shaders(vs_p, fs_p).unwrap();
    println!("[GPU] Instancer + shaders OK");

    let w = 1280.0f32;
    let h = 720.0f32;
    let cols = 100u32;
    let mut count = 50000usize;

    let mut particles = Vec::with_capacity(250000);
    let mut vx_buf = Vec::with_capacity(250000);
    let mut vy_buf = Vec::with_capacity(250000);
    fill_particles(&mut particles, &mut vx_buf, &mut vy_buf, count, w, h, 5.0);
    inst.set_particles(&particles);
    println!("[GPU] {} partículas animadas tipo estrellas\n", count);

    let mut ev = sdl.event_pump().unwrap();
    let mut frame = 0u64;
    let mut fps_t = Instant::now();
    let mut fps = 0u32;
    let mut fps_c = 0u32;

    println!("1-5: 50K-250K | ESC: salir\n");

    'run: loop {
        for e in ev.poll_iter() {
            match e {
                Event::Quit { .. } => break 'run,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'run,
                Event::KeyDown { keycode: Some(k), repeat: false, .. } => match k {
                    Keycode::Num1 => { count = 50000; fill_particles(&mut particles, &mut vx_buf, &mut vy_buf, count, w, h, 5.0); inst.set_particles(&particles); println!("50K"); }
                    Keycode::Num2 => { count = 100000; fill_particles(&mut particles, &mut vx_buf, &mut vy_buf, count, w, h, 5.0); inst.set_particles(&particles); println!("100K"); }
                    Keycode::Num3 => { count = 150000; fill_particles(&mut particles, &mut vx_buf, &mut vy_buf, count, w, h, 5.0); inst.set_particles(&particles); println!("150K"); }
                    Keycode::Num4 => { count = 200000; fill_particles(&mut particles, &mut vx_buf, &mut vy_buf, count, w, h, 5.0); inst.set_particles(&particles); println!("200K"); }
                    Keycode::Num5 => { count = 250000; fill_particles(&mut particles, &mut vx_buf, &mut vy_buf, count, w, h, 5.0); inst.set_particles(&particles); println!("250K"); }
                    _ => {}
                },
                _ => {}
            }
        }

        // Animación tipo estrellas cayendo (como menu torreta)
        update_particles(&mut particles, &vx_buf, &vy_buf, w, h);
        inst.set_particles(&particles);

        unsafe {
            gl::ClearColor(0.05, 0.05, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Disable(gl::DEPTH_TEST);
            gl::Disable(gl::SCISSOR_TEST);
            gl::Viewport(0, 0, 1280, 720);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        inst.draw();
        win.gl_swap_window();

        frame += 1;
        fps_c += 1;
        if fps_t.elapsed().as_secs() >= 1 {
            fps = fps_c; fps_c = 0; fps_t = Instant::now();
        }
        if frame % 120 == 0 {
            println!("[GPU] FPS: {} | {} pts", fps, count);
        }
    }
    println!("\n✅ {} frames — {} FPS", frame, fps);
}

fn fill_particles(
    buf: &mut Vec<ParticleData>,
    vx_buf: &mut Vec<f32>,
    vy_buf: &mut Vec<f32>,
    n: usize,
    w: f32,
    h: f32,
    sz: f32,
) {
    buf.clear();
    vx_buf.clear();
    vy_buf.clear();
    buf.reserve(n);
    vx_buf.reserve(n);
    vy_buf.reserve(n);
    for i in 0..n {
        // Posición pseudo-aleatoria tipo estrella
        let sx = ((i.wrapping_mul(137)) % (w as usize)) as f32;
        let sy = ((i.wrapping_mul(251) + i / 10) % (h as usize)) as f32;
        let star_type = i % 4;
        let (rr, gg, bb) = match star_type {
            0 => (1.0, 1.0, 1.0),
            1 => (0.6, 0.7, 1.0),
            2 => (1.0, 0.9, 0.5),
            _ => (0.8, 0.6, 1.0),
        };
        let s = sz * (0.5 + (i % 3) as f32 / 3.0);
        // Velocidad tipo estrella del menu torreta
        let vx = (i % 7) as f32 * 0.3 - 1.0;
        let vy = 0.5 + (i % 5) as f32 * 0.2;
        buf.push(ParticleData::new(sx, sy, s, rr, gg, bb, 1.0));
        vx_buf.push(vx);
        vy_buf.push(vy);
    }
}

fn update_particles(buf: &mut [ParticleData], vx: &[f32], vy: &[f32], w: f32, h: f32) {
    for (i, p) in buf.iter_mut().enumerate() {
        if i >= vx.len() { break; }
        p.offset[0] += vx[i];
        p.offset[1] += vy[i];
        if p.offset[1] > h + 10.0 {
            p.offset[1] = -10.0;
            p.offset[0] = ((i * 137) % (w as usize)) as f32;
        }
        if p.offset[0] < -10.0 { p.offset[0] = w + 10.0; }
        if p.offset[0] > w + 10.0 { p.offset[0] = -10.0; }
    }
}
