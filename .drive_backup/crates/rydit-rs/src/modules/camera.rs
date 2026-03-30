// crates/rydit-rs/src/modules/camera.rs
// Módulo de Cámara 2D para RyDit
//
// Funciones:
// - camera::set_position(x, y)
// - camera::get_position() → (x, y)
// - camera::set_zoom(level)
// - camera::get_zoom() → level
// - camera::set_rotation(angle)
// - camera::get_rotation() → angle
// - camera::scroll(dx, dy)
// - camera::scroll_to(x, y)
// - camera::set_bounds(min_x, min_y, max_x, max_y)
// - camera::clear_bounds()
// - camera::follow(target_x, target_y)
// - camera::follow_smooth(target_x, target_y, smooth)
// - camera::set_follow_offset(offset_x, offset_y)
// - camera::world_to_screen(wx, wy) → (sx, sy)
// - camera::screen_to_world(sx, sy) → (wx, wy)
// - camera::reset()

use blast_core::{Executor, Valor};
use lizer::{Expr, Stmt};
use rydit_gfx::camera::get_camera;
use std::collections::HashMap;

use crate::eval::evaluar_expr;

// ============================================================================
// FUNCIONES BÁSICAS
// ============================================================================

/// camera::set_position(x, y) - Establecer posición de la cámara
pub fn camera_set_position(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("camera::set_position() requiere 2 argumentos: x, y".to_string());
    }

    let x_val = evaluar_expr(&args[0], executor, funcs);
    let y_val = evaluar_expr(&args[1], executor, funcs);

    let x = match x_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::set_position() x debe ser número".to_string()),
    };

    let y = match y_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::set_position() y debe ser número".to_string()),
    };

    let cam = get_camera();
    let mut cam_ref = cam.borrow_mut();
    cam_ref.set_position(x, y);

    Valor::Texto(format!("camera::set_position({}, {})", x, y))
}

/// camera::get_position() - Obtener posición actual
pub fn camera_get_position(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let cam = get_camera();
    let cam_ref = cam.borrow();
    let (x, y) = cam_ref.get_position();

    // Retornar como array [x, y]
    Valor::Array(vec![Valor::Num(x as f64), Valor::Num(y as f64)])
}

/// camera::set_zoom(level) - Establecer zoom
pub fn camera_set_zoom(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("camera::set_zoom() requiere 1 argumento: level".to_string());
    }

    let level_val = evaluar_expr(&args[0], executor, funcs);

    let level = match level_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::set_zoom() level debe ser número".to_string()),
    };

    let cam = get_camera();
    let mut cam_ref = cam.borrow_mut();
    cam_ref.set_zoom(level);

    Valor::Texto(format!("camera::set_zoom({})", level))
}

/// camera::get_zoom() - Obtener zoom actual
pub fn camera_get_zoom(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let cam = get_camera();
    let cam_ref = cam.borrow();
    Valor::Num(cam_ref.get_zoom() as f64)
}

/// camera::set_rotation(angle) - Establecer rotación en grados
pub fn camera_set_rotation(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("camera::set_rotation() requiere 1 argumento: angle".to_string());
    }

    let angle_val = evaluar_expr(&args[0], executor, funcs);

    let angle = match angle_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::set_rotation() angle debe ser número".to_string()),
    };

    let cam = get_camera();
    let mut cam_ref = cam.borrow_mut();
    cam_ref.set_rotation(angle);

    Valor::Texto(format!("camera::set_rotation({}°)", angle))
}

/// camera::get_rotation() - Obtener rotación actual
pub fn camera_get_rotation(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let cam = get_camera();
    let cam_ref = cam.borrow();
    Valor::Num(cam_ref.get_rotation() as f64)
}

// ============================================================================
// SCROLL
// ============================================================================

/// camera::scroll(dx, dy) - Mover cámara relativamente
pub fn camera_scroll(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("camera::scroll() requiere 2 argumentos: dx, dy".to_string());
    }

    let dx_val = evaluar_expr(&args[0], executor, funcs);
    let dy_val = evaluar_expr(&args[1], executor, funcs);

    let dx = match dx_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::scroll() dx debe ser número".to_string()),
    };

    let dy = match dy_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::scroll() dy debe ser número".to_string()),
    };

    let cam = get_camera();
    let mut cam_ref = cam.borrow_mut();
    cam_ref.scroll(dx, dy);

    Valor::Texto(format!("camera::scroll({}, {})", dx, dy))
}

/// camera::scroll_to(x, y) - Mover cámara a posición absoluta
pub fn camera_scroll_to(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("camera::scroll_to() requiere 2 argumentos: x, y".to_string());
    }

    let x_val = evaluar_expr(&args[0], executor, funcs);
    let y_val = evaluar_expr(&args[1], executor, funcs);

    let x = match x_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::scroll_to() x debe ser número".to_string()),
    };

    let y = match y_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::scroll_to() y debe ser número".to_string()),
    };

    let cam = get_camera();
    let mut cam_ref = cam.borrow_mut();
    cam_ref.scroll_to(x, y);

    Valor::Texto(format!("camera::scroll_to({}, {})", x, y))
}

/// camera::set_bounds(min_x, min_y, max_x, max_y) - Establecer límites
pub fn camera_set_bounds(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 4 {
        return Valor::Error("camera::set_bounds() requiere 4 argumentos: min_x, min_y, max_x, max_y".to_string());
    }

    let min_x_val = evaluar_expr(&args[0], executor, funcs);
    let min_y_val = evaluar_expr(&args[1], executor, funcs);
    let max_x_val = evaluar_expr(&args[2], executor, funcs);
    let max_y_val = evaluar_expr(&args[3], executor, funcs);

    let min_x = match min_x_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::set_bounds() min_x debe ser número".to_string()),
    };

    let min_y = match min_y_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::set_bounds() min_y debe ser número".to_string()),
    };

    let max_x = match max_x_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::set_bounds() max_x debe ser número".to_string()),
    };

    let max_y = match max_y_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::set_bounds() max_y debe ser número".to_string()),
    };

    let cam = get_camera();
    let mut cam_ref = cam.borrow_mut();
    cam_ref.set_bounds(min_x, min_y, max_x, max_y);

    Valor::Texto(format!("camera::set_bounds({}, {}, {}, {})", min_x, min_y, max_x, max_y))
}

/// camera::clear_bounds() - Limpiar límites
pub fn camera_clear_bounds(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let cam = get_camera();
    let mut cam_ref = cam.borrow_mut();
    cam_ref.clear_bounds();

    Valor::Texto("camera::clear_bounds() - Límites eliminados".to_string())
}

// ============================================================================
// SEGUIMIENTO DEL JUGADOR
// ============================================================================

/// camera::follow(target_x, target_y) - Seguir objetivo instantáneamente
pub fn camera_follow(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("camera::follow() requiere 2 argumentos: target_x, target_y".to_string());
    }

    let tx_val = evaluar_expr(&args[0], executor, funcs);
    let ty_val = evaluar_expr(&args[1], executor, funcs);

    let target_x = match tx_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::follow() target_x debe ser número".to_string()),
    };

    let target_y = match ty_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::follow() target_y debe ser número".to_string()),
    };

    let cam = get_camera();
    let mut cam_ref = cam.borrow_mut();
    cam_ref.follow(target_x, target_y);

    Valor::Texto(format!("camera::follow({}, {})", target_x, target_y))
}

/// camera::follow_smooth(target_x, target_y, smooth) - Seguir con suavizado
pub fn camera_follow_smooth(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 3 {
        return Valor::Error("camera::follow_smooth() requiere 3 argumentos: target_x, target_y, smooth".to_string());
    }

    let tx_val = evaluar_expr(&args[0], executor, funcs);
    let ty_val = evaluar_expr(&args[1], executor, funcs);
    let smooth_val = evaluar_expr(&args[2], executor, funcs);

    let target_x = match tx_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::follow_smooth() target_x debe ser número".to_string()),
    };

    let target_y = match ty_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::follow_smooth() target_y debe ser número".to_string()),
    };

    let smooth = match smooth_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::follow_smooth() smooth debe ser número".to_string()),
    };

    let cam = get_camera();
    let mut cam_ref = cam.borrow_mut();
    cam_ref.follow_smooth(target_x, target_y, smooth);

    Valor::Texto(format!("camera::follow_smooth({}, {}, {})", target_x, target_y, smooth))
}

/// camera::set_follow_offset(offset_x, offset_y) - Offset para seguimiento
pub fn camera_set_follow_offset(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("camera::set_follow_offset() requiere 2 argumentos: offset_x, offset_y".to_string());
    }

    let ox_val = evaluar_expr(&args[0], executor, funcs);
    let oy_val = evaluar_expr(&args[1], executor, funcs);

    let offset_x = match ox_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::set_follow_offset() offset_x debe ser número".to_string()),
    };

    let offset_y = match oy_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::set_follow_offset() offset_y debe ser número".to_string()),
    };

    let cam = get_camera();
    let mut cam_ref = cam.borrow_mut();
    cam_ref.set_follow_offset(offset_x, offset_y);

    Valor::Texto(format!("camera::set_follow_offset({}, {})", offset_x, offset_y))
}

// ============================================================================
// CONVERSIÓN DE COORDENADAS
// ============================================================================

/// camera::world_to_screen(wx, wy) - Convertir mundo a pantalla
pub fn camera_world_to_screen(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("camera::world_to_screen() requiere 2 argumentos: wx, wy".to_string());
    }

    let wx_val = evaluar_expr(&args[0], executor, funcs);
    let wy_val = evaluar_expr(&args[1], executor, funcs);

    let wx = match wx_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::world_to_screen() wx debe ser número".to_string()),
    };

    let wy = match wy_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("camera::world_to_screen() wy debe ser número".to_string()),
    };

    // Usar tamaño de pantalla por defecto (800x600)
    // En el futuro, obtener del estado gráfico
    let screen_width = 800;
    let screen_height = 600;

    let cam = get_camera();
    let cam_ref = cam.borrow();
    let (sx, sy) = cam_ref.world_to_screen(wx, wy, screen_width, screen_height);

    Valor::Array(vec![Valor::Num(sx as f64), Valor::Num(sy as f64)])
}

/// camera::screen_to_world(sx, sy) - Convertir pantalla a mundo
pub fn camera_screen_to_world(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("camera::screen_to_world() requiere 2 argumentos: sx, sy".to_string());
    }

    let sx_val = evaluar_expr(&args[0], executor, funcs);
    let sy_val = evaluar_expr(&args[1], executor, funcs);

    let sx = match sx_val {
        Valor::Num(n) => n as i32,
        _ => return Valor::Error("camera::screen_to_world() sx debe ser número".to_string()),
    };

    let sy = match sy_val {
        Valor::Num(n) => n as i32,
        _ => return Valor::Error("camera::screen_to_world() sy debe ser número".to_string()),
    };

    // Usar tamaño de pantalla por defecto (800x600)
    let screen_width = 800;
    let screen_height = 600;

    let cam = get_camera();
    let cam_ref = cam.borrow();
    let (wx, wy) = cam_ref.screen_to_world(sx, sy, screen_width, screen_height);

    Valor::Array(vec![Valor::Num(wx as f64), Valor::Num(wy as f64)])
}

// ============================================================================
// UTILIDADES
// ============================================================================

/// camera::reset() - Resetear cámara a valores por defecto
pub fn camera_reset(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let cam = get_camera();
    let mut cam_ref = cam.borrow_mut();
    cam_ref.reset();

    Valor::Texto("camera::reset() - Cámara reseteada".to_string())
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
    fn test_camera_set_position() {
        let (mut executor, mut funcs) = setup_test();

        let args = vec![Expr::Num(100.0), Expr::Num(200.0)];
        let result = camera_set_position(&args, &mut executor, &mut funcs);

        if let Valor::Texto(msg) = result {
            assert!(msg.contains("100"));
            assert!(msg.contains("200"));
        } else {
            panic!("camera_set_position debería retornar Texto");
        }
    }

    #[test]
    fn test_camera_set_zoom() {
        let (mut executor, mut funcs) = setup_test();

        let args = vec![Expr::Num(2.0)];
        let result = camera_set_zoom(&args, &mut executor, &mut funcs);

        if let Valor::Texto(msg) = result {
            assert!(msg.contains("2"));
        } else {
            panic!("camera_set_zoom debería retornar Texto");
        }
    }

    #[test]
    fn test_camera_follow() {
        let (mut executor, mut funcs) = setup_test();

        let args = vec![Expr::Num(50.0), Expr::Num(100.0)];
        let result = camera_follow(&args, &mut executor, &mut funcs);

        if let Valor::Texto(msg) = result {
            assert!(msg.contains("50"));
            assert!(msg.contains("100"));
        } else {
            panic!("camera_follow debería retornar Texto");
        }
    }

    #[test]
    fn test_camera_functions_exist() {
        let _ = camera_set_position;
        let _ = camera_get_position;
        let _ = camera_set_zoom;
        let _ = camera_get_zoom;
        let _ = camera_follow;
        let _ = camera_follow_smooth;
        let _ = camera_world_to_screen;
        let _ = camera_screen_to_world;
        let _ = camera_reset;
    }
}
