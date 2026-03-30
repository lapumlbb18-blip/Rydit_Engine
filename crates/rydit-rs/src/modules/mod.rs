// crates/rydit-rs/src/modules/mod.rs
// Módulos extensibles para RyDit
// ✅ v0.10.2: Solo módulos que no dependen de eval::

pub mod assets;
pub mod audio;
pub mod csv;
pub mod input_map;
pub mod input_ime;
pub mod physics;
pub mod camera;
pub mod entity;
// pub mod particles;  // ✅ v0.10.2: Movido a bin/ para compilación scene_runner
// pub mod level;      // ⚠️ Temporalmente comentado (depende de eval::)
// pub mod tilemap;    // ⚠️ Temporalmente comentado (depende de eval::)
// pub mod collision;  // ⚠️ Temporalmente comentado (depende de eval::)
// pub mod window;     // ⚠️ Temporalmente comentado (depende de eval::)
