//! InputManager - Manager unificado que conecta las 3 capas
//!
//! El InputManager es el punto de entrada principal:
//! - Recibe eventos de un backend (SDL2, raylib, mock)
//! - Los pasa al TextInput para composición
//! - Los pasa al Shell para ejecución de comandos
//! - Expone eventos procesados al usuario

use crate::backend::InputBackend;
use crate::input_event::InputEvent;
use crate::key_code::Key;
use crate::text_input::{TextInput, TextInputAction};
use crate::shell::{Shell, ShellResult};

/// InputManager - Punto de entrada unificado
pub struct InputManager {
    /// Backend de input (SDL2, raylib, mock)
    backend: Box<dyn InputBackend>,
    /// TextInput para composición de strings
    text_input: TextInput,
    /// Shell para comandos
    shell: Shell,
    /// Buffer de eventos pendientes
    event_buffer: Vec<InputEvent>,
    /// Acciones de text input pendientes
    text_actions: Vec<TextInputAction>,
    /// Si el input de texto está activo
    text_input_active: bool,
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}

impl InputManager {
    /// Crear InputManager con backend mock (sin dependencias)
    pub fn new() -> Self {
        Self {
            backend: Box::new(crate::backend::MockBackend::new()),
            text_input: TextInput::new(),
            shell: Shell::with_defaults(),
            event_buffer: Vec::new(),
            text_actions: Vec::new(),
            text_input_active: false,
        }
    }

    /// Crear con backend personalizado
    pub fn with_backend<B: InputBackend + 'static>(backend: B) -> Self {
        Self {
            backend: Box::new(backend),
            text_input: TextInput::new(),
            shell: Shell::with_defaults(),
            event_buffer: Vec::new(),
            text_actions: Vec::new(),
            text_input_active: false,
        }
    }

    /// Inicializar backend
    pub fn init(&mut self) -> Result<(), String> {
        self.backend.init()
    }

    /// Obtener eventos raw del backend
    pub fn poll_raw_events(&mut self) -> Vec<InputEvent> {
        let backend_events = self.backend.poll_events();
        self.event_buffer.extend(backend_events.clone());
        backend_events
    }

    /// Inyectar un evento manualmente (para tests o backends custom)
    pub fn inject_event(&mut self, event: InputEvent) {
        // Procesar text input si está activo
        if self.text_input_active {
            if let Some(action) = self.text_input.process_event(&event) {
                self.text_actions.push(action);
            }
        }

        self.event_buffer.push(event);
    }

    /// Obtener eventos pendientes
    pub fn poll_events(&mut self) -> Vec<InputEvent> {
        std::mem::take(&mut self.event_buffer)
    }

    /// Obtener acciones de text input pendientes
    pub fn poll_text_actions(&mut self) -> Vec<TextInputAction> {
        std::mem::take(&mut self.text_actions)
    }

    // ========================================================================
    // TEXT INPUT
    // ========================================================================

    /// Activar input de texto (para editores, consola, etc.)
    pub fn enable_text_input(&mut self) {
        self.text_input_active = true;
        self.text_input.begin_composition();
        self.backend.enable_text_input();
    }

    /// Desactivar input de texto
    pub fn disable_text_input(&mut self) {
        self.text_input_active = false;
        self.text_input.cancel();
        self.backend.disable_text_input();
    }

    /// Verificar si text input está activo
    pub fn is_text_input_active(&self) -> bool {
        self.text_input_active
    }

    /// Obtener referencia al TextInput
    pub fn text_input(&self) -> &TextInput {
        &self.text_input
    }

    /// Obtener referencia mutable al TextInput
    pub fn text_input_mut(&mut self) -> &mut TextInput {
        &mut self.text_input
    }

    // ========================================================================
    // SHELL
    // ========================================================================

    /// Ejecutar un comando en el shell
    pub fn execute_command(&mut self, input: &str) -> ShellResult {
        self.shell.execute(input)
    }

    /// Obtener referencia al shell
    pub fn shell(&self) -> &Shell {
        &self.shell
    }

    /// Obtener referencia mutable al shell
    pub fn shell_mut(&mut self) -> &mut Shell {
        &mut self.shell
    }

    // ========================================================================
    // INPUT DIRECTO (passthrough al backend)
    // ========================================================================

    /// Verificar si una tecla está presionada ahora
    pub fn is_key_down(&self, key: Key) -> bool {
        self.backend.is_key_down(key)
    }

    /// Verificar si una tecla fue presionada este frame
    pub fn is_key_just_pressed(&self, key: Key) -> bool {
        self.backend.is_key_just_pressed(key)
    }

    /// Obtener posición del mouse
    pub fn mouse_position(&self) -> (i32, i32) {
        self.backend.mouse_position()
    }

    /// Verificar botón del mouse
    pub fn is_mouse_button_down(&self, button: crate::input_event::MouseButton) -> bool {
        self.backend.is_mouse_button_down(button)
    }

    // ========================================================================
    // SHUTDOWN
    // ========================================================================

    /// Cerrar y liberar recursos
    pub fn shutdown(&mut self) {
        self.backend.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::MockBackend;

    #[test]
    fn test_create_default() {
        let manager = InputManager::new();
        assert!(!manager.is_text_input_active());
    }

    #[test]
    fn test_inject_and_poll_events() {
        let mut manager = InputManager::new();
        manager.inject_event(InputEvent::KeyPressed { key: Key::A });
        manager.inject_event(InputEvent::KeyPressed { key: Key::B });

        let events = manager.poll_events();
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn test_text_input_activation() {
        let mut manager = InputManager::new();
        manager.enable_text_input();
        assert!(manager.is_text_input_active());

        manager.disable_text_input();
        assert!(!manager.is_text_input_active());
    }

    #[test]
    fn test_text_input_while_active() {
        let mut manager = InputManager::new();
        manager.enable_text_input();

        manager.inject_event(InputEvent::CharTyped { ch: 'h' });
        manager.inject_event(InputEvent::CharTyped { ch: 'o' });

        let actions = manager.poll_text_actions();
        assert_eq!(actions.len(), 2);
        assert_eq!(manager.text_input().current_text(), "ho");
    }

    #[test]
    fn test_shell_command() {
        let mut manager = InputManager::new();
        let result = manager.execute_command("echo hello");
        assert!(result.success);
        assert_eq!(result.output, "hello");
    }

    #[test]
    fn test_full_text_input_commit_flow() {
        let mut manager = InputManager::new();
        manager.enable_text_input();

        // Tipear "cmd"
        manager.inject_event(InputEvent::CharTyped { ch: 'c' });
        manager.inject_event(InputEvent::CharTyped { ch: 'm' });
        manager.inject_event(InputEvent::CharTyped { ch: 'd' });

        // Enter para commit
        manager.inject_event(InputEvent::KeyPressed { key: Key::Enter });

        // Obtener acciones
        let actions = manager.poll_text_actions();

        // La última acción debe ser el commit
        let commit_action = actions.iter().rev().find(|a| matches!(a, TextInputAction::Committed(_)));
        assert!(commit_action.is_some());
        if let Some(TextInputAction::Committed(text)) = commit_action {
            assert_eq!(text, "cmd");
        }
    }

    #[test]
    fn test_with_custom_backend() {
        let mut backend = MockBackend::new();
        backend.inject(InputEvent::KeyPressed { key: Key::Space });

        let mut manager = InputManager::with_backend(backend);
        let events = manager.poll_raw_events();
        assert_eq!(events.len(), 1);
    }
}
