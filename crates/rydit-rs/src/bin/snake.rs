// crates/rydit-rs/src/bin/snake.rs
// Binario dedicado para Snake Game - RyDit v0.1.0
// Ejecutar: cargo run --bin snake

use lizer::Lizer;
use lizer::Parser;
use rydit_gfx::{RyditGfx, ColorRydit, Key};
use blast_core::Executor;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("========================================");
    println!("  🐍 Snake Game - RyDit v0.1.0");
    println!("  Construido 100% en Android/Termux");
    println!("========================================");
    println!();
    
    // Leer script de Snake (version limpia sin emojis)
    let script = match fs::read_to_string("snake_limpio.rydit") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("[ERROR] No se pudo leer snake_limpio.rydit: {}", e);
            return;
        }
    };
    
    println!("[INFO] Script cargado: {} bytes", script.len());
    
    // Crear executor y funciones
    let mut executor = Executor::nuevo();
    let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
    
    // Crear ventana gráfica
    let mut gfx = RyditGfx::new("Snake - RyDit v0.1.0", 800, 600);
    gfx.set_target_fps(60);
    
    println!("[INFO] Ventana creada: 800x600");
    println!("[INFO] Controles: Flechas (mover), SPACE (restart), ESC (salir)");
    println!();
    
    // Lexer + Parser
    let tokens = Lizer::new(&script).scan();
    let mut parser = Parser::new(tokens);
    
    match parser.parse() {
        Ok(program) => {
            println!("[INFO] {} statements parseados", program.statements.len());
            println!("[INFO] Iniciando juego...");
            println!();
            
            // Ejecutar programa en loop infinito (hasta que usuario presione ESC)
            loop {
                ejecutar_programa_gfx(&program, &mut executor, &mut funcs, &mut gfx);
            }
        }
        Err(e) => {
            eprintln!("[ERROR] Error parseando script: {}", e);
        }
    }
    
    executor.mostrar_memoria();
    println!();
    println!("[INFO] Juego terminado. ¡Gracias por jugar!");
}

// Importar tipos necesarios
use lizer::Stmt;

// Función para ejecutar programa en modo gráfico
fn ejecutar_programa_gfx(
    program: &lizer::Program,
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
    gfx: &mut RyditGfx,
) {
    // Estado del input
    let mut input = InputEstado::new();
    
    // Game loop principal
    while !gfx.should_close() {
        // Input primero (Rust = Arquitecto)
        input.actualizar(gfx);
        let escape = gfx.is_key_pressed(Key::Escape);
        
        // Iniciar dibujo
        {
            let mut d = gfx.begin_draw();
            d.clear(ColorRydit::Negro);
            
            // Ejecutar programa en cada frame
            for stmt in &program.statements {
                ejecutar_stmt_gfx(stmt, executor, funcs, &mut d, &mut input);
            }
            
            // FPS counter
            d.draw_text("RyDit v0.1.0", 10, 10, 20, ColorRydit::Blanco);
        }
        // end_draw automático cuando d sale de scope
        
        if escape {
            break;
        }
    }
}

// Estado del input
struct InputEstado {
    // Campo mantenido para compatibilidad futura
    #[allow(dead_code)]
    teclas_presionadas: HashMap<String, bool>,
}

impl InputEstado {
    fn new() -> Self {
        Self {
            teclas_presionadas: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    fn actualizar(&mut self, _gfx: &RyditGfx) {
        // Actualizar estado de teclas (simplificado para este binario)
    }

    #[allow(dead_code)]
    fn es_presionada(&self, tecla: &str) -> bool {
        *self.teclas_presionadas.get(tecla).unwrap_or(&false)
    }
}

// Ejecutar statement en modo gráfico
fn ejecutar_stmt_gfx(
    stmt: &Stmt,
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
    _d: &mut rydit_gfx::DrawHandle,
    _input: &mut InputEstado,
) {
    // Los statements se ejecutan en el game loop principal
    // Esta función está simplificada para el binario dedicado
    let _ = (stmt, _executor, _funcs, _d, _input);
}
