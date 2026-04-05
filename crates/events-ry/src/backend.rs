//! Trait de backend de input (abstrae SDL2, raylib, etc.)

use crate::InputEvent;

/// Trait que debe implementar cada backend de input
///
/// Un backend convierte eventos nativos de la plataforma en `InputEvent`.
pub trait InputBackend {
    /// Inicializar el backend
    fn init(&mut self) -> Result<(), String>;

    /// Obtener eventos pendientes (no bloqueante)
    fn poll_events(&mut self) -> Vec<InputEvent>;

    /// Activar input de texto (SDL_TEXTINPUT / equivalente)
    /// Necesario para teclados virtuales Android y composición IME
    fn enable_text_input(&mut self);

    /// Desactivar input de texto
    fn disable_text_input(&mut self);

    /// Verificar si una tecla está presionada ahora
    fn is_key_down(&self, key: crate::Key) -> bool;

    /// Verificar si una tecla fue presionada este frame (solo una vez)
    fn is_key_just_pressed(&self, key: crate::Key) -> bool;

    /// Obtener posición actual del mouse
    fn mouse_position(&self) -> (i32, i32);

    /// Verificar si un botón del mouse está presionado
    fn is_mouse_button_down(&self, button: crate::input_event::MouseButton) -> bool;

    /// Cerrar backend y liberar recursos
    fn shutdown(&mut self);
}

/// Backend mock para tests (sin dependencias externas)
#[derive(Default)]
pub struct MockBackend {
    queued_events: Vec<InputEvent>,
    text_input_enabled: bool,
}

impl MockBackend {
    pub fn new() -> Self {
        Self {
            queued_events: Vec::new(),
            text_input_enabled: false,
        }
    }

    /// Inyectar un evento manualmente (para tests)
    pub fn inject(&mut self, event: InputEvent) {
        self.queued_events.push(event);
    }
}

impl InputBackend for MockBackend {
    fn init(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn poll_events(&mut self) -> Vec<InputEvent> {
        std::mem::take(&mut self.queued_events)
    }

    fn enable_text_input(&mut self) {
        self.text_input_enabled = true;
    }

    fn disable_text_input(&mut self) {
        self.text_input_enabled = false;
    }

    fn is_key_down(&self, _key: crate::Key) -> bool {
        false
    }

    fn is_key_just_pressed(&self, _key: crate::Key) -> bool {
        false
    }

    fn mouse_position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn is_mouse_button_down(&self, _button: crate::input_event::MouseButton) -> bool {
        false
    }

    fn shutdown(&mut self) {}
}
