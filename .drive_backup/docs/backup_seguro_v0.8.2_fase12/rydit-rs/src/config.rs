// crates/rydit-rs/src/config.rs
// Configuración de entorno y carga de módulos

use std::{env, fs, path::Path};

// =============================================================================
// CONFIGURACIÓN DE ENTORNO (v0.6.0)
// =============================================================================

/// Configurar variables de entorno para Termux-X11 automáticamente
pub fn configurar_entorno_termux() {
    // Detectar si estamos en Termux
    let es_termux =
        env::var("TERMUX_VERSION").is_ok() || Path::new("/data/data/com.termux").exists();

    if es_termux {
        println!("[RYDIT] Termux detectado - Configurando entorno gráfico...");

        // Configurar DISPLAY si no está establecido
        if env::var("DISPLAY").is_err() {
            env::set_var("DISPLAY", ":0");
            println!("[RYDIT] DISPLAY=:0 configurado automáticamente");
        }

        // Configurar driver zink si no está establecido
        if env::var("MESA_LOADER_DRIVER_OVERRIDE").is_err() {
            env::set_var("MESA_LOADER_DRIVER_OVERRIDE", "zink");
            println!("[RYDIT] zink GPU driver configurado automáticamente");
        }

        // Configurar DRI3 si no está establecido
        if env::var("DRI3").is_err() {
            env::set_var("DRI3", "1");
            println!("[RYDIT] DRI3=1 configurado automáticamente");
        }

        println!("[RYDIT] ✅ Entorno gráfico listo para Termux-X11");
    }
}

// =============================================================================
// MÓDULOS STDLIB EMBEBIDOS (v0.6.0)
// =============================================================================

const MATH_MODULE: &str = include_str!("../../modules/math.rydit");
const ARRAYS_MODULE: &str = include_str!("../../modules/arrays.rydit");
const STRINGS_MODULE: &str = include_str!("../../modules/strings.rydit");
const IO_MODULE: &str = include_str!("../../modules/io.rydit");
const RANDOM_MODULE: &str = include_str!("../../modules/random.rydit");
const TIME_MODULE: &str = include_str!("../../modules/time.rydit");
const JSON_MODULE: &str = include_str!("../../modules/json.rydit");
const COLISIONES_MODULE: &str = include_str!("../../modules/colisiones.rydit");
const REGEX_MODULE: &str = include_str!("../../modules/regex.rydit");
const FILES_MODULE: &str = include_str!("../../modules/files.rydit");

/// Cargar módulo (archivo local o embebido)
pub fn cargar_modulo(nombre: &str) -> Result<String, String> {
    // 1. Intentar archivo local
    let ruta_local = format!("modules/{}.rydit", nombre);
    if Path::new(&ruta_local).exists() {
        fs::read_to_string(&ruta_local).map_err(|e| format!("Error leyendo '{}': {}", nombre, e))
    } else {
        // 2. Fallback embebido
        match nombre {
            "math" => Ok(MATH_MODULE.to_string()),
            "arrays" => Ok(ARRAYS_MODULE.to_string()),
            "strings" => Ok(STRINGS_MODULE.to_string()),
            "io" => Ok(IO_MODULE.to_string()),
            "random" => Ok(RANDOM_MODULE.to_string()),
            "time" => Ok(TIME_MODULE.to_string()),
            "json" => Ok(JSON_MODULE.to_string()),
            "colisiones" => Ok(COLISIONES_MODULE.to_string()),
            "regex" => Ok(REGEX_MODULE.to_string()),
            "files" => Ok(FILES_MODULE.to_string()),
            _ => Err(format!("Módulo '{}' no encontrado", nombre)),
        }
    }
}
