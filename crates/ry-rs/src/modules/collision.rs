// crates/rydit-rs/src/modules/collision.rs
// Collision System - Sistema de Colisiones 2D para RyDit
#![allow(unused_variables)]
#![allow(unused_mut)]
//
// Funciones:
// - collision::check_rect_rect(...) - Colisión rectángulo vs rectángulo
// - collision::check_circle_circle(...) - Colisión círculo vs círculo
// Variables placeholder se usan en lógica futura
// - collision::check_rect_circle(...) - Colisión rectángulo vs círculo
// - collision::check_point_rect(...) - Punto vs rectángulo
// - collision::check_point_circle(...) - Punto vs círculo
// - area2d::create(id, x, y, w, h) - Crear área 2D
// - area2d::set_position(id, x, y) - Mover área
// - area2d::get_position(id) - Obtener posición
// - area2d::check(id, other_id) - Verificar colisión entre áreas
// - area2d::get_overlapping(id) - Obtener áreas superpuestas
// - area2d::destroy(id) - Eliminar área
// - collision::resolve(...) - Resolver colisión (overlap)
// - collision::bounce(...) - Aplicar rebote

use blast_core::{Executor, Valor};
use ry_parser::{Expr, Stmt};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::eval::evaluar_expr;

// ============================================================================
// AREA2D STRUCT
// ============================================================================

/// Área 2D para detección de colisiones (tipo Godot Area2D)
#[derive(Debug, Clone)]
pub struct Area2D {
    /// ID único del área
    #[allow(dead_code)] // El id se usa para debugging y lookup
    pub id: String,
    /// Posición X
    pub x: f32,
    /// Posición Y
    pub y: f32,
    /// Ancho
    pub width: f32,
    /// Alto
    pub height: f32,
    /// Si está activa (detecta colisiones)
    #[allow(dead_code)] // Para futura activación dinámica
    pub active: bool,
    /// Áreas superpuestas actualmente
    pub overlapping: Vec<String>,
    /// Capa de colisión (para filtrar)
    #[allow(dead_code)] // Para futuras capas de colisión
    pub collision_layer: u32,
    /// Máscara de colisión (qué capas detecta)
    #[allow(dead_code)] // Para futuras máscaras de colisión
    pub collision_mask: u32,
}

impl Area2D {
    /// Crear nueva área 2D
    pub fn new<'a>(id: &str, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            id: id.to_string(),
            x,
            y,
            width,
            height,
            active: true,
            overlapping: Vec::new(),
            collision_layer: 1,
            collision_mask: 1,
        }
    }

    /// Verificar si colisiona con otra área
    pub fn collides_with<'a>(&self, other: &Area2D) -> bool {
        if !self.active || !other.active {
            return false;
        }

        // Verificar capas de colisión
        if self.collision_mask & other.collision_layer == 0 {
            return false;
        }
        if other.collision_mask & self.collision_layer == 0 {
            return false;
        }

        // AABB collision check
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    /// Obtener centro del área
    #[allow(dead_code)] // Para futuras funciones de centro de área
    pub fn get_center<'a>(&self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

// ============================================================================
// ESTADO GLOBAL
// ============================================================================

thread_local! {
    static AREAS: Rc<RefCell<HashMap<String, Area2D>>> = Rc::new(RefCell::new(HashMap::new()));
}

/// Obtener referencia a las áreas
pub fn get_areas<'a>() -> Rc<RefCell<HashMap<String, Area2D>>> {
    AREAS.with(|a| a.clone())
}

// ============================================================================
// FUNCIONES DE COLISIÓN BÁSICAS
// ============================================================================

/// collision::check_rect_rect(x1, y1, w1, h1, x2, y2, w2, h2) - Colisión rect vs rect
pub fn collision_check_rect_rect<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 8 {
        return Valor::Error("collision::check_rect_rect() requiere 8 argumentos".to_string());
    }

    // Evaluar argumentos y validar que sean números
    let mut vals = Vec::with_capacity(8);
    for (i, arg) in args.iter().enumerate() {
        match evaluar_expr(arg, executor, funcs) {
            Valor::Num(n) => vals.push(n as f32),
            other => {
                return Valor::Error(format!(
                    "collision::check_rect_rect() argumento {} debe ser número, se obtuvo: {:?}",
                    i + 1,
                    other
                ))
            }
        }
    }

    let [x1, y1, w1, h1, x2, y2, w2, h2] = vals[..] else {
        return Valor::Error("collision::check_rect_rect() error interno".to_string());
    };

    let collides = x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2;

    Valor::Bool(collides)
}

/// collision::check_circle_circle(x1, y1, r1, x2, y2, r2) - Colisión circle vs circle
pub fn collision_check_circle_circle<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 6 {
        return Valor::Error("collision::check_circle_circle() requiere 6 argumentos".to_string());
    }

    let vals: Vec<f32> = args
        .iter()
        .map(|arg| evaluar_expr(arg, executor, funcs))
        .map(|v| match v {
            Valor::Num(n) => n as f32,
            _ => -1.0,
        })
        .collect();

    let [x1, y1, r1, x2, y2, r2] = vals[..] else {
        return Valor::Error(
            "collision::check_circle_circle() todos los argumentos deben ser números".to_string(),
        );
    };

    let dx = x2 - x1;
    let dy = y2 - y1;
    let distance = (dx * dx + dy * dy).sqrt();
    let collides = distance < r1 + r2;

    Valor::Bool(collides)
}

/// collision::check_rect_circle(rx, ry, rw, rh, cx, cy, cr) - Colisión rect vs circle
pub fn collision_check_rect_circle<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 7 {
        return Valor::Error("collision::check_rect_circle() requiere 7 argumentos".to_string());
    }

    let vals: Vec<f32> = args
        .iter()
        .map(|arg| evaluar_expr(arg, executor, funcs))
        .map(|v| match v {
            Valor::Num(n) => n as f32,
            _ => -1.0,
        })
        .collect();

    let [rx, ry, rw, rh, cx, cy, cr] = vals[..] else {
        return Valor::Error(
            "collision::check_rect_circle() todos los argumentos deben ser números".to_string(),
        );
    };

    // Encontrar punto más cercano en el rectángulo al centro del círculo
    let closest_x = cx.max(rx).min(rx + rw);
    let closest_y = cy.max(ry).min(ry + rh);

    // Calcular distancia desde el punto más cercano al centro del círculo
    let dx = cx - closest_x;
    let dy = cy - closest_y;
    let distance = (dx * dx + dy * dy).sqrt();

    let collides = distance < cr;

    Valor::Bool(collides)
}

/// collision::check_point_rect(px, py, rx, ry, rw, rh) - Punto vs rectángulo
pub fn collision_check_point_rect<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 6 {
        return Valor::Error("collision::check_point_rect() requiere 6 argumentos".to_string());
    }

    let vals: Vec<f32> = args
        .iter()
        .map(|arg| evaluar_expr(arg, executor, funcs))
        .map(|v| match v {
            Valor::Num(n) => n as f32,
            _ => -1.0,
        })
        .collect();

    let [px, py, rx, ry, rw, rh] = vals[..] else {
        return Valor::Error(
            "collision::check_point_rect() todos los argumentos deben ser números".to_string(),
        );
    };

    let collides = px >= rx && px <= rx + rw && py >= ry && py <= ry + rh;

    Valor::Bool(collides)
}

/// collision::check_point_circle(px, py, cx, cy, cr) - Punto vs círculo
pub fn collision_check_point_circle<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 5 {
        return Valor::Error("collision::check_point_circle() requiere 5 argumentos".to_string());
    }

    let vals: Vec<f32> = args
        .iter()
        .map(|arg| evaluar_expr(arg, executor, funcs))
        .map(|v| match v {
            Valor::Num(n) => n as f32,
            _ => -1.0,
        })
        .collect();

    let [px, py, cx, cy, cr] = vals[..] else {
        return Valor::Error(
            "collision::check_point_circle() todos los argumentos deben ser números".to_string(),
        );
    };

    let dx = px - cx;
    let dy = py - cy;
    let distance = (dx * dx + dy * dy).sqrt();
    let collides = distance < cr;

    Valor::Bool(collides)
}

/// collision::resolve(x1, y1, w1, h1, x2, y2, w2, h2) - Obtener overlap de colisión
pub fn collision_resolve<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 8 {
        return Valor::Error("collision::resolve() requiere 8 argumentos".to_string());
    }

    let vals: Vec<f32> = args
        .iter()
        .map(|arg| evaluar_expr(arg, executor, funcs))
        .map(|v| match v {
            Valor::Num(n) => n as f32,
            _ => 0.0,
        })
        .collect();

    let [x1, y1, w1, h1, x2, y2, w2, h2] = vals[..] else {
        return Valor::Error(
            "collision::resolve() todos los argumentos deben ser números".to_string(),
        );
    };

    // Verificar si hay colisión
    if x1 >= x2 + w2 || x1 + w1 <= x2 || y1 >= y2 + h2 || y1 + h1 <= y2 {
        return Valor::Array(vec![Valor::Num(0.0), Valor::Num(0.0)]);
    }

    // Calcular overlap en cada eje
    let overlap_left = (x1 + w1) - x2;
    let overlap_right = (x2 + w2) - x1;
    let overlap_top = (y1 + h1) - y2;
    let overlap_bottom = (y2 + h2) - y1;

    // Obtener el overlap mínimo (dirección de menor penetración)
    let min_x = overlap_left.min(overlap_right);
    let min_y = overlap_top.min(overlap_bottom);

    // Determinar dirección (negativo = izquierda/arriba, positivo = derecha/abajo)
    let overlap_x = if overlap_left < overlap_right {
        -overlap_left
    } else {
        overlap_right
    };
    let overlap_y = if overlap_top < overlap_bottom {
        -overlap_top
    } else {
        overlap_bottom
    };

    // Retornar el overlap en la dirección mínima
    if min_x.abs() < min_y.abs() {
        Valor::Array(vec![Valor::Num(overlap_x as f64), Valor::Num(0.0)])
    } else {
        Valor::Array(vec![Valor::Num(0.0), Valor::Num(overlap_y as f64)])
    }
}

// ============================================================================
// FUNCIONES AREA2D
// ============================================================================

/// area2d::create(id, x, y, w, h) - Crear área 2D
pub fn area2d_create<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 5 {
        return Valor::Error("area2d::create() requiere 5 argumentos: id, x, y, w, h".to_string());
    }

    let id_val = evaluar_expr(&args[0], executor, funcs);
    let vals: Vec<f32> = args[1..5]
        .iter()
        .map(|arg| evaluar_expr(arg, executor, funcs))
        .map(|v| match v {
            Valor::Num(n) => n as f32,
            _ => -1.0,
        })
        .collect();

    let id = match id_val {
        Valor::Texto(i) => i,
        _ => return Valor::Error("area2d::create() id debe ser texto".to_string()),
    };

    let [x, y, w, h] = vals[..] else {
        return Valor::Error("area2d::create() x, y, w, h deben ser números".to_string());
    };

    let areas = get_areas();
    let mut areas_ref = areas.borrow_mut();

    if areas_ref.contains_key(&id) {
        return Valor::Error(format!("area2d::create() el área '{}' ya existe", id));
    }

    let area = Area2D::new(&id, x, y, w, h);
    areas_ref.insert(id.clone(), area);

    Valor::Texto(format!(
        "area2d::create() - Área '{}' creada en ({}, {})",
        id, x, y
    ))
}

/// area2d::set_position(id, x, y) - Mover área
pub fn area2d_set_position<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 3 {
        return Valor::Error("area2d::set_position() requiere 3 argumentos: id, x, y".to_string());
    }

    let id_val = evaluar_expr(&args[0], executor, funcs);
    let x_val = evaluar_expr(&args[1], executor, funcs);
    let y_val = evaluar_expr(&args[2], executor, funcs);

    let id = match id_val {
        Valor::Texto(i) => i,
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

    let areas = get_areas();
    let mut areas_ref = areas.borrow_mut();

    if let Some(area) = areas_ref.get_mut(&id) {
        area.x = x;
        area.y = y;
        Valor::Texto(format!(
            "area2d::set_position() - '{}' movida a ({}, {})",
            id, x, y
        ))
    } else {
        Valor::Error(format!("area2d::set_position() el área '{}' no existe", id))
    }
}

/// area2d::get_position(id) - Obtener posición del área
pub fn area2d_get_position<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("area2d::get_position() requiere 1 argumento: id".to_string());
    }

    let id_val = evaluar_expr(&args[0], executor, funcs);

    let id = match id_val {
        Valor::Texto(i) => i,
        _ => return Valor::Error("area2d::get_position() id debe ser texto".to_string()),
    };

    let areas = get_areas();
    let areas_ref = areas.borrow();

    if let Some(area) = areas_ref.get(&id) {
        Valor::Array(vec![Valor::Num(area.x as f64), Valor::Num(area.y as f64)])
    } else {
        Valor::Error(format!("area2d::get_position() el área '{}' no existe", id))
    }
}

/// area2d::check(id, other_id) - Verificar colisión entre áreas
pub fn area2d_check<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("area2d::check() requiere 2 argumentos: id, other_id".to_string());
    }

    let id_val = evaluar_expr(&args[0], executor, funcs);
    let other_id_val = evaluar_expr(&args[1], executor, funcs);

    let id = match id_val {
        Valor::Texto(i) => i,
        _ => return Valor::Error("area2d::check() id debe ser texto".to_string()),
    };

    let other_id = match other_id_val {
        Valor::Texto(i) => i,
        _ => return Valor::Error("area2d::check() other_id debe ser texto".to_string()),
    };

    let areas = get_areas();
    let areas_ref = areas.borrow();

    let area1 = areas_ref.get(&id);
    let area2 = areas_ref.get(&other_id);

    match (area1, area2) {
        (Some(a1), Some(a2)) => {
            let collides = a1.collides_with(a2);
            Valor::Bool(collides)
        }
        (None, _) => Valor::Error(format!("area2d::check() el área '{}' no existe", id)),
        (_, None) => Valor::Error(format!("area2d::check() el área '{}' no existe", other_id)),
    }
}

/// area2d::get_overlapping(id) - Obtener áreas superpuestas
pub fn area2d_get_overlapping<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("area2d::get_overlapping() requiere 1 argumento: id".to_string());
    }

    let id_val = evaluar_expr(&args[0], executor, funcs);

    let id = match id_val {
        Valor::Texto(i) => i,
        _ => return Valor::Error("area2d::get_overlapping() id debe ser texto".to_string()),
    };

    let areas = get_areas();

    // Primero, obtener IDs de áreas que colisionan
    let mut overlapping_ids: Vec<String> = Vec::new();

    {
        let areas_ref = areas.borrow();
        let my_area = areas_ref.get(&id);

        if my_area.is_none() {
            return Valor::Error(format!(
                "area2d::get_overlapping() el área '{}' no existe",
                id
            ));
        }

        let my_id = id.clone();
        for (other_id, other_area) in areas_ref.iter() {
            if other_id != &my_id && my_area.unwrap().collides_with(other_area) {
                overlapping_ids.push(other_id.clone());
            }
        }
    }

    // Ahora actualizar el área con la lista
    {
        let mut areas_ref = areas.borrow_mut();
        if let Some(area) = areas_ref.get_mut(&id) {
            area.overlapping = overlapping_ids.clone();
        }
    }

    let valores: Vec<Valor> = overlapping_ids
        .iter()
        .map(|s| Valor::Texto(s.clone()))
        .collect();

    Valor::Array(valores)
}

/// area2d::set_active(id, active) - Activar/desactivar área
pub fn area2d_set_active<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("area2d::set_active() requiere 2 argumentos: id, active".to_string());
    }

    let id_val = evaluar_expr(&args[0], executor, funcs);
    let active_val = evaluar_expr(&args[1], executor, funcs);

    let id = match id_val {
        Valor::Texto(i) => i,
        _ => return Valor::Error("area2d::set_active() id debe ser texto".to_string()),
    };

    let active = match active_val {
        Valor::Bool(a) => a,
        Valor::Num(n) => n != 0.0,
        _ => {
            return Valor::Error(
                "area2d::set_active() active debe ser booleano o número".to_string(),
            )
        }
    };

    let areas = get_areas();
    let mut areas_ref = areas.borrow_mut();

    if let Some(area) = areas_ref.get_mut(&id) {
        area.active = active;
        Valor::Texto(format!(
            "area2d::set_active() - '{}' {}",
            id,
            if active { "activada" } else { "desactivada" }
        ))
    } else {
        Valor::Error(format!("area2d::set_active() el área '{}' no existe", id))
    }
}

/// area2d::destroy(id) - Eliminar área
pub fn area2d_destroy<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("area2d::destroy() requiere 1 argumento: id".to_string());
    }

    let id_val = evaluar_expr(&args[0], executor, funcs);

    let id = match id_val {
        Valor::Texto(i) => i,
        _ => return Valor::Error("area2d::destroy() id debe ser texto".to_string()),
    };

    let areas = get_areas();
    let mut areas_ref = areas.borrow_mut();

    if areas_ref.remove(&id).is_some() {
        Valor::Texto(format!("area2d::destroy() - Área '{}' eliminada", id))
    } else {
        Valor::Error(format!("area2d::destroy() el área '{}' no existe", id))
    }
}

/// area2d::count() - Contar áreas
pub fn area2d_count<'a>(
    _args: &[Expr<'a>],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    let areas = get_areas();
    let areas_ref = areas.borrow();
    Valor::Num(areas_ref.len() as f64)
}

// ============================================================================
// ONE-WAY PLATFORMS (v0.13.1) 🟩
// ============================================================================

/// collision::check_one_way(px, py, pw, ph, prev_py, pvy, ox, oy, ow, oh)
/// Verifica colisión one-way (solo al caer sobre la plataforma)
///
/// Lógica:
/// - Colisiona solo si el jugador está cayendo (vy >= 0)
/// - Los pies del jugador estaban ARRIBA de la plataforma en el frame anterior
/// - Los pies del jugador están AHORA dentro o debajo de la plataforma
pub fn collision_check_one_way<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 10 {
        return Valor::Error("collision::check_one_way() requiere 10 args: px, py, pw, ph, prev_py, vy, ox, oy, ow, oh".to_string());
    }

    let vals: Result<Vec<f32>, _> = args.iter().map(|a| {
        match evaluar_expr(a, executor, funcs) {
            Valor::Num(n) => Ok(n as f32),
            _ => Err("collision::check_one_way() todos los args deben ser números".to_string()),
        }
    }).collect();

    let vals = match vals {
        Ok(v) => v,
        Err(e) => return Valor::Error(e),
    };

    let [px, py, pw, ph, prev_py, vy, ox, oy, ow, oh] = vals[..] else {
        return Valor::Error("collision::check_one_way() error interno".to_string());
    };

    // 1. Verificar que el jugador está cayendo (vy >= 0)
    if vy < 0.0 {
        return Valor::Bool(false);
    }

    // 2. Verificar overlap horizontal
    if px + pw <= ox || px >= ox + ow {
        return Valor::Bool(false);
    }

    // 3. Verificar que los pies del jugador estaban ARRIBA de la plataforma antes
    let prev_feet = prev_py + ph;
    if prev_feet > oy + 2.0 {
        // Tolerancia de 2px para evitar flickering
        return Valor::Bool(false);
    }

    // 4. Verificar que los pies del jugador están AHORA dentro o debajo de la plataforma
    let feet = py + ph;
    if feet < oy {
        return Valor::Bool(false);
    }

    Valor::Bool(true)
}

/// collision::resolve_one_way(px, py, ph, oy)
/// Resuelve colisión one-way: coloca al jugador SOBRE la plataforma
/// Retorna [new_px, new_py, grounded]
pub fn collision_resolve_one_way<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 4 {
        return Valor::Error("collision::resolve_one_way() requiere 4 args: px, py, ph, oy".to_string());
    }

    let vals: Result<Vec<f32>, _> = args.iter().map(|a| {
        match evaluar_expr(a, executor, funcs) {
            Valor::Num(n) => Ok(n as f32),
            _ => Err("collision::resolve_one_way() todos los args deben ser números".to_string()),
        }
    }).collect();

    let vals = match vals {
        Ok(v) => v,
        Err(e) => return Valor::Error(e),
    };

    let [px, py, ph, oy] = vals[..] else {
        return Valor::Error("collision::resolve_one_way() error interno".to_string());
    };

    // Colocar al jugador SOBRE la plataforma
    let new_py = oy - ph;
    let grounded = 1.0; // grounded = true

    let result = vec![
        Valor::Num(px as f64),
        Valor::Num(new_py as f64),
        Valor::Num(grounded as f64),
    ];
    Valor::Array(result)
}
