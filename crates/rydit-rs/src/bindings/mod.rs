// crates/rydit-rs/src/bindings/mod.rs
// Bindings de funciones RyDit → Rust
//
// NOTA: Los bindings están actualmente integrados en eval/mod.rs
// Este archivo es placeholder para futura extracción

/// Registrar todos los bindings en el evaluador
#[allow(dead_code)]
pub fn registrar_todos() {
    // Placeholder - los bindings se registran en eval::evaluar_expr
    // Futura refactorización moverá esto aquí
}

// Futuros módulos:
// - audio.rs: audio::load_sound, audio::play, etc.
// - particles.rs: particles::fire, smoke, explosion, etc.
// - assets.rs: assets::load_texture, assets::draw, etc.
// - regex.rs: regex::match, replace, split, etc.
// - files.rs: files::read, write, append, etc.
// - migui.rs: migui::button, slider, checkbox, etc.
