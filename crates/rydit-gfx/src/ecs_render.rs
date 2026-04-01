// crates/rydit-gfx/src/ecs_render.rs
// ECS Render - Integración de ECS con rydit-gfx usando rlgl
// v0.10.0: ECS + rlgl para renderizado eficiente

use rydit_ecs::EcsWorld;
use raylib::ffi::*;

// Constantes rlgl
const RL_QUADS: i32 = 3;
const RL_LINES: i32 = 2;

// ============================================================================
// ECS RENDERER
// ============================================================================

/// Renderizador ECS que usa rlgl para dibujar entidades
pub struct EcsRenderer {
    camera_x: f32,
    camera_y: f32,
    camera_zoom: f32,
}

impl EcsRenderer {
    pub fn new() -> Self {
        Self {
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
        }
    }

    /// Configurar cámara
    pub fn set_camera(&mut self, x: f32, y: f32, zoom: f32) {
        self.camera_x = x;
        self.camera_y = y;
        self.camera_zoom = zoom;
    }

    /// Renderizar todas las entidades del ECS world
    pub fn render(&self, world: &EcsWorld) {
        // Obtener datos de renderizado del ECS
        let render_data = world.get_render_data();

        // Renderizar cada entidad
        for (x, y, _texture_id, w, h) in render_data {
            // Aplicar cámara
            let screen_x = (x - self.camera_x) * self.camera_zoom;
            let screen_y = (y - self.camera_y) * self.camera_zoom;
            let screen_w = w * self.camera_zoom;
            let screen_h = h * self.camera_zoom;

            // Dibujar con rlgl (rectángulo de color por ahora)
            // En el futuro, usar texturas reales
            unsafe {
                rlPushMatrix();
                rlTranslatef(screen_x, screen_y, 0.0);
                
                // Dibujar rectángulo
                rlBegin(RL_QUADS);
                rlColor4ub(255, 0, 0, 255); // Rojo por defecto
                rlVertex2f(0.0, 0.0);
                rlVertex2f(screen_w, 0.0);
                rlVertex2f(screen_w, screen_h);
                rlVertex2f(0.0, screen_h);
                rlEnd();
                
                rlPopMatrix();
            }
        }
    }

    /// Renderizar entidades con colores por tipo
    pub fn render_colored(&self, world: &EcsWorld) {
        let render_data = world.get_render_data();

        for (x, y, texture_id, w, h) in render_data {
            let screen_x = (x - self.camera_x) * self.camera_zoom;
            let screen_y = (y - self.camera_y) * self.camera_zoom;
            let screen_w = w * self.camera_zoom;
            let screen_h = h * self.camera_zoom;

            // Color según tipo de entidad
            let (r, g, b) = if texture_id == "player" {
                (0, 255, 0) // Verde para jugador
            } else if texture_id == "enemy" {
                (255, 0, 0) // Rojo para enemigo
            } else if texture_id.is_empty() {
                (255, 255, 0) // Amarillo para partículas
            } else {
                (255, 255, 255) // Blanco para otros
            };

            unsafe {
                rlPushMatrix();
                rlTranslatef(screen_x, screen_y, 0.0);
                
                rlBegin(RL_QUADS);
                rlColor4ub(r, g, b, 255);
                rlVertex2f(0.0, 0.0);
                rlVertex2f(screen_w, 0.0);
                rlVertex2f(screen_w, screen_h);
                rlVertex2f(0.0, screen_h);
                rlEnd();
                
                rlPopMatrix();
            }
        }
    }

    /// Renderizar cuerpos N-Body con líneas de fuerza
    pub fn render_nbody(&self, world: &EcsWorld) {
        let render_data = world.get_render_data();

        for (x, y, _, w, _h) in render_data {
            let screen_x = (x - self.camera_x) * self.camera_zoom;
            let screen_y = (y - self.camera_y) * self.camera_zoom;
            let size = w * self.camera_zoom;

            // Dibujar cuerpo como círculo (usando líneas)
            unsafe {
                rlPushMatrix();
                rlTranslatef(screen_x, screen_y, 0.0);
                
                rlBegin(RL_LINES);
                rlColor4ub(255, 255, 0, 255); // Amarillo para cuerpos
                
                // Dibujar cruz para representar cuerpo
                rlVertex2f(-size/2.0, 0.0);
                rlVertex2f(size/2.0, 0.0);
                rlVertex2f(0.0, -size/2.0);
                rlVertex2f(0.0, size/2.0);
                
                rlEnd();
                rlPopMatrix();
            }
        }
    }

    /// Contar entidades para debug
    pub fn get_entity_count(&self, world: &EcsWorld) -> usize {
        world.entity_count()
    }
}

impl Default for EcsRenderer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// DEMO: 10K ENTIDADES
// ============================================================================

/// Crear demo de 10K entidades para testing
pub fn create_demo_world(count: usize) -> EcsWorld {
    let mut world = EcsWorld::new();
    
    // Crear entidades en posiciones aleatorias
    for i in 0..count {
        let x = (i % 100) as f32 * 10.0;
        let y = (i / 100) as f32 * 10.0;
        world.create_sprite_entity(x, y, "demo", 8.0, 8.0);
    }
    
    world
}

/// Crear demo N-Body con múltiples cuerpos
pub fn create_nbody_demo(body_count: usize) -> EcsWorld {
    let mut world = EcsWorld::new();
    
    // Crear cuerpo central masivo (estático)
    world.create_static_body_entity(400.0, 300.0, 10000.0);
    
    // Crear cuerpos orbitando
    for i in 0..body_count {
        let angle = (i as f32 / body_count as f32) * std::f32::consts::PI * 2.0;
        let radius = 100.0 + (i as f32 * 10.0);
        
        let x = 400.0 + angle.cos() * radius;
        let y = 300.0 + angle.sin() * radius;
        
        // Velocidad tangencial para órbita
        let vx = -angle.sin() * 50.0;
        let vy = angle.cos() * 50.0;
        
        world.create_body_entity(x, y, 100.0, vx, vy);
    }
    
    world
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecs_renderer_creation() {
        let renderer = EcsRenderer::new();
        assert_eq!(renderer.camera_x, 0.0);
        assert_eq!(renderer.camera_y, 0.0);
        assert_eq!(renderer.camera_zoom, 1.0);
    }

    #[test]
    fn test_create_demo_world() {
        let world = create_demo_world(100);
        assert_eq!(world.entity_count(), 100);
    }

    #[test]
    fn test_create_nbody_demo() {
        let world = create_nbody_demo(5);
        assert_eq!(world.entity_count(), 6); // 1 central + 5 orbitando
    }
}
