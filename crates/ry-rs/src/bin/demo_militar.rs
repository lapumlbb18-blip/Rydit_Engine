// demo_militar.rs
// Demo: Soldado con sprite procedural + partículas de disparo/explosión/humo
//
// Muestra:
// - Sprite procedural de soldado (cuerpo, cabeza, casco, arma)
// - Animaciones: idle, run, shoot
// - Partículas: muzzle flash, sparks, explosión, humo
// - Objetivos/enemigos para disparar
// - Escenario militar con trinchera
//
// Controles:
// ← → / A D: Mover
// SPACE: Disparar (con partículas)
// E: Lanzar granada (explosión + humo)
// R: Recargar (resetear munición)
// ESC: Salir

use ry_anim::anim_particles::{Particle, ParticleSystem};
use ry_gfx::backend_sdl2::Sdl2Backend;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashMap;

// ============================================================================
// RAND MEJORADO
// ============================================================================
fn rand_f64() -> f64 {
    static mut SEED: u32 = 54321;
    unsafe {
        SEED = SEED.wrapping_mul(1664525).wrapping_add(1013904223);
        (SEED as f64) / (u32::MAX as f64)
    }
}

fn rand_range(min: f64, max: f64) -> f64 {
    min + rand_f64() * (max - min)
}

// ============================================================================
// SPRITE PROCEDURAL DE SOLDADO
// ============================================================================
struct SoldierSprite {
    x: f32,
    y: f32,
    width: i32,
    height: i32,
    direccion: i32, // 1 = derecha, -1 = izquierda
    estado: SoldierState,
    frame_timer: f32,
    anim_frame: u32,
}

#[derive(PartialEq, Clone, Copy)]
enum SoldierState {
    Idle,
    Run,
    Shoot,
}

impl SoldierSprite {
    fn new() -> Self {
        let width = 64;
        let height = 96;

        Self {
            x: 200.0,
            y: 400.0,
            width: width as i32,
            height: height as i32,
            direccion: 1,
            estado: SoldierState::Idle,
            frame_timer: 0.0,
            anim_frame: 0,
        }
    }

    fn draw_soldier(surface: &mut sdl2::surface::Surface, direccion: i32) {
        // Limpiar con transparencia
        let _ = surface.fill_rect(None, Color::RGBA(0, 0, 0, 0));

        let w = surface.width() as i32;
        let flip_x = if direccion < 0 { w } else { 0 };
        let scale_x = if direccion < 0 { -1 } else { 1 };

        let rect = |x: i32, y: i32, w: u32, h: u32| -> Rect {
            Rect::new(
                if direccion < 0 { (w as i32) - x - (w as i32) } else { x },
                y,
                w,
                h,
            )
        };

        // Botas (negras)
        let _ = surface.fill_rect(
            if direccion > 0 { Rect::new(18, 80, 12, 16) } else { Rect::new(34, 80, 12, 16) },
            Color::RGB(30, 30, 30),
        );
        let _ = surface.fill_rect(
            if direccion > 0 { Rect::new(34, 80, 12, 16) } else { Rect::new(18, 80, 12, 16) },
            Color::RGB(30, 30, 30),
        );

        // Piernas (verde oliva)
        let _ = surface.fill_rect(
            if direccion > 0 { Rect::new(20, 62, 10, 20) } else { Rect::new(34, 62, 10, 20) },
            Color::RGB(85, 107, 47),
        );
        let _ = surface.fill_rect(
            if direccion > 0 { Rect::new(34, 62, 10, 20) } else { Rect::new(20, 62, 10, 20) },
            Color::RGB(75, 97, 37),
        );

        // Cuerpo (verde militar)
        let _ = surface.fill_rect(
            Rect::new(16, 36, 32, 28),
            Color::RGB(107, 142, 35),
        );

        // Chaleco táctico (marrón)
        let _ = surface.fill_rect(
            Rect::new(18, 38, 28, 16),
            Color::RGB(139, 90, 43),
        );

        // Bolsillos del chaleco
        let _ = surface.fill_rect(
            if direccion > 0 { Rect::new(20, 42, 8, 6) } else { Rect::new(36, 42, 8, 6) },
            Color::RGB(120, 75, 35),
        );
        let _ = surface.fill_rect(
            if direccion > 0 { Rect::new(32, 42, 8, 6) } else { Rect::new(24, 42, 8, 6) },
            Color::RGB(120, 75, 35),
        );

        // Brazos
        let _ = surface.fill_rect(
            if direccion > 0 { Rect::new(10, 38, 8, 22) } else { Rect::new(46, 38, 8, 22) },
            Color::RGB(95, 125, 30),
        );
        let _ = surface.fill_rect(
            if direccion > 0 { Rect::new(46, 38, 8, 22) } else { Rect::new(10, 38, 8, 22) },
            Color::RGB(85, 115, 25),
        );

        // Cabeza (piel)
        let _ = surface.fill_rect(
            Rect::new(24, 16, 16, 18),
            Color::RGB(210, 180, 140),
        );

        // Casco (verde oscuro)
        let _ = surface.fill_rect(
            Rect::new(20, 10, 24, 12),
            Color::RGB(50, 80, 30),
        );
        // Borde del casco
        let _ = surface.fill_rect(
            Rect::new(18, 18, 28, 4),
            Color::RGB(45, 70, 25),
        );

        // Visor del casco
        let _ = surface.fill_rect(
            if direccion > 0 { Rect::new(36, 14, 8, 6) } else { Rect::new(20, 14, 8, 6) },
            Color::RGB(35, 55, 20),
        );

        // Ojos
        let eye_x = if direccion > 0 { 32 } else { 26 };
        let _ = surface.fill_rect(
            Rect::new(eye_x, 22, 3, 3),
            Color::RGB(40, 40, 40),
        );
        let eye_x2 = if direccion > 0 { 38 } else { 32 };
        let _ = surface.fill_rect(
            Rect::new(eye_x2, 22, 3, 3),
            Color::RGB(40, 40, 40),
        );

        // Arma (gris oscuro/negro)
        let gun_x = if direccion > 0 { 42 } else { 4 };
        let _ = surface.fill_rect(
            Rect::new(gun_x, 44, 18, 6),
            Color::RGB(50, 50, 50),
        );
        // Cañón
        let barrel_x = if direccion > 0 { 56 } else { 0 };
        let _ = surface.fill_rect(
            Rect::new(barrel_x, 45, 8, 4),
            Color::RGB(40, 40, 40),
        );
        // Mango del arma
        let grip_x = if direccion > 0 { 44 } else { 12 };
        let _ = surface.fill_rect(
            Rect::new(grip_x, 50, 5, 8),
            Color::RGB(60, 60, 60),
        );
        // Mira
        let sight_x = if direccion > 0 { 50 } else { 6 };
        let _ = surface.fill_rect(
            Rect::new(sight_x, 42, 4, 3),
            Color::RGB(70, 70, 70),
        );

        // Cargador
        let mag_x = if direccion > 0 { 46 } else { 10 };
        let _ = surface.fill_rect(
            Rect::new(mag_x, 50, 4, 10),
            Color::RGB(55, 55, 55),
        );
    }

    fn update(&mut self, dt: f32) {
        self.frame_timer += dt;

        match self.estado {
            SoldierState::Idle => {
                if self.frame_timer > 0.5 {
                    self.frame_timer = 0.0;
                    self.anim_frame = (self.anim_frame + 1) % 4;
                }
            }
            SoldierState::Run => {
                if self.frame_timer > 0.1 {
                    self.frame_timer = 0.0;
                    self.anim_frame = (self.anim_frame + 1) % 8;
                }
            }
            SoldierState::Shoot => {
                if self.frame_timer > 0.15 {
                    self.frame_timer = 0.0;
                    self.anim_frame += 1;
                    if self.anim_frame >= 3 {
                        self.estado = SoldierState::Idle;
                        self.anim_frame = 0;
                    }
                }
            }
        }
    }

    fn shoot(&mut self) {
        self.estado = SoldierState::Shoot;
        self.anim_frame = 0;
        self.frame_timer = 0.0;
    }

    fn texture<'a>(&self, tc: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Result<sdl2::render::Texture<'a>, String> {
        let mut surface = sdl2::surface::Surface::new(self.width as u32, self.height as u32, sdl2::pixels::PixelFormatEnum::RGBA8888)
            .map_err(|e| e.to_string())?;
        Self::draw_soldier(&mut surface, self.direccion);
        tc.create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
    }
}

// ============================================================================
// OBJETIVO/ENEMIGO
// ============================================================================
struct Target {
    x: f32,
    y: f32,
    width: i32,
    height: i32,
    alive: bool,
    hit_timer: f64,
    color: (u8, u8, u8),
}

impl Target {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            width: 48,
            height: 72,
            alive: true,
            hit_timer: 0.0,
            color: (180, 50, 50),
        }
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        if !self.alive {
            return;
        }

        if self.hit_timer > 0.0 {
            // Flash blanco al recibir impacto
            canvas.set_draw_color(Color::WHITE);
        } else {
            canvas.set_draw_color(Color::RGB(self.color.0, self.color.1, self.color.2));
        }

        let x = self.x as i32;
        let y = self.y as i32;
        let w = self.width;
        let h = self.height;

        // Cuerpo
        let _ = canvas.fill_rect(Rect::new(x + 8, y + 20, (w - 16) as u32, (h - 36) as u32));
        // Cabeza
        let _ = canvas.fill_rect(Rect::new(x + 14, y + 4, (w - 28) as u32, 18));
        // Piernas
        let _ = canvas.fill_rect(Rect::new(x + 10, y + h - 20, 10, 20));
        let _ = canvas.fill_rect(Rect::new(x + w - 20, y + h - 20, 10, 20));

        // Ojos
        canvas.set_draw_color(Color::BLACK);
        let _ = canvas.fill_rect(Rect::new(x + 18, y + 10, 4, 4));
        let _ = canvas.fill_rect(Rect::new(x + 28, y + 10, 4, 4));

        // Reset color
        canvas.set_draw_color(Color::RGB(180, 50, 50));
    }

    fn hit(&mut self) {
        self.hit_timer = 0.2;
    }

    fn update(&mut self, dt: f32) {
        if self.hit_timer > 0.0 {
            self.hit_timer -= dt as f64;
        }
    }

    fn contains(&self, px: f32, py: f32) -> bool {
        self.alive &&
            px >= self.x && px <= self.x + self.width as f32 &&
            py >= self.y && py <= self.y + self.height as f32
    }
}

// ============================================================================
// BALA/PROYECTIL
// ============================================================================
struct Bullet {
    x: f32,
    y: f32,
    vx: f32,
    active: bool,
}

struct Granada {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    timer: f32,
    exploded: bool,
}

// ============================================================================
// TEXT HELPER
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
    println!("🎖️  RyDit - Demo Militar con Partículas");
    println!("← → / A D: Mover | W/↑: Saltar | SPACE: Disparar | E: Granada | R: Recargar | ESC: Salir\n");

    let mut backend = Sdl2Backend::new("Demo Militar - RyDit", 900, 600)?;

    // Fuente
    for p in &["/system/fonts/DroidSans.ttf", "/data/data/com.termux/files/usr/share/fonts/noto-sans/NotoSans-Regular.ttf"] {
        if std::path::Path::new(p).exists() {
            let _ = backend.load_font(p, 14);
            break;
        }
    }

    // ========================================================================
    // OBJETOS DEL JUEGO
    // ========================================================================
    let mut soldado = SoldierSprite::new();
    let mut balas: Vec<Bullet> = Vec::new();
    let mut municion = 30;
    let mut score = 0;
    let mut granadas: Vec<Granada> = Vec::new();

    // Física de salto del soldado
    let mut soldado_vy: f32 = 0.0;
    let mut soldado_en_suelo = true;
    let suelo_y: f32 = 400.0; // Posición base del soldado
    let gravedad: f32 = 900.0;
    let fuerza_salto: f32 = -350.0;

    // Objetivos
    let mut objetivos = vec![
        Target::new(600.0, 380.0),
        Target::new(700.0, 350.0),
        Target::new(780.0, 390.0),
        Target::new(650.0, 320.0),
    ];

    // Sistema de partículas local
    let mut particle_sys = ParticleSystem::new();

    // Efectos de partículas a renderizar
    let mut muzzle_flash_pos: Option<(f32, f32, f32)> = None;
    let mut impact_effects: Vec<(f32, f32, String, f32)> = Vec::new();

    // Texturas HUD cacheadas
    let tc = &backend.canvas.texture_creator();
    let mut txt_hud: Option<sdl2::render::Texture<'static>> = None;
    let mut txt_municion: Option<sdl2::render::Texture<'static>> = None;

    // Cooldown para disparo
    let mut shoot_cooldown: f32 = 0.0;
    const SHOOT_COOLDOWN: f32 = 0.25; // 250ms entre disparos

    // ========================================================================
    // GAME LOOP
    // ========================================================================
    let mut running = true;
    let mut frame_count: u64 = 0;

    'run: loop {
        let frame_start = std::time::Instant::now();
        let dt = 0.016; // ~60 FPS

        // ---- INPUT (como demo_torreta_vs_sprites) ----
        for ev in backend.event_pump.poll_iter() {
            match ev {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false; break 'run;
                }
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    // DISPARAR
                    if municion > 0 && shoot_cooldown <= 0.0 {
                        municion -= 1;
                        shoot_cooldown = SHOOT_COOLDOWN;
                        soldado.shoot();

                        // Crear bala
                        let bullet_x = soldado.x + if soldado.direccion > 0 { 72.0 } else { -8.0 };
                        let bullet_y = soldado.y + 48.0;
                        let bullet_vx = soldado.direccion as f32 * 600.0;
                        balas.push(Bullet {
                            x: bullet_x,
                            y: bullet_y,
                            vx: bullet_vx,
                            active: true,
                        });

                        // Muzzle flash - partículas de disparo
                        let flash_x = soldado.x + if soldado.direccion > 0 { 76.0 } else { -12.0 };
                        let flash_y = soldado.y + 46.0;
                        muzzle_flash_pos = Some((flash_x, flash_y, 0.1));

                        // Emitir sparks en dirección del disparo
                        particle_sys.emit(flash_x as f64, flash_y as f64, "spark", 15);

                        frame_count += 1;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::E), repeat: false, .. } => {
                    // LANZAR GRANADA con trayectoria en arco
                    let gx = soldado.x + 32.0;
                    let gy = soldado.y;
                    let gvx = soldado.direccion as f32 * 200.0; // velocidad horizontal
                    let gvy = -300.0; // velocidad vertical inicial (hacia arriba)
                    granadas.push(Granada {
                        x: gx,
                        y: gy,
                        vx: gvx,
                        vy: gvy,
                        timer: 1.5, // 1.5 segundos para explosión
                        exploded: false,
                    });
                }
                Event::KeyDown { keycode: Some(Keycode::R), repeat: false, .. } => {
                    municion = 30;
                }
                Event::KeyDown { keycode: Some(kc), .. } => {
                    match kc {
                        Keycode::Up | Keycode::W => {
                            // SALTAR
                            if soldado_en_suelo {
                                soldado_vy = fuerza_salto;
                                soldado_en_suelo = false;
                            }
                        }
                        Keycode::Left | Keycode::A => {
                            soldado.x -= 5.0;
                            soldado.direccion = -1;
                            if soldado.estado != SoldierState::Shoot && soldado_en_suelo {
                                soldado.estado = SoldierState::Run;
                            }
                        }
                        Keycode::Right | Keycode::D => {
                            soldado.x += 5.0;
                            soldado.direccion = 1;
                            if soldado.estado != SoldierState::Shoot && soldado_en_suelo {
                                soldado.estado = SoldierState::Run;
                            }
                        }
                        _ => {}
                    }
                }
                Event::KeyUp { keycode: Some(kc), .. } => {
                    match kc {
                        Keycode::Left | Keycode::A | Keycode::Right | Keycode::D => {
                            if soldado.estado == SoldierState::Run {
                                soldado.estado = SoldierState::Idle;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // ---- UPDATE SOLDADO (gravedad y salto) ----
        // Cooldown disparo
        if shoot_cooldown > 0.0 {
            shoot_cooldown -= dt as f32;
        }

        // Gravedad
        if !soldado_en_suelo {
            soldado_vy += gravedad * dt;
            soldado.y += soldado_vy * dt;
            
            if soldado.y >= suelo_y {
                soldado.y = suelo_y;
                soldado_vy = 0.0;
                soldado_en_suelo = true;
                if soldado.estado == SoldierState::Shoot {
                    // mantener estado
                } else {
                    soldado.estado = SoldierState::Idle;
                }
            }
        }

        soldado.x = soldado.x.max(0.0).min(900.0 - 64.0);
        soldado.update(dt as f32);

        // ---- UPDATE BALAS ----
        for bala in &mut balas {
            if !bala.active { continue; }
            bala.x += bala.vx * dt;

            // Verificar colisión con objetivos
            for obj in &mut objetivos {
                if obj.alive && obj.contains(bala.x, bala.y) {
                    bala.active = false;
                    obj.hit();
                    obj.hit_timer = 0.3;

                    // Impact effect - explosión pequeña + humo
                    impact_effects.push((bala.x, bala.y, "explosion".to_string(), 0.4));
                    impact_effects.push((bala.x, bala.y, "smoke".to_string(), 0.8));

                    particle_sys.emit(bala.x as f64, bala.y as f64, "explosion", 20);
                    particle_sys.emit(bala.x as f64, bala.y as f64, "smoke", 10);

                    score += 100;
                    break;
                }
            }

            // Fuera de pantalla
            if bala.x < -20.0 || bala.x > 920.0 {
                bala.active = false;
            }
        }
        balas.retain(|b| b.active);

        // ---- UPDATE GRANADAS (con trayectoria en arco) ----
        let granada_gravedad: f32 = 500.0;
        for g in &mut granadas {
            if g.exploded { continue; }
            
            g.timer -= dt as f32;
            
            // Física de proyectil
            g.vy += granada_gravedad * dt; // gravedad
            g.x += g.vx * dt;
            g.y += g.vy * dt;
            
            // Si toca el suelo
            if g.y >= suelo_y {
                g.y = suelo_y;
                g.vx = 0.0;
                g.vy = 0.0;
            }
            
            if g.timer <= 0.0 && !g.exploded {
                g.exploded = true;
                // Explosión grande
                impact_effects.push((g.x, g.y, "explosion".to_string(), 0.6));
                impact_effects.push((g.x, g.y, "smoke".to_string(), 1.5));

                particle_sys.emit(g.x as f64, g.y as f64, "explosion", 50);
                particle_sys.emit(g.x as f64, g.y as f64, "fire", 30);
                particle_sys.emit(g.x as f64, g.y as f64, "smoke", 25);

                // Dañar objetivos cercanos
                for obj in &mut objetivos {
                    let dx = (obj.x as f32 + obj.width as f32 / 2.0) - g.x;
                    let dy = (obj.y as f32 + obj.height as f32 / 2.0) - g.y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < 100.0 && obj.alive {
                        obj.alive = false;
                        score += 200;
                    }
                }
            }
        }
        granadas.retain(|g| g.timer > -2.0 || !g.exploded); // Mantener 2 segundos post-explosión para efectos

        // ---- UPDATE OBJETIVOS ----
        for obj in &mut objetivos {
            obj.update(dt as f32);
        }

        // ---- UPDATE PARTÍCULAS ----
        particle_sys.update();

        // ---- UPDATE EFECTOS VISUALES ----
        if let Some(ref mut flash) = muzzle_flash_pos {
            flash.2 -= dt as f32;
        }
        muzzle_flash_pos = muzzle_flash_pos.filter(|f| f.2 > 0.0);

        for eff in &mut impact_effects {
            eff.3 -= dt as f32;
        }
        impact_effects.retain(|e| e.3 > 0.0);

        // ---- HUD TEXT ----
        frame_count += 1;
        if frame_count % 20 == 0 {
            let estado_str = match soldado.estado {
                SoldierState::Idle => "IDLE",
                SoldierState::Run => "RUN",
                SoldierState::Shoot => "SHOOT",
            };
            let txt = format!(
                "🎖️ DEMO MILITAR | Score: {} | Estado: {} | Partículas: {}",
                score,
                estado_str,
                particle_sys.count()
            );
            txt_hud = crear_textura(&backend.font, &txt, 230, 230, 240, tc)
                .map(|t| unsafe { std::mem::transmute(t) });

            let mun_txt = format!("Munición: {}/30", municion);
            txt_municion = crear_textura(&backend.font, &mun_txt, 255, 220, 50, tc)
                .map(|t| unsafe { std::mem::transmute(t) });
        }

        // ---- RENDER ----
        // Cielo (gradiente azul oscuro)
        backend.canvas.set_draw_color(Color::RGB(25, 25, 40));
        backend.canvas.clear();

        // Montañas de fondo
        backend.canvas.set_draw_color(Color::RGB(40, 50, 60));
        let _ = backend.canvas.fill_rect(Rect::new(0, 250, 900, 150));
        backend.canvas.set_draw_color(Color::RGB(50, 60, 70));
        // Picos de montaña
        for i in 0..9 {
            let mx = i * 120;
            let mh = 60 + (i % 3) * 30;
            let pts = [
                (mx, 250),
                (mx + 60, 250 - mh),
                (mx + 120, 250),
            ];
            // Aproximación con rects
            let _ = backend.canvas.fill_rect(Rect::new(mx + 40, 250 - mh + 10, 40, (mh - 10) as u32));
            let _ = backend.canvas.fill_rect(Rect::new(mx + 30, 250 - mh + 20, 60, (mh - 20) as u32));
        }

        // Suelo (tierra)
        backend.canvas.set_draw_color(Color::RGB(120, 95, 65));
        let _ = backend.canvas.fill_rect(Rect::new(0, 450, 900, 150));

        // Línea de suelo
        backend.canvas.set_draw_color(Color::RGB(100, 80, 50));
        let _ = backend.canvas.fill_rect(Rect::new(0, 448, 900, 4));

        // Trinchera del soldado
        backend.canvas.set_draw_color(Color::RGB(90, 75, 55));
        let _ = backend.canvas.fill_rect(Rect::new(
            soldado.x as i32 - 10,
            soldado.y as i32 + soldado.height - 8,
            (soldado.width + 20) as u32,
            16,
        ));
        // Sacos de arena
        backend.canvas.set_draw_color(Color::RGB(160, 140, 100));
        for i in 0..4 {
            let _ = backend.canvas.fill_rect(Rect::new(
                soldado.x as i32 - 15 + i * 22,
                soldado.y as i32 + soldado.height + 4,
                20,
                10,
            ));
        }

        // Dibujar objetivos
        for obj in &objetivos {
            obj.draw(&mut backend.canvas);
        }

        // Dibujar balas
        backend.canvas.set_draw_color(Color::RGB(255, 255, 100));
        for bala in &balas {
            if !bala.active { continue; }
            let _ = backend.canvas.fill_rect(Rect::new(bala.x as i32, bala.y as i32, 8, 3));
        }

        // Dibujar soldado
        if let Ok(tex) = soldado.texture(tc) {
            let scale = 1.0;
            let dest_w = (soldado.width as f32 * scale) as u32;
            let dest_h = (soldado.height as f32 * scale) as u32;
            let dest_x = soldado.x as i32;
            let dest_y = soldado.y as i32;
            backend.canvas.copy(
                &tex,
                None,
                Rect::new(dest_x, dest_y, dest_w, dest_h),
            ).ok();
        }

        // Muzzle flash visual
        if let Some((fx, fy, timer)) = muzzle_flash_pos {
            let alpha = (timer / 0.1 * 255.0) as u8;
            backend.canvas.set_draw_color(Color::RGBA(255, 200, 50, alpha));
            let flash_size = ((timer / 0.1 * 16.0) as i32).max(1);
            let fs = flash_size as u32;
            let _ = backend.canvas.fill_rect(Rect::new(
                fx as i32 - flash_size / 2,
                fy as i32 - flash_size / 2,
                fs,
                fs,
            ));
            // Núcleo blanco
            backend.canvas.set_draw_color(Color::RGBA(255, 255, 255, alpha));
            let core = (flash_size / 2).max(1) as u32;
            let _ = backend.canvas.fill_rect(Rect::new(
                fx as i32 - flash_size / 4,
                fy as i32 - flash_size / 4,
                core,
                core,
            ));
        }

        // Granadas (con trayectoria en arco)
        for g in &granadas {
            if !g.exploded {
                // Granada volando - punto verde/rojo parpadeante
                let blink = (g.timer * 10.0).sin() > 0.0;
                if blink {
                    backend.canvas.set_draw_color(Color::RGB(50, 200, 50));
                } else {
                    backend.canvas.set_draw_color(Color::RGB(200, 50, 50));
                }
                let _ = backend.canvas.fill_rect(Rect::new(g.x as i32 - 4, g.y as i32 - 4, 8, 8));
                
                // Trail de humo detrás de la granada
                backend.canvas.set_draw_color(Color::RGBA(150, 150, 150, 100));
                let _ = backend.canvas.fill_rect(Rect::new(g.x as i32 - 2, g.y as i32 - 8, 4, 4));
            } else {
                // Explosión visual grande
                let exp_progress = (-g.timer).min(1.0).max(0.0);
                let radius = (exp_progress * 60.0).max(1.0) as i32;
                let alpha = ((1.0 - exp_progress) * 200.0) as u8;
                let r = radius as u32;

                // Anillo de explosión
                backend.canvas.set_draw_color(Color::RGBA(255, 150, 50, alpha));
                let _ = backend.canvas.fill_rect(Rect::new(
                    g.x as i32 - radius,
                    g.y as i32 - radius,
                    r * 2,
                    r * 2,
                ));

                // Núcleo
                backend.canvas.set_draw_color(Color::RGBA(255, 255, 200, alpha));
                let core = (radius / 2).max(1) as u32;
                let _ = backend.canvas.fill_rect(Rect::new(
                    g.x as i32 - core as i32,
                    g.y as i32 - core as i32,
                    core * 2,
                    core * 2,
                ));
            }
        }

        // Dibujar partículas
        for p in &particle_sys.particles {
                let color = match p.color.as_str() {
                    "rojo" => Color::RGBA(255, 50, 50, (p.life * 255.0) as u8),
                    "naranja" => Color::RGBA(255, 150, 50, (p.life * 255.0) as u8),
                    "amarillo" => Color::RGBA(255, 255, 100, (p.life * 255.0) as u8),
                    "gris" => Color::RGBA(150, 150, 150, (p.life * 180.0) as u8),
                    "azul" => Color::RGBA(100, 150, 255, (p.life * 255.0) as u8),
                    _ => Color::RGBA(255, 255, 255, (p.life * 255.0) as u8),
                };
                backend.canvas.set_draw_color(color);
                let size = ((p.size * p.life).max(1.0)) as u32;
                let _ = backend.canvas.fill_rect(Rect::new(
                    p.x as i32 - size as i32 / 2,
                    p.y as i32 - size as i32 / 2,
                    size,
                    size,
                ));
        }

        // HUD
        if let Some(ref tex) = txt_hud {
            let q = tex.query();
            let w = q.width as u32;
            backend.canvas.set_draw_color(Color::RGBA(0, 0, 0, 180));
            let _ = backend.canvas.fill_rect(Rect::new(10, 10, w + 16, 24));
            backend.canvas.copy(tex, None, Rect::new(14, 12, w, 18)).ok();
        }

        if let Some(ref tex) = txt_municion {
            let q = tex.query();
            let w = q.width as u32;
            backend.canvas.set_draw_color(Color::RGBA(0, 0, 0, 180));
            let _ = backend.canvas.fill_rect(Rect::new(10, 44, w + 16, 24));
            backend.canvas.copy(tex, None, Rect::new(14, 46, w, 18)).ok();
        }

        // Instrucciones
        let instrucciones = crear_textura(&backend.font, "← → / A D: Mover | W/↑: Saltar | SPACE: Disparar | E: Granada | R: Recargar | ESC: Salir", 150, 150, 150, tc);
        if let Some(ref tex) = instrucciones {
            let q = tex.query();
            let w = q.width as u32;
            let _ = backend.canvas.copy(tex, None, Rect::new(10, 572, w.min(880), 18));
        }

        backend.canvas.present();

        // Cap 60 FPS
        let elapsed = frame_start.elapsed();
        if elapsed < std::time::Duration::from_millis(16) {
            std::thread::sleep(std::time::Duration::from_millis(16) - elapsed);
        }
    }

    println!("\n✅ Demo Militar cerrado | Score final: {}", score);
    Ok(())
}
