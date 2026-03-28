// crates/rydit-rs/src/modules/entity.rs
// Entity System - Sistema de Entidades para RyDit
//
// Tipos de entidades:
// - player: Personaje principal controlado por input
// - enemy: Enemigos con IA (patrol, chase)
// - boss: Jefes con fases y ataques especiales
// - trap: Trampas (spike, arrow, fire)
// - coin: Monedas y items recolectables
//
// Decoración usa Assets Manager directamente (no entity::)

use blast_core::{Executor, Valor};
use lizer::{Expr, Stmt};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::eval::evaluar_expr;

// ============================================================================
// ENTITY STRUCT
// ============================================================================

/// Entidad base (todas comparten estos campos)
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: String,
    pub entity_type: String,  // "player", "enemy", "boss", "trap", "coin"
    pub x: f32,
    pub y: f32,
    pub vx: f32,  // velocidad X
    pub vy: f32,  // velocidad Y
    pub width: f32,
    pub height: f32,
    pub sprite_id: String,
    pub is_active: bool,
    pub is_collidable: bool,
    // Datos específicos por tipo (se llenan según el tipo)
    pub data: HashMap<String, Valor>,
}

impl Entity {
    pub fn new(id: &str, entity_type: &str, x: f32, y: f32) -> Self {
        let mut data = HashMap::new();
        
        // Valores por defecto según tipo
        match entity_type {
            "player" => {
                data.insert("speed".to_string(), Valor::Num(200.0));
                data.insert("health".to_string(), Valor::Num(100.0));
                data.insert("max_health".to_string(), Valor::Num(100.0));
                data.insert("state".to_string(), Valor::Texto("idle".to_string()));
                data.insert("is_grounded".to_string(), Valor::Bool(true));
                data.insert("gravity".to_string(), Valor::Num(500.0));
            }
            "enemy" => {
                data.insert("ai_type".to_string(), Valor::Texto("patrol".to_string()));
                data.insert("health".to_string(), Valor::Num(30.0));
                data.insert("damage".to_string(), Valor::Num(10.0));
                data.insert("reward".to_string(), Valor::Num(10.0));
                data.insert("speed".to_string(), Valor::Num(100.0));
                data.insert("detection_range".to_string(), Valor::Num(200.0));
                data.insert("state".to_string(), Valor::Texto("patrol".to_string()));
            }
            "boss" => {
                data.insert("health".to_string(), Valor::Num(200.0));
                data.insert("damage".to_string(), Valor::Num(25.0));
                data.insert("current_phase".to_string(), Valor::Texto("phase1".to_string()));
                data.insert("is_enraged".to_string(), Valor::Bool(false));
            }
            "trap" => {
                data.insert("trap_type".to_string(), Valor::Texto("spike".to_string()));
                data.insert("damage".to_string(), Valor::Num(15.0));
                data.insert("trigger_range".to_string(), Valor::Num(30.0));
                data.insert("is_triggered".to_string(), Valor::Bool(false));
                data.insert("visible".to_string(), Valor::Bool(true));
                data.insert("cooldown".to_string(), Valor::Num(2000.0));
            }
            "coin" => {
                data.insert("value".to_string(), Valor::Num(5.0));
                data.insert("coin_type".to_string(), Valor::Texto("gold".to_string()));
                data.insert("is_collected".to_string(), Valor::Bool(false));
            }
            _ => {}
        }
        
        Self {
            id: id.to_string(),
            entity_type: entity_type.to_string(),
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            width: 32.0,
            height: 32.0,
            sprite_id: String::new(),
            is_active: true,
            is_collidable: true,
            data,
        }
    }
    
    /// Obtener un dato del HashMap
    pub fn get_data(&self, key: &str) -> Option<&Valor> {
        self.data.get(key)
    }
    
    /// Establecer un dato en el HashMap
    pub fn set_data(&mut self, key: &str, value: Valor) {
        self.data.insert(key.to_string(), value);
    }
}

// ============================================================================
// ENTITY MANAGER (GLOBAL)
// ============================================================================

thread_local! {
    static ENTITY_MANAGER: Rc<RefCell<EntityManager>> = Rc::new(RefCell::new(EntityManager::new()));
}

/// Gestor de todas las entidades
pub struct EntityManager {
    entities: HashMap<String, Entity>,
    next_id: u32,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            next_id: 0,
        }
    }
    
    /// Crear nueva entidad
    pub fn create(&mut self, entity_type: &str, x: f32, y: f32) -> String {
        let id = format!("{}_{}", entity_type, self.next_id);
        self.next_id += 1;
        
        let entity = Entity::new(&id, entity_type, x, y);
        self.entities.insert(id.clone(), entity);
        
        id
    }
    
    /// Obtener entidad por ID
    pub fn get(&self, id: &str) -> Option<&Entity> {
        self.entities.get(id)
    }
    
    /// Obtener entidad mutable por ID
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Entity> {
        self.entities.get_mut(id)
    }
    
    /// Destruir entidad
    pub fn destroy(&mut self, id: &str) -> bool {
        self.entities.remove(id).is_some()
    }
    
    /// Obtener todas las entidades de un tipo
    pub fn get_by_type(&self, entity_type: &str) -> Vec<String> {
        self.entities
            .iter()
            .filter(|(_, e)| e.entity_type == entity_type && e.is_active)
            .map(|(id, _)| id.clone())
            .collect()
    }
    
    /// Contar entidades de un tipo
    pub fn count_by_type(&self, entity_type: &str) -> usize {
        self.entities
            .iter()
            .filter(|(_, e)| e.entity_type == entity_type && e.is_active)
            .count()
    }
    
    /// Contar todas las entidades activas
    pub fn count(&self) -> usize {
        self.entities.values().filter(|e| e.is_active).count()
    }
    
    /// Obtener todas las entidades activas
    pub fn get_all(&self) -> Vec<String> {
        self.entities
            .iter()
            .filter(|(_, e)| e.is_active)
            .map(|(id, _)| id.clone())
            .collect()
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Obtener referencia al Entity Manager global
pub fn get_entity_manager() -> Rc<RefCell<EntityManager>> {
    ENTITY_MANAGER.with(|e| e.clone())
}

// ============================================================================
// FUNCIONES ENTITY MANAGER
// ============================================================================

/// entity::create(type, x, y) - Crear nueva entidad
pub fn entity_create(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 3 {
        return Valor::Error("entity::create() requiere 3 argumentos: type, x, y".to_string());
    }
    
    let type_val = evaluar_expr(&args[0], executor, funcs);
    let x_val = evaluar_expr(&args[1], executor, funcs);
    let y_val = evaluar_expr(&args[2], executor, funcs);
    
    let entity_type = match type_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("entity::create() type debe ser texto".to_string()),
    };
    
    // Validar tipo
    if !["player", "enemy", "boss", "trap", "coin"].contains(&entity_type.as_str()) {
        return Valor::Error(format!(
            "entity::create() tipo '{}' no válido. Usa: player, enemy, boss, trap, coin",
            entity_type
        ));
    }
    
    let x = match x_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("entity::create() x debe ser número".to_string()),
    };
    
    let y = match y_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("entity::create() y debe ser número".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    let id = em_ref.create(&entity_type, x, y);
    
    Valor::Texto(id)
}

/// entity::destroy(id) - Destruir entidad
pub fn entity_destroy(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("entity::destroy() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("entity::destroy() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if em_ref.destroy(&id) {
        Valor::Texto(format!("entity::destroy() - '{}' destruida", id))
    } else {
        Valor::Error(format!("entity::destroy() La entidad '{}' no existe", id))
    }
}

/// entity::get_by_type(type) - Obtener IDs por tipo
pub fn entity_get_by_type(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("entity::get_by_type() requiere 1 argumento: type".to_string());
    }
    
    let type_val = evaluar_expr(&args[0], executor, funcs);
    let entity_type = match type_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("entity::get_by_type() type debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let em_ref = em.borrow();
    let ids = em_ref.get_by_type(&entity_type);
    
    let result: Vec<Valor> = ids.into_iter().map(Valor::Texto).collect();
    Valor::Array(result)
}

/// entity::count() - Contar todas las entidades activas
pub fn entity_count(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let em = get_entity_manager();
    let em_ref = em.borrow();
    Valor::Num(em_ref.count() as f64)
}

/// entity::count_by_type(type) - Contar entidades por tipo
pub fn entity_count_by_type(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("entity::count_by_type() requiere 1 argumento: type".to_string());
    }
    
    let type_val = evaluar_expr(&args[0], executor, funcs);
    let entity_type = match type_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("entity::count_by_type() type debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let em_ref = em.borrow();
    Valor::Num(em_ref.count_by_type(&entity_type) as f64)
}

/// entity::get_all() - Obtener todas las entidades activas
pub fn entity_get_all(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let em = get_entity_manager();
    let em_ref = em.borrow();
    let ids = em_ref.get_all();
    
    let result: Vec<Valor> = ids.into_iter().map(Valor::Texto).collect();
    Valor::Array(result)
}

// ============================================================================
// FUNCIONES BASE DE ENTIDAD (compartidas por todos los tipos)
// ============================================================================

/// entity::set_position(id, x, y) - Establecer posición
pub fn entity_set_position(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 3 {
        return Valor::Error("entity::set_position() requiere 3 argumentos: id, x, y".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let x_val = evaluar_expr(&args[1], executor, funcs);
    let y_val = evaluar_expr(&args[2], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("entity::set_position() id debe ser texto".to_string()),
    };
    
    let x = match x_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("entity::set_position() x debe ser número".to_string()),
    };
    
    let y = match y_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("entity::set_position() y debe ser número".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        entity.x = x;
        entity.y = y;
        Valor::Texto(format!("entity::set_position('{}', {}, {})", id, x, y))
    } else {
        Valor::Error(format!("entity::set_position() La entidad '{}' no existe", id))
    }
}

/// entity::get_position(id) - Obtener posición
pub fn entity_get_position(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("entity::get_position() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("entity::get_position() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let em_ref = em.borrow();
    
    if let Some(entity) = em_ref.get(&id) {
        Valor::Array(vec![
            Valor::Num(entity.x as f64),
            Valor::Num(entity.y as f64),
        ])
    } else {
        Valor::Error(format!("entity::get_position() La entidad '{}' no existe", id))
    }
}

/// entity::set_sprite(id, sprite_id) - Establecer sprite
pub fn entity_set_sprite(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("entity::set_sprite() requiere 2 argumentos: id, sprite_id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let sprite_id_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("entity::set_sprite() id debe ser texto".to_string()),
    };
    
    let sprite_id = match sprite_id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("entity::set_sprite() sprite_id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        entity.sprite_id = sprite_id.clone();
        Valor::Texto(format!("entity::set_sprite('{}', '{}')", id, sprite_id))
    } else {
        Valor::Error(format!("entity::set_sprite() La entidad '{}' no existe", id))
    }
}

/// entity::set_collidable(id, collidable) - Establecer colisión
pub fn entity_set_collidable(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("entity::set_collidable() requiere 2 argumentos: id, collidable".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let collidable_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("entity::set_collidable() id debe ser texto".to_string()),
    };
    
    let collidable = match collidable_val {
        Valor::Bool(b) => b,
        Valor::Num(n) => n != 0.0,
        _ => return Valor::Error("entity::set_collidable() collidable debe ser bool o número".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        entity.is_collidable = collidable;
        Valor::Texto(format!("entity::set_collidable('{}', {})", id, collidable))
    } else {
        Valor::Error(format!("entity::set_collidable() La entidad '{}' no existe", id))
    }
}

/// entity::is_collidable(id) - Verificar si tiene colisión
pub fn entity_is_collidable(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("entity::is_collidable() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("entity::is_collidable() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let em_ref = em.borrow();
    
    if let Some(entity) = em_ref.get(&id) {
        Valor::Bool(entity.is_collidable)
    } else {
        Valor::Error(format!("entity::is_collidable() La entidad '{}' no existe", id))
    }
}

/// entity::set_active(id, active) - Activar/desactivar entidad
pub fn entity_set_active(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("entity::set_active() requiere 2 argumentos: id, active".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let active_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("entity::set_active() id debe ser texto".to_string()),
    };
    
    let active = match active_val {
        Valor::Bool(b) => b,
        Valor::Num(n) => n != 0.0,
        _ => return Valor::Error("entity::set_active() active debe ser bool o número".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        entity.is_active = active;
        Valor::Texto(format!("entity::set_active('{}', {})", id, active))
    } else {
        Valor::Error(format!("entity::set_active() La entidad '{}' no existe", id))
    }
}

/// entity::is_active(id) - Verificar si está activa
pub fn entity_is_active(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("entity::is_active() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("entity::is_active() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let em_ref = em.borrow();
    
    if let Some(entity) = em_ref.get(&id) {
        Valor::Bool(entity.is_active)
    } else {
        Valor::Error(format!("entity::is_active() La entidad '{}' no existe", id))
    }
}

// ============================================================================
// PLAYER COMPONENT
// ============================================================================

/// player::set_speed(id, speed) - Establecer velocidad
pub fn player_set_speed(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("player::set_speed() requiere 2 argumentos: id, speed".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let speed_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("player::set_speed() id debe ser texto".to_string()),
    };
    
    let speed = match speed_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("player::set_speed() speed debe ser número".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "player" {
            return Valor::Error(format!("entity '{}' no es de tipo 'player'", id));
        }
        entity.set_data("speed", Valor::Num(speed as f64));
        Valor::Texto(format!("player::set_speed('{}', {})", id, speed))
    } else {
        Valor::Error(format!("player::set_speed() La entidad '{}' no existe", id))
    }
}

/// player::get_speed(id) - Obtener velocidad
pub fn player_get_speed(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("player::get_speed() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("player::get_speed() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let em_ref = em.borrow();
    
    if let Some(entity) = em_ref.get(&id) {
        if let Some(Valor::Num(speed)) = entity.get_data("speed") {
            Valor::Num(*speed)
        } else {
            Valor::Num(200.0)  // default
        }
    } else {
        Valor::Error(format!("player::get_speed() La entidad '{}' no existe", id))
    }
}

/// player::move_left(id) - Mover izquierda
pub fn player_move_left(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("player::move_left() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("player::move_left() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "player" {
            return Valor::Error(format!("entity '{}' no es de tipo 'player'", id));
        }
        
        let speed = match entity.get_data("speed") {
            Some(Valor::Num(s)) => *s as f32,
            _ => 200.0,
        };
        
        entity.vx = -speed;
        entity.set_data("state", Valor::Texto("run".to_string()));
        
        Valor::Texto(format!("player::move_left('{}') - vx={}", id, entity.vx))
    } else {
        Valor::Error(format!("player::move_left() La entidad '{}' no existe", id))
    }
}

/// player::move_right(id) - Mover derecha
pub fn player_move_right(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("player::move_right() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("player::move_right() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "player" {
            return Valor::Error(format!("entity '{}' no es de tipo 'player'", id));
        }
        
        let speed = match entity.get_data("speed") {
            Some(Valor::Num(s)) => *s as f32,
            _ => 200.0,
        };
        
        entity.vx = speed;
        entity.set_data("state", Valor::Texto("run".to_string()));
        
        Valor::Texto(format!("player::move_right('{}') - vx={}", id, entity.vx))
    } else {
        Valor::Error(format!("player::move_right() La entidad '{}' no existe", id))
    }
}

/// player::jump(id) - Saltar
pub fn player_jump(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("player::jump() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("player::jump() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "player" {
            return Valor::Error(format!("entity '{}' no es de tipo 'player'", id));
        }
        
        let is_grounded = match entity.get_data("is_grounded") {
            Some(Valor::Bool(g)) => *g,
            _ => false,
        };
        
        if is_grounded {
            entity.vy = -300.0;  // fuerza de salto
            entity.set_data("is_grounded", Valor::Bool(false));
            entity.set_data("state", Valor::Texto("jump".to_string()));
            Valor::Texto(format!("player::jump('{}') - vy={}", id, entity.vy))
        } else {
            Valor::Texto(format!("player::jump('{}') - ya está en el aire", id))
        }
    } else {
        Valor::Error(format!("player::jump() La entidad '{}' no existe", id))
    }
}

/// player::apply_gravity(id, gravity) - Aplicar gravedad
pub fn player_apply_gravity(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("player::apply_gravity() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("player::apply_gravity() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "player" {
            return Valor::Error(format!("entity '{}' no es de tipo 'player'", id));
        }
        
        let gravity = match entity.get_data("gravity") {
            Some(Valor::Num(g)) => *g as f32,
            _ => 500.0,
        };
        
        // Aplicar gravedad (asumiendo dt = 0.016 para 60 FPS)
        let dt = 0.016;
        entity.vy += gravity * dt;
        
        // Actualizar posición
        entity.x += entity.vx * dt;
        entity.y += entity.vy * dt;
        
        // Verificar suelo (simplificado: y >= 500 es suelo)
        if entity.y >= 500.0 {
            entity.y = 500.0;
            entity.vy = 0.0;
            entity.set_data("is_grounded", Valor::Bool(true));
            
            let state = match entity.get_data("state") {
                Some(Valor::Texto(s)) if s == "jump" => "idle".to_string(),
                _ => entity.get_data("state").map(|v| match v {
                    Valor::Texto(s) => s.clone(),
                    _ => "idle".to_string(),
                }).unwrap_or("idle".to_string()),
            };
            entity.set_data("state", Valor::Texto(state));
        }
        
        Valor::Texto(format!("player::apply_gravity('{}') - vy={}, y={}", id, entity.vy, entity.y))
    } else {
        Valor::Error(format!("player::apply_gravity() La entidad '{}' no existe", id))
    }
}

/// player::get_state(id) - Obtener estado
pub fn player_get_state(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("player::get_state() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("player::get_state() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let em_ref = em.borrow();
    
    if let Some(entity) = em_ref.get(&id) {
        if let Some(Valor::Texto(state)) = entity.get_data("state") {
            Valor::Texto(state.clone())
        } else {
            Valor::Texto("idle".to_string())
        }
    } else {
        Valor::Error(format!("player::get_state() La entidad '{}' no existe", id))
    }
}

/// player::is_grounded(id) - Verificar si está en suelo
pub fn player_is_grounded(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("player::is_grounded() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("player::is_grounded() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let em_ref = em.borrow();
    
    if let Some(entity) = em_ref.get(&id) {
        if let Some(Valor::Bool(grounded)) = entity.get_data("is_grounded") {
            Valor::Bool(*grounded)
        } else {
            Valor::Bool(false)
        }
    } else {
        Valor::Error(format!("player::is_grounded() La entidad '{}' no existe", id))
    }
}

/// player::set_health(id, health) - Establecer vida
pub fn player_set_health(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("player::set_health() requiere 2 argumentos: id, health".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let health_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("player::set_health() id debe ser texto".to_string()),
    };
    
    let health = match health_val {
        Valor::Num(n) => n,
        _ => return Valor::Error("player::set_health() health debe ser número".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "player" {
            return Valor::Error(format!("entity '{}' no es de tipo 'player'", id));
        }
        entity.set_data("health", Valor::Num(health));
        Valor::Texto(format!("player::set_health('{}', {})", id, health))
    } else {
        Valor::Error(format!("player::set_health() La entidad '{}' no existe", id))
    }
}

/// player::get_health(id) - Obtener vida
pub fn player_get_health(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("player::get_health() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("player::get_health() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let em_ref = em.borrow();
    
    if let Some(entity) = em_ref.get(&id) {
        if let Some(Valor::Num(health)) = entity.get_data("health") {
            Valor::Num(*health)
        } else {
            Valor::Num(100.0)
        }
    } else {
        Valor::Error(format!("player::get_health() La entidad '{}' no existe", id))
    }
}

/// player::take_damage(id, amount) - Recibir daño
pub fn player_take_damage(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("player::take_damage() requiere 2 argumentos: id, amount".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let amount_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("player::take_damage() id debe ser texto".to_string()),
    };
    
    let amount = match amount_val {
        Valor::Num(n) => n,
        _ => return Valor::Error("player::take_damage() amount debe ser número".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "player" {
            return Valor::Error(format!("entity '{}' no es de tipo 'player'", id));
        }
        
        let current_health = match entity.get_data("health") {
            Some(Valor::Num(h)) => *h,
            _ => 100.0,
        };
        
        let new_health = (current_health - amount).max(0.0);
        entity.set_data("health", Valor::Num(new_health));
        
        if new_health <= 0.0 {
            entity.is_active = false;
            Valor::Texto(format!("player::take_damage() - '{}' ha muerto", id))
        } else {
            Valor::Texto(format!("player::take_damage('{}', {}) - health={}", id, amount, new_health))
        }
    } else {
        Valor::Error(format!("player::take_damage() La entidad '{}' no existe", id))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test() -> (Executor, HashMap<String, (Vec<String>, Vec<Stmt>)>) {
        (Executor::nuevo(), HashMap::new())
    }

    #[test]
    fn test_entity_create() {
        let (mut executor, mut funcs) = setup_test();
        
        let args = vec![
            Expr::Texto("player".to_string()),
            Expr::Num(400.0),
            Expr::Num(300.0),
        ];
        let result = entity_create(&args, &mut executor, &mut funcs);
        
        if let Valor::Texto(id) = result {
            assert!(id.starts_with("player_"));
        } else {
            panic!("entity_create debería retornar Texto");
        }
    }

    #[test]
    fn test_entity_destroy() {
        let (mut executor, mut funcs) = setup_test();
        
        // Crear
        let create_args = vec![
            Expr::Texto("enemy".to_string()),
            Expr::Num(100.0),
            Expr::Num(100.0),
        ];
        let id_result = entity_create(&create_args, &mut executor, &mut funcs);
        let id = match id_result {
            Valor::Texto(s) => s,
            _ => panic!("entity_create falló"),
        };
        
        // Destruir
        let destroy_args = vec![Expr::Texto(id.clone())];
        let result = entity_destroy(&destroy_args, &mut executor, &mut funcs);
        
        if let Valor::Texto(msg) = result {
            assert!(msg.contains("destruida"));
        } else {
            panic!("entity_destroy debería retornar Texto");
        }
    }

    #[test]
    fn test_entity_get_by_type() {
        let (mut executor, mut funcs) = setup_test();
        
        // Crear varios enemies
        for i in 0..3 {
            let args = vec![
                Expr::Texto("enemy".to_string()),
                Expr::Num(100.0 + i as f64 * 50.0),
                Expr::Num(100.0),
            ];
            let _ = entity_create(&args, &mut executor, &mut funcs);
        }
        
        // Obtener por tipo
        let args = vec![Expr::Texto("enemy".to_string())];
        let result = entity_get_by_type(&args, &mut executor, &mut funcs);
        
        if let Valor::Array(ids) = result {
            assert_eq!(ids.len(), 3);
        } else {
            panic!("entity_get_by_type debería retornar Array");
        }
    }

    #[test]
    fn test_entity_set_position() {
        let (mut executor, mut funcs) = setup_test();
        
        // Crear player
        let create_args = vec![
            Expr::Texto("player".to_string()),
            Expr::Num(400.0),
            Expr::Num(300.0),
        ];
        let id_result = entity_create(&create_args, &mut executor, &mut funcs);
        let id = match id_result {
            Valor::Texto(s) => s,
            _ => panic!("entity_create falló"),
        };
        
        // Mover
        let args = vec![
            Expr::Texto(id.clone()),
            Expr::Num(500.0),
            Expr::Num(400.0),
        ];
        let result = entity_set_position(&args, &mut executor, &mut funcs);
        
        if let Valor::Texto(msg) = result {
            assert!(msg.contains("500"));
            assert!(msg.contains("400"));
        } else {
            panic!("entity_set_position debería retornar Texto");
        }
    }

    #[test]
    fn test_player_move() {
        let (mut executor, mut funcs) = setup_test();
        
        // Crear player
        let create_args = vec![
            Expr::Texto("player".to_string()),
            Expr::Num(400.0),
            Expr::Num(300.0),
        ];
        let id_result = entity_create(&create_args, &mut executor, &mut funcs);
        let id = match id_result {
            Valor::Texto(s) => s,
            _ => panic!("entity_create falló"),
        };
        
        // Mover derecha
        let args = vec![Expr::Texto(id.clone())];
        let result = player_move_right(&args, &mut executor, &mut funcs);
        
        if let Valor::Texto(msg) = result {
            assert!(msg.contains("vx="));
        } else {
            panic!("player_move_right debería retornar Texto");
        }
    }

    #[test]
    fn test_player_jump() {
        let (mut executor, mut funcs) = setup_test();
        
        let create_args = vec![
            Expr::Texto("player".to_string()),
            Expr::Num(400.0),
            Expr::Num(300.0),
        ];
        let id_result = entity_create(&create_args, &mut executor, &mut funcs);
        let id = match id_result {
            Valor::Texto(s) => s,
            _ => panic!("entity_create falló"),
        };
        
        let args = vec![Expr::Texto(id.clone())];
        let result = player_jump(&args, &mut executor, &mut funcs);
        
        if let Valor::Texto(msg) = result {
            assert!(msg.contains("vy="));
        } else {
            panic!("player_jump debería retornar Texto");
        }
    }

    #[test]
    fn test_player_health() {
        let (mut executor, mut funcs) = setup_test();
        
        let create_args = vec![
            Expr::Texto("player".to_string()),
            Expr::Num(400.0),
            Expr::Num(300.0),
        ];
        let id_result = entity_create(&create_args, &mut executor, &mut funcs);
        let id = match id_result {
            Valor::Texto(s) => s,
            _ => panic!("entity_create falló"),
        };
        
        // Set health
        let args = vec![
            Expr::Texto(id.clone()),
            Expr::Num(80.0),
        ];
        let _ = player_set_health(&args, &mut executor, &mut funcs);
        
        // Get health
        let args = vec![Expr::Texto(id.clone())];
        let result = player_get_health(&args, &mut executor, &mut funcs);
        
        if let Valor::Num(health) = result {
            assert_eq!(health, 80.0);
        } else {
            panic!("player_get_health debería retornar Num");
        }
    }

    #[test]
    fn test_entity_functions_exist() {
        let _ = entity_create;
        let _ = entity_destroy;
        let _ = entity_get_by_type;
        let _ = entity_count;
        let _ = entity_get_all;
        let _ = entity_set_position;
        let _ = entity_get_position;
        let _ = player_set_speed;
        let _ = player_move_left;
        let _ = player_move_right;
        let _ = player_jump;
        let _ = player_apply_gravity;
        let _ = player_get_health;
        let _ = player_take_damage;
    }
}
