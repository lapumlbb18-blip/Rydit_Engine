// demo_transitions.rs
// Demo: 15+ transiciones tipo editor de video
//
// Muestra cada transición en secuencia automática
// Controles:
// SPACE: Siguiente transición
// ← →: Anterior/Siguiente manual
// +/-: Velocidad
// ESC: Salir

use ry_gfx::backend_sdl2::Sdl2Backend;
use ry_gfx::sdl2_ffi::FontFFI;
use ry_gfx::transitions::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn render_txt(font: &Option<FontFFI>, txt: &str, r: u8, g: u8, b: u8,
    tc: &sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Option<sdl2::render::Texture<'static>> {
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

fn draw_tex(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, tex: &sdl2::render::Texture, x: i32, y: i32, max_w: u32) {
    let q = tex.query();
    let w = q.width.min(max_w);
    let _ = canvas.copy(tex, None, Rect::new(x, y, w, q.height));
}

fn main() -> Result<(), String> {
    println!("🎬 RyDit - Demo 15+ Transiciones");
    
    let mut backend = Sdl2Backend::new("Demo Transiciones - RyDit", 800, 500)?;
    
    let mut font: Option<FontFFI> = None;
    for p in &["/system/fonts/DroidSans.ttf", "/system/fonts/NotoSans-Regular.ttf",
               "/data/data/com.termux/files/usr/share/fonts/noto-sans/NotoSans-Regular.ttf"] {
        if std::path::Path::new(p).exists() {
            font = FontFFI::load(p, 16).ok();
            if font.is_some() { println!("✅ Fuente: {}", p); break; }
        }
    }
    
    let tc = &backend.canvas.texture_creator();
    
    // Lista de todas las transiciones con nombres
    let all_transitions: Vec<(TransitionType, &str)> = vec![
        (TransitionType::FadeIn, "Fade In"),
        (TransitionType::FadeOut, "Fade Out"),
        (TransitionType::CrossFade, "Cross Fade"),
        (TransitionType::SlideLeft, "Slide Left"),
        (TransitionType::SlideRight, "Slide Right"),
        (TransitionType::SlideUp, "Slide Up"),
        (TransitionType::SlideDown, "Slide Down"),
        (TransitionType::WipeRight, "Wipe Right"),
        (TransitionType::WipeDown, "Wipe Down"),
        (TransitionType::ZoomIn, "Zoom In"),
        (TransitionType::ZoomOut, "Zoom Out"),
        (TransitionType::CircleOpen, "Circle Open"),
        (TransitionType::CircleClose, "Circle Close"),
        (TransitionType::BlindsHorizontal, "Blinds Horizontal"),
        (TransitionType::BlindsVertical, "Blinds Vertical"),
        (TransitionType::Dissolve, "Dissolve"),
        (TransitionType::Spiral, "Spiral"),
        (TransitionType::Checkerboard, "Checkerboard"),
        (TransitionType::Pixelate, "Pixelate"),
    ];
    
    let mut current_idx = 0;
    let mut duration = 1.5;
    let mut auto_advance = true;
    let mut transition = Transition::new(all_transitions[0].0.clone(), duration);
    
    // Texturas para CrossFade (A → B)
    let tex_a = render_txt(&font, "ESCENA A", 255, 100, 100, tc);
    let tex_b = render_txt(&font, "ESCENA B", 100, 100, 255, tc);
    
    // Crear texturas de escena con colores de fondo
    let mut scene_a = sdl2::surface::Surface::new(800, 500, sdl2::pixels::PixelFormatEnum::RGBA8888).unwrap();
    let _ = scene_a.fill_rect(None, Color::RGB(40, 20, 60));
    let tex_scene_a = tc.create_texture_from_surface(&scene_a).ok();
    let tex_scene_a: Option<sdl2::render::Texture<'static>> = tex_scene_a.map(|t| unsafe { std::mem::transmute(t) });
    
    let mut scene_b = sdl2::surface::Surface::new(800, 500, sdl2::pixels::PixelFormatEnum::RGBA8888).unwrap();
    let _ = scene_b.fill_rect(None, Color::RGB(20, 40, 60));
    let tex_scene_b = tc.create_texture_from_surface(&scene_b).ok();
    let tex_scene_b: Option<sdl2::render::Texture<'static>> = tex_scene_b.map(|t| unsafe { std::mem::transmute(t) });
    
    let tex_titulo = render_txt(&font, "🎬 Transiciones - ry-gfx v0.18.0", 255, 255, 100, tc);
    
    let mut frame_count: u64 = 0;
    let mut running = true;
    
    'run: loop {
        let fs = std::time::Instant::now();
        let dt = 0.016;
        
        for ev in backend.event_pump.poll_iter() {
            match ev {
                Event::Quit{..}|Event::KeyDown{keycode:Some(Keycode::Escape),..} => {running=false;break 'run;}
                Event::KeyDown{keycode:Some(Keycode::Space),..} => {
                    // Siguiente transición
                    current_idx = (current_idx + 1) % all_transitions.len();
                    transition = Transition::new(all_transitions[current_idx].0.clone(), duration);
                }
                Event::KeyDown{keycode:Some(Keycode::Left),..} => {
                    current_idx = if current_idx == 0 { all_transitions.len() - 1 } else { current_idx - 1 };
                    transition = Transition::new(all_transitions[current_idx].0.clone(), duration);
                }
                Event::KeyDown{keycode:Some(Keycode::Right),..} => {
                    current_idx = (current_idx + 1) % all_transitions.len();
                    transition = Transition::new(all_transitions[current_idx].0.clone(), duration);
                }
                Event::KeyDown{keycode:Some(Keycode::Equals),..} => {
                    duration = (duration + 0.25).min(5.0);
                    transition.duration = duration;
                }
                Event::KeyDown{keycode:Some(Keycode::Minus),..} => {
                    duration = (duration - 0.25).max(0.25);
                    transition.duration = duration;
                }
                Event::KeyDown{keycode:Some(Keycode::A),..} => {
                    auto_advance = !auto_advance;
                }
                _ => {}
            }
        }
        
        // Actualizar transición
        transition.update(dt);
        
        // Auto-advance
        if auto_advance && transition.done {
            current_idx = (current_idx + 1) % all_transitions.len();
            transition = Transition::new(all_transitions[current_idx].0.clone(), duration);
        }
        
        // RENDER
        let (scene_tex, bg_color) = if transition.progress() < 0.5 {
            (&tex_scene_a, Color::RGB(40, 20, 60))
        } else {
            (&tex_scene_b, Color::RGB(20, 40, 60))
        };
        
        // Dibujar escena base
        if let Some(ref t) = scene_tex {
            let _ = backend.canvas.copy(t, None, Rect::new(0, 0, 800, 500));
        }
        
        // Texto sobre escena
        let scene_label = if transition.progress() < 0.5 { "ESCENA A" } else { "ESCENA B" };
        if let Some(ref t) = render_txt(&font, scene_label, 255, 255, 255, tc) {
            draw_tex(&mut backend.canvas, t, 350, 230, 200);
        }
        
        // Aplicar transición
        transition.render(&mut backend.canvas, 800, 500, scene_tex.as_ref());
        
        // UI overlays
        backend.canvas.set_draw_color(Color::RGBA(0, 0, 0, 180));
        let _ = backend.canvas.fill_rect(Rect::new(0, 0, 800, 36));
        let _ = backend.canvas.fill_rect(Rect::new(0, 460, 800, 40));
        
        if let Some(ref t) = tex_titulo { draw_tex(&mut backend.canvas, t, 10, 6, 780); }
        
        // Nombre de transición actual
        let trans_name = all_transitions[current_idx].1;
        if let Some(ref t) = render_txt(&font, trans_name, 255, 255, 100, tc) {
            draw_tex(&mut backend.canvas, t, 300, 462, 300);
        }
        
        // Barra de progreso
        let bar_w = 200;
        let bar_h = 8;
        let bar_x = 300;
        let bar_y = 485;
        backend.canvas.set_draw_color(Color::RGB(60, 60, 80));
        let _ = backend.canvas.fill_rect(Rect::new(bar_x, bar_y, bar_w, bar_h));
        backend.canvas.set_draw_color(Color::RGB(100, 200, 255));
        let progress_w = (transition.eased_progress() * bar_w as f32) as u32;
        let _ = backend.canvas.fill_rect(Rect::new(bar_x, bar_y, progress_w, bar_h));
        
        // Info
        let info = format!("{}/{} | {:.1}s | Auto: {}", current_idx + 1, all_transitions.len(), duration, if auto_advance{"ON"}else{"OFF"});
        if let Some(ref t) = render_txt(&font, &info, 180, 180, 180, tc) {
            draw_tex(&mut backend.canvas, t, 10, 462, 280);
        }
        
        let controls = "SPACE: Siguiente | ← →: Nav | +/-: Velocidad | A: Auto";
        if let Some(ref t) = render_txt(&font, controls, 150, 150, 150, tc) {
            draw_tex(&mut backend.canvas, t, 100, 478, 700);
        }
        
        backend.canvas.present();
        frame_count += 1;
        
        let e = fs.elapsed();
        if e < std::time::Duration::from_millis(16) { std::thread::sleep(std::time::Duration::from_millis(16)-e); }
    }
    
    println!("\n✅ Demo Transiciones cerrado");
    Ok(())
}
