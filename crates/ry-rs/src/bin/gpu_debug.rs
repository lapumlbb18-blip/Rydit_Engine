//! gpu_debug.rs — 9 partículas GIGANTES para diagnosticar pantalla negra
//! Si ves 9 cuadros blancos = shaders OK. Si no = bug en shader.

use ry_gfx::gpu_instancing::{GPUInstancer, ParticleData};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use std::time::Instant;

const GPU_INST_VS: &str = include_str!("../../../ry-gfx/shaders/vertex.glsl");
const GPU_INST_FS: &str = include_str!("../../../ry-gfx/shaders/fragment.glsl");

fn main() {
    println!("[DEBUG] 9 partículas GIGANTES (50px) blancas en grid 3x3\n");

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video
        .window("GPU DEBUG — 9 partículas", 1280, 720)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let ctx = window.gl_create_context().unwrap();
    window.gl_make_current(&ctx).unwrap();
    gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

    unsafe {
        let v = gl::GetString(gl::VERSION);
        let r = gl::GetString(gl::RENDERER);
        println!("[GL] {}", std::ffi::CStr::from_ptr(v as *const _).to_string_lossy());
        println!("[GL] {}", std::ffi::CStr::from_ptr(r as *const _).to_string_lossy());
    }

    let mut inst = GPUInstancer::new();
    inst.set_resolution(1280.0, 720.0);

    let vs_p = "/data/data/com.termux/files/usr/tmp/gpu_inst_vs.glsl";
    let fs_p = "/data/data/com.termux/files/usr/tmp/gpu_inst_fs.glsl";
    std::fs::write(vs_p, GPU_INST_VS).unwrap();
    std::fs::write(fs_p, GPU_INST_FS).unwrap();
    inst.load_shaders(vs_p, fs_p).unwrap();

    // 9 partículas GIGANTES en grid 3x3
    let w = 1280.0f32;
    let h = 720.0f32;
    let positions = [
        (0.0, 0.0), (w/2.0, 0.0), (w-1.0, 0.0),
        (0.0, h/2.0), (w/2.0, h/2.0), (w-1.0, h/2.0),
        (0.0, h-1.0), (w/2.0, h-1.0), (w-1.0, h-1.0),
    ];

    let mut particles = Vec::new();
    for (i, (px, py)) in positions.iter().enumerate() {
        let colors = [
            (1.0, 0.0, 0.0), // rojo - esquina sup izq
            (0.0, 1.0, 0.0), // verde - centro arriba
            (0.0, 0.0, 1.0), // azul - esquina sup der
            (1.0, 1.0, 0.0), // amarillo - izq centro
            (1.0, 1.0, 1.0), // BLANCO - CENTRO
            (1.0, 0.0, 1.0), // magenta - der centro
            (0.0, 1.0, 1.0), // cyan - esquina inf izq
            (1.0, 0.5, 0.0), // naranja - centro abajo
            (0.5, 0.0, 1.0), // morado - esquina inf der
        ];
        let (r, g, b) = colors[i];
        println!("[{}] pos=({:.0}, {:.0}) color=({}, {}, {}) size=50", i, px, py, r, g, b);
        particles.push(ParticleData::new(*px, *py, 50.0, r, g, b, 1.0));
    }

    inst.set_particles(&particles);
    println!("\n[GPU] 9 partículas de 50px cargadas");
    println!("Si ves colores = shaders OK. Si pantalla negra = bug shader.\n");

    let mut ev = sdl.event_pump().unwrap();
    let mut frame = 0u64;

    'run: loop {
        for e in ev.poll_iter() {
            match e {
                Event::Quit { .. } => break 'run,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'run,
                _ => {}
            }
        }

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        inst.draw();
        window.gl_swap_window();

        frame += 1;
        if frame == 1 {
            println!("[FRAME 1] draw() ejecutado");
        }
        if frame == 60 {
            println!("[FRAME 60] 60 frames renderizados");
        }
    }
    println!("\n✅ {} frames", frame);
}
