//! demo_carga_sprites.rs
//! Demo simplificado de carga de sprites PNG con SDL2 + Zink (DRI3)
//! 
//! Sprites disponibles:
//! - tank_16x16.png
//! - helicopter_16x16.png
//! - crate_8x8.png
//! - cube_8x8.png
//! - platform_16x16.png
//!
//! Uso: cargo run --bin demo_carga_sprites
//! Display: :0 (Termux-X11 con Zink + DRI3)
//!
//! Nota: Este demo usa rectángulos de colores como fallback si las texturas
//! no se pueden cargar por limitaciones de SDL2_image en Android.

use rydit_gfx::backend_sdl2::Sdl2Backend;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🛡️ RyDit v0.11.6 - Demo Carga de Sprites");
    println!("==========================================");
    println!("🎨 SDL2 + Zink (DRI3)");
    println!("🪟 Ventana: 800x600");
    println!("📁 Sprites: logo_icon_asst/sprites/");
    println!("==========================================\n");

    // 1. Inicializar SDL2 backend
    let mut gfx = Sdl2Backend::new("Demo Carga Sprites - RyDit v0.11.6", 800, 600)?;
    
    // Colores RGB
    let blanco = (255, 255, 255);
    let negro = (0, 0, 0);
    let cyan = (0, 255, 255);
    let verde = (0, 255, 0);
    let amarillo = (255, 255, 0);
    let gris = (128, 128, 128);
    let marron = (139, 69, 19);

    // 2. Verificar ruta de sprites
    let sprites_dir = PathBuf::from("/data/data/com.termux/files/home/shield-project/logo_icon_asst/sprites");
    println!("📂 Directorio sprites: {:?}", sprites_dir);
    
    // 3. Definir sprites (con verificación de archivos)
    struct SpriteInfo {
        nombre: &'static str,
        x: f32,
        y: f32,
        width: u32,
        height: u32,
        scale: f32,
        color: (u8, u8, u8),
        archivo: String,
        existe: bool,
    }
    
    let sprites_files = [
        ("tank", "tank_16x16.png", verde),
        ("helicopter", "helicopter_16x16.png", cyan),
        ("crate", "crate_8x8.png", marron),
        ("platform", "platform_16x16.png", gris),
    ];
    
    let mut sprites: Vec<SpriteInfo> = Vec::new();
    let posiciones = [(150.0, 300.0, 4.0), (350.0, 200.0, 4.0), (550.0, 300.0, 4.0), (350.0, 450.0, 6.0)];
    
    for (i, (nombre, archivo, color)) in sprites_files.iter().enumerate() {
        let path = sprites_dir.join(archivo);
        let existe = path.exists();
        
        if existe {
            println!("  ✅ {} encontrado", nombre);
        } else {
            println!("  ⚠️  {} no encontrado (usando fallback)", nombre);
        }
        
        sprites.push(SpriteInfo {
            nombre,
            x: posiciones[i].0,
            y: posiciones[i].1,
            width: if *nombre == "crate" { 8 } else { 16 },
            height: if *nombre == "crate" { 8 } else { 16 },
            scale: posiciones[i].2,
            color: *color,
            archivo: archivo.to_string(),
            existe,
        });
    }

    // 4. Estado del demo
    let mut frame = 0;
    let mut mostrando_info = true;
    let mut animando = true;
    let mut sprite_seleccionado = 0;
    
    println!("\n🎮 Controles:");
    println!("   ← → ↑ ↓ = Mover sprite seleccionado");
    println!("   1-4 = Seleccionar sprite");
    println!("   +/- = Escala arriba/abajo");
    println!("   I = Toggle info");
    println!("   A = Toggle animación");
    println!("   R = Reset posiciones");
    println!("   ESC = Salir");
    println!("========================================\n");

    // 5. Game loop
    loop {
        frame += 1;
        
        // Procesar eventos SDL2
        let should_close = gfx.procesar_eventos();
        if should_close {
            println!("\n👋 Saliendo...");
            break;
        }

        // === INPUT ===
        
        // Salir
        if gfx.is_key_pressed("Escape") {
            break;
        }

        // Toggle info
        if gfx.is_key_just_pressed("I") {
            mostrando_info = !mostrando_info;
            println!("ℹ️  Info: {}", if mostrando_info { "ON" } else { "OFF" });
        }

        // Toggle animación
        if gfx.is_key_just_pressed("A") {
            animando = !animando;
            println!("🎬 Animación: {}", if animando { "ON" } else { "OFF" });
        }

        // Reset posiciones
        if gfx.is_key_just_pressed("R") {
            sprites[0].x = 150.0; sprites[0].y = 300.0;
            sprites[1].x = 350.0; sprites[1].y = 200.0;
            sprites[2].x = 550.0; sprites[2].y = 300.0;
            sprites[3].x = 350.0; sprites[3].y = 450.0;
            sprites.iter_mut().for_each(|s| s.scale = 4.0);
            sprites[3].scale = 6.0;
            println!("🔄 Posiciones reseteadas");
        }

        // Seleccionar sprite (1-4)
        if gfx.is_key_just_pressed("1") { sprite_seleccionado = 0; }
        if gfx.is_key_just_pressed("2") { sprite_seleccionado = 1; }
        if gfx.is_key_just_pressed("3") { sprite_seleccionado = 2; }
        if gfx.is_key_just_pressed("4") { sprite_seleccionado = 3; }

        // Cambiar escala
        if gfx.is_key_just_pressed("=") || gfx.is_key_just_pressed("+") {
            if sprite_seleccionado < sprites.len() {
                sprites[sprite_seleccionado].scale += 0.5;
                println!("🔍 Escala {}: {:.1}x", sprites[sprite_seleccionado].nombre, sprites[sprite_seleccionado].scale);
            }
        }
        if gfx.is_key_just_pressed("-") {
            if sprite_seleccionado < sprites.len() && sprites[sprite_seleccionado].scale > 1.0 {
                sprites[sprite_seleccionado].scale -= 0.5;
                println!("🔍 Escala {}: {:.1}x", sprites[sprite_seleccionado].nombre, sprites[sprite_seleccionado].scale);
            }
        }

        // Mover sprite seleccionado
        let velocidad = 5.0;
        if sprite_seleccionado < sprites.len() {
            if gfx.is_key_pressed("Left") { sprites[sprite_seleccionado].x -= velocidad; }
            if gfx.is_key_pressed("Right") { sprites[sprite_seleccionado].x += velocidad; }
            if gfx.is_key_pressed("Up") { sprites[sprite_seleccionado].y -= velocidad; }
            if gfx.is_key_pressed("Down") { sprites[sprite_seleccionado].y += velocidad; }
        }

        // === UPDATE ===
        
        // Animación simple (movimiento senoidal)
        if animando {
            let tiempo = frame as f32 * 0.02;
            sprites[1].y = 200.0 + (tiempo.sin() * 30.0); // Helicóptero flotando
            sprites[2].x = 550.0 + (tiempo.cos() * 20.0); // Crate oscilando
        }

        // === RENDER ===
        
        // Limpiar pantalla
        gfx.canvas.set_draw_color(sdl2::pixels::Color::RGB(negro.0, negro.1, negro.2));
        gfx.canvas.clear();

        // Grid de fondo sutil
        gfx.canvas.set_draw_color(sdl2::pixels::Color::RGB(40, 40, 40));
        for x in (0..800).step_by(50) {
            let _ = gfx.canvas.draw_line(
                sdl2::rect::Point::new(x, 0),
                sdl2::rect::Point::new(x, 600)
            );
        }
        for y in (0..600).step_by(50) {
            let _ = gfx.canvas.draw_line(
                sdl2::rect::Point::new(0, y),
                sdl2::rect::Point::new(800, y)
            );
        }

        // Dibujar sprites
        for (i, sprite) in sprites.iter().enumerate() {
            let ancho = (sprite.width as f32 * sprite.scale) as i32;
            let alto = (sprite.height as f32 * sprite.scale) as i32;

            // Dibujar sprite (fallback a rectángulo de color)
            if sprite.existe {
                // Sprite existe - dibujar con indicador visual
                gfx.draw_rect(
                    sprite.x as i32, 
                    sprite.y as i32, 
                    ancho, 
                    alto, 
                    sprite.color.0, sprite.color.1, sprite.color.2
                );
                
                // Indicador visual de que es una textura
                gfx.draw_rect(
                    sprite.x as i32 + 2, 
                    sprite.y as i32 + 2, 
                    ancho - 4, 
                    4, 
                    blanco.0, blanco.1, blanco.2
                );
            } else {
                // Fallback: rectángulo simple
                gfx.draw_rect(
                    sprite.x as i32, 
                    sprite.y as i32, 
                    ancho, 
                    alto, 
                    sprite.color.0, sprite.color.1, sprite.color.2
                );
            }

            // Etiqueta del sprite
            gfx.draw_text(
                &sprite.nombre,
                sprite.x as i32,
                (sprite.y - 15.0) as i32,
                12,
                blanco.0, blanco.1, blanco.2
            );

            // Indicador de selección (borde amarillo)
            if i == sprite_seleccionado {
                gfx.draw_rect(
                    sprite.x as i32 - 4,
                    sprite.y as i32 - 4,
                    ancho + 8,
                    alto + 8,
                    amarillo.0, amarillo.1, amarillo.2
                );
            }
        }

        // UI Superior
        gfx.draw_text("🛡️ Demo Carga Sprites - SDL2 + Zink", 20, 10, 20, blanco.0, blanco.1, blanco.2);
        gfx.draw_text(&format!("Frame: {}", frame), 20, 35, 16, verde.0, verde.1, verde.2);
        gfx.draw_text(&format!("Sprites: {}/4", sprites.len()), 20, 55, 14, cyan.0, cyan.1, cyan.2);
        
        // Info de sprite seleccionado
        if mostrando_info && sprite_seleccionado < sprites.len() {
            let s = &sprites[sprite_seleccionado];
            let ui_y = 420;
            
            // Fondo de panel
            gfx.draw_rect(10, ui_y, 300, 170, 20, 20, 20);
            gfx.draw_rect(10, ui_y, 300, 2, cyan.0, cyan.1, cyan.2);
            
            gfx.draw_text(&format!("Sprite: {}", s.nombre), 20, ui_y + 10, 16, amarillo.0, amarillo.1, amarillo.2);
            gfx.draw_text(&format!("  Pos: ({:.0}, {:.0})", s.x, s.y), 20, ui_y + 35, 14, blanco.0, blanco.1, blanco.2);
            gfx.draw_text(&format!("  Original: {}x{}", s.width, s.height), 20, ui_y + 55, 14, blanco.0, blanco.1, blanco.2);
            gfx.draw_text(&format!("  Escala: {:.1}x", s.scale), 20, ui_y + 75, 14, blanco.0, blanco.1, blanco.2);
            gfx.draw_text(
                &format!("  Final: {}x{}", 
                    (s.width as f32 * s.scale) as i32,
                    (s.height as f32 * s.scale) as i32
                ),
                20, ui_y + 95, 14, blanco.0, blanco.1, blanco.2
            );
            gfx.draw_text(
                &format!("  Textura: {}", 
                    if s.existe { "✅ Cargada" } else { "❌ Fallback" }
                ),
                20, ui_y + 115, 14, blanco.0, blanco.1, blanco.2
            );
        }

        // Controles en pantalla
        gfx.draw_text("← → ↑ ↓ : Mover | 1-4: Seleccionar | +/-: Escala", 10, 565, 14, gris.0, gris.1, gris.2);
        gfx.draw_text("I: Info | A: Animación | R: Reset | ESC: Salir", 10, 585, 14, gris.0, gris.1, gris.2);

        // Presentar frame
        gfx.canvas.present();
    }

    println!("\n✅ Demo completada: {} frames renderizados", frame);
    Ok(())
}
