//! # SDL2 Helpers para Ry-Dit
//!
//! Puente entre las features de ry-gfx (color por velocidad, blend aditivo, partículas)
//! y el backend SDL2. Cualquier demo SDL2 puede importar estos helpers.
//!
//! ## Uso
//!
//! ```rust,ignore
//! use ry_gfx::sdl2_helpers::*;
//!
//! // Color por velocidad
//! let c = velocity_color_sdl2(speed, 300.0);
//! canvas.set_draw_color(c);
//!
//! // Blend aditivo para explosiones
//! set_blend_additive(&mut canvas);
//! draw_particles_sdl2(&mut canvas, &ps, max_speed);
//! set_blend_normal(&mut canvas);
//! ```

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

// ============================================================================
// COLOR POR VELOCIDAD
// ============================================================================

/// 🔵 Azul oscuro → 🔵 Azul → 🟡 Amarillo → 🟠 Naranja → 🔴 Rojo → ⚪ Blanco
pub fn velocity_color_sdl2(speed: f32, max_speed: f32) -> Color {
    let t = (speed / max_speed).clamp(0.0, 1.0);
    let (r, g, b) = if t < 0.2 {
        lerp3((20, 40, 120), (80, 160, 255), t / 0.2)
    } else if t < 0.4 {
        lerp3((80, 160, 255), (255, 255, 80), (t - 0.2) / 0.2)
    } else if t < 0.6 {
        lerp3((255, 255, 80), (255, 160, 20), (t - 0.4) / 0.2)
    } else if t < 0.8 {
        lerp3((255, 160, 20), (255, 40, 20), (t - 0.6) / 0.2)
    } else if t < 0.95 {
        lerp3((255, 40, 20), (255, 100, 80), (t - 0.8) / 0.15)
    } else {
        lerp3((255, 100, 80), (255, 255, 255), (t - 0.95) / 0.05)
    };
    Color::RGB(r, g, b)
}

fn lerp3(from: (u8, u8, u8), to: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    (
        (from.0 as f32 + (to.0 as f32 - from.0 as f32) * t) as u8,
        (from.1 as f32 + (to.1 as f32 - from.1 as f32) * t) as u8,
        (from.2 as f32 + (to.2 as f32 - from.2 as f32) * t) as u8,
    )
}

// ============================================================================
// BLEND ADITIVO (SDL2)
// ============================================================================

/// Activar blend aditivo — los colores se SUMAN al superponerse
pub fn set_blend_additive(canvas: &mut Canvas<Window>) {
    canvas.set_blend_mode(sdl2::render::BlendMode::Add);
}

/// Desactivar blend aditivo — volver a blend normal (alpha)
pub fn set_blend_normal(canvas: &mut Canvas<Window>) {
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
}

// ============================================================================
// DIBUJO DE PARTÍCULAS (SDL2)
// ============================================================================

/// Dibujar una partícula individual con color por velocidad
pub fn draw_particle_sdl2(
    canvas: &mut Canvas<Window>,
    x: f32, y: f32, radius: f32,
    color: Color,
) {
    canvas.set_draw_color(color);
    // Aproximación de círculo con puntos (suficiente para partículas pequeñas)
    let cx = x as i32;
    let cy = y as i32;
    let r = radius as i32;

    if r <= 2 {
        // Radio pequeño = punto simple
        let _ = canvas.draw_point(Point::new(cx, cy));
    } else if r <= 4 {
        // Radio mediano = cruz
        let _ = canvas.draw_point(Point::new(cx, cy));
        let _ = canvas.draw_point(Point::new(cx - 1, cy));
        let _ = canvas.draw_point(Point::new(cx + 1, cy));
        let _ = canvas.draw_point(Point::new(cx, cy - 1));
        let _ = canvas.draw_point(Point::new(cx, cy + 1));
    } else {
        // Radio grande = rectángulo relleno (rápido)
        let _ = canvas.fill_rect(sdl2::rect::Rect::new(cx - r, cy - r, (r * 2) as u32, (r * 2) as u32));
    }
}

/// Dibujar sistema de partículas con color por velocidad
pub fn draw_particles_sdl2(
    canvas: &mut Canvas<Window>,
    ps: &crate::gpu_particles::ParticleSystem,
    max_speed: f32,
) {
    for emitter in ps.emitters.values() {
        for particle in &emitter.particles {
            if !particle.is_alive() { continue; }
            let speed = (particle.vx * particle.vx + particle.vy * particle.vy).sqrt();
            let color = if max_speed > 0.0 {
                velocity_color_sdl2(speed, max_speed)
            } else {
                let c = particle.color;
                Color::RGB(c.r, c.g, c.b)
            };
            let alpha = particle.get_alpha();
            canvas.set_draw_color(Color::RGBA(color.r, color.g, color.b, alpha));

            let cx = particle.x as i32;
            let cy = particle.y as i32;
            let r = particle.size as i32;

            if r <= 1 {
                let _ = canvas.draw_point(Point::new(cx, cy));
            } else {
                let _ = canvas.fill_rect(sdl2::rect::Rect::new(cx - r, cy - r, (r * 2) as u32, (r * 2) as u32));
            }
        }
    }
}

// ============================================================================
// GRAVITACIÓN NEWTONIANA (reutilizable)
// ============================================================================

/// Aplicar gravitación F = G·m₁·m₂/r² entre cuerpos 2D genéricos
///
/// # Params
/// - `positions`: slice de (x, y)
/// - `velocities`: slice mutable de (vx, vy)
/// - `masses`: slice de masas
/// - `g`: constante gravitacional (escala del juego, ~50-200)
/// - `dt`: delta time
pub fn apply_newtonian_gravity_2d(
    positions: &[(f32, f32)],
    velocities: &mut [(f32, f32)],
    masses: &[f32],
    g: f64,
    dt: f32,
) {
    let n = positions.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = positions[j].0 - positions[i].0;
            let dy = positions[j].1 - positions[i].1;
            let d2 = dx * dx + dy * dy;
            let d = d2.sqrt();
            if d < 5.0 { continue; }

            let force = (g * masses[i] as f64 * masses[j] as f64 / d2 as f64) as f32;
            let ax = force * dx / (d * masses[i]);
            let ay = force * dy / (d * masses[i]);

            velocities[i].0 += ax * dt;
            velocities[i].1 += ay * dt;
            velocities[j].0 -= ax * dt;
            velocities[j].1 -= ay * dt;
        }
    }
}

// ============================================================================
// AUDIO PROCEDURAL (SDL2)
// ============================================================================

/// Generar datos de onda WAV en memoria (8-bit unsigned)
/// Retorna Vec<u8> que se puede pasar a SDL2_mixer::RWops
pub fn generate_wave_data(duration: f32, sample_rate: u32, freq: f32, wave_type: &str) -> Vec<u8> {
    let samples = (sample_rate as f32 * duration) as usize;
    let mut data = Vec::with_capacity(samples);
    let sr = sample_rate as f32;

    for i in 0..samples {
        let t = i as f32 / sr;
        let sample = match wave_type {
            "shoot" => {
                let env = (-t * 30.0).exp();
                (freq * t * 2.0 * std::f32::consts::PI).sin() * env * 0.6
            }
            "explosion" => {
                let noise = (rand_f32() * 2.0 - 1.0) * 0.4;
                let env = if t < 0.02 { t / 0.02 } else { (-(t - 0.02) * 4.0).exp() };
                noise * env * 0.8
            }
            "powerup" => {
                let sweep_f = freq + t * 2000.0;
                (sweep_f * t * 2.0 * std::f32::consts::PI).sin() * 0.5
            }
            _ => 0.0,
        };
        data.push(((sample + 1.0) * 127.5) as u8);
    }
    data
}

fn rand_f32() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let s = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as f32;
    (s.sin() * 10000.0).fract()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocity_color_sdl2() {
        let c0 = velocity_color_sdl2(0.0, 300.0);
        let (r0, g0, b0) = c0.rgb();
        assert!(b0 > r0); // Azul a baja velocidad

        let c_max = velocity_color_sdl2(300.0, 300.0);
        let (r_max, _, _) = c_max.rgb();
        assert!(r_max >= 240); // Blanco/rojo a alta velocidad
    }

    #[test]
    fn test_generate_wave_data() {
        let data = generate_wave_data(0.1, 22050, 800.0, "shoot");
        assert!(!data.is_empty());
        assert_eq!(data.len(), 2205); // 0.1s * 22050
    }

    #[test]
    fn test_newtonian_gravity() {
        let positions = [(0.0, 0.0), (100.0, 0.0)];
        let mut velocities = [(0.0, 0.0), (0.0, 0.0)];
        let masses = [10.0, 20.0];

        apply_newtonian_gravity_2d(&positions, &mut velocities, &masses, 50.0, 0.016);

        // Deberían atraerse
        assert!(velocities[0].0 > 0.0); // cuerpo 1 se mueve hacia la derecha
        assert!(velocities[1].0 < 0.0); // cuerpo 2 se mueve hacia la izquierda
    }
}
