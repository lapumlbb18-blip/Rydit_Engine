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

/// Estado de teclas presionadas
pub struct InputMapState {
    pub volumen_up: bool,
    pub volumen_down: bool,
    pub teclas_presionadas: HashMap<String, bool>,
    pub combinaciones: HashMap<String, String>,
}

impl InputMapState {
    pub fn new() -> Self {
        let mut state = Self {
            volumen_up: false,
            volumen_down: false,
            teclas_presionadas: HashMap::new(),
            combinaciones: HashMap::new(),
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
        self.combinaciones.insert(combinacion.to_string(), accion.to_string());
    }
    
    /// Verificar si una tecla de acción está presionada
    pub fn is_accion_presionada(&self, accion: &str, tecla_base: &str, volumen_up: bool) -> bool {
        // Verificar combinación con VolUP
        if volumen_up {
            let combinacion_key = format!("volup_{}", tecla_base.to_lowercase());
            if let Some(accion_mapeada) = self.combinaciones.get(&combinacion_key) {
                return accion_mapeada == accion;
            }
        }
        
        // Verificar tecla directa
        tecla_base == accion
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
        return Valor::Error("input_map::register() requiere 2 argumentos: combo, action".to_string());
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

    Valor::Texto(format!("input_map::register() - '{}' => '{}'", combo, action))
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

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_map_new() {
        let map = InputMapState::new();
        assert!(map.combinaciones.len() > 0);
    }

    #[test]
    fn test_input_map_register() {
        let mut map = InputMapState::new();
        map.registrar_combinacion("test_combo", "test_action");
        assert!(map.combinaciones.contains_key("test_combo"));
    }

    #[test]
    fn test_input_map_functions() {
        let _ = input_map_register;
        let _ = input_map_list;
        let _ = input_map_clear;
        let _ = input_map_count;
    }
}
