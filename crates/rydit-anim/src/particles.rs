// crates/rydit-anim/src/particles.rs
// Particle System - Sistema de partículas para efectos visuales

use std::cell::RefCell;
use std::rc::Rc;

// ============================================================================
// ESTRUCTURAS DE PARTÍCULAS
// ============================================================================

/// Partícula individual
#[derive(Debug, Clone)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub life: f64,
    pub size: f64,
    pub color: String,
}

impl Particle {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64, life: f64, size: f64, color: &str) -> Self {
        Self {
            x,
            y,
            vx,
            vy,
            life,
            size,
            color: color.to_string(),
        }
    }

    pub fn update(&mut self, gravity: f64, friction: f64) {
        self.vy += gravity;
        self.vx *= friction;
        self.vy *= friction;
        self.x += self.vx;
        self.y += self.vy;
        self.life -= 0.02;
    }

    pub fn is_alive(&self) -> bool {
        self.life > 0.0
    }
}

/// Sistema de partículas
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub gravity: f64,
    pub friction: f64,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            gravity: 0.1,
            friction: 0.98,
        }
    }

    pub fn emit(&mut self, x: f64, y: f64, effect: &str, count: usize) {
        match effect {
            "fire" => self.emit_fire(x, y, count),
            "smoke" => self.emit_smoke(x, y, count),
            "spark" => self.emit_spark(x, y, count),
            "explosion" => self.emit_explosion(x, y, count),
            "rain" => self.emit_rain(x, y, count),
            _ => self.emit_default(x, y, count),
        }
    }

    pub fn update(&mut self) {
        for particle in &mut self.particles {
            particle.update(self.gravity, self.friction);
        }
        self.particles.retain(|p| p.is_alive());
    }

    pub fn count(&self) -> usize {
        self.particles.len()
    }

    pub fn clear(&mut self) {
        self.particles.clear();
    }

    fn emit_fire(&mut self, x: f64, y: f64, count: usize) {
        for _ in 0..count {
            let vx = (rand_f64() - 0.5) * 2.0;
            let vy = -(rand_f64() * 3.0 + 2.0);
            let life = rand_f64() * 0.5 + 0.5;
            let size = rand_f64() * 4.0 + 2.0;
            let color = if rand_f64() > 0.5 { "naranja" } else { "amarillo" };
            self.particles.push(Particle::new(x, y, vx, vy, life, size, color));
        }
    }

    fn emit_smoke(&mut self, x: f64, y: f64, count: usize) {
        for _ in 0..count {
            let vx = (rand_f64() - 0.5) * 1.0;
            let vy = -(rand_f64() * 1.5 + 0.5);
            let life = rand_f64() * 0.8 + 0.2;
            let size = rand_f64() * 6.0 + 4.0;
            self.particles.push(Particle::new(x, y, vx, vy, life, size, "gris"));
        }
    }

    fn emit_spark(&mut self, x: f64, y: f64, count: usize) {
        for _ in 0..count {
            let angle = rand_f64() * std::f64::consts::PI * 2.0;
            let speed = rand_f64() * 5.0 + 3.0;
            let vx = angle.cos() * speed;
            let vy = angle.sin() * speed;
            let life = rand_f64() * 0.3 + 0.2;
            let size = rand_f64() * 2.0 + 1.0;
            self.particles.push(Particle::new(x, y, vx, vy, life, size, "amarillo"));
        }
    }

    fn emit_explosion(&mut self, x: f64, y: f64, count: usize) {
        for _ in 0..count {
            let angle = rand_f64() * std::f64::consts::PI * 2.0;
            let speed = rand_f64() * 6.0 + 4.0;
            let vx = angle.cos() * speed;
            let vy = angle.sin() * speed;
            let life = rand_f64() * 0.4 + 0.2;
            let size = rand_f64() * 3.0 + 2.0;
            let color = if rand_f64() > 0.5 { "naranja" } else { "rojo" };
            self.particles.push(Particle::new(x, y, vx, vy, life, size, color));
        }
    }

    fn emit_rain(&mut self, x: f64, y: f64, count: usize) {
        for _ in 0..count {
            let vx = (rand_f64() - 0.5) * 0.5;
            let vy = rand_f64() * 3.0 + 5.0;
            let life = 1.0;
            let size = rand_f64() * 8.0 + 4.0;
            self.particles.push(Particle::new(x, y, vx, vy, life, size, "azul"));
        }
    }

    fn emit_default(&mut self, x: f64, y: f64, count: usize) {
        for _ in 0..count {
            let vx = (rand_f64() - 0.5) * 2.0;
            let vy = (rand_f64() - 0.5) * 2.0;
            let life = rand_f64() * 0.5 + 0.5;
            let size = rand_f64() * 4.0 + 2.0;
            self.particles.push(Particle::new(x, y, vx, vy, life, size, "blanco"));
        }
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new()
    }
}

fn rand_f64() -> f64 {
    static mut SEED: u32 = 12345;
    unsafe {
        SEED = SEED.wrapping_mul(1664525).wrapping_add(1013904223);
        (SEED as f64) / (u32::MAX as f64)
    }
}

// ============================================================================
// ESTADO GLOBAL
// ============================================================================

thread_local! {
    static PARTICLE_SYSTEM: Rc<RefCell<ParticleSystem>> = Rc::new(RefCell::new(ParticleSystem::new()));
}

pub fn get_particle_system() -> Rc<RefCell<ParticleSystem>> {
    PARTICLE_SYSTEM.with(|ps| ps.clone())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_new() {
        let p = Particle::new(100.0, 200.0, 1.0, -2.0, 1.0, 5.0, "rojo");
        assert_eq!(p.x, 100.0);
        assert_eq!(p.y, 200.0);
    }

    #[test]
    fn test_particle_system() {
        let mut ps = ParticleSystem::new();
        ps.emit(400.0, 300.0, "fire", 20);
        assert!(ps.count() > 0);
        ps.update();
        ps.clear();
        assert_eq!(ps.count(), 0);
    }
}
