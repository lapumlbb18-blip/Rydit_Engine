// crates/rydit-rs/src/modules/input_ime.rs
// Input Method Editor (IME) para teclado virtual de Android
// v0.9.2: Soporte para input de texto con teclado desplegable

use blast_core::{Executor, Valor};
use lizer::{Expr, Stmt};
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

// ============================================================================
// ESTADO DEL IME
// ============================================================================

/// Estado del IME (Input Method Editor)
pub struct IMEState {
    /// Si el teclado está visible
    pub visible: bool,
    /// Texto actual ingresado
    pub text: String,
    /// Cursor position
    pub cursor_pos: usize,
    /// Si hay texto nuevo disponible
    pub text_changed: bool,
}

impl IMEState {
    pub fn new() -> Self {
        Self {
            visible: false,
            text: String::new(),
            cursor_pos: 0,
            text_changed: false,
        }
    }

    /// Mostrar teclado virtual
    pub fn show_keyboard(&mut self) {
        self.visible = true;
        self.text.clear();
        self.cursor_pos = 0;
        self.text_changed = false;
        
        // NOTA: En Android real, aquí llamaríamos a JNI para mostrar el teclado
        // InputMethodManager.showSoftInput()
        eprintln!("[IME] Teclado virtual mostrado (simulado)");
    }

    /// Ocultar teclado virtual
    pub fn hide_keyboard(&mut self) {
        self.visible = false;
        eprintln!("[IME] Teclado virtual ocultado");
    }

    /// Ingresar carácter desde el teclado virtual (para uso futuro)
    #[allow(dead_code)]
    pub fn input_char(&mut self, c: char) {
        if self.cursor_pos < self.text.len() {
            self.text.insert(self.cursor_pos, c);
        } else {
            self.text.push(c);
        }
        self.cursor_pos += 1;
        self.text_changed = true;
    }

    /// Borrar último carácter (backspace) (para uso futuro)
    #[allow(dead_code)]
    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 && !self.text.is_empty() {
            self.cursor_pos -= 1;
            self.text.remove(self.cursor_pos);
            self.text_changed = true;
        }
    }

    /// Confirmar texto (Enter) (para uso futuro)
    #[allow(dead_code)]
    pub fn confirm(&mut self) {
        self.hide_keyboard();
        self.text_changed = false;
    }

    /// Obtener texto actual
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /// Verificar si hay texto nuevo
    pub fn has_new_text(&self) -> bool {
        self.text_changed
    }

    /// Resetear estado de texto nuevo (para uso futuro)
    #[allow(dead_code)]
    pub fn reset_text_flag(&mut self) {
        self.text_changed = false;
    }
}

impl Default for IMEState {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ESTADO GLOBAL
// ============================================================================

thread_local! {
    static IME: Rc<RefCell<IMEState>> = Rc::new(RefCell::new(IMEState::new()));
}

/// Obtener referencia al IME global
pub fn get_ime() -> Rc<RefCell<IMEState>> {
    IME.with(|m| m.clone())
}

// ============================================================================
// FUNCIONES PARA RYDIT
// ============================================================================

/// input::show_keyboard() - Mostrar teclado virtual
pub fn input_show_keyboard(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let ime = get_ime();
    let mut ime_ref = ime.borrow_mut();
    ime_ref.show_keyboard();
    Valor::Texto("Teclado virtual mostrado".to_string())
}

/// input::hide_keyboard() - Ocultar teclado virtual
pub fn input_hide_keyboard(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let ime = get_ime();
    let mut ime_ref = ime.borrow_mut();
    ime_ref.hide_keyboard();
    Valor::Texto("Teclado virtual ocultado".to_string())
}

/// input::get_text() - Obtener texto ingresado
pub fn input_get_text(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let ime = get_ime();
    let ime_ref = ime.borrow();
    Valor::Texto(ime_ref.get_text().to_string())
}

/// input::has_text() - Verificar si hay texto nuevo
pub fn input_has_text(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let ime = get_ime();
    let ime_ref = ime.borrow();
    Valor::Bool(ime_ref.has_new_text())
}

/// input::clear_text() - Limpiar texto ingresado
pub fn input_clear_text(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let ime = get_ime();
    let mut ime_ref = ime.borrow_mut();
    ime_ref.text.clear();
    ime_ref.cursor_pos = 0;
    ime_ref.text_changed = false;
    Valor::Texto("Texto limpiado".to_string())
}

/// input::text(prompt) - Mostrar teclado y esperar input (bloqueante)
/// NOTA: Esta es una versión simplificada no bloqueante
pub fn input_text(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let prompt = if !args.is_empty() {
        use crate::eval::evaluar_expr;
        match evaluar_expr(&args[0], executor, funcs) {
            Valor::Texto(s) => s,
            _ => "Ingrese texto:".to_string(),
        }
    } else {
        "Ingrese texto:".to_string()
    };

    let ime = get_ime();
    let mut ime_ref = ime.borrow_mut();
    
    if !ime_ref.visible {
        // Primera llamada: mostrar teclado
        ime_ref.show_keyboard();
        executor.guardar("__IME_PROMPT", Valor::Texto(prompt));
        Valor::Texto("Teclado mostrado, esperando input...".to_string())
    } else {
        // Teclado ya visible: verificar si hay texto
        if ime_ref.text_changed {
            // Texto ingresado: ocultar teclado y retornar
            let text = ime_ref.text.clone();
            ime_ref.hide_keyboard();
            ime_ref.text_changed = false;
            Valor::Texto(text)
        } else {
            // Todavía esperando input
            Valor::Texto("".to_string())
        }
    }
}

/// input::is_keyboard_visible() - Verificar si el teclado está visible
pub fn input_is_keyboard_visible(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let ime = get_ime();
    let ime_ref = ime.borrow();
    Valor::Bool(ime_ref.visible)
}

// ============================================================================
// SIMULACIÓN DE INPUT PARA DEMOS
// ============================================================================

/// input::simulate_text(text) - Simular input de texto (para demos)
pub fn input_simulate_text(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;
    
    if args.is_empty() {
        return Valor::Error("input::simulate_text() requiere 1 argumento: text".to_string());
    }

    let text = match evaluar_expr(&args[0], executor, funcs) {
        Valor::Texto(s) => s,
        _ => return Valor::Error("El argumento debe ser texto".to_string()),
    };

    let ime = get_ime();
    let mut ime_ref = ime.borrow_mut();
    
    // Simular input carácter por carácter
    ime_ref.text = text.clone();
    ime_ref.cursor_pos = text.len();
    ime_ref.text_changed = true;
    
    Valor::Texto(format!("Texto simulado: {}", text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ime_state() {
        let mut ime = IMEState::new();
        assert!(!ime.visible);
        
        ime.show_keyboard();
        assert!(ime.visible);
        
        ime.input_char('H');
        ime.input_char('o');
        ime.input_char('l');
        ime.input_char('a');
        
        assert_eq!(ime.get_text(), "Hola");
        
        ime.backspace();
        assert_eq!(ime.get_text(), "Hol");
        
        ime.hide_keyboard();
        assert!(!ime.visible);
    }
}
