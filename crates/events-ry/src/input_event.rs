//! Eventos de input raw (Capa 1)

use crate::Key;

/// Evento de input crudo - abstrae todos los backends
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum InputEvent {
    // --- Teclado ---
    /// Tecla presionada (se repite mientras se mantiene)
    KeyPressed { key: Key },
    /// Tecla liberada
    KeyReleased { key: Key },
    /// Carácter tipeado (resultado de composición IME/SDL_TEXTINPUT)
    CharTyped { ch: char },
    /// Inicio de composición IME (teclados virtuales Android)
    CompositionStart,
    /// Fin de composición IME
    CompositionEnd,

    // --- Mouse ---
    /// Mouse movido
    MouseMoved { x: i32, y: i32 },
    /// Botón presionado
    MousePressed { x: i32, y: i32, button: MouseButton },
    /// Botón liberado
    MouseReleased { x: i32, y: i32, button: MouseButton },
    /// Scroll (wheel)
    MouseWheel { delta_x: f32, delta_y: f32 },

    // --- Touch (Android/móvil) ---
    /// Toque iniciado
    TouchDown { id: u64, x: i32, y: i32 },
    /// Toque movido
    TouchMotion { id: u64, x: i32, y: i32 },
    /// Toque liberado
    TouchUp { id: u64, x: i32, y: i32 },

    // --- Gamepad ---
    /// Botón de gamepad presionado
    GamepadButtonPressed { button: GamepadButton, state: bool },
    /// Eje de gamepad movido
    GamepadAxisMoved { axis: GamepadAxis, value: f32 },

    // --- Sistema ---
    /// Ventana redimensionada
    WindowResized { width: i32, height: i32 },
    /// Ventana cerrándose
    WindowCloseRequested,
}

/// Botón del mouse
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    /// Botón adicional (índice)
    Extra(u8),
}

/// Botón de gamepad (mapeo estándar)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    FaceDown,    // A (Xbox) / Cross (PS)
    FaceRight,   // B (Xbox) / Circle (PS)
    FaceLeft,    // X (Xbox) / Square (PS)
    FaceUp,      // Y (Xbox) / Triangle (PS)
    LeftShoulder,
    RightShoulder,
    LeftStick,
    RightStick,
    Back,        // Select / Share
    Start,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}

/// Eje de gamepad (mapeo estándar)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    LeftTrigger,
    RightTrigger,
}

impl InputEvent {
    /// Crear evento KeyPressed
    pub fn key_pressed(key: Key) -> Self {
        InputEvent::KeyPressed { key }
    }

    /// Crear evento KeyReleased
    pub fn key_released(key: Key) -> Self {
        InputEvent::KeyReleased { key }
    }

    /// Crear evento CharTyped
    pub fn char_typed(ch: char) -> Self {
        InputEvent::CharTyped { ch }
    }

    /// Crear evento MouseMoved
    pub fn mouse_moved(x: i32, y: i32) -> Self {
        InputEvent::MouseMoved { x, y }
    }

    /// Crear evento MousePressed
    pub fn mouse_pressed(x: i32, y: i32, button: MouseButton) -> Self {
        InputEvent::MousePressed { x, y, button }
    }

    /// Verificar si es un evento de teclado
    pub fn is_keyboard(&self) -> bool {
        matches!(
            self,
            InputEvent::KeyPressed { .. }
                | InputEvent::KeyReleased { .. }
                | InputEvent::CharTyped { .. }
                | InputEvent::CompositionStart
                | InputEvent::CompositionEnd
        )
    }

    /// Verificar si es un evento de mouse
    pub fn is_mouse(&self) -> bool {
        matches!(
            self,
            InputEvent::MouseMoved { .. }
                | InputEvent::MousePressed { .. }
                | InputEvent::MouseReleased { .. }
                | InputEvent::MouseWheel { .. }
        )
    }

    /// Verificar si es un evento de touch
    pub fn is_touch(&self) -> bool {
        matches!(
            self,
            InputEvent::TouchDown { .. } | InputEvent::TouchMotion { .. } | InputEvent::TouchUp { .. }
        )
    }

    /// Verificar si es un evento de gamepad
    pub fn is_gamepad(&self) -> bool {
        matches!(
            self,
            InputEvent::GamepadButtonPressed { .. } | InputEvent::GamepadAxisMoved { .. }
        )
    }

    /// Verificar si es un evento de ventana
    pub fn is_window(&self) -> bool {
        matches!(
            self,
            InputEvent::WindowResized { .. } | InputEvent::WindowCloseRequested
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_event() {
        let ev = InputEvent::KeyPressed { key: Key::A };
        assert!(ev.is_keyboard());
        assert!(!ev.is_mouse());
    }

    #[test]
    fn test_mouse_event() {
        let ev = InputEvent::MouseMoved { x: 100, y: 200 };
        assert!(ev.is_mouse());
        assert!(!ev.is_keyboard());
    }

    #[test]
    fn test_char_typed() {
        let ev = InputEvent::CharTyped { ch: 'h' };
        assert!(ev.is_keyboard());
    }

    #[test]
    fn test_touch_event() {
        let ev = InputEvent::TouchDown { id: 0, x: 50, y: 50 };
        assert!(ev.is_touch());
    }

    #[test]
    fn test_gamepad_event() {
        let ev = InputEvent::GamepadButtonPressed {
            button: GamepadButton::FaceDown,
            state: true,
        };
        assert!(ev.is_gamepad());
    }
}
