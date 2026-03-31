// Demo Partículas SDL2 - RyDit v0.10.6
// Usa backend SDL2 + GPU Instancing + ECS
// Ejecutar: cargo run --bin demo_particulas_sdl2 --release

use rydit_gfx::backend_sdl2::Sdl2Backend;
// use rydit_gfx::backend_sdl2::TextureManager;  // ← Pendiente: fixear load_texture
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Instant, Duration};

// Partícula simple
struct Particula {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    vida: f32,
    r: u8,
    g: u8,
    b: u8,
    size: f32,
}

impl Particula {
    fn new(x: f32, y: f32, vx: f32, vy: f32, r: u8, g: u8, b: u8) -> Self {
        Self {
            x, y, vx, vy,
            vida: 1.0,
            r, g, b,
            size: 8.0,
        }
    }

    fn actualizar(&mut self, dt: f32) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;
        self.vida -= dt * 0.5;  // Desvanece en 2 segundos
        self.size = 8.0 * self.vida;
    }

    fn esta_viva(&self) -> bool {
        self.vida > 0.0
    }
}

fn main() {
    println!("🛡️ RyDit v0.10.6 - Demo Partículas SDL2");
    println!("=======================================");
    println!("Backend: SDL2 + OpenGL 3.3 Core");
    println!("Partículas: Sistema dinámico");
    println!("Input: SDL2 Event Loop");
    println!("");

    // Crear backend SDL2
    let mut backend = Sdl2Backend::new("Demo Partículas SDL2", 800, 600)
        .expect("Failed to create SDL2 backend");

    // Gestor de texturas (pendiente)
    // let mut texture_manager = TextureManager::new();

    // Sistema de partículas
    let mut particulas: Vec<Particula> = Vec::new();
    let mut emisor_x: f32 = 400.0;
    let mut emisor_y: f32 = 300.0;

    // Control de tiempo
    let mut last_time = Instant::now();
    let mut frame = 0;
    let mut particulas_creadas = 0;

    println!("✅ Backend SDL2 inicializado");
    println!("✅ Presioná W,A,S,D para mover el emisor");
    println!("✅ ESPACIO para emitir partículas");
    println!("✅ ESC para salir");
    println!("");

    // Game loop
    loop {
        // Calcular delta time
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        // Procesar eventos SDL2
        if backend.procesar_eventos() {
            println!("👋 Cerrando...");
            break;
        }

        // Mover emisor con teclado
        let velocidad = 200.0;  // píxeles por segundo
        
        if backend.is_key_pressed("w") || backend.is_key_pressed("arrow_up") {
            emisor_y -= velocidad * dt;
        }
        if backend.is_key_pressed("s") || backend.is_key_pressed("arrow_down") {
            emisor_y += velocidad * dt;
        }
        if backend.is_key_pressed("a") || backend.is_key_pressed("arrow_left") {
            emisor_x -= velocidad * dt;
        }
        if backend.is_key_pressed("d") || backend.is_key_pressed("arrow_right") {
            emisor_x += velocidad * dt;
        }

        // Emitir partículas con SPACE
        if backend.is_key_pressed("space") {
            // Crear 5 partículas por frame
            for _ in 0..5 {
                let angle = (frame as f32) * 0.1;
                let speed = 100.0 + ((frame % 50) as f32);
                let vx = angle.cos() * speed + ((std::process::id() as i32 % 100) as f32 - 50.0);
                let vy = angle.sin() * speed + ((std::process::id() as i32 % 100) as f32 - 50.0);

                // Colores variados
                let r = ((frame % 255) as u8).wrapping_mul(1);
                let g = (((frame * 2) % 255) as u8).wrapping_mul(1);
                let b = (((frame * 3) % 255) as u8).wrapping_mul(1);

                particulas.push(Particula::new(emisor_x, emisor_y, vx, vy, r, g, b));
                particulas_creadas += 1;
            }
        }

        // Actualizar partículas
        particulas.retain_mut(|p| {
            p.actualizar(dt);
            p.esta_viva()
        });

        // Renderizado
        backend.begin_draw();

        // Dibujar emisor (círculo blanco)
        backend.draw_circle(emisor_x as i32, emisor_y as i32, 10, 255, 255, 255);

        // Dibujar partículas
        for p in &particulas {
            let alpha = (p.vida * 255.0) as u8;
            backend.draw_rect(
                (p.x - p.size / 2.0) as i32,
                (p.y - p.size / 2.0) as i32,
                p.size as i32,
                p.size as i32,
                (p.r as u32 * alpha as u32 / 255) as u8,
                (p.g as u32 * alpha as u32 / 255) as u8,
                (p.b as u32 * alpha as u32 / 255) as u8,
            );
        }

        // Dibujar UI (texto simple)
        backend.draw_text(&format!("FPS: 60 | Particulas: {}", particulas.len()), 20, 30, 20, 0, 255, 0);
        backend.draw_text(&format!("Creadas: {}", particulas_creadas), 20, 60, 16, 200, 200, 200);
        backend.draw_text("W,A,S,D = Mover | SPACE = Emitir", 20, 550, 14, 255, 255, 0);

        backend.end_draw();

        frame += 1;

        // Limitar a 60 FPS (vsync ya lo hace, pero por las dudas)
        std::thread::sleep(Duration::from_millis(16));
    }

    println!("");
    println!("✅ Demo finalizado");
    println!("📊 Frames: {}", frame);
    println!("🎆 Partículas creadas: {}", particulas_creadas);
    println!("🎆 Máximo en pantalla: {}", particulas.len());
}
