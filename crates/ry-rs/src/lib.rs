// crates/ry-rs/src/lib.rs
//! # 🛡️ Ry-Dit — Biblioteca Principal Unificada
//!
//! Este crate es la "Puerta Principal" al ecosistema Ry-Dit.
//! Centraliza los re-exports de todos los subsistemas para facilitar el desarrollo.

// Re-exports de Core y Config
pub use ry_core::{RyditModule, ModuleRegistry, ModuleError, ModuleResult};
pub use ry_config::{ConfigParser, EntityConfig, NivelConfig, ValorConfig};
pub use ry_loader::AssetServer;

// Re-exports de Motor y Orquestación
pub use rybot::{RybotEngine, EngineState, EngineStats, SceneTree, SceneNode, NodeType, RybotGui};

// Re-exports de Gráficos y Arte
pub use ry_gfx::{RyditGfx, ColorRydit, Key};
pub use ry3d_gfx::{Camera3D, Mesh3D};
pub use ry_art::{ArtCanvas, PhysicsBrush, Brush, ArtGenerator};
pub use ry_anim::AnimModule;

// Re-exports de Input
pub use events_ry::{InputManager, InputEvent, MouseButton, Key as RyKey};

// Re-exports de Ciencia y Física
pub use ry_science::ScienceModule;
pub use ry_physics::PhysicsModule;

// Módulos del Core (Intérprete y Scripting)
pub mod bindings;
pub mod cli;
pub mod config;
pub mod eval;
pub mod executor;
pub mod json_helpers;
pub mod lazos;
pub mod repl;
pub mod modules;
pub mod rybot_stub;
pub mod interpreter;

// El Prelude maestro para inicializar el motor en una sola línea
pub mod prelude {
    pub use crate::RybotEngine;
    pub use crate::InputManager;
    pub use crate::RyditGfx;
    pub use crate::ColorRydit;
    pub use crate::PhysicsBrush;
    pub use crate::AssetServer;
    pub use crate::RyKey as Key;
}
