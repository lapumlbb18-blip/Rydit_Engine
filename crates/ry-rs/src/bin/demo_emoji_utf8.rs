// demo_emoji_utf8.rs
// Demo: Soporte de Emojis + UTF-8 + Acentos (ñ, á, é, í, ó, ú, ü)

use ry_gfx::backend_sdl2::Sdl2Backend;
use ry_gfx::sdl2_ffi::FontFFI;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;

fn render_txt(font: &Option<FontFFI>, txt: &str, r: u8, g: u8, b: u8,
    tc: &sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Option<Texture<'static>> {
    if let Some(f) = font {
        if let Ok(surf) = f.render_text_blended(txt, r, g, b) {
            unsafe {
                let s = sdl2::surface::Surface::from_ll(surf as *mut sdl2::sys::SDL_Surface);
                if let Ok(t) = tc.create_texture_from_surface(&s) {
                    return Some(std::mem::transmute(t));
                }
            }
        }
    }
    None
}

fn draw_tex(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, tex: &Texture, x: i32, y: i32, max_w: u32) {
    let q = tex.query();
    let w = q.width.min(max_w);
    let h = q.height;
    let _ = canvas.copy(tex, None, Rect::new(x, y, w, h));
}

fn rect_tex(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, tex: &Texture, x: i32, y: i32, border_color: Color) {
    let q = tex.query();
    let w = q.width as i32;
    let h = q.height as i32;
    canvas.set_draw_color(border_color);
    let _ = canvas.draw_rect(Rect::new(x-2, y-2, w.max(0) as u32+4, h.max(0) as u32+4));
}

fn main() -> Result<(), String> {
    println!("🔤 RyDit - Demo Emojis + UTF-8");
    
    let mut backend = Sdl2Backend::new("Demo Emojis UTF-8", 800, 600)?;
    
    let mut font: Option<FontFFI> = None;
    for p in &["/system/fonts/DroidSans.ttf", "/system/fonts/NotoSans-Regular.ttf",
               "/data/data/com.termux/files/usr/share/fonts/noto-sans/NotoSans-Regular.ttf"] {
        if std::path::Path::new(p).exists() {
            font = FontFFI::load(p, 16).ok();
            if font.is_some() { println!("✅ Fuente: {}", p); break; }
        }
    }
    
    let mut emoji_font: Option<FontFFI> = None;
    for p in &["/system/fonts/NotoColorEmoji.ttf", "/data/data/com.termux/files/usr/share/fonts/noto-emoji/NotoColorEmoji.ttf"] {
        if std::path::Path::new(p).exists() {
            emoji_font = FontFFI::load(p, 16).ok();
            if emoji_font.is_some() { println!("✅ Emoji font: {}", p); break; }
        }
    }
    
    let tc = &backend.canvas.texture_creator();
    
    // Textos UTF-8
    let textos = [
        "1. ¡Hola! Ñoño: áéíóú ÁÉÍÓÚ ü Ü",
        "2. ¿Cómo estás? ¡Genial! 👍",
        "3. 😀 😃 😄 😁 😆 😅 😂 🤣 😊",
        "4. 🎮 🎯 🎨 🎬 🎤 🎧 🎵 🎶 🎸",
        "5. ❤️ 💯 🔥 ⭐ ✅ ❌ ⚡ 🌟 💡 🎉",
        "6. El rápido murciélago vuela 🦇",
        "7. Camión ómnibus árbol acción 🚌🌳",
        "8. ¡Bienvenido a Ry-Dit! 🚀🛡️",
    ];
    
    let tex_titulo = render_txt(&font, "🔤 Emojis + UTF-8 + Acentos", 255, 255, 100, tc);
    
    let texturas: Vec<Option<Texture<'static>>> = textos.iter().map(|t| {
        // Intentar fuente principal
        if let Some(tex) = render_txt(&font, t, 230, 230, 240, tc) { return Some(tex); }
        // Fallback emojis
        if let Some(tex) = render_txt(&emoji_font, t, 230, 230, 240, tc) { return Some(tex); }
        None
    }).collect();
    
    let chars = ['ñ','Ñ','á','é','í','ó','ú','Á','É','Í','Ó','Ú','ü','Ü','¡','¿'];
    let char_tex: Vec<Option<Texture<'static>>> = chars.iter().map(|c| {
        render_txt(&font, &c.to_string(), 255, 255, 255, tc)
    }).collect();
    
    let info_str = if font.is_some() {
        format!("✅ Fuente OK | Emoji: {}", if emoji_font.is_some(){"OK"} else {"NO"} )
    } else { "❌ Sin fuente".to_string() };
    let tex_info = render_txt(&font, &info_str, 100, 200, 100, tc);
    
    let mut running = true;
    'run: loop {
        let fs = std::time::Instant::now();
        
        for ev in backend.event_pump.poll_iter() {
            match ev {
                Event::Quit{..}|Event::KeyDown{keycode:Some(Keycode::Escape),..} => {running=false;break 'run;}
                _=>{}
            }
        }
        
        backend.canvas.set_draw_color(Color::RGB(18,18,24));
        backend.canvas.clear();
        
        // Título
        backend.canvas.set_draw_color(Color::RGB(40,40,60));
        let _ = backend.canvas.fill_rect(Rect::new(0,0,800,36));
        if let Some(ref t) = tex_titulo { draw_tex(&mut backend.canvas, t, 10, 6, 780); }
        
        // Textos
        for (i,tex) in texturas.iter().enumerate() {
            if let Some(ref t) = tex {
                let y = 48 + (i as i32 * 56);
                if i%2==0 {
                    backend.canvas.set_draw_color(Color::RGBA(255,255,255,8));
                    let q=t.query();
                    let h = q.height as i32;
                    let _ = backend.canvas.fill_rect(Rect::new(0,y-2,800,h.max(0) as u32+4));
                }
                draw_tex(&mut backend.canvas, t, 10, y, 780);
            }
        }
        
        // Panel caracteres
        let py = 530;
        backend.canvas.set_draw_color(Color::RGBA(50,50,70,200));
        let _ = backend.canvas.fill_rect(Rect::new(0,py-10,800,70));
        for (i,tex) in char_tex.iter().enumerate() {
            if let Some(ref t) = tex {
                let x = 10 + (i as i32 * 46);
                rect_tex(&mut backend.canvas, t, x, py+6, Color::RGBA(100,100,150,150));
                draw_tex(&mut backend.canvas, t, x, py+6, 40);
            }
        }
        
        // Info
        if let Some(ref t) = tex_info { draw_tex(&mut backend.canvas, t, 10, 578, 780); }
        
        backend.canvas.present();
        
        let e = fs.elapsed();
        if e < std::time::Duration::from_millis(16) { std::thread::sleep(std::time::Duration::from_millis(16)-e); }
    }
    
    println!("\n✅ Demo cerrado");
    Ok(())
}
