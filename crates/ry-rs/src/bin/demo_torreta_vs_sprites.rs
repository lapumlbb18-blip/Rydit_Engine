// demo_torreta_vs_sprites.rs
// Juego completo: Torreta vs Enemigos con niveles, mapa extenso, cámara, menús
//
// Patrón: demo_rigidbody.rs (Sdl2Backend + TTF + Audio + Sprites)
// Pipeline: Zink/DRI3 → OpenGL ES
//
// Controles:
// ← → ó A/D = Mover torreta
// SPACE = Disparar
// P = Pausa
// R = Reiniciar nivel
// ESC = Salir al menú

use ry_gfx::backend_sdl2::Sdl2Backend;
use ry_gfx::audio_sdl2::AudioSystemSDL2;
use ry_gfx::ColorRydit;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::image::LoadSurface;

// ============================================================================
// TIPOS DEL JUEGO
// ============================================================================

enum GameState {
    Menu,
    Playing { nivel: usize },
    Paused { nivel: usize },
    GameOver,
    GameWin,
    LevelComplete { nivel: usize },
}

struct Bala {
    x: f32, y: f32,
    vx: f32,
    activa: bool,
}

struct Enemigo {
    nombre: String,
    textura: Option<sdl2::render::Texture<'static>>,
    x: f32, y: f32,
    vx: f32, vy: f32,
    w: u32, h: u32, scale: u32,
    color: Color,
    vivo: bool,
    en_suelo: bool,
    dir: i32, // -1 izquierda, 1 derecha
    patrol_start: f32,
    patrol_end: f32,
    vida: i32,
}

struct Torreta {
    x: f32, y: f32,
    vx: f32, vy: f32,
    w: u32, h: u32, scale: u32,
    en_suelo: bool,
    vidas: i32,
    puntaje: i32,
    dir: i32,
}

struct Camera {
    x: f32, y: f32,
    map_w: i32, map_h: i32,
}

impl Camera {
    fn follow(&mut self, target_x: f32, target_y: f32, screen_w: i32, screen_h: i32) {
        // Centro de pantalla en el target
        self.x = target_x - screen_w as f32 / 2.0;
        self.y = target_y - screen_h as f32 / 2.0;

        // Clamp a bounds del mapa
        self.x = self.x.max(0.0).min((self.map_w - screen_w) as f32);
        self.y = self.y.max(0.0).min((self.map_h - screen_h) as f32);
    }

    fn screen_x(&self, x: f32) -> i32 {
        (x - self.x) as i32
    }

    fn screen_y(&self, y: f32) -> i32 {
        (y - self.y) as i32
    }

    fn is_visible(&self, x: f32, y: f32, w: u32, h: u32, screen_w: i32, screen_h: i32) -> bool {
        let sx = self.screen_x(x);
        let sy = self.screen_y(y);
        sx + w as i32 > 0 && sx < screen_w && sy + h as i32 > 0 && sy < screen_h
    }
}

// ============================================================================
// HELPERS
// ============================================================================

fn crear_textura<'a>(
    font: &Option<ry_gfx::sdl2_ffi::FontFFI>,
    texto: &str,
    r: u8, g: u8, b: u8,
    tc: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) -> Option<sdl2::render::Texture<'a>> {
    if let Some(f) = font {
        if let Ok(surface_ptr) = f.render_text_blended(texto, r, g, b) {
            unsafe {
                let sdl_surface = sdl2::surface::Surface::from_ll(surface_ptr as *mut sdl2::sys::SDL_Surface);
                if let Ok(tex) = tc.create_texture_from_surface(&sdl_surface) {
                    return Some(std::mem::transmute(tex));
                }
            }
        }
    }
    None
}

fn generar_tono(path: &str, frequency: f32, duration: f32) -> Result<(), String> {
    use std::io::Write;
    let sample_rate = 44100u32;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut data = Vec::with_capacity(num_samples * 2);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (frequency * 2.0 * std::f32::consts::PI * t).sin() * 0.5;
        let sample_i16 = (sample * 32767.0) as i16;
        data.extend_from_slice(&sample_i16.to_le_bytes());
    }
    let data_size = data.len() as u32;
    let file_size = 36 + data_size;
    let mut file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    file.write_all(b"RIFF").map_err(|e| e.to_string())?;
    file.write_all(&file_size.to_le_bytes()).map_err(|e| e.to_string())?;
    file.write_all(b"WAVE").map_err(|e| e.to_string())?;
    file.write_all(b"fmt ").map_err(|e| e.to_string())?;
    file.write_all(&16u32.to_le_bytes()).map_err(|e| e.to_string())?;
    file.write_all(&1u16.to_le_bytes()).map_err(|e| e.to_string())?;
    file.write_all(&1u16.to_le_bytes()).map_err(|e| e.to_string())?;
    file.write_all(&sample_rate.to_le_bytes()).map_err(|e| e.to_string())?;
    file.write_all(&(sample_rate * 2).to_le_bytes()).map_err(|e| e.to_string())?;
    file.write_all(&2u16.to_le_bytes()).map_err(|e| e.to_string())?;
    file.write_all(&16u16.to_le_bytes()).map_err(|e| e.to_string())?;
    file.write_all(b"data").map_err(|e| e.to_string())?;
    file.write_all(&data_size.to_le_bytes()).map_err(|e| e.to_string())?;
    file.write_all(&data).map_err(|e| e.to_string())?;
    Ok(())
}

// ============================================================================
// CARGA DE SPRITE
// ============================================================================

fn cargar_sprite<'a>(
    path: &str,
    tc: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) -> Option<sdl2::render::Texture<'a>> {
    if !std::path::Path::new(path).exists() { return None; }
    match Surface::from_file(path) {
        Ok(surface) => {
            match tc.create_texture_from_surface(&surface) {
                Ok(tex) => Some(unsafe { std::mem::transmute(tex) }),
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}

// ============================================================================
// NIVELES
// ============================================================================

struct NivelConfig {
    nombre: &'static str,
    enemigo_count: usize,
    enemigos: [(f32, f32, u32, u32, u32, &'static str); 6],
    plataformas: [(i32, i32, u32, u32); 12],
    huecos: [(i32, i32, u32); 4],
}

fn get_nivel(n: usize) -> NivelConfig {
    match n {
        0 => NivelConfig {
            nombre: "Nivel 1: Primer Contacto",
            enemigo_count: 1,
            enemigos: [
                (300.0, 300.0, 16, 16, 4, "tank"),
                (0.0, 0.0, 0, 0, 0, ""),
                (0.0, 0.0, 0, 0, 0, ""),
                (0.0, 0.0, 0, 0, 0, ""),
                (0.0, 0.0, 0, 0, 0, ""),
                (0.0, 0.0, 0, 0, 0, ""),
            ],
            plataformas: [
                (0, 560, 800, 40), (100, 480, 120, 15), (300, 420, 120, 15),
                (500, 360, 120, 15), (200, 280, 150, 15), (450, 220, 120, 15),
                (100, 160, 100, 15), (600, 500, 100, 15), (700, 440, 80, 15),
                (0, 550, 200, 10), (300, 550, 200, 10), (600, 550, 200, 10),
            ],
            huecos: [(200, 550, 100), (500, 550, 100), (0, 0, 0), (0, 0, 0)],
        },
        1 => NivelConfig {
            nombre: "Nivel 2: Refuerzos",
            enemigo_count: 3,
            enemigos: [
                (200.0, 200.0, 16, 16, 4, "tank"),
                (500.0, 150.0, 16, 16, 4, "helicopter"),
                (700.0, 300.0, 8, 8, 4, "crate"),
                (0.0, 0.0, 0, 0, 0, ""),
                (0.0, 0.0, 0, 0, 0, ""),
                (0.0, 0.0, 0, 0, 0, ""),
            ],
            plataformas: [
                (0, 560, 800, 40), (50, 480, 100, 15), (200, 400, 120, 15),
                (400, 340, 100, 15), (600, 280, 120, 15), (150, 200, 100, 15),
                (350, 150, 120, 15), (550, 100, 100, 15), (700, 450, 80, 15),
                (0, 550, 150, 10), (250, 550, 150, 10), (500, 550, 150, 10),
            ],
            huecos: [(150, 550, 100), (400, 550, 100), (650, 550, 100), (0, 0, 0)],
        },
        _ => NivelConfig {
            nombre: "Nivel Final: La Snake Boss",
            enemigo_count: 5,
            enemigos: [
                (150.0, 200.0, 16, 16, 4, "tank"),
                (400.0, 150.0, 16, 16, 4, "helicopter"),
                (600.0, 250.0, 16, 16, 4, "platform"),
                (750.0, 350.0, 8, 8, 4, "crate"),
                (300.0, 100.0, 16, 16, 4, "tank"),
                (0.0, 0.0, 0, 0, 0, ""),
            ],
            plataformas: [
                (0, 560, 800, 40), (100, 480, 80, 15), (250, 400, 100, 15),
                (450, 340, 120, 15), (650, 280, 100, 15), (50, 200, 100, 15),
                (300, 150, 120, 15), (550, 100, 100, 15), (700, 450, 80, 15),
                (0, 550, 120, 10), (200, 550, 120, 10), (400, 550, 120, 10),
            ],
            huecos: [(120, 550, 80), (320, 550, 80), (520, 550, 80), (680, 550, 80)],
        },
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() -> Result<(), String> {
    println!("[RyDit] Torreta vs Sprites");
    println!("======================================\n");

    // Backend SDL2 (patrón demo_rigidbody)
    let mut backend = Sdl2Backend::new("RyDit - Torreta vs Sprites", 800, 600)?;

    // Audio
    let shoot_path = "/data/data/com.termux/files/home/shield-project/jump_sound.wav";
    let hit_path = "/data/data/com.termux/files/home/shield-project/push_sound.wav";
    if !std::path::Path::new(shoot_path).exists() { let _ = generar_tono(shoot_path, 800.0, 0.1); }
    if !std::path::Path::new(hit_path).exists() { let _ = generar_tono(hit_path, 200.0, 0.15); }

    let mut audio: Option<AudioSystemSDL2> = match AudioSystemSDL2::new() {
        Ok(mut a) => {
            let _ = a.load_sound("shoot", shoot_path);
            let _ = a.load_sound("hit", hit_path);
            Some(a)
        }
        Err(_) => None,
    };

    // Fuente TTF
    for path in &["/system/fonts/DroidSans.ttf", "/system/fonts/Roboto-Regular.ttf"] {
        if std::path::Path::new(path).exists() {
            let _ = backend.load_font(path, 18);
            break;
        }
    }

    let tc = &backend.canvas.texture_creator();

    // Texturas de texto pre-cargadas
    let txt_titulo = crear_textura(&backend.font, "[RyDit] Torreta vs Sprites", 255, 255, 255, tc);
    let txt_menu_start = crear_textura(&backend.font, "> START GAME", 0, 255, 100, tc);
    let txt_menu_controls = crear_textura(&backend.font, "- CONTROLES", 100, 200, 255, tc);
    let txt_controls = crear_textura(&backend.font, "← → ó ← → ó A/D = Mover | W ó ↑ = Saltar | SPACE = Disparar | P = Pausa | ESC = Salir", 150, 150, 150, tc);

    // ====================================================================
    // GAME STATE
    // ====================================================================
    let mut state = GameState::Menu;
    let mut camera = Camera { x: 0.0, y: 0.0, map_w: 1200, map_h: 800 };

    let mut torreta = Torreta {
        x: 100.0, y: 400.0, vx: 0.0, vy: 0.0,
        w: 32, h: 32, scale: 1, en_suelo: false,
        vidas: 3, puntaje: 0, dir: 1,
    };

    let mut balas: Vec<Bala> = Vec::new();
    let mut enemigos: Vec<Enemigo> = Vec::new();
    let mut plataformas: Vec<Rect> = Vec::new();
    let mut huecos: Vec<(i32, i32, u32)> = Vec::new();
    let mut nivel_actual: usize = 0;

    // Caches texto dinámico
    let mut txt_info: Option<sdl2::render::Texture<'static>> = None;
    let mut txt_level: Option<sdl2::render::Texture<'static>> = None;
    let mut last_cache = 0u64;

    let mut frame: u64 = 0;
    let mut cooldown: u32 = 0;
    let mut death_timer: u32 = 0;
    let mut win_timer: u32 = 0;
    let mut mensaje: String = String::new();

    println!("Inicializado. Presiona ENTER en el menú.\n");

    'running: loop {
        let dt = 0.016f32;
        frame += 1;
        if cooldown > 0 { cooldown -= 1; }
        if death_timer > 0 { death_timer -= 1; }
        if win_timer > 0 { win_timer -= 1; }

        // ====================================================================
        // INPUT
        // ====================================================================
        for event in backend.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    match state {
                        GameState::Menu => break 'running,
                        GameState::Playing { .. } => state = GameState::Paused { nivel: nivel_actual },
                        GameState::Paused { .. } => state = GameState::Playing { nivel: nivel_actual },
                        _ => state = GameState::Menu,
                    }
                }
                Event::KeyDown { keycode: Some(kc), repeat: false, .. } => match &state {
                    GameState::Menu => {
                        if kc == Keycode::Return || kc == Keycode::Return2 {
                            nivel_actual = 0;
                            cargar_nivel(&mut torreta, &mut enemigos, &mut plataformas, &mut huecos, &mut balas, nivel_actual, &backend.canvas);
                            state = GameState::Playing { nivel: 0 };
                        }
                    }
                    GameState::Playing { nivel } => {
                        match kc {
                            Keycode::Space if cooldown == 0 => {
                                balas.push(Bala {
                                    x: torreta.x + 16.0, y: torreta.y + 10.0,
                                    vx: torreta.dir as f32 * 500.0, activa: true,
                                });
                                cooldown = 15;
                                if let Some(ref mut a) = audio { let _ = a.play_sound("shoot"); }
                            }
                            Keycode::Up | Keycode::W => {
                                if torreta.en_suelo {
                                    torreta.vy = -450.0;
                                    torreta.en_suelo = false;
                                }
                            }
                            Keycode::Down | Keycode::S => {
                                torreta.vy += 200.0;
                            }
                            Keycode::Left | Keycode::A => { torreta.x -= 4.0; torreta.dir = -1; }
                            Keycode::Right | Keycode::D => { torreta.x += 4.0; torreta.dir = 1; }
                            Keycode::P => state = GameState::Paused { nivel: *nivel },
                            Keycode::R => {
                                cargar_nivel(&mut torreta, &mut enemigos, &mut plataformas, &mut huecos, &mut balas, *nivel, &backend.canvas);
                            }
                            _ => {}
                        }
                    }
                    GameState::Paused { nivel } => {
                        if kc == Keycode::P || kc == Keycode::Escape {
                            state = GameState::Playing { nivel: *nivel };
                        }
                    }
                    GameState::GameOver => {
                        if kc == Keycode::Return || kc == Keycode::Return2 {
                            torreta.vidas = 3; torreta.puntaje = 0; nivel_actual = 0;
                            cargar_nivel(&mut torreta, &mut enemigos, &mut plataformas, &mut huecos, &mut balas, 0, &backend.canvas);
                            state = GameState::Playing { nivel: 0 };
                        }
                    }
                    GameState::GameWin => {
                        if kc == Keycode::Return || kc == Keycode::Return2 { state = GameState::Menu; }
                    }
                    GameState::LevelComplete { nivel } => {
                        if kc == Keycode::Return || kc == Keycode::Return2 {
                            nivel_actual = nivel + 1;
                            if nivel_actual > 2 { state = GameState::GameWin; win_timer = 120; }
                            else {
                                cargar_nivel(&mut torreta, &mut enemigos, &mut plataformas, &mut huecos, &mut balas, nivel_actual, &backend.canvas);
                                state = GameState::Playing { nivel: nivel_actual };
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // ====================================================================
        // LÓGICA DEL JUEGO
        // ====================================================================
        if let GameState::Playing { .. } = state {
            // Gravedad torreta
            torreta.vy += 800.0 * dt;
            torreta.y += torreta.vy * dt;

            let t_rect = Rect::new(torreta.x as i32, torreta.y as i32, torreta.w, torreta.h);
            torreta.en_suelo = false;

            for plat in &plataformas {
                if t_rect.has_intersection(*plat) {
                    if t_rect.bottom() as i32 <= plat.y + 10 && torreta.vy > 0.0 {
                        torreta.y = plat.y as f32 - torreta.h as f32;
                        torreta.vy = 0.0;
                        torreta.en_suelo = true;
                    }
                }
            }

            // Verificar huecos (muerte)
            for (hx, hy, hw) in &huecos {
                let h_rect = Rect::new(*hx, *hy, *hw, 10);
                if t_rect.has_intersection(h_rect) && torreta.en_suelo {
                    // Cayendo en hueco
                }
            }

            // Muerte por caer
            if torreta.y > 700.0 {
                torreta.vidas -= 1;
                if torreta.vidas <= 0 {
                    state = GameState::GameOver;
                    death_timer = 120;
                    mensaje = "GAME OVER - Presiona ENTER".to_string();
                } else {
                    torreta.x = 100.0;
                    torreta.y = 100.0;
                    torreta.vy = 0.0;
                    mensaje = format!("-1 vida! Restantes: {}", torreta.vidas);
                }
            }

            // Clamp
            torreta.x = torreta.x.max(0.0).min((camera.map_w - torreta.w as i32) as f32);

            // Balas
            for bala in &mut balas {
                if !bala.activa { continue; }
                bala.x += bala.vx * dt;

                // Colisión bala-enemigo
                for enemigo in &mut enemigos {
                    if !enemigo.vivo { continue; }
                    let e_rect = Rect::new(enemigo.x as i32, enemigo.y as i32, enemigo.w * enemigo.scale, enemigo.h * enemigo.scale);
                    let b_rect = Rect::new(bala.x as i32, bala.y as i32, 8, 4);
                    if b_rect.has_intersection(e_rect) {
                        enemigo.vida -= 1;
                        bala.activa = false;
                        if enemigo.vida <= 0 {
                            enemigo.vivo = false;
                            torreta.puntaje += 100;
                            if let Some(ref mut a) = audio { let _ = a.play_sound("hit"); }
                        }
                    }
                }

                // Fuera de pantalla
                if bala.x < camera.x - 50.0 || bala.x > camera.x + 850.0 {
                    bala.activa = false;
                }
            }
            balas.retain(|b| b.activa);

            // Enemigos IA (patrol)
            for enemigo in &mut enemigos {
                if !enemigo.vivo { continue; }
                enemigo.vx = enemigo.dir as f32 * 50.0;
                enemigo.vy += 600.0 * dt;
                enemigo.x += enemigo.vx * dt;
                enemigo.y += enemigo.vy * dt;

                let e_rect = Rect::new(enemigo.x as i32, enemigo.y as i32, enemigo.w * enemigo.scale, enemigo.h * enemigo.scale);
                enemigo.en_suelo = false;

                for plat in &plataformas {
                    if e_rect.has_intersection(*plat) {
                        if e_rect.bottom() as i32 <= plat.y + 10 && enemigo.vy > 0.0 {
                            enemigo.y = plat.y as f32 - enemigo.h as f32 * enemigo.scale as f32;
                            enemigo.vy = 0.0;
                            enemigo.en_suelo = true;
                        }
                    }
                }

                // Patrol boundaries
                if enemigo.x <= enemigo.patrol_start || enemigo.x >= enemigo.patrol_end {
                    enemigo.dir *= -1;
                }

                // Colisión enemigo-jugador (daño)
                if e_rect.has_intersection(t_rect) {
                    torreta.vidas -= 1;
                    if torreta.vidas <= 0 {
                        state = GameState::GameOver;
                        death_timer = 120;
                        mensaje = "GAME OVER - Presiona ENTER".to_string();
                    } else {
                        mensaje = format!("-1 vida! Restantes: {}", torreta.vidas);
                        torreta.x = 100.0;
                        torreta.y = 100.0;
                        torreta.vy = 0.0;
                    }
                }

                // Respawn si cae
                if enemigo.y > 700.0 {
                    enemigo.x = enemigo.patrol_start;
                    enemigo.y = 50.0;
                    enemigo.vy = 0.0;
                }
            }

            // Verificar nivel completo
            let vivos = enemigos.iter().filter(|e| e.vivo).count();
            if vivos == 0 {
                state = GameState::LevelComplete { nivel: nivel_actual };
                win_timer = 60;
            }

            // Cámara follow
            camera.follow(torreta.x + 16.0, torreta.y + 16.0, 800, 600);

            // Cache texto
            if frame - last_cache > 30 {
                let lvl = get_nivel(nivel_actual);
                txt_level = crear_textura(&backend.font, lvl.nombre, 0, 200, 255, tc).map(|t| unsafe { std::mem::transmute(t) });
                txt_info = crear_textura(&backend.font,
                    &format!("HP: {} | Score: {} | Enemigos: {}", torreta.vidas, torreta.puntaje, vivos),
                    0, 255, 0, tc).map(|t| unsafe { std::mem::transmute(t) });
                last_cache = frame;
            }
        }

        // ====================================================================
        // RENDER
        // ====================================================================
        backend.clear_background(ColorRydit::Negro);

        match &state {
            GameState::Menu => {
                // Fondo estrellas
                backend.canvas.set_draw_color(Color::RGB(40, 40, 60));
                for i in 0..50 {
                    let sx = ((i * 137 + frame as usize) % 800) as i32;
                    let sy = ((i * 251 + frame as usize / 20) % 600) as i32;
                    let _ = backend.canvas.fill_rect(Rect::new(sx, sy, 2, 2));
                }

                // Título
                if let Some(ref tex) = txt_titulo {
                    let q = tex.query();
                    let _ = backend.canvas.copy(tex, None, Rect::new(150, 150, q.width, q.height));
                }

                // Opciones de menú
                let blink = (frame / 30) % 2 == 0;
                if blink {
                    if let Some(ref tex) = txt_menu_start {
                        let q = tex.query();
                        let _ = backend.canvas.copy(tex, None, Rect::new(300, 280, q.width, q.height));
                    }
                }
                if let Some(ref tex) = txt_menu_controls {
                    let q = tex.query();
                    let _ = backend.canvas.copy(tex, None, Rect::new(300, 330, q.width, q.height));
                }
                if let Some(ref tex) = txt_controls {
                    let q = tex.query();
                    let _ = backend.canvas.copy(tex, None, Rect::new(100, 550, q.width, q.height));
                }
            }

            GameState::Playing { .. } | GameState::Paused { .. } => {
                // Fondo estrellas (parallax ligero)
                backend.canvas.set_draw_color(Color::RGB(20, 20, 35));
                for i in 0..60 {
                    let parallax = 0.3;
                    let sx = (((i * 137) as f32 - camera.x * parallax) as i32 % 800 + 800) % 800;
                    let sy = (((i * 251) as f32 - camera.y * parallax) as i32 % 600 + 600) % 600;
                    let _ = backend.canvas.fill_rect(Rect::new(sx, sy, 2, 2));
                }

                // Plataformas
                backend.canvas.set_draw_color(Color::RGB(80, 80, 100));
                for plat in &plataformas {
                    let sx = camera.screen_x(plat.x as f32);
                    let sy = camera.screen_y(plat.y as f32);
                    if sx > -200 && sx < 1000 {
                        let _ = backend.canvas.fill_rect(Rect::new(sx, sy, plat.width(), plat.height()));
                        // Borde brillante
                        backend.canvas.set_draw_color(Color::RGB(0, 200, 100));
                        let _ = backend.canvas.fill_rect(Rect::new(sx, sy, plat.width(), 3));
                        backend.canvas.set_draw_color(Color::RGB(80, 80, 100));
                    }
                }

                // Huecos (indicador rojo)
                backend.canvas.set_draw_color(Color::RGBA(255, 0, 0, 80));
                for (hx, hy, hw) in &huecos {
                    let sx = camera.screen_x(*hx as f32);
                    let sy = camera.screen_y(*hy as f32);
                    if sx > -200 && sx < 1000 {
                        let _ = backend.canvas.fill_rect(Rect::new(sx, sy, *hw, 10));
                    }
                }

                // Enemigos
                for enemigo in &enemigos {
                    if !enemigo.vivo { continue; }
                    let sx = camera.screen_x(enemigo.x);
                    let sy = camera.screen_y(enemigo.y);
                    let w = enemigo.w * enemigo.scale;
                    let h = enemigo.h * enemigo.scale;

                    if sx > -100 && sx < 900 {
                        if let Some(ref tex) = enemigo.textura {
                            let _ = backend.canvas.copy(tex, None, Rect::new(sx, sy, w, h));
                        } else {
                            backend.canvas.set_draw_color(enemigo.color);
                            let _ = backend.canvas.fill_rect(Rect::new(sx, sy, w, h));
                        }
                        // Barra de vida
                        if enemigo.vida > 1 {
                            backend.canvas.set_draw_color(Color::RGB(255, 0, 0));
                            let _ = backend.canvas.fill_rect(Rect::new(sx, sy - 6, w, 4));
                            backend.canvas.set_draw_color(Color::RGB(0, 255, 0));
                            let _ = backend.canvas.fill_rect(Rect::new(sx, sy - 6, (w as i32 * enemigo.vida / 3).max(0) as u32, 4));
                        }
                    }
                }

                // Balas
                backend.canvas.set_draw_color(Color::RGB(255, 255, 0));
                for bala in &balas {
                    let sx = camera.screen_x(bala.x);
                    let sy = camera.screen_y(bala.y);
                    if sx > -20 && sx < 820 {
                        let _ = backend.canvas.fill_rect(Rect::new(sx, sy, 8, 4));
                    }
                }

                // Torreta
                let tsx = camera.screen_x(torreta.x);
                let tsy = camera.screen_y(torreta.y);
                backend.canvas.set_draw_color(Color::RGB(50, 50, 200));
                let _ = backend.canvas.fill_rect(Rect::new(tsx, tsy, torreta.w, torreta.h));
                // Cañón
                let cannon_x = if torreta.dir > 0 { tsx + 28 } else { tsx - 12 };
                backend.canvas.set_draw_color(Color::RGB(100, 100, 255));
                let _ = backend.canvas.fill_rect(Rect::new(cannon_x, tsy + 12, 16, 6));
                // Ojos
                backend.canvas.set_draw_color(Color::RGB(255, 255, 255));
                let ojo_off = if torreta.dir > 0 { 18 } else { 4 };
                let _ = backend.canvas.fill_rect(Rect::new(tsx + ojo_off, tsy + 8, 4, 4));
                let _ = backend.canvas.fill_rect(Rect::new(tsx + ojo_off + 8, tsy + 8, 4, 4));

                // HUD texto
                if let Some(ref tex) = txt_level {
                    let q = tex.query();
                    let _ = backend.canvas.copy(tex, None, Rect::new(10, 10, q.width, q.height));
                }
                if let Some(ref tex) = txt_info {
                    let q = tex.query();
                    let _ = backend.canvas.copy(tex, None, Rect::new(10, 30, q.width, q.height));
                }

                // Mensaje temporal
                if !mensaje.is_empty() && (death_timer > 0 || win_timer > 0) {
                    let alpha = if (frame / 10) % 2 == 0 { 255 } else { 150 };
                    backend.canvas.set_draw_color(Color::RGBA(0, 0, 0, alpha));
                    let _ = backend.canvas.fill_rect(Rect::new(200, 250, 400, 60));
                    backend.canvas.set_draw_color(Color::RGBA(255, 255, 255, alpha));
                    let _ = backend.canvas.draw_rect(Rect::new(200, 250, 400, 60));
                    // Texto simple (bloques)
                    for (ci, _ch) in mensaje.chars().enumerate().take(30) {
                        let _ = backend.canvas.fill_rect(Rect::new(220 + ci as i32 * 10, 270, 8, 16));
                    }
                }

                // Pausa overlay
                if matches!(state, GameState::Paused { .. }) {
                    backend.canvas.set_draw_color(Color::RGBA(0, 0, 0, 150));
                    let _ = backend.canvas.fill_rect(Rect::new(0, 0, 800, 600));
                    if let Some(ref tex) = crear_textura(&backend.font, "PAUSA - ESC para continuar", 255, 255, 0, tc) {
                        let q = tex.query();
                        let _ = backend.canvas.copy(tex, None, Rect::new(150, 280, q.width, q.height));
                    }
                }
            }

            GameState::GameOver => {
                backend.canvas.set_draw_color(Color::RGBA(80, 0, 0, 200));
                let _ = backend.canvas.fill_rect(Rect::new(0, 0, 800, 600));
                if let Some(ref tex) = crear_textura(&backend.font, "GAME OVER", 255, 0, 0, tc) {
                    let q = tex.query();
                    let _ = backend.canvas.copy(tex, None, Rect::new(250, 200, q.width, q.height));
                }
                let msg = format!("Puntaje final: {} | Presiona ENTER para reiniciar", torreta.puntaje);
                if let Some(ref tex) = crear_textura(&backend.font, &msg, 200, 200, 200, tc) {
                    let q = tex.query();
                    let _ = backend.canvas.copy(tex, None, Rect::new(100, 300, q.width, q.height));
                }
            }

            GameState::GameWin => {
                backend.canvas.set_draw_color(Color::RGBA(0, 50, 0, 200));
                let _ = backend.canvas.fill_rect(Rect::new(0, 0, 800, 600));
                if let Some(ref tex) = crear_textura(&backend.font, "GAME WIN - La Snake Boss ha caído", 0, 255, 0, tc) {
                    let q = tex.query();
                    let _ = backend.canvas.copy(tex, None, Rect::new(50, 200, q.width, q.height));
                }
                let msg = format!("Puntaje final: {} | Presiona ENTER para menú", torreta.puntaje);
                if let Some(ref tex) = crear_textura(&backend.font, &msg, 200, 255, 200, tc) {
                    let q = tex.query();
                    let _ = backend.canvas.copy(tex, None, Rect::new(100, 300, q.width, q.height));
                }
            }

            GameState::LevelComplete { .. } => {
                backend.canvas.set_draw_color(Color::RGBA(0, 30, 60, 200));
                let _ = backend.canvas.fill_rect(Rect::new(0, 0, 800, 600));
                if let Some(ref tex) = crear_textura(&backend.font, "Nivel Completado", 0, 255, 100, tc) {
                    let q = tex.query();
                    let _ = backend.canvas.copy(tex, None, Rect::new(200, 250, q.width, q.height));
                }
                if let Some(ref tex) = crear_textura(&backend.font, "Presiona ENTER para siguiente nivel", 200, 200, 200, tc) {
                    let q = tex.query();
                    let _ = backend.canvas.copy(tex, None, Rect::new(150, 320, q.width, q.height));
                }
            }
        }

        backend.end_draw();
    }

    println!("\nDemo completado: {} frames | Puntaje: {}", frame, torreta.puntaje);
    Ok(())
}

// ============================================================================
// CARGA DE NIVEL
// ============================================================================

fn cargar_nivel(
    torreta: &mut Torreta,
    enemigos: &mut Vec<Enemigo>,
    plataformas: &mut Vec<Rect>,
    huecos: &mut Vec<(i32, i32, u32)>,
    balas: &mut Vec<Bala>,
    nivel: usize,
    canvas: &sdl2::render::Canvas<sdl2::video::Window>,
) {
    let config = get_nivel(nivel);
    let tc = canvas.texture_creator();

    torreta.x = 100.0;
    torreta.y = 400.0;
    torreta.vx = 0.0;
    torreta.vy = 0.0;
    torreta.en_suelo = false;
    balas.clear();

    // Plataformas
    plataformas.clear();
    for (x, y, w, h) in &config.plataformas {
        if *w > 0 {
            plataformas.push(Rect::new(*x, *y, *w, *h));
        }
    }

    // Huecos
    huecos.clear();
    for h in &config.huecos {
        if h.2 > 0 {
            huecos.push(*h);
        }
    }

    // Enemigos
    enemigos.clear();
    let sprites_dir = "/data/data/com.termux/files/home/shield-project/logo_icon_asst/sprites";
    for i in 0..config.enemigo_count {
        let (ex, ey, ew, eh, escale, nombre) = config.enemigos[i];
        if ew == 0 { continue; }

        let path = format!("{}/{}_16x16.png", sprites_dir, nombre);
        let textura = cargar_sprite(&path, &tc);
        let fallback_path = format!("{}/{}.png", sprites_dir, nombre);
        let textura = textura.or_else(|| cargar_sprite(&fallback_path, &tc));

        enemigos.push(Enemigo {
            nombre: nombre.to_string(),
            textura: textura.map(|t| unsafe { std::mem::transmute(t) }),
            x: ex, y: ey,
            vx: 0.0, vy: 0.0,
            w: ew, h: eh, scale: escale,
            color: Color::RGB(255, 100, 100),
            vivo: true, en_suelo: false, dir: 1,
            patrol_start: ex - 100.0,
            patrol_end: ex + 100.0,
            vida: 3,
        });
    }

    println!("Nivel {}: {} enemigos, {} plataformas", nivel + 1, config.enemigo_count, plataformas.len());
}
