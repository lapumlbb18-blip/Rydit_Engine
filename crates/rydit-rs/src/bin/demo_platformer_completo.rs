// crates/rydit-rs/src/bin/demo_platformer_completo.rs
// Demo Platformer Completo - v0.11.0
// Con plataformas, gravedad, salto y colisiones
//
// Ejecutar: cargo run --bin demo_platformer_completo --release

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() {
    println!("🛡️ RyDit v0.11.0 - Demo Platformer Completo");
    println!("============================================\n");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Demo Platformer Completo", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Jugador
    let mut jugador_x: f32 = 100.0;
    let jugador_y_inicial: f32 = 500.0;
    let mut jugador_y: f32 = jugador_y_inicial;
    let mut velocidad_y: f32 = 0.0;
    let jugador_ancho: i32 = 40;
    let jugador_alto: i32 = 40;

    // Físicas
    let gravedad: f32 = 800.0;
    let fuerza_salto: f32 = -500.0;
    let velocidad_movimiento: f32 = 250.0;
    let mut en_suelo: bool = true;

    // Plataformas (x, y, ancho, alto)
    let plataformas: Vec<Rect> = vec![
        Rect::new(0, 550, 800, 50),   // Suelo
        Rect::new(200, 480, 150, 20), // Plataforma 1
        Rect::new(450, 420, 150, 20), // Plataforma 2
        Rect::new(150, 350, 150, 20), // Plataforma 3
        Rect::new(500, 300, 200, 20), // Plataforma 4
        Rect::new(300, 220, 100, 20), // Plataforma 5
    ];

    let mut running = true;

    println!("✅ Ventana: 800x600");
    println!("✅ Controles:");
    println!("   - A / ← : Mover izquierda");
    println!("   - D / → : Mover derecha");
    println!("   - W / ↑ / SPACE : Saltar");
    println!("   - ESC : Salir");
    println!();

    'running: while running {
        let dt: f32 = 0.016; // 60 FPS

        // Eventos
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    #[allow(unused_assignments)]
                    {
                        running = false;
                    }
                    break 'running;
                }

                // Teclas mantenidas (movimiento continuo)
                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: true,
                    ..
                } => match keycode {
                    Keycode::D | Keycode::Right => {
                        jugador_x += velocidad_movimiento * dt;
                    }
                    Keycode::A | Keycode::Left => {
                        jugador_x -= velocidad_movimiento * dt;
                    }
                    Keycode::W | Keycode::Up | Keycode::Space => {
                        if en_suelo {
                            velocidad_y = fuerza_salto;
                            en_suelo = false;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Aplicar gravedad
        velocidad_y += gravedad * dt;
        jugador_y += velocidad_y * dt;

        // Jugador rect
        let jugador_rect = Rect::new(
            jugador_x as i32,
            jugador_y as i32,
            jugador_ancho as u32,
            jugador_alto as u32,
        );

        // Colisiones con plataformas
        en_suelo = false;
        for plataforma in &plataformas {
            if jugador_rect.has_intersection(*plataforma) {
                // Colisión desde arriba (aterrizar)
                if jugador_rect.bottom() <= plataforma.y + 10 && velocidad_y > 0.0 {
                    jugador_y = plataforma.y as f32 - jugador_alto as f32;
                    velocidad_y = 0.0;
                    en_suelo = true;
                }
                // Colisión desde abajo (golpear cabeza)
                else if jugador_rect.top() >= plataforma.bottom() - 10 && velocidad_y < 0.0 {
                    jugador_y = plataforma.bottom() as f32;
                    velocidad_y = 0.0;
                }
                // Colisión lateral
                else if jugador_rect.right() <= plataforma.x + 10 {
                    jugador_x = plataforma.x as f32 - jugador_ancho as f32;
                } else if jugador_rect.left() >= plataforma.right() - 10 {
                    jugador_x = plataforma.right() as f32;
                }
            }
        }

        // Límites de pantalla
        if jugador_x < 0.0 {
            jugador_x = 0.0;
        }
        if jugador_x > 760.0 {
            jugador_x = 760.0;
        }
        if jugador_y > 600.0 {
            // Cayó al vacío
            jugador_x = 100.0;
            jugador_y = jugador_y_inicial;
            velocidad_y = 0.0;
        }

        // Renderizado
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();

        // Dibujar plataformas
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        for plataforma in &plataformas {
            canvas.fill_rect(*plataforma).unwrap();
        }

        // Dibujar jugador
        canvas.set_draw_color(Color::RGB(255, 50, 50));
        canvas.fill_rect(jugador_rect).unwrap();

        // Ojos del jugador (dirección)
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let ojo_offset = if jugador_x > 100.0 { 25 } else { 5 };
        canvas
            .fill_rect(Rect::new(
                jugador_x as i32 + ojo_offset,
                jugador_y as i32 + 10,
                4,
                4,
            ))
            .unwrap();

        canvas.present();
    }

    println!("\n✅ Demo finalizado");
    println!("📍 Posición final: ({:.1}, {:.1})", jugador_x, jugador_y);
}
