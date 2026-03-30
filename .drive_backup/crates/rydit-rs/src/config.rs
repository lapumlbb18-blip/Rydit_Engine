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
        configurar_display();
        println!("[RYDIT] ✅ Entorno gráfico listo para Termux-X11");
    }
}

/// Configurar DISPLAY, zink y DRI3 explícitamente
pub fn configurar_display() {
    // Configurar DISPLAY si no está establecido
    if env::var("DISPLAY").is_err() {
        env::set_var("DISPLAY", ":0");
        println!("[CONFIG] DISPLAY=:0 configurado");
    } else {
        let display = env::var("DISPLAY").unwrap_or_default();
        println!("[CONFIG] DISPLAY={} (existente)", display);
    }

    // Configurar driver zink si no está establecido
    if env::var("MESA_LOADER_DRIVER_OVERRIDE").is_err() {
        env::set_var("MESA_LOADER_DRIVER_OVERRIDE", "zink");
        println!("[CONFIG] MESA_LOADER_DRIVER_OVERRIDE=zink configurado");
    } else {
        let driver = env::var("MESA_LOADER_DRIVER_OVERRIDE").unwrap_or_default();
        println!(
            "[CONFIG] MESA_LOADER_DRIVER_OVERRIDE={} (existente)",
            driver
        );
    }

    // Configurar DRI3 si no está establecido
    if env::var("DRI3").is_err() {
        env::set_var("DRI3", "1");
        println!("[CONFIG] DRI3=1 configurado");
    } else {
        let dri3 = env::var("DRI3").unwrap_or_default();
        println!("[CONFIG] DRI3={} (existente)", dri3);
    }
}

/// Mostrar configuración actual de entorno
pub fn mostrar_configuracion() {
    println!("\n=== CONFIGURACIÓN DE ENTORNO ===");
    println!(
        "DISPLAY: {}",
        env::var("DISPLAY").unwrap_or_else(|_| "No configurado".to_string())
    );
    println!(
        "MESA_LOADER_DRIVER_OVERRIDE: {}",
        env::var("MESA_LOADER_DRIVER_OVERRIDE").unwrap_or_else(|_| "No configurado".to_string())
    );
    println!(
        "DRI3: {}",
        env::var("DRI3").unwrap_or_else(|_| "No configurado".to_string())
    );
    println!(
        "TERMUX_VERSION: {}",
        env::var("TERMUX_VERSION").unwrap_or_else(|_| "No detectado".to_string())
    );
    println!("===============================\n");
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
