// crates/rydit-gfx/src/particles.rs
// Sistema de Partículas v0.5.3

use raylib::prelude::*;
use std::collections::HashMap;

// ============================================================================
// PARTICLE
// ============================================================================

/// Partícula individual
#[derive(Debug, Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,       // Velocidad X
    pub vy: f32,       // Velocidad Y
    pub life: f32,     // Vida actual (0.0 - 1.0)
    pub max_life: f32, // Vida máxima
    pub size: f32,
    pub color: Color,
    pub gravity: f32,
    pub friction: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32, color: Color, size: f32, life: f32) -> Self {
        Self {
            x,
            y,
            vx,
            vy,
            life,
            max_life: life,
            size,
            color,
            gravity: 0.0,
            friction: 1.0,
        }
    }

    /// Actualizar partícula
    pub fn update(&mut self, dt: f32, global_gravity: f32, wind_x: f32, wind_y: f32) {
        // Aplicar fuerzas
        self.vy += (self.gravity + global_gravity) * dt;
        self.vx += wind_x * dt;
        self.vy += wind_y * dt;

        // Aplicar fricción
        self.vx *= self.friction;
        self.vy *= self.friction;

        // Actualizar posición
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Reducir vida
        self.life -= dt;
    }

    /// Verificar si la partícula está viva
    pub fn is_alive(&self) -> bool {
        self.life > 0.0
    }

    /// Obtener alpha basado en vida (fade in/out)
    pub fn get_alpha(&self) -> u8 {
        let life_ratio = self.life / self.max_life;
        // Fade in al inicio, fade out al final
        if life_ratio > 0.8 {
            // Fade out
            ((1.0 - life_ratio) * 5.0 * 255.0) as u8
        } else if life_ratio < 0.2 {
            // Fade in
            (life_ratio * 5.0 * 255.0) as u8
        } else {
            self.color.a
        }
    }

    /// Dibujar partícula
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let alpha = self.get_alpha();
        let color_with_alpha = Color {
            r: self.color.r,
            g: self.color.g,
            b: self.color.b,
            a: alpha,
        };

        d.draw_circle(self.x as i32, self.y as i32, self.size, color_with_alpha);
    }
}

// ============================================================================
// PARTICLE EMITTER
// ============================================================================

/// Emisor de partículas
#[derive(Debug)]
pub struct ParticleEmitter {
    pub x: f32,
    pub y: f32,
    pub rate: f32,   // Partículas por segundo
    pub spread: f32, // Dispersión angular (grados)
    pub speed_min: f32,
    pub speed_max: f32,
    pub size_min: f32,
    pub size_max: f32,
    pub color_start: Color,
    pub color_end: Color,
    pub particles: Vec<Particle>,
    pub gravity: f32,
    pub friction: f32,
    pub wind_x: f32,
    pub wind_y: f32,
    pub emission_timer: f32,
    pub active: bool,
    pub one_shot: bool, // Emitir una vez y desactivar
    pub emitted: bool,  // Ya emitió (para one_shot)
}

impl ParticleEmitter {
    pub fn new(x: f32, y: f32, rate: f32) -> Self {
        Self {
            x,
            y,
            rate,
            spread: 360.0,
            speed_min: 50.0,
            speed_max: 150.0,
            size_min: 2.0,
            size_max: 5.0,
            color_start: Color::WHITE,
            color_end: Color::WHITE,
            particles: Vec::new(),
            gravity: 0.0,
            friction: 1.0,
            wind_x: 0.0,
            wind_y: 0.0,
            emission_timer: 0.0,
            active: true,
            one_shot: false,
            emitted: false,
        }
    }

    /// Emitir partícula
    pub fn emit(&mut self) {
        if !self.active || (self.one_shot && self.emitted) {
            return;
        }

        // Calcular ángulo aleatorio
        let angle_deg = if self.spread >= 360.0 {
            rand_f32() * 360.0
        } else {
            -self.spread / 2.0 + rand_f32() * self.spread
        };

        let angle_rad = angle_deg.to_radians();

        // Calcular velocidad
        let speed = self.speed_min + rand_f32() * (self.speed_max - self.speed_min);
        let vx = speed * angle_rad.cos();
        let vy = speed * angle_rad.sin();

        // Tamaño aleatorio
        let size = self.size_min + rand_f32() * (self.size_max - self.size_min);

        // Color interpolado
        let t = rand_f32();
        let color = Color {
            r: (self.color_start.r as f32
                + t * (self.color_end.r as f32 - self.color_start.r as f32)) as u8,
            g: (self.color_start.g as f32
                + t * (self.color_end.g as f32 - self.color_start.g as f32)) as u8,
            b: (self.color_start.b as f32
                + t * (self.color_end.b as f32 - self.color_start.b as f32)) as u8,
            a: (self.color_start.a as f32
                + t * (self.color_end.a as f32 - self.color_start.a as f32)) as u8,
        };

        // Vida aleatoria
        let life = 0.5 + rand_f32() * 1.5; // 0.5 - 2.0 segundos

        let mut particle = Particle::new(self.x, self.y, vx, vy, color, size, life);
        particle.gravity = self.gravity;
        particle.friction = self.friction;
        self.particles.push(particle);

        if self.one_shot {
            self.emitted = true;
        }
    }

    /// Actualizar emisor
    pub fn update(&mut self, dt: f32, global_gravity: f32, global_wind_x: f32, global_wind_y: f32) {
        if !self.active {
            return;
        }

        // Emitir nuevas partículas
        if !self.one_shot || !self.emitted {
            self.emission_timer += dt;
            let emit_interval = 1.0 / self.rate;

            while self.emission_timer >= emit_interval {
                self.emit();
                self.emission_timer -= emit_interval;
            }
        }

        // Actualizar partículas
        for particle in &mut self.particles {
            particle.update(
                dt,
                global_gravity + self.gravity,
                global_wind_x + self.wind_x,
                global_wind_y + self.wind_y,
            );
        }

        // Eliminar partículas muertas
        self.particles.retain(|p| p.is_alive());
    }

    /// Dibujar emisor
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for particle in &self.particles {
            particle.draw(d);
        }
    }

    /// Verificar si el emisor está vacío
    pub fn is_empty(&self) -> bool {
        self.particles.is_empty() && (!self.active || self.emitted)
    }
}

// ============================================================================
// PARTICLE SYSTEM
// ============================================================================

/// Sistema global de partículas
pub struct ParticleSystem {
    pub emitters: HashMap<String, ParticleEmitter>,
    pub global_gravity: f32,
    pub global_wind_x: f32,
    pub global_wind_y: f32,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            emitters: HashMap::new(),
            global_gravity: 200.0, // Gravedad hacia abajo
            global_wind_x: 0.0,
            global_wind_y: 0.0,
        }
    }

    /// Crear emisor
    pub fn create_emitter(&mut self, id: &str, x: f32, y: f32, rate: f32) {
        let emitter = ParticleEmitter::new(x, y, rate);
        self.emitters.insert(id.to_string(), emitter);
    }

    /// Obtener emisor mutable
    pub fn get_emitter_mut(&mut self, id: &str) -> Option<&mut ParticleEmitter> {
        self.emitters.get_mut(id)
    }

    /// Actualizar todos los emisores
    pub fn update(&mut self, dt: f32) {
        for emitter in self.emitters.values_mut() {
            emitter.update(
                dt,
                self.global_gravity,
                self.global_wind_x,
                self.global_wind_y,
            );
        }

        // Eliminar emisores vacíos
        self.emitters.retain(|_, e| !e.is_empty());
    }

    /// Dibujar todas las partículas
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for emitter in self.emitters.values() {
            emitter.draw(d);
        }
    }

    /// Configurar gravedad global
    pub fn set_gravity(&mut self, gravity: f32) {
        self.global_gravity = gravity;
    }

    /// Configurar viento global
    pub fn set_wind(&mut self, wind_x: f32, wind_y: f32) {
        self.global_wind_x = wind_x;
        self.global_wind_y = wind_y;
    }

    /// Eliminar emisor
    pub fn remove_emitter(&mut self, id: &str) {
        self.emitters.remove(id);
    }

    /// Eliminar todos los emisores
    pub fn clear(&mut self) {
        self.emitters.clear();
    }

    /// Contar partículas totales
    pub fn particle_count(&self) -> usize {
        self.emitters.values().map(|e| e.particles.len()).sum()
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UTILIDADES
// ============================================================================

/// Número aleatorio entre 0.0 y 1.0
fn rand_f32() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as f32;
    (seed.sin() * 10000.0).fract()
}

// ============================================================================
// PRESETS DE EFECTOS
// ============================================================================

impl ParticleEmitter {
    /// Efecto de explosión
    pub fn explosion(x: f32, y: f32) -> Self {
        let mut emitter = Self::new(x, y, 500.0); // Alta tasa inicial
        emitter.spread = 360.0;
        emitter.speed_min = 100.0;
        emitter.speed_max = 300.0;
        emitter.size_min = 3.0;
        emitter.size_max = 8.0;
        emitter.color_start = Color::new(255, 200, 50, 255); // Amarillo
        emitter.color_end = Color::new(255, 50, 0, 255); // Rojo
        emitter.gravity = -50.0; // Anti-gravedad inicial
        emitter.one_shot = true; // Una sola explosión
        emitter
    }

    /// Efecto de fuego
    pub fn fire(x: f32, y: f32) -> Self {
        let mut emitter = Self::new(x, y, 30.0);
        emitter.spread = 30.0; // Hacia arriba
        emitter.speed_min = 50.0;
        emitter.speed_max = 100.0;
        emitter.size_min = 5.0;
        emitter.size_max = 15.0;
        emitter.color_start = Color::new(255, 200, 50, 200); // Amarillo
        emitter.color_end = Color::new(255, 50, 0, 50); // Rojo transparente
        emitter.gravity = -30.0; // Hacia arriba
        emitter
    }

    /// Efecto de humo
    pub fn smoke(x: f32, y: f32) -> Self {
        let mut emitter = Self::new(x, y, 10.0);
        emitter.spread = 45.0;
        emitter.speed_min = 20.0;
        emitter.speed_max = 50.0;
        emitter.size_min = 10.0;
        emitter.size_max = 30.0;
        emitter.color_start = Color::new(100, 100, 100, 150); // Gris
        emitter.color_end = Color::new(50, 50, 50, 50);
        emitter.gravity = -20.0; // Hacia arriba lento
        emitter
    }

    /// Efecto de lluvia
    pub fn rain(x: f32, y: f32, _width: f32) -> Self {
        let mut emitter = Self::new(x, y, 100.0);
        emitter.spread = 5.0; // Casi recto
        emitter.speed_min = 200.0;
        emitter.speed_max = 400.0;
        emitter.size_min = 2.0;
        emitter.size_max = 3.0;
        emitter.color_start = Color::new(100, 150, 255, 200); // Azul claro
        emitter.color_end = Color::new(50, 100, 200, 150);
        emitter.gravity = 50.0; // Hacia abajo
        emitter.wind_x = 20.0; // Viento lateral
        emitter
    }

    /// Efecto de chispas
    pub fn sparks(x: f32, y: f32) -> Self {
        let mut emitter = Self::new(x, y, 50.0);
        emitter.spread = 180.0;
        emitter.speed_min = 100.0;
        emitter.speed_max = 250.0;
        emitter.size_min = 2.0;
        emitter.size_max = 4.0;
        emitter.color_start = Color::new(255, 255, 100, 255); // Amarillo brillante
        emitter.color_end = Color::new(255, 100, 0, 255);
        emitter.gravity = 100.0; // Caen rápido
        emitter
    }
}
