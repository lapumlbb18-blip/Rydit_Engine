//! Sdl2InputBackend - Backend real de SDL2 para InputBackend trait
//!
//! Conecta SDL2 event pump directamente al sistema unificado de eventos.

#[cfg(feature = "sdl2-backend")]
use crate::backend::InputBackend;
#[cfg(feature = "sdl2-backend")]
use crate::input_event::{GamepadAxis, GamepadButton, InputEvent, MouseButton};
#[cfg(feature = "sdl2-backend")]
use crate::key_code::Key;

#[cfg(feature = "sdl2-backend")]
use sdl2::event::Event;
#[cfg(feature = "sdl2-backend")]
use std::collections::HashSet;

/// Backend real de SDL2
///
/// Convierte eventos SDL2 (EventPump) en `InputEvent` unificado.
#[cfg(feature = "sdl2-backend")]
pub struct Sdl2InputBackend {
    event_pump: sdl2::EventPump,
    keys_down: HashSet<Key>,
    mouse_state: HashSet<MouseButton>,
    text_input_enabled: bool,
}

#[cfg(feature = "sdl2-backend")]
impl Sdl2InputBackend {
    /// Crear backend desde un EventPump existente
    pub fn new(event_pump: sdl2::EventPump) -> Self {
        Self {
            event_pump,
            keys_down: HashSet::new(),
            mouse_state: HashSet::new(),
            text_input_enabled: false,
        }
    }

    /// Obtener referencia al event pump (para otros usos)
    pub fn event_pump(&self) -> &sdl2::EventPump {
        &self.event_pump
    }
}

// ============================================================================
// CONVERSIÓN: SDL2 Keycode → events_ry::Key
// ============================================================================

#[cfg(feature = "sdl2-backend")]
fn sdl_keycode_to_ry(keycode: sdl2::keyboard::Keycode) -> Key {
    use sdl2::keyboard::Keycode;
    match keycode {
        Keycode::A => Key::A, Keycode::B => Key::B, Keycode::C => Key::C,
        Keycode::D => Key::D, Keycode::E => Key::E, Keycode::F => Key::F,
        Keycode::G => Key::G, Keycode::H => Key::H, Keycode::I => Key::I,
        Keycode::J => Key::J, Keycode::K => Key::K, Keycode::L => Key::L,
        Keycode::M => Key::M, Keycode::N => Key::N, Keycode::O => Key::O,
        Keycode::P => Key::P, Keycode::Q => Key::Q, Keycode::R => Key::R,
        Keycode::S => Key::S, Keycode::T => Key::T, Keycode::U => Key::U,
        Keycode::V => Key::V, Keycode::W => Key::W, Keycode::X => Key::X,
        Keycode::Y => Key::Y, Keycode::Z => Key::Z,

        Keycode::Num1 => Key::Num1, Keycode::Num2 => Key::Num2,
        Keycode::Num3 => Key::Num3, Keycode::Num4 => Key::Num4,
        Keycode::Num5 => Key::Num5, Keycode::Num6 => Key::Num6,
        Keycode::Num7 => Key::Num7, Keycode::Num8 => Key::Num8,
        Keycode::Num9 => Key::Num9, Keycode::Num0 => Key::Num0,

        Keycode::F1 => Key::F1, Keycode::F2 => Key::F2, Keycode::F3 => Key::F3,
        Keycode::F4 => Key::F4, Keycode::F5 => Key::F5, Keycode::F6 => Key::F6,
        Keycode::F7 => Key::F7, Keycode::F8 => Key::F8, Keycode::F9 => Key::F9,
        Keycode::F10 => Key::F10, Keycode::F11 => Key::F11, Keycode::F12 => Key::F12,

        Keycode::Escape => Key::Escape,
        Keycode::Tab => Key::Tab,
        Keycode::Return => Key::Enter,
        Keycode::Return2 => Key::Enter,
        Keycode::Space => Key::Space,
        Keycode::Backspace => Key::Backspace,
        Keycode::Delete => Key::Delete,
        Keycode::Insert => Key::Insert,

        Keycode::Up => Key::Up, Keycode::Down => Key::Down,
        Keycode::Left => Key::Left, Keycode::Right => Key::Right,
        Keycode::Home => Key::Home, Keycode::End => Key::End,
        Keycode::PageUp => Key::PageUp, Keycode::PageDown => Key::PageDown,

        Keycode::LShift => Key::LeftShift, Keycode::RShift => Key::RightShift,
        Keycode::LCtrl => Key::LeftCtrl, Keycode::RCtrl => Key::RightCtrl,
        Keycode::LAlt => Key::LeftAlt, Keycode::RAlt => Key::RightAlt,

        Keycode::Comma => Key::Comma, Keycode::Period => Key::Period,
        Keycode::Slash => Key::Slash, Keycode::Semicolon => Key::Semicolon,
        Keycode::Colon => Key::Colon, Keycode::Quote => Key::Quote,
        Keycode::LeftBracket => Key::LeftBracket, Keycode::RightBracket => Key::RightBracket,
        Keycode::Backslash => Key::Backslash, Keycode::Backquote => Key::Tilde,
        Keycode::Minus => Key::Minus, Keycode::Equals => Key::Equal,

        Keycode::KP_0 => Key::Kp0, Keycode::KP_1 => Key::Kp1,
        Keycode::KP_2 => Key::Kp2, Keycode::KP_3 => Key::Kp3,
        Keycode::KP_4 => Key::Kp4, Keycode::KP_5 => Key::Kp5,
        Keycode::KP_6 => Key::Kp6, Keycode::KP_7 => Key::Kp7,
        Keycode::KP_8 => Key::Kp8, Keycode::KP_9 => Key::Kp9,
        Keycode::KP_ENTER => Key::KpEnter,
        Keycode::KP_PLUS => Key::KpPlus, Keycode::KP_MINUS => Key::KpMinus,
        Keycode::KP_MULTIPLY => Key::KpMultiply, Keycode::KP_DIVIDE => Key::KpDivide,
        Keycode::KP_PERIOD => Key::KpDecimal,

        Keycode::CapsLock => Key::CapsLock,
        Keycode::ScrollLock => Key::ScrollLock,
        Keycode::NumLockClear => Key::NumLock,
        Keycode::PrintScreen => Key::PrintScreen,
        Keycode::Pause => Key::Pause,

        _ => Key::Escape, // Fallback seguro
    }
}

// ============================================================================
// CONVERSIÓN: SDL2 Mouse Button → events_ry::MouseButton
// ============================================================================

#[cfg(feature = "sdl2-backend")]
fn sdl_mouse_button_to_ry(button: sdl2::mouse::MouseButton) -> MouseButton {
    match button {
        sdl2::mouse::MouseButton::Left => MouseButton::Left,
        sdl2::mouse::MouseButton::Right => MouseButton::Right,
        sdl2::mouse::MouseButton::Middle => MouseButton::Middle,
        sdl2::mouse::MouseButton::X1 | sdl2::mouse::MouseButton::X2 => {
            MouseButton::Extra(if button == sdl2::mouse::MouseButton::X1 { 1 } else { 2 })
        }
        _ => MouseButton::Left,
    }
}

// ============================================================================
// CONVERSIÓN: SDL2 Gamepad → events_ry
// ============================================================================

#[cfg(feature = "sdl2-backend")]
fn sdl_gamepad_button_to_ry(button: sdl2::controller::Button) -> GamepadButton {
    use sdl2::controller::Button;
    match button {
        Button::A => GamepadButton::FaceDown,
        Button::B => GamepadButton::FaceRight,
        Button::X => GamepadButton::FaceLeft,
        Button::Y => GamepadButton::FaceUp,
        Button::LeftShoulder => GamepadButton::LeftShoulder,
        Button::RightShoulder => GamepadButton::RightShoulder,
        Button::LeftStick => GamepadButton::LeftStick,
        Button::RightStick => GamepadButton::RightStick,
        Button::Back => GamepadButton::Back,
        Button::Start => GamepadButton::Start,
        Button::DPadUp => GamepadButton::DPadUp,
        Button::DPadDown => GamepadButton::DPadDown,
        Button::DPadLeft => GamepadButton::DPadLeft,
        Button::DPadRight => GamepadButton::DPadRight,
        _ => GamepadButton::FaceDown,
    }
}

#[cfg(feature = "sdl2-backend")]
fn sdl_gamepad_axis_to_ry(axis: sdl2::controller::Axis) -> GamepadAxis {
    use sdl2::controller::Axis;
    match axis {
        Axis::LeftX => GamepadAxis::LeftX,
        Axis::LeftY => GamepadAxis::LeftY,
        Axis::RightX => GamepadAxis::RightX,
        Axis::RightY => GamepadAxis::RightY,
        Axis::TriggerLeft => GamepadAxis::LeftTrigger,
        Axis::TriggerRight => GamepadAxis::RightTrigger,
        _ => GamepadAxis::LeftX,
    }
}

// ============================================================================
// IMPLEMENTACIÓN DEL TRAIT InputBackend
// ============================================================================

#[cfg(feature = "sdl2-backend")]
impl InputBackend for Sdl2InputBackend {
    fn init(&mut self) -> Result<(), String> {
        Ok(()) // Ya está inicializado
    }

    fn poll_events(&mut self) -> Vec<InputEvent> {
        let mut events = Vec::new();

        for sdl_event in self.event_pump.poll_iter() {
            match sdl_event {
                // --- Teclado ---
                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => {
                    let key = sdl_keycode_to_ry(keycode);
                    self.keys_down.insert(key);
                    events.push(InputEvent::KeyPressed { key });
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    let key = sdl_keycode_to_ry(keycode);
                    self.keys_down.remove(&key);
                    events.push(InputEvent::KeyReleased { key });
                }

                // --- Text Input (SDL_TEXTINPUT / IME) ---
                Event::TextInput { text, .. } => {
                    for ch in text.chars() {
                        events.push(InputEvent::CharTyped { ch });
                    }
                }
                Event::TextEditing { text, .. } => {
                    if !text.is_empty() {
                        events.push(InputEvent::CompositionStart);
                        for ch in text.chars() {
                            events.push(InputEvent::CharTyped { ch });
                        }
                    }
                }

                // --- Mouse ---
                Event::MouseMotion { x, y, .. } => {
                    events.push(InputEvent::MouseMoved { x, y });
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    let btn = sdl_mouse_button_to_ry(mouse_btn);
                    self.mouse_state.insert(btn);
                    events.push(InputEvent::MousePressed { x, y, button: btn });
                }
                Event::MouseButtonUp {
                    x, y, mouse_btn, ..
                } => {
                    let btn = sdl_mouse_button_to_ry(mouse_btn);
                    self.mouse_state.remove(&btn);
                    events.push(InputEvent::MouseReleased { x, y, button: btn });
                }
                Event::MouseWheel {
                    x, y, direction, ..
                } => {
                    let (dx, dy) = match direction {
                        sdl2::mouse::MouseWheelDirection::Flipped => (-x, -y),
                        _ => (x, y),
                    };
                    events.push(InputEvent::MouseWheel {
                        delta_x: dx as f32,
                        delta_y: dy as f32,
                    });
                }

                // --- Touch (Android) ---
                Event::FingerDown {
                    touch_id, x, y, ..
                } => {
                    events.push(InputEvent::TouchDown {
                        id: touch_id as u64,
                        x: (x * 800.0) as i32,
                        y: (y * 600.0) as i32,
                    });
                }
                Event::FingerMotion {
                    touch_id, x, y, ..
                } => {
                    events.push(InputEvent::TouchMotion {
                        id: touch_id as u64,
                        x: (x * 800.0) as i32,
                        y: (y * 600.0) as i32,
                    });
                }
                Event::FingerUp {
                    touch_id, x, y, ..
                } => {
                    events.push(InputEvent::TouchUp {
                        id: touch_id as u64,
                        x: (x * 800.0) as i32,
                        y: (y * 600.0) as i32,
                    });
                }

                // --- Gamepad ---
                Event::ControllerButtonDown { button, .. } => {
                    events.push(InputEvent::GamepadButtonPressed {
                        button: sdl_gamepad_button_to_ry(button),
                        state: true,
                    });
                }
                Event::ControllerButtonUp { button, .. } => {
                    events.push(InputEvent::GamepadButtonPressed {
                        button: sdl_gamepad_button_to_ry(button),
                        state: false,
                    });
                }
                Event::ControllerAxisMotion {
                    axis, value, ..
                } => {
                    events.push(InputEvent::GamepadAxisMoved {
                        axis: sdl_gamepad_axis_to_ry(axis),
                        value: value as f32 / 32767.0,
                    });
                }

                // --- Ventana ---
                Event::Window {
                    win_event: sdl2::event::WindowEvent::SizeChanged(w, h),
                    ..
                } => {
                    events.push(InputEvent::WindowResized {
                        width: w as i32,
                        height: h as i32,
                    });
                }
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Close,
                    ..
                } => {
                    events.push(InputEvent::WindowCloseRequested);
                }

                // --- Ignorados silenciosamente ---
                _ => {}
            }
        }

        events
    }

    fn enable_text_input(&mut self) {
        sdl2::hint::set("SDL_HINT_ANDROID_SEPARATE_MOUSE_AND_TOUCH", "1");
        sdl2::hint::set("SDL_HINT_TOUCH_MOUSE_EVENTS", "1");
        sdl2::hint::set("SDL_HINT_ENABLE_SCREEN_KEYBOARD", "1");
        sdl2::hint::set("SDL_HINT_IME_SHOW_UI", "1");
        self.text_input_enabled = true;
        // SDL2 0.37: text_input es un método global en el crate
        unsafe {
            sdl2::sys::SDL_StartTextInput();
        }
    }

    fn disable_text_input(&mut self) {
        self.text_input_enabled = false;
        unsafe {
            sdl2::sys::SDL_StopTextInput();
        }
    }

    fn is_key_down(&self, key: Key) -> bool {
        self.keys_down.contains(&key)
    }

    fn is_key_just_pressed(&self, _key: Key) -> bool {
        // Se maneja en InputManager comparando frames
        false
    }

    fn mouse_position(&self) -> (i32, i32) {
        let state = sdl2::mouse::MouseState::new(&self.event_pump);
        (state.x(), state.y())
    }

    fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        self.mouse_state.contains(&button)
    }

    fn shutdown(&mut self) {
        self.disable_text_input();
    }
}
