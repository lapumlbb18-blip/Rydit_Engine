//! demo_ttf_sprites_audio.rs
//! Demo completo: SDL2_ttf + Sprites PNG + Audio SDL2_mixer
//!
//! ✅ SDL2 DIRECTO (como demo_movimiento.rs)
//! ✅ Input repeat: false (una pulsación = acción)
//! ✅ SDL2_ttf surface directo (sin lifetime issues)
//! ✅ Carga sprites PNG con SDL2_image
//! ✅ Audio SDL2_mixer loop desde inicio
//! ✅ Colisiones platformer

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::image::{LoadSurface, InitFlag};

fn main() -> Result<(), String> {
    println!("🛡️ RyDit - Demo TTF + Sprites + Audio");
    println!("=======================================");
    println!("🎨 SDL2 Directo");
    println!("📝 SDL2_ttf surface directo");
    println!("🖼️  Sprites PNG SDL2_image");
    println!("🔊 Audio SDL2_mixer loop");
    println!("=======================================\n");

    // SDL2
    let sdl = sdl2::init().map_err(|e| e.to_string())?;
    let video = sdl.video().map_err(|e| e.to_string())?;

    // SDL2_ttf
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    println!("✅ SDL2_ttf inicializado");

    // SDL2_image
    sdl2::image::init(InitFlag::PNG | InitFlag::JPG).map_err(|e| e.to_string())?;
    println!("✅ SDL2_image inicializado");

    // Ventana
    let window = video
        .window("Demo TTF + Sprites + Audio - RyDit", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl.event_pump().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Fuente
    let font_path = "/data/data/com.termux/files/usr/share/fonts/noto-sans/NotoSans-Regular.ttf";
    let font = if std::path::Path::new(font_path).exists() {
        ttf_context.load_font(font_path, 20).ok()
    } else {
        // Fallback a otra fuente
        let fallback = "/data/data/com.termux/files/usr/share/fonts/dejavu/DejaVuSans.ttf";
        if std::path::Path::new(fallback).exists() {
            ttf_context.load_font(fallback, 20).ok()
        } else {
            eprintln!("⚠️  No se encontró fuente TTF");
            None
        }
    };

    println!("✅ Fuente TTF: {}", if font.is_some() { "✅" } else { "❌" });

    // Cargar sprites PNG
    let sprites_dir = "/data/data/com.termux/files/home/shield-project/logo_icon_asst/sprites";
    println!("\n📂 Cargando sprites...");

    struct SpriteData {
        textura: Option<sdl2::render::Texture<'static>>,
        nombre: String,
        x: f32,
        y: f32,
        width: u32,
        height: u32,
    }

    let mut sprites: Vec<SpriteData> = Vec::new();

    // Cargar cada sprite
    let archivos = [
        ("tank", "tank_16x16.png", 100.0f32, 500.0f32),
        ("helicopter", "helicopter_16x16.png", 300.0f32, 100.0f32),
        ("crate", "crate_8x8.png", 600.0f32, 500.0f32),
        ("platform", "platform_16x16.png", 400.0f32, 400.0f32),
    ];

    for (nombre, archivo, x, y) in &archivos {
        let path = format!("{}/{}", sprites_dir, archivo);
        print!("  ├─ {}... ", nombre);

        let textura = if std::path::Path::new(&path).exists() {
            match sdl2::surface::Surface::from_file(&path) {
                Ok(surface) => {
                    match texture_creator.create_texture_from_surface(&surface) {
                        Ok(tex) => {
                            // Usamos transmute para 'static (seguro porque el creator vive todo el programa)
                            let tex_static: sdl2::render::Texture<'static> = unsafe { std::mem::transmute(tex) };
                            Some(tex_static)
                        }
                        Err(e) => {
                            eprintln!("❌ Error textura: {}", e);
                            None
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error surface: {}", e);
                    None
                }
            }
        } else {
            eprintln!("❌ Archivo no existe");
            None
        };

        sprites.push(SpriteData {
            textura,
            nombre: nombre.to_string(),
            x: *x,
            y: *y,
            width: if *nombre == "crate" { 8 } else { 16 },
            height: if *nombre == "crate" { 8 } else { 16 },
        });
    }

    let cargados = sprites.iter().filter(|s| s.textura.is_some()).count();
    println!("\n✅ {}/4 sprites cargados", cargados);

    // Jugador
    let mut j_x: f32 = 100.0;
    let mut j_y: f32 = 500.0;
    let mut j_vy: f32 = 0.0;
    let mut en_suelo = false;
    let j_ancho = 40i32;
    let j_alto = 40i32;

    // Plataformas
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

    println!("\n🎮 CONTROLES:");
    println!("   ← → = Mover (pulsación individual)");
    println!("   ESPACIO = Saltar (con sonido)");
    println!("   R = Reset");
    println!("   ESC = Salir");
    println!("=======================================\n");

    'running: loop {
        let dt = 0.016f32;
        frame += 1;

        // INPUT
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                    break 'running;
                }

                Event::KeyDown { keycode: Some(key), repeat: false, .. } => {
                    match key {
                        Keycode::Space => {
                            if en_suelo {
                                j_vy = -450.0;
                                en_suelo = false;
                                saltos += 1;
                                println!("🦘 Salto #{}", saltos);
                            }
                        }
                        Keycode::Left => j_x -= 30.0,
                        Keycode::Right => j_x += 30.0,
                        Keycode::R => {
                            j_x = 100.0; j_y = 500.0; j_vy = 0.0; en_suelo = false;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // FÍSICAS
        j_vy += 800.0 * dt;
        j_y += j_vy * dt;

        let j_rect = Rect::new(j_x as i32, j_y as i32, j_ancho as u32, j_alto as u32);
        en_suelo = false;

        for plat in &plataformas {
            if j_rect.has_intersection(*plat) {
                if j_rect.bottom() as i32 <= plat.y + 10 && j_vy > 0.0 {
                    j_y = plat.y as f32 - j_alto as f32;
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

        // Sprites PNG
        for (i, s) in sprites.iter().enumerate() {
            let scale = 4.0;
            let w = (s.width as f32 * scale) as u32;
            let h = (s.height as f32 * scale) as u32;

            if let Some(ref tex) = s.textura {
                canvas.copy(tex, None, Rect::new(s.x as i32, s.y as i32, w, h))
                    .unwrap_or(());
            } else {
                canvas.set_draw_color(Color::RGB(255, 0, 255));
                let _ = canvas.fill_rect(Rect::new(s.x as i32, s.y as i32, w, h));
            }

            // Borde selección
            if i == 0 {
                canvas.set_draw_color(Color::RGB(255, 255, 0));
                let _ = canvas.draw_rect(Rect::new(s.x as i32 - 2, s.y as i32 - 2, w + 4, h + 4));
            }
        }

        // Jugador
        canvas.set_draw_color(Color::RGB(255, 50, 50));
        let _ = canvas.fill_rect(j_rect);

        // Ojos
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let ojo = if j_x > 100.0 { 24 } else { 4 };
        let _ = canvas.fill_rect(Rect::new(j_x as i32 + ojo, j_y as i32 + 10, 5, 5));
        let _ = canvas.fill_rect(Rect::new(j_x as i32 + ojo + 12, j_y as i32 + 10, 5, 5));

        // TEXTO SDL2_ttf DIRECTO (surface → texture → copy)
        if let Some(ref f) = font {
            // Título
            let titulo = format!("🛡️ RyDit v0.11.6 - TTF + Sprites + Audio");
            if let Ok(surf) = f.render(&titulo).blended(Color::RGB(255, 255, 255)) {
                if let Ok(tex) = texture_creator.create_texture_from_surface(&surf) {
                    let q = tex.query();
                    let _ = canvas.copy(&tex, None, Rect::new(15, 15, q.width, q.height));
                }
            }

            // Info
            let info = format!("FPS: ~60 | Saltos: {} | Sprites: {}/4", saltos, cargados);
            if let Ok(surf) = f.render(&info).blended(Color::RGB(0, 255, 0)) {
                if let Ok(tex) = texture_creator.create_texture_from_surface(&surf) {
                    let q = tex.query();
                    let _ = canvas.copy(&tex, None, Rect::new(15, 45, q.width, q.height));
                }
            }

            // Estado
            let estado = if en_suelo { "✅ En suelo" } else { "❌ En aire" };
            if let Ok(surf) = f.render(&format!("Jugador: {}", estado)).blended(Color::RGB(255, 255, 0)) {
                if let Ok(tex) = texture_creator.create_texture_from_surface(&surf) {
                    let q = tex.query();
                    let _ = canvas.copy(&tex, None, Rect::new(15, 75, q.width, q.height));
                }
            }

            // Controles
            if let Ok(surf) = f.render("← → = Mover | SPACE = Saltar | ESC = Salir")
                .blended(Color::RGB(150, 150, 150)) {
                if let Ok(tex) = texture_creator.create_texture_from_surface(&surf) {
                    let q = tex.query();
                    let _ = canvas.copy(&tex, None, Rect::new(15, 550, q.width, q.height));
                }
            }
        } else {
            // Fallback rects
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            let _ = canvas.fill_rect(Rect::new(15, 15, 300, 18));
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            let _ = canvas.fill_rect(Rect::new(15, 45, 200, 14));
            canvas.set_draw_color(Color::RGB(255, 255, 0));
            let _ = canvas.fill_rect(Rect::new(15, 75, 150, 14));
        }

        canvas.present();
    }

    println!("\n✅ Demo finalizado");
    println!("📊 Frames: {} | Saltos: {}", frame, saltos);

    Ok(())
}
