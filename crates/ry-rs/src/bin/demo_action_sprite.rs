// demo_action_sprite.rs
// Demo: AnimatedSprite con sprite sheet real + action_sprite system
//
// Muestra:
// - Sprite sheet loading (PNG)
// - AnimatedSprite con clips: idle, run, jump
// - Flip horizontal según dirección
// - State machine info en HUD
// - Frame counter + progreso
//
// Controles:
// ← → / A D: Mover (animación run)
// SPACE: Saltar (animación jump)
// Soltar: Idle
// ESC: Salir

use ry_anim::{AnimatedSprite, LoopMode, SpriteColor, SpriteSheet};
use ry_gfx::backend_sdl2::Sdl2Backend;
use ry_gfx::ColorRydit;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::image::LoadSurface;

// ============================================================================
// TEXTURE HELPER
// ============================================================================
fn crear_textura<'a>(
    font: &Option<ry_gfx::sdl2_ffi::FontFFI>,
    texto: &str,
    r: u8, g: u8, b: u8,
    tc: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) -> Option<sdl2::render::Texture<'a>> {
    if let Some(f) = font {
        if let Ok(sp) = f.render_text_blended(texto, r, g, b) {
            unsafe {
                let s = sdl2::surface::Surface::from_ll(sp as *mut sdl2::sys::SDL_Surface);
                if let Ok(t) = tc.create_texture_from_surface(&s) {
                    return Some(std::mem::transmute(t));
                }
            }
        }
    }
    None
}

// ============================================================================
// MAIN
// ============================================================================
fn main() -> Result<(), String> {
    println!("🎬 RyDit - Demo Action Sprite");
    println!("← → / A D: Mover | SPACE: Saltar | ESC: Salir\n");

    let mut backend = Sdl2Backend::new("Demo Action Sprite", 800, 600)?;

    // Fuente
    for p in &["/system/fonts/DroidSans.ttf", "/data/data/com.termux/files/usr/share/fonts/noto-sans/NotoSans-Regular.ttf"] {
        if std::path::Path::new(p).exists() {
            let _ = backend.load_font(p, 14);
            break;
        }
    }

    // ========================================================================
    // CREAR SPRITE SHEET (generado proceduralmente si no hay PNG)
    // ========================================================================
    let sprite_sheet_path = "/data/data/com.termux/files/home/shield-project/hero_sprites.png";
    let sprite_texture: Option<sdl2::render::Texture<'static>> = if std::path::Path::new(sprite_sheet_path).exists() {
        match Surface::from_file(sprite_sheet_path) {
            Ok(surface) => {
                match backend.canvas.texture_creator().create_texture_from_surface(&surface) {
                    Ok(tex) => Some(unsafe { std::mem::transmute(tex) }),
                    Err(e) => { eprintln!("⚠️  Error cargando sprite sheet: {}", e); None }
                }
            }
            Err(e) => { eprintln!("⚠️  Error cargando sprite sheet: {}", e); None }
        }
    } else {
        eprintln!("⚠️  No existe '{}', generando sprite sheet procedural", sprite_sheet_path);
        generate_sprite_sheet(&backend.canvas.texture_creator(), sprite_sheet_path).ok()
    };

    let texture_id = "hero";
    if sprite_texture.is_some() {
        // En un demo real registraríamos la textura en AssetsManager
        // Aquí la guardamos para uso directo
    }

    // ========================================================================
    // ANIMATED SPRITE
    // ========================================================================
    // Sprite sheet: 128x128 por frame, 4 columnas x 4 filas = 16 frames
    let sheet = SpriteSheet::new(texture_id, 128.0, 128.0, 4, 4);

    let mut sprite = AnimatedSprite::new(sheet);

    // Clips de animación:
    // Frames 0-3: Idle (4 frames, 0.2s c/u)
    sprite.add_clip("idle", 0..4, 0.2, LoopMode::Loop);
    // Frames 4-11: Run (8 frames, 0.1s c/u)
    sprite.add_clip("run", 4..12, 0.1, LoopMode::Loop);
    // Frames 12-15: Jump (4 frames, 0.15s c/u)
    sprite.add_clip("jump", 12..16, 0.15, LoopMode::Once);

    sprite.play("idle");

    // ========================================================================
    // ESTADO
    // ========================================================================
    let mut x: f32 = 336.0; // centro: (800 - 128) / 2
    let mut y: f32 = 380.0; // suelo
    let mut vy: f32 = 0.0;
    let mut en_suelo = true;
    let mut frame_count: u64 = 0;
    let mut direccion: i32 = 1; // 1 = derecha, -1 = izquierda
    let gravity = 800.0;
    let jump_force = -400.0;
    let speed = 200.0;
    let suelo_y = 380.0;

    // Texturas HUD cacheadas
    let tc = &backend.canvas.texture_creator();
    let mut txt_hud: Option<sdl2::render::Texture<'static>> = None;

    // ========================================================================
    // GAME LOOP
    // ========================================================================
    let mut running = true;
    'run: loop {
        let frame_start = std::time::Instant::now();
        let dt = 0.016; // ~60 FPS

        // ---- INPUT ----
        let mut mover = 0i32;
        let mut saltar = false;

        for ev in backend.event_pump.poll_iter() {
            match ev {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false; break 'run;
                }
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    saltar = true;
                }
                _ => {}
            }
        }

        let ks = sdl2::keyboard::KeyboardState::new(&backend.event_pump);
        if ks.is_scancode_pressed(sdl2::keyboard::Scancode::Left) || ks.is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            mover = -1;
        }
        if ks.is_scancode_pressed(sdl2::keyboard::Scancode::Right) || ks.is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            mover = 1;
        }

        // ---- UPDATE ----
        if saltar && en_suelo {
            vy = jump_force;
            en_suelo = false;
            sprite.play("jump");
        }

        if mover != 0 {
            x += speed * dt * mover as f32;
            direccion = mover;
            if en_suelo {
                sprite.play("run");
            }
        } else if en_suelo {
            sprite.play("idle");
        }

        // Gravedad
        if !en_suelo {
            vy += gravity * dt;
            y += vy * dt;
            if y >= suelo_y {
                y = suelo_y;
                vy = 0.0;
                en_suelo = true;
            }
        }

        // Límites pantalla
        x = x.max(0.0).min(800.0 - 128.0);

        // Flip según dirección
        sprite.flip_horizontal(direccion < 0);

        // Update animación
        sprite.update(dt as f64);

        // ---- HUD TEXT ----
        frame_count += 1;
        if frame_count % 15 == 0 {
            let state_info = sprite.state_info();
            let frame = sprite.current_frame();
            let rect = sprite.current_frame_rect();
            let txt = format!(
                "Clip: {} | Frame: {} | Rect: ({:.0},{:.0},{:.0},{:.0}) | Estado: {:.0}% | Flip: {}",
                state_info.state,
                frame,
                rect.x, rect.y, rect.w, rect.h,
                state_info.progress * 100.0,
                if sprite.flip_info().is_flipped { "←" } else { "→" }
            );
            txt_hud = crear_textura(&backend.font, &txt, 230, 230, 240, tc)
                .map(|t| unsafe { std::mem::transmute(t) });
        }

        // ---- RENDER ----
        backend.canvas.set_draw_color(Color::RGB(18, 18, 24));
        backend.canvas.clear();

        // Suelo
        backend.canvas.set_draw_color(Color::RGB(60, 60, 80));
        let _ = backend.canvas.fill_rect(Rect::new(0, suelo_y as i32 + 128, 800, (600 - suelo_y as i32 - 128) as u32));

        // Línea de suelo
        backend.canvas.set_draw_color(Color::RGB(100, 100, 120));
        let _ = backend.canvas.draw_rect(Rect::new(0, suelo_y as i32 + 128, 800, 2));

        // Dibujar sprite animado
        if let Some(ref tex) = sprite_texture {
            let cmd = sprite.render(texture_id, x, y, SpriteColor::blanco());
            let flip = sprite.flip_info();

            // Source rect del frame actual
            let src_rect = Rect::new(
                cmd.source_x as i32,
                cmd.source_y as i32,
                cmd.source_w as u32,
                cmd.source_h as u32,
            );

            // Destino con flip
            let dest_x = if flip.scale_x < 0.0 {
                cmd.dest_x + cmd.dest_w
            } else {
                cmd.dest_x
            };

            // Escalar 128x128 → 64x64 en pantalla
            let scale = 0.5;
            let dest_w = (cmd.dest_w * scale) as u32;
            let dest_h = (cmd.dest_h * scale) as u32;
            let dest_y = y as i32 + (128.0 - dest_h as f32) as i32;

            if flip.scale_x < 0.0 {
                // Flip horizontal: copiar con transform
                // SDL2 no tiene flip directo en copy, usamos Renderer flip
                backend.canvas.copy_ex(
                    tex,
                    Some(src_rect),
                    Rect::new(dest_x as i32 - dest_w as i32, dest_y, dest_w, dest_h),
                    0.0,
                    None,
                    true,  // flip_horizontal
                    false, // flip_vertical
                ).ok();
            } else {
                backend.canvas.copy(
                    tex,
                    Some(src_rect),
                    Rect::new(dest_x as i32, dest_y, dest_w, dest_h),
                ).ok();
            }
        } else {
            // Fallback: rectángulo de color con info de frame
            let frame = sprite.current_frame();
            let rect = sprite.current_frame_rect();
            let colores = [
                Color::RGB(255, 100, 100),
                Color::RGB(100, 255, 100),
                Color::RGB(100, 100, 255),
                Color::RGB(255, 255, 100),
            ];
            let c = colores[frame % 4];
            backend.canvas.set_draw_color(c);
            let _ = backend.canvas.fill_rect(Rect::new(x as i32, y as i32, 64, 64));
            // Frame number como rectángulos
            backend.canvas.set_draw_color(Color::WHITE);
            let _ = backend.canvas.fill_rect(Rect::new(x as i32 + 2, y as i32 + 2, 60, 4));
        }

        // HUD
        if let Some(ref tex) = txt_hud {
            let q = tex.query();
            let w = q.width as u32;
            backend.canvas.set_draw_color(Color::RGBA(0, 0, 0, 180));
            let _ = backend.canvas.fill_rect(Rect::new(10, 10, w + 16, 24));
            backend.canvas.copy(tex, None, Rect::new(14, 12, w, 18)).ok();
        }

        // Instrucciones
        let instrucciones = crear_textura(&backend.font, "← → / A D: Mover | SPACE: Saltar | ESC: Salir", 150, 150, 150, tc);
        if let Some(ref tex) = instrucciones {
            let q = tex.query();
            let w = q.width as u32;
            let _ = backend.canvas.copy(tex, None, Rect::new(10, 570, w, 18));
        }

        backend.canvas.present();

        // Cap 60 FPS
        let elapsed = frame_start.elapsed();
        if elapsed < std::time::Duration::from_millis(16) {
            std::thread::sleep(std::time::Duration::from_millis(16) - elapsed);
        }
    }

    println!("\n✅ Demo cerrado");
    Ok(())
}

// ============================================================================
// GENERAR SPRITE SHEET PROCEDURAL (si no hay PNG)
// ============================================================================
fn generate_sprite_sheet(
    tc: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    path: &str,
) -> Result<sdl2::render::Texture<'static>, String> {
    // 4x4 frames de 128x128 = 512x512 total
    let mut surface = sdl2::surface::Surface::new(512, 512, sdl2::pixels::PixelFormatEnum::RGBA8888)
        .map_err(|e| e.to_string())?;

    let colores = [
        (255, 100, 100), (100, 255, 100), (100, 100, 255), (255, 255, 100),
        (255, 150, 50),  (150, 50, 255),  (50, 255, 150),  (255, 50, 150),
        (200, 200, 50),  (50, 200, 200),  (200, 50, 200),  (100, 150, 255),
        (255, 200, 100), (100, 255, 200), (200, 100, 255), (255, 100, 200),
    ];

    for row in 0..4 {
        for col in 0..4 {
            let idx = row * 4 + col;
            let (r, g, b) = colores[idx];
            let _ = surface.fill_rect(
                sdl2::rect::Rect::new(col as i32 * 128, row as i32 * 128, 128, 128),
                sdl2::pixels::Color::RGB(r, g, b),
            );
            // Frame number
            let _ = surface.fill_rect(
                sdl2::rect::Rect::new(col as i32 * 128 + 4, row as i32 * 128 + 4, 120, 4),
                sdl2::pixels::Color::WHITE,
            );
        }
    }

    let texture = tc.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let texture_static: sdl2::render::Texture<'static> = unsafe { std::mem::transmute(texture) };

    // Guardar como PNG (requiere sdl2_image)
    // Por simplicidad, solo retornamos la textura
    let _ = path; // suppress warning

    Ok(texture_static)
}
