// crates/rydit-rs/src/modules/physics.rs
// Físicas 2D con respuesta a colisiones
// v0.9.3: Gravedad, fricción, colisión con respuesta

use blast_core::{Executor, Valor};
use rydit_parser::{Expr, Stmt};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// ============================================================================
// CONSTANTES FÍSICAS
// ============================================================================

/// Gravedad por defecto (pixels/segundo²)
pub const DEFAULT_GRAVITY: f32 = 980.0;

/// Fricción por defecto (0-1)
pub const DEFAULT_FRICTION: f32 = 0.9;

/// Elasticidad por defecto (rebote, 0-1)
pub const DEFAULT_BOUNCE: f32 = 0.3;

// ============================================================================
// CUERPO FÍSICO
// ============================================================================

/// Cuerpo físico para simulación 2D
pub struct PhysicsBody {
    pub x: f32,
    pub y: f32,
    pub vx: f32, // Velocidad X
    pub vy: f32, // Velocidad Y
    pub width: f32,
    pub height: f32,
    #[allow(dead_code)] // Para futuras físicas de masa
    pub mass: f32,
    #[allow(dead_code)] // Para futuras físicas personalizables
    pub gravity: f32,
    pub friction: f32,   // Fricción
    pub bounce: f32,     // Rebote
    pub is_static: bool, // Si es estático (no se mueve)
    pub is_active: bool, // Si está activo en la simulación
}

impl PhysicsBody {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            width: w,
            height: h,
            mass: 1.0,
            gravity: DEFAULT_GRAVITY,
            friction: DEFAULT_FRICTION,
            bounce: DEFAULT_BOUNCE,
            is_static: false,
            is_active: true,
        }
    }

    /// Aplicar gravedad
    pub fn apply_gravity(&mut self, dt: f32) {
        if !self.is_static && self.gravity != 0.0 {
            self.vy += self.gravity * dt;
        }
    }

    /// Aplicar fricción
    pub fn apply_friction(&mut self, dt: f32) {
        if !self.is_static && self.friction != 1.0 {
            // Aplicar fricción exponencial para suavidad
            let friction_factor = self.friction.powf(dt * 60.0);
            self.vx *= friction_factor;
            self.vy *= friction_factor;
        }
    }

    /// Actualizar posición
    pub fn update(&mut self, dt: f32) {
        if !self.is_static {
            self.x += self.vx * dt;
            self.y += self.vy * dt;
        }
    }

    /// Verificar colisión con otro cuerpo (AABB)
    pub fn collides_with(&self, other: &PhysicsBody) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    /// Obtener overlap (superposición) con otro cuerpo
    pub fn get_overlap(&self, other: &PhysicsBody) -> (f32, f32) {
        // Calcular centros
        let self_cx = self.x + self.width / 2.0;
        let self_cy = self.y + self.height / 2.0;
        let other_cx = other.x + other.width / 2.0;
        let other_cy = other.y + other.height / 2.0;

        // Calcular diferencia de centros
        let dx = self_cx - other_cx;
        let dy = self_cy - other_cy;

        // Calcular suma de mitades
        let half_widths = (self.width + other.width) / 2.0;
        let half_heights = (self.height + other.height) / 2.0;

        // Calcular overlap
        let overlap_x = half_widths - dx.abs();
        let overlap_y = half_heights - dy.abs();

        // Si hay colisión, retornar overlap
        if overlap_x > 0.0 && overlap_y > 0.0 {
            // Retornar overlap en dirección mínima
            if overlap_x < overlap_y {
                (if dx > 0.0 { overlap_x } else { -overlap_x }, 0.0)
            } else {
                (0.0, if dy > 0.0 { overlap_y } else { -overlap_y })
            }
        } else {
            (0.0, 0.0)
        }
    }

    /// Resolver colisión (slide)
    #[allow(dead_code)] // Para futura integración con entity system
    pub fn resolve_collision(&mut self, other: &PhysicsBody) -> (f32, f32) {
        let (overlap_x, overlap_y) = self.get_overlap(other);

        // Aplicar resolución
        if overlap_x != 0.0 {
            self.x += overlap_x;
            self.vx = 0.0; // Detener velocidad en X
        }
        if overlap_y != 0.0 {
            self.y += overlap_y;
            // Si cae sobre algo, detener velocidad Y
            if overlap_y < 0.0 {
                self.vy = 0.0;
            } else {
                // Rebote si golpea desde abajo
                self.vy = -self.vy * self.bounce;
            }
        }

        (overlap_x, overlap_y)
    }

    /// Aplicar impulso
    #[allow(dead_code)] // Para futuras fuerzas e impulsos
    pub fn apply_impulse(&mut self, ix: f32, iy: f32) {
        if !self.is_static {
            self.vx += ix / self.mass;
            self.vy += iy / self.mass;
        }
    }

    /// Aplicar fuerza
    #[allow(dead_code)] // Para futuras fuerzas continuas
    pub fn apply_force(&mut self, fx: f32, fy: f32, dt: f32) {
        if !self.is_static {
            self.vx += (fx / self.mass) * dt;
            self.vy += (fy / self.mass) * dt;
        }
    }
}

// ============================================================================
// MUNDO FÍSICO
// ============================================================================

/// Mundo físico para simulación
pub struct PhysicsWorld {
    pub bodies: HashMap<String, PhysicsBody>,
    #[allow(dead_code)] // Para futuras físicas globales personalizables
    pub gravity: f32,
    pub bounds: Option<(f32, f32, f32, f32)>, // (x, y, w, h)
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            bodies: HashMap::new(),
            gravity: DEFAULT_GRAVITY,
            bounds: None,
        }
    }

    /// Crear cuerpo físico
    pub fn create_body(&mut self, id: &str, x: f32, y: f32, w: f32, h: f32) {
        self.bodies
            .insert(id.to_string(), PhysicsBody::new(x, y, w, h));
    }

    /// Obtener cuerpo
    pub fn get_body(&mut self, id: &str) -> Option<&mut PhysicsBody> {
        self.bodies.get_mut(id)
    }

    /// Eliminar cuerpo
    #[allow(dead_code)] // Para futura gestión dinámica de cuerpos
    pub fn remove_body(&mut self, id: &str) -> bool {
        self.bodies.remove(id).is_some()
    }

    /// Actualizar mundo físico
    pub fn update(&mut self, dt: f32) {
        // Aplicar gravedad y fricción a todos los cuerpos
        for (_, body) in &mut self.bodies {
            if body.is_active && !body.is_static {
                body.apply_gravity(dt);
                body.apply_friction(dt);
                body.update(dt);

                // Verificar límites
                if let Some((bx, by, bw, bh)) = self.bounds {
                    // Límite izquierdo
                    if body.x < bx {
                        body.x = bx;
                        body.vx = -body.vx * body.bounce;
                    }
                    // Límite derecho
                    if body.x + body.width > bx + bw {
                        body.x = bx + bw - body.width;
                        body.vx = -body.vx * body.bounce;
                    }
                    // Límite superior
                    if body.y < by {
                        body.y = by;
                        body.vy = -body.vy * body.bounce;
                    }
                    // Límite inferior (suelo)
                    if body.y + body.height > by + bh {
                        body.y = by + bh - body.height;
                        body.vy = 0.0;
                    }
                }
            }
        }

        // Resolver colisiones entre cuerpos
        let ids: Vec<String> = self.bodies.keys().cloned().collect();
        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                let id_a = ids[i].clone();
                let id_b = ids[j].clone();

                // Verificar si hay colisión (solo lectura)
                let collides = {
                    if let (Some(a), Some(b)) = (self.bodies.get(&id_a), self.bodies.get(&id_b)) {
                        a.is_active && b.is_active && a.collides_with(b)
                    } else {
                        false
                    }
                };

                if collides {
                    // Obtener información estática antes de mutar
                    let is_a_static = self.bodies.get(&id_a).map(|b| b.is_static).unwrap_or(true);
                    let is_b_static = self.bodies.get(&id_b).map(|b| b.is_static).unwrap_or(true);

                    if is_b_static && !is_a_static {
                        // Solo A se mueve - obtener overlap primero
                        let overlap = {
                            if let (Some(a), Some(b)) =
                                (self.bodies.get(&id_a), self.bodies.get(&id_b))
                            {
                                a.get_overlap(b)
                            } else {
                                (0.0, 0.0)
                            }
                        };
                        if let Some(body_a) = self.bodies.get_mut(&id_a) {
                            body_a.x += overlap.0;
                            body_a.y += overlap.1;
                            if overlap.1 < 0.0 {
                                body_a.vy = 0.0;
                            }
                            body_a.vx = 0.0;
                        }
                    } else if is_a_static && !is_b_static {
                        // Solo B se mueve
                        let overlap = {
                            if let (Some(a), Some(b)) =
                                (self.bodies.get(&id_a), self.bodies.get(&id_b))
                            {
                                b.get_overlap(a)
                            } else {
                                (0.0, 0.0)
                            }
                        };
                        if let Some(body_b) = self.bodies.get_mut(&id_b) {
                            body_b.x += overlap.0;
                            body_b.y += overlap.1;
                            if overlap.1 < 0.0 {
                                body_b.vy = 0.0;
                            }
                            body_b.vx = 0.0;
                        }
                    } else if !is_a_static && !is_b_static {
                        // Ambos dinámicos
                        let (ox, oy) = {
                            if let (Some(a), Some(b)) =
                                (self.bodies.get(&id_a), self.bodies.get(&id_b))
                            {
                                a.get_overlap(b)
                            } else {
                                (0.0, 0.0)
                            }
                        };

                        if ox != 0.0 {
                            if let Some(body_a) = self.bodies.get_mut(&id_a) {
                                body_a.x += ox / 2.0;
                            }
                            if let Some(body_b) = self.bodies.get_mut(&id_b) {
                                body_b.x -= ox / 2.0;
                            }
                        }
                        if oy != 0.0 {
                            if let Some(body_a) = self.bodies.get_mut(&id_a) {
                                body_a.y += oy / 2.0;
                            }
                            if let Some(body_b) = self.bodies.get_mut(&id_b) {
                                body_b.y -= oy / 2.0;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Verificar colisión entre dos cuerpos
    pub fn check_collision(&self, id_a: &str, id_b: &str) -> Option<bool> {
        if let (Some(a), Some(b)) = (self.bodies.get(id_a), self.bodies.get(id_b)) {
            Some(a.collides_with(b))
        } else {
            None
        }
    }

    /// Obtener posición de un cuerpo
    pub fn get_position(&self, id: &str) -> Option<(f32, f32)> {
        self.bodies.get(id).map(|b| (b.x, b.y))
    }

    /// Establecer posición de un cuerpo
    pub fn set_position(&mut self, id: &str, x: f32, y: f32) -> bool {
        if let Some(body) = self.bodies.get_mut(id) {
            body.x = x;
            body.y = y;
            true
        } else {
            false
        }
    }

    /// Establecer límites del mundo
    pub fn set_bounds(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.bounds = Some((x, y, w, h));
    }

    /// Limpiar límites
    #[allow(dead_code)] // Para futura gestión dinámica de límites
    pub fn clear_bounds(&mut self) {
        self.bounds = None;
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ESTADO GLOBAL
// ============================================================================

thread_local! {
    static PHYSICS_WORLD: Rc<RefCell<PhysicsWorld>> = Rc::new(RefCell::new(PhysicsWorld::new()));
}

/// Obtener referencia al mundo físico global
pub fn get_physics_world() -> Rc<RefCell<PhysicsWorld>> {
    PHYSICS_WORLD.with(|w| w.clone())
}

// ============================================================================
// FUNCIONES PARA RYDIT
// ============================================================================

/// physics::create_body(id, x, y, w, h) - Crear cuerpo físico
pub fn physics_create_body(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 5 {
        return Valor::Error(
            "physics::create_body() requiere 5 argumentos: id, x, y, w, h".to_string(),
        );
    }

    let id = match evaluar_expr(&args[0], executor, funcs) {
        Valor::Texto(s) => s,
        _ => return Valor::Error("El ID debe ser texto".to_string()),
    };

    let x = match evaluar_expr(&args[1], executor, funcs) {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("x debe ser número".to_string()),
    };

    let y = match evaluar_expr(&args[2], executor, funcs) {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("y debe ser número".to_string()),
    };

    let w = match evaluar_expr(&args[3], executor, funcs) {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("w debe ser número".to_string()),
    };

    let h = match evaluar_expr(&args[4], executor, funcs) {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("h debe ser número".to_string()),
    };

    let world = get_physics_world();
    let mut world_ref = world.borrow_mut();
    world_ref.create_body(&id, x, y, w, h);

    Valor::Texto(format!("Cuerpo físico '{}' creado", id))
}

/// physics::update(dt) - Actualizar mundo físico
pub fn physics_update(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    let dt = if !args.is_empty() {
        match evaluar_expr(&args[0], executor, funcs) {
            Valor::Num(n) => n as f32,
            _ => 1.0 / 60.0, // Default: 60 FPS
        }
    } else {
        1.0 / 60.0
    };

    let world = get_physics_world();
    let mut world_ref = world.borrow_mut();
    world_ref.update(dt);

    Valor::Texto(format!("Mundo físico actualizado (dt={})", dt))
}

/// physics::get_position(id) - Obtener posición de cuerpo
pub fn physics_get_position(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 1 {
        return Valor::Error("physics::get_position() requiere 1 argumento: id".to_string());
    }

    let id = match evaluar_expr(&args[0], executor, funcs) {
        Valor::Texto(s) => s,
        _ => return Valor::Error("El ID debe ser texto".to_string()),
    };

    let world = get_physics_world();
    let world_ref = world.borrow();

    if let Some((x, y)) = world_ref.get_position(&id) {
        Valor::Array(vec![Valor::Num(x as f64), Valor::Num(y as f64)])
    } else {
        Valor::Error(format!("Cuerpo '{}' no encontrado", id))
    }
}

/// physics::set_position(id, x, y) - Establecer posición
pub fn physics_set_position(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 3 {
        return Valor::Error("physics::set_position() requiere 3 argumentos: id, x, y".to_string());
    }

    let id = match evaluar_expr(&args[0], executor, funcs) {
        Valor::Texto(s) => s,
        _ => return Valor::Error("El ID debe ser texto".to_string()),
    };

    let x = match evaluar_expr(&args[1], executor, funcs) {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("x debe ser número".to_string()),
    };

    let y = match evaluar_expr(&args[2], executor, funcs) {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("y debe ser número".to_string()),
    };

    let world = get_physics_world();
    let mut world_ref = world.borrow_mut();

    if world_ref.set_position(&id, x, y) {
        Valor::Texto(format!("Posición de '{}' actualizada", id))
    } else {
        Valor::Error(format!("Cuerpo '{}' no encontrado", id))
    }
}

/// physics::set_velocity(id, vx, vy) - Establecer velocidad
pub fn physics_set_velocity(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 3 {
        return Valor::Error(
            "physics::set_velocity() requiere 3 argumentos: id, vx, vy".to_string(),
        );
    }

    let id = match evaluar_expr(&args[0], executor, funcs) {
        Valor::Texto(s) => s,
        _ => return Valor::Error("El ID debe ser texto".to_string()),
    };

    let vx = match evaluar_expr(&args[1], executor, funcs) {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("vx debe ser número".to_string()),
    };

    let vy = match evaluar_expr(&args[2], executor, funcs) {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("vy debe ser número".to_string()),
    };

    let world = get_physics_world();
    let mut world_ref = world.borrow_mut();

    if let Some(body) = world_ref.get_body(&id) {
        body.vx = vx;
        body.vy = vy;
        Valor::Texto(format!("Velocidad de '{}' actualizada", id))
    } else {
        Valor::Error(format!("Cuerpo '{}' no encontrado", id))
    }
}

/// physics::apply_gravity(id) - Aplicar gravedad a cuerpo
pub fn physics_apply_gravity(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 1 {
        return Valor::Error("physics::apply_gravity() requiere 1 argumento: id".to_string());
    }

    let id = match evaluar_expr(&args[0], executor, funcs) {
        Valor::Texto(s) => s,
        _ => return Valor::Error("El ID debe ser texto".to_string()),
    };

    let dt = 1.0 / 60.0; // Asumir 60 FPS

    let world = get_physics_world();
    let mut world_ref = world.borrow_mut();

    if let Some(body) = world_ref.get_body(&id) {
        body.apply_gravity(dt);
        Valor::Texto(format!("Gravedad aplicada a '{}'", id))
    } else {
        Valor::Error(format!("Cuerpo '{}' no encontrado", id))
    }
}

/// physics::set_bounds(x, y, w, h) - Establecer límites del mundo
pub fn physics_set_bounds(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 4 {
        return Valor::Error("physics::set_bounds() requiere 4 argumentos: x, y, w, h".to_string());
    }

    let x = match evaluar_expr(&args[0], executor, funcs) {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("x debe ser número".to_string()),
    };

    let y = match evaluar_expr(&args[1], executor, funcs) {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("y debe ser número".to_string()),
    };

    let w = match evaluar_expr(&args[2], executor, funcs) {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("w debe ser número".to_string()),
    };

    let h = match evaluar_expr(&args[3], executor, funcs) {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("h debe ser número".to_string()),
    };

    let world = get_physics_world();
    let mut world_ref = world.borrow_mut();
    world_ref.set_bounds(x, y, w, h);

    Valor::Texto(format!(
        "Límites del mundo establecidos: ({}, {}, {}, {})",
        x, y, w, h
    ))
}

/// physics::check_collision(id_a, id_b) - Verificar colisión
pub fn physics_check_collision(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 2 {
        return Valor::Error(
            "physics::check_collision() requiere 2 argumentos: id_a, id_b".to_string(),
        );
    }

    let id_a = match evaluar_expr(&args[0], executor, funcs) {
        Valor::Texto(s) => s,
        _ => return Valor::Error("id_a debe ser texto".to_string()),
    };

    let id_b = match evaluar_expr(&args[1], executor, funcs) {
        Valor::Texto(s) => s,
        _ => return Valor::Error("id_b debe ser texto".to_string()),
    };

    let world = get_physics_world();
    let world_ref = world.borrow();

    if let Some(collides) = world_ref.check_collision(&id_a, &id_b) {
        Valor::Bool(collides)
    } else {
        Valor::Error("Uno o ambos cuerpos no existen".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_body_creation() {
        let body = PhysicsBody::new(100.0, 200.0, 32.0, 32.0);
        assert_eq!(body.x, 100.0);
        assert_eq!(body.y, 200.0);
        assert_eq!(body.width, 32.0);
        assert_eq!(body.height, 32.0);
        assert_eq!(body.vx, 0.0);
        assert_eq!(body.vy, 0.0);
    }

    #[test]
    fn test_physics_gravity() {
        let mut body = PhysicsBody::new(0.0, 0.0, 10.0, 10.0);
        body.apply_gravity(1.0 / 60.0);
        assert!(body.vy > 0.0); // Debe caer hacia abajo
    }

    #[test]
    fn test_collision_detection() {
        let body_a = PhysicsBody::new(0.0, 0.0, 50.0, 50.0);
        let body_b = PhysicsBody::new(25.0, 25.0, 50.0, 50.0);
        assert!(body_a.collides_with(body_b));

        let body_c = PhysicsBody::new(100.0, 100.0, 50.0, 50.0);
        assert!(!body_a.collides_with(body_c));
    }
}
