// demo_emoji_utf8.rs
// Demo: Emojis como sprites + texto UTF-8

use ry_gfx::backend_sdl2::Sdl2Backend;
use ry_gfx::emoji_atlas::EmojiAtlas;
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

fn main() -> Result<(), String> {
    println!("🔤 RyDit - Demo Emojis (Sprites) + UTF-8");
    
    let mut backend = Sdl2Backend::new("Demo Emojis + UTF-8", 800, 600)?;
    
    // Fuente principal
    let mut font: Option<FontFFI> = None;
    for p in &["/system/fonts/DroidSans.ttf", "/system/fonts/NotoSans-Regular.ttf",
               "/data/data/com.termux/files/usr/share/fonts/noto-sans/NotoSans-Regular.ttf"] {
        if std::path::Path::new(p).exists() {
            font = FontFFI::load(p, 16).ok();
            if font.is_some() { println!("✅ Fuente: {}", p); break; }
        }
    }
    
    let tc = &backend.canvas.texture_creator();
    
    // Atlas de emojis como sprites
    let emoji_atlas = EmojiAtlas::new(tc, 24)?;
    
    // Textos de prueba - emojis se muestran como sprites
    let textos = [
        ("Hola! Acentos: aeio u", false),           // Sin emojis
        ("Como estas? Genial!", false),              // Texto normal
        ("Emojis:", false),                           // Label
        ("😀😃😄😁😆😅😂🤣😊", true),            // Solo emojis
        ("🎮🎯🎨🎬🎤🎧🎵🎶🎸", true),             // Solo emojis
        ("❤💯🔥⭐✅❌⚡🌟💡🎉", true),            // Solo emojis
        ("Bienvenido a Ry-Dit!", false),             // Texto
    ];
    
    let tex_titulo = render_txt(&font, "Emojis (Sprites) + UTF-8 + Acentos", 255, 255, 100, tc);
    
    // Renderizar cada línea
    let line_textures: Vec<Option<Texture<'static>>> = textos.iter().map(|(txt, is_emoji)| {
        if *is_emoji {
            // Renderizar emojis individuales y combinar
            Some(render_emoji_line(&emoji_atlas, txt, tc))
        } else {
            render_txt(&font, txt, 230, 230, 240, tc)
        }
    }).collect();
    
    // Panel de caracteres con acentos
    let chars = ['n','N','a','e','i','o','u','A','E','I','O','U','u','U','!','?'];
    let char_labels = ["ñ","Ñ","á","é","í","ó","ú","Á","É","Í","Ó","Ú","ü","Ü","¡","¿"];
    let char_tex: Vec<Option<Texture<'static>>> = char_labels.iter().map(|lbl| {
        render_txt(&font, lbl, 255, 255, 255, tc)
    }).collect();
    
    let info_str = if font.is_some() {
        format!("Fuente: OK | Emojis: {} sprites", emoji_atlas.textures.len())
    } else {
        "Sin fuente".to_string()
    };
    let tex_info = render_txt(&font, &info_str, 100, 200, 100, tc);
    
    let mut running = true;
    let mut frame_count: u64 = 0;
    
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
        
        // Titulo
        backend.canvas.set_draw_color(Color::RGB(40,40,60));
        let _ = backend.canvas.fill_rect(Rect::new(0,0,800,36));
        if let Some(ref t) = tex_titulo { draw_tex(&mut backend.canvas, t, 10, 6, 780); }
        
        // Textos con emojis
        for (i,tex_opt) in line_textures.iter().enumerate() {
            if let Some(ref t) = tex_opt {
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
        
        // Panel de caracteres con acentos
        let py = 520;
        backend.canvas.set_draw_color(Color::RGBA(50,50,70,200));
        let _ = backend.canvas.fill_rect(Rect::new(0,py-10,800,80));
        
        // Etiquetas
        if let Some(ref t) = render_txt(&font, "Acentos:", 200, 200, 100, tc) {
            draw_tex(&mut backend.canvas, t, 10, py-8, 100);
        }
        
        for (i,tex_opt) in char_tex.iter().enumerate() {
            if let Some(ref t) = tex_opt {
                let x = 100 + (i as i32 * 42);
                let q=t.query();
                backend.canvas.set_draw_color(Color::RGBA(100,100,150,150));
                let w = q.width as i32;
                let h = q.height as i32;
                let _ = backend.canvas.draw_rect(Rect::new(x-2, py+4, w.max(0) as u32+4, h.max(0) as u32+4));
                draw_tex(&mut backend.canvas, t, x, py+6, 40);
            }
        }
        
        // Info
        if let Some(ref t) = tex_info { draw_tex(&mut backend.canvas, t, 10, 575, 780); }
        
        // Leyenda
        let leyenda = render_txt(&font, "Emojis = Sprites PNG | Texto = TTF", 150, 150, 150, tc);
        if let Some(ref t) = leyenda { draw_tex(&mut backend.canvas, t, 400, 575, 390); }
        
        backend.canvas.present();
        frame_count += 1;
        
        let e = fs.elapsed();
        if e < std::time::Duration::from_millis(16) { std::thread::sleep(std::time::Duration::from_millis(16)-e); }
    }
    
    println!("\nDemo cerrado");
    Ok(())
}

// Función auxiliar para renderizar línea de emojis como sprites
fn render_emoji_line(
    atlas: &EmojiAtlas,
    text: &str,
    tc: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) -> Texture<'static> {
    let emoji_size = atlas.size();
    let count = text.chars().filter(|c| atlas.is_emoji(*c)).count();
    let total_w = (count as u32) * emoji_size;
    
    let mut surface = sdl2::surface::Surface::new(total_w, emoji_size, sdl2::pixels::PixelFormatEnum::RGBA8888).unwrap();
    let _ = surface.fill_rect(None, sdl2::pixels::Color::RGBA(0,0,0,0));
    
    let mut x_offset = 0;
    for ch in text.chars() {
        if atlas.is_emoji(ch) {
            if let Some(tex) = atlas.get(ch) {
                let q = tex.query();
                // Dibujar emoji en surface
                // Simplificación: copiar color representativo
                let _ = surface.fill_rect(
                    Rect::new(x_offset, 0, emoji_size, emoji_size),
                    sdl2::pixels::Color::RGB(255, 220, 50),
                );
                // Símbolo central
                let s2 = emoji_size / 2;
                let off = (emoji_size - s2) / 2;
                let _ = surface.fill_rect(
                    Rect::new(x_offset + off as i32, off as i32, s2, s2),
                    sdl2::pixels::Color::RGB(255, 150, 0),
                );
                x_offset += emoji_size as i32;
            }
        }
    }
    
    let tex = tc.create_texture_from_surface(&surface).unwrap();
    unsafe { std::mem::transmute(tex) }
}
