// crates/rydit-ecs/src/lib.rs
// ECS (Entity Component System) para RyDit Engine
// Versión simplificada que compila

pub mod components;

pub use components::*;

// ============================================================================
// ECS WORLD - Versión simplificada
// ============================================================================

use std::collections::HashMap;

type EntityId = u64;

/// Entidad ECS
#[derive(Debug)]
pub struct Entity {
    pub id: EntityId,
    pub position: Option<Position>,
    pub velocity: Option<Velocity>,
    pub sprite: Option<Sprite>,
    pub particle: Option<Particle>,
    pub body: Option<Body>,
    pub collider: Option<Collider>,
    pub is_player: bool,
    pub is_enemy: bool,
    pub is_alive: bool,
}

impl Entity {
    pub fn new(id: EntityId) -> Self {
        Self {
            id,
            position: None,
            velocity: None,
            sprite: None,
            particle: None,
            body: None,
            collider: None,
            is_player: false,
            is_enemy: false,
            is_alive: true,
        }
    }
}

/// Mundo ECS
pub struct EcsWorld {
    pub entities: HashMap<EntityId, Entity>,
    pub next_id: EntityId,
    pub gravity: Gravity,
    pub delta_time: f32,
    pub render_config: RenderConfig,
}

impl EcsWorld {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            next_id: 0,
            gravity: Gravity::default(),
            delta_time: 0.016,
            render_config: RenderConfig::default(),
        }
    }

    /// Crear nueva entidad
    pub fn create_entity(&mut self) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;
        self.entities.insert(id, Entity::new(id));
        id
    }

    /// Eliminar entidad
    pub fn destroy_entity(&mut self, id: EntityId) {
        self.entities.remove(&id);
    }

    /// Agregar componente Position
    pub fn add_position(&mut self, entity: EntityId, x: f32, y: f32) -> bool {
        if let Some(e) = self.entities.get_mut(&entity) {
            e.position = Some(Position::new(x, y));
            true
        } else {
            false
        }
    }

    /// Agregar componente Velocity
    pub fn add_velocity(&mut self, entity: EntityId, vx: f32, vy: f32) -> bool {
        if let Some(e) = self.entities.get_mut(&entity) {
            e.velocity = Some(Velocity::new(vx, vy));
            true
        } else {
            false
        }
    }

    /// Agregar componente Sprite
    pub fn add_sprite(&mut self, entity: EntityId, texture: &str, w: f32, h: f32) -> bool {
        if let Some(e) = self.entities.get_mut(&entity) {
            e.sprite = Some(Sprite::new(texture, w, h));
            true
        } else {
            false
        }
    }

    /// Agregar componente Body (para N-Body)
    pub fn add_body(&mut self, entity: EntityId, mass: f32, is_static: bool) -> bool {
        if let Some(e) = self.entities.get_mut(&entity) {
            e.body = Some(if is_static {
                Body::static_body(mass)
            } else {
                Body::new(mass)
            });
            true
        } else {
            false
        }
    }

    /// Crear sprite entity
    pub fn create_sprite_entity(
        &mut self,
        x: f32,
        y: f32,
        texture: &str,
        w: f32,
        h: f32,
    ) -> EntityId {
        let id = self.create_entity();
        self.add_position(id, x, y);
        self.add_sprite(id, texture, w, h);
        id
    }

    /// Crear entity con velocidad
    pub fn create_moving_entity(
        &mut self,
        x: f32,
        y: f32,
        texture: &str,
        w: f32,
        h: f32,
        vx: f32,
        vy: f32,
    ) -> EntityId {
        let id = self.create_entity();
        self.add_position(id, x, y);
        self.add_velocity(id, vx, vy);
        self.add_sprite(id, texture, w, h);
        id
    }

    /// Crear cuerpo físico (N-Body)
    pub fn create_body_entity(&mut self, x: f32, y: f32, mass: f32, vx: f32, vy: f32) -> EntityId {
        let id = self.create_entity();
        self.add_position(id, x, y);
        self.add_velocity(id, vx, vy);
        self.add_body(id, mass, false);
        id
    }

    /// Crear cuerpo estático
    pub fn create_static_body_entity(&mut self, x: f32, y: f32, mass: f32) -> EntityId {
        let id = self.create_entity();
        self.add_position(id, x, y);
        self.add_body(id, mass, true);
        id
    }

    /// Crear jugador
    pub fn create_player(&mut self, x: f32, y: f32) -> EntityId {
        let id = self.create_entity();
        self.add_position(id, x, y);
        self.add_velocity(id, 0.0, 0.0);
        self.add_sprite(id, "player", 32.0, 32.0);
        self.add_collider(id, 32.0, 32.0, false);
        if let Some(e) = self.entities.get_mut(&id) {
            e.is_player = true;
        }
        id
    }

    /// Agregar collider
    pub fn add_collider(&mut self, entity: EntityId, w: f32, h: f32, is_trigger: bool) -> bool {
        if let Some(e) = self.entities.get_mut(&entity) {
            e.collider = Some(Collider {
                width: w,
                height: h,
                is_trigger,
            });
            true
        } else {
            false
        }
    }

    /// Agregar partícula
    pub fn add_particle(&mut self, entity: EntityId, lifetime: f32, size: f32) -> bool {
        if let Some(e) = self.entities.get_mut(&entity) {
            e.particle = Some(Particle::new(lifetime, size));
            true
        } else {
            false
        }
    }

    /// Crear partícula
    pub fn create_particle_entity(
        &mut self,
        x: f32,
        y: f32,
        lifetime: f32,
        size: f32,
        color: (f32, f32, f32, f32),
    ) -> EntityId {
        let id = self.create_entity();
        self.add_position(id, x, y);
        self.add_particle(id, lifetime, size);
        self.add_sprite(id, "", size, size);
        if let Some(e) = self.entities.get_mut(&id) {
            if let Some(ref mut s) = e.sprite {
                s.color = color;
            }
        }
        id
    }

    // ========================================================================
    // SISTEMAS DE ACTUALIZACIÓN
    // ========================================================================

    /// Sistema de movimiento
    pub fn update_movement(&mut self) {
        let dt = self.delta_time;

        // Obtener IDs de entidades con posición y velocidad
        let ids: Vec<EntityId> = self
            .entities
            .iter()
            .filter(|(_, e)| e.position.is_some() && e.velocity.is_some())
            .map(|(id, _)| *id)
            .collect();

        // Actualizar posiciones
        for id in ids {
            if let Some(e) = self.entities.get_mut(&id) {
                if let (Some(pos), Some(vel)) = (&mut e.position, &e.velocity) {
                    pos.x += vel.vx * dt;
                    pos.y += vel.vy * dt;
                }
            }
        }
    }

    /// Sistema de gravedad global
    pub fn update_gravity(&mut self) {
        let dt = self.delta_time;
        let gravity = self.gravity;

        // IDs con velocity y body (no estático)
        let ids_with_body: Vec<EntityId> = self
            .entities
            .iter()
            .filter(|(_, e)| e.velocity.is_some() && e.body.is_some() && !e.body.unwrap().is_static)
            .map(|(id, _)| *id)
            .collect();

        for id in ids_with_body {
            if let Some(e) = self.entities.get_mut(&id) {
                if let Some(vel) = &mut e.velocity {
                    vel.vx += gravity.x * dt;
                    vel.vy += gravity.y * dt;
                }
            }
        }

        // IDs con velocity pero sin body (gravedad global)
        let ids_no_body: Vec<EntityId> = self
            .entities
            .iter()
            .filter(|(_, e)| e.velocity.is_some() && e.body.is_none())
            .map(|(id, _)| *id)
            .collect();

        for id in ids_no_body {
            if let Some(e) = self.entities.get_mut(&id) {
                if let Some(vel) = &mut e.velocity {
                    vel.vx += gravity.x * dt;
                    vel.vy += gravity.y * dt;
                }
            }
        }
    }

    /// Sistema N-Body gravity (versión simplificada)
    pub fn update_nbody(&mut self) {
        let dt = self.delta_time;
        let g = self.gravity.g_constant;

        // Obtener todos los cuerpos con sus datos
        let bodies: Vec<(EntityId, Position, Body)> = self
            .entities
            .iter()
            .filter(|(_, e)| e.body.is_some() && e.position.is_some())
            .map(|(id, e)| (*id, e.position.unwrap(), e.body.unwrap()))
            .collect();

        let n = bodies.len();

        // Calcular fuerzas entre pares
        for i in 0..n {
            for j in (i + 1)..n {
                let (id_i, pos_i, body_i) = bodies[i];
                let (id_j, pos_j, body_j) = bodies[j];

                if body_i.is_static || body_j.is_static {
                    continue;
                }

                // Calcular distancia
                let dx = pos_j.x - pos_i.x;
                let dy = pos_j.y - pos_i.y;
                let dist_sq = dx * dx + dy * dy;
                let dist = dist_sq.sqrt();

                if dist < 0.001 {
                    continue;
                }

                // F = G * m1 * m2 / r²
                let force = g * body_i.mass * body_j.mass / dist_sq;

                // a = F / m
                let ax_i = force * dx / (dist * body_i.mass);
                let ay_i = force * dy / (dist * body_i.mass);

                let ax_j = -force * dx / (dist * body_j.mass);
                let ay_j = -force * dy / (dist * body_j.mass);

                // Aplicar aceleraciones
                if let Some(e) = self.entities.get_mut(&id_i) {
                    if let Some(vel) = &mut e.velocity {
                        vel.vx += ax_i * dt;
                        vel.vy += ay_i * dt;
                    }
                }

                if let Some(e) = self.entities.get_mut(&id_j) {
                    if let Some(vel) = &mut e.velocity {
                        vel.vx += ax_j * dt;
                        vel.vy += ay_j * dt;
                    }
                }
            }
        }
    }

    /// Sistema de partículas
    pub fn update_particles(&mut self) {
        let dt = self.delta_time;
        let to_remove: Vec<EntityId> = self
            .entities
            .iter()
            .filter(|(_, e)| e.particle.is_some() && e.particle.unwrap().lifetime <= dt)
            .map(|(id, _)| *id)
            .collect();

        for id in to_remove {
            self.entities.remove(&id);
        }

        // Actualizar lifetime de partículas restantes
        for e in self.entities.values_mut() {
            if let Some(ref mut p) = e.particle {
                p.lifetime -= dt;
            }
        }
    }

    /// Actualizar todos los sistemas
    pub fn update(&mut self, dt: f32) {
        self.delta_time = dt;
        self.update_movement();
        self.update_gravity();
        self.update_nbody();
        self.update_particles();
    }

    // ========================================================================
    // ACCESORES
    // ========================================================================

    pub fn set_gravity(&mut self, x: f32, y: f32) {
        self.gravity = Gravity {
            x,
            y,
            g_constant: 6.674e-11,
        };
    }

    pub fn get_gravity(&self) -> Gravity {
        self.gravity
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Obtener datos para render
    pub fn get_render_data(&self) -> Vec<(f32, f32, String, f32, f32)> {
        self.entities
            .iter()
            .filter_map(|(_, e)| {
                if let (Some(pos), Some(sprite)) = (&e.position, &e.sprite) {
                    Some((
                        pos.x,
                        pos.y,
                        sprite.texture_id.clone(),
                        sprite.width,
                        sprite.height,
                    ))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for EcsWorld {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_entity() {
        let mut world = EcsWorld::new();
        let id = world.create_entity();
        assert_eq!(world.entity_count(), 1);
        assert!(world.entities.contains_key(&id));
    }

    #[test]
    fn test_create_sprite() {
        let mut world = EcsWorld::new();
        let id = world.create_sprite_entity(100.0, 200.0, "test", 32.0, 32.0);
        assert!(world.entities.get(&id).is_some());
    }

    #[test]
    fn test_movement() {
        let mut world = EcsWorld::new();
        let id = world.create_moving_entity(0.0, 0.0, "test", 32.0, 32.0, 10.0, 0.0);

        world.update(0.1);

        let entity = world.entities.get(&id).unwrap();
        let pos = entity.position.unwrap();
        assert!(pos.x > 0.0, "La entidad debería haberse movido");
    }

    #[test]
    fn test_gravity() {
        let mut world = EcsWorld::new();
        // Crear entidad con posición y velocidad, pero SIN body (para gravedad global)
        let id = world.create_moving_entity(0.0, 0.0, "test", 32.0, 32.0, 0.0, 0.0);

        world.update(0.1);

        let entity = world.entities.get(&id).unwrap();
        let vel = entity.velocity.unwrap();
        // La gravedad por defecto es (0, 9.8), así que vy debería ser positivo (hacia abajo)
        assert!(
            vel.vy > 0.0,
            "La gravedad debería afectar la velocidad Y (vy={})",
            vel.vy
        );
    }

    #[test]
    fn test_nbody() {
        let mut world = EcsWorld::new();

        // Crear dos cuerpos
        world.create_body_entity(0.0, 0.0, 1000.0, 0.0, 0.0);
        world.create_body_entity(10.0, 0.0, 500.0, 0.0, 0.0);

        world.update(0.016);

        // Los cuerpos deberían atraerse
        assert!(world.entity_count() == 2);
    }
}
