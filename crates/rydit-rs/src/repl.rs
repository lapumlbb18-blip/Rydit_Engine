// crates/rydit-rs/src/repl.rs
// REPL Interactivo para RyDit

use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

use blast_core::Executor;
use rydit_parser::{Parser, Stmt};
use rydit_lexer::Lexer;

/// Iniciar modo REPL interactivo
pub fn repl_mode() {
    println!("=== RYDIT REPL v0.7.0 ===");
    println!("Escribe comandos RyDit y presiona Enter");
    println!("Comandos: 'help', 'mem', 'clear', 'exit'");
    println!();

    let mut executor = Executor::nuevo();
    let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
    let mut loaded_modules = HashSet::new();
    let mut importing_stack = Vec::new();
    let stdin = io::stdin();

    loop {
        print!("rydit> ");
        if let Err(e) = io::stdout().flush() {
            eprintln!("[REPL ERROR] Flush falló: {}", e);
            break;
        }

        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();

        match input {
            "" => continue,
            "exit" | "quit" | "q" => {
                println!("[REPL] Saliendo...");
                break;
            }
            "help" | "h" => {
                println!("Comandos RyDit:");
                println!("  shield.init          - Inicializar sistema");
                println!("  onda.core            - Ejecutar acción");
                println!("  ryprime              - Operación especial");
                println!("  dark.slot x = N      - Crear variable");
                println!("  onif x > 0 ... blelse - Condicional");
                println!();
                println!("Comandos REPL:");
                println!("  help / h    - Esta ayuda");
                println!("  mem / m     - Ver memoria");
                println!("  clear / c   - Limpiar pantalla");
                println!("  exit / q    - Salir");
                continue;
            }
            "mem" | "m" => {
                executor.mostrar_memoria();
                continue;
            }
            "clear" | "c" => {
                print!("\x1B[2J\x1B[1;1H");
                continue;
            }
            _ => {
                let tokens = Lizer::new(input).scan();
                let mut parser = Parser::new(tokens);

                match parser.parse() {
                    Ok(program) => {
                        println!("[RYDIT] {} statements", program.statements.len());
                        // Ejecutar statements
                        for stmt in &program.statements {
                            crate::ejecutar_stmt(
                                stmt,
                                &mut executor,
                                &mut funcs,
                                &mut loaded_modules,
                                &mut importing_stack,
                            );
                        }
                    }
                    Err(e) => {
                        println!("[ERROR] {}", e);
                    }
                }
            }
        }
    }
}
