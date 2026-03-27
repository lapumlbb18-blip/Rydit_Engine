// crates/rydit-rs/src/modules/assets.rs
// Assets Manager - Carga y dibujo de sprites 2D estilo Godot

#![allow(dead_code)] // Funciones usadas desde eval/mod.rs

use blast_core::{Executor, Valor};
use lizer::{Expr, Stmt};
use rydit_gfx::Assets;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

use crate::eval::evaluar_expr;
use rydit_gfx::ColorRydit;

// Estado global de assets (compartido entre módulos)
thread_local! {
    static GLOBAL_ASSETS: Rc<RefCell<Assets>> = Rc::new(RefCell::new(Assets::new()));
}

/// Obtener referencia a los assets globales
pub fn get_assets() -> Rc<RefCell<Assets>> {
    GLOBAL_ASSETS.with(|a| a.clone())
}

// ============================================================================
// FUNCIONES DEL MÓDULO ASSETS
// ============================================================================

/// assets::load(id, path) - Cargar textura desde archivo
pub fn assets_load(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("assets::load() requiere 2 argumentos: id, path".to_string());
    }

    // Evaluar ID
    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        Valor::Num(n) => n.to_string(),
        _ => return Valor::Error("assets::load() el primer argumento debe ser un ID (texto)".to_string()),
    };

    // Evaluar path
    let path_val = evaluar_expr(&args[1], executor, _funcs);
    let path = match path_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("assets::load() el segundo argumento debe ser el path (texto)".to_string()),
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
pub fn assets_sprite(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    // sprite es alias de load
    assets_load(args, executor, _funcs)
}

/// assets::draw(id, x, y, color) - Dibujar sprite en posición
pub fn assets_draw(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
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
        Valor::Error(format!("assets::draw() La textura '{}' no está cargada", id))
    }
}

/// assets::draw_scaled(id, x, y, scale, color) - Dibujar sprite escalado
pub fn assets_draw_scaled(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() < 4 {
        return Valor::Error("assets::draw_scaled() requiere al menos 4 argumentos: id, x, y, scale".to_string());
    }

    // Evaluar ID
    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        Valor::Num(n) => n.to_string(),
        _ => return Valor::Error("assets::draw_scaled() el primer argumento debe ser el ID".to_string()),
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
        Valor::Texto(format!("assets::draw_scaled() - '{}' lista para dibujar", id))
    } else {
        Valor::Error(format!("assets::draw_scaled() La textura '{}' no está cargada", id))
    }
}

/// assets::exists(id) - Verificar si existe una textura
pub fn assets_exists(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
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
pub fn assets_unload(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
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
pub fn assets_count(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let assets = get_assets();
    let assets_ref = assets.borrow();
    Valor::Num(assets_ref.texture_count() as f64)
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
