// crates/rydit-rs/src/cli.rs
// Parsing de argumentos de línea de comandos y entrada principal
// v0.10.2: Inversión de Control + AST Caching

use std::{env, fs};

use blast_core::Executor;
use migui::Migui;
use ry_gfx::RyditGfx;
  // 🆕 v0.14.0: ry-config crate
use crate::executor::{
    ejecutar_programa, ejecutar_programa_gfx, ejecutar_programa_migui,
};
use crate::interpreter::init_global_loader;

pub use crate::{
    lazos::lazos_loop, repl::repl_mode,
};

/// Punto de entrada principal después de configurar entorno
pub fn run() {
    // Inicializar loader de módulos dinámicos (v0.8.2)
    init_global_loader();

    let args: Vec<String> = env::args().collect();

    // ✅ v0.10.2: SCENE RUNNER (Inversión de Control)
    if args.len() > 1 && args[1] == "--scene" {
        // Ejecutar scene_runner con el nombre de escena
        let escena = if args.len() > 2 {
            &args[2]
        } else {
            "demos/nivel_config.rydit"
        };

        println!("🛡️ Iniciando Scene Runner v0.10.2");
        println!("   Escena: {}\n", escena);

        // Ejecutar scene_runner directamente
        std::process::Command::new(std::env::current_exe().unwrap())
            .arg("--scene-run")
            .arg(escena)
            .status()
            .expect("Failed to run scene runner");
        return;
    }

    // ✅ v0.10.2: SCENE RUN interno — 🗑️ Eliminado v0.13.1 (usaba ry-ecs eliminado)
    // Usar --gfx o --migui en su lugar
    if args.len() > 1 && args[1] == "--scene-run" {
        eprintln!("⚠️ --scene-run eliminado v0.13.1 (ry-ecs eliminado). Usa --gfx o --migui.");
        return;
    }

    // Verificar si es modo LAZOS (Protocolo LAZOS)
    if args.len() > 1 && (args[1] == "--lazos" || args[1] == "-l") {
        lazos_loop();
        return;
    }

    // Verificar si es modo REPL
    if args.len() > 1 && (args[1] == "--repl" || args[1] == "-r") {
        repl_mode();
        return;
    }

    // Verificar si es modo gráfico (gfx) o modo migui
    let modo_gfx = args.iter().any(|x| x == "--gfx" || x == "-g");
    let modo_migui = args.iter().any(|x| x == "--migui" || x == "-m");

    let default_script = "shield.init";

    // Buscar script: después de --gfx/--migui o al final
    let script_arg = if let Some(flag_pos) = args
        .iter()
        .position(|x| x == "--gfx" || x == "-g" || x == "--migui" || x == "-m")
    {
        // El script está después del flag
        if args.len() > flag_pos + 1 {
            Some(&args[flag_pos + 1])
        } else {
            None
        }
    } else if args.len() > 1 && args[1] != "--repl" && args[1] != "-r" {
        // El script es el segundo argumento
        Some(&args[1])
    } else {
        None
    };

    // Determinar si es archivo o script directo
    let script: String = if let Some(arg) = script_arg {
        if arg.ends_with(".rydit") {
            match fs::read_to_string(arg) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("[ERROR] No se pudo leer el archivo '{}': {}", arg, e);
                    return;
                }
            }
        } else {
            arg.clone()
        }
    } else {
        default_script.to_string()
    };

    if modo_gfx {
        run_gfx_mode(&script);
    } else if modo_migui {
        run_migui_mode(&script);
    } else {
        run_comandante_mode(&script);
    }
}

/// Ejecutar en modo gráfico (gfx)
fn run_gfx_mode(script: &str) {
    println!("--- SHIELD SYSTEM: MODO GRÁFICO ---");
    println!(
        "[RYDIT-GFX] Parseando: {}",
        script.lines().next().unwrap_or("(script vacío)")
    );

    let mut executor = Executor::nuevo();
    let mut funcs: std::collections::HashMap<String, (Vec<String>, Vec<lizer::Stmt>)> =
        std::collections::HashMap::new();
    let mut gfx = RyditGfx::new("Ry-Dit", 800, 600);
    gfx.set_target_fps(60);

    // Lexer + Parser (AST) - ✅ v0.10.2: Usar caching
    let program = match lizer::parse_cached(script) {
        Ok(p) => {
            println!("[RYDIT] {} statements en AST (cached)", p.statements.len());
            p
        }
        Err(e) => {
            println!("[ERROR] {}", e);
            return;
        }
    };

    ejecutar_programa_gfx(&program, &mut executor, &mut funcs, &mut gfx);

    executor.mostrar_memoria();
    println!("--- SISTEMA PROTEGIDO ---");
}

/// Ejecutar en modo migui (GUI)
fn run_migui_mode(script: &str) {
    println!("--- SHIELD SYSTEM: MODO MIGUI ---");
    println!(
        "[MIGUI] Parseando: {}",
        script.lines().next().unwrap_or("(script vacío)")
    );

    let mut executor = Executor::nuevo();
    let mut funcs: std::collections::HashMap<String, (Vec<String>, Vec<lizer::Stmt>)> =
        std::collections::HashMap::new();
    let mut gui = Migui::new();
    let mut gfx = RyditGfx::new("RyDit migui v0.4.1", 800, 600);
    gfx.set_target_fps(60);

    // Lexer + Parser (AST) - ✅ v0.10.2: Usar caching
    let program = match lizer::parse_cached(script) {
        Ok(p) => {
            println!("[MIGUI] {} statements en AST (cached)", p.statements.len());
            p
        }
        Err(e) => {
            println!("[ERROR] {}", e);
            return;
        }
    };

    ejecutar_programa_migui(&program, &mut executor, &mut funcs, &mut gui, &mut gfx);

    executor.mostrar_memoria();
    println!("--- SISTEMA PROTEGIDO ---");
}

/// Ejecutar en modo comandante (CLI sin gráficos)
fn run_comandante_mode(script: &str) {
    println!("--- SHIELD SYSTEM: MODO COMANDANTE ---");
    println!(
        "[RYDIT] Parseando: {}",
        script.lines().next().unwrap_or("(script vacío)")
    );

    let mut executor = Executor::nuevo();
    let mut funcs: std::collections::HashMap<String, (Vec<String>, Vec<lizer::Stmt>)> =
        std::collections::HashMap::new();

    // Lexer + Parser (AST) - ✅ v0.10.2: Usar caching
    let program = match lizer::parse_cached(script) {
        Ok(p) => {
            println!("[RYDIT] {} statements en AST (cached)", p.statements.len());
            p
        }
        Err(e) => {
            println!("[ERROR] {}", e);
            return;
        }
    };

    ejecutar_programa(&program, &mut executor, &mut funcs);

    executor.mostrar_memoria();
    println!("--- SISTEMA PROTEGIDO ---");
}
