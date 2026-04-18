// crates/ry-rs/src/main.rs
//! # 🛡️ Ry-Dit — Punto de Entrada del Binario
//!
//! Este binario es el cargador principal del motor Ry-Dit.
//! Delegua toda la lógica a la biblioteca unificada ry_rs.

fn main() {
    // Inicializar el loader global
    ry_rs::interpreter::init_global_loader();
    
    // Ejecutar el CLI principal
    ry_rs::cli::run();
}
