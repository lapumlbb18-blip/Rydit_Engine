//! # ryArt - El Motor de Expresión de Ry-Dit
//!
//! "IA sin IA": Arte generativo impulsado por simulación, física y matemática.
//! Este crate permite crear texturas, patrones y diseños finos de forma procedimental.

use ry_gfx::ColorRydit;
use serde::{Deserialize, Serialize};

/// Representa una superficie de dibujo generativa en memoria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtCanvas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<ColorRydit>,
}

impl ArtCanvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![ColorRydit::Negro; (width * height) as usize],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: ColorRydit) {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.pixels[idx] = color;
        }
    }
}

/// El Trait Maestro para cualquier generador de arte en Ry-Dit
pub trait ArtGenerator {
    /// Generar o actualizar el arte basado en un estado o tiempo
    fn generate(&mut self, canvas: &mut ArtCanvas, t: f64);
    
    /// Nombre del estilo o patrón
    fn name(&self) -> &'static str;
}

// ============================================================================
// PRIMITIVOS ARTÍSTICOS (Ejemplos iniciales)
// ============================================================================

/// Generador de ruido basado en ry-science para texturas orgánicas
pub struct OrganicNoise {
    pub scale: f64,
    pub seed: u64,
}

impl ArtGenerator for OrganicNoise {
    fn generate(&mut self, canvas: &mut ArtCanvas, t: f64) {
        // Aquí conectaremos con ry_science::math para ruido Perlin/Simplex
        for y in 0..canvas.height {
            for x in 0..canvas.width {
                // Simulación de valor "orgánico"
                let val = ((x as f64 * self.scale + t).sin() * (y as f64 * self.scale + t).cos()).abs();
                let color = if val > 0.5 { ColorRydit::Blanco } else { ColorRydit::Negro };
                canvas.set_pixel(x, y, color);
            }
        }
    }

    fn name(&self) -> &'static str { "OrganicNoise" }
}

/// Trait para brochas que pueden tener comportamiento propio
pub trait Brush {
    /// Dibujar un trazo en el canvas
    fn stroke(&mut self, canvas: &mut ArtCanvas, x: f32, y: f32, pressure: f32);
    
    /// Actualizar estado interno de la brocha (ej: física)
    fn update(&mut self, dt: f32);
}

// ============================================================================
// IMPLEMENTACIONES DE BROCHAS
// ============================================================================

/// Una brocha que tiene inercia y deja un rastro basado en su velocidad
pub struct PhysicsBrush {
    pub pos_x: f32,
    pub pos_y: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub friction: f32,
    pub color: ColorRydit,
    pub size: f32,
}

impl PhysicsBrush {
    pub fn new(color: ColorRydit, size: f32) -> Self {
        Self {
            pos_x: 0.0, pos_y: 0.0,
            vel_x: 0.0, vel_y: 0.0,
            friction: 0.95,
            color,
            size,
        }
    }
}

impl Brush for PhysicsBrush {
    fn stroke(&mut self, canvas: &mut ArtCanvas, target_x: f32, target_y: f32, pressure: f32) {
        // La brocha intenta seguir al cursor con "fuerza"
        let dx = target_x - self.pos_x;
        let dy = target_y - self.pos_y;
        
        self.vel_x += dx * 0.1;
        self.vel_y += dy * 0.1;
        
        // Dibujar en el canvas basado en la posición física actual
        let r = (self.size * pressure) as u32;
        for i in -(r as i32)..=(r as i32) {
            for j in -(r as i32)..=(r as i32) {
                if (i*i + j*j) <= (r*r) as i32 {
                    canvas.set_pixel(
                        (self.pos_x + i as f32) as u32,
                        (self.pos_y + j as f32) as u32,
                        self.color
                    );
                }
            }
        }
    }

    fn update(&mut self, _dt: f32) {
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;
        self.vel_x *= self.friction;
        self.vel_y *= self.friction;
    }
}

/// Brocha Orgánica: Usa ruido de Perlin (o similar) para variar el trazo
pub struct OrganicBrush {
    pub scale: f32,
    pub color: ColorRydit,
}

impl Brush for OrganicBrush {
    fn stroke(&mut self, canvas: &mut ArtCanvas, x: f32, y: f32, pressure: f32) {
        // Aquí se usaría ry_science::math::noise para modular el tamaño
        let noise_val = (x * self.scale).sin() * (y * self.scale).cos(); // Placeholder
        let dynamic_size = (10.0 * pressure * (1.0 + noise_val)) as u32;
        
        for i in -(dynamic_size as i32)..=(dynamic_size as i32) {
            for j in -(dynamic_size as i32)..=(dynamic_size as i32) {
                if (i*i + j*j) <= (dynamic_size*dynamic_size) as i32 {
                    canvas.set_pixel((x + i as f32) as u32, (y + j as f32) as u32, self.color);
                }
            }
        }
    }
    
    fn update(&mut self, _dt: f32) {}
}
