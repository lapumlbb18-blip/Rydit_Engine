// crates/rydit-rs/src/modules/mod.rs
// Módulos extensibles para RyDit

pub mod assets;
pub mod audio;
pub mod csv;
pub mod input_map;
pub mod input_ime;  // IME v0.9.2
pub mod physics;  // Físicas v0.9.3
pub mod camera;  // Cámara 2D v0.9.0
pub mod entity;  // Entity System v0.9.0
pub mod particles;  // ✅ v0.9.2 - Sistema de partículas para .rydit
pub mod level;  // ✅ v0.9.4 - Level Manager
pub mod tilemap;  // ✅ v0.9.4 - Tilemap System
pub mod collision;  // ✅ v0.9.4 - Collision System
pub mod window;  // ✅ v0.9.4 - Window Manager

// Próximos módulos:
// pub mod http;
