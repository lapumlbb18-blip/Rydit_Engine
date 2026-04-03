//! demo_rigidbody.rs
//! Demo Rigid Body - Personaje principal controlable + Sprites con físicas
//!
//! ✅ JUGADOR (cuadro rojo): Control total con ← → SPACE
//! ✅ RIGID BODIES (sprites PNG): Caen con gravedad, colisionan, son empujados
//! ✅ Input repeat: false (una pulsación = acción)
//! ✅ TTF texto real
//! ✅ 7 plataformas con colisiones

use rydit_gfx::backend_sdl2::Sdl2Backend;
use rydit_gfx::ColorRydit;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::image::LoadSurface;

struct RigidBody {
    nombre: String,
    textura: Option<sdl2::render::Texture<'static>>,
    x: f32, y: f32,
    vx: f32, vy: f32,
    w: u32, h: u32,
    scale: u32,
    color: Color,
    en_suelo: bool,
    empujado: bool,
}

impl RigidBody {
    fn rect(&self) -> Rect {
        Rect::new(self.x as i32, self.y as i32, self.w * self.scale, self.h * self.scale)
    }

    fn aplicar_gravedad(&mut self, dt: f32, gravedad: f32, plataformas: &[Rect], jugador_rect: Rect) {
        // Gravedad
        self.vy += gravedad * dt;
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Fricción
        self.vx *= 0.95;

        let rect = self.rect();
        self.en_suelo = false;

        // Colisión con plataformas
        for plat in plataformas {
            if rect.has_intersection(*plat) {
                if rect.bottom() as i32 <= plat.y + 10 && self.vy > 0.0 {
                    self.y = plat.y as f32 - (self.h * self.scale) as f32;
                    self.vy = 0.0;
                    self.en_suelo = true;
                } else if rect.top() as i32 >= plat.bottom() - 10 && self.vy < 0.0 {
                    self.y = plat.bottom() as f32;
                    self.vy = 0.0;
                }
            }
        }

        // Colisión con JUGADOR (empuje)
        if rect.has_intersection(jugador_rect) {
            // Si el jugador está debajo, empujar hacia arriba
            if jugador_rect.y < rect.y as i32 {
                self.vy = -200.0;
                self.empujado = true;
            }
            // Si el jugador está a los lados, empujar horizontal
            if jugador_rect.x < rect.x as i32 {
                self.vx -= 100.0;
                self.empujado = true;
            } else {
                self.vx += 100.0;
                self.empujado = true;
            }
        }

        // Respawn si cae
        if self.y > 700.0 {
            self.x = 200.0 + (self.x % 400.0);
            self.y = 50.0;
            self.vy = 0.0;
            self.vx = 0.0;
            self.empujado = false;
        }
    }
}

fn main() -> Result<(), String> {
    println!("🛡️ RyDit v0.11.6 - Demo Rigid Body");
    println!("====================================");
    println!("🎮 JUGADOR: ← → = Mover | SPACE = Saltar");
    println!("📦 RIGID BODIES: Caen, colisionan, son empujados");
    println!("====================================\n");

    // Backend
    let mut backend = Sdl2Backend::new("Demo Rigid Body - RyDit", 800, 600)?;

    // Fuente
    for path in &["/system/fonts/DroidSans.ttf", "/data/data/com.termux/files/usr/share/fonts/noto-sans/NotoSans-Regular.ttf"] {
        if std::path::Path::new(path).exists() {
            let _ = backend.load_font(path, 18);
            println!("✅ Fuente: {}", path);
            break;
        }
    }

    // Plataformas
    let plataformas = vec![
        Rect::new(0, 560, 800, 40),
        Rect::new(100, 480, 120, 15),
        Rect::new(300, 420, 120, 15),
        Rect::new(500, 360, 120, 15),
        Rect::new(200, 280, 150, 15),
        Rect::new(450, 220, 120, 15),
        Rect::new(100, 160, 100, 15),
    ];

    // Cargar Rigid Bodies (sprites PNG)
    let sprites_dir = "/data/data/com.termux/files/home/shield-project/logo_icon_asst/sprites";
    println!("\n📦 Cargando rigid bodies...");

    let mut bodies: Vec<RigidBody> = Vec::new();
    let archivos = [
        ("tank", "tank_16x16.png", 150.0f32, 100.0f32, 16u32, 16u32, 4u32, Color::RGB(0,255,0)),
        ("helicopter", "helicopter_16x16.png", 350.0f32, 80.0f32, 16u32, 16u32, 4u32, Color::RGB(0,255,255)),
        ("crate", "crate_8x8.png", 550.0f32, 120.0f32, 8u32, 8u32, 4u32, Color::RGB(139,69,19)),
        ("platform", "platform_16x16.png", 650.0f32, 60.0f32, 16u32, 16u32, 6u32, Color::RGB(128,128,128)),
    ];

    let mut cargados = 0;
    for (nombre, archivo, x, y, w, h, scale, color) in &archivos {
        let path = format!("{}/{}", sprites_dir, archivo);
        print!("  ├─ {}... ", nombre);

        let textura = if std::path::Path::new(&path).exists() {
            match Surface::from_file(&path) {
                Ok(surface) => {
                    match backend.canvas.texture_creator().create_texture_from_surface(&surface) {
                        Ok(tex) => {
                            let tex_static: sdl2::render::Texture<'static> = unsafe { std::mem::transmute(tex) };
                            cargados += 1;
                            println!("✅");
                            Some(tex_static)
                        }
                        Err(e) => { eprintln!("❌ {}", e); None }
                    }
                }
                Err(e) => { eprintln!("❌ {}", e); None }
            }
        } else {
            eprintln!("❌ no existe");
            None
        };

        bodies.push(RigidBody {
            nombre: nombre.to_string(), textura, x: *x, y: *y,
            vx: 0.0, vy: 0.0, w: *w, h: *h, scale: *scale,
            color: *color, en_suelo: false, empujado: false,
        });
    }
    println!("\n✅ {}/4 rigid bodies cargados", cargados);

    // JUGADOR (cuadro rojo controlable)
    let mut j_x = 400.0f32;
    let mut j_y = 500.0f32;
    let mut j_vy = 0.0f32;
    let mut j_en_suelo = false;
    let j_w = 40i32;
    let j_h = 40i32;
    let mut saltos = 0u64;
    let mut empujones = 0u64;

    println!("\n🎮 CONTROLES (como demo_colisiones):");
    println!("   ← → ó A D = Mover jugador");
    println!("   ↑ ↓ ó W S = Subir/bajar");
    println!("   ESPACIO = Saltar");
    println!("   R = Reset rigid bodies");
    println!("   G = Toggle gravedad");
    println!("   ESC = Salir");
    println!("💡 Empuja los rigid bodies con tu cuerpo!\n");

    let mut frame = 0u64;
    let mut running = true;
    let mut gravedad_on = true;

    'running: loop {
        let dt = 0.016f32;
        frame += 1;

        // ================================================================
        // INPUT JUGADOR - PATRÓN DIRECTO (como demo_colisiones.rs)
        // ================================================================
        for event in backend.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                    break 'running;
                }

                Event::KeyDown { keycode: Some(key), repeat: false, .. } => {
                    match key {
                        // SALTO
                        Keycode::Space => {
                            if j_en_suelo {
                                j_vy = -450.0;
                                j_en_suelo = false;
                                saltos += 1;
                            }
                        }
                        // MOVER
                        Keycode::Left | Keycode::A => j_x -= 30.0,
                        Keycode::Right | Keycode::D => j_x += 30.0,
                        Keycode::Up | Keycode::W => j_y -= 30.0,
                        Keycode::Down | Keycode::S => j_y += 30.0,
                        // RESET
                        Keycode::R => {
                            let posiciones = [(150.0, 100.0), (350.0, 80.0), (550.0, 120.0), (650.0, 60.0)];
                            for (i, b) in bodies.iter_mut().enumerate() {
                                b.x = posiciones[i].0;
                                b.y = posiciones[i].1;
                                b.vx = 0.0; b.vy = 0.0;
                                b.en_suelo = false; b.empujado = false;
                            }
                        }
                        // GRAVEDAD
                        Keycode::G => gravedad_on = !gravedad_on,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // ================================================================
        // FÍSICAS JUGADOR
        // ================================================================
        j_vy += 800.0 * dt;
        j_y += j_vy * dt;

        let j_rect = Rect::new(j_x as i32, j_y as i32, j_w as u32, j_h as u32);
        j_en_suelo = false;

        for plat in &plataformas {
            if j_rect.has_intersection(*plat) {
                if j_rect.bottom() as i32 <= plat.y + 10 && j_vy > 0.0 {
                    j_y = plat.y as f32 - j_h as f32;
                    j_vy = 0.0;
                    j_en_suelo = true;
                } else if j_rect.top() as i32 >= plat.bottom() - 10 && j_vy < 0.0 {
                    j_y = plat.bottom() as f32;
                    j_vy = 0.0;
                }
            }
        }

        if j_x < 0.0 { j_x = 0.0; }
        if j_x > 760.0 { j_x = 760.0; }
        if j_y > 620.0 { j_x = 400.0; j_y = 100.0; j_vy = 0.0; }

        // ================================================================
        // FÍSICAS RIGID BODIES
        // ================================================================
        let grav = if gravedad_on { 800.0 } else { 0.0 };
        let mut bodies_empujados = 0;
        for b in &mut bodies {
            b.aplicar_gravedad(dt, grav, &plataformas, j_rect);
            if b.empujado {
                bodies_empujados += 1;
                b.empujado = false; // Reset flag
            }
        }
        empujones += bodies_empujados;

        // ================================================================
        // RENDER
        // ================================================================
        backend.clear_background(ColorRydit::Negro);

        // Estrellas
        backend.canvas.set_draw_color(Color::RGB(40, 40, 60));
        for i in 0..40 {
            let sx = ((i * 137 + frame as usize * 2) % 800) as i32;
            let sy = ((i * 251 + frame as usize / 15) % 600) as i32;
            let _ = backend.canvas.fill_rect(Rect::new(sx, sy, 2, 2));
        }

        // PLATAFORMAS
        backend.canvas.set_draw_color(Color::RGB(80, 80, 100));
        for plat in &plataformas {
            let _ = backend.canvas.fill_rect(*plat);
            // Borde brillante verde (parpadeo)
            if (frame / 30) % 2 == 0 {
                backend.canvas.set_draw_color(Color::RGB(0, 255, 100));
            } else {
                backend.canvas.set_draw_color(Color::RGB(120, 120, 140));
            }
            let _ = backend.canvas.fill_rect(Rect::new(plat.x, plat.y, plat.width(), 3));
            backend.canvas.set_draw_color(Color::RGB(80, 80, 100));
        }

        // RIGID BODIES (sprites)
        for b in &bodies {
            let w = b.w * b.scale;
            let h = b.h * b.scale;

            if let Some(ref tex) = b.textura {
                let _ = backend.canvas.copy(tex, None, Rect::new(b.x as i32, b.y as i32, w, h));
            } else {
                backend.canvas.set_draw_color(b.color);
                let _ = backend.canvas.fill_rect(Rect::new(b.x as i32, b.y as i32, w, h));
            }

            // Indicador brillante verde si está en suelo (parpadeo)
            if b.en_suelo && (frame / 15) % 2 == 0 {
                backend.canvas.set_draw_color(Color::RGB(0, 255, 0));
                let _ = backend.canvas.fill_rect(Rect::new(b.x as i32, (b.y + h as f32 + 2.0) as i32, w, 4));
            }
        }

        // JUGADOR (cuadro rojo con ojos)
        backend.canvas.set_draw_color(Color::RGB(255, 50, 50));
        let _ = backend.canvas.fill_rect(j_rect);
        // Ojos
        backend.canvas.set_draw_color(Color::RGB(255, 255, 255));
        let ojo = if j_x > 400.0 { 24 } else { 4 };
        let _ = backend.canvas.fill_rect(Rect::new(j_x as i32 + ojo, j_y as i32 + 10, 5, 5));
        let _ = backend.canvas.fill_rect(Rect::new(j_x as i32 + ojo + 12, j_y as i32 + 10, 5, 5));

        // Brillo alrededor del jugador (parpadeo verde)
        if (frame / 20) % 2 == 0 {
            backend.canvas.set_draw_color(Color::RGBA(0, 255, 0, 100));
            let _ = backend.canvas.draw_rect(Rect::new(j_x as i32 - 4, j_y as i32 - 4, j_w as u32 + 8, j_h as u32 + 8));
        }

        // TEXTO TTF
        backend.draw_text("🛡️ RyDit - Rigid Body Demo", 15, 15, 18, 255, 255, 255);
        backend.draw_text(&format!("Jugador: ({:.0},{:.0}) | Saltos: {} | Empujones: {}", j_x, j_y, saltos, empujones), 15, 45, 14, 0, 255, 0);
        
        let mut y_off = 75i32;
        let colores = [(0u8, 255, 0), (0, 255, 255), (139, 69, 19), (128, 128, 128)];
        for (i, b) in bodies.iter().enumerate() {
            let (r, g, b_color) = colores[i];
            let st = if b.en_suelo { "✅suelo" } else { "❌caer" };
            backend.draw_text(
                &format!("{}: ({:.0},{:.0}) vy:{:.0} {}", b.nombre, b.x, b.y, b.vy, st),
                15, y_off, 12, r, g, b_color
            );
            y_off += 16;
        }

        backend.draw_text("← → ó A/D = Mover | SPACE = Saltar | G = Gravedad | R = Reset | ESC = Salir", 15, 575, 12, 150, 150, 150);

        backend.end_draw();
    }

    println!("\n✅ Demo: {} frames | Saltos: {} | Empujones: {}", frame, saltos, empujones);
    Ok(())
}
