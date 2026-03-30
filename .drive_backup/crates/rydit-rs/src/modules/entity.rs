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
    #[allow(dead_code)] // El id se usa para debugging y logging
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

/// player::move_up(id) - Mover arriba
pub fn player_move_up(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("player::move_up() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("player::move_up() id debe ser texto".to_string()),
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
        
        entity.vy = -speed;
        entity.set_data("state", Valor::Texto("run".to_string()));
        
        Valor::Texto(format!("player::move_up('{}') - vy={}", id, entity.vy))
    } else {
        Valor::Error(format!("player::move_up() La entidad '{}' no existe", id))
    }
}

/// player::move_down(id) - Mover abajo
pub fn player_move_down(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("player::move_down() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("player::move_down() id debe ser texto".to_string()),
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
        
        entity.vy = speed;
        entity.set_data("state", Valor::Texto("run".to_string()));
        
        Valor::Texto(format!("player::move_down('{}') - vy={}", id, entity.vy))
    } else {
        Valor::Error(format!("player::move_down() La entidad '{}' no existe", id))
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
// ENEMY COMPONENT
// ============================================================================

/// enemy::set_ai_type(id, ai_type) - Establecer tipo de IA
pub fn enemy_set_ai_type(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("enemy::set_ai_type() requiere 2 argumentos: id, ai_type".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let ai_type_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("enemy::set_ai_type() id debe ser texto".to_string()),
    };
    
    let ai_type = match ai_type_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("enemy::set_ai_type() ai_type debe ser texto".to_string()),
    };
    
    // Validar tipo de IA
    if !["patrol", "chase", "stationary"].contains(&ai_type.as_str()) {
        return Valor::Error("enemy::set_ai_type() usa: patrol, chase, stationary".to_string());
    }
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "enemy" {
            return Valor::Error(format!("entity '{}' no es de tipo 'enemy'", id));
        }
        entity.set_data("ai_type", Valor::Texto(ai_type.clone()));
        Valor::Texto(format!("enemy::set_ai_type('{}', '{}')", id, ai_type))
    } else {
        Valor::Error(format!("enemy::set_ai_type() La entidad '{}' no existe", id))
    }
}

/// enemy::set_patrol_points(id, [(x1,y1), (x2,y2)]) - Establecer puntos de patrulla
pub fn enemy_set_patrol_points(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("enemy::set_patrol_points() requiere 2 argumentos: id, points".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let points_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("enemy::set_patrol_points() id debe ser texto".to_string()),
    };
    
    // Guardar puntos como Valor (array de arrays)
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "enemy" {
            return Valor::Error(format!("entity '{}' no es de tipo 'enemy'", id));
        }
        entity.set_data("patrol_points", points_val.clone());
        Valor::Texto(format!("enemy::set_patrol_points('{}', ...)", id))
    } else {
        Valor::Error(format!("enemy::set_patrol_points() La entidad '{}' no existe", id))
    }
}

/// enemy::set_detection_range(id, range) - Establecer rango de detección
pub fn enemy_set_detection_range(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("enemy::set_detection_range() requiere 2 argumentos: id, range".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let range_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("enemy::set_detection_range() id debe ser texto".to_string()),
    };
    
    let range = match range_val {
        Valor::Num(n) => n,
        _ => return Valor::Error("enemy::set_detection_range() range debe ser número".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "enemy" {
            return Valor::Error(format!("entity '{}' no es de tipo 'enemy'", id));
        }
        entity.set_data("detection_range", Valor::Num(range));
        Valor::Texto(format!("enemy::set_detection_range('{}', {})", id, range))
    } else {
        Valor::Error(format!("enemy::set_detection_range() La entidad '{}' no existe", id))
    }
}

/// enemy::update_ai(id, player_id) - Actualizar IA del enemigo
pub fn enemy_update_ai(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("enemy::update_ai() requiere 2 argumentos: id, player_id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let player_id_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("enemy::update_ai() id debe ser texto".to_string()),
    };
    
    let player_id = match player_id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("enemy::update_ai() player_id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "enemy" {
            return Valor::Error(format!("entity '{}' no es de tipo 'enemy'", id));
        }
        
        // Obtener tipo de IA
        let ai_type = match entity.get_data("ai_type") {
            Some(Valor::Texto(t)) => t.clone(),
            _ => "patrol".to_string(),
        };
        
        // Obtener posición del jugador
        let player_em = get_entity_manager();
        let player_ref = player_em.borrow();
        let player = player_ref.get(&player_id);
        
        if let Some(player_entity) = player {
            let dx = player_entity.x - entity.x;
            let dy = player_entity.y - entity.y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            let detection_range = match entity.get_data("detection_range") {
                Some(Valor::Num(r)) => *r as f32,
                _ => 200.0,
            };
            
            if ai_type == "chase" || (ai_type == "patrol" && distance < detection_range) {
                // Perseguir jugador
                let speed = match entity.get_data("speed") {
                    Some(Valor::Num(s)) => *s as f32,
                    _ => 100.0,
                };
                
                let dt = 0.016;
                if dx.abs() > 1.0 {
                    entity.vx = (dx / distance) * speed;
                    entity.x += entity.vx * dt;
                }
                if dy.abs() > 1.0 {
                    entity.vy = (dy / distance) * speed;
                    entity.y += entity.vy * dt;
                }
                
                entity.set_data("state", Valor::Texto("chase".to_string()));
                
                return Valor::Texto(format!("enemy::update_ai('{}') - chasing player, dist={:.1}", id, distance));
            }
        }
        
        entity.set_data("state", Valor::Texto(ai_type.clone()));
        Valor::Texto(format!("enemy::update_ai('{}') - state={}", id, ai_type))
    } else {
        Valor::Error(format!("enemy::update_ai() La entidad '{}' no existe", id))
    }
}

/// enemy::is_alerted(id) - Verificar si está alerta
pub fn enemy_is_alerted(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("enemy::is_alerted() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("enemy::is_alerted() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let em_ref = em.borrow();
    
    if let Some(entity) = em_ref.get(&id) {
        if let Some(Valor::Texto(state)) = entity.get_data("state") {
            Valor::Bool(state == "chase" || state == "attack")
        } else {
            Valor::Bool(false)
        }
    } else {
        Valor::Error(format!("enemy::is_alerted() La entidad '{}' no existe", id))
    }
}

/// enemy::set_health(id, health) - Establecer vida
pub fn enemy_set_health(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("enemy::set_health() requiere 2 argumentos: id, health".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let health_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("enemy::set_health() id debe ser texto".to_string()),
    };
    
    let health = match health_val {
        Valor::Num(n) => n,
        _ => return Valor::Error("enemy::set_health() health debe ser número".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "enemy" {
            return Valor::Error(format!("entity '{}' no es de tipo 'enemy'", id));
        }
        entity.set_data("health", Valor::Num(health));
        Valor::Texto(format!("enemy::set_health('{}', {})", id, health))
    } else {
        Valor::Error(format!("enemy::set_health() La entidad '{}' no existe", id))
    }
}

/// enemy::set_damage(id, damage) - Establecer daño
pub fn enemy_set_damage(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("enemy::set_damage() requiere 2 argumentos: id, damage".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let damage_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("enemy::set_damage() id debe ser texto".to_string()),
    };
    
    let damage = match damage_val {
        Valor::Num(n) => n,
        _ => return Valor::Error("enemy::set_damage() damage debe ser número".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "enemy" {
            return Valor::Error(format!("entity '{}' no es de tipo 'enemy'", id));
        }
        entity.set_data("damage", Valor::Num(damage));
        Valor::Texto(format!("enemy::set_damage('{}', {})", id, damage))
    } else {
        Valor::Error(format!("enemy::set_damage() La entidad '{}' no existe", id))
    }
}

/// enemy::set_reward(id, coins) - Establecer recompensa de monedas
pub fn enemy_set_reward(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("enemy::set_reward() requiere 2 argumentos: id, coins".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let coins_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("enemy::set_reward() id debe ser texto".to_string()),
    };
    
    let coins = match coins_val {
        Valor::Num(n) => n,
        _ => return Valor::Error("enemy::set_reward() coins debe ser número".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "enemy" {
            return Valor::Error(format!("entity '{}' no es de tipo 'enemy'", id));
        }
        entity.set_data("reward", Valor::Num(coins));
        Valor::Texto(format!("enemy::set_reward('{}', {})", id, coins))
    } else {
        Valor::Error(format!("enemy::set_reward() La entidad '{}' no existe", id))
    }
}

// ============================================================================
// BOSS COMPONENT
// ============================================================================

/// boss::set_phases(id, ["phase1", "phase2"]) - Establecer fases
pub fn boss_set_phases(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("boss::set_phases() requiere 2 argumentos: id, phases".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let phases_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("boss::set_phases() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "boss" {
            return Valor::Error(format!("entity '{}' no es de tipo 'boss'", id));
        }
        entity.set_data("phases", phases_val.clone());
        Valor::Texto(format!("boss::set_phases('{}', ...)", id))
    } else {
        Valor::Error(format!("boss::set_phases() La entidad '{}' no existe", id))
    }
}

/// boss::get_current_phase(id) - Obtener fase actual
pub fn boss_get_current_phase(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("boss::get_current_phase() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("boss::get_current_phase() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let em_ref = em.borrow();
    
    if let Some(entity) = em_ref.get(&id) {
        if let Some(Valor::Texto(phase)) = entity.get_data("current_phase") {
            Valor::Texto(phase.clone())
        } else {
            Valor::Texto("phase1".to_string())
        }
    } else {
        Valor::Error(format!("boss::get_current_phase() La entidad '{}' no existe", id))
    }
}

/// boss::transition_to_phase(id, phase) - Transición de fase
pub fn boss_transition_to_phase(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("boss::transition_to_phase() requiere 2 argumentos: id, phase".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let phase_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("boss::transition_to_phase() id debe ser texto".to_string()),
    };
    
    let phase = match phase_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("boss::transition_to_phase() phase debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "boss" {
            return Valor::Error(format!("entity '{}' no es de tipo 'boss'", id));
        }
        entity.set_data("current_phase", Valor::Texto(phase.clone()));
        
        // Verificar si es fase enraged
        if phase == "enraged" || phase == "phase2" {
            entity.set_data("is_enraged", Valor::Bool(true));
        }
        
        Valor::Texto(format!("boss::transition_to_phase('{}', '{}')", id, phase))
    } else {
        Valor::Error(format!("boss::transition_to_phase() La entidad '{}' no existe", id))
    }
}

/// boss::set_arena_bounds(id, min_x, min_y, max_x, max_y) - Establecer límites de arena
pub fn boss_set_arena_bounds(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 5 {
        return Valor::Error("boss::set_arena_bounds() requiere 5 argumentos: id, min_x, min_y, max_x, max_y".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let min_x_val = evaluar_expr(&args[1], executor, funcs);
    let min_y_val = evaluar_expr(&args[2], executor, funcs);
    let max_x_val = evaluar_expr(&args[3], executor, funcs);
    let max_y_val = evaluar_expr(&args[4], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("boss::set_arena_bounds() id debe ser texto".to_string()),
    };
    
    let min_x = match min_x_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("boss::set_arena_bounds() min_x debe ser número".to_string()),
    };
    
    let min_y = match min_y_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("boss::set_arena_bounds() min_y debe ser número".to_string()),
    };
    
    let max_x = match max_x_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("boss::set_arena_bounds() max_x debe ser número".to_string()),
    };
    
    let max_y = match max_y_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("boss::set_arena_bounds() max_y debe ser número".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "boss" {
            return Valor::Error(format!("entity '{}' no es de tipo 'boss'", id));
        }
        entity.set_data("arena_min_x", Valor::Num(min_x as f64));
        entity.set_data("arena_min_y", Valor::Num(min_y as f64));
        entity.set_data("arena_max_x", Valor::Num(max_x as f64));
        entity.set_data("arena_max_y", Valor::Num(max_y as f64));
        Valor::Texto(format!("boss::set_arena_bounds('{}', {}, {}, {}, {})", id, min_x, min_y, max_x, max_y))
    } else {
        Valor::Error(format!("boss::set_arena_bounds() La entidad '{}' no existe", id))
    }
}

// ============================================================================
// COLLISION SYSTEM (FASE 2D)
// ============================================================================

/// collision::check_rect_rect(x1,y1,w1,h1, x2,y2,w2,h2) - Colisión rectángulo
pub fn collision_check_rect_rect(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 8 {
        return Valor::Error("collision::check_rect_rect() requiere 8 argumentos".to_string());
    }
    
    let vals: Vec<f32> = args.iter().map(|arg| {
        match evaluar_expr(arg, executor, funcs) {
            Valor::Num(n) => n as f32,
            _ => 0.0,
        }
    }).collect();
    
    let (x1, y1, w1, h1, x2, y2, w2, h2) = (vals[0], vals[1], vals[2], vals[3], vals[4], vals[5], vals[6], vals[7]);
    
    // AABB collision detection
    let collision = x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2;
    
    Valor::Bool(collision)
}

/// collision::check_circle_circle(x1,y1,r1, x2,y2,r2) - Colisión círculo
pub fn collision_check_circle_circle(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 6 {
        return Valor::Error("collision::check_circle_circle() requiere 6 argumentos".to_string());
    }
    
    let vals: Vec<f32> = args.iter().map(|arg| {
        match evaluar_expr(arg, executor, funcs) {
            Valor::Num(n) => n as f32,
            _ => 0.0,
        }
    }).collect();
    
    let (x1, y1, r1, x2, y2, r2) = (vals[0], vals[1], vals[2], vals[3], vals[4], vals[5]);
    
    let dx = x2 - x1;
    let dy = y2 - y1;
    let distance = (dx * dx + dy * dy).sqrt();
    let collision = distance < (r1 + r2);
    
    Valor::Bool(collision)
}

/// collision::check_rect_circle(rx,ry,rw,rh, cx,cy,cr) - Colisión rect-círculo
pub fn collision_check_rect_circle(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 7 {
        return Valor::Error("collision::check_rect_circle() requiere 7 argumentos".to_string());
    }
    
    let vals: Vec<f32> = args.iter().map(|arg| {
        match evaluar_expr(arg, executor, funcs) {
            Valor::Num(n) => n as f32,
            _ => 0.0,
        }
    }).collect();
    
    let (rx, ry, rw, rh, cx, cy, cr) = (vals[0], vals[1], vals[2], vals[3], vals[4], vals[5], vals[6]);
    
    // Find closest point on rect to circle center
    let closest_x = cx.max(rx).min(rx + rw);
    let closest_y = cy.max(ry).min(ry + rh);
    
    let dx = cx - closest_x;
    let dy = cy - closest_y;
    let collision = (dx * dx + dy * dy) < (cr * cr);
    
    Valor::Bool(collision)
}

/// collision::check_point_rect(px,py, rx,ry,rw,rh) - Punto en rectángulo
pub fn collision_check_point_rect(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 6 {
        return Valor::Error("collision::check_point_rect() requiere 6 argumentos".to_string());
    }
    
    let vals: Vec<f32> = args.iter().map(|arg| {
        match evaluar_expr(arg, executor, funcs) {
            Valor::Num(n) => n as f32,
            _ => 0.0,
        }
    }).collect();
    
    let (px, py, rx, ry, rw, rh) = (vals[0], vals[1], vals[2], vals[3], vals[4], vals[5]);
    
    let collision = px >= rx && px <= rx + rw && py >= ry && py <= ry + rh;
    
    Valor::Bool(collision)
}

/// collision::check(id1, id2) - Colisión entre entidades
pub fn collision_check(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("collision::check() requiere 2 argumentos: id1, id2".to_string());
    }
    
    let id1_val = evaluar_expr(&args[0], executor, funcs);
    let id2_val = evaluar_expr(&args[1], executor, funcs);
    
    let id1 = match id1_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("collision::check() id1 debe ser texto".to_string()),
    };
    
    let id2 = match id2_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("collision::check() id2 debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let em_ref = em.borrow();
    
    let entity1 = em_ref.get(&id1);
    let entity2 = em_ref.get(&id2);
    
    if let (Some(e1), Some(e2)) = (entity1, entity2) {
        // Usar colisión rectángulo simple
        let collision = e1.x < e2.x + e2.width && e1.x + e1.width > e2.x &&
                       e1.y < e2.y + e2.height && e1.y + e1.height > e2.y;
        Valor::Bool(collision)
    } else {
        Valor::Error("collision::check() Una o ambas entidades no existen".to_string())
    }
}

// ============================================================================
// AREA2D SYSTEM (Godot-style)
// ============================================================================

thread_local! {
    static AREA2D_MANAGER: Rc<RefCell<Area2DManager>> = Rc::new(RefCell::new(Area2DManager::new()));
}

/// Área 2D para detección de colisiones
#[derive(Debug, Clone)]
pub struct Area2D {
    #[allow(dead_code)] // El id se usa para debugging y logging
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub is_active: bool,
}

impl Area2D {
    pub fn new(id: &str, x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            id: id.to_string(),
            x, y,
            width: w,
            height: h,
            is_active: true,
        }
    }
}

/// Gestor de áreas 2D
pub struct Area2DManager {
    areas: HashMap<String, Area2D>,
    next_id: u32,
}

impl Area2DManager {
    pub fn new() -> Self {
        Self {
            areas: HashMap::new(),
            next_id: 0,
        }
    }
    
    pub fn create(&mut self, x: f32, y: f32, w: f32, h: f32) -> String {
        let id = format!("area2d_{}", self.next_id);
        self.next_id += 1;
        let area = Area2D::new(&id, x, y, w, h);
        self.areas.insert(id.clone(), area);
        id
    }
    
    pub fn set_position(&mut self, id: &str, x: f32, y: f32) -> bool {
        if let Some(area) = self.areas.get_mut(id) {
            area.x = x;
            area.y = y;
            true
        } else {
            false
        }
    }
    
    pub fn get_position(&self, id: &str) -> Option<(f32, f32)> {
        self.areas.get(id).map(|a| (a.x, a.y))
    }
    
    pub fn check(&self, id1: &str, id2: &str) -> Option<bool> {
        if let (Some(a1), Some(a2)) = (self.areas.get(id1), self.areas.get(id2)) {
            if !a1.is_active || !a2.is_active {
                return Some(false);
            }
            let collision = a1.x < a2.x + a2.width && a1.x + a1.width > a2.x &&
                           a1.y < a2.y + a2.height && a1.y + a1.height > a2.y;
            Some(collision)
        } else {
            None
        }
    }
    
    pub fn get_overlapping(&self, id: &str) -> Vec<String> {
        let mut overlapping = Vec::new();
        if let Some(area1) = self.areas.get(id) {
            for (id2, area2) in &self.areas {
                if id2 != id && area2.is_active {
                    let collision = area1.x < area2.x + area2.width && area1.x + area1.width > area2.x &&
                                   area1.y < area2.y + area2.height && area1.y + area1.height > area2.y;
                    if collision {
                        overlapping.push(id2.clone());
                    }
                }
            }
        }
        overlapping
    }
    
    pub fn destroy(&mut self, id: &str) -> bool {
        self.areas.remove(id).is_some()
    }
}

impl Default for Area2DManager {
    fn default() -> Self {
        Self::new()
    }
}

/// area2d::create(x, y, w, h) - Crear área 2D
pub fn area2d_create(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 4 {
        return Valor::Error("area2d::create() requiere 4 argumentos: x, y, w, h".to_string());
    }
    
    let vals: Vec<f32> = args.iter().map(|arg| {
        match evaluar_expr(arg, executor, funcs) {
            Valor::Num(n) => n as f32,
            _ => 0.0,
        }
    }).collect();
    
    let (x, y, w, h) = (vals[0], vals[1], vals[2], vals[3]);
    
    let am = get_area2d_manager();
    let mut am_ref = am.borrow_mut();
    let id = am_ref.create(x, y, w, h);
    
    Valor::Texto(id)
}

/// area2d::set_position(id, x, y) - Mover área
pub fn area2d_set_position(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 3 {
        return Valor::Error("area2d::set_position() requiere 3 argumentos: id, x, y".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let x_val = evaluar_expr(&args[1], executor, funcs);
    let y_val = evaluar_expr(&args[2], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("area2d::set_position() id debe ser texto".to_string()),
    };
    
    let x = match x_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("area2d::set_position() x debe ser número".to_string()),
    };
    
    let y = match y_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("area2d::set_position() y debe ser número".to_string()),
    };
    
    let am = get_area2d_manager();
    let mut am_ref = am.borrow_mut();
    
    if am_ref.set_position(&id, x, y) {
        Valor::Texto(format!("area2d::set_position('{}', {}, {})", id, x, y))
    } else {
        Valor::Error(format!("area2d::set_position() El área '{}' no existe", id))
    }
}

/// area2d::get_position(id) - Obtener posición
pub fn area2d_get_position(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("area2d::get_position() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("area2d::get_position() id debe ser texto".to_string()),
    };
    
    let am = get_area2d_manager();
    let am_ref = am.borrow();
    
    if let Some((x, y)) = am_ref.get_position(&id) {
        Valor::Array(vec![Valor::Num(x as f64), Valor::Num(y as f64)])
    } else {
        Valor::Error(format!("area2d::get_position() El área '{}' no existe", id))
    }
}

/// area2d::check(id1, id2) - Verificar colisión entre áreas
pub fn area2d_check(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("area2d::check() requiere 2 argumentos: id1, id2".to_string());
    }
    
    let id1_val = evaluar_expr(&args[0], executor, funcs);
    let id2_val = evaluar_expr(&args[1], executor, funcs);
    
    let id1 = match id1_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("area2d::check() id1 debe ser texto".to_string()),
    };
    
    let id2 = match id2_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("area2d::check() id2 debe ser texto".to_string()),
    };
    
    let am = get_area2d_manager();
    let am_ref = am.borrow();
    
    match am_ref.check(&id1, &id2) {
        Some(collision) => Valor::Bool(collision),
        None => Valor::Error("area2d::check() Una o ambas áreas no existen".to_string()),
    }
}

/// area2d::get_overlapping(id) - Obtener áreas que se superponen
pub fn area2d_get_overlapping(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("area2d::get_overlapping() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("area2d::get_overlapping() id debe ser texto".to_string()),
    };
    
    let am = get_area2d_manager();
    let am_ref = am.borrow();
    let ids = am_ref.get_overlapping(&id);
    
    let result: Vec<Valor> = ids.into_iter().map(Valor::Texto).collect();
    Valor::Array(result)
}

/// area2d::destroy(id) - Destruir área
pub fn area2d_destroy(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("area2d::destroy() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("area2d::destroy() id debe ser texto".to_string()),
    };
    
    let am = get_area2d_manager();
    let mut am_ref = am.borrow_mut();
    
    if am_ref.destroy(&id) {
        Valor::Texto(format!("area2d::destroy() - '{}' destruida", id))
    } else {
        Valor::Error(format!("area2d::destroy() El área '{}' no existe", id))
    }
}

/// Obtener referencia al Area2D Manager global
pub fn get_area2d_manager() -> Rc<RefCell<Area2DManager>> {
    AREA2D_MANAGER.with(|a| a.clone())
}

// ============================================================================
// TRAP COMPONENT (FASE 2C)
// ============================================================================

/// trap::set_type(id, type) - Establecer tipo de trampa
pub fn trap_set_type(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("trap::set_type() requiere 2 argumentos: id, type".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let type_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("trap::set_type() id debe ser texto".to_string()),
    };
    
    let trap_type = match type_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("trap::set_type() type debe ser texto".to_string()),
    };
    
    // Validar tipo
    if !["spike", "arrow", "fire", "falling", "saw"].contains(&trap_type.as_str()) {
        return Valor::Error("trap::set_type() usa: spike, arrow, fire, falling, saw".to_string());
    }
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "trap" {
            return Valor::Error(format!("entity '{}' no es de tipo 'trap'", id));
        }
        entity.set_data("trap_type", Valor::Texto(trap_type.clone()));
        Valor::Texto(format!("trap::set_type('{}', '{}')", id, trap_type))
    } else {
        Valor::Error(format!("trap::set_type() La entidad '{}' no existe", id))
    }
}

/// trap::set_damage(id, damage) - Establecer daño
pub fn trap_set_damage(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("trap::set_damage() requiere 2 argumentos: id, damage".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let damage_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("trap::set_damage() id debe ser texto".to_string()),
    };
    
    let damage = match damage_val {
        Valor::Num(n) => n,
        _ => return Valor::Error("trap::set_damage() damage debe ser número".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "trap" {
            return Valor::Error(format!("entity '{}' no es de tipo 'trap'", id));
        }
        entity.set_data("damage", Valor::Num(damage));
        Valor::Texto(format!("trap::set_damage('{}', {})", id, damage))
    } else {
        Valor::Error(format!("trap::set_damage() La entidad '{}' no existe", id))
    }
}

/// trap::set_trigger_range(id, range) - Rango de activación
pub fn trap_set_trigger_range(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("trap::set_trigger_range() requiere 2 argumentos: id, range".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let range_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("trap::set_trigger_range() id debe ser texto".to_string()),
    };
    
    let range = match range_val {
        Valor::Num(n) => n,
        _ => return Valor::Error("trap::set_trigger_range() range debe ser número".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "trap" {
            return Valor::Error(format!("entity '{}' no es de tipo 'trap'", id));
        }
        entity.set_data("trigger_range", Valor::Num(range));
        Valor::Texto(format!("trap::set_trigger_range('{}', {})", id, range))
    } else {
        Valor::Error(format!("trap::set_trigger_range() La entidad '{}' no existe", id))
    }
}

/// trap::set_visible(id, visible) - Hacer visible/invisible
pub fn trap_set_visible(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("trap::set_visible() requiere 2 argumentos: id, visible".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let visible_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("trap::set_visible() id debe ser texto".to_string()),
    };
    
    let visible = match visible_val {
        Valor::Bool(b) => b,
        Valor::Num(n) => n != 0.0,
        _ => return Valor::Error("trap::set_visible() visible debe ser bool o número".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "trap" {
            return Valor::Error(format!("entity '{}' no es de tipo 'trap'", id));
        }
        entity.set_data("visible", Valor::Bool(visible));
        Valor::Texto(format!("trap::set_visible('{}', {})", id, visible))
    } else {
        Valor::Error(format!("trap::set_visible() La entidad '{}' no existe", id))
    }
}

/// trap::is_triggered(id) - Verificar si está activada
pub fn trap_is_triggered(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("trap::is_triggered() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("trap::is_triggered() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let em_ref = em.borrow();
    
    if let Some(entity) = em_ref.get(&id) {
        if let Some(Valor::Bool(triggered)) = entity.get_data("is_triggered") {
            Valor::Bool(*triggered)
        } else {
            Valor::Bool(false)
        }
    } else {
        Valor::Error(format!("trap::is_triggered() La entidad '{}' no existe", id))
    }
}

/// trap::activate(id) - Activar trampa
pub fn trap_activate(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("trap::activate() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("trap::activate() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "trap" {
            return Valor::Error(format!("entity '{}' no es de tipo 'trap'", id));
        }
        entity.set_data("is_triggered", Valor::Bool(true));
        Valor::Texto(format!("trap::activate() - '{}' activada", id))
    } else {
        Valor::Error(format!("trap::activate() La entidad '{}' no existe", id))
    }
}

/// trap::reset(id) - Resetear trampa
pub fn trap_reset(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("trap::reset() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("trap::reset() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "trap" {
            return Valor::Error(format!("entity '{}' no es de tipo 'trap'", id));
        }
        entity.set_data("is_triggered", Valor::Bool(false));
        Valor::Texto(format!("trap::reset() - '{}' reseteada", id))
    } else {
        Valor::Error(format!("trap::reset() La entidad '{}' no existe", id))
    }
}

// ============================================================================
// COIN COMPONENT (FASE 2C)
// ============================================================================

/// coin::set_value(id, value) - Establecer valor de moneda
pub fn coin_set_value(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("coin::set_value() requiere 2 argumentos: id, value".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let value_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("coin::set_value() id debe ser texto".to_string()),
    };
    
    let value = match value_val {
        Valor::Num(n) => n,
        _ => return Valor::Error("coin::set_value() value debe ser número".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "coin" {
            return Valor::Error(format!("entity '{}' no es de tipo 'coin'", id));
        }
        entity.set_data("value", Valor::Num(value));
        Valor::Texto(format!("coin::set_value('{}', {})", id, value))
    } else {
        Valor::Error(format!("coin::set_value() La entidad '{}' no existe", id))
    }
}

/// coin::set_type(id, type) - Tipo de moneda
pub fn coin_set_type(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("coin::set_type() requiere 2 argumentos: id, type".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let type_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("coin::set_type() id debe ser texto".to_string()),
    };
    
    let coin_type = match type_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("coin::set_type() type debe ser texto".to_string()),
    };
    
    // Validar tipo
    if !["bronze", "silver", "gold", "gem", "diamond"].contains(&coin_type.as_str()) {
        return Valor::Error("coin::set_type() usa: bronze, silver, gold, gem, diamond".to_string());
    }
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "coin" {
            return Valor::Error(format!("entity '{}' no es de tipo 'coin'", id));
        }
        entity.set_data("coin_type", Valor::Texto(coin_type.clone()));
        Valor::Texto(format!("coin::set_type('{}', '{}')", id, coin_type))
    } else {
        Valor::Error(format!("coin::set_type() La entidad '{}' no existe", id))
    }
}

/// coin::get_value(id) - Obtener valor
pub fn coin_get_value(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("coin::get_value() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("coin::get_value() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let em_ref = em.borrow();
    
    if let Some(entity) = em_ref.get(&id) {
        if let Some(Valor::Num(value)) = entity.get_data("value") {
            Valor::Num(*value)
        } else {
            Valor::Num(5.0)  // default
        }
    } else {
        Valor::Error(format!("coin::get_value() La entidad '{}' no existe", id))
    }
}

/// coin::collect(id, player_id) - Recolectar moneda
pub fn coin_collect(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("coin::collect() requiere 2 argumentos: id, player_id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let _player_id_val = evaluar_expr(&args[1], executor, funcs);
    
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("coin::collect() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let mut em_ref = em.borrow_mut();
    
    if let Some(entity) = em_ref.get_mut(&id) {
        if entity.entity_type != "coin" {
            return Valor::Error(format!("entity '{}' no es de tipo 'coin'", id));
        }
        
        let value = match entity.get_data("value") {
            Some(Valor::Num(v)) => *v,
            _ => 5.0,
        };
        
        // Marcar como recolectada
        entity.set_data("is_collected", Valor::Bool(true));
        entity.is_active = false;
        
        Valor::Texto(format!("coin::collect() - '{}' recolectada (+{} coins)", id, value))
    } else {
        Valor::Error(format!("coin::collect() La entidad '{}' no existe", id))
    }
}

/// coin::is_collected(id) - Verificar si fue recolectada
pub fn coin_is_collected(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("coin::is_collected() requiere 1 argumento: id".to_string());
    }
    
    let id_val = evaluar_expr(&args[0], executor, funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("coin::is_collected() id debe ser texto".to_string()),
    };
    
    let em = get_entity_manager();
    let em_ref = em.borrow();
    
    if let Some(entity) = em_ref.get(&id) {
        if let Some(Valor::Bool(collected)) = entity.get_data("is_collected") {
            Valor::Bool(*collected)
        } else {
            Valor::Bool(false)
        }
    } else {
        Valor::Error(format!("coin::is_collected() La entidad '{}' no existe", id))
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
