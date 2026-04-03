//! demo_50k_particulas.rs
//! Demo 50,000 partículas - SDL2 directo + Zink + DRI3
//!
//! ✅ SDL2 DIRECTO (como demo_movimiento.rs)
//! ✅ 50K partículas en pantalla 1280x720
//! ✅ Sistema de partículas optimizado con Vec
//!
//! BASADO EN: demo_particulas_sdl2.rs + demo_movimiento.rs
//!
//! Uso: cargo run --bin demo_50k_particulas --release

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Instant;

struct Particula {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    vida: f32,
    max_vida: f32,
    r: u8,
    g: u8,
    b: u8,
    size: u32,
}

impl Particula {
    fn nueva(x: f32, y: f32, vx: f32, vy: f32, r: u8, g: u8, b: u8, size: u32, max_vida: f32) -> Self {
        Self { x, y, vx, vy, vida: max_vida, max_vida, r, g, b, size }
    }

    fn actualizar(&mut self, dt: f32, gravedad: f32) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;
        self.vy += gravedad * dt;
        self.vida -= dt;
    }

    fn viva(&self) -> bool {
        self.vida > 0.0 && self.x > -50.0 && self.x < 1350.0 && self.y > -50.0 && self.y < 800.0
    }
}

fn main() {
    println!("🛡️ RyDit v0.11.6 - Demo 50K Partículas");
    println!("========================================");
    println!("🎨 SDL2 Directo + Zink + DRI3");
    println!("🪟 Ventana: 1280x720");
    println!("🎆 Objetivo: 50,000 partículas");
    println!("========================================\n");

    // SDL2 DIRECTO (como demo_movimiento.rs)
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    
    let window = video
        .window("Demo 50K Partículas - RyDit v0.11.6", 1280, 720)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    // Sistema de partículas
    let mut particulas: Vec<Particula> = Vec::with_capacity(60000);
    let mut emisor_x = 640.0f32;
    let mut emisor_y = 360.0f32;
    let mut gravedad = 200.0f32;
    let mut max_particulas = 50000;
    let mut emitir = true;
    let mut frame = 0;
    let mut total_emitidas = 0u64;
    let mut running = true;
    let mut last_time = Instant::now();
    let mut fps_time = Instant::now();
    let mut fps_counter = 0;
    let mut fps_display = 0;

    println!("🎮 CONTROLES:");
    println!("   ← → ↑ ↓ = Mover emisor");
    println!("   ESPACIO = Toggle emisión");
    println!("   G = Toggle gravedad");
    println!("   +/- = Más/menos partículas");
    println!("   R = Reset");
    println!("   ESC = Salir");
    println!("========================================\n");

    'running: loop {
        let now = Instant::now();
        let dt = (now - last_time).as_secs_f32().min(0.05);
        last_time = now;
        frame += 1;
        fps_counter += 1;

        if fps_time.elapsed().as_secs() >= 1 {
            fps_display = fps_counter;
            fps_counter = 0;
            fps_time = Instant::now();
        }

        // INPUT (PATRÓN DOS EVENTOS)
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                    break 'running;
                }

                Event::KeyDown { keycode: Some(key), repeat: false, .. } => {
                    match key {
                        Keycode::Space => {
                            emitir = !emitir;
                            println!("🎆 Emisión: {}", if emitir { "ON" } else { "OFF" });
                        }
                        Keycode::G => {
                            gravedad = if gravedad > 0.0 { -200.0 } else { 200.0 };
                            println!("🌍 Gravedad: {}", gravedad);
                        }
                        Keycode::R => {
                            particulas.clear();
                            total_emitidas = 0;
                            println!("🔄 Reset");
                        }
                        Keycode::Num1 => max_particulas = 10000,
                        Keycode::Num2 => max_particulas = 25000,
                        Keycode::Num3 => max_particulas = 50000,
                        Keycode::Num4 => max_particulas = 75000,
                        Keycode::Num5 => max_particulas = 100000,
                        _ => {}
                    }
                }

                Event::KeyDown { keycode: Some(key), repeat: true, .. } => {
                    let vel = 300.0 * dt;
                    match key {
                        Keycode::Left | Keycode::A => emisor_x -= vel,
                        Keycode::Right | Keycode::D => emisor_x += vel,
                        Keycode::Up | Keycode::W => emisor_y -= vel,
                        Keycode::Down | Keycode::S => emisor_y += vel,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Emitir partículas
        if emitir && particulas.len() < max_particulas {
            let emitir_count = ((max_particulas - particulas.len()) / 10).min(2000).max(100);
            
            for _ in 0..emitir_count {
                let angle = (frame as f32).to_radians() * 2.0 + (std::process::id() as f32 % 360.0);
                let speed = 50.0 + ((frame as f32 % 100.0) * 3.0);
                let spread = 2.0 * std::f32::consts::PI / 8.0;
                
                for i in 0..8 {
                    if particulas.len() >= max_particulas { break; }
                    
                    let a = angle + (i as f32) * spread;
                    let vx = a.cos() * speed;
                    let vy = a.sin() * speed - 50.0;
                    
                    let fase = (frame as f32 * 0.5 + i as f32 * 45.0) % 360.0;
                    let r = ((fase.sin() * 0.5 + 0.5) * 255.0) as u8;
                    let g = (((fase + 120.0).sin() * 0.5 + 0.5) * 255.0) as u8;
                    let b = (((fase + 240.0).sin() * 0.5 + 0.5) * 255.0) as u8;
                    let size = 2 + (frame % 4) as u32;
                    let vida = 2.0 + (frame as f32 % 3.0);
                    
                    particulas.push(Particula::nueva(emisor_x, emisor_y, vx, vy, r, g, b, size, vida));
                    total_emitidas += 1;
                }
            }
        }

        // Actualizar partículas
        particulas.retain_mut(|p| {
            p.actualizar(dt, gravedad);
            p.viva()
        });

        // RENDER
        canvas.set_draw_color(Color::RGB(5, 5, 10));
        canvas.clear();

        // Dibujar partículas como rects pequeños
        for p in &particulas {
            let alpha = (p.vida / p.max_vida * 255.0) as u8;
            let sz = p.size.max(1);
            let x = p.x as i32 - (sz / 2) as i32;
            let y = p.y as i32 - (sz / 2) as i32;
            
            canvas.set_draw_color(Color::RGBA(p.r, p.g, p.b, alpha));
            let _ = canvas.fill_rect(Rect::new(x, y, sz, sz));
        }

        // Emisor (círculo blanco)
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let _ = canvas.fill_rect(Rect::new(emisor_x as i32 - 5, emisor_y as i32 - 5, 10, 10));

        // UI
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        let _ = canvas.fill_rect(Rect::new(20, 20, 300, 20));
        canvas.set_draw_color(Color::RGB(255, 255, 0));
        let _ = canvas.fill_rect(Rect::new(20, 50, 250, 18));
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        let _ = canvas.fill_rect(Rect::new(20, 78, 200, 16));
        canvas.set_draw_color(Color::RGB(128, 128, 128));
        let _ = canvas.fill_rect(Rect::new(20, 680, 500, 16));
        let _ = canvas.fill_rect(Rect::new(20, 700, 300, 16));

        canvas.present();
    }

    println!("\n✅ Demo finalizado");
    println!("📊 Frames: {}", frame);
    println!("🎆 Totales emitidas: {}", total_emitidas);
    println!("🎆 Máx en pantalla: {}", particulas.len());
    println!("🎆 FPS promedio: {}", fps_display);
}
