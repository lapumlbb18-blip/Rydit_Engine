// crates/rydit-rs/src/modules/input_map.rs
// Input Map para Termux-X11 - Mapeo de teclado Android
//
// Combinaciones de Termux:
// - VolUP + W = Arrow Up
// - VolUP + A = Arrow Left
// - VolUP + S = Arrow Down
// - VolUP + D = Arrow Right
// - VolUP + E = Escape
// - VolUP + Q = Extra Keys
// - VolUP + T = Tab
// - VolUP + P = Page Up
// - VolUP + N = Page Down

use blast_core::Valor;
use std::collections::HashMap;

// ============================================================================
// ESTADO DEL INPUT MAP
// ============================================================================

/// Estado del Input Map - almacena combinaciones y estado de teclas
pub struct InputMapState {
    /// Combinaciones mapeadas: "volup_w" => "arrow_up"
    pub combinaciones: HashMap<String, String>,
    /// Teclas actualmente presionadas
    pub teclas_presionadas: HashMap<String, bool>,
    /// Estado de VolUP (tecla modificadora)
    pub volumen_up: bool,
}

impl InputMapState {
    pub fn new() -> Self {
        let mut state = Self {
            combinaciones: HashMap::new(),
            teclas_presionadas: HashMap::new(),
            volumen_up: false,
        };

        // Registrar combinaciones por defecto de Termux
        state.registrar_combinacion("volup_w", "arrow_up");
        state.registrar_combinacion("volup_a", "arrow_left");
        state.registrar_combinacion("volup_s", "arrow_down");
        state.registrar_combinacion("volup_d", "arrow_right");
        state.registrar_combinacion("volup_e", "escape");
        state.registrar_combinacion("volup_q", "extra_keys");
        state.registrar_combinacion("volup_t", "tab");
        state.registrar_combinacion("volup_p", "page_up");
        state.registrar_combinacion("volup_n", "page_down");
        state.registrar_combinacion("volup_k", "extra_keys");

        state
    }

    /// Registrar una combinación personalizada
    pub fn registrar_combinacion(&mut self, combinacion: &str, accion: &str) {
        self.combinaciones
            .insert(combinacion.to_string(), accion.to_string());
    }

    /// Presionar una tecla (actualiza el estado interno)
    pub fn press_key(&mut self, key: &str) {
        let key_lower = key.to_lowercase();

        // Verificar si es VolUP
        if key_lower == "volumen_up" || key_lower == "volup" {
            self.volumen_up = true;
        }

        // Marcar tecla como presionada
        self.teclas_presionadas.insert(key_lower, true);
    }

    /// Soltar una tecla (actualiza el estado interno)
    pub fn release_key(&mut self, key: &str) {
        let key_lower = key.to_lowercase();

        // Verificar si es VolUP
        if key_lower == "volumen_up" || key_lower == "volup" {
            self.volumen_up = false;
        }

        // Marcar tecla como no presionada
        self.teclas_presionadas.insert(key_lower, false);
    }

    /// Verificar si una tecla específica está presionada (sin mapeo)
    pub fn is_key_pressed(&self, key: &str) -> bool {
        let key_lower = key.to_lowercase();
        *self.teclas_presionadas.get(&key_lower).unwrap_or(&false)
    }

    /// Verificar si una acción está presionada (con mapeo de combinaciones)
    pub fn is_action_pressed(&self, accion: &str) -> bool {
        let accion_lower = accion.to_lowercase();

        // 1. Buscar combinaciones que mapeen a esta acción
        for (combo, mapped_accion) in &self.combinaciones {
            if mapped_accion.to_lowercase() == accion_lower {
                // Verificar si la combinación está activa
                if self.es_combinacion_activa(combo) {
                    return true;
                }
            }
        }

        // 2. Verificar si la tecla directa está presionada
        self.is_key_pressed(&accion_lower)
    }

    /// Verificar si una combinación específica está activa
    fn es_combinacion_activa(&self, combo: &str) -> bool {
        let combo_lower = combo.to_lowercase();

        // Combinaciones con VolUP
        if combo_lower.starts_with("volup_") {
            if !self.volumen_up {
                return false;
            }

            // Obtener tecla base (ej: "volup_w" -> "w")
            let tecla_base = combo_lower.strip_prefix("volup_").unwrap();
            return self.is_key_pressed(tecla_base);
        }

        // Combinación simple
        self.is_key_pressed(&combo_lower)
    }

    /// Obtener lista de todas las acciones activas actualmente
    pub fn get_active_actions(&self) -> Vec<String> {
        let mut acciones = Vec::new();

        for mapped_accion in self.combinaciones.values() {
            let accion_lower = mapped_accion.to_lowercase();
            if self.is_action_pressed(&accion_lower) && !acciones.contains(&accion_lower) {
                acciones.push(accion_lower);
            }
        }

        // También agregar teclas directas
        for (key, &pressed) in &self.teclas_presionadas {
            if pressed && !acciones.contains(key) {
                acciones.push(key.clone());
            }
        }

        acciones
    }
}

impl Default for InputMapState {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ESTADO GLOBAL
// ============================================================================

use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static INPUT_MAP: Rc<RefCell<InputMapState>> = Rc::new(RefCell::new(InputMapState::new()));
}

/// Obtener referencia al Input Map global
pub fn get_input_map() -> Rc<RefCell<InputMapState>> {
    INPUT_MAP.with(|m| m.clone())
}

// ============================================================================
// FUNCIONES PARA RYDIT
// ============================================================================

use blast_core::Executor;
use lizer::{Expr, Stmt};

/// input_map::register(combo, action) - Registrar combinación personalizada
pub fn input_map_register(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error(
            "input_map::register() requiere 2 argumentos: combo, action".to_string(),
        );
    }

    use crate::eval::evaluar_expr;

    let combo_val = evaluar_expr(&args[0], executor, _funcs);
    let action_val = evaluar_expr(&args[1], executor, _funcs);

    let combo = match combo_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("input_map::register() combo debe ser texto".to_string()),
    };

    let action = match action_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("input_map::register() action debe ser texto".to_string()),
    };

    let input_map = get_input_map();
    let mut map_ref = input_map.borrow_mut();
    map_ref.registrar_combinacion(&combo, &action);

    Valor::Texto(format!(
        "input_map::register() - '{}' => '{}'",
        combo, action
    ))
}

/// input_map::list() - Listar todas las combinaciones
pub fn input_map_list(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let input_map = get_input_map();
    let map_ref = input_map.borrow();

    let mut items = Vec::new();
    for (combo, action) in map_ref.combinaciones.iter() {
        items.push(Valor::Texto(format!("{} => {}", combo, action)));
    }

    Valor::Array(items)
}

/// input_map::clear() - Limpiar combinaciones personalizadas
pub fn input_map_clear(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let input_map = get_input_map();
    let mut map_ref = input_map.borrow_mut();
    map_ref.combinaciones.clear();

    // Restaurar defaults
    map_ref.registrar_combinacion("volup_w", "arrow_up");
    map_ref.registrar_combinacion("volup_a", "arrow_left");
    map_ref.registrar_combinacion("volup_s", "arrow_down");
    map_ref.registrar_combinacion("volup_d", "arrow_right");
    map_ref.registrar_combinacion("volup_e", "escape");
    map_ref.registrar_combinacion("volup_q", "extra_keys");
    map_ref.registrar_combinacion("volup_t", "tab");
    map_ref.registrar_combinacion("volup_p", "page_up");
    map_ref.registrar_combinacion("volup_n", "page_down");
    map_ref.registrar_combinacion("volup_k", "extra_keys");

    Valor::Texto("input_map::clear() - Combinaciones restauradas a default".to_string())
}

/// input_map::count() - Cantidad de combinaciones registradas
pub fn input_map_count(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let input_map = get_input_map();
    let map_ref = input_map.borrow();

    Valor::Num(map_ref.combinaciones.len() as f64)
}

/// input_map::press(key) - Registrar que una tecla fue presionada
pub fn input_map_press(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("input_map::press() requiere 1 argumento: key".to_string());
    }

    use crate::eval::evaluar_expr;

    let key_val = evaluar_expr(&args[0], executor, _funcs);

    let key = match key_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("input_map::press() key debe ser texto".to_string()),
    };

    let input_map = get_input_map();
    let mut map_ref = input_map.borrow_mut();
    map_ref.press_key(&key);

    Valor::Texto(format!("input_map::press() - '{}' presionada", key))
}

/// input_map::release(key) - Registrar que una tecla fue soltada
pub fn input_map_release(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("input_map::release() requiere 1 argumento: key".to_string());
    }

    use crate::eval::evaluar_expr;

    let key_val = evaluar_expr(&args[0], executor, _funcs);

    let key = match key_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("input_map::release() key debe ser texto".to_string()),
    };

    let input_map = get_input_map();
    let mut map_ref = input_map.borrow_mut();
    map_ref.release_key(&key);

    Valor::Texto(format!("input_map::release() - '{}' soltada", key))
}

/// input_map::is_pressed(action) - Verificar si una acción está presionada
pub fn input_map_is_pressed(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("input_map::is_pressed() requiere 1 argumento: action".to_string());
    }

    use crate::eval::evaluar_expr;

    let action_val = evaluar_expr(&args[0], executor, _funcs);

    let action = match action_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("input_map::is_pressed() action debe ser texto".to_string()),
    };

    let input_map = get_input_map();
    let map_ref = input_map.borrow();

    let presionada = map_ref.is_action_pressed(&action);

    Valor::Bool(presionada)
}

/// input_map::get_active() - Obtener lista de acciones activas
pub fn input_map_get_active(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let input_map = get_input_map();
    let map_ref = input_map.borrow();

    let acciones = map_ref.get_active_actions();
    let items: Vec<Valor> = acciones.into_iter().map(Valor::Texto).collect();

    Valor::Array(items)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_map_new() {
        let map = InputMapState::new();
        assert!(!map.combinaciones.is_empty());
    }

    #[test]
    fn test_input_map_register() {
        let mut map = InputMapState::new();
        map.registrar_combinacion("test_combo", "test_action");
        assert!(map.combinaciones.contains_key("test_combo"));
    }

    #[test]
    fn test_input_map_press_release() {
        let mut map = InputMapState::new();

        // Presionar tecla
        map.press_key("w");
        assert!(map.is_key_pressed("w"));

        // Soltar tecla
        map.release_key("w");
        assert!(!map.is_key_pressed("w"));
    }

    #[test]
    fn test_input_map_volup_combination() {
        let mut map = InputMapState::new();

        // Presionar VolUP + W debería activar arrow_up
        map.press_key("volup");
        map.press_key("w");

        assert!(map.is_action_pressed("arrow_up"));

        // Soltar VolUP
        map.release_key("volup");
        assert!(!map.is_action_pressed("arrow_up"));
    }

    #[test]
    fn test_input_map_direct_key() {
        let mut map = InputMapState::new();

        // Tecla directa sin combinación
        map.press_key("arrow_up");
        assert!(map.is_action_pressed("arrow_up"));

        map.release_key("arrow_up");
        assert!(!map.is_action_pressed("arrow_up"));
    }

    #[test]
    fn test_input_map_functions() {
        let _ = input_map_register;
        let _ = input_map_list;
        let _ = input_map_clear;
        let _ = input_map_count;
        let _ = input_map_press;
        let _ = input_map_release;
        let _ = input_map_is_pressed;
        let _ = input_map_get_active;
    }
}
