//! # events-ry
//!
//! Sistema de input unificado + Text Input + Shell para ry-dit.
//!
//! ## Arquitectura de 3 Capas
//!
//! ```text
//! CAPA 1: InputEvent (raw)        → Teclas, mouse, touch, gamepad
//! CAPA 2: TextInput (composición)  → Strings completos (IME, Unicode)
//! CAPA 3: Shell (ejecución)        → Comandos, REPL, carga assets
//! ```
//!
//! ## Filosofía
//!
//! - **Backend agnóstico**: SDL2, raylib, o híbrido
//! - **Mobile-first**: Funciona en Termux-X11 (Android)
//! - **Editor-ready**: Shell integrado para debug y carga de assets
//!
//! ## Ejemplo
//!
//! ```rust
//! use events_ry::{InputManager, InputEvent, Key};
//!
//! // Crear manager con backend mock (sin dependencias)
//! let mut manager = InputManager::new();
//!
//! // Alimentar eventos manualmente (backend real los genera)
//! manager.inject_event(InputEvent::KeyPressed { key: Key::A });
//! manager.inject_event(InputEvent::CharTyped { ch: 'h' });
//!
//! // Procesar
//! let events = manager.poll_events();
//! assert_eq!(events.len(), 2);
//! ```

#![allow(missing_docs)]

// ============================================================================
// CAPA 1: InputEvent (raw)
// ============================================================================
mod input_event;
mod key_code;
mod backend;

// ============================================================================
// BACKEND SDL2 (feature-gated)
// ============================================================================
#[cfg(feature = "sdl2-backend")]
pub mod sdl2_backend;
#[cfg(feature = "sdl2-backend")]
pub use sdl2_backend::Sdl2InputBackend;

// ============================================================================
// CAPA 2: TextInput (composición)
// ============================================================================
mod text_input;

// ============================================================================
// CAPA 3: Shell (ejecución)
// ============================================================================
mod shell;

// ============================================================================
// Manager unificado
// ============================================================================
mod manager;

// ============================================================================
// Export público
// ============================================================================
pub use input_event::InputEvent;
pub use key_code::Key;
pub use backend::InputBackend;
pub use text_input::TextInput;
pub use shell::{Shell, ShellCommand, ShellResult, CommandHandler};
pub use manager::InputManager;
