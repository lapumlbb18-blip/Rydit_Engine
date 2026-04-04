//! demo_sprites_v2.rs  
//! Demo simplificado - Sprites con verificación de carga SDL2
//!
//! ✅ Verifica que los archivos PNG existen
//! ✅ Input SDL2 correcto (69 teclas mapeadas)
//! ✅ Animación fluida
//! ✅ Sin problemas de lifetime
//!
//! NOTA: Las texturas SDL2 tienen lifetimes complejos.
//! Este demo verifica archivos y usará rects de colores.
//! Para texturas reales se necesita un gestor de assets dedicado.
//!
//! Uso: cargo run --bin demo_sprites_v2 --release

use ry_gfx::backend_sdl2::Sdl2Backend;
use std::path::Path;

struct Sprite {
    nombre: String,
    archivo: String,
    existe: bool,
    x: i32,
    y: i32,
    size: u32,
    color: (u8, u8, u8),
}

fn main() -> Result<(), String> {
    println!("🛡️ RyDit v0.11.6 - Demo Sprites V2");
    println!("====================================");
    println!("🎨 SDL2 + Zink (DRI3)");
    println!("📂 Verificación archivos PNG");
    println!("🎮 Input SDL2 (69 teclas)");
    println!("====================================\n");

    let mut gfx = Sdl2Backend::new("Demo Sprites V2", 800, 600)?;
    
    let sprites_dir = "/data/data/com.termux/files/home/shield-project/logo_icon_asst/sprites";
    
    // Definir sprites
    let mut sprites = vec![
        Sprite { nombre: "tank".into(), archivo: "tank_16x16.png".into(), existe: false, x: 150, y: 300, size: 64, color: (0, 255, 0) },
        Sprite { nombre: "helicopter".into(), archivo: "helicopter_16x16.png".into(), existe: false, x: 350, y: 200, size: 64, color: (0, 255, 255) },
        Sprite { nombre: "crate".into(), archivo: "crate_8x8.png".into(), existe: false, x: 550, y: 300, size: 32, color: (139, 69, 19) },
        Sprite { nombre: "platform".into(), archivo: "platform_16x16.png".into(), existe: false, x: 350, y: 450, size: 96, color: (128, 128, 128) },
    ];
    
    // Verificar archivos
    println!("📂 Verificando sprites:");
    for s in &mut sprites {
        let path = format!("{}/{}", sprites_dir, s.archivo);
        s.existe = Path::new(&path).exists();
        println!("  ├─ {}... {}", s.archivo, if s.existe { "✅" } else { "❌" });
    }
    
    let existentes = sprites.iter().filter(|s| s.existe).count();
    println!("\n✅ {}/4 archivos encontrados\n", existentes);
    
    let mut sel = 0;
    let mut frame = 0;
    let mut anim = true;
    
    println!("🎮 ← → ↑ ↓=Mover | 1-4=Select | A=Anim | R=Reset | ESC=Salir");

    loop {
        frame += 1;
        
        if gfx.procesar_eventos() { break; }
        if gfx.is_key_pressed("escape") { break; }
        
        if gfx.is_key_pressed("a") { anim = !anim; }
        
        if gfx.is_key_pressed("r") {
            sprites[0].x = 150; sprites[0].y = 300;
            sprites[1].x = 350; sprites[1].y = 200;
            sprites[2].x = 550; sprites[2].y = 300;
            sprites[3].x = 350; sprites[3].y = 450;
            sel = 0;
        }
        
        if gfx.is_key_pressed("1") { sel = 0; }
        if gfx.is_key_pressed("2") { sel = 1; }
        if gfx.is_key_pressed("3") { sel = 2; }
        if gfx.is_key_pressed("4") { sel = 3; }
        
        let v = 5;
        if gfx.is_key_pressed("arrow_left") { sprites[sel].x -= v; }
        if gfx.is_key_pressed("arrow_right") { sprites[sel].x += v; }
        if gfx.is_key_pressed("arrow_up") { sprites[sel].y -= v; }
        if gfx.is_key_pressed("arrow_down") { sprites[sel].y += v; }
        
        if anim {
            let t = (frame as f32 * 0.03).sin();
            sprites[1].y = 200 + (t * 30.0) as i32;
            sprites[2].x = 550 + (t.cos() * 20.0) as i32;
        }
        
        // Render
        gfx.canvas.set_draw_color(sdl2::pixels::Color::RGB(10, 10, 20));
        gfx.canvas.clear();
        
        // Grid
        gfx.canvas.set_draw_color(sdl2::pixels::Color::RGB(30, 30, 40));
        for x in (0..800).step_by(50) {
            let _ = gfx.canvas.draw_line(sdl2::rect::Point::new(x, 0), sdl2::rect::Point::new(x, 600));
        }
        for y in (0..600).step_by(50) {
            let _ = gfx.canvas.draw_line(sdl2::rect::Point::new(0, y), sdl2::rect::Point::new(800, y));
        }
        
        // Sprites
        for (i, s) in sprites.iter().enumerate() {
            let sz = s.size as i32;
            
            // Rect de color
            gfx.canvas.set_draw_color(sdl2::pixels::Color::RGB(s.color.0, s.color.1, s.color.2));
            let _ = gfx.canvas.fill_rect(sdl2::rect::Rect::new(s.x, s.y, sz as u32, sz as u32));
            
            // Indicador si existe archivo
            if s.existe {
                gfx.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
                let _ = gfx.canvas.fill_rect(sdl2::rect::Rect::new(s.x + 2, s.y + 2, (sz - 4) as u32, 4));
            }
            
            // Borde selección
            if i == sel {
                gfx.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 0));
                let _ = gfx.canvas.draw_rect(sdl2::rect::Rect::new(s.x - 3, s.y - 3, (sz + 6) as u32, (sz + 6) as u32));
            }
        }
        
        // UI
        gfx.draw_text("🛡️ Demo Sprites V2", 20, 10, 20, 255, 255, 255);
        gfx.draw_text(&format!("Frame: {} | Archivos: {}/4", frame, existentes), 20, 35, 16, 0, 255, 0);
        gfx.draw_text(&format!("Sel: {}", sprites[sel].nombre), 20, 55, 14, 255, 255, 0);
        gfx.draw_text("<->=Mover | 1-4=Select | A=Anim | R=Reset | ESC=Salir", 10, 565, 14, 128, 128, 128);
        gfx.draw_text("Blanco arriba = archivo existe", 10, 585, 14, 128, 128, 128);
        
        gfx.canvas.present();
    }
    
    println!("\n✅ Demo: {} frames", frame);
    Ok(())
}
