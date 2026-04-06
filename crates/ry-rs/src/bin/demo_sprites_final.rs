//! demo_sprites_final.rs
//! Demo FINAL - Input SDL2 correcto + Verificación sprites
//!
//! ✅ PATRÓN CLAVE: DOS EVENTOS (repeat: false + repeat: true)
//! ✅ SDL2 DIRECTO (como test_callback_sdl2.rs y demo_movimiento.rs)
//! ✅ Verifica archivos PNG de sprites
//! ✅ Sin problemas de lifetime
//!
//! BASADO EN CÓDIGO QUE FUNCIONÓ:
//! - test_callback_sdl2.rs (original que descubrió input SDL2)
//! - demo_movimiento.rs (copia exacta del que funcionó)
//! - demo_sdl2_puro.rs (demo SDL2 puro)
//! - demo_platformer_completo.rs (platformer con gravedad/colisiones)
//!
//! Uso: cargo run --bin demo_sprites_final --release

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use std::path::Path;

fn main() {
    println!("🛡️ RyDit v0.11.6 - Demo Sprites FINAL");
    println!("======================================");
    println!("🎨 SDL2 Directo (como test_callback_sdl2.rs)");
    println!("🖼️  Verificación archivos PNG sprites");
    println!("🎮 Input DOS EVENTOS (repeat: false + true)");
    println!("======================================\n");

    // 1. SDL2 DIRECTO (como test_callback_sdl2.rs)
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Demo Sprites FINAL - RyDit v0.11.6", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // 2. Verificar sprites
    let sprites_dir = "/data/data/com.termux/files/home/shield-project/logo_icon_asst/sprites";
    println!("📂 Verificando sprites en: {}\n", sprites_dir);

    let mut sprites = vec![
        ("tank", "tank_16x16.png", 150.0, 300.0, 64, Color::RGB(0, 255, 0)),
        ("helicopter", "helicopter_16x16.png", 350.0, 200.0, 64, Color::RGB(0, 255, 255)),
        ("crate", "crate_8x8.png", 550.0, 300.0, 32, Color::RGB(139, 69, 19)),
        ("platform", "platform_16x16.png", 350.0, 450.0, 96, Color::RGB(128, 128, 128)),
    ];

    let mut existentes = 0;
    for (_nombre, archivo, _, _, _, _) in &mut sprites {
        let path = format!("{}/{}", sprites_dir, archivo);
        let existe = Path::new(&path).exists();
        if existe { existentes += 1; }
        println!("  ├─ {}... {}", archivo, if existe { "✅" } else { "❌" });
        *archivo = if existe { "✅" } else { "❌" };
    }

    println!("\n✅ {}/4 archivos encontrados\n", existentes);

    // Posiciones y estado
    let mut pos_x: Vec<f32> = sprites.iter().map(|s| s.2).collect();
    let mut pos_y: Vec<f32> = sprites.iter().map(|s| s.3).collect();
    let mut tamaños: Vec<i32> = sprites.iter().map(|s| s.4 as i32).collect();
    let mut sel = 0;
    let mut frame = 0;
    let mut anim = true;
    let running = true;

    println!("🎮 CONTROLES (PATRÓN DOS EVENTOS):");
    println!("   ← → ↑ ↓ = Mover (MANTENER)");
    println!("   1-4 = Seleccionar sprite");
    println!("   A = Toggle animación");
    println!("   R = Reset posiciones");
    println!("   ESC = Salir");
    println!("======================================\n");

    // 3. Game loop CON PATRÓN DOS EVENTOS
    'running: while running {
        frame += 1;

        // === INPUT SDL2 (CLAVE: DOS EVENTOS) ===
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    println!("\n👋 Saliendo...");
                    break 'running;
                }

                // ✅ EVENTO 1: Primera pulsación
                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => {
                    match keycode {
                        Keycode::Num1 => sel = 0,
                        Keycode::Num2 => sel = 1,
                        Keycode::Num3 => sel = 2,
                        Keycode::Num4 => sel = 3,
                        Keycode::A => {
                            anim = !anim;
                            println!("🎬 Animación: {}", if anim { "ON" } else { "OFF" });
                        }
                        Keycode::R => {
                            pos_x = vec![150.0, 350.0, 550.0, 350.0];
                            pos_y = vec![300.0, 200.0, 300.0, 450.0];
                            tamaños = vec![64, 64, 32, 96];
                            sel = 0;
                            println!("🔄 Reset");
                        }
                        _ => {}
                    }
                }

                // ✅ EVENTO 2: Tecla mantenida (movimiento continuo) - LA CLAVE
                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: true,
                    ..
                } => {
                    let vel = 5.0;
                    match keycode {
                        Keycode::Left | Keycode::A => pos_x[sel] -= vel,
                        Keycode::Right | Keycode::D => pos_x[sel] += vel,
                        Keycode::Up | Keycode::W => pos_y[sel] -= vel,
                        Keycode::Down | Keycode::S => pos_y[sel] += vel,
                        _ => {}
                    }
                }

                _ => {}
            }
        }

        // === UPDATE ===
        if anim {
            let t = (frame as f32 * 0.03).sin();
            pos_y[1] = 200.0 + (t * 30.0);
            pos_x[2] = 550.0 + (t.cos() * 20.0);
        }

        // === RENDER SDL2 ===
        canvas.set_draw_color(Color::RGB(10, 10, 20));
        canvas.clear();

        // Grid
        canvas.set_draw_color(Color::RGB(30, 30, 40));
        for x in (0..800).step_by(50) {
            let _ = canvas.draw_line(Point::new(x, 0), Point::new(x, 600));
        }
        for y in (0..600).step_by(50) {
            let _ = canvas.draw_line(Point::new(0, y), Point::new(800, y));
        }

        // Dibujar sprites
        for i in 0..sprites.len() {
            let sz = tamaños[i];
            let x = pos_x[i] as i32 - sz / 2;
            let y = pos_y[i] as i32 - sz / 2;

            // Rect de color
            canvas.set_draw_color(sprites[i].5);
            let _ = canvas.fill_rect(Rect::new(x, y, sz as u32, sz as u32));

            // Indicador si archivo existe (barra blanca arriba)
            if sprites[i].1 == "✅" {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                let _ = canvas.fill_rect(Rect::new(x + 2, y + 2, (sz - 4) as u32, 4));
            }

            // Borde selección
            if i == sel {
                canvas.set_draw_color(Color::RGB(255, 255, 0));
                let _ = canvas.draw_rect(Rect::new(x - 3, y - 3, (sz + 6) as u32, (sz + 6) as u32));
            }
        }

        // UI superior (rectángulos como texto)
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let _ = canvas.fill_rect(Rect::new(20, 10, 250, 18));
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        let _ = canvas.fill_rect(Rect::new(20, 35, 150, 14));
        canvas.set_draw_color(Color::RGB(255, 255, 0));
        let _ = canvas.fill_rect(Rect::new(20, 55, 120, 12));

        canvas.present();
    }

    println!("\n✅ Demo: {} frames | Sprites: {}/4", frame, existentes);
}
