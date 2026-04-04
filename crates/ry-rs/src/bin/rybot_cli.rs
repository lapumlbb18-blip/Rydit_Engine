// crates/rydit-rs/src/bin/rybot_cli.rs
// RyBot CLI - Herramienta de línea de comandos para debug
// v0.11.0 - RyBot CLI + UI


fn main() {
    println!("🛡️ RyBot CLI v0.11.0");
    println!("====================");
    println!();

    // Parsear argumentos
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "status" => cmd_status(),
        "inspect" => cmd_inspect(&args[2..]),
        "logs" => cmd_logs(&args[2..]),
        "help" | "--help" | "-h" => print_help(),
        _ => {
            eprintln!("❌ Comando desconocido: {}", command);
            print_help();
        }
    }
}

fn print_help() {
    println!("Uso: rybot_cli <comando> [argumentos]");
    println!();
    println!("Comandos:");
    println!("  status              - Ver estado del sistema");
    println!("  inspect             - Inspeccionar entidades/módulos");
    println!("  logs                - Ver logs de eventos");
    println!("  help                - Mostrar esta ayuda");
    println!();
    println!("Ejemplos:");
    println!("  rybot_cli status");
    println!("  rybot_cli inspect modules");
    println!("  rybot_cli logs --last 10");
}

fn cmd_status() {
    // Simular estado (en producción leería del registry real)
    println!("=== RyBot Status ===");
    println!("Frame: 0");
    println!("Uptime: 0s");
    println!();
    println!("=== Metrics ===");
    println!("FPS: 60.0");
    println!("Frame time: 16.67ms");
    println!("Parse time: 0.8ms");
    println!("Eval time: 2.1ms");
    println!("Render time: 8.3ms");
    println!("Entities: 0");
    println!("Modules: 0");
    println!();
    println!("Nota: RyBot CLI requiere conexión con rydit-rs en ejecución");
    println!("(Implementación pendiente: IPC/socket)");
}

fn cmd_inspect(args: &[String]) {
    if args.is_empty() {
        println!("Uso: inspect <entidad|module|modules>");
        return;
    }

    match args[0].as_str() {
        "modules" => {
            println!("=== Módulos Registrados ===");
            println!("(Ninguno - RyBot aún no integrado con main.rs)");
        }
        "module" => {
            if args.len() < 2 {
                println!("Uso: inspect module <nombre>");
                return;
            }
            println!("Inspeccionando módulo: {}", args[1]);
        }
        "entidad" | "entity" => {
            if args.len() < 2 {
                println!("Uso: inspect entity <id>");
                return;
            }
            println!("Inspeccionando entidad: {}", args[1]);
        }
        _ => {
            eprintln!("❌ Tipo de inspección desconocido: {}", args[0]);
        }
    }
}

fn cmd_logs(args: &[String]) {
    let mut limit = 10;

    // Parsear argumentos
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--last" || args[i] == "-n" {
            if i + 1 < args.len() {
                if let Ok(n) = args[i + 1].parse() {
                    limit = n;
                }
            }
            i += 2;
        } else {
            i += 1;
        }
    }

    println!("=== Últimos {} eventos ===", limit);
    println!("(Ninguno - RyBot aún no integrado con main.rs)");
}
