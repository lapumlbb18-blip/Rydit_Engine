// crates/rydit-ecs/src/components.rs
// Componentes ECS para RyDit Engine
// Inspirado en bevy_ecs, integrado con N-Body gravity

use bevy_ecs::prelude::*;

// ============================================================================
// COMPONENTES BÁSICOS
// ============================================================================

/// Posición 2D
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Velocidad 2D
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

impl Velocity {
    pub fn new(vx: f32, vy: f32) -> Self {
        Self { vx, vy }
    }
}

/// Sprite para renderizado
#[derive(Component, Clone, Debug)]
pub struct Sprite {
    pub texture_id: String,
    pub width: f32,
    pub height: f32,
    pub color: (f32, f32, f32, f32), // RGBA (0.0-1.0)
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Sprite {
    pub fn new(texture_id: &str, width: f32, height: f32) -> Self {
        Self {
            texture_id: texture_id.to_string(),
            width,
            height,
            color: (1.0, 1.0, 1.0, 1.0),
            flip_x: false,
            flip_y: false,
        }
    }

    pub fn with_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.color = (r, g, b, a);
        self
    }
}

/// Partícula para sistemas de partículas
#[derive(Component, Clone, Copy, Debug)]
pub struct Particle {
    pub lifetime: f32,      // Tiempo de vida restante
    pub max_lifetime: f32,  // Tiempo de vida máximo
    pub size: f32,
    pub rotation: f32,
}

impl Particle {
    pub fn new(lifetime: f32, size: f32) -> Self {
        Self {
            lifetime,
            max_lifetime: lifetime,
            size,
            rotation: 0.0,
        }
    }
}

/// Cuerpo físico para N-Body gravity
#[derive(Component, Clone, Copy, Debug)]
pub struct Body {
    pub mass: f32,
    pub is_static: bool,    // Si es true, no se mueve por gravedad
}

impl Body {
    pub fn new(mass: f32) -> Self {
        Self {
            mass,
            is_static: false,
        }
    }

    pub fn static_body(mass: f32) -> Self {
        Self {
            mass,
            is_static: true,
        }
    }
}

/// Collider para detección de colisiones
#[derive(Component, Clone, Copy, Debug)]
pub struct Collider {
    pub width: f32,
    pub height: f32,
    pub is_trigger: bool,   // Si es true, no hay respuesta física
}

impl Collider {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            is_trigger: false,
        }
    }
}

// ============================================================================
// MARCADORES (Marker Components)
// ============================================================================

/// Marcador para entidades de jugador
#[derive(Component)]
pub struct Player;

/// Marcador para entidades de enemigo
#[derive(Component)]
pub struct Enemy;

/// Marcador para entidades que deben ser eliminadas
#[derive(Component)]
pub struct DespawnMarker;

// ============================================================================
// RECURSOS GLOBALES
// ============================================================================

/// Gravedad global (para N-Body)
#[derive(Resource, Clone, Copy, Debug)]
pub struct Gravity {
    pub x: f32,
    pub y: f32,
    pub g_constant: f32,  // Constante gravitacional
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 9.8,  // Gravedad terrestre por defecto
            g_constant: 6.674e-11,
        }
    }
}

/// Delta time para actualización de sistemas
#[derive(Resource, Clone, Copy, Debug)]
pub struct DeltaTime(pub f32);

/// Configuración de renderizado
#[derive(Resource, Clone, Debug)]
pub struct RenderConfig {
    pub camera_x: f32,
    pub camera_y: f32,
    pub camera_zoom: f32,
    pub screen_width: f32,
    pub screen_height: f32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            screen_width: 800.0,
            screen_height: 600.0,
        }
    }
}
