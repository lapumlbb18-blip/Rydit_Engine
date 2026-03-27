// RYDIT-RS v0.7.3.4 - LAZOS EDITION con RyditModule trait
// Main.rs dividido en módulos para mejor mantenibilidad

// Módulos locales
mod bindings;
mod cli;
mod config;
mod eval;
mod executor;
mod json_helpers;
mod lazos; // ← PROTOCOLO LAZOS
mod module;
mod module_loader; // ← CARGA DINÁMICA v0.8.2
// mod physics;  ← AHORA ES CRATE EXTERNO: use rydit_physics::PhysicsModule;
mod repl;
// mod science;  ← AHORA ES CRATE EXTERNO: use rydit_science::ScienceModule;
mod tests;

// Re-exportar funciones del módulo eval
pub use eval::evaluar_expr;

// Re-exportar helpers de config y json
pub use config::{cargar_modulo, configurar_entorno_termux};
pub use json_helpers::{valor_rydit_a_serde, valor_serde_a_rydit};

// Re-exportar ejecutores
pub use executor::{ejecutar_programa, ejecutar_programa_gfx, ejecutar_programa_migui};

// Re-exportar módulo system (blast-core integration)
pub use module::{
    MathModule, ModuleContext, ModuleRegistry as BlastModuleRegistry,
    RyditModule as BlastRyditModule,
};

// Re-exportar módulos con trait rydit-core (crates externos)
pub use rydit_anim::AnimModule;
pub use rydit_core::{ModuleError, ModuleRegistry, ModuleResult, RyditModule};
pub use rydit_loader::DynamicModuleLoader;
pub use rydit_physics::PhysicsModule;
pub use rydit_science::ScienceModule;

// Imports necesarios para el código restante en main.rs
use blast_core::{Executor, Valor};
use lizer::{Expr, Lizer, Parser, Stmt};
use migui::{Color as MiguiColor, Migui, Rect, WidgetId};
use rydit_gfx::{ColorRydit, Key, RyditGfx};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Mutex;

// Loader global para módulos dinámicos (v0.8.2)
static mut GLOBAL_LOADER: Option<Mutex<DynamicModuleLoader>> = None;

/// Inicializar el loader global
pub fn init_global_loader() {
    unsafe {
        GLOBAL_LOADER = Some(Mutex::new(DynamicModuleLoader::new()));
    }
}

/// Obtener referencia al loader global
pub fn get_loader() -> Option<&'static Mutex<DynamicModuleLoader>> {
    unsafe { GLOBAL_LOADER.as_ref() }
}

fn main() {
    cli::run();
}
// EJECUTOR DE STATEMENTS (pública para módulos)

/// Ejecutar un statement (pública para módulos)
pub fn ejecutar_stmt(
    stmt: &Stmt,
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
    loaded_modules: &mut HashSet<String>,
    importing_stack: &mut Vec<String>,
) -> (Option<bool>, Option<Valor>) {
    match stmt {
        Stmt::Init => {
            println!("[SHIELD] Inicializando sistema...");
        }
        Stmt::Command(cmd) => {
            executor.ejecutar(cmd);
        }
        Stmt::Assign { name, value } => {
            let valor = evaluar_expr(value, executor, funcs);
            executor.guardar(name, valor);
        }
        Stmt::IndexAssign {
            array,
            index,
            value,
        } => {
            // Asignación a índice de array: arr[index] = value
            let index_val = evaluar_expr(index, executor, funcs);
            let valor = evaluar_expr(value, executor, funcs);

            // Obtener el array actual
            if let Some(Valor::Array(arr)) = executor.leer(array) {
                // Calcular índice
                let idx = match index_val {
                    Valor::Num(n) => n as usize,
                    _ => {
                        println!("[ERROR] Índice debe ser número");
                        return (None, None);
                    }
                };

                // Verificar límites
                if idx >= arr.len() {
                    println!(
                        "[ERROR] Índice {} fuera de límites (array de {} elementos)",
                        idx,
                        arr.len()
                    );
                    return (None, None);
                }

                // Modificar el array
                let mut nuevo_arr = arr.clone();
                nuevo_arr[idx] = valor;
                executor.guardar(array, Valor::Array(nuevo_arr));
            } else {
                println!("[ERROR] '{}' no es un array", array);
                return (None, None);
            }
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let cond_val = evaluar_expr(condition, executor, funcs);

            let es_verdad = match cond_val {
                Valor::Num(n) => n != 0.0,
                Valor::Bool(b) => b,
                _ => false,
            };

            if es_verdad {
                for s in then_body {
                    match ejecutar_stmt(s, executor, funcs, loaded_modules, importing_stack) {
                        (Some(true), _) => return (Some(true), None), // Propagar break
                        (_, Some(val)) => return (None, Some(val)),   // Propagar return
                        _ => {}
                    }
                }
            } else if let Some(else_body) = else_body {
                for s in else_body {
                    match ejecutar_stmt(s, executor, funcs, loaded_modules, importing_stack) {
                        (Some(true), _) => return (Some(true), None),
                        (_, Some(val)) => return (None, Some(val)),
                        _ => {}
                    }
                }
            }
        }
        Stmt::While { condition, body } => {
            // Ejecutar mientras la condición sea verdadera
            let mut iterations = 0;
            while iterations < 100 {
                // Límite de seguridad
                let cond_val = evaluar_expr(condition, executor, funcs);
                let es_verdad = match cond_val {
                    Valor::Num(n) => n != 0.0,
                    Valor::Bool(b) => b,
                    _ => false,
                };

                if !es_verdad {
                    break;
                }

                for s in body {
                    match ejecutar_stmt(s, executor, funcs, loaded_modules, importing_stack) {
                        (Some(true), _) => return (Some(true), None), // Break detectado
                        (_, Some(val)) => return (None, Some(val)),   // Return detectado
                        _ => {}
                    }
                }
                iterations += 1;
            }
        }
        Stmt::ForEach {
            var,
            iterable,
            body,
        } => {
            // cada x en lista { ... }
            let iterable_val = evaluar_expr(iterable, executor, funcs);

            if let Valor::Array(arr) = iterable_val {
                // Iterar sobre cada elemento del array
                for item in arr {
                    // Guardar variable del iterador en memoria
                    executor.guardar(var, item.clone());

                    // Ejecutar cuerpo del loop
                    for s in body {
                        match ejecutar_stmt(s, executor, funcs, loaded_modules, importing_stack) {
                            (Some(true), _) => return (Some(true), None), // Break detectado
                            (_, Some(val)) => return (None, Some(val)),   // Return detectado
                            _ => {}
                        }
                    }
                }
            } else {
                println!(
                    "[ERROR] 'cada' requiere un array, se obtuvo: {:?}",
                    iterable_val
                );
            }
        }
        Stmt::Block(stmts) => {
            // Ejecutar todos los statements del bloque
            for s in stmts {
                match ejecutar_stmt(s, executor, funcs, loaded_modules, importing_stack) {
                    (Some(true), _) => return (Some(true), None), // Propagar break
                    (_, Some(val)) => return (None, Some(val)),   // Propagar return
                    _ => {}
                }
            }
        }
        Stmt::Function { name, params, body } => {
            // Guardar función en el registro
            println!("[FUNC] {}({:?}) definida", name, params);
            funcs.insert(name.clone(), (params.clone(), body.clone()));
        }
        Stmt::Call { name, args } => {
            // Llamar función builtin o de usuario
            // Primero verificar funciones builtin
            if name == "sumar" || name == "restar" || name == "multiplicar" || name == "dividir" {
                // Funciones builtin ya manejadas en evaluar_expr
                println!(
                    "[WARNING] Función builtin '{}' debe usarse en expresiones",
                    name
                );
            } else {
                // Función de usuario - clonar datos para evitar borrow checker issues
                let func_data = funcs.get(name).map(|(p, b)| (p.clone(), b.clone()));

                if let Some((params, body)) = func_data {
                    // Función de usuario
                    // Evaluar argumentos
                    let mut arg_values = vec![];
                    for arg in args {
                        arg_values.push(evaluar_expr(arg, executor, funcs));
                    }

                    // Crear scope local para la función
                    executor.push_scope();

                    // Mapear parámetros → valores de argumentos en scope local
                    for (i, param) in params.iter().enumerate() {
                        if i < arg_values.len() {
                            executor.guardar_local(param, arg_values[i].clone());
                        }
                    }

                    // Ejecutar body de la función y capturar retorno
                    let mut return_value: Option<Valor> = None;
                    for s in &body {
                        match ejecutar_stmt(s, executor, funcs, loaded_modules, importing_stack) {
                            (Some(true), _) => {
                                executor.pop_scope(); // Limpiar scope antes de salir
                                return (Some(true), None);
                            }
                            (_, Some(val)) => {
                                return_value = Some(val);
                                break; // Salir del loop, hay retorno
                            }
                            _ => {}
                        }
                    }

                    // Pop scope al finalizar la función
                    executor.pop_scope();

                    // Si hubo retorno, propagarlo
                    if let Some(val) = return_value {
                        return (None, Some(val));
                    }
                } else {
                    println!("[ERROR] Función '{}' no definida", name);
                }
            }
        }
        Stmt::Return(expr) => {
            // Return: retornamos el valor de la expresión
            if let Some(e) = expr {
                let val = evaluar_expr(e, executor, funcs);
                return (None, Some(val)); // Retornar valor al llamador
            } else {
                return (None, Some(Valor::Vacio)); // Return sin valor
            }
        }
        Stmt::Expr(expr) => {
            let val = evaluar_expr(expr, executor, funcs);
            executor.voz(&val); // Usar voz en vez de println
        }
        Stmt::Break => {
            return (Some(true), None); // Señal de break
        }
        Stmt::Import { module, alias } => {
            // Importar módulo: import <modulo> [as <alias>]
            // Cargar desde archivo local o embebido

            // DEUDA #2 FIX: Detectar import cíclico
            if importing_stack.contains(module) {
                println!("[ERROR] Importe cíclico detectado: '{}'", module);
                println!(
                    "[ERROR] Stack de imports: {} -> {}",
                    importing_stack.join(" -> "),
                    module
                );
                return (None, None);
            }

            // DEUDA #1 FIX: Verificar si ya está cargado (evitar re-ejecución)
            if loaded_modules.contains(module.as_str()) {
                println!("[IMPORT] Módulo '{}' ya cargado (usando cache)", module);
                // Solo renombrar funciones existentes
                let prefix = if let Some(alias_name) = alias {
                    alias_name.clone()
                } else {
                    module.clone()
                };

                // Copiar funciones con nuevo nombre desde el cache
                let mut funcs_to_copy: Vec<(String, String)> = Vec::new();
                for (func_name, _) in funcs.iter() {
                    if func_name.starts_with(&format!("{}::", module)) {
                        let orig_name = func_name.strip_prefix(&format!("{}::", module)).unwrap();
                        let new_name = format!("{}::{}", prefix, orig_name);
                        funcs_to_copy.push((func_name.clone(), new_name));
                    }
                }

                for (old_name, new_name) in funcs_to_copy {
                    if let Some(func_data) = funcs.get(&old_name) {
                        funcs.insert(new_name, func_data.clone());
                    }
                }

                if let Some(alias_name) = alias {
                    println!(
                        "[IMPORT] Módulo '{}' disponible como '{}'",
                        module, alias_name
                    );
                }
                return (None, None);
            }

            // Cargar módulo (archivo local o embebido)
            let module_content = match cargar_modulo(module) {
                Ok(content) => {
                    println!("[IMPORT] Módulo '{}' cargado", module);
                    content
                }
                Err(e) => {
                    println!("[ERROR] {}", e);
                    return (None, None);
                }
            };

            // Agregar al stack de imports en progreso
            importing_stack.push(module.clone());

            // Lexer + Parser
            let tokens = Lizer::new(&module_content).scan();
            let mut parser = Parser::new(tokens);

            let program = match parser.parse() {
                Ok(p) => p,
                Err(e) => {
                    println!("[ERROR] Error parseando módulo '{}': {}", module, e);
                    importing_stack.pop();
                    return (None, None);
                }
            };

            // Recolectar nombres de funciones originales antes de ejecutar
            let mut original_funcs: Vec<String> = Vec::new();
            for s in &program.statements {
                if let Stmt::Function { name, .. } = s {
                    original_funcs.push(name.clone());
                }
            }

            // Ejecutar módulo en scope global
            for s in &program.statements {
                match ejecutar_stmt(s, executor, funcs, loaded_modules, importing_stack) {
                    (Some(true), _) => {
                        println!("[ERROR] break no permitido en módulo '{}'", module);
                        break;
                    }
                    (_, Some(_)) => {
                        println!("[ERROR] return no permitido en módulo '{}'", module);
                        break;
                    }
                    _ => {}
                }
            }

            // Remover del stack de imports en progreso
            importing_stack.pop();

            // Marcar módulo como cargado
            loaded_modules.insert(module.clone());

            // Renombrar funciones con el prefijo del módulo
            let prefix = if let Some(alias_name) = alias {
                alias_name.clone()
            } else {
                module.clone()
            };

            // Copiar funciones con nuevo nombre
            for orig_name in &original_funcs {
                if let Some(func_data) = funcs.get(orig_name) {
                    let new_name = format!("{}::{}", prefix, orig_name);
                    funcs.insert(new_name, func_data.clone());
                }
            }

            // DEUDA #3 FIX: Eliminar funciones originales SOLO si no hay alias
            // Si el usuario usó "import math", eliminar "sumar" y dejar solo "math::sumar"
            // Si el usuario usó "import math as m", dejar "math::sumar" y "m::sumar"
            if alias.is_none() {
                // Sin alias: eliminar funciones originales
                for orig_name in &original_funcs {
                    funcs.remove(orig_name);
                }
            }
            // Con alias: las funciones originales se mantienen como module::func

            // Registrar alias si existe
            if let Some(alias_name) = alias {
                println!(
                    "[IMPORT] Módulo '{}' disponible como '{}'",
                    module, alias_name
                );
            } else {
                println!("[IMPORT] Módulo '{}' disponible", module);
            }
        }
        Stmt::DrawCircle { x, y, radio, color } => {
            let x_val = evaluar_expr(x, executor, funcs);
            let y_val = evaluar_expr(y, executor, funcs);
            let radio_val = evaluar_expr(radio, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (Valor::Num(x), Valor::Num(y), Valor::Num(radio)) = (x_val, y_val, radio_val) {
                println!("[DRAW] circle({}, {}, {}, {:?})", x, y, radio, color_val);
            } else {
                println!("[ERROR] draw.circle requiere números");
            }
        }
        Stmt::DrawRect {
            x,
            y,
            ancho,
            alto,
            color,
        } => {
            let x_val = evaluar_expr(x, executor, funcs);
            let y_val = evaluar_expr(y, executor, funcs);
            let ancho_val = evaluar_expr(ancho, executor, funcs);
            let alto_val = evaluar_expr(alto, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (Valor::Num(x), Valor::Num(y), Valor::Num(ancho), Valor::Num(alto)) =
                (x_val, y_val, ancho_val, alto_val)
            {
                println!(
                    "[DRAW] rect({}, {}, {}, {}, {:?})",
                    x, y, ancho, alto, color_val
                );
            } else {
                println!("[ERROR] draw.rect requiere números");
            }
        }
        Stmt::DrawLine {
            x1,
            y1,
            x2,
            y2,
            color,
        } => {
            let x1_val = evaluar_expr(x1, executor, funcs);
            let y1_val = evaluar_expr(y1, executor, funcs);
            let x2_val = evaluar_expr(x2, executor, funcs);
            let y2_val = evaluar_expr(y2, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (Valor::Num(x1), Valor::Num(y1), Valor::Num(x2), Valor::Num(y2)) =
                (x1_val, y1_val, x2_val, y2_val)
            {
                println!(
                    "[DRAW] line({}, {}, {}, {}, {:?})",
                    x1, y1, x2, y2, color_val
                );
            } else {
                println!("[ERROR] draw.line requiere números");
            }
        }
        Stmt::DrawText {
            texto,
            x,
            y,
            tamano,
            color,
        } => {
            let x_val = evaluar_expr(x, executor, funcs);
            let y_val = evaluar_expr(y, executor, funcs);
            let tamano_val = evaluar_expr(tamano, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (Valor::Num(x), Valor::Num(y), Valor::Num(tamano)) = (x_val, y_val, tamano_val) {
                println!(
                    "[DRAW] text('{}', {}, {}, {}, {:?})",
                    texto, x, y, tamano, color_val
                );
            } else {
                println!("[ERROR] draw.text requiere números");
            }
        }
        // Statements v0.2.0 - Nuevas formas
        Stmt::DrawTriangle {
            v1_x,
            v1_y,
            v2_x,
            v2_y,
            v3_x,
            v3_y,
            color,
        } => {
            let v1_x_val = evaluar_expr(v1_x, executor, funcs);
            let v1_y_val = evaluar_expr(v1_y, executor, funcs);
            let v2_x_val = evaluar_expr(v2_x, executor, funcs);
            let v2_y_val = evaluar_expr(v2_y, executor, funcs);
            let v3_x_val = evaluar_expr(v3_x, executor, funcs);
            let v3_y_val = evaluar_expr(v3_y, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (
                Valor::Num(v1_x),
                Valor::Num(v1_y),
                Valor::Num(v2_x),
                Valor::Num(v2_y),
                Valor::Num(v3_x),
                Valor::Num(v3_y),
            ) = (v1_x_val, v1_y_val, v2_x_val, v2_y_val, v3_x_val, v3_y_val)
            {
                println!(
                    "[DRAW] triangle({}, {}, {}, {}, {}, {}, {:?})",
                    v1_x, v1_y, v2_x, v2_y, v3_x, v3_y, color_val
                );
            } else {
                println!("[ERROR] draw.triangle requiere números");
            }
        }
        Stmt::DrawRing {
            center_x,
            center_y,
            inner_radius,
            outer_radius,
            color,
        } => {
            let center_x_val = evaluar_expr(center_x, executor, funcs);
            let center_y_val = evaluar_expr(center_y, executor, funcs);
            let inner_radius_val = evaluar_expr(inner_radius, executor, funcs);
            let outer_radius_val = evaluar_expr(outer_radius, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (Valor::Num(cx), Valor::Num(cy), Valor::Num(ir), Valor::Num(or)) = (
                center_x_val,
                center_y_val,
                inner_radius_val,
                outer_radius_val,
            ) {
                println!(
                    "[DRAW] ring({}, {}, {}, {}, {:?})",
                    cx, cy, ir, or, color_val
                );
            } else {
                println!("[ERROR] draw.ring requiere números");
            }
        }
        Stmt::DrawRectangleLines {
            x,
            y,
            ancho,
            alto,
            color,
        } => {
            let x_val = evaluar_expr(x, executor, funcs);
            let y_val = evaluar_expr(y, executor, funcs);
            let ancho_val = evaluar_expr(ancho, executor, funcs);
            let alto_val = evaluar_expr(alto, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (Valor::Num(x), Valor::Num(y), Valor::Num(ancho), Valor::Num(alto)) =
                (x_val, y_val, ancho_val, alto_val)
            {
                println!(
                    "[DRAW] rectangle_lines({}, {}, {}, {}, {:?})",
                    x, y, ancho, alto, color_val
                );
            } else {
                println!("[ERROR] draw.rectangle_lines requiere números");
            }
        }
        Stmt::DrawEllipse {
            center_x,
            center_y,
            radius_h,
            radius_v,
            color,
        } => {
            let center_x_val = evaluar_expr(center_x, executor, funcs);
            let center_y_val = evaluar_expr(center_y, executor, funcs);
            let radius_h_val = evaluar_expr(radius_h, executor, funcs);
            let radius_v_val = evaluar_expr(radius_v, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (Valor::Num(cx), Valor::Num(cy), Valor::Num(rh), Valor::Num(rv)) =
                (center_x_val, center_y_val, radius_h_val, radius_v_val)
            {
                println!(
                    "[DRAW] ellipse({}, {}, {}, {}, {:?})",
                    cx, cy, rh, rv, color_val
                );
            } else {
                println!("[ERROR] draw.ellipse requiere números");
            }
        }
        Stmt::DrawLineThick {
            x1,
            y1,
            x2,
            y2,
            thick,
            color,
        } => {
            let x1_val = evaluar_expr(x1, executor, funcs);
            let y1_val = evaluar_expr(y1, executor, funcs);
            let x2_val = evaluar_expr(x2, executor, funcs);
            let y2_val = evaluar_expr(y2, executor, funcs);
            let thick_val = evaluar_expr(thick, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (
                Valor::Num(x1),
                Valor::Num(y1),
                Valor::Num(x2),
                Valor::Num(y2),
                Valor::Num(thick),
            ) = (x1_val, y1_val, x2_val, y2_val, thick_val)
            {
                println!(
                    "[DRAW] line_thick({}, {}, {}, {}, {}, {:?})",
                    x1, y1, x2, y2, thick, color_val
                );
            } else {
                println!("[ERROR] draw.line_thick requiere números");
            }
        }
    }
    (None, None) // No break, no return value
}

// EJECUTOR GRÁFICO (con DrawHandle)

use rydit_gfx::DrawHandle;

// Estado del input para Snake y juegos
/// Estado del input (público para game_loop)
pub struct InputEstado {
    arrow_up: bool,
    arrow_down: bool,
    arrow_left: bool,
    arrow_right: bool,
    space: bool,
    enter: bool,
    // Mouse v0.3.0
    mouse_x: i32,
    mouse_y: i32,
    mouse_left: bool,
    mouse_right: bool,
    mouse_middle: bool,
}

impl InputEstado {
    fn new() -> Self {
        Self {
            arrow_up: false,
            arrow_down: false,
            arrow_left: false,
            arrow_right: false,
            space: false,
            enter: false,
            mouse_x: 0,
            mouse_y: 0,
            mouse_left: false,
            mouse_right: false,
            mouse_middle: false,
        }
    }

    fn actualizar(&mut self, gfx: &RyditGfx) {
        self.arrow_up = gfx.is_key_pressed(Key::ArrowUp);
        self.arrow_down = gfx.is_key_pressed(Key::ArrowDown);
        self.arrow_left = gfx.is_key_pressed(Key::ArrowLeft);
        self.arrow_right = gfx.is_key_pressed(Key::ArrowRight);
        self.space = gfx.is_key_pressed(Key::Space);
        self.enter = gfx.is_key_pressed(Key::Enter);

        // Actualizar mouse v0.3.0
        let mouse_pos = gfx.get_mouse_position();
        self.mouse_x = mouse_pos.0;
        self.mouse_y = mouse_pos.1;
        self.mouse_left = gfx.is_mouse_button_pressed(0);
        self.mouse_right = gfx.is_mouse_button_pressed(1);
        self.mouse_middle = gfx.is_mouse_button_pressed(2);
    }

    fn es_presionada(&self, tecla: &str) -> bool {
        match tecla.to_lowercase().as_str() {
            "arrow_up" | "arriba" => self.arrow_up,
            "arrow_down" | "abajo" => self.arrow_down,
            "arrow_left" | "izquierda" => self.arrow_left,
            "arrow_right" | "derecha" => self.arrow_right,
            "space" | "espacio" => self.space,
            "enter" | "entrada" => self.enter,
            _ => false,
        }
    }
}

/// Ejecutar statement en modo gráfico
fn ejecutar_stmt_gfx(
    stmt: &Stmt,
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
    d: &mut DrawHandle,
    input: &mut InputEstado,
    loaded_modules: &mut HashSet<String>,
    importing_stack: &mut Vec<String>,
) -> Option<bool> {
    match stmt {
        Stmt::Init => {
            // shield.init en modo gráfico - no hacer nada especial
        }
        Stmt::Command(cmd) => {
            executor.ejecutar(cmd);
        }
        Stmt::Assign { name, value } => {
            let valor = evaluar_expr_gfx(value, executor, input, funcs);
            executor.guardar(name, valor);
        }
        Stmt::IndexAssign {
            array,
            index,
            value,
        } => {
            // Asignación a índice de array: arr[index] = value
            let index_val = evaluar_expr_gfx(index, executor, input, funcs);
            let valor = evaluar_expr_gfx(value, executor, input, funcs);

            // Obtener el array actual
            if let Some(Valor::Array(arr)) = executor.leer(array) {
                // Calcular índice
                let idx = match index_val {
                    Valor::Num(n) => n as usize,
                    _ => {
                        println!("[ERROR] Índice debe ser número");
                        return None;
                    }
                };

                // Verificar límites
                if idx >= arr.len() {
                    println!(
                        "[ERROR] Índice {} fuera de límites (array de {} elementos)",
                        idx,
                        arr.len()
                    );
                    return None;
                }

                // Modificar el array
                let mut nuevo_arr = arr.clone();
                nuevo_arr[idx] = valor;
                executor.guardar(array, Valor::Array(nuevo_arr));
            } else {
                println!("[ERROR] '{}' no es un array", array);
                return None;
            }
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let cond_val = evaluar_expr_gfx(condition, executor, input, funcs);
            let es_verdad = match cond_val {
                Valor::Num(n) => n != 0.0,
                Valor::Bool(b) => b,
                _ => false,
            };

            if es_verdad {
                for s in then_body {
                    ejecutar_stmt_gfx(
                        s,
                        executor,
                        funcs,
                        d,
                        input,
                        loaded_modules,
                        importing_stack,
                    );
                }
            } else if let Some(else_stmts) = else_body {
                for s in else_stmts {
                    ejecutar_stmt_gfx(
                        s,
                        executor,
                        funcs,
                        d,
                        input,
                        loaded_modules,
                        importing_stack,
                    );
                }
            }
        }
        Stmt::While { condition, body } => {
            // Cuidado: while en modo gráfico puede causar loop infinito
            // Usar solo con condiciones controladas
            let mut iterations = 0;
            while iterations < 10 {
                // Límite estricto en modo gráfico
                let cond_val = evaluar_expr_gfx(condition, executor, input, funcs);
                let es_verdad = match cond_val {
                    Valor::Num(n) => n != 0.0,
                    Valor::Bool(b) => b,
                    _ => false,
                };

                if !es_verdad {
                    break;
                }

                for s in body {
                    ejecutar_stmt_gfx(
                        s,
                        executor,
                        funcs,
                        d,
                        input,
                        loaded_modules,
                        importing_stack,
                    );
                }
                iterations += 1;
            }
        }
        Stmt::ForEach {
            var,
            iterable,
            body,
        } => {
            let iterable_val = evaluar_expr_gfx(iterable, executor, input, funcs);
            if let Valor::Array(arr) = iterable_val {
                for item in arr {
                    executor.guardar(var, item.clone());
                    for s in body {
                        ejecutar_stmt_gfx(
                            s,
                            executor,
                            funcs,
                            d,
                            input,
                            loaded_modules,
                            importing_stack,
                        );
                    }
                }
            }
        }
        Stmt::Block(stmts) => {
            for s in stmts {
                ejecutar_stmt_gfx(
                    s,
                    executor,
                    funcs,
                    d,
                    input,
                    loaded_modules,
                    importing_stack,
                );
            }
        }
        Stmt::Function { name, params, body } => {
            funcs.insert(name.clone(), (params.clone(), body.clone()));
        }
        Stmt::Call { name, args } => {
            // Verificar si es tecla_presionada("tecla")
            if name == "tecla_presionada" && args.len() == 1 {
                if let Expr::Texto(tecla) = &args[0] {
                    let presionada = input.es_presionada(tecla);
                    executor.guardar("__RESULT__", Valor::Num(if presionada { 1.0 } else { 0.0 }));
                }
            } else {
                // Función de usuario - clonar datos para evitar borrow checker issues
                let func_data = funcs.get(name).map(|(p, b)| (p.clone(), b.clone()));

                if let Some((_params, body)) = func_data {
                    // Evaluar argumentos
                    let mut arg_values = vec![];
                    for arg in args {
                        arg_values.push(evaluar_expr_gfx(arg, executor, input, funcs));
                    }
                    // Ejecutar body con argumentos (sin scope real por ahora)
                    let _ = arg_values; // Por ahora no usamos los args
                    for s in &body {
                        ejecutar_stmt_gfx(
                            s,
                            executor,
                            funcs,
                            d,
                            input,
                            loaded_modules,
                            importing_stack,
                        );
                    }
                }
            }
        }
        Stmt::Return(expr) => {
            if let Some(e) = expr {
                let val = evaluar_expr(e, executor, funcs);
                executor.voz(&val);
            }
        }
        Stmt::Expr(expr) => {
            let val = evaluar_expr(expr, executor, funcs);
            executor.voz(&val);
        }
        Stmt::Import { module, alias } => {
            // Importar módulo en modo gráfico (mismo comportamiento con fixes)
            let module_path = format!("crates/modules/{}.rydit", module);

            // DEUDA #2 FIX: Detectar import cíclico
            if importing_stack.contains(module) {
                println!("[ERROR] Importe cíclico detectado: '{}'", module);
                println!(
                    "[ERROR] Stack de imports: {} -> {}",
                    importing_stack.join(" -> "),
                    module
                );
                return None;
            }

            // DEUDA #1 FIX: Verificar si ya está cargado (evitar re-ejecución)
            if loaded_modules.contains(module.as_str()) {
                println!("[IMPORT] Módulo '{}' ya cargado (usando cache)", module);
                // Solo renombrar funciones existentes
                let prefix = if let Some(alias_name) = alias {
                    alias_name.clone()
                } else {
                    module.clone()
                };

                // Copiar funciones con nuevo nombre desde el cache
                let mut funcs_to_copy: Vec<(String, String)> = Vec::new();
                for (func_name, _) in funcs.iter() {
                    if func_name.starts_with(&format!("{}::", module)) {
                        let orig_name = func_name.strip_prefix(&format!("{}::", module)).unwrap();
                        let new_name = format!("{}::{}", prefix, orig_name);
                        funcs_to_copy.push((func_name.clone(), new_name));
                    }
                }

                for (old_name, new_name) in funcs_to_copy {
                    if let Some(func_data) = funcs.get(&old_name) {
                        funcs.insert(new_name, func_data.clone());
                    }
                }

                if let Some(alias_name) = alias {
                    println!(
                        "[IMPORT] Módulo '{}' disponible como '{}'",
                        module, alias_name
                    );
                }
                return None;
            }

            if let Ok(content) = std::fs::read_to_string(&module_path) {
                println!(
                    "[IMPORT] Cargando módulo '{}' desde '{}'",
                    module, module_path
                );

                // Agregar al stack de imports en progreso
                importing_stack.push(module.clone());

                // Lexer + Parser
                let tokens = Lizer::new(&content).scan();
                let mut parser = Parser::new(tokens);

                let program = match parser.parse() {
                    Ok(p) => p,
                    Err(e) => {
                        println!("[ERROR] Error parseando módulo '{}': {}", module, e);
                        importing_stack.pop();
                        return None;
                    }
                };

                // Recolectar nombres de funciones originales
                let mut original_funcs: Vec<String> = Vec::new();
                for s in &program.statements {
                    if let Stmt::Function { name, .. } = s {
                        original_funcs.push(name.clone());
                    }
                }

                for s in &program.statements {
                    ejecutar_stmt_gfx(
                        s,
                        executor,
                        funcs,
                        d,
                        input,
                        loaded_modules,
                        importing_stack,
                    );
                }

                // Remover del stack de imports en progreso
                importing_stack.pop();

                // Marcar módulo como cargado
                loaded_modules.insert(module.clone());

                // Renombrar funciones con el prefijo del módulo
                let prefix = if let Some(alias_name) = alias {
                    alias_name.clone()
                } else {
                    module.clone()
                };

                // Copiar funciones con nuevo nombre
                for orig_name in &original_funcs {
                    if let Some(func_data) = funcs.get(orig_name) {
                        let new_name = format!("{}::{}", prefix, orig_name);
                        funcs.insert(new_name, func_data.clone());
                    }
                }

                // DEUDA #3 FIX: Eliminar funciones originales SOLO si no hay alias
                if alias.is_none() {
                    // Sin alias: eliminar funciones originales
                    for orig_name in &original_funcs {
                        funcs.remove(orig_name);
                    }
                }
                // Con alias: las funciones originales se mantienen como module::func

                if let Some(alias_name) = alias {
                    println!(
                        "[IMPORT] Módulo '{}' disponible como '{}'",
                        module, alias_name
                    );
                } else {
                    println!("[IMPORT] Módulo '{}' disponible", module);
                }
            } else {
                println!(
                    "[ERROR] Módulo '{}' no encontrado en '{}'",
                    module, module_path
                );
            }
        }
        // Comandos de dibujo - dibujan realmente en la ventana
        Stmt::DrawCircle { x, y, radio, color } => {
            let x_val = evaluar_expr(x, executor, funcs);
            let y_val = evaluar_expr(y, executor, funcs);
            let radio_val = evaluar_expr(radio, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (Valor::Num(x), Valor::Num(y), Valor::Num(radio)) = (x_val, y_val, radio_val) {
                d.draw_circle(x as i32, y as i32, radio as i32, color_val);
            }
        }
        Stmt::DrawRect {
            x,
            y,
            ancho,
            alto,
            color,
        } => {
            let x_val = evaluar_expr(x, executor, funcs);
            let y_val = evaluar_expr(y, executor, funcs);
            let ancho_val = evaluar_expr(ancho, executor, funcs);
            let alto_val = evaluar_expr(alto, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (Valor::Num(x), Valor::Num(y), Valor::Num(ancho), Valor::Num(alto)) =
                (x_val, y_val, ancho_val, alto_val)
            {
                d.draw_rectangle(x as i32, y as i32, ancho as i32, alto as i32, color_val);
            }
        }
        Stmt::DrawLine {
            x1,
            y1,
            x2,
            y2,
            color,
        } => {
            let x1_val = evaluar_expr(x1, executor, funcs);
            let y1_val = evaluar_expr(y1, executor, funcs);
            let x2_val = evaluar_expr(x2, executor, funcs);
            let y2_val = evaluar_expr(y2, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (Valor::Num(x1), Valor::Num(y1), Valor::Num(x2), Valor::Num(y2)) =
                (x1_val, y1_val, x2_val, y2_val)
            {
                d.draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, color_val);
            }
        }
        Stmt::DrawText {
            texto,
            x,
            y,
            tamano,
            color,
        } => {
            let x_val = evaluar_expr(x, executor, funcs);
            let y_val = evaluar_expr(y, executor, funcs);
            let tamano_val = evaluar_expr(tamano, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (Valor::Num(x), Valor::Num(y), Valor::Num(tamano)) = (x_val, y_val, tamano_val) {
                d.draw_text(texto, x as i32, y as i32, tamano as i32, color_val);
            }
        }
        // Statements v0.2.0 - Nuevas formas (gráficos reales)
        Stmt::DrawTriangle {
            v1_x,
            v1_y,
            v2_x,
            v2_y,
            v3_x,
            v3_y,
            color,
        } => {
            let v1_x_val = evaluar_expr(v1_x, executor, funcs);
            let v1_y_val = evaluar_expr(v1_y, executor, funcs);
            let v2_x_val = evaluar_expr(v2_x, executor, funcs);
            let v2_y_val = evaluar_expr(v2_y, executor, funcs);
            let v3_x_val = evaluar_expr(v3_x, executor, funcs);
            let v3_y_val = evaluar_expr(v3_y, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (
                Valor::Num(v1_x),
                Valor::Num(v1_y),
                Valor::Num(v2_x),
                Valor::Num(v2_y),
                Valor::Num(v3_x),
                Valor::Num(v3_y),
            ) = (v1_x_val, v1_y_val, v2_x_val, v2_y_val, v3_x_val, v3_y_val)
            {
                d.draw_triangle(
                    (v1_x as i32, v1_y as i32),
                    (v2_x as i32, v2_y as i32),
                    (v3_x as i32, v3_y as i32),
                    color_val,
                );
            }
        }
        Stmt::DrawRing {
            center_x,
            center_y,
            inner_radius,
            outer_radius,
            color,
        } => {
            let center_x_val = evaluar_expr(center_x, executor, funcs);
            let center_y_val = evaluar_expr(center_y, executor, funcs);
            let inner_radius_val = evaluar_expr(inner_radius, executor, funcs);
            let outer_radius_val = evaluar_expr(outer_radius, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (Valor::Num(cx), Valor::Num(cy), Valor::Num(ir), Valor::Num(or)) = (
                center_x_val,
                center_y_val,
                inner_radius_val,
                outer_radius_val,
            ) {
                d.draw_ring((cx as i32, cy as i32), ir as i32, or as i32, color_val);
            }
        }
        Stmt::DrawRectangleLines {
            x,
            y,
            ancho,
            alto,
            color,
        } => {
            let x_val = evaluar_expr(x, executor, funcs);
            let y_val = evaluar_expr(y, executor, funcs);
            let ancho_val = evaluar_expr(ancho, executor, funcs);
            let alto_val = evaluar_expr(alto, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (Valor::Num(x), Valor::Num(y), Valor::Num(ancho), Valor::Num(alto)) =
                (x_val, y_val, ancho_val, alto_val)
            {
                d.draw_rectangle_lines(x as i32, y as i32, ancho as i32, alto as i32, color_val);
            }
        }
        Stmt::DrawEllipse {
            center_x,
            center_y,
            radius_h,
            radius_v,
            color,
        } => {
            let center_x_val = evaluar_expr(center_x, executor, funcs);
            let center_y_val = evaluar_expr(center_y, executor, funcs);
            let radius_h_val = evaluar_expr(radius_h, executor, funcs);
            let radius_v_val = evaluar_expr(radius_v, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (Valor::Num(cx), Valor::Num(cy), Valor::Num(rh), Valor::Num(rv)) =
                (center_x_val, center_y_val, radius_h_val, radius_v_val)
            {
                d.draw_ellipse((cx as i32, cy as i32), rh as i32, rv as i32, color_val);
            }
        }
        Stmt::DrawLineThick {
            x1,
            y1,
            x2,
            y2,
            thick,
            color,
        } => {
            let x1_val = evaluar_expr(x1, executor, funcs);
            let y1_val = evaluar_expr(y1, executor, funcs);
            let x2_val = evaluar_expr(x2, executor, funcs);
            let y2_val = evaluar_expr(y2, executor, funcs);
            let thick_val = evaluar_expr(thick, executor, funcs);
            let color_val = ColorRydit::from_str(color).unwrap_or(ColorRydit::Blanco);

            if let (
                Valor::Num(x1),
                Valor::Num(y1),
                Valor::Num(x2),
                Valor::Num(y2),
                Valor::Num(thick),
            ) = (x1_val, y1_val, x2_val, y2_val, thick_val)
            {
                d.draw_line_thick(
                    (x1 as i32, y1 as i32),
                    (x2 as i32, y2 as i32),
                    thick as f32,
                    color_val,
                );
            }
        }
        Stmt::Break => {
            return Some(true); // Señal de break
        }
    }
    None
}

/// Convertir Valor a bool - PÚBLICA para eval
pub fn valor_a_bool(val: &Valor) -> bool {
    match val {
        Valor::Bool(b) => *b,
        Valor::Num(n) => *n != 0.0,
        _ => false,
    }
}
fn evaluar_expr_gfx(
    expr: &Expr,
    executor: &mut Executor,
    input: &InputEstado,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    match expr {
        Expr::Num(n) => Valor::Num(*n),
        Expr::Texto(s) => Valor::Texto(s.clone()),
        Expr::Var(name) => {
            if name == "__INPUT__" {
                return executor.input("> ");
            }
            executor.leer(name).unwrap_or(Valor::Vacio)
        }
        Expr::Bool(b) => Valor::Bool(*b),
        Expr::Array(elements) => {
            let valores: Vec<Valor> = elements
                .iter()
                .map(|e| evaluar_expr_gfx(e, executor, input, funcs))
                .collect();
            Valor::Array(valores)
        }
        Expr::Index { array, index } => {
            let array_val = evaluar_expr_gfx(array, executor, input, funcs);
            let index_val = evaluar_expr_gfx(index, executor, input, funcs);

            if let Valor::Array(arr) = array_val {
                if let Valor::Num(i) = index_val {
                    let idx = i as usize;
                    if idx < arr.len() {
                        arr[idx].clone()
                    } else {
                        Valor::Error(format!("Índice {} fuera de rango (len={})", idx, arr.len()))
                    }
                } else {
                    Valor::Error("El índice debe ser un número".to_string())
                }
            } else {
                Valor::Error("Solo se puede indexar arrays".to_string())
            }
        }
        Expr::Call { name, args } => {
            // tecla_presionada("tecla") - retorna 1.0 si presionada, 0.0 si no
            if name == "tecla_presionada" && args.len() == 1 {
                if let Expr::Texto(tecla) = &args[0] {
                    let presionada = input.es_presionada(tecla);
                    return Valor::Num(if presionada { 1.0 } else { 0.0 });
                }
            }

            // ========================================================================
            // INPUT MOUSE - V0.3.0 (Tank Combat)
            // ========================================================================

            // input::mouse_x() - Retorna posición X del mouse
            if name == "input::mouse_x" || name == "__input_mouse_x" {
                return Valor::Num(input.mouse_x as f64);
            }

            // input::mouse_y() - Retorna posición Y del mouse
            if name == "input::mouse_y" || name == "__input_mouse_y" {
                return Valor::Num(input.mouse_y as f64);
            }

            // input::mouse_position() - Retorna [x, y]
            if name == "input::mouse_position" || name == "__input_mouse_position" {
                return Valor::Array(vec![
                    Valor::Num(input.mouse_x as f64),
                    Valor::Num(input.mouse_y as f64),
                ]);
            }

            // input::is_mouse_button_pressed(button) - 0=izq, 1=der, 2=medio
            if (name == "input::is_mouse_button_pressed"
                || name == "__input_is_mouse_button_pressed")
                && args.len() == 1
            {
                if let Valor::Num(button) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    // Usamos el input estado para botones de mouse
                    let presionado = match button as i32 {
                        0 => input.mouse_left,
                        1 => input.mouse_right,
                        2 => input.mouse_middle,
                        _ => false,
                    };
                    return Valor::Num(if presionado { 1.0 } else { 0.0 });
                }
            }

            // Funciones aritméticas builtin
            if name == "sumar" && args.len() >= 2 {
                let mut suma = 0.0;
                for arg in args {
                    if let Valor::Num(n) = evaluar_expr_gfx(arg, executor, input, funcs) {
                        suma += n;
                    } else {
                        return Valor::Error("sumar() requiere números".to_string());
                    }
                }
                return Valor::Num(suma);
            }

            if name == "restar" && args.len() == 2 {
                let a = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let b = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Num(a), Valor::Num(b)) = (a, b) {
                    return Valor::Num(a - b);
                } else {
                    return Valor::Error("restar() requiere números".to_string());
                }
            }

            if name == "multiplicar" && args.len() >= 2 {
                let mut producto = 1.0;
                for arg in args {
                    if let Valor::Num(n) = evaluar_expr_gfx(arg, executor, input, funcs) {
                        producto *= n;
                    } else {
                        return Valor::Error("multiplicar() requiere números".to_string());
                    }
                }
                return Valor::Num(producto);
            }

            if name == "dividir" && args.len() == 2 {
                let a = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let b = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Num(a), Valor::Num(b)) = (a, b) {
                    if b != 0.0 {
                        return Valor::Num(a / b);
                    } else {
                        return Valor::Error("División por cero".to_string());
                    }
                } else {
                    return Valor::Error("dividir() requiere números".to_string());
                }
            }

            // ========================================================================
            // FUNCIONES MATH AVANZADAS - V0.3.0 (Tank Combat)
            // ========================================================================

            // math::sqrt(x) - Raíz cuadrada
            if (name == "__math_sqrt" || name == "math::sqrt") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    if x >= 0.0 {
                        return Valor::Num(x.sqrt());
                    } else {
                        return Valor::Error("math::sqrt() requiere número >= 0".to_string());
                    }
                } else {
                    return Valor::Error("math::sqrt() requiere número".to_string());
                }
            }

            // math::sin(x) - Seno (x en radianes)
            if (name == "__math_sin" || name == "math::sin") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Num(x.sin());
                } else {
                    return Valor::Error("math::sin() requiere número".to_string());
                }
            }

            // math::cos(x) - Coseno (x en radianes)
            if (name == "__math_cos" || name == "math::cos") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Num(x.cos());
                } else {
                    return Valor::Error("math::cos() requiere número".to_string());
                }
            }

            // math::tan(x) - Tangente (x en radianes)
            if (name == "__math_tan" || name == "math::tan") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Num(x.tan());
                } else {
                    return Valor::Error("math::tan() requiere número".to_string());
                }
            }

            // math::atan2(y, x) - Arcotangente de y/x (retorna radianes)
            if (name == "__math_atan2" || name == "math::atan2") && args.len() == 2 {
                let y_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let x_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Num(y), Valor::Num(x)) = (y_val, x_val) {
                    return Valor::Num(y.atan2(x));
                } else {
                    return Valor::Error("math::atan2() requiere dos números".to_string());
                }
            }

            // math::deg2rad(x) - Convertir grados a radianes
            if (name == "__math_deg2rad" || name == "math::deg2rad") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Num(x.to_radians());
                } else {
                    return Valor::Error("math::deg2rad() requiere número".to_string());
                }
            }

            // math::rad2deg(x) - Convertir radianes a grados
            if (name == "__math_rad2deg" || name == "math::rad2deg") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Num(x.to_degrees());
                } else {
                    return Valor::Error("math::rad2deg() requiere número".to_string());
                }
            }

            // ========== FUNCIONES STRING (v0.1.2) ==========
            // Soporte para strings::length, strings::upper, etc.
            if (name == "__str_length" || name == "strings::length") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Num(s.len() as f64);
                } else {
                    return Valor::Error("strings::length() requiere string".to_string());
                }
            }

            if (name == "__str_upper" || name == "strings::upper") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Texto(s.to_uppercase());
                } else {
                    return Valor::Error("strings::upper() requiere string".to_string());
                }
            }

            if (name == "__str_lower" || name == "strings::lower") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Texto(s.to_lowercase());
                } else {
                    return Valor::Error("strings::lower() requiere string".to_string());
                }
            }

            if (name == "__str_concat" || name == "strings::concat") && args.len() == 2 {
                let a = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let b = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Texto(a), Valor::Texto(b)) = (a, b) {
                    return Valor::Texto(format!("{}{}", a, b));
                } else {
                    return Valor::Error("strings::concat() requiere dos strings".to_string());
                }
            }

            if (name == "__str_trim" || name == "strings::trim") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Texto(s.trim().to_string());
                } else {
                    return Valor::Error("strings::trim() requiere string".to_string());
                }
            }

            if (name == "__str_substr" || name == "strings::substr") && args.len() == 3 {
                let s_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let start_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                let len_val = evaluar_expr_gfx(&args[2], executor, input, funcs);
                if let (Valor::Texto(s), Valor::Num(start), Valor::Num(len)) =
                    (s_val, start_val, len_val)
                {
                    let start_idx = start as usize;
                    let length = len as usize;
                    if start_idx + length <= s.len() {
                        return Valor::Texto(s[start_idx..start_idx + length].to_string());
                    } else {
                        return Valor::Error(
                            "strings::substr(): índices fuera de rango".to_string(),
                        );
                    }
                } else {
                    return Valor::Error(
                        "strings::substr() requiere (string, inicio, longitud)".to_string(),
                    );
                }
            }

            if (name == "__str_replace" || name == "strings::replace") && args.len() == 3 {
                let s_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let buscar_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                let reemplazar_val = evaluar_expr_gfx(&args[2], executor, input, funcs);
                if let (Valor::Texto(s), Valor::Texto(buscar), Valor::Texto(reemplazar)) =
                    (s_val, buscar_val, reemplazar_val)
                {
                    return Valor::Texto(s.replace(&buscar, &reemplazar));
                } else {
                    return Valor::Error("strings::replace() requiere tres strings".to_string());
                }
            }

            // ========== NUEVAS FUNCIONES STRINGS (v0.1.4) ==========
            if (name == "__str_split" || name == "strings::split") && args.len() == 2 {
                let s_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let sep_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Texto(s), Valor::Texto(sep)) = (s_val, sep_val) {
                    let partes: Vec<Valor> =
                        s.split(&sep).map(|p| Valor::Texto(p.to_string())).collect();
                    return Valor::Array(partes);
                } else {
                    return Valor::Error(
                        "strings::split() requiere (string, separador)".to_string(),
                    );
                }
            }

            if (name == "__str_starts_with" || name == "strings::starts_with") && args.len() == 2 {
                let s_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let prefix_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Texto(s), Valor::Texto(prefix)) = (s_val, prefix_val) {
                    return Valor::Bool(s.starts_with(&prefix));
                } else {
                    return Valor::Error("strings::starts_with() requiere dos strings".to_string());
                }
            }

            if (name == "__str_ends_with" || name == "strings::ends_with") && args.len() == 2 {
                let s_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let suffix_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Texto(s), Valor::Texto(suffix)) = (s_val, suffix_val) {
                    return Valor::Bool(s.ends_with(&suffix));
                } else {
                    return Valor::Error("strings::ends_with() requiere dos strings".to_string());
                }
            }

            if (name == "__str_replace_all" || name == "strings::replace_all") && args.len() == 3 {
                let s_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let buscar_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                let reemplazar_val = evaluar_expr_gfx(&args[2], executor, input, funcs);
                if let (Valor::Texto(s), Valor::Texto(buscar), Valor::Texto(reemplazar)) =
                    (s_val, buscar_val, reemplazar_val)
                {
                    return Valor::Texto(s.replace(&buscar, &reemplazar));
                } else {
                    return Valor::Error(
                        "strings::replace_all() requiere tres strings".to_string(),
                    );
                }
            }

            if (name == "__str_join" || name == "strings::join") && args.len() == 2 {
                let sep_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let arr_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Texto(sep), Valor::Array(arr)) = (sep_val, arr_val) {
                    let strs: Result<Vec<String>, _> = arr
                        .iter()
                        .map(|v| {
                            if let Valor::Texto(s) = v {
                                Ok(s.clone())
                            } else {
                                Err("strings::join() requiere array de strings")
                            }
                        })
                        .collect();
                    match strs {
                        Ok(parts) => return Valor::Texto(parts.join(&sep)),
                        Err(msg) => return Valor::Error(msg.to_string()),
                    }
                } else {
                    return Valor::Error("strings::join() requiere (separador, array)".to_string());
                }
            }

            // ========== FUNCIONES IO (v0.1.2) ==========
            if (name == "__file_read" || name == "io::read_file") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    match std::fs::read_to_string(&path) {
                        Ok(content) => return Valor::Texto(content),
                        Err(e) => return Valor::Error(format!("io::read_file(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::read_file() requiere path (string)".to_string());
                }
            }

            if (name == "__file_write" || name == "io::write_file") && args.len() == 2 {
                let path_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let content_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Texto(path), Valor::Texto(content)) = (path_val, content_val) {
                    match std::fs::write(&path, &content) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::write_file(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::write_file() requiere (path, content)".to_string());
                }
            }

            if (name == "__file_exists" || name == "io::file_exists") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Bool(std::path::Path::new(&path).exists());
                } else {
                    return Valor::Error("io::file_exists() requiere path (string)".to_string());
                }
            }

            // ========== NUEVAS FUNCIONES IO (v0.1.4) ==========
            if (name == "__dir_mkdir" || name == "io::mkdir") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    match std::fs::create_dir_all(&path) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::mkdir(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::mkdir() requiere path (string)".to_string());
                }
            }

            if (name == "__file_remove" || name == "io::remove") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    match std::fs::remove_file(&path) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => match std::fs::remove_dir_all(&path) {
                            Ok(_) => return Valor::Num(1.0),
                            Err(_) => return Valor::Error(format!("io::remove(): {}", e)),
                        },
                    }
                } else {
                    return Valor::Error("io::remove() requiere path (string)".to_string());
                }
            }

            if (name == "__file_rename" || name == "io::rename") && args.len() == 2 {
                let old_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let new_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Texto(old), Valor::Texto(new)) = (old_val, new_val) {
                    match std::fs::rename(&old, &new) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::rename(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::rename() requiere (old, new)".to_string());
                }
            }

            if (name == "__file_copy" || name == "io::copy") && args.len() == 2 {
                let src_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let dst_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Texto(src), Valor::Texto(dst)) = (src_val, dst_val) {
                    match std::fs::copy(&src, &dst) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::copy(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::copy() requiere (src, dst)".to_string());
                }
            }

            // ========== NUEVAS FUNCIONES ARRAYS (v0.1.4) ==========
            if (name == "__array_push" || name == "arrays::push") && args.len() == 2 {
                let arr_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let elem_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let Valor::Array(mut arr) = arr_val {
                    arr.push(elem_val);
                    return Valor::Array(arr);
                } else {
                    return Valor::Error("arrays::push() requiere (array, elemento)".to_string());
                }
            }

            if (name == "__array_pop" || name == "arrays::pop") && args.len() == 1 {
                if let Valor::Array(mut arr) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    if arr.is_empty() {
                        return Valor::Error("arrays::pop(): array vacío".to_string());
                    }
                    let last = arr.pop().unwrap();
                    return last;
                } else {
                    return Valor::Error("arrays::pop() requiere array".to_string());
                }
            }

            if (name == "__array_shift" || name == "arrays::shift") && args.len() == 1 {
                if let Valor::Array(mut arr) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    if arr.is_empty() {
                        return Valor::Error("arrays::shift(): array vacío".to_string());
                    }
                    let first = arr.remove(0);
                    return first;
                } else {
                    return Valor::Error("arrays::shift() requiere array".to_string());
                }
            }

            if (name == "__array_unshift" || name == "arrays::unshift") && args.len() == 2 {
                let arr_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let elem_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let Valor::Array(mut arr) = arr_val {
                    arr.insert(0, elem_val);
                    return Valor::Array(arr);
                } else {
                    return Valor::Error(
                        "arrays::unshift() requiere (array, elemento)".to_string(),
                    );
                }
            }

            if (name == "__array_slice" || name == "arrays::slice") && args.len() == 3 {
                let arr_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let start_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                let end_val = evaluar_expr_gfx(&args[2], executor, input, funcs);
                if let (Valor::Array(arr), Valor::Num(start), Valor::Num(end)) =
                    (arr_val, start_val, end_val)
                {
                    let s = start as usize;
                    let e = end as usize;
                    if s <= e && e <= arr.len() {
                        let sliced: Vec<Valor> = arr[s..e].to_vec();
                        return Valor::Array(sliced);
                    } else {
                        return Valor::Error("arrays::slice(): índices inválidos".to_string());
                    }
                } else {
                    return Valor::Error(
                        "arrays::slice() requiere (array, inicio, fin)".to_string(),
                    );
                }
            }

            if (name == "__array_reverse" || name == "arrays::reverse") && args.len() == 1 {
                if let Valor::Array(mut arr) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    arr.reverse();
                    return Valor::Array(arr);
                } else {
                    return Valor::Error("arrays::reverse() requiere array".to_string());
                }
            }

            // ========== FUNCIONES RANDOM (v0.1.6) ==========
            if (name == "__random_int" || name == "random::int") && args.len() == 2 {
                let min_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let max_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Num(min), Valor::Num(max)) = (min_val, max_val) {
                    let seed = executor
                        .leer("__random_seed")
                        .unwrap_or(Valor::Num(12345.0));
                    let mut s = if let Valor::Num(n) = seed {
                        n as u32
                    } else {
                        12345
                    };
                    s ^= s << 13;
                    s ^= s >> 17;
                    s ^= s << 5;
                    executor.guardar("__random_seed", Valor::Num(s as f64));
                    let range = (max - min).abs() + 1.0;
                    let random_val = (s as f64 / u32::MAX as f64) * range;
                    return Valor::Num(min + random_val);
                } else {
                    return Valor::Error("random::int() requiere (min, max) números".to_string());
                }
            }

            if (name == "__random_float" || name == "random::float") && args.is_empty() {
                let seed = executor
                    .leer("__random_seed")
                    .unwrap_or(Valor::Num(12345.0));
                let mut s = if let Valor::Num(n) = seed {
                    n as u32
                } else {
                    12345
                };
                s ^= s << 13;
                s ^= s >> 17;
                s ^= s << 5;
                executor.guardar("__random_seed", Valor::Num(s as f64));
                return Valor::Num(s as f64 / u32::MAX as f64);
            }

            if (name == "__random_choice" || name == "random::choice") && args.len() == 1 {
                let arr_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                if let Valor::Array(arr) = arr_val {
                    if arr.is_empty() {
                        return Valor::Error("random::choice(): array vacío".to_string());
                    }
                    let seed = executor
                        .leer("__random_seed")
                        .unwrap_or(Valor::Num(12345.0));
                    let mut s = if let Valor::Num(n) = seed {
                        n as u32
                    } else {
                        12345
                    };
                    s ^= s << 13;
                    s ^= s >> 17;
                    s ^= s << 5;
                    executor.guardar("__random_seed", Valor::Num(s as f64));
                    let idx = (s as usize) % arr.len();
                    return arr[idx].clone();
                } else {
                    return Valor::Error("random::choice() requiere array".to_string());
                }
            }

            // ========== FUNCIONES JSON (v0.1.5) ==========
            if (name == "__json_parse" || name == "json::parse") && args.len() == 1 {
                if let Valor::Texto(json_str) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    match serde_json::from_str::<serde_json::Value>(&json_str) {
                        Ok(val) => {
                            return valor_serde_a_rydit(&val);
                        }
                        Err(e) => return Valor::Error(format!("json::parse(): {}", e)),
                    }
                } else {
                    return Valor::Error("json::parse() requiere string JSON".to_string());
                }
            }

            if (name == "__json_stringify" || name == "json::stringify") && args.len() == 1 {
                let val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                match valor_rydit_a_serde(&val) {
                    Ok(serde_val) => match serde_json::to_string(&serde_val) {
                        Ok(json_str) => return Valor::Texto(json_str),
                        Err(e) => return Valor::Error(format!("json::stringify(): {}", e)),
                    },
                    Err(e) => return Valor::Error(format!("json::stringify(): {}", e)),
                }
            }

            // ========== FUNCIONES TIME (v0.1.6) ==========
            if (name == "__time_now" || name == "time::now") && args.is_empty() {
                use std::time::{SystemTime, UNIX_EPOCH};
                match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(duration) => return Valor::Num(duration.as_secs_f64()),
                    Err(e) => return Valor::Error(format!("time::now(): {}", e)),
                }
            }

            if (name == "__time_sleep" || name == "time::sleep") && args.len() == 1 {
                use std::{thread, time::Duration};
                let ms_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                if let Valor::Num(ms) = ms_val {
                    thread::sleep(Duration::from_millis(ms as u64));
                    return Valor::Vacio;
                } else {
                    return Valor::Error(
                        "time::sleep() requiere milisegundos (número)".to_string(),
                    );
                }
            }

            // ========== FUNCIONES REGEX (v0.6.2) ==========
            if (name == "__regex_match" || name == "regex::match") && args.len() == 2 {
                if let (Valor::Texto(pattern), Valor::Texto(text)) = (
                    &evaluar_expr_gfx(&args[0], executor, input, funcs),
                    &evaluar_expr_gfx(&args[1], executor, input, funcs),
                ) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => return Valor::Bool(re.is_match(text)),
                        Err(e) => return Valor::Error(format!("regex::match(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::match() requiere (patrón, texto)".to_string());
                }
            }

            if (name == "__regex_replace" || name == "regex::replace") && args.len() == 3 {
                if let (Valor::Texto(pattern), Valor::Texto(replacement), Valor::Texto(text)) = (
                    &evaluar_expr_gfx(&args[0], executor, input, funcs),
                    &evaluar_expr_gfx(&args[1], executor, input, funcs),
                    &evaluar_expr_gfx(&args[2], executor, input, funcs),
                ) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => {
                            return Valor::Texto(
                                re.replace_all(text, replacement.as_str()).to_string(),
                            )
                        }
                        Err(e) => return Valor::Error(format!("regex::replace(): {}", e)),
                    }
                } else {
                    return Valor::Error(
                        "regex::replace() requiere (patrón, reemplazo, texto)".to_string(),
                    );
                }
            }

            if (name == "__regex_split" || name == "regex::split") && args.len() == 2 {
                if let (Valor::Texto(pattern), Valor::Texto(text)) = (
                    &evaluar_expr_gfx(&args[0], executor, input, funcs),
                    &evaluar_expr_gfx(&args[1], executor, input, funcs),
                ) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => {
                            let parts: Vec<Valor> = re
                                .split(text)
                                .map(|s| Valor::Texto(s.to_string()))
                                .collect();
                            return Valor::Array(parts);
                        }
                        Err(e) => return Valor::Error(format!("regex::split(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::split() requiere (patrón, texto)".to_string());
                }
            }

            if (name == "__regex_find_all" || name == "regex::find_all") && args.len() == 2 {
                if let (Valor::Texto(pattern), Valor::Texto(text)) = (
                    &evaluar_expr_gfx(&args[0], executor, input, funcs),
                    &evaluar_expr_gfx(&args[1], executor, input, funcs),
                ) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => {
                            let matches: Vec<Valor> = re
                                .find_iter(text)
                                .map(|m| Valor::Texto(m.as_str().to_string()))
                                .collect();
                            return Valor::Array(matches);
                        }
                        Err(e) => return Valor::Error(format!("regex::find_all(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::find_all() requiere (patrón, texto)".to_string());
                }
            }

            if (name == "__regex_capture" || name == "regex::capture") && args.len() == 2 {
                if let (Valor::Texto(pattern), Valor::Texto(text)) = (
                    &evaluar_expr_gfx(&args[0], executor, input, funcs),
                    &evaluar_expr_gfx(&args[1], executor, input, funcs),
                ) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => {
                            if let Some(caps) = re.captures(text) {
                                let mut result: Vec<Valor> = Vec::new();
                                result
                                    .push(Valor::Texto(caps.get(0).unwrap().as_str().to_string()));
                                for i in 1..caps.len() {
                                    if let Some(m) = caps.get(i) {
                                        result.push(Valor::Texto(m.as_str().to_string()));
                                    } else {
                                        result.push(Valor::Vacio);
                                    }
                                }
                                return Valor::Array(result);
                            } else {
                                return Valor::Array(vec![]);
                            }
                        }
                        Err(e) => return Valor::Error(format!("regex::capture(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::capture() requiere (patrón, texto)".to_string());
                }
            }

            // ========== FUNCIONES FILES (v0.6.3) ==========
            if (name == "__files_read" || name == "files::read") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    match std::fs::read_to_string(&path) {
                        Ok(content) => return Valor::Texto(content),
                        Err(e) => return Valor::Error(format!("files::read(): {}", e)),
                    }
                } else {
                    return Valor::Error("files::read() requiere ruta (string)".to_string());
                }
            }

            if (name == "__files_write" || name == "files::write") && args.len() == 2 {
                if let (Valor::Texto(path), Valor::Texto(content)) = (
                    &evaluar_expr_gfx(&args[0], executor, input, funcs),
                    &evaluar_expr_gfx(&args[1], executor, input, funcs),
                ) {
                    match std::fs::write(path, content) {
                        Ok(_) => return Valor::Bool(true),
                        Err(e) => return Valor::Error(format!("files::write(): {}", e)),
                    }
                } else {
                    return Valor::Error("files::write() requiere (ruta, contenido)".to_string());
                }
            }

            if (name == "__files_append" || name == "files::append") && args.len() == 2 {
                if let (Valor::Texto(path), Valor::Texto(content)) = (
                    &evaluar_expr_gfx(&args[0], executor, input, funcs),
                    &evaluar_expr_gfx(&args[1], executor, input, funcs),
                ) {
                    use std::io::Write;
                    match std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(path)
                    {
                        Ok(mut file) => match file.write_all(content.as_bytes()) {
                            Ok(_) => return Valor::Bool(true),
                            Err(e) => return Valor::Error(format!("files::append(): {}", e)),
                        },
                        Err(e) => return Valor::Error(format!("files::append(): {}", e)),
                    }
                } else {
                    return Valor::Error("files::append() requiere (ruta, contenido)".to_string());
                }
            }

            if (name == "__files_exists" || name == "files::exists") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    let exists = std::path::Path::new(&path).exists();
                    return Valor::Bool(exists);
                } else {
                    return Valor::Error("files::exists() requiere ruta (string)".to_string());
                }
            }

            if (name == "__files_delete" || name == "files::delete") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    match std::fs::remove_file(&path) {
                        Ok(_) => return Valor::Bool(true),
                        Err(e) => return Valor::Error(format!("files::delete(): {}", e)),
                    }
                } else {
                    return Valor::Error("files::delete() requiere ruta (string)".to_string());
                }
            }

            Valor::Error(format!("Función '{}' no soportada", name))
        }
        Expr::BinOp { left, op, right } => {
            let left_val = evaluar_expr_gfx(left, executor, input, funcs);
            let right_val = evaluar_expr_gfx(right, executor, input, funcs);

            match op {
                lizer::BinOp::And => {
                    let l_bool = valor_a_bool(&left_val);
                    let r_bool = valor_a_bool(&right_val);
                    return Valor::Bool(l_bool && r_bool);
                }
                lizer::BinOp::Or => {
                    let l_bool = valor_a_bool(&left_val);
                    let r_bool = valor_a_bool(&right_val);
                    return Valor::Bool(l_bool || r_bool);
                }
                _ => {}
            }

            // Concatenación de strings con + (con coerción automática de números)
            if matches!(op, lizer::BinOp::Suma) {
                match (&left_val, &right_val) {
                    (Valor::Texto(l), Valor::Texto(r)) => {
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Texto(l), Valor::Num(r)) => {
                        // "texto" + numero -> "texto123"
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Num(l), Valor::Texto(r)) => {
                        // numero + "texto" -> "123texto"
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Num(_), Valor::Num(_)) => {
                        // numero + numero -> suma aritmética (comportamiento normal, se maneja abajo)
                    }
                    _ => {}
                }
            }

            if let (Valor::Num(l), Valor::Num(r)) = (left_val, right_val) {
                return match op {
                    lizer::BinOp::Suma => Valor::Num(l + r),
                    lizer::BinOp::Resta => Valor::Num(l - r),
                    lizer::BinOp::Mult => Valor::Num(l * r),
                    lizer::BinOp::Div => {
                        if r != 0.0 {
                            Valor::Num(l / r)
                        } else {
                            Valor::Error("División por cero".to_string())
                        }
                    }
                    lizer::BinOp::Mayor => Valor::Bool(l > r),
                    lizer::BinOp::Menor => Valor::Bool(l < r),
                    lizer::BinOp::Igual => Valor::Bool((l - r).abs() < 0.0001),
                    lizer::BinOp::MayorIgual => Valor::Bool(l >= r),
                    lizer::BinOp::MenorIgual => Valor::Bool(l <= r),
                    _ => Valor::Error("Operador no soportado".to_string()),
                };
            }

            Valor::Error("Operación inválida".to_string())
        }
        Expr::Unary { op, expr } => {
            let val = evaluar_expr_gfx(expr, executor, input, funcs);
            match op {
                lizer::UnaryOp::Not => {
                    let b = valor_a_bool(&val);
                    Valor::Bool(!b)
                }
                lizer::UnaryOp::Neg => {
                    if let Valor::Num(n) = val {
                        Valor::Num(-n)
                    } else {
                        Valor::Error("Neg requiere número".to_string())
                    }
                }
            }
        }
    }
}

// EVALUAR EXPRESION MODO MIGUI

pub fn evaluar_expr_migui(
    expr: &Expr,
    executor: &mut Executor,
    gui: &mut Migui,
    checkbox_states: &mut HashMap<String, bool>,
    slider_states: &mut HashMap<String, f32>,
    textbox_states: &mut HashMap<String, String>,
    window_states: &mut HashMap<String, bool>,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    match expr {
        Expr::Num(n) => Valor::Num(*n),
        Expr::Texto(s) => Valor::Texto(s.clone()),
        Expr::Var(name) => {
            if name == "__INPUT__" {
                return executor.input("> ");
            }
            executor.leer(name).unwrap_or(Valor::Vacio)
        }
        Expr::Bool(b) => Valor::Bool(*b),
        Expr::Array(elements) => {
            let valores: Vec<Valor> = elements
                .iter()
                .map(|e| {
                    evaluar_expr_migui(
                        e,
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    )
                })
                .collect();
            Valor::Array(valores)
        }
        Expr::Index { array, index } => {
            let array_val = evaluar_expr_migui(
                array,
                executor,
                gui,
                checkbox_states,
                slider_states,
                textbox_states,
                window_states,
                funcs,
            );
            let index_val = evaluar_expr_migui(
                index,
                executor,
                gui,
                checkbox_states,
                slider_states,
                textbox_states,
                window_states,
                funcs,
            );

            if let Valor::Array(arr) = array_val {
                if let Valor::Num(i) = index_val {
                    let idx = i as usize;
                    if idx < arr.len() {
                        arr[idx].clone()
                    } else {
                        Valor::Error(format!("Índice {} fuera de rango (len={})", idx, arr.len()))
                    }
                } else {
                    Valor::Error("El índice debe ser un número".to_string())
                }
            } else {
                Valor::Error("Solo se puede indexar arrays".to_string())
            }
        }
        Expr::Call { name, args } => {
            println!("[MIGUI DEBUG] Llamada a funcion: {}", name);
            // ========================================================================
            // FUNCIONES MIGUI - V0.4.0 (Immediate Mode GUI)
            // ========================================================================

            // migui::button(id, text, x, y, w, h) -> bool
            if (name == "migui::button" || name == "__migui_button") && args.len() == 6 {
                if let (
                    Valor::Texto(id),
                    Valor::Texto(text),
                    Valor::Num(x),
                    Valor::Num(y),
                    Valor::Num(w),
                    Valor::Num(h),
                ) = (
                    evaluar_expr_migui(
                        &args[0],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[1],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[2],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[3],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[4],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[5],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                ) {
                    let clicked = gui.button(
                        WidgetId::new(&id),
                        Rect::new(x as f32, y as f32, w as f32, h as f32),
                        &text,
                    );
                    return Valor::Bool(clicked);
                } else {
                    return Valor::Error(
                        "migui::button() requiere (id, text, x, y, w, h)".to_string(),
                    );
                }
            }

            // migui::label(id, text, x, y, w, h)
            if (name == "migui::label" || name == "__migui_label") && args.len() == 6 {
                if let (
                    Valor::Texto(id),
                    Valor::Texto(text),
                    Valor::Num(x),
                    Valor::Num(y),
                    Valor::Num(w),
                    Valor::Num(h),
                ) = (
                    evaluar_expr_migui(
                        &args[0],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[1],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[2],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[3],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[4],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[5],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                ) {
                    gui.label(
                        WidgetId::new(&id),
                        &text,
                        Rect::new(x as f32, y as f32, w as f32, h as f32),
                    );
                    return Valor::Vacio;
                } else {
                    return Valor::Error(
                        "migui::label() requiere (id, text, x, y, w, h)".to_string(),
                    );
                }
            }

            // migui::checkbox(id, text, checked, x, y, w, h) -> bool
            if (name == "migui::checkbox" || name == "__migui_checkbox") && args.len() == 7 {
                if let (
                    Valor::Texto(id),
                    Valor::Texto(text),
                    Valor::Bool(checked),
                    Valor::Num(x),
                    Valor::Num(y),
                    Valor::Num(w),
                    Valor::Num(h),
                ) = (
                    evaluar_expr_migui(
                        &args[0],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[1],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[2],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[3],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[4],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[5],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[6],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                ) {
                    let state = checkbox_states.entry(id.clone()).or_insert(checked);
                    let changed = gui.checkbox(
                        WidgetId::new(&id),
                        &text,
                        state,
                        Rect::new(x as f32, y as f32, w as f32, h as f32),
                    );
                    return Valor::Bool(changed);
                } else {
                    return Valor::Error(
                        "migui::checkbox() requiere (id, text, checked, x, y, w, h)".to_string(),
                    );
                }
            }

            // migui::slider(id, value, min, max, x, y, w, h) -> f32
            if (name == "migui::slider" || name == "__migui_slider") && args.len() == 8 {
                if let (
                    Valor::Texto(id),
                    Valor::Num(value),
                    Valor::Num(min),
                    Valor::Num(max),
                    Valor::Num(x),
                    Valor::Num(y),
                    Valor::Num(w),
                    Valor::Num(h),
                ) = (
                    evaluar_expr_migui(
                        &args[0],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[1],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[2],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[3],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[4],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[5],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[6],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[7],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                ) {
                    let state = slider_states.entry(id.clone()).or_insert(value as f32);
                    *state = gui.slider(
                        WidgetId::new(&id),
                        *state,
                        min as f32,
                        max as f32,
                        Rect::new(x as f32, y as f32, w as f32, h as f32),
                    );
                    return Valor::Num(*state as f64);
                } else {
                    return Valor::Error(
                        "migui::slider() requiere (id, value, min, max, x, y, w, h)".to_string(),
                    );
                }
            }

            // migui::panel(id, x, y, w, h, color)
            if (name == "migui::panel" || name == "__migui_panel") && args.len() == 6 {
                if let (
                    Valor::Texto(id),
                    Valor::Num(x),
                    Valor::Num(y),
                    Valor::Num(w),
                    Valor::Num(h),
                    Valor::Texto(color_str),
                ) = (
                    evaluar_expr_migui(
                        &args[0],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[1],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[2],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[3],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[4],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[5],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                ) {
                    let color = MiguiColor::from_str(&color_str).unwrap_or(MiguiColor::PANEL);
                    gui.panel(
                        WidgetId::new(&id),
                        Rect::new(x as f32, y as f32, w as f32, h as f32),
                        color,
                    );
                    return Valor::Vacio;
                } else {
                    return Valor::Error(
                        "migui::panel() requiere (id, x, y, w, h, color)".to_string(),
                    );
                }
            }

            // migui::textbox(id, x, y, w, h) -> String
            if (name == "migui::textbox" || name == "__migui_textbox") && args.len() == 5 {
                if let (
                    Valor::Texto(id),
                    Valor::Num(x),
                    Valor::Num(y),
                    Valor::Num(w),
                    Valor::Num(h),
                ) = (
                    evaluar_expr_migui(
                        &args[0],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[1],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[2],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[3],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[4],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                ) {
                    gui.set_textbox_text(&id, String::new());
                    gui.textbox(
                        WidgetId::new(&id),
                        Rect::new(x as f32, y as f32, w as f32, h as f32),
                    );
                    return Valor::Texto(
                        gui.textbox_states
                            .get(&id)
                            .map(|s| s.text.clone())
                            .unwrap_or_default(),
                    );
                } else {
                    return Valor::Error("migui::textbox() requiere (id, x, y, w, h)".to_string());
                }
            }

            // migui::window(id, title, open, x, y, w, h) -> bool
            if (name == "migui::window" || name == "__migui_window") && args.len() == 7 {
                if let (
                    Valor::Texto(id),
                    Valor::Texto(title),
                    Valor::Bool(open),
                    Valor::Num(x),
                    Valor::Num(y),
                    Valor::Num(w),
                    Valor::Num(h),
                ) = (
                    evaluar_expr_migui(
                        &args[0],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[1],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[2],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[3],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[4],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[5],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[6],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                ) {
                    let state = window_states.entry(id.clone()).or_insert(open);
                    let is_open = gui.window(
                        WidgetId::new(&id),
                        &title,
                        Rect::new(x as f32, y as f32, w as f32, h as f32),
                        state,
                    );
                    return Valor::Bool(is_open);
                } else {
                    return Valor::Error(
                        "migui::window() requiere (id, title, open, x, y, w, h)".to_string(),
                    );
                }
            }

            // migui::message_box(title, message, buttons, x, y, w, h) -> i32
            if (name == "migui::message_box" || name == "__migui_message_box") && args.len() == 7 {
                if let (
                    Valor::Texto(title),
                    Valor::Texto(message),
                    Valor::Array(buttons_arr),
                    Valor::Num(x),
                    Valor::Num(y),
                    Valor::Num(w),
                    Valor::Num(h),
                ) = (
                    evaluar_expr_migui(
                        &args[0],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[1],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[2],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[3],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[4],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[5],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                    evaluar_expr_migui(
                        &args[6],
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ),
                ) {
                    let buttons: Vec<&str> = buttons_arr
                        .iter()
                        .filter_map(|v| {
                            if let Valor::Texto(s) = v {
                                Some(s.as_str())
                            } else {
                                None
                            }
                        })
                        .collect();
                    let result = gui.message_box(
                        &title,
                        &message,
                        &buttons,
                        Rect::new(x as f32, y as f32, w as f32, h as f32),
                    );
                    return Valor::Num(result as f64);
                } else {
                    return Valor::Error(
                        "migui::message_box() requiere (title, message, buttons, x, y, w, h)"
                            .to_string(),
                    );
                }
            }

            // migui::mouse_x() -> f64
            if name == "migui::mouse_x" || name == "__migui_mouse_x" {
                return Valor::Num(gui.mouse_x() as f64);
            }

            // migui::mouse_y() -> f64
            if name == "migui::mouse_y" || name == "__migui_mouse_y" {
                return Valor::Num(gui.mouse_y() as f64);
            }

            // migui::mouse_position() -> [x, y]
            if name == "migui::mouse_position" || name == "__migui_mouse_position" {
                let (x, y) = gui.mouse_position();
                return Valor::Array(vec![Valor::Num(x as f64), Valor::Num(y as f64)]);
            }

            // migui::is_mouse_button_pressed() -> bool
            if name == "migui::is_mouse_button_pressed" || name == "__migui_is_mouse_button_pressed"
            {
                return Valor::Bool(gui.is_mouse_pressed());
            }

            // Funciones definidas por el usuario
            let func_name = if name.contains("::") {
                if funcs.contains_key(name) {
                    name.clone()
                } else {
                    name.split("::").last().unwrap_or(name).to_string()
                }
            } else {
                name.clone()
            };

            let func_data = funcs.get(&func_name).map(|(p, b)| (p.clone(), b.clone()));

            if let Some((params, body)) = func_data {
                let mut arg_values = vec![];
                for arg in args {
                    arg_values.push(evaluar_expr_migui(
                        arg,
                        executor,
                        gui,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                        funcs,
                    ));
                }

                executor.push_scope();
                for (i, param) in params.iter().enumerate() {
                    if i < arg_values.len() {
                        executor.guardar_local(param, arg_values[i].clone());
                    }
                }

                let mut empty_loaded: HashSet<String> = HashSet::new();
                let mut empty_stack: Vec<String> = Vec::new();

                let mut return_value: Option<Valor> = None;
                for s in &body {
                    match ejecutar_stmt_migui(
                        s,
                        executor,
                        funcs,
                        gui,
                        &mut empty_loaded,
                        &mut empty_stack,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                    ) {
                        (Some(true), _) => {
                            executor.pop_scope();
                            return Valor::Error(
                                "Break no permitido en función de expresión".to_string(),
                            );
                        }
                        (_, Some(val)) => {
                            return_value = Some(val);
                            break;
                        }
                        _ => {}
                    }
                }

                executor.pop_scope();
                return return_value.unwrap_or(Valor::Vacio);
            }

            Valor::Error(format!("Función '{}' no soportada en expresiones", name))
        }
        Expr::BinOp { left, op, right } => {
            let left_val = evaluar_expr_migui(
                left,
                executor,
                gui,
                checkbox_states,
                slider_states,
                textbox_states,
                window_states,
                funcs,
            );
            let right_val = evaluar_expr_migui(
                right,
                executor,
                gui,
                checkbox_states,
                slider_states,
                textbox_states,
                window_states,
                funcs,
            );

            match op {
                lizer::BinOp::And => {
                    let l_bool = valor_a_bool(&left_val);
                    let r_bool = valor_a_bool(&right_val);
                    return Valor::Bool(l_bool && r_bool);
                }
                lizer::BinOp::Or => {
                    let l_bool = valor_a_bool(&left_val);
                    let r_bool = valor_a_bool(&right_val);
                    return Valor::Bool(l_bool || r_bool);
                }
                _ => {}
            }

            if matches!(op, lizer::BinOp::Suma) {
                match (&left_val, &right_val) {
                    (Valor::Texto(l), Valor::Texto(r)) => {
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Texto(l), Valor::Num(r)) => {
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Num(l), Valor::Texto(r)) => {
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Num(l), Valor::Num(r)) => {
                        return Valor::Num(l + r);
                    }
                    _ => {}
                }
            }

            if let (Valor::Num(l), Valor::Num(r)) = (&left_val, &right_val) {
                match op {
                    lizer::BinOp::Suma => return Valor::Num(l + r),
                    lizer::BinOp::Resta => return Valor::Num(l - r),
                    lizer::BinOp::Mult => return Valor::Num(l * r),
                    lizer::BinOp::Div => {
                        if *r != 0.0 {
                            return Valor::Num(l / r);
                        } else {
                            return Valor::Error("División por cero".to_string());
                        }
                    }
                    // Mod y Exp no existen en lizer::BinOp
                    lizer::BinOp::Igual => return Valor::Bool((l - r).abs() < f64::EPSILON),
                    lizer::BinOp::Menor => return Valor::Bool(l < r),
                    lizer::BinOp::MenorIgual => return Valor::Bool(l <= r),
                    lizer::BinOp::Mayor => return Valor::Bool(l > r),
                    lizer::BinOp::MayorIgual => return Valor::Bool(l >= r),
                    _ => {}
                }
            }

            match (&left_val, &right_val) {
                (Valor::Texto(l), Valor::Texto(r)) => {
                    if op == &lizer::BinOp::Igual {
                        return Valor::Bool(l == r);
                    }
                }
                (Valor::Bool(l), Valor::Bool(r)) => {
                    if op == &lizer::BinOp::Igual {
                        return Valor::Bool(l == r);
                    }
                }
                _ => {}
            }

            Valor::Error(format!(
                "Operación no soportada: {:?} entre {:?} y {:?}",
                op, left_val, right_val
            ))
        }
        Expr::Unary { op, expr: inner } => {
            let val = evaluar_expr_migui(
                inner,
                executor,
                gui,
                checkbox_states,
                slider_states,
                textbox_states,
                window_states,
                funcs,
            );
            match op {
                lizer::UnaryOp::Neg => {
                    if let Valor::Num(n) = val {
                        Valor::Num(-n)
                    } else {
                        Valor::Error("Unary - requires number".to_string())
                    }
                }
                lizer::UnaryOp::Not => {
                    let b = valor_a_bool(&val);
                    Valor::Bool(!b)
                }
            }
        }
    }
}

// EJECUTAR STATEMENT MODO MIGUI

/// Ejecutar statement en modo migui
pub fn ejecutar_stmt_migui(
    stmt: &Stmt,
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
    gui: &mut Migui,
    loaded_modules: &mut HashSet<String>,
    importing_stack: &mut Vec<String>,
    checkbox_states: &mut HashMap<String, bool>,
    slider_states: &mut HashMap<String, f32>,
    textbox_states: &mut HashMap<String, String>,
    window_states: &mut HashMap<String, bool>,
) -> (Option<bool>, Option<Valor>) {
    match stmt {
        Stmt::Init => {}
        Stmt::Command(cmd) => {
            executor.ejecutar(cmd);
        }
        Stmt::Assign { name, value } => {
            let valor = evaluar_expr_migui(
                value,
                executor,
                gui,
                checkbox_states,
                slider_states,
                textbox_states,
                window_states,
                funcs,
            );
            executor.guardar(name, valor);
        }
        Stmt::IndexAssign {
            array,
            index,
            value,
        } => {
            let index_val = evaluar_expr_migui(
                index,
                executor,
                gui,
                checkbox_states,
                slider_states,
                textbox_states,
                window_states,
                funcs,
            );
            let valor = evaluar_expr_migui(
                value,
                executor,
                gui,
                checkbox_states,
                slider_states,
                textbox_states,
                window_states,
                funcs,
            );

            if let Some(Valor::Array(arr)) = executor.leer(array) {
                let idx = match index_val {
                    Valor::Num(n) => n as usize,
                    _ => {
                        println!("[ERROR] Índice debe ser número");
                        return (None, None);
                    }
                };

                if idx >= arr.len() {
                    println!(
                        "[ERROR] Índice {} fuera de límites (array de {} elementos)",
                        idx,
                        arr.len()
                    );
                    return (None, None);
                }

                let mut nuevo_arr = arr.clone();
                nuevo_arr[idx] = valor;
                executor.guardar(array, Valor::Array(nuevo_arr));
            } else {
                println!("[ERROR] '{}' no es un array", array);
            }
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let cond_val = evaluar_expr_migui(
                condition,
                executor,
                gui,
                checkbox_states,
                slider_states,
                textbox_states,
                window_states,
                funcs,
            );
            let es_verdad = match cond_val {
                Valor::Num(n) => n != 0.0,
                Valor::Bool(b) => b,
                _ => false,
            };

            if es_verdad {
                for s in then_body {
                    ejecutar_stmt_migui(
                        s,
                        executor,
                        funcs,
                        gui,
                        loaded_modules,
                        importing_stack,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                    );
                }
            } else if let Some(else_stmts) = else_body {
                for s in else_stmts {
                    ejecutar_stmt_migui(
                        s,
                        executor,
                        funcs,
                        gui,
                        loaded_modules,
                        importing_stack,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                    );
                }
            }
        }
        Stmt::While { condition, body } => {
            let mut iterations = 0;
            while iterations < 10 {
                let cond_val = evaluar_expr_migui(
                    condition,
                    executor,
                    gui,
                    checkbox_states,
                    slider_states,
                    textbox_states,
                    window_states,
                    funcs,
                );
                let es_verdad = match cond_val {
                    Valor::Num(n) => n != 0.0,
                    Valor::Bool(b) => b,
                    _ => false,
                };

                if !es_verdad {
                    break;
                }

                for s in body {
                    ejecutar_stmt_migui(
                        s,
                        executor,
                        funcs,
                        gui,
                        loaded_modules,
                        importing_stack,
                        checkbox_states,
                        slider_states,
                        textbox_states,
                        window_states,
                    );
                }
                iterations += 1;
            }
        }
        Stmt::ForEach {
            var,
            iterable,
            body,
        } => {
            let iterable_val = evaluar_expr_migui(
                iterable,
                executor,
                gui,
                checkbox_states,
                slider_states,
                textbox_states,
                window_states,
                funcs,
            );
            if let Valor::Array(arr) = iterable_val {
                for item in arr {
                    executor.guardar(var, item.clone());
                    for s in body {
                        ejecutar_stmt_migui(
                            s,
                            executor,
                            funcs,
                            gui,
                            loaded_modules,
                            importing_stack,
                            checkbox_states,
                            slider_states,
                            textbox_states,
                            window_states,
                        );
                    }
                }
            } else {
                println!("[ERROR] 'cada' requiere un array");
            }
        }
        Stmt::Block(stmts) => {
            println!(
                "[MIGUI DEBUG] Ejecutando bloque con {} statements",
                stmts.len()
            );
            for (i, s) in stmts.iter().enumerate() {
                println!(
                    "[MIGUI DEBUG] Statement {}: {:?}",
                    i,
                    match s {
                        Stmt::Expr(_) => "Expr",
                        Stmt::Assign { .. } => "Assign",
                        Stmt::Call { .. } => "Call",
                        Stmt::Function { .. } => "Function",
                        _ => "Other",
                    }
                );
                ejecutar_stmt_migui(
                    s,
                    executor,
                    funcs,
                    gui,
                    loaded_modules,
                    importing_stack,
                    checkbox_states,
                    slider_states,
                    textbox_states,
                    window_states,
                );
            }
        }
        Stmt::Function { name, params, body } => {
            funcs.insert(name.clone(), (params.clone(), body.clone()));
        }
        Stmt::Call { name, args } => {
            // Para migui, evaluar como expresión (las funciones migui generan draw commands)
            let _ = evaluar_expr_migui(
                &Expr::Call {
                    name: name.clone(),
                    args: args.clone(),
                },
                executor,
                gui,
                checkbox_states,
                slider_states,
                textbox_states,
                window_states,
                funcs,
            );
            return (None, None);
        }
        Stmt::Import { module, alias } => {
            let module_path = format!("crates/modules/{}.rydit", module);

            if importing_stack.contains(module) {
                println!("[ERROR] Importe cíclico detectado: '{}'", module);
                return (None, None);
            }

            if loaded_modules.contains(module.as_str()) {
                let prefix = if let Some(alias_name) = alias {
                    alias_name.clone()
                } else {
                    module.clone()
                };

                let mut funcs_to_copy: Vec<(String, String)> = Vec::new();
                for (func_name, _) in funcs.iter() {
                    if func_name.starts_with(&format!("{}::", module)) {
                        let orig_name = func_name.strip_prefix(&format!("{}::", module)).unwrap();
                        let new_name = format!("{}::{}", prefix, orig_name);
                        funcs_to_copy.push((func_name.clone(), new_name));
                    }
                }

                for (old_name, new_name) in funcs_to_copy {
                    if let Some(func_data) = funcs.get(&old_name) {
                        funcs.insert(new_name, func_data.clone());
                    }
                }
                return (None, None);
            }

            if let Ok(content) = std::fs::read_to_string(&module_path) {
                importing_stack.push(module.clone());

                let tokens = Lizer::new(&content).scan();
                let mut parser = Parser::new(tokens);

                let program = match parser.parse() {
                    Ok(p) => p,
                    Err(e) => {
                        println!("[ERROR] Error parseando módulo '{}': {}", module, e);
                        importing_stack.pop();
                        return (None, None);
                    }
                };

                let mut original_funcs: Vec<String> = Vec::new();
                for s in &program.statements {
                    if let Stmt::Function { name, .. } = s {
                        original_funcs.push(name.clone());
                    }
                }

                for s in &program.statements {
                    match ejecutar_stmt(s, executor, funcs, loaded_modules, importing_stack) {
                        (Some(true), _) => {
                            println!("[ERROR] break no permitido en módulo '{}'", module);
                            break;
                        }
                        (_, Some(_)) => {
                            println!("[ERROR] return no permitido en módulo '{}'", module);
                            break;
                        }
                        _ => {}
                    }
                }

                importing_stack.pop();
                loaded_modules.insert(module.clone());

                let prefix = if let Some(alias_name) = alias {
                    alias_name.clone()
                } else {
                    module.clone()
                };

                for orig_name in &original_funcs {
                    if let Some(func_data) = funcs.get(orig_name) {
                        let new_name = format!("{}::{}", prefix, orig_name);
                        funcs.insert(new_name, func_data.clone());
                    }
                }

                if alias.is_none() {
                    for orig_name in &original_funcs {
                        funcs.remove(orig_name);
                    }
                }
            } else {
                println!("[ERROR] Módulo '{}' no encontrado", module);
            }
        }
        Stmt::Return(Some(val)) => {
            let valor = evaluar_expr_migui(
                val,
                executor,
                gui,
                checkbox_states,
                slider_states,
                textbox_states,
                window_states,
                funcs,
            );
            println!(
                "[RETURN] {}",
                match valor {
                    Valor::Num(n) => format!("{}", n),
                    Valor::Texto(s) => s,
                    Valor::Bool(b) => format!("{}", b),
                    _ => format!("{:?}", valor),
                }
            );
        }
        Stmt::Expr(expr) => {
            let valor = evaluar_expr_migui(
                expr,
                executor,
                gui,
                checkbox_states,
                slider_states,
                textbox_states,
                window_states,
                funcs,
            );
            executor.voz(&valor);
        }
        _ => {}
    }
    (None, None)
}

// Tests movidos a: crates/rydit-rs/src/tests/mod.rs
