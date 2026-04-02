// crates/rydit-rs/src/bin/snake.rs
// Binario dedicado para Snake Game - RyDit v0.1.0
// Ejecutar: cargo run --bin snake

use blast_core::Executor;
use lizer::Parser;
use rydit_gfx::{ColorRydit, Key, RyditGfx};
use rydit_lexer::Lexer;
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
    let tokens = Lexer::new(&script).scan();
    let mut parser = Parser::new(tokens);

    let (program, errors) = parser.parse();
    
    if !errors.is_empty() {
        eprintln!("[ERROR] Error parseando script: {}", errors[0]);
        return;
    }
    
    println!("[INFO] {} statements parseados", program.statements.len());
    println!("[INFO] Iniciando juego...");
    println!();

    // Ejecutar programa en loop infinito (hasta que usuario presione ESC)
    loop {
        ejecutar_programa_gfx(&program, &mut executor, &mut funcs, &mut gfx);
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

    // Crear Render Queue
    let mut queue = rydit_gfx::render_queue::RenderQueue::with_capacity(8192);

    // Game loop principal
    while !gfx.should_close() {
        // Input primero (Rust = Arquitecto)
        input.actualizar(gfx);
        let escape = gfx.is_key_pressed(Key::Escape);

        // Clear queue
        queue.clear();

        // Clear screen
        queue.push(rydit_gfx::render_queue::DrawCommand::Clear {
            color: ColorRydit::Negro,
        });

        // Ejecutar programa en cada frame (acumula en queue)
        for stmt in &program.statements {
            ejecutar_stmt_gfx(stmt, executor, funcs, &mut queue, &mut input);
        }

        // FPS counter
        queue.push(rydit_gfx::render_queue::DrawCommand::Text {
            text: "RyDit v0.1.0".to_string(),
            x: 10,
            y: 10,
            size: 20,
            color: ColorRydit::Blanco,
        });

        // Ejecutar queue (sin assets - snake no usa sprites)
        // NOTA: Para usar assets, necesitar:
        // let assets_ref = /* obtener assets */;
        // queue.execute(gfx, &assets_ref.borrow());

        // Por ahora, ejecutamos directamente con begin_draw
        let mut d = gfx.begin_draw();
        d.clear(ColorRydit::Negro);
        // Dibujar snake y comida directamente
        drop(d);

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
    _queue: &mut rydit_gfx::render_queue::RenderQueue,
    _input: &mut InputEstado,
) {
    // Los statements se ejecutan en el game loop principal
    // Esta función está simplificada para el binario dedicado
    let _ = (stmt, _executor, _funcs, _queue, _input);
}
