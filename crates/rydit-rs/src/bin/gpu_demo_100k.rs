// crates/rydit-rs/src/bin/gpu_demo_100k.rs
// Demo: 100,000 partículas con GPU Instancing
// v0.10.1: GPU Instancing + Shaders GLSL

use rydit_gfx::gpu_instancing::{GPUInstancer, ParticleData};
use rydit_gfx::{RyditGfx, ColorRydit, Key};
use gl;

fn main() {
    println!("🛡️ RyDit v0.10.1 - GPU Instancing Demo 100K");
    println!("=============================================");
    
    // Crear ventana
    let mut gfx = RyditGfx::new("RyDit v0.10.1 - GPU 100K Demo", 800, 600);
    gfx.set_target_fps(60);
    
    // Inicializar OpenGL (necesario para GPU Instancing)
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
    
    // Crear GPU Instancer
    let mut gpu = GPUInstancer::new();
    
    // Cargar shaders
    println!("Cargando shaders...");
    match gpu.load_shaders(
        "crates/rydit-gfx/shaders/vertex.glsl",
        "crates/rydit-gfx/shaders/fragment.glsl",
    ) {
        Ok(_) => println!("✅ Shaders cargados"),
        Err(e) => {
            println!("❌ Error cargando shaders: {}", e);
            return;
        }
    }
    
    // Configurar proyección
    gpu.set_projection(800.0, 600.0);
    gpu.set_camera(0.0, 0.0);
    
    // Crear 100K partículas
    println!("Creando 100,000 partículas...");
    let mut particles: Vec<ParticleData> = Vec::with_capacity(100000);
    
    for i in 0..100000 {
        let x = ((i % 1000) as f32) * 0.8 + 100.0;
        let y = ((i / 1000) as f32) * 0.6 + 50.0;
        
        // Colores variados
        let r = ((i % 255) as f32) / 255.0;
        let g = (((i / 255) % 255) as f32) / 255.0;
        let b = (((i / 65025) % 255) as f32) / 255.0;
        
        particles.push(ParticleData::new(x, y, 8.0, r, g, b, 0.8));
    }
    
    println!("✅ {} partículas creadas", particles.len());
    
    // Subir partículas a GPU
    gpu.set_particles(&particles);
    
    // Variables de control
    let mut frame = 0;
    let mut last_fps = 0;
    let mut offset_x = 0.0;
    let mut offset_y = 0.0;
    
    println!("\nControles:");
    println!("  W, A, S, D = Mover cámara");
    println!("  R = Reiniciar");
    println!("  ESC = Salir\n");
    
    // Game loop
    while !gfx.should_close() {
        gfx.begin_draw();
        gfx.clear_background(ColorRydit::Negro);
        
        // Input
        if gfx.is_key_pressed(Key::Escape) {
            break;
        }
        
        if gfx.is_key_pressed(Key::R) {
            offset_x = 0.0;
            offset_y = 0.0;
            gpu.set_camera(0.0, 0.0);
            println!("Cámara reiniciada");
        }
        
        // Mover cámara con W, A, S, D
        let speed = 10.0;
        if gfx.is_key_down(Key::D) {
            offset_x += speed;
        }
        if gfx.is_key_down(Key::A) {
            offset_x -= speed;
        }
        if gfx.is_key_down(Key::S) {
            offset_y += speed;
        }
        if gfx.is_key_down(Key::W) {
            offset_y -= speed;
        }
        
        gpu.set_camera(offset_x, offset_y);
        
        // Actualizar partículas (animación simple)
        if frame % 60 == 0 {
            for i in 0..particles.len() {
                particles[i].offset[1] += 0.5;
                if particles[i].offset[1] > 650.0 {
                    particles[i].offset[1] = 50.0;
                }
            }
            gpu.set_particles(&particles);
        }
        
        // Renderizar con GPU Instancing
        gpu.draw();
        
        // Debug UI
        let current_fps = gfx.get_fps();
        if current_fps != last_fps {
            last_fps = current_fps;
            println!("FPS: {} | Partículas: {}", current_fps, particles.len());
        }
        
        gfx.draw_text(
            &format!("RyDit v0.10.1 - GPU Instancing"),
            10, 20, 20, ColorRydit::Blanco,
        );
        
        gfx.draw_text(
            &format!("Partículas: {}", particles.len()),
            10, 45, 16, ColorRydit::Verde,
        );
        
        gfx.draw_text(
            &format!("FPS: {}", current_fps),
            10, 70, 16, ColorRydit::Cyan,
        );
        
        gfx.draw_text(
            &format!("Cámara: ({}, {})", offset_x, offset_y),
            10, 95, 14, ColorRydit::Gris,
        );
        
        gfx.draw_text(
            "W,A,S,D=Mover, R=Reiniciar, ESC=Salir",
            10, 580, 12, ColorRydit::Gris,
        );
        
        gfx.end_draw();
        
        frame += 1;
    }
    
    println!("\n✅ Demo completado: {} frames totales", frame);
    println!("🛡️ GPU Instancing: 100K partículas @ {} FPS", last_fps);
}
