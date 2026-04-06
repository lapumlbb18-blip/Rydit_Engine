// crates/rydit-rs/src/lib.rs
// Ry-Dit - Biblioteca principal
// 🆕 v0.14.0: Re-exporta ry-config en vez de config_parser local

pub use ry_config::{ConfigParser, EntityConfig, NivelConfig, ValorConfig};
// 🗑️ ry_ecs eliminado v0.13.1 — duplicado de modules/entity.rs
pub use ry_gfx;
