//! demo_fsr.rs
//! Demo FSR 1.0 - FidelityFX Super Resolution con pipeline render-to-texture
//!
//! ✅ SDL2 + OpenGL 3.3 Core + FBO render-to-texture
//! ✅ FSR EASU upscale + RCAS sharpen
//! ✅ Toggle manual (tecla F) + Auto-detect por FPS
//! ✅ Visual: grid + formas de test para ver calidad de upscale
//!
//! Uso: cargo run --bin demo_fsr --release

use gl::types::GLuint;
use ry_gfx::fsr::{FboFrame, FsrQuality, FsrUpscaler};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use std::ffi::CString;
use std::time::Instant;

// Colores
const BG_COLOR: (f32, f32, f32) = (0.02, 0.02, 0.04);
const GRID_COLOR: (f32, f32, f32) = (0.1, 0.1, 0.3);
const RED: (f32, f32, f32) = (0.9, 0.16, 0.22);
const YELLOW: (f32, f32, f32) = (0.99, 0.98, 0.0);
const WHITE_C: (f32, f32, f32) = (1.0, 1.0, 1.0);
const GREEN_C: (f32, f32, f32) = (0.46, 0.8, 0.39);
const CYAN_C: (f32, f32, f32) = (0.0, 1.0, 1.0);

// Shader simple para dibujar formas 2D
const SIMPLE_VS: &str = r#"#version 330 core
layout(location=0) in vec2 aPos;
layout(location=1) in vec3 aColor;
uniform vec2 uResolution;
out vec3 vColor;
void main(){
    vec2 ndc = (aPos / uResolution) * 2.0 - 1.0;
    gl_Position = vec4(ndc.x, -ndc.y, 0.0, 1.0);
    vColor = aColor;
}"#;

const SIMPLE_FS: &str = r#"#version 330 core
in vec3 vColor;
out vec4 fragColor;
void main(){
    fragColor = vec4(vColor, 1.0);
}"#;

struct SimpleShader {
    program: GLuint,
    resolution_loc: i32,
    vao: GLuint,
    vbo: GLuint,
}

impl SimpleShader {
    fn new() -> Self {
        unsafe {
            let vs = compile_shader(gl::VERTEX_SHADER, SIMPLE_VS);
            let fs = compile_shader(gl::FRAGMENT_SHADER, SIMPLE_FS);
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);

            gl::UseProgram(program);
            let resolution_loc =
                gl::GetUniformLocation(program, b"uResolution\0".as_ptr() as *const _);

            let mut vao = 0;
            let mut vbo = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 20, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                20,
                (8 * 4) as *const _,
            );
            gl::EnableVertexAttribArray(1);
            gl::BindVertexArray(0);
            gl::UseProgram(0);

            Self {
                program,
                resolution_loc,
                vao,
                vbo,
            }
        }
    }

    fn draw(&self, vertices: &[(f32, f32, f32, f32, f32)], w: f32, h: f32) {
        if vertices.is_empty() {
            return;
        }
        unsafe {
            gl::UseProgram(self.program);
            gl::Uniform2f(self.resolution_loc, w, h);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * 20) as isize,
                vertices.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );
            gl::DrawArrays(gl::TRIANGLES, 0, vertices.len() as i32);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }
}

fn main() {
    println!("🛡️ RyDit v0.15.0 - Demo FSR 1.0");
    println!("========================================");
    println!("🎮 SDL2 + OpenGL 3.3 + FBO + FSR EASU+RCAS");
    println!("🪟 Ventana: 1280x720 (output nativo)");
    println!("🔍 Render interno: escalado según calidad FSR");
    println!("========================================\n");

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_double_buffer(true);

    let window = video
        .window("FSR 1.0 Demo - RyDit v0.15.0", 1280, 720)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();
    gl::load_with(|name| video.gl_get_proc_address(name) as *const _);

    unsafe {
        let version = gl::GetString(gl::VERSION);
        println!(
            "[OPENGL] {}",
            std::ffi::CStr::from_ptr(version as *const _)
                .to_string_lossy()
        );
    }

    let fsr = match FsrUpscaler::new() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[FATAL] No se pudo inicializar FSR: {}", e);
            panic!("[FATAL] FSR initialization failed");
        }
    };
    println!("[FSR] FsrUpscaler creado");

    let screen_w = 1280u32;
    let screen_h = 720u32;
    let mut quality = FsrQuality::Quality;
    let mut fsr_enabled = true;

    let (render_w, render_h) = calc_render_size(screen_w, screen_h, quality, fsr_enabled);
    let mut fbo = FboFrame::new(render_w, render_h).unwrap();
    println!(
        "[FBO] {}x{} → {}x{} (FSR {:?})",
        render_w, render_h, screen_w, screen_h, quality
    );

    let shader = SimpleShader::new();
    println!("[GPU] SimpleShader creado (VAO/VBO moderno)");

    let mut event_pump = sdl.event_pump().unwrap();

    let mut frame = 0u64;
    let mut last_time = Instant::now();
    let mut fps_time = Instant::now();
    let mut fps_display = 0u32;
    let mut fps_counter = 0u32;
    let mut time = 0.0f32;
    let mut auto_detect = true;

    println!("\n🎮 CONTROLES:");
    println!("   F = Cycle calidad FSR (Quality → Balanced → Performance)");
    println!("   E = Toggle FSR ON/OFF");
    println!("   A = Toggle Auto-detect (baja resolución si FPS < 30)");
    println!("   ESC = Salir");
    println!("========================================\n");

    'running: loop {
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32().min(0.05);
        last_time = now;
        time += dt;
        frame += 1;
        fps_counter += 1;

        if fps_time.elapsed().as_secs() >= 1 {
            fps_display = fps_counter;
            fps_counter = 0;
            fps_time = Instant::now();

            if auto_detect && fps_display < 30 && fsr_enabled {
                quality = match quality {
                    FsrQuality::Quality => FsrQuality::Balanced,
                    FsrQuality::Balanced => FsrQuality::Performance,
                    FsrQuality::Performance => FsrQuality::Performance,
                };
                let (rw, rh) = calc_render_size(screen_w, screen_h, quality, fsr_enabled);
                fbo = FboFrame::new(rw, rh).unwrap();
                println!("[AUTO] FPS bajo ({}), cambiando a {:?}", fps_display, quality);
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(key),
                    repeat: false,
                    ..
                } => match key {
                    Keycode::Escape => break 'running,
                    Keycode::F => {
                        quality = match quality {
                            FsrQuality::Quality => FsrQuality::Balanced,
                            FsrQuality::Balanced => FsrQuality::Performance,
                            FsrQuality::Performance => FsrQuality::Quality,
                        };
                        let (rw, rh) = calc_render_size(screen_w, screen_h, quality, fsr_enabled);
                        fbo = FboFrame::new(rw, rh).unwrap();
                        println!(
                            "[FSR] Calidad: {:?} | Render: {}x{} → Screen: {}x{}",
                            quality, rw, rh, screen_w, screen_h
                        );
                    }
                    Keycode::E => {
                        fsr_enabled = !fsr_enabled;
                        let (rw, rh) = calc_render_size(screen_w, screen_h, quality, fsr_enabled);
                        fbo = FboFrame::new(rw, rh).unwrap();
                        println!("[FSR] {}", if fsr_enabled { "ON" } else { "OFF" });
                    }
                    Keycode::A => {
                        auto_detect = !auto_detect;
                        println!("[AUTO-DETECT] {}", if auto_detect { "ON" } else { "OFF" });
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // ===== PIPELINE: Escena → FBO → FSR → Screen =====
        fbo.bind();
        render_scene_to_shader(&shader, time, render_w as f32, render_h as f32);
        fbo.unbind(screen_w, screen_h);

        unsafe {
            gl::ClearColor(BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        if fsr_enabled {
            fsr.render(fbo.texture(), (render_w, render_h), (screen_w, screen_h));
        } else {
            render_texture_passthrough(fbo.texture());
        }

        window.gl_swap_window();

        if frame % 60 == 0 {
            let scale = if fsr_enabled { quality.scale() } else { 1.0 };
            println!(
                "[FSR] FPS: {} | Calidad: {:?} | Scale: {:.2} | Render: {}x{} | {}",
                fps_display, quality, scale, render_w, render_h,
                if fsr_enabled { "FSR ON" } else { "FSR OFF" }
            );
        }
    }

    println!("\n✅ Demo finalizado");
    println!("📊 Frames: {}", frame);
    println!("🎆 FPS final: {}", fps_display);
}

fn calc_render_size(
    screen_w: u32,
    screen_h: u32,
    quality: FsrQuality,
    enabled: bool,
) -> (u32, u32) {
    if !enabled {
        return (screen_w, screen_h);
    }
    let scale = quality.scale();
    (
        (screen_w as f32 * scale) as u32,
        (screen_h as f32 * scale) as u32,
    )
}

/// Construir vértices de la escena (triángulos)
fn build_scene_vertices(time: f32, w: f32, h: f32) -> Vec<(f32, f32, f32, f32, f32)> {
    let mut verts: Vec<(f32, f32, f32, f32, f32)> = Vec::with_capacity(2000);

    // Grid
    let spacing = 40.0f32;
    let mut x = 0.0f32;
    while x < w {
        add_line(&mut verts, x, 0.0, x, h, 1.0, GRID_COLOR);
        x += spacing;
    }
    let mut y = 0.0f32;
    while y < h {
        add_line(&mut verts, 0.0, y, w, y, 1.0, GRID_COLOR);
        y += spacing;
    }

    // Círculos concéntricos
    add_circle(&mut verts, w / 2.0, h / 2.0, 80.0 + time.sin() * 10.0, 32, 3.0, RED);
    add_circle(&mut verts, w / 2.0, h / 2.0, 50.0 + time.cos() * 8.0, 32, 3.0, YELLOW);
    add_circle(&mut verts, w / 2.0, h / 2.0, 25.0, 32, 2.0, WHITE_C);

    // Círculos orbitando
    for i in 0..8 {
        let angle = time * 1.5 + (i as f32) * std::f32::consts::TAU / 8.0;
        let radius = 150.0 + time.sin() * 30.0;
        let cx = w / 2.0 + angle.cos() * radius;
        let cy = h / 2.0 + angle.sin() * radius;
        let color = match i % 3 {
            0 => RED,
            1 => GREEN_C,
            _ => CYAN_C,
        };
        add_circle(&mut verts, cx, cy, 8.0, 16, 2.0, color);
    }

    // Rectángulos orbitando
    for i in 0..12 {
        let angle = (i as f32 / 12.0) * std::f32::consts::TAU + time * 0.5;
        let rx = w / 2.0 + angle.cos() * 250.0 - 15.0;
        let ry = h / 2.0 + angle.sin() * 250.0 - 15.0;
        let color = match i % 4 {
            0 => RED,
            1 => GREEN_C,
            2 => CYAN_C,
            _ => YELLOW,
        };
        add_rect(&mut verts, rx, ry, 30.0, 30.0, color);
    }

    // "Texto" simulado
    for i in 0..20 {
        let x = 30.0 + (i as f32) * 14.0;
        let color = if i % 3 == 0 { GREEN_C } else { WHITE_C };
        add_rect(&mut verts, x, 20.0, 8.0, 12.0, color);
    }

    verts
}

fn add_line(
    verts: &mut Vec<(f32, f32, f32, f32, f32)>,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    width: f32,
    color: (f32, f32, f32),
) {
    let dx = (x2 - x1).hypot(y2 - y1);
    let nx = -(y2 - y1) / dx * width / 2.0;
    let ny = (x2 - x1) / dx * width / 2.0;
    // 2 triángulos = 6 vértices
    verts.extend_from_slice(&[
        (x1 + nx, y1 + ny, color.0, color.1, color.2),
        (x1 - nx, y1 - ny, color.0, color.1, color.2),
        (x2 + nx, y2 + ny, color.0, color.1, color.2),
        (x1 - nx, y1 - ny, color.0, color.1, color.2),
        (x2 - nx, y2 - ny, color.0, color.1, color.2),
        (x2 + nx, y2 + ny, color.0, color.1, color.2),
    ]);
}

fn add_circle(
    verts: &mut Vec<(f32, f32, f32, f32, f32)>,
    cx: f32,
    cy: f32,
    radius: f32,
    segments: usize,
    line_width: f32,
    color: (f32, f32, f32),
) {
    for i in 0..segments {
        let a1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let a2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;
        let x1 = cx + a1.cos() * radius;
        let y1 = cy + a1.sin() * radius;
        let x2 = cx + a2.cos() * radius;
        let y2 = cy + a2.sin() * radius;
        add_line(verts, x1, y1, x2, y2, line_width, color);
    }
}

fn add_rect(
    verts: &mut Vec<(f32, f32, f32, f32, f32)>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: (f32, f32, f32),
) {
    verts.extend_from_slice(&[
        (x, y, color.0, color.1, color.2),
        (x + w, y, color.0, color.1, color.2),
        (x + w, y + h, color.0, color.1, color.2),
        (x, y, color.0, color.1, color.2),
        (x + w, y + h, color.0, color.1, color.2),
        (x, y + h, color.0, color.1, color.2),
    ]);
}

fn render_scene_to_shader(shader: &SimpleShader, time: f32, w: f32, h: f32) {
    unsafe {
        gl::ClearColor(BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::Disable(gl::DEPTH_TEST);
    }
    let verts = build_scene_vertices(time, w, h);
    shader.draw(&verts, w, h);
}

fn render_texture_passthrough(texture: GLuint) {
    use std::ffi::CString;

    static mut PROGRAM: GLuint = 0;
    static mut VAO: GLuint = 0;
    static mut VBO: GLuint = 0;
    static mut INIT: bool = false;

    unsafe {
        if !INIT {
            let vs = compile_shader(
                gl::VERTEX_SHADER,
                r#"#version 330 core
                layout(location=0) in vec2 pos;
                layout(location=1) in vec2 uv;
                out vec2 vUV;
                void main(){
                    gl_Position = vec4(pos, 0.0, 1.0);
                    vUV = uv;
                }"#,
            );
            let fs = compile_shader(
                gl::FRAGMENT_SHADER,
                r#"#version 330 core
                in vec2 vUV;
                uniform sampler2D tex;
                out vec4 fragColor;
                void main(){
                    fragColor = texture(tex, vUV);
                }"#,
            );
            PROGRAM = gl::CreateProgram();
            gl::AttachShader(PROGRAM, vs);
            gl::AttachShader(PROGRAM, fs);
            gl::LinkProgram(PROGRAM);
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);

            let vertices: [f32; 16] = [
                -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 0.0,
                1.0,
            ];
            gl::GenVertexArrays(1, &mut VAO);
            gl::GenBuffers(1, &mut VBO);
            gl::BindVertexArray(VAO);
            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * 4) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 16, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 16, (8 * 4) as *const _);
            gl::EnableVertexAttribArray(1);
            gl::BindVertexArray(0);
            INIT = true;
        }

        gl::UseProgram(PROGRAM);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        let loc = gl::GetUniformLocation(PROGRAM, b"tex\0".as_ptr() as *const _);
        gl::Uniform1i(loc, 0);
        gl::BindVertexArray(VAO);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
        gl::BindVertexArray(0);
        gl::UseProgram(0);
    }
}

unsafe fn compile_shader(shader_type: GLuint, source: &str) -> GLuint {
    let shader = gl::CreateShader(shader_type);
    let c_str = CString::new(source).unwrap();
    gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
    gl::CompileShader(shader);
    let mut success = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    if success == 0 {
        let mut len = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = vec![0u8; len as usize];
        gl::GetShaderInfoLog(shader, len, &mut len, buf.as_mut_ptr() as *mut _);
        let msg = String::from_utf8_lossy(&buf[..len as usize]);
        eprintln!("[SHADER ERROR] {}: {}", shader_type, msg);
    }
    shader
}
