//! demo_colisiones.rs
//! Demo colisiones SDL2 - INPUT CORREGIDO para Termux-X11
//!
//! ✅ SOLUCIÓN: Cada pulsación = acción individual (NO mantener teclas)
//! ✅ ESPACIO = Saltar (una pulsación)
//! ← → = Mover (cada pulsación mueve un poco)
//!
//! PROBLEMA: Teclado virtual Android NO envía repeat: true
//! SOLUCIÓN: Cada pulsación mueve una cantidad fija

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() -> Result<(), String> {
    println!("🛡️ RyDit - Demo Colisiones (Input Corregido)");
    println!("==============================================");
    println!("⚠️  Teclado virtual: cada tecla = acción única");
    println!("🎮 ← → = Mover (pulsación individual)");
    println!("🎮 ESPACIO = Saltar");
    println!("==============================================\n");

    let sdl = sdl2::init().map_err(|e| e.to_string())?;
    let video = sdl.video().map_err(|e| e.to_string())?;
    let window = video
        .window("Demo Colisiones - RyDit", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl.event_pump().map_err(|e| e.to_string())?;

    // JUGADOR
    let mut j_x: f32 = 100.0;
    let mut j_y: f32 = 500.0;
    let mut j_vy: f32 = 0.0;
    let j_ancho: i32 = 40;
    let j_alto: i32 = 40;
    let movimiento = 30.0; // Cada pulsación mueve esto

    // FÍSICAS
    let gravedad: f32 = 800.0;
    let fuerza_salto: f32 = -450.0;
    let mut en_suelo: bool = false;

    // PLATAFORMAS
    let plataformas = vec![
        Rect::new(0, 560, 800, 40),    // Suelo
        Rect::new(150, 480, 150, 15),  // Plataforma 1
        Rect::new(400, 400, 150, 15),  // Plataforma 2
        Rect::new(100, 320, 150, 15),  // Plataforma 3
        Rect::new(500, 280, 180, 15),  // Plataforma 4
        Rect::new(250, 200, 120, 15),  // Plataforma 5
        Rect::new(550, 140, 120, 15),  // Plataforma 6
    ];

    let mut frame = 0u64;
    let mut saltos = 0u64;
    let mut movs_izq = 0u64;
    let mut movs_der = 0u64;

    println!("🎮 CONTROLES (cada tecla = acción):");
    println!("   ← = Mover izquierda (una pulsación)");
    println!("   → = Mover derecha (una pulsación)");
    println!("   ESPACIO = Saltar");
    println!("   R = Reset");
    println!("   ESC = Salir");
    println!("==============================================\n");

    'running: loop {
        let dt = 0.016f32;
        frame += 1;

        // ================================================================
        // INPUT - SOLO repeat: false (teclado virtual no envía repeat: true)
        // ================================================================
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }

                // ✅ SOLO PRIMERA PULSACIÓN (esto es lo que funciona en Termux-X11)
                Event::KeyDown { keycode: Some(key), repeat: false, .. } => {
                    match key {
                        Keycode::Space => {
                            if en_suelo {
                                j_vy = fuerza_salto;
                                en_suelo = false;
                                saltos += 1;
                                println!("🦘 Salto #{} (y={:.0})", saltos, j_y);
                            }
                        }
                        Keycode::Left => {
                            j_x -= movimiento;
                            movs_izq += 1;
                            println!("⬅ Izquierda #{} (x={:.0})", movs_izq, j_x);
                        }
                        Keycode::Right => {
                            j_x += movimiento;
                            movs_der += 1;
                            println!("➡ Derecha #{} (x={:.0})", movs_der, j_x);
                        }
                        Keycode::R => {
                            j_x = 100.0;
                            j_y = 500.0;
                            j_vy = 0.0;
                            en_suelo = false;
                            println!("🔄 Reset");
                        }
                        Keycode::Up => {
                            if en_suelo {
                                j_vy = fuerza_salto;
                                en_suelo = false;
                                saltos += 1;
                                println!("🦘 Salto con ↑ #{} (y={:.0})", saltos, j_y);
                            }
                        }
                        _ => {}
                    }
                }

                // ⚠️ repeat: true NO llega en teclado virtual Android
                Event::KeyDown { keycode: Some(_), repeat: true, .. } => {
                    // Este evento NUNCA llega con teclado virtual
                    // Descomenta para verificar:
                    // println!("⚠️ repeat: true detectado (esto no debería pasar)");
                }

                _ => {}
            }
        }

        // ================================================================
        // UPDATE - Físicas + Colisiones
        // ================================================================
        j_vy += gravedad * dt;
        j_y += j_vy * dt;

        let j_rect = Rect::new(j_x as i32, j_y as i32, j_ancho as u32, j_alto as u32);

        en_suelo = false;
        for plat in &plataformas {
            if j_rect.has_intersection(*plat) {
                // Desde ARRIBA (aterrizar)
                if j_rect.bottom() as i32 <= plat.y + 10 && j_vy > 0.0 {
                    j_y = plat.y as f32 - j_alto as f32;
                    j_vy = 0.0;
                    en_suelo = true;
                }
                // Desde ABAJO (cabeza)
                else if j_rect.top() as i32 >= plat.bottom() - 10 && j_vy < 0.0 {
                    j_y = plat.bottom() as f32;
                    j_vy = 0.0;
                }
                // Lateral IZQ
                else if j_rect.right() as i32 <= plat.x + 10 {
                    j_x = plat.x as f32 - j_ancho as f32;
                }
                // Lateral DER
                else if j_rect.left() as i32 >= plat.right() - 10 {
                    j_x = plat.right() as f32;
                }
            }
        }

        // Límites
        if j_x < 0.0 { j_x = 0.0; }
        if j_x > 760.0 { j_x = 760.0; }

        // Respawn si cae
        if j_y > 620.0 {
            j_x = 100.0;
            j_y = 100.0;
            j_vy = 0.0;
            println!("💀 Caíste - Respawn");
        }

        // ================================================================
        // RENDER
        // ================================================================
        canvas.set_draw_color(Color::RGB(20, 20, 30));
        canvas.clear();

        // Plataformas
        canvas.set_draw_color(Color::RGB(100, 100, 120));
        for plat in &plataformas {
            let _ = canvas.fill_rect(*plat);
            canvas.set_draw_color(Color::RGB(150, 150, 170));
            let _ = canvas.fill_rect(Rect::new(plat.x, plat.y, plat.width(), 3));
            canvas.set_draw_color(Color::RGB(100, 100, 120));
        }

        // Jugador (rojo)
        canvas.set_draw_color(Color::RGB(255, 50, 50));
        let _ = canvas.fill_rect(Rect::new(j_x as i32, j_y as i32, j_ancho as u32, j_alto as u32));

        // Ojos
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let ojo = if j_x > 100.0 { 24 } else { 4 };
        let _ = canvas.fill_rect(Rect::new(j_x as i32 + ojo, j_y as i32 + 10, 5, 5));
        let _ = canvas.fill_rect(Rect::new(j_x as i32 + ojo + 12, j_y as i32 + 10, 5, 5));

        // Referencia centro
        canvas.set_draw_color(Color::RGB(0, 100, 255));
        let _ = canvas.fill_rect(Rect::new(397, 297, 6, 6));

        // Info
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let _ = canvas.fill_rect(Rect::new(15, 15, 180, 18));
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        let _ = canvas.fill_rect(Rect::new(15, 45, 160, 14));

        // En suelo (verde=si, rojo=no)
        canvas.set_draw_color(if en_suelo { Color::RGB(0, 255, 0) } else { Color::RGB(255, 0, 0) });
        let _ = canvas.fill_rect(Rect::new(15, 70, 140, 14));

        // Controles
        canvas.set_draw_color(Color::RGB(80, 80, 80));
        let _ = canvas.fill_rect(Rect::new(15, 550, 400, 16));
        let _ = canvas.fill_rect(Rect::new(15, 570, 300, 16));

        canvas.present();

        // Log cada 180 frames (~3 seg)
        if frame % 180 == 0 {
            println!("📊 Frame {} | Pos: ({:.0}, {:.0}) | Suelo: {} | Saltos: {} | ←:{} →:{}",
                frame, j_x, j_y, if en_suelo { "✅" } else { "❌" }, saltos, movs_izq, movs_der);
        }
    }

    println!("\n✅ Demo finalizado");
    println!("📊 Frames: {} | Saltos: {} | ←:{} →:{}", frame, saltos, movs_izq, movs_der);

    Ok(())
}
