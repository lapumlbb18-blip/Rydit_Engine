//! demo_ttf_sprites.rs
//! Demo TTF + Sprites - Usa Sdl2Backend de rydit-gfx (que ya tiene TTF linkado)
//!
//! ✅ Sdl2Backend ya tiene TTF + texture_creator funcionando
//! ✅ draw_text() usa SDL2_ttf via FontFFI
//! ✅ Input repeat: false (una pulsación)
//! ✅ Colisiones platformer
//! ✅ Sprites PNG verificados

use rydit_gfx::backend_sdl2::Sdl2Backend;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::image::LoadSurface;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> Result<(), String> {
    println!("🛡️ RyDit v0.11.6 - Demo TTF + Sprites");
    println!("=======================================");
    println!("🎨 Sdl2Backend (ya tiene TTF linkado)");
    println!("📝 draw_text() con FontFFI");
    println!("🖼️  Sprites PNG");
    println!("=======================================\n");

    // Sdl2Backend ya tiene TTF + Image init + texture_creator
    let mut backend = Sdl2Backend::new("Demo TTF + Sprites - RyDit", 800, 600)?;

    // Cargar fuente para draw_text
    let font_paths = [
        "/system/fonts/DroidSans.ttf",
        "/data/data/com.termux/files/usr/share/fonts/noto-sans/NotoSans-Regular.ttf",
        "/data/data/com.termux/files/usr/share/fonts/dejavu/DejaVuSans.ttf",
    ];
    
    let mut fuente_cargada = false;
    for path in &font_paths {
        if std::path::Path::new(path).exists() {
            match backend.load_font(path, 20) {
                Ok(_) => {
                    println!("✅ Fuente cargada: {}", path);
                    fuente_cargada = true;
                    break;
                }
                Err(e) => eprintln!("⚠️  Error con {}: {}", path, e),
            }
        }
    }
    if !fuente_cargada {
        println!("⚠️  Sin fuente TTF, usando fallback");
    }

    // Cargar sprites
    let sprites_dir = "/data/data/com.termux/files/home/shield-project/logo_icon_asst/sprites";
    println!("\n📂 Sprites:");

    struct SpriteData {
        textura: Option<sdl2::render::Texture<'static>>,
        x: f32, y: f32, w: u32, h: u32, color: Color,
    }

    let archivos = [
        ("tank_16x16.png", 100.0f32, 500.0f32, 16u32, 16u32, Color::RGB(0,255,0)),
        ("helicopter_16x16.png", 300.0f32, 100.0f32, 16u32, 16u32, Color::RGB(0,255,255)),
        ("crate_8x8.png", 600.0f32, 500.0f32, 8u32, 8u32, Color::RGB(139,69,19)),
        ("platform_16x16.png", 400.0f32, 400.0f32, 16u32, 16u32, Color::RGB(128,128,128)),
    ];

    let mut sprites: Vec<SpriteData> = Vec::new();
    let mut cargados = 0;

    for (archivo, x, y, w, h, color) in &archivos {
        let path = format!("{}/{}", sprites_dir, archivo);
        print!("  ├─ {}... ", archivo);

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
                        Err(e) => { eprintln!("❌ textura: {}", e); None }
                    }
                }
                Err(e) => { eprintln!("❌ surface: {}", e); None }
            }
        } else {
            eprintln!("❌ no existe");
            None
        };

        sprites.push(SpriteData { textura, x: *x, y: *y, w: *w, h: *h, color: *color });
    }
    println!("\n✅ {}/4 sprites cargados", cargados);

    // Jugador
    let mut j_x = 100.0f32;
    let mut j_y = 500.0f32;
    let mut j_vy = 0.0f32;
    let mut en_suelo = false;
    let j_w = 40i32;
    let j_h = 40i32;

    let plataformas = vec![
        Rect::new(0, 560, 800, 40),
        Rect::new(150, 480, 150, 15),
        Rect::new(400, 400, 150, 15),
        Rect::new(100, 320, 150, 15),
        Rect::new(500, 280, 180, 15),
        Rect::new(250, 200, 120, 15),
    ];

    let mut frame = 0u64;
    let mut saltos = 0u64;
    let mut running = true;

    println!("\n🎮 ← → = Mover | SPACE = Saltar | R = Reset | ESC = Salir\n");

    'running: loop {
        let dt = 0.016f32;
        frame += 1;

        // INPUT (repeat: false solamente)
        if backend.procesar_eventos() { break; }

        if backend.is_key_pressed("space") {
            if en_suelo {
                j_vy = -450.0;
                en_suelo = false;
                saltos += 1;
            }
        }
        if backend.is_key_pressed("a") || backend.is_key_pressed("arrow_left") { j_x -= 30.0; }
        if backend.is_key_pressed("d") || backend.is_key_pressed("arrow_right") { j_x += 30.0; }
        if backend.is_key_pressed("r") {
            j_x = 100.0; j_y = 500.0; j_vy = 0.0; en_suelo = false;
        }

        // FÍSICAS
        j_vy += 800.0 * dt;
        j_y += j_vy * dt;

        let j_rect = Rect::new(j_x as i32, j_y as i32, j_w as u32, j_h as u32);
        en_suelo = false;
        for plat in &plataformas {
            if j_rect.has_intersection(*plat) {
                if j_rect.bottom() as i32 <= plat.y + 10 && j_vy > 0.0 {
                    j_y = plat.y as f32 - j_h as f32;
                    j_vy = 0.0;
                    en_suelo = true;
                } else if j_rect.top() as i32 >= plat.bottom() - 10 && j_vy < 0.0 {
                    j_y = plat.bottom() as f32;
                    j_vy = 0.0;
                }
            }
        }

        if j_x < 0.0 { j_x = 0.0; }
        if j_x > 760.0 { j_x = 760.0; }
        if j_y > 620.0 { j_x = 100.0; j_y = 100.0; j_vy = 0.0; }

        // RENDER
        backend.clear_background(rydit_gfx::ColorRydit::Negro);

        // Plataformas
        backend.canvas.set_draw_color(Color::RGB(100, 100, 120));
        for plat in &plataformas {
            let _ = backend.canvas.fill_rect(*plat);
            backend.canvas.set_draw_color(Color::RGB(150, 150, 170));
            let _ = backend.canvas.fill_rect(Rect::new(plat.x, plat.y, plat.width(), 3));
            backend.canvas.set_draw_color(Color::RGB(100, 100, 120));
        }

        // Sprites
        for s in &sprites {
            let scale = 4;
            if let Some(ref tex) = s.textura {
                let _ = backend.canvas.copy(tex, None, Rect::new(s.x as i32, s.y as i32, s.w * scale, s.h * scale));
            } else {
                backend.canvas.set_draw_color(s.color);
                let _ = backend.canvas.fill_rect(Rect::new(s.x as i32, s.y as i32, s.w * scale, s.h * scale));
            }
        }

        // Jugador
        backend.canvas.set_draw_color(Color::RGB(255, 50, 50));
        let _ = backend.canvas.fill_rect(j_rect);

        // Ojos
        backend.canvas.set_draw_color(Color::RGB(255, 255, 255));
        let ojo = if j_x > 100.0 { 24 } else { 4 };
        let _ = backend.canvas.fill_rect(Rect::new(j_x as i32 + ojo, j_y as i32 + 10, 5, 5));
        let _ = backend.canvas.fill_rect(Rect::new(j_x as i32 + ojo + 12, j_y as i32 + 10, 5, 5));

        // TEXTO con draw_text del backend (usa FontFFI internamente)
        backend.draw_text("🛡️ RyDit - TTF + Sprites + Colisiones", 15, 15, 20, 255, 255, 255);
        backend.draw_text(&format!("Saltos: {} | Sprites: {}/4", saltos, cargados), 15, 45, 16, 0, 255, 0);
        backend.draw_text(&format!("Jugador: ({:.0}, {:.0}) | {}", j_x, j_y, if en_suelo { "✅ suelo" } else { "❌ aire" }), 15, 75, 14, 255, 255, 0);
        backend.draw_text("A/D = Mover | SPACE = Saltar | ESC = Salir", 15, 555, 14, 150, 150, 150);

        backend.end_draw();
    }

    println!("\n✅ Demo: {} frames | Saltos: {}", frame, saltos);
    Ok(())
}
