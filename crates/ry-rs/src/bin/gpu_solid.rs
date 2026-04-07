//! gpu_solid.rs — Test con quads SÓLIDOS (sin círculo)
//! Si ves 9 cuadros = VAO/instanciado OK. El bug está en el fragment shader de círculo.

use ry_gfx::gpu_instancing::{GPUInstancer, ParticleData};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use std::time::Instant;
use std::ffi::CString;
use gl::types::GLuint;

const GPU_INST_VS: &str = include_str!("../../../ry-gfx/shaders/vertex.glsl");

// Fragment shader SÓLIDO — sin discard de círculo
const SOLID_FS: &str = r#"#version 330 core
in vec4 vColor;
in vec2 vLocalPos;
out vec4 fragColor;
void main(){
    // CUAD SÓLIDO SIN DESCART — si esto no se ve, el bug es VAO/instanciado
    fragColor = vColor;
}"#;

fn compile_shader(stype: GLuint, src: &str) -> GLuint {
    unsafe {
        let s = gl::CreateShader(stype);
        let c = CString::new(src).unwrap();
        gl::ShaderSource(s, 1, &c.as_ptr(), std::ptr::null());
        gl::CompileShader(s);
        let mut ok = 0;
        gl::GetShaderiv(s, gl::COMPILE_STATUS, &mut ok);
        if ok == 0 {
            let mut len = 0;
            gl::GetShaderiv(s, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0u8; len as usize];
            gl::GetShaderInfoLog(s, len, &mut len, buf.as_mut_ptr() as *mut _);
            eprintln!("[SHADER ERR] {}", String::from_utf8_lossy(&buf));
        }
        s
    }
}

fn main() {
    println!("[SOLID TEST] 9 quads sólidos — sin círculo\n");

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let ga = video.gl_attr();
    ga.set_context_profile(GLProfile::Core);
    ga.set_context_version(3, 3);

    let win = video.window("GPU SOLID TEST", 1280, 720)
        .position_centered().opengl().build().unwrap();
    let ctx = win.gl_create_context().unwrap();
    win.gl_make_current(&ctx).unwrap();
    gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

    unsafe {
        let v = gl::GetString(gl::VERSION);
        let r = gl::GetString(gl::RENDERER);
        println!("[GL] {} / {}",
            std::ffi::CStr::from_ptr(v as *const _).to_string_lossy(),
            std::ffi::CStr::from_ptr(r as *const _).to_string_lossy());
    }

    // Manual instancer con shader sólido
    let vs_p = "/data/data/com.termux/files/usr/tmp/gpu_inst_vs_test.glsl";
    let fs_p = "/data/data/com.termux/files/usr/tmp/gpu_inst_fs_solid.glsl";
    std::fs::write(vs_p, GPU_INST_VS).unwrap();
    std::fs::write(fs_p, SOLID_FS).unwrap();

    let mut inst = GPUInstancer::new();
    inst.set_resolution(1280.0, 720.0);
    inst.load_shaders(vs_p, fs_p).unwrap();
    println!("[GPU] Shader sólido cargado");

    // 9 quads GIGANTES en posiciones conocidas
    let w = 1280.0f32;
    let h = 720.0f32;
    let positions = [
        (0.0, 0.0), (w/2.0, 0.0), (w-100.0, 0.0),
        (0.0, h/2.0), (w/2.0, h/2.0), (w-100.0, h/2.0),
        (0.0, h-100.0), (w/2.0, h-100.0), (w-100.0, h-100.0),
    ];
    let colors = [
        (1.0,0.0,0.0),(0.0,1.0,0.0),(0.0,0.0,1.0),
        (1.0,1.0,0.0),(1.0,1.0,1.0),(1.0,0.0,1.0),
        (0.0,1.0,1.0),(1.0,0.5,0.0),(0.5,0.0,1.0),
    ];

    let mut particles = Vec::new();
    for i in 0..9 {
        let (px, py) = positions[i];
        let (r, g, b) = colors[i];
        // QUAD de 100px — debe verse SIN problema
        particles.push(ParticleData::new(px, py, 100.0, r, g, b, 1.0));
        println!("[{}] ({:.0},{:.0}) size=100 color=({},{},{})", i, px, py, r, g, b);
    }

    inst.set_particles(&particles);

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
            gl::ClearColor(0.15, 0.0, 0.0, 1.0); // ROJO OSCURO de fondo
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        inst.draw();
        win.gl_swap_window();

        frame += 1;
        if frame == 1 || frame == 60 {
            println!("[FRAME {}] draw() ejecutado", frame);
        }
    }
    println!("\n✅ {} frames", frame);
}
