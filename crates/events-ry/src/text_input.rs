//! TextInput - Composición de strings (Capa 2)
//!
//! Maneja la composición de texto desde eventos raw:
//! - SDL_TEXTINPUT / IME composition
//! - Backspace, delete, cursor movement
//! - Commit/cancel del texto compuesto
//!
//! ## Flujo de composición
//!
//! ```text
//! CompositionStart → add_char('h') → add_char('o') → commit() → "ho"
//! CompositionStart → add_char('h') → cancel() → ""
//! ```

use crate::InputEvent;

/// Estado del compositor de texto
#[derive(Debug, Clone, PartialEq)]
enum CompositionState {
    /// Sin composición activa
    Idle,
    /// Composición en progreso
    Composing {
        /// Texto acumulado hasta ahora
        buffer: String,
        /// Cursor posición dentro del buffer
        cursor: usize,
    },
}

/// TextInput - Manager de composición de texto
///
/// Recibe eventos raw y produce strings completos.
#[derive(Debug, Clone)]
pub struct TextInput {
    state: CompositionState,
    /// Máximo largo de string (0 = ilimitado)
    max_length: usize,
    /// Historial de strings commits
    history: Vec<String>,
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}

impl TextInput {
    /// Crear nuevo TextInput
    pub fn new() -> Self {
        Self {
            state: CompositionState::Idle,
            max_length: 0,
            history: Vec::new(),
        }
    }

    /// Crear con límite de largo
    pub fn with_max_length(max: usize) -> Self {
        Self {
            max_length: max,
            ..Self::new()
        }
    }

    /// Iniciar composición (CompositionStart event)
    pub fn begin_composition(&mut self) {
        self.state = CompositionState::Composing {
            buffer: String::with_capacity(64),
            cursor: 0,
        };
    }

    /// Agregar carácter a la composición
    pub fn add_char(&mut self, ch: char) {
        if let CompositionState::Composing { buffer, cursor } = &mut self.state {
            if self.max_length > 0 && buffer.chars().count() >= self.max_length {
                return; // Límite alcanzado
            }

            // Insertar en posición del cursor
            let char_idx = buffer
                .char_indices()
                .nth(*cursor)
                .map(|(i, _)| i)
                .unwrap_or(buffer.len());
            buffer.insert(char_idx, ch);
            *cursor += 1;
        }
    }

    /// Borrar carácter antes del cursor (backspace)
    pub fn backspace(&mut self) -> Option<char> {
        if let CompositionState::Composing { buffer, cursor } = &mut self.state {
            if *cursor == 0 || buffer.is_empty() {
                return None;
            }

            // Encontrar el carácter antes del cursor
            let char_idx = buffer
                .char_indices()
                .nth(cursor.wrapping_sub(1))
                .map(|(i, _)| i)
                .unwrap_or(buffer.len());

            let ch = buffer.remove(char_idx);
            *cursor -= 1;
            Some(ch)
        } else {
            None
        }
    }

    /// Borrar carácter en el cursor (delete)
    pub fn delete(&mut self) -> Option<char> {
        if let CompositionState::Composing { buffer, cursor } = &mut self.state {
            let char_idx = buffer
                .char_indices()
                .nth(*cursor)
                .map(|(i, _)| i);

            if let Some(idx) = char_idx {
                let ch = buffer.remove(idx);
                Some(ch)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Mover cursor a la izquierda
    pub fn cursor_left(&mut self) {
        if let CompositionState::Composing { cursor, buffer: _ } = &mut self.state {
            if *cursor > 0 {
                *cursor -= 1;
            }
        }
    }

    /// Mover cursor a la derecha
    pub fn cursor_right(&mut self) {
        if let CompositionState::Composing { cursor, buffer } = &mut self.state {
            let len = buffer.chars().count();
            if *cursor < len {
                *cursor += 1;
            }
        }
    }

    /// Commit del texto (confirmar string)
    pub fn commit(&mut self) -> String {
        if let CompositionState::Composing { buffer, cursor: _ } =
            std::mem::replace(&mut self.state, CompositionState::Idle)
        {
            let text = buffer;
            if !text.is_empty() {
                self.history.push(text.clone());
            }
            text
        } else {
            String::new()
        }
    }

    /// Cancelar composición (descartar texto)
    pub fn cancel(&mut self) -> String {
        if let CompositionState::Composing { buffer, .. } =
            std::mem::replace(&mut self.state, CompositionState::Idle)
        {
            buffer
        } else {
            String::new()
        }
    }

    /// Obtener texto actual sin cancelar
    pub fn current_text(&self) -> &str {
        match &self.state {
            CompositionState::Composing { buffer, .. } => buffer.as_str(),
            CompositionState::Idle => "",
        }
    }

    /// Obtener posición del cursor
    pub fn cursor_position(&self) -> usize {
        match &self.state {
            CompositionState::Composing { cursor, .. } => *cursor,
            CompositionState::Idle => 0,
        }
    }

    /// Verificar si está componiendo
    pub fn is_composing(&self) -> bool {
        matches!(&self.state, CompositionState::Composing { .. })
    }

    /// Procesar un evento raw automáticamente
    pub fn process_event(&mut self, event: &InputEvent) -> Option<TextInputAction> {
        match event {
            InputEvent::CompositionStart => {
                self.begin_composition();
                Some(TextInputAction::CompositionStarted)
            }
            InputEvent::CompositionEnd => {
                let text = self.commit();
                Some(TextInputAction::Committed(text))
            }
            InputEvent::CharTyped { ch } => {
                if self.is_composing() {
                    self.add_char(*ch);
                    Some(TextInputAction::CharAdded(*ch))
                } else {
                    // Auto-begin si llega char sin composition
                    self.begin_composition();
                    self.add_char(*ch);
                    Some(TextInputAction::CharAdded(*ch))
                }
            }
            InputEvent::KeyPressed { key } => {
                use crate::Key;
                match key {
                    Key::Backspace => {
                        self.backspace();
                        Some(TextInputAction::CharDeleted)
                    }
                    Key::Delete => {
                        self.delete();
                        Some(TextInputAction::CharDeleted)
                    }
                    Key::Left => {
                        self.cursor_left();
                        Some(TextInputAction::CursorMoved)
                    }
                    Key::Right => {
                        self.cursor_right();
                        Some(TextInputAction::CursorMoved)
                    }
                    Key::Enter => {
                        let text = self.commit();
                        Some(TextInputAction::Committed(text))
                    }
                    Key::Escape => {
                        let text = self.cancel();
                        Some(TextInputAction::Cancelled(text))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Historial de commits (para undo/redo)
    pub fn history(&self) -> &[String] {
        &self.history
    }

    /// Último commit
    pub fn last_commit(&self) -> Option<&str> {
        self.history.last().map(|s| s.as_str())
    }

    /// Limpiar historial
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

/// Acción resultante de procesar input de texto
#[derive(Debug, Clone, PartialEq)]
pub enum TextInputAction {
    /// Composición iniciada
    CompositionStarted,
    /// Carácter agregado
    CharAdded(char),
    /// Carácter borrado
    CharDeleted,
    /// Cursor movido
    CursorMoved,
    /// Texto confirmado (commit)
    Committed(String),
    /// Composición cancelada
    Cancelled(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_typing() {
        let mut input = TextInput::new();
        input.begin_composition();
        input.add_char('h');
        input.add_char('o');
        input.add_char('l');
        input.add_char('a');
        assert_eq!(input.current_text(), "hola");

        let result = input.commit();
        assert_eq!(result, "hola");
    }

    #[test]
    fn test_backspace() {
        let mut input = TextInput::new();
        input.begin_composition();
        input.add_char('h');
        input.add_char('o');
        input.add_char('l');
        assert_eq!(input.current_text(), "hol");

        input.backspace();
        assert_eq!(input.current_text(), "ho");
    }

    #[test]
    fn test_cancel() {
        let mut input = TextInput::new();
        input.begin_composition();
        input.add_char('t');
        input.add_char('e');
        input.add_char('s');
        input.add_char('t');

        let cancelled = input.cancel();
        assert_eq!(cancelled, "test");
        assert_eq!(input.current_text(), "");
    }

    #[test]
    fn test_max_length() {
        let mut input = TextInput::with_max_length(3);
        input.begin_composition();
        input.add_char('a');
        input.add_char('b');
        input.add_char('c');
        input.add_char('d'); // No debe entrar
        assert_eq!(input.current_text(), "abc");
    }

    #[test]
    fn test_cursor_movement() {
        let mut input = TextInput::new();
        input.begin_composition();
        input.add_char('a');
        input.add_char('b');
        input.add_char('c');
        assert_eq!(input.cursor_position(), 3);

        input.cursor_left();
        assert_eq!(input.cursor_position(), 2);

        input.add_char('X');
        assert_eq!(input.current_text(), "abXc");
        assert_eq!(input.cursor_position(), 3);
    }

    #[test]
    fn test_process_event() {
        let mut input = TextInput::new();

        let action = input.process_event(&InputEvent::CharTyped { ch: 'h' });
        assert!(matches!(action, Some(TextInputAction::CharAdded('h'))));

        let action = input.process_event(&InputEvent::KeyPressed {
            key: crate::Key::Enter,
        });
        assert!(matches!(action, Some(TextInputAction::Committed(ref s)) if s == "h"));
    }

    #[test]
    fn test_process_enter_escapes() {
        let mut input = TextInput::new();
        input.process_event(&InputEvent::CharTyped { ch: 'c' });
        input.process_event(&InputEvent::CharTyped { ch: 'm' });
        input.process_event(&InputEvent::CharTyped { ch: 'd' });

        let action = input.process_event(&InputEvent::KeyPressed {
            key: crate::Key::Enter,
        });

        assert!(matches!(action, Some(TextInputAction::Committed(ref s)) if s == "cmd"));
        assert!(!input.is_composing());
    }

    #[test]
    fn test_history() {
        let mut input = TextInput::new();

        input.begin_composition();
        input.add_char('c');
        input.add_char('m');
        input.add_char('d');
        input.commit();
        assert_eq!(input.last_commit(), Some("cmd"));

        input.begin_composition();
        input.add_char('l');
        input.add_char('s');
        input.commit();
        assert_eq!(input.history(), &["cmd".to_string(), "ls".to_string()]);
    }

    #[test]
    fn test_delete_at_cursor() {
        let mut input = TextInput::new();
        input.begin_composition();
        input.add_char('a');
        input.add_char('b');
        input.add_char('c');
        input.cursor_left(); // cursor en 2 (antes de 'c')
        input.delete();      // borra 'c'
        assert_eq!(input.current_text(), "ab");
    }
}
