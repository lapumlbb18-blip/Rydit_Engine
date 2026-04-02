// crates/migui/build.rs
// Copia fuente del sistema para embeber

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let font_paths = [
        "/data/data/com.termux/files/usr/share/fonts/TTF/DejaVuSans.ttf",
        "/usr/share/fonts/TTF/DejaVuSans.ttf",
    ];

    for path in &font_paths {
        if Path::new(path).exists() {
            let dest = Path::new(&out_dir).join("font.ttf");
            fs::copy(path, &dest).expect("Failed to copy font");
            println!("cargo:rerun-if-changed={}", path);
            return;
        }
    }

    // Fallback: crear archivo vacío
    let dest = Path::new(&out_dir).join("font.ttf");
    fs::write(&dest, []).ok();
}
