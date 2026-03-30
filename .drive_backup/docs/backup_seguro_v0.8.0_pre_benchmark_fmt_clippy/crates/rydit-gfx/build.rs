// build.rs para rydit-gfx
// Configura el linking con raylib nativo usando pkg-config

fn main() {
    // Usar pkg-config para encontrar y linkear raylib
    pkg_config::Config::new()
        .atleast_version("5.0")
        .probe("raylib")
        .expect("raylib no encontrado. Instala raylib con: pkg install raylib");

    println!("cargo:rerun-if-changed=build.rs");
}
