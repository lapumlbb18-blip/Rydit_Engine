//! Código de tecla unificado (abstrae SDL2 Keycode, raylib KeyboardKey, migui::Key)

use std::fmt;

/// Código de tecla maestro - unifica todos los backends
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Key {
    // --- Letras ---
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    // --- Números (fila superior) ---
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,

    // --- Función ---
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    // --- Especial ---
    Escape,
    Tab,
    Enter,
    Space,
    Backspace,
    Delete,
    Insert,

    // --- Navegación ---
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,

    // --- Modificadores ---
    LeftShift,
    RightShift,
    LeftCtrl,
    RightCtrl,
    LeftAlt,
    RightAlt,

    // --- Puntuación ---
    Comma,
    Period,
    Slash,
    Semicolon,
    Colon,
    Quote,
    DoubleQuote,
    LeftBracket,
    RightBracket,
    Backslash,
    Tilde,
    Minus,
    Plus,
    Equal,

    // --- Keypad ---
    Kp0, Kp1, Kp2, Kp3, Kp4, Kp5, Kp6, Kp7, Kp8, Kp9,
    KpEnter,
    KpPlus,
    KpMinus,
    KpMultiply,
    KpDivide,
    KpDecimal,

    // --- Sistema ---
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,

    // --- Media (gamepad/keyboard multimedia) ---
    VolumeUp,
    VolumeDown,
    Mute,
    Play,
    Stop,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Key {
    /// Convertir a string legible
    pub fn name(&self) -> &'static str {
        match self {
            Key::A => "A", Key::B => "B", Key::C => "C", Key::D => "D",
            Key::E => "E", Key::F => "F", Key::G => "G", Key::H => "H",
            Key::I => "I", Key::J => "J", Key::K => "K", Key::L => "L",
            Key::M => "M", Key::N => "N", Key::O => "O", Key::P => "P",
            Key::Q => "Q", Key::R => "R", Key::S => "S", Key::T => "T",
            Key::U => "U", Key::V => "V", Key::W => "W", Key::X => "X",
            Key::Y => "Y", Key::Z => "Z",
            Key::Num0 => "0", Key::Num1 => "1", Key::Num2 => "2",
            Key::Num3 => "3", Key::Num4 => "4", Key::Num5 => "5",
            Key::Num6 => "6", Key::Num7 => "7", Key::Num8 => "8", Key::Num9 => "9",
            Key::F1 => "F1", Key::F2 => "F2", Key::F3 => "F3", Key::F4 => "F4",
            Key::F5 => "F5", Key::F6 => "F6", Key::F7 => "F7", Key::F8 => "F8",
            Key::F9 => "F9", Key::F10 => "F10", Key::F11 => "F11", Key::F12 => "F12",
            Key::Escape => "Escape", Key::Tab => "Tab", Key::Enter => "Enter",
            Key::Space => "Space", Key::Backspace => "Backspace",
            Key::Delete => "Delete", Key::Insert => "Insert",
            Key::Up => "Up", Key::Down => "Down", Key::Left => "Left",
            Key::Right => "Right", Key::Home => "Home", Key::End => "End",
            Key::PageUp => "PageUp", Key::PageDown => "PageDown",
            Key::LeftShift => "LeftShift", Key::RightShift => "RightShift",
            Key::LeftCtrl => "LeftCtrl", Key::RightCtrl => "RightCtrl",
            Key::LeftAlt => "LeftAlt", Key::RightAlt => "RightAlt",
            Key::Comma => ",", Key::Period => ".", Key::Slash => "/",
            Key::Semicolon => ";", Key::Colon => ":", Key::Quote => "'",
            Key::DoubleQuote => "\"", Key::LeftBracket => "[", Key::RightBracket => "]",
            Key::Backslash => "\\", Key::Tilde => "~", Key::Minus => "-",
            Key::Plus => "+", Key::Equal => "=",
            Key::Kp0 => "KP0", Key::Kp1 => "KP1", Key::Kp2 => "KP2",
            Key::Kp3 => "KP3", Key::Kp4 => "KP4", Key::Kp5 => "KP5",
            Key::Kp6 => "KP6", Key::Kp7 => "KP7", Key::Kp8 => "KP8", Key::Kp9 => "KP9",
            Key::KpEnter => "KPEnter", Key::KpPlus => "KP+", Key::KpMinus => "KP-",
            Key::KpMultiply => "KP*", Key::KpDivide => "KP/", Key::KpDecimal => "KP.",
            Key::CapsLock => "CapsLock", Key::ScrollLock => "ScrollLock",
            Key::NumLock => "NumLock", Key::PrintScreen => "PrintScreen",
            Key::Pause => "Pause",
            Key::VolumeUp => "VolumeUp", Key::VolumeDown => "VolumeDown",
            Key::Mute => "Mute", Key::Play => "Play", Key::Stop => "Stop",
        }
    }

    /// Verificar si es un modificador
    pub fn is_modifier(&self) -> bool {
        matches!(
            self,
            Key::LeftShift | Key::RightShift
                | Key::LeftCtrl | Key::RightCtrl
                | Key::LeftAlt | Key::RightAlt
        )
    }

    /// Verificar si es una letra
    pub fn is_letter(&self) -> bool {
        matches!(
            self,
            Key::A | Key::B | Key::C | Key::D | Key::E | Key::F | Key::G
                | Key::H | Key::I | Key::J | Key::K | Key::L | Key::M
                | Key::N | Key::O | Key::P | Key::Q | Key::R | Key::S
                | Key::T | Key::U | Key::V | Key::W | Key::X | Key::Y | Key::Z
        )
    }

    /// Verificar si es un número
    pub fn is_number(&self) -> bool {
        matches!(
            self,
            Key::Num0 | Key::Num1 | Key::Num2 | Key::Num3 | Key::Num4
                | Key::Num5 | Key::Num6 | Key::Num7 | Key::Num8 | Key::Num9
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_name() {
        assert_eq!(Key::A.name(), "A");
        assert_eq!(Key::Escape.name(), "Escape");
        assert_eq!(Key::Space.name(), "Space");
        assert_eq!(Key::Enter.name(), "Enter");
    }

    #[test]
    fn test_key_is_modifier() {
        assert!(Key::LeftShift.is_modifier());
        assert!(Key::RightCtrl.is_modifier());
        assert!(!Key::A.is_modifier());
    }

    #[test]
    fn test_key_is_letter() {
        assert!(Key::A.is_letter());
        assert!(Key::Z.is_letter());
        assert!(!Key::Num1.is_letter());
    }

    #[test]
    fn test_key_is_number() {
        assert!(Key::Num0.is_number());
        assert!(Key::Num9.is_number());
        assert!(!Key::A.is_number());
    }
}
