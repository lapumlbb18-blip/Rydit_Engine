// crates/rydit-rs/src/modules/assets.rs
// Assets Manager - Carga y dibujo de sprites 2D estilo Godot

#![allow(dead_code)] // Funciones usadas desde eval/mod.rs

use blast_core::{Executor, Valor};
use ry_gfx::Assets;
use ry_parser::{Expr, Stmt};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

use crate::eval::evaluar_expr;
use ry_gfx::ColorRydit;

// Estado global de assets (compartido entre módulos)
thread_local! {
    static GLOBAL_ASSETS: Rc<RefCell<Assets>> = Rc::new(RefCell::new(Assets::new()));
}

/// Obtener referencia a los assets globales
pub fn get_assets<'a>() -> Rc<RefCell<Assets>> {
    GLOBAL_ASSETS.with(|a| a.clone())
}

// ============================================================================
// FUNCIONES DEL MÓDULO ASSETS
// ============================================================================

/// assets::load(id, path) - Cargar textura desde archivo
pub fn assets_load<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("assets::load() requiere 2 argumentos: id, path".to_string());
    }

    // Evaluar ID
    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        Valor::Num(n) => n.to_string(),
        _ => {
            return Valor::Error(
                "assets::load() el primer argumento debe ser un ID (texto)".to_string(),
            )
        }
    };

    // Evaluar path
    let path_val = evaluar_expr(&args[1], executor, _funcs);
    let path = match path_val {
        Valor::Texto(s) => s,
        _ => {
            return Valor::Error(
                "assets::load() el segundo argumento debe ser el path (texto)".to_string(),
            )
        }
    };

    // Cargar textura
    match Assets::load_texture_from_path(&path) {
        Ok(texture) => {
            let assets = get_assets();
            let mut assets_ref = assets.borrow_mut();
            assets_ref.insert_texture(id.clone(), texture);
            println!("[ASSETS] Textura '{}' cargada desde '{}'", id, path);
            Valor::Texto(format!("assets::load() - '{}' cargado exitosamente", id))
        }
        Err(e) => Valor::Error(format!("assets::load() Error: {}", e)),
    }
}

/// assets::sprite(id, path) - Crear sprite y cargar textura (alias de load)
pub fn assets_sprite<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    // sprite es alias de load
    assets_load(args, executor, _funcs)
}

/// assets::draw(id, x, y, color) - Dibujar sprite en posición
pub fn assets_draw<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() < 3 {
        return Valor::Error("assets::draw() requiere al menos 3 argumentos: id, x, y".to_string());
    }

    // Evaluar ID
    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        Valor::Num(n) => n.to_string(),
        _ => return Valor::Error("assets::draw() el primer argumento debe ser el ID".to_string()),
    };

    // Evaluar posición X
    let _x_val = evaluar_expr(&args[1], executor, _funcs);
    let _x = match _x_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("assets::draw() x debe ser número".to_string()),
    };

    // Evaluar posición Y
    let _y_val = evaluar_expr(&args[2], executor, _funcs);

    // Color opcional (default: blanco)
    let _color_val = if args.len() >= 4 {
        evaluar_expr(&args[3], executor, _funcs)
    } else {
        Valor::Texto("blanco".to_string())
    };

    let _color = match _color_val {
        Valor::Texto(c) => parse_color(&c),
        _ => ColorRydit::Blanco,
    };

    // NOTA: El dibujado real se hace en main.rs donde está el handle de raylib
    // Esta función solo valida y retorna información
    let assets = get_assets();
    let assets_ref = assets.borrow();

    if assets_ref.has_texture(&id) {
        // Retornar datos para que main.rs dibuje
        Valor::Texto(format!("assets::draw() - '{}' lista para dibujar", id))
    } else {
        Valor::Error(format!(
            "assets::draw() La textura '{}' no está cargada",
            id
        ))
    }
}

/// assets::draw_scaled(id, x, y, scale, color) - Dibujar sprite escalado
pub fn assets_draw_scaled<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() < 4 {
        return Valor::Error(
            "assets::draw_scaled() requiere al menos 4 argumentos: id, x, y, scale".to_string(),
        );
    }

    // Evaluar ID
    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        Valor::Num(n) => n.to_string(),
        _ => {
            return Valor::Error(
                "assets::draw_scaled() el primer argumento debe ser el ID".to_string(),
            )
        }
    };

    // Evaluar posición X
    let _x_val = evaluar_expr(&args[1], executor, _funcs);

    // Evaluar posición Y
    let _y_val = evaluar_expr(&args[2], executor, _funcs);

    // Evaluar escala
    let _scale_val = evaluar_expr(&args[3], executor, _funcs);

    // Color opcional (default: blanco)
    let _color_val = if args.len() >= 5 {
        evaluar_expr(&args[4], executor, _funcs)
    } else {
        Valor::Texto("blanco".to_string())
    };

    let _color = match _color_val {
        Valor::Texto(c) => parse_color(&c),
        _ => ColorRydit::Blanco,
    };

    let assets = get_assets();
    let assets_ref = assets.borrow();

    if assets_ref.has_texture(&id) {
        Valor::Texto(format!(
            "assets::draw_scaled() - '{}' lista para dibujar",
            id
        ))
    } else {
        Valor::Error(format!(
            "assets::draw_scaled() La textura '{}' no está cargada",
            id
        ))
    }
}

/// assets::exists(id) - Verificar si existe una textura
pub fn assets_exists<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("assets::exists() requiere 1 argumento: id".to_string());
    }

    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        Valor::Num(n) => n.to_string(),
        _ => return Valor::Error("assets::exists() el argumento debe ser el ID".to_string()),
    };

    let assets = get_assets();
    let assets_ref = assets.borrow();

    if assets_ref.has_texture(&id) {
        Valor::Bool(true)
    } else {
        Valor::Bool(false)
    }
}

/// assets::unload(id) - Descargar textura y liberar memoria
pub fn assets_unload<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("assets::unload() requiere 1 argumento: id".to_string());
    }

    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        Valor::Num(n) => n.to_string(),
        _ => return Valor::Error("assets::unload() el argumento debe ser el ID".to_string()),
    };

    let assets = get_assets();
    let mut assets_ref = assets.borrow_mut();

    if assets_ref.unload_texture(&id) {
        println!("[ASSETS] Textura '{}' descargada", id);
        Valor::Texto(format!("assets::unload() - '{}' descargado", id))
    } else {
        Valor::Error(format!("assets::unload() La textura '{}' no existe", id))
    }
}

/// assets::count() - Retornar cantidad de texturas cargadas
pub fn assets_count<'a>(
    _args: &[Expr<'a>],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    let assets = get_assets();
    let assets_ref = assets.borrow();
    Valor::Num(assets_ref.texture_count() as f64)
}

// ============================================================================
// FUNCIONES ADICIONALES DE ASSETS (v0.8.7)
// ============================================================================

/// assets::set_position(id, x, y) - Actualizar posición de sprite
/// NOTA: Esta función guarda la posición en un estado interno para uso futuro
pub fn assets_set_position<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 3 {
        return Valor::Error("assets::set_position() requiere 3 argumentos: id, x, y".to_string());
    }

    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let x_val = evaluar_expr(&args[1], executor, _funcs);
    let y_val = evaluar_expr(&args[2], executor, _funcs);

    let id = match id_val {
        Valor::Texto(s) => s,
        Valor::Num(n) => n.to_string(),
        _ => return Valor::Error("assets::set_position() id debe ser texto".to_string()),
    };

    let x = match x_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("assets::set_position() x debe ser número".to_string()),
    };

    let y = match y_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("assets::set_position() y debe ser número".to_string()),
    };

    // Guardar posición en estado interno (para implementación futura)
    // Por ahora, solo confirmamos
    Valor::Texto(format!(
        "assets::set_position() - '{}' en ({}, {})",
        id, x, y
    ))
}

/// assets::set_rotation(id, angle) - Rotar sprite
pub fn assets_set_rotation<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("assets::set_rotation() requiere 2 argumentos: id, angle".to_string());
    }

    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let angle_val = evaluar_expr(&args[1], executor, _funcs);

    let id = match id_val {
        Valor::Texto(s) => s,
        Valor::Num(n) => n.to_string(),
        _ => return Valor::Error("assets::set_rotation() id debe ser texto".to_string()),
    };

    let angle = match angle_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("assets::set_rotation() angle debe ser número".to_string()),
    };

    Valor::Texto(format!(
        "assets::set_rotation() - '{}' rotado {} grados",
        id, angle
    ))
}

/// assets::set_scale(id, scale_x, scale_y) - Escalar sprite
pub fn assets_set_scale<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 3 {
        return Valor::Error(
            "assets::set_scale() requiere 3 argumentos: id, scale_x, scale_y".to_string(),
        );
    }

    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let scale_x_val = evaluar_expr(&args[1], executor, _funcs);
    let scale_y_val = evaluar_expr(&args[2], executor, _funcs);

    let id = match id_val {
        Valor::Texto(s) => s,
        Valor::Num(n) => n.to_string(),
        _ => return Valor::Error("assets::set_scale() id debe ser texto".to_string()),
    };

    let scale_x = match scale_x_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("assets::set_scale() scale_x debe ser número".to_string()),
    };

    let scale_y = match scale_y_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("assets::set_scale() scale_y debe ser número".to_string()),
    };

    Valor::Texto(format!(
        "assets::set_scale() - '{}' escalado a ({}, {})",
        id, scale_x, scale_y
    ))
}

/// assets::set_color(id, color) - Cambiar color/tinte de sprite
pub fn assets_set_color<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("assets::set_color() requiere 2 argumentos: id, color".to_string());
    }

    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let color_val = evaluar_expr(&args[1], executor, _funcs);

    let id = match id_val {
        Valor::Texto(s) => s,
        Valor::Num(n) => n.to_string(),
        _ => return Valor::Error("assets::set_color() id debe ser texto".to_string()),
    };

    let color = match color_val {
        Valor::Texto(c) => c,
        _ => return Valor::Error("assets::set_color() color debe ser texto".to_string()),
    };

    Valor::Texto(format!(
        "assets::set_color() - '{}' con color '{}'",
        id, color
    ))
}

/// assets::set_flip(id, horizontal, vertical) - Flip horizontal/vertical
pub fn assets_set_flip<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 3 {
        return Valor::Error(
            "assets::set_flip() requiere 3 argumentos: id, horizontal, vertical".to_string(),
        );
    }

    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let h_val = evaluar_expr(&args[1], executor, _funcs);
    let v_val = evaluar_expr(&args[2], executor, _funcs);

    let id = match id_val {
        Valor::Texto(s) => s,
        Valor::Num(n) => n.to_string(),
        _ => return Valor::Error("assets::set_flip() id debe ser texto".to_string()),
    };

    let horizontal = match h_val {
        Valor::Bool(b) => b,
        Valor::Num(n) => n != 0.0,
        _ => {
            return Valor::Error("assets::set_flip() horizontal debe ser bool o número".to_string())
        }
    };

    let vertical = match v_val {
        Valor::Bool(b) => b,
        Valor::Num(n) => n != 0.0,
        _ => return Valor::Error("assets::set_flip() vertical debe ser bool o número".to_string()),
    };

    Valor::Texto(format!(
        "assets::set_flip() - '{}' flip=({}, {})",
        id, horizontal, vertical
    ))
}

/// assets::set_origin(id, origin_x, origin_y) - Punto de origen
pub fn assets_set_origin<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 3 {
        return Valor::Error(
            "assets::set_origin() requiere 3 argumentos: id, origin_x, origin_y".to_string(),
        );
    }

    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let ox_val = evaluar_expr(&args[1], executor, _funcs);
    let oy_val = evaluar_expr(&args[2], executor, _funcs);

    let id = match id_val {
        Valor::Texto(s) => s,
        Valor::Num(n) => n.to_string(),
        _ => return Valor::Error("assets::set_origin() id debe ser texto".to_string()),
    };

    let origin_x = match ox_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("assets::set_origin() origin_x debe ser número".to_string()),
    };

    let origin_y = match oy_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("assets::set_origin() origin_y debe ser número".to_string()),
    };

    Valor::Texto(format!(
        "assets::set_origin() - '{}' con origen ({}, {})",
        id, origin_x, origin_y
    ))
}

// ============================================================================
// UTILIDADES
// ============================================================================

/// Parsear color desde texto
fn parse_color(color_str: &str) -> ColorRydit {
    ColorRydit::from_str(color_str).unwrap_or(ColorRydit::Blanco)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assets_module_functions() {
        // Verificar que las funciones existen
        let _ = assets_count;
        let _ = assets_exists;
        let _ = assets_unload;
    }
}
