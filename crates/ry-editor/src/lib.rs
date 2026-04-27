//! Ry-Editor: Entorno de Desarrollo Consolidado
//! 
//! Consolida:
//! - RyBot (Supervisión)
//! - RyArt (Estilos procedimentales)
//! - Viewport Manager (Lienzos 3D/2D)
//! - Migui (Interfaz)

pub mod editor_state;
pub mod viewport_manager;
pub mod viewport_controller;

pub use editor_state::EditorState;
pub use viewport_controller::ViewportController;
