// crates/rydit-gfx/build.rs
// Build script para linking de SDL2

fn main() {
    // Linking explícito de SDL2 y extensiones
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_image");
    println!("cargo:rustc-link-lib=SDL2_ttf");
    println!("cargo:rustc-link-lib=SDL2_mixer");

    // Usar pkg-config para encontrar las bibliotecas
    if let Ok(libs) = pkg_config::Config::new()
        .atleast_version("2.0")
        .probe("sdl2")
    {
        println!(
            "cargo:rustc-link-search=native={}",
            libs.link_paths
                .iter()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>()
                .join(":")
        );
    }

    println!("cargo:rerun-if-changed=build.rs");
}
