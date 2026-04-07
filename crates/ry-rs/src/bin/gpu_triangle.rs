//! gpu_triangle.rs — Test mínimo: 1 triángulo rojo SIN instancing
//! Si ves un triángulo rojo = OpenGL funciona. 
//! Si no = problema de configuración de contexto/viewport.

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use std::time::Instant;

fn main() {
    println!("[TRIANGLE TEST] 1 triángulo rojo en el centro\n");

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let ga = video.gl_attr();
    ga.set_context_profile(GLProfile::Core);
    ga.set_context_version(3, 3);

    let win = video.window("Triangle Test", 1280, 720)
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

        // Viewport explícito
        gl::Viewport(0, 0, 1280, 720);
    }

    // Shader simple
    let vs_src = r#"#version 330 core
    layout(location=0) in vec2 aPos;
    void main(){
        gl_Position = vec4(aPos, 0.0, 1.0);
    }"#;
    let fs_src = r#"#version 330 core
    out vec4 fc;
    void main(){
        fc = vec4(1.0, 0.0, 0.0, 1.0);
    }"#;

    unsafe {
        let vs = gl::CreateShader(gl::VERTEX_SHADER);
        let cs = std::ffi::CString::new(vs_src).unwrap();
        gl::ShaderSource(vs, 1, &cs.as_ptr(), std::ptr::null());
        gl::CompileShader(vs);

        let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
        let cf = std::ffi::CString::new(fs_src).unwrap();
        gl::ShaderSource(fs, 1, &cf.as_ptr(), std::ptr::null());
        gl::CompileShader(fs);

        let prog = gl::CreateProgram();
        gl::AttachShader(prog, vs);
        gl::AttachShader(prog, fs);
        gl::LinkProgram(prog);

        let mut ok = 0;
        gl::GetProgramiv(prog, gl::LINK_STATUS, &mut ok);
        if ok == 0 {
            let mut len = 0;
            gl::GetProgramiv(prog, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0u8; len as usize];
            gl::GetProgramInfoLog(prog, len, &mut len, buf.as_mut_ptr() as *mut _);
            eprintln!("[LINK ERR] {}", String::from_utf8_lossy(&buf));
        }

        gl::UseProgram(prog);

        // VAO + VBO con triángulo grande (NDC -0.5 a 0.5)
        let mut vao = 0;
        let mut vbo = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Triángulo grande en NDC: cubre buena parte de la pantalla
        let tri: [f32; 6] = [
            0.0, 0.5,    // arriba centro
            -0.5, -0.5,  // abajo izq
            0.5, -0.5,   // abajo der
        ];
        gl::BufferData(gl::ARRAY_BUFFER, (tri.len() * 4) as isize,
            tri.as_ptr() as *const _, gl::STATIC_DRAW);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::BindVertexArray(0);

        println!("[GL] Triángulo NDC creado");

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

            // Fondo VERDE OSCURO (para distinguir del rojo del triángulo)
            gl::ClearColor(0.0, 0.15, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(prog);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            let err = gl::GetError();
            if err != gl::NO_ERROR && frame < 5 {
                eprintln!("[GL ERROR] 0x{:X}", err);
            }

            win.gl_swap_window();
            frame += 1;
            if frame == 1 || frame == 60 {
                println!("[FRAME {}] renderizado", frame);
            }
        }
        println!("\n✅ {} frames", frame);
    }
}
