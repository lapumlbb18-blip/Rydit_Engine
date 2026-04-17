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

    /// 🆕 v0.19.2: Color por velocidad — Azul oscuro → Azul → Amarillo → Rojo → Blanco
    ///
    /// Escala de velocidad (pixels/seg):
    /// - 0-20%: Azul oscuro (lento/pesado)
    /// - 20-40%: Azul claro (acelerando)
    /// - 40-60%: Amarillo (eléctrico)
    /// - 60-80%: Naranja (rápido)
    /// - 80-95%: Rojo (nitrógeno NFS)
    /// - 95-100%: Blanco flash (velocidad de la luz)
    pub fn get_velocity_color(&self, max_speed: f32) -> Color {
        let speed = (self.vx * self.vx + self.vy * self.vy).sqrt();
        let t = (speed / max_speed).clamp(0.0, 1.0);

        let (r, g, b) = if t < 0.2 {
            // Azul oscuro → Azul claro
            let lt = t / 0.2;
            lerp3((20, 40, 120), (80, 160, 255), lt)
        } else if t < 0.4 {
            // Azul claro → Amarillo
            let lt = (t - 0.2) / 0.2;
            lerp3((80, 160, 255), (255, 255, 80), lt)
        } else if t < 0.6 {
            // Amarillo → Naranja
            let lt = (t - 0.4) / 0.2;
            lerp3((255, 255, 80), (255, 160, 20), lt)
        } else if t < 0.8 {
            // Naranja → Rojo
            let lt = (t - 0.6) / 0.2;
            lerp3((255, 160, 20), (255, 40, 20), lt)
        } else if t < 0.95 {
            // Rojo → Rojo brillante (nitrógeno)
            let lt = (t - 0.8) / 0.15;
            lerp3((255, 40, 20), (255, 100, 80), lt)
        } else {
            // Rojo brillante → Blanco flash
            let lt = (t - 0.95) / 0.05;
            lerp3((255, 100, 80), (255, 255, 255), lt)
        };

        Color {
            r: r as u8,
            g: g as u8,
            b: b as u8,
            a: self.get_alpha(),
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

    /// 🆕 v0.19.2: Dibujar con color por velocidad
    pub fn draw_with_velocity(&self, d: &mut RaylibDrawHandle, max_speed: f32) {
        let vel_color = self.get_velocity_color(max_speed);
        d.draw_circle(self.x as i32, self.y as i32, self.size, vel_color);
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
    /// Turbulencia térmica (fuego convección, humo)
    pub turbulence: f32,
    /// Alias para rate (compatibilidad)
    pub particles_per_second: f32,
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
            turbulence: 0.0,
            particles_per_second: 0.0,
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
        // Añadir turbulencia térmica si está configurada
        if self.turbulence > 0.0 {
            particle.vx += (rand_f32() - 0.5) * self.turbulence;
            particle.vy += (rand_f32() - 0.5) * self.turbulence * 0.5;
        }
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

    /// 🆕 v0.19.2: Dibujar todas las partículas con color por velocidad
    pub fn draw_with_velocity(&self, d: &mut RaylibDrawHandle, max_speed: f32) {
        for particle in &self.particles {
            particle.draw_with_velocity(d, max_speed);
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
    /// 🆕 v0.19.2: Blend aditivo para explosiones brillantes
    pub additive_blend: bool,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            emitters: HashMap::new(),
            global_gravity: 200.0, // Gravedad hacia abajo
            global_wind_x: 0.0,
            global_wind_y: 0.0,
            additive_blend: false,
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

    /// Dibujar todas las artículos
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.additive_blend {
            unsafe { raylib::ffi::BeginBlendMode(raylib::ffi::BlendMode::BLEND_ADDITIVE as i32) };
        }
        for emitter in self.emitters.values() {
            emitter.draw(d);
        }
        if self.additive_blend {
            unsafe { raylib::ffi::EndBlendMode() };
        }
    }

    /// 🆕 v0.19.2: Dibujar todas las partículas con color por velocidad
    pub fn draw_with_velocity(&self, d: &mut RaylibDrawHandle, max_speed: f32) {
        if self.additive_blend {
            unsafe { raylib::ffi::BeginBlendMode(raylib::ffi::BlendMode::BLEND_ADDITIVE as i32) };
        }
        for emitter in self.emitters.values() {
            emitter.draw_with_velocity(d, max_speed);
        }
        if self.additive_blend {
            unsafe { raylib::ffi::EndBlendMode() };
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

/// 🆕 v0.19.2: Interpolación lineal 3D (r, g, b)
fn lerp3(from: (u8, u8, u8), to: (u8, u8, u8), t: f32) -> (u16, u16, u16) {
    let t = t.clamp(0.0, 1.0);
    (
        (from.0 as f32 + (to.0 as f32 - from.0 as f32) * t) as u16,
        (from.1 as f32 + (to.1 as f32 - from.1 as f32) * t) as u16,
        (from.2 as f32 + (to.2 as f32 - from.2 as f32) * t) as u16,
    )
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

    /// Efecto de fuego (fuego básico, llama baja)
    ///
    /// Wrapper de `fire_convection` con intensidad 0.35 para fuego simple.
    /// Para fuego avanzado con turbulencia térmica, usar `fire_convection` directamente.
    pub fn fire(x: f32, y: f32) -> Self {
        Self::fire_convection(x, y, 0.35)
    }

    /// Efecto de humo (sin fuego)
    ///
    /// Humo gris que sube lentamente — independiente del fuego.
    /// Para humo como fase final del fuego, usar `fire_convection`
    /// que incluye el degradado → humo oscuro automáticamente.
    ///
    /// # Usos típicos
    /// - Estela de granada/explosión (demo_militar)
    /// - Chimenea, niebla, vapor
    /// - Combina con `fire_convection` para humo secundario separado
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

    // ========================================================================
    // FUEGO CON CONVECCIÓN — Hoguera, antorcha, incendio
    // ========================================================================

    /// Fuego con convección (partículas suben + turbulencia térmica)
    ///
    /// Las partículas calientes suben (convección) con turbulencia
    /// que simula viento térmico. Colores: blanco→amarillo→naranja→rojo→negro (humo).
    ///
    /// El humo es la **fase final** del enfriamiento del fuego — no se duplica
    /// con `smoke()`. Para humo sin fuego (chimenea, niebla), usar `smoke()`.
    ///
    /// # Parámetros
    /// - `intensity`: 0.0 = brasa suave, 0.35 = fuego bajo, 0.8 = antorcha, 1.2+ = incendio
    pub fn fire_convection(x: f32, y: f32, intensity: f32) -> Self {
        let intensity = intensity.max(0.1); // Mínimo brasa
        let mut emitter = Self::new(x, y, 80.0 * intensity);
        emitter.spread = 30.0; // Cono estrecho
        emitter.speed_min = 80.0 * intensity;
        emitter.speed_max = 200.0 * intensity;
        emitter.size_min = 8.0;
        emitter.size_max = 20.0;
        emitter.color_start = Color::new(255, 255, 200, 255); // Blanco caliente
        emitter.color_end = Color::new(40, 20, 10, 150); // Humo oscuro (fase final)
        emitter.gravity = -150.0; // Convección: suben (negativo = arriba)
        emitter.wind_x = 0.0; // Sin viento lateral base
        emitter.turbulence = 50.0 * intensity; // Turbulencia térmica
        emitter
    }

    /// Antorcha (fuego vertical con humo)
    pub fn torch(x: f32, y: f32) -> Self {
        Self::fire_convection(x, y, 0.8)
    }

    /// Hoguera grande (fuego intenso con mucho humo)
    pub fn bonfire(x: f32, y: f32) -> Self {
        let mut emitter = Self::fire_convection(x, y, 1.2);
        emitter.spread = 50.0;
        emitter.rate = 150.0;
        emitter
    }

    /// Incendio (fuego grande con viento y turbulencia alta)
    pub fn wildfire(x: f32, y: f32, wind_strength: f32) -> Self {
        let mut emitter = Self::bonfire(x, y);
        emitter.wind_x = wind_strength;
        emitter.turbulence = 100.0;
        emitter.rate = 200.0;
        emitter
    }
}
