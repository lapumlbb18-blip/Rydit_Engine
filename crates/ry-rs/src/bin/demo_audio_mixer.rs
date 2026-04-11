// demo_audio_mixer.rs
// Demo: Audio Mixer Avanzado - 4 buses + spatial 2D + fade
//
// Controles:
// 1: Toggle Música on/off
// 2: Toggle bus SFX
// 3: Toggle bus Ambiente
// 4: Toggle bus UI
// + / -: Volumen master
// F: Fade out música
// R: Fade in música
// P: Reproducir sonido SFX
// A: Reproducer sonido Ambiente
// U: Reproducer sonido UI
// ← →: Mover oyente (audio espacial)
// ESC: Salir

use ry_gfx::audio_mixer::{AudioBus, AudioMixer, SpatialSound};
use ry_gfx::backend_sdl2::Sdl2Backend;
use ry_gfx::sdl2_ffi::FontFFI;
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
    println!("🔊 RyDit - Demo Audio Mixer Avanzado");
    
    let mut backend = Sdl2Backend::new("Demo Audio Mixer", 800, 500)?;
    
    // Fuente
    let mut font: Option<FontFFI> = None;
    for p in &["/system/fonts/DroidSans.ttf", "/system/fonts/NotoSans-Regular.ttf",
               "/data/data/com.termux/files/usr/share/fonts/noto-sans/NotoSans-Regular.ttf"] {
        if std::path::Path::new(p).exists() {
            font = FontFFI::load(p, 14).ok();
            if font.is_some() { println!("✅ Fuente: {}", p); break; }
        }
    }
    
    let tc = &backend.canvas.texture_creator();
    
    // Inicializar Audio Mixer
    let mut mixer = match AudioMixer::new() {
        Ok(m) => {
            println!("✅ Audio Mixer inicializado");
            Some(m)
        }
        Err(e) => {
            eprintln!("⚠️  Audio Mixer falló: {}", e);
            None
        }
    };
    
    // Cargar sonidos de prueba (generados proceduralmente si no existen)
    if let Some(ref mut mx) = mixer {
        // Intentar cargar sonidos (si no existen, generar tonos WAV)
        let sounds = [
            ("sfx_shoot", "/data/data/com.termux/files/home/shield-project/shoot.wav"),
            ("sfx_jump", "/data/data/com.termux/files/home/shield-project/jump_sound.wav"),
            ("sfx_push", "/data/data/com.termux/files/home/shield-project/push_sound.wav"),
            ("ui_click", "/data/data/com.termux/files/home/shield-project/test_tone.wav"),
            ("amb_wind", "/data/data/com.termux/files/home/shield-project/test_audio.wav"),
        ];
        
        for (id, path) in &sounds {
            if std::path::Path::new(path).exists() {
                let _ = mx.load_sound(id, path);
            } else {
                // Generar tono WAV procedural
                println!("⚠️  Generando tono: {}", path);
                generate_tone(path, 440.0, 0.3);
                let _ = mx.load_sound(id, path);
            }
        }
    }
    
    // Texturas UI
    let tex_titulo = render_txt(&font, "🔊 Audio Mixer - 4 Buses + Spatial 2D", 255, 255, 100, tc);
    
    let mut tex_status: Option<sdl2::render::Texture<'static>> = None;
    let mut tex_buses: Option<sdl2::render::Texture<'static>> = None;
    let mut tex_spatial: Option<sdl2::render::Texture<'static>> = None;
    let mut tex_controles: Option<sdl2::render::Texture<'static>> = None;
    let mut frame_count: u64 = 0;
    
    // Estado
    let mut listener_x: f32 = 400.0;
    let mut listener_y: f32 = 250.0;
    let mut last_msg: String = "Audio Mixer listo".to_string();
    let mut msg_timer: f32 = 0.0;
    
    let mut running = true;
    'run: loop {
        let fs = std::time::Instant::now();
        let dt = 0.016;
        
        for ev in backend.event_pump.poll_iter() {
            match ev {
                Event::Quit{..}|Event::KeyDown{keycode:Some(Keycode::Escape),..} => {running=false;break 'run;}
                Event::KeyDown{keycode:Some(kc),..} => {
                    if let Some(ref mut mx) = mixer {
                        match kc {
                            Keycode::M => {
                                if mx.is_music_playing() {
                                    mx.stop_music();
                                    last_msg = "🔇 Música detenida".to_string();
                                } else {
                                    // Generar música si no existe
                                    let music_path = "/data/data/com.termux/files/home/shield-project/music_test.ogg";
                                    if !std::path::Path::new(music_path).exists() {
                                        generate_tone(music_path, 330.0, 3.0);
                                    }
                                    let _ = mx.load_music(music_path);
                                    if mx.play_music(-1) {
                                        last_msg = "🎵 Música reproduciendo".to_string();
                                    }
                                }
                                msg_timer = 2.0;
                            }
                            Keycode::S => {
                                let vol = if mx.get_bus_volume(AudioBus::SFX) > 0.5 { 0.0 } else { 1.0 };
                                mx.set_bus_volume(AudioBus::SFX, vol);
                                last_msg = if vol > 0.5 { "🔊 Bus SFX ON" } else { "🔇 Bus SFX OFF" }.to_string();
                                msg_timer = 2.0;
                            }
                            Keycode::B => {
                                let vol = if mx.get_bus_volume(AudioBus::Ambiente) > 0.5 { 0.0 } else { 1.0 };
                                mx.set_bus_volume(AudioBus::Ambiente, vol);
                                last_msg = if vol > 0.5 { "🌊 Bus Ambiente ON" } else { "🌊 Bus Ambiente OFF" }.to_string();
                                msg_timer = 2.0;
                            }
                            Keycode::I => {
                                let vol = if mx.get_bus_volume(AudioBus::UI) > 0.5 { 0.0 } else { 1.0 };
                                mx.set_bus_volume(AudioBus::UI, vol);
                                last_msg = if vol > 0.5 { "🖱️ Bus UI ON" } else { "🖱️ Bus UI OFF" }.to_string();
                                msg_timer = 2.0;
                            }
                            Keycode::Equals | Keycode::RightBracket => {
                                mx.set_master_volume((mx.get_bus_volume(AudioBus::SFX) + 0.1).min(1.0));
                                last_msg = "🔊 Volumen +".to_string();
                                msg_timer = 1.5;
                            }
                            Keycode::Slash | Keycode::LeftBracket => {
                                mx.set_master_volume((mx.get_bus_volume(AudioBus::SFX) - 0.1).max(0.0));
                                last_msg = "🔉 Volumen -".to_string();
                                msg_timer = 1.5;
                            }
                            Keycode::F => {
                                mx.fade_out_music(1000);
                                last_msg = "📉 Fade out música 1s".to_string();
                                msg_timer = 2.0;
                            }
                            Keycode::R => {
                                if mx.fade_in_music(1500, -1) {
                                    last_msg = "📈 Fade in música 1.5s".to_string();
                                }
                                msg_timer = 2.0;
                            }
                            Keycode::P => {
                                if mx.play_sound("sfx_shoot") {
                                    last_msg = "💥 SFX: Shoot!".to_string();
                                } else {
                                    last_msg = "⚠️ SFX no encontrado".to_string();
                                }
                                msg_timer = 1.5;
                            }
                            Keycode::A => {
                                if mx.play_sound("amb_wind") {
                                    last_msg = "🌊 Ambiente: Wind".to_string();
                                }
                                msg_timer = 1.5;
                            }
                            Keycode::U => {
                                if mx.play_sound("ui_click") {
                                    last_msg = "🖱️ UI: Click".to_string();
                                }
                                msg_timer = 1.5;
                            }
                            Keycode::Left => {
                                listener_x = (listener_x - 30.0).max(50.0);
                                mx.set_listener_pos(listener_x, listener_y);
                                last_msg = format!("⬅ Oyente: ({:.0},{:.0})", listener_x, listener_y);
                                msg_timer = 1.5;
                            }
                            Keycode::Right => {
                                listener_x = (listener_x + 30.0).min(750.0);
                                mx.set_listener_pos(listener_x, listener_y);
                                last_msg = format!("➡ Oyente: ({:.0},{:.0})", listener_x, listener_y);
                                msg_timer = 1.5;
                            }
                            Keycode::Up => {
                                listener_y = (listener_y - 30.0).max(50.0);
                                mx.set_listener_pos(listener_x, listener_y);
                            }
                            Keycode::Down => {
                                listener_y = (listener_y + 30.0).min(450.0);
                                mx.set_listener_pos(listener_x, listener_y);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        
        if msg_timer > 0.0 {
            msg_timer -= dt as f32;
        } else {
            last_msg = "Listo - Presiona teclas".to_string();
        }
        
        // UPDATE texturas cada 10 frames
        frame_count += 1;
        if frame_count % 10 == 0 {
            if let Some(ref mx) = mixer {
                let status_txt = format!(
                    "Master: {:.0}% | Música: {} | Bus SFX: {:.0}% | Amb: {:.0}% | UI: {:.0}%",
                    mx.get_bus_volume(AudioBus::SFX) * 100.0,
                    if mx.is_music_playing() { "▶ ON" } else { "⏹ OFF" },
                    mx.get_bus_volume(AudioBus::SFX) * 100.0,
                    mx.get_bus_volume(AudioBus::Ambiente) * 100.0,
                    mx.get_bus_volume(AudioBus::UI) * 100.0,
                );
                tex_status = render_txt(&font, &status_txt, 200, 255, 200, tc);
                
                let bus_txt = format!(
                    "Buses: [1]Musica [2]SFX [3]Ambiente [4]UI"
                );
                tex_buses = render_txt(&font, &bus_txt, 150, 200, 255, tc);
                
                let spatial_txt = format!(
                    "Oyente: ({:.0}, {:.0}) | ← → ↑ ↓ mover",
                    listener_x, listener_y
                );
                tex_spatial = render_txt(&font, &spatial_txt, 255, 200, 100, tc);
                
                let ctrl_txt = "M:Musica S:SFX B:Ambiente I:UI P:Shoot A:Amb U:UI F:FadeOut R:FadeIn";
                tex_controles = render_txt(&font, ctrl_txt, 180, 180, 180, tc);
            }
        }
        
        // RENDER
        backend.canvas.set_draw_color(Color::RGB(15, 15, 25));
        backend.canvas.clear();
        
        // Barra título
        backend.canvas.set_draw_color(Color::RGB(30, 30, 50));
        let _ = backend.canvas.fill_rect(Rect::new(0, 0, 800, 32));
        if let Some(ref t) = tex_titulo { draw_tex(&mut backend.canvas, t, 10, 4, 780); }
        
        // Status
        if let Some(ref t) = tex_status { draw_tex(&mut backend.canvas, t, 10, 45, 780); }
        
        // Mensaje reciente
        if let Some(ref t) = render_txt(&font, &last_msg, 255, 255, 150, tc) {
            draw_tex(&mut backend.canvas, t, 10, 70, 780);
        }
        
        // Buses
        if let Some(ref t) = tex_buses { draw_tex(&mut backend.canvas, t, 10, 100, 780); }
        
        // Listener position (audio espacial)
        if let Some(ref t) = tex_spatial { draw_tex(&mut backend.canvas, t, 10, 130, 780); }
        
        // Visualización del oyente
        backend.canvas.set_draw_color(Color::RGB(50, 50, 80));
        let _ = backend.canvas.fill_rect(Rect::new(0, 170, 800, 200));
        
        // Oyente (círculo azul)
        backend.canvas.set_draw_color(Color::RGB(50, 100, 255));
        let _ = backend.canvas.fill_rect(Rect::new(listener_x as i32 - 8, listener_y as i32 - 120 - 8, 16, 16));
        backend.canvas.set_draw_color(Color::RGB(100, 150, 255));
        let _ = backend.canvas.fill_rect(Rect::new(listener_x as i32 - 4, listener_y as i32 - 120 - 4, 8, 8));
        
        // Sonidos espaciales (indicadores)
        let sound_positions = [(200.0, 170.0), (600.0, 170.0), (400.0, 270.0)];
        let sound_labels = ["🔫 Shoot", "🌊 Wind", "🖱 Click"];
        for (i, (sx, sy)) in sound_positions.iter().enumerate() {
            let dx = *sx - listener_x;
            let dy = (*sy + 120.0) - listener_y;
            let dist = (dx * dx + dy * dy).sqrt();
            let max_dist = 300.0;
            let vol = (1.0 - (dist / max_dist)).max(0.0);
            
            let alpha = (vol * 200.0) as u8;
            backend.canvas.set_draw_color(Color::RGBA(255, 200, 50, alpha));
            let size = (vol * 12.0) as i32;
            let _ = backend.canvas.fill_rect(Rect::new(*sx as i32 - size/2, *sy as i32 - size/2, size.max(4) as u32, size.max(4) as u32));
            
            if let Some(ref t) = render_txt(&font, sound_labels[i], 200, 200, 200, tc) {
                draw_tex(&mut backend.canvas, t, *sx as i32 - 20, *sy as i32 + 16, 60);
            }
        }
        
        // Controles
        if let Some(ref t) = tex_controles { draw_tex(&mut backend.canvas, t, 10, 390, 780); }
        
        backend.canvas.present();
        
        let e = fs.elapsed();
        if e < std::time::Duration::from_millis(16) { std::thread::sleep(std::time::Duration::from_millis(16)-e); }
    }
    
    println!("\n✅ Demo Audio Mixer cerrado");
    Ok(())
}

/// Generar un tono WAV simple (para pruebas)
fn generate_tone(path: &str, frequency: f32, duration: f32) {
    use std::io::Write;
    let sample_rate = 44100u32;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut data = Vec::with_capacity(num_samples * 2);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let env = (1.0 - (i as f32 / num_samples as f32)).max(0.0); // Fade out
        let sample = (frequency * 2.0 * std::f32::consts::PI * t).sin() * 0.5 * env;
        let sample_i16 = (sample * 32767.0) as i16;
        data.extend_from_slice(&sample_i16.to_le_bytes());
    }
    let data_size = data.len() as u32;
    let file_size = 36 + data_size;
    if let Ok(mut file) = std::fs::File::create(path) {
        let _ = file.write_all(b"RIFF");
        let _ = file.write_all(&file_size.to_le_bytes());
        let _ = file.write_all(b"WAVEfmt ");
        let _ = file.write_all(&16u32.to_le_bytes());
        let _ = file.write_all(&1u16.to_le_bytes());
        let _ = file.write_all(&1u16.to_le_bytes());
        let _ = file.write_all(&sample_rate.to_le_bytes());
        let _ = file.write_all(&(sample_rate * 2).to_le_bytes());
        let _ = file.write_all(&2u16.to_le_bytes());
        let _ = file.write_all(&16u16.to_le_bytes());
        let _ = file.write_all(b"data");
        let _ = file.write_all(&data_size.to_le_bytes());
        let _ = file.write_all(&data);
    }
}
