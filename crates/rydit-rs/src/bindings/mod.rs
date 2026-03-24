// crates/rydit-rs/src/bindings/mod.rs
// Bindings de funciones RyDit → Rust
//
// Módulo central para todos los bindings del lenguaje
//
// FUTURO (v0.7.1+):
// - RyditModule trait para registro dinámico de módulos
// - Macros para registro automático
// - Módulos independientes: gfx, audio, particles, etc.

/// Registrar todos los bindings en el evaluador
/// FUTURO: Esto se moverá a RyditModule trait
#[allow(dead_code)]
pub fn registrar_todos() {
    // Placeholder para futuro sistema de módulos
}

// Futuros módulos:
// - stdlib.rs: math, arrays, strings, io, random, time, json, regex, files
// - gfx.rs: draw::*, input::*, colisiones::*
// - audio.rs: audio::load_sound, audio::play, etc.
// - particles.rs: particles::fire, smoke, explosion, etc.
// - assets.rs: assets::load_texture, assets::draw, etc.
// - migui.rs: migui::button, slider, checkbox, etc.
