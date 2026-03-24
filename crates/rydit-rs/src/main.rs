use lizer::Lizer;
use lizer::Parser;
use lizer::Program;
use lizer::Stmt;
use lizer::Expr;
use blast_core::Executor;
use blast_core::Valor;
use rydit_gfx::{RyditGfx, ColorRydit, Key};
use migui::{Migui, WidgetId, Rect, Color as MiguiColor};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::collections::{HashMap, HashSet};

// =============================================================================
// CONFIGURACIÓN AUTOMÁTICA DE ENTORNO (v0.6.0)
// =============================================================================
/// Configurar variables de entorno para Termux-X11 automáticamente
fn configurar_entorno_termux() {
    // Detectar si estamos en Termux
    let es_termux = std::env::var("TERMUX_VERSION").is_ok() || 
                    std::path::Path::new("/data/data/com.termux").exists();
    
    if es_termux {
        println!("[RYDIT] Termux detectado - Configurando entorno gráfico...");
        
        // Configurar DISPLAY si no está establecido
        if std::env::var("DISPLAY").is_err() {
            std::env::set_var("DISPLAY", ":0");
            println!("[RYDIT] DISPLAY=:0 configurado automáticamente");
        }
        
        // Configurar driver zink si no está establecido
        if std::env::var("MESA_LOADER_DRIVER_OVERRIDE").is_err() {
            std::env::set_var("MESA_LOADER_DRIVER_OVERRIDE", "zink");
            println!("[RYDIT] zink GPU driver configurado automáticamente");
        }
        
        // Configurar DRI3 si no está establecido
        if std::env::var("DRI3").is_err() {
            std::env::set_var("DRI3", "1");
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

/// Cargar módulo (archivo local o embebido)
fn cargar_modulo(nombre: &str) -> Result<String, String> {
    // 1. Intentar archivo local
    let ruta_local = format!("modules/{}.rydit", nombre);
    if std::path::Path::new(&ruta_local).exists() {
        std::fs::read_to_string(&ruta_local)
            .map_err(|e| format!("Error leyendo '{}': {}", nombre, e))
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
            _ => Err(format!("Módulo '{}' no encontrado", nombre)),
        }
    }
}

// =============================================================================
// FUNCIONES AUXILIARES PARA JSON (serde_json)
// =============================================================================

/// Convertir serde_json::Value a Valor (Rydit)
fn valor_serde_a_rydit(val: &serde_json::Value) -> Valor {
    match val {
        serde_json::Value::Null => Valor::Vacio,
        serde_json::Value::Bool(b) => Valor::Bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Valor::Num(i as f64)
            } else if let Some(f) = n.as_f64() {
                Valor::Num(f)
            } else {
                Valor::Num(0.0)
            }
        }
        serde_json::Value::String(s) => Valor::Texto(s.clone()),
        serde_json::Value::Array(arr) => {
            let valores: Vec<Valor> = arr.iter().map(|v| valor_serde_a_rydit(v)).collect();
            Valor::Array(valores)
        }
        serde_json::Value::Object(obj) => {
            // Los objetos JSON los convertimos a array de pares [key, value]
            let pares: Vec<Valor> = obj.iter().map(|(k, v)| {
                Valor::Array(vec![Valor::Texto(k.clone()), valor_serde_a_rydit(v)])
            }).collect();
            Valor::Array(pares)
        }
    }
}

/// Convertir Valor (Rydit) a serde_json::Value
fn valor_rydit_a_serde(val: &Valor) -> Result<serde_json::Value, String> {
    match val {
        Valor::Num(n) => Ok(serde_json::Value::Number(serde_json::Number::from_f64(*n).unwrap_or(serde_json::Number::from(0)))),
        Valor::Texto(s) => Ok(serde_json::Value::String(s.clone())),
        Valor::Bool(b) => Ok(serde_json::Value::Bool(*b)),
        Valor::Array(arr) => {
            let valores: Result<Vec<serde_json::Value>, _> = arr.iter().map(|v| valor_rydit_a_serde(v)).collect();
            Ok(serde_json::Value::Array(valores?))
        }
        Valor::Vacio => Ok(serde_json::Value::Null),
        Valor::Error(msg) => Err(format!("Valor de error: {}", msg)),
    }
}

fn main() {
    // Configurar entorno automáticamente (Termux-X11)
    configurar_entorno_termux();
    
    let args: Vec<String> = env::args().collect();

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
    let script_arg = if let Some(flag_pos) = args.iter().position(|x| x == "--gfx" || x == "-g" || x == "--migui" || x == "-m") {
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
        println!("--- SHIELD SYSTEM: MODO GRÁFICO ---");
        println!("[RYDIT-GFX] Parseando: {}", script.lines().next().unwrap_or("(script vacío)"));
        
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let mut gfx = RyditGfx::new("RyDit Engine", 800, 600);
        gfx.set_target_fps(60);

        // Lexer + Parser (AST)
        let tokens = Lizer::new(&script).scan();
        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(program) => {
                println!("[RYDIT] {} statements en AST", program.statements.len());
                ejecutar_programa_gfx(&program, &mut executor, &mut funcs, &mut gfx);
            }
            Err(e) => {
                println!("[ERROR] {}", e);
            }
        }

        executor.mostrar_memoria();
        println!("--- SISTEMA PROTEGIDO ---");
    } else if modo_migui {
        println!("--- SHIELD SYSTEM: MODO MIGUI ---");
        println!("[MIGUI] Parseando: {}", script.lines().next().unwrap_or("(script vacío)"));

        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let mut gui = Migui::new();
        let mut gfx = RyditGfx::new("RyDit migui v0.4.1", 800, 600);
        gfx.set_target_fps(60);

        let tokens = Lizer::new(&script).scan();
        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(program) => {
                println!("[RYDIT] {} statements en AST", program.statements.len());
                ejecutar_programa_migui(&program, &mut executor, &mut funcs, &mut gui, &mut gfx);
            }
            Err(e) => {
                println!("[ERROR] {}", e);
            }
        }

        executor.mostrar_memoria();
        println!("--- SISTEMA PROTEGIDO ---");
    } else {
        println!("--- SHIELD SYSTEM: MODO COMANDANTE ---");
        println!("[RYDIT] Parseando: {}", script.lines().next().unwrap_or("(script vacío)"));

        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();  // (params, body)

        // Lexer + Parser (AST)
        let tokens = Lizer::new(&script).scan();
        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(program) => {
                println!("[RYDIT] {} statements en AST", program.statements.len());
                ejecutar_programa(&program, &mut executor, &mut funcs);
            }
            Err(e) => {
                println!("[ERROR] {}", e);
            }
        }

        executor.mostrar_memoria();
        println!("--- SISTEMA PROTEGIDO ---");
    }
}

fn ejecutar_programa(program: &Program, executor: &mut Executor, funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>) {
    // Contexto de imports: módulos cargados y stack de imports en progreso
    let mut loaded_modules: HashSet<String> = HashSet::new();
    let mut importing_stack: Vec<String> = Vec::new();
    
    for stmt in &program.statements {
        let (break_flag, return_val) = ejecutar_stmt(stmt, executor, funcs, &mut loaded_modules, &mut importing_stack);

        // Si hay valor de retorno en el nivel global, imprimirlo
        if let Some(val) = return_val {
            executor.voz(&val);
        }

        // Si hay break en nivel global, es error
        if break_flag == Some(true) {
            println!("[ERROR] 'break' fuera de un loop");
        }
    }
}

fn ejecutar_programa_gfx(program: &Program, executor: &mut Executor, funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>, gfx: &mut RyditGfx) {
    // Estado del input
    let mut input = InputEstado::new();
    
    // Contexto de imports: módulos cargados y stack de imports en progreso
    let mut loaded_modules: HashSet<String> = HashSet::new();
    let mut importing_stack: Vec<String> = Vec::new();

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
                ejecutar_stmt_gfx(stmt, executor, funcs, &mut d, &mut input, &mut loaded_modules, &mut importing_stack);
            }

            // FPS counter
            d.draw_text("RyDit v0.0.9", 10, 10, 20, ColorRydit::Blanco);
        }
        // end_draw automático cuando d sale de scope

        if escape {
            break;
        }
    }
}

fn ejecutar_programa_migui(program: &Program, executor: &mut Executor, funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>, gui: &mut Migui, gfx: &mut RyditGfx) {
    use migui::{Event, MouseButton};
    use rydit_gfx::Key as GfxKey;

    let mut loaded_modules: HashSet<String> = HashSet::new();
    let mut importing_stack: Vec<String> = Vec::new();
    let mut checkbox_states: HashMap<String, bool> = HashMap::new();
    let mut slider_states: HashMap<String, f32> = HashMap::new();
    let mut textbox_states: HashMap<String, String> = HashMap::new();
    let mut window_states: HashMap<String, bool> = HashMap::new();

    // Primero, ejecutar statements iniciales (definiciones de funciones, variables)
    for stmt in &program.statements {
        match stmt {
            Stmt::Function { name, params, body } => {
                funcs.insert(name.clone(), (params.clone(), body.clone()));
            }
            Stmt::Assign { name, value } => {
                let valor = evaluar_expr_migui(value, executor, gui, &mut checkbox_states, &mut slider_states, &mut textbox_states, &mut window_states, funcs);
                executor.guardar(name, valor);
            }
            _ => {}
        }
    }

    // Guardar el bloque de código para ejecutar en cada frame
    let frame_stmts: Vec<&Stmt> = program.statements.iter()
        .filter(|s| matches!(s, Stmt::Block(_)))
        .flat_map(|s| if let Stmt::Block(stmts) = s { stmts.iter().collect() } else { vec![] })
        .collect();

    // Game loop principal con migui + backend
    while !gfx.should_close() {
        // Input de teclado para salir
        if gfx.is_key_pressed(GfxKey::Escape) {
            break;
        }

        // Input de mouse para migui
        let (mx, my) = gfx.get_mouse_position();
        gui.handle_event(Event::MouseMove { x: mx as f32, y: my as f32 });

        if gfx.is_mouse_button_pressed(0) {
            gui.handle_event(Event::MouseDown { button: MouseButton::Left, x: mx as f32, y: my as f32 });
        }
        if gfx.is_mouse_button_pressed(0) == false && gui.is_mouse_down() {
            gui.handle_event(Event::MouseUp { button: MouseButton::Left, x: mx as f32, y: my as f32 });
        }

        // Iniciar frame de migui
        gui.begin_frame();

        // Ejecutar statements del bloque en cada frame
        for stmt in &frame_stmts {
            ejecutar_stmt_migui(stmt, executor, funcs, gui, &mut loaded_modules, &mut importing_stack,
                               &mut checkbox_states, &mut slider_states, &mut textbox_states, &mut window_states);
        }

        gui.end_frame();

        // Debug: mostrar comandos generados
        if gui.draw_commands().len() > 0 {
            println!("[MIGUI] {} comandos generados", gui.draw_commands().len());
        }

        // Renderizar con el backend optimizado
        gfx.render_migui_frame(gui.draw_commands());
    }
}

fn ejecutar_stmt(stmt: &Stmt, executor: &mut Executor, funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>, 
                 loaded_modules: &mut HashSet<String>, importing_stack: &mut Vec<String>) -> (Option<bool>, Option<Valor>) {
    match stmt {
        Stmt::Init => {
            println!("[SHIELD] Inicializando sistema...");
        }
        Stmt::Command(cmd) => {
            executor.ejecutar(cmd);
        }
        Stmt::Assign { name, value } => {
            let valor = evaluar_expr(value, executor, funcs);
            executor.guardar(name, valor);
        }
        Stmt::IndexAssign { array, index, value } => {
            // Asignación a índice de array: arr[index] = value
            let index_val = evaluar_expr(index, executor, funcs);
            let valor = evaluar_expr(value, executor, funcs);
            
            // Obtener el array actual
            if let Some(Valor::Array(arr)) = executor.leer(array) {
                // Calcular índice
                let idx = match index_val {
                    Valor::Num(n) => n as usize,
                    _ => {
                        println!("[ERROR] Índice debe ser número");
                        return (None, None);
                    }
                };
                
                // Verificar límites
                if idx >= arr.len() {
                    println!("[ERROR] Índice {} fuera de límites (array de {} elementos)", idx, arr.len());
                    return (None, None);
                }
                
                // Modificar el array
                let mut nuevo_arr = arr.clone();
                nuevo_arr[idx] = valor;
                executor.guardar(array, Valor::Array(nuevo_arr));
            } else {
                println!("[ERROR] '{}' no es un array", array);
                return (None, None);
            }
        }
        Stmt::If { condition, then_body, else_body } => {
            let cond_val = evaluar_expr(condition, executor, funcs);

            let es_verdad = match cond_val {
                Valor::Num(n) => n != 0.0,
                Valor::Bool(b) => b,
                _ => false,
            };

            if es_verdad {
                for s in then_body {
                    match ejecutar_stmt(s, executor, funcs, loaded_modules, importing_stack) {
                        (Some(true), _) => return (Some(true), None),  // Propagar break
                        (_, Some(val)) => return (None, Some(val)),  // Propagar return
                        _ => {}
                    }
                }
            } else if let Some(else_body) = else_body {
                for s in else_body {
                    match ejecutar_stmt(s, executor, funcs, loaded_modules, importing_stack) {
                        (Some(true), _) => return (Some(true), None),
                        (_, Some(val)) => return (None, Some(val)),
                        _ => {}
                    }
                }
            }
        }
        Stmt::While { condition, body } => {
            // Ejecutar mientras la condición sea verdadera
            let mut iterations = 0;
            while iterations < 100 {  // Límite de seguridad
                let cond_val = evaluar_expr(condition, executor, funcs);
                let es_verdad = match cond_val {
                    Valor::Num(n) => n != 0.0,
                    Valor::Bool(b) => b,
                    _ => false,
                };

                if !es_verdad {
                    break;
                }

                for s in body {
                    match ejecutar_stmt(s, executor, funcs, loaded_modules, importing_stack) {
                        (Some(true), _) => return (Some(true), None),  // Break detectado
                        (_, Some(val)) => return (None, Some(val)),  // Return detectado
                        _ => {}
                    }
                }
                iterations += 1;
            }
        }
        Stmt::ForEach { var, iterable, body } => {
            // cada x en lista { ... }
            let iterable_val = evaluar_expr(iterable, executor, funcs);

            if let Valor::Array(arr) = iterable_val {
                // Iterar sobre cada elemento del array
                for item in arr {
                    // Guardar variable del iterador en memoria
                    executor.guardar(var, item.clone());

                    // Ejecutar cuerpo del loop
                    for s in body {
                        match ejecutar_stmt(s, executor, funcs, loaded_modules, importing_stack) {
                            (Some(true), _) => return (Some(true), None),  // Break detectado
                            (_, Some(val)) => return (None, Some(val)),  // Return detectado
                            _ => {}
                        }
                    }
                }
            } else {
                println!("[ERROR] 'cada' requiere un array, se obtuvo: {:?}", iterable_val);
            }
        }
        Stmt::Block(stmts) => {
            // Ejecutar todos los statements del bloque
            for s in stmts {
                match ejecutar_stmt(s, executor, funcs, loaded_modules, importing_stack) {
                    (Some(true), _) => return (Some(true), None),  // Propagar break
                    (_, Some(val)) => return (None, Some(val)),  // Propagar return
                    _ => {}
                }
            }
        }
        Stmt::Function { name, params, body } => {
            // Guardar función en el registro
            println!("[FUNC] {}({:?}) definida", name, params);
            funcs.insert(name.clone(), (params.clone(), body.clone()));
        }
        Stmt::Call { name, args } => {
            // Llamar función builtin o de usuario
            // Primero verificar funciones builtin
            if name == "sumar" || name == "restar" || name == "multiplicar" || name == "dividir" {
                // Funciones builtin ya manejadas en evaluar_expr
                println!("[WARNING] Función builtin '{}' debe usarse en expresiones", name);
            } else {
                // Función de usuario - clonar datos para evitar borrow checker issues
                let func_data = funcs.get(name).map(|(p, b)| (p.clone(), b.clone()));
                
                if let Some((params, body)) = func_data {
                    // Función de usuario
                    // Evaluar argumentos
                    let mut arg_values = vec![];
                    for arg in args {
                        arg_values.push(evaluar_expr(arg, executor, funcs));
                    }

                    // Crear scope local para la función
                    executor.push_scope();

                    // Mapear parámetros → valores de argumentos en scope local
                    for (i, param) in params.iter().enumerate() {
                        if i < arg_values.len() {
                            executor.guardar_local(param, arg_values[i].clone());
                        }
                    }

                    // Ejecutar body de la función y capturar retorno
                    let mut return_value: Option<Valor> = None;
                    for s in &body {
                        match ejecutar_stmt(s, executor, funcs, loaded_modules, importing_stack) {
                            (Some(true), _) => {
                                executor.pop_scope();  // Limpiar scope antes de salir
                                return (Some(true), None);
                            },
                            (_, Some(val)) => {
                                return_value = Some(val);
                                break;  // Salir del loop, hay retorno
                            },
                            _ => {}
                        }
                    }

                    // Pop scope al finalizar la función
                    executor.pop_scope();

                    // Si hubo retorno, propagarlo
                    if let Some(val) = return_value {
                        return (None, Some(val));
                    }
                } else {
                    println!("[ERROR] Función '{}' no definida", name);
                }
            }
        }
        Stmt::Return(expr) => {
            // Return: retornamos el valor de la expresión
            if let Some(e) = expr {
                let val = evaluar_expr(e, executor, funcs);
                return (None, Some(val));  // Retornar valor al llamador
            } else {
                return (None, Some(Valor::Vacio));  // Return sin valor
            }
        }
        Stmt::Expr(expr) => {
            let val = evaluar_expr(expr, executor, funcs);
            executor.voz(&val);  // Usar voz en vez de println
        }
        Stmt::Break => {
            return (Some(true), None);  // Señal de break
        }
        Stmt::Import { module, alias } => {
            // Importar módulo: import <modulo> [as <alias>]
            // Cargar desde archivo local o embebido

            // DEUDA #2 FIX: Detectar import cíclico
            if importing_stack.contains(&module) {
                println!("[ERROR] Importe cíclico detectado: '{}'", module);
                println!("[ERROR] Stack de imports: {} -> {}",
                    importing_stack.join(" -> "), module);
                return (None, None);
            }

            // DEUDA #1 FIX: Verificar si ya está cargado (evitar re-ejecución)
            if loaded_modules.contains(module.as_str()) {
                println!("[IMPORT] Módulo '{}' ya cargado (usando cache)", module);
                // Solo renombrar funciones existentes
                let prefix = if let Some(alias_name) = alias {
                    alias_name.clone()
                } else {
                    module.clone()
                };

                // Copiar funciones con nuevo nombre desde el cache
                let mut funcs_to_copy: Vec<(String, String)> = Vec::new();
                for (func_name, _) in funcs.iter() {
                    if func_name.starts_with(&format!("{}::", module)) {
                        let orig_name = func_name.strip_prefix(&format!("{}::", module)).unwrap();
                        let new_name = format!("{}::{}", prefix, orig_name);
                        funcs_to_copy.push((func_name.clone(), new_name));
                    }
                }

                for (old_name, new_name) in funcs_to_copy {
                    if let Some(func_data) = funcs.get(&old_name) {
                        funcs.insert(new_name, func_data.clone());
                    }
                }
                
                if let Some(alias_name) = alias {
                    println!("[IMPORT] Módulo '{}' disponible como '{}'", module, alias_name);
                }
                return (None, None);
            }

            // Cargar módulo (archivo local o embebido)
            let module_content = match cargar_modulo(&module) {
                Ok(content) => {
                    println!("[IMPORT] Módulo '{}' cargado", module);
                    content
                },
                Err(e) => {
                    println!("[ERROR] {}", e);
                    return (None, None);
                }
            };

                // Agregar al stack de imports en progreso
                importing_stack.push(module.clone());

                // Lexer + Parser
                let tokens = Lizer::new(&module_content).scan();
                let mut parser = Parser::new(tokens);

                let program = match parser.parse() {
                    Ok(p) => p,
                    Err(e) => {
                        println!("[ERROR] Error parseando módulo '{}': {}", module, e);
                        importing_stack.pop();
                        return (None, None);
                    }
                };

                // Recolectar nombres de funciones originales antes de ejecutar
                let mut original_funcs: Vec<String> = Vec::new();
                for s in &program.statements {
                    if let Stmt::Function { name, .. } = s {
                        original_funcs.push(name.clone());
                    }
                }

                // Ejecutar módulo en scope global
                for s in &program.statements {
                    match ejecutar_stmt(s, executor, funcs, loaded_modules, importing_stack) {
                        (Some(true), _) => {
                            println!("[ERROR] break no permitido en módulo '{}'", module);
                            break;
                        },
                        (_, Some(_)) => {
                            println!("[ERROR] return no permitido en módulo '{}'", module);
                            break;
                        },
                        _ => {}
                    }
                }

                // Remover del stack de imports en progreso
                importing_stack.pop();

                // Marcar módulo como cargado
                loaded_modules.insert(module.clone());

                // Renombrar funciones con el prefijo del módulo
                let prefix = if let Some(alias_name) = alias {
                    alias_name.clone()
                } else {
                    module.clone()
                };

                // Copiar funciones con nuevo nombre
                for orig_name in &original_funcs {
                    if let Some(func_data) = funcs.get(orig_name) {
                        let new_name = format!("{}::{}", prefix, orig_name);
                        funcs.insert(new_name, func_data.clone());
                    }
                }

                // DEUDA #3 FIX: Eliminar funciones originales SOLO si no hay alias
                // Si el usuario usó "import math", eliminar "sumar" y dejar solo "math::sumar"
                // Si el usuario usó "import math as m", dejar "math::sumar" y "m::sumar"
                if alias.is_none() {
                    // Sin alias: eliminar funciones originales
                    for orig_name in &original_funcs {
                        funcs.remove(orig_name);
                    }
                }
                // Con alias: las funciones originales se mantienen como module::func

                // Registrar alias si existe
                if let Some(alias_name) = alias {
                    println!("[IMPORT] Módulo '{}' disponible como '{}'", module, alias_name);
                } else {
                    println!("[IMPORT] Módulo '{}' disponible", module);
                }
        }
        Stmt::DrawCircle { x, y, radio, color } => {
            let x_val = evaluar_expr(x, executor, funcs);
            let y_val = evaluar_expr(y, executor, funcs);
            let radio_val = evaluar_expr(radio, executor, funcs);
            let color_val = ColorRydit::from_str(color);
            
            if let (Valor::Num(x), Valor::Num(y), Valor::Num(radio)) = (x_val, y_val, radio_val) {
                println!("[DRAW] circle({}, {}, {}, {:?})", x, y, radio, color_val);
            } else {
                println!("[ERROR] draw.circle requiere números");
            }
        }
        Stmt::DrawRect { x, y, ancho, alto, color } => {
            let x_val = evaluar_expr(x, executor, funcs);
            let y_val = evaluar_expr(y, executor, funcs);
            let ancho_val = evaluar_expr(ancho, executor, funcs);
            let alto_val = evaluar_expr(alto, executor, funcs);
            let color_val = ColorRydit::from_str(color);
            
            if let (Valor::Num(x), Valor::Num(y), Valor::Num(ancho), Valor::Num(alto)) = (x_val, y_val, ancho_val, alto_val) {
                println!("[DRAW] rect({}, {}, {}, {}, {:?})", x, y, ancho, alto, color_val);
            } else {
                println!("[ERROR] draw.rect requiere números");
            }
        }
        Stmt::DrawLine { x1, y1, x2, y2, color } => {
            let x1_val = evaluar_expr(x1, executor, funcs);
            let y1_val = evaluar_expr(y1, executor, funcs);
            let x2_val = evaluar_expr(x2, executor, funcs);
            let y2_val = evaluar_expr(y2, executor, funcs);
            let color_val = ColorRydit::from_str(color);
            
            if let (Valor::Num(x1), Valor::Num(y1), Valor::Num(x2), Valor::Num(y2)) = (x1_val, y1_val, x2_val, y2_val) {
                println!("[DRAW] line({}, {}, {}, {}, {:?})", x1, y1, x2, y2, color_val);
            } else {
                println!("[ERROR] draw.line requiere números");
            }
        }
        Stmt::DrawText { texto, x, y, tamano, color } => {
            let x_val = evaluar_expr(x, executor, funcs);
            let y_val = evaluar_expr(y, executor, funcs);
            let tamano_val = evaluar_expr(tamano, executor, funcs);
            let color_val = ColorRydit::from_str(color);

            if let (Valor::Num(x), Valor::Num(y), Valor::Num(tamano)) = (x_val, y_val, tamano_val) {
                println!("[DRAW] text('{}', {}, {}, {}, {:?})", texto, x, y, tamano, color_val);
            } else {
                println!("[ERROR] draw.text requiere números");
            }
        }
        // Statements v0.2.0 - Nuevas formas
        Stmt::DrawTriangle { v1_x, v1_y, v2_x, v2_y, v3_x, v3_y, color } => {
            let v1_x_val = evaluar_expr(v1_x, executor, funcs);
            let v1_y_val = evaluar_expr(v1_y, executor, funcs);
            let v2_x_val = evaluar_expr(v2_x, executor, funcs);
            let v2_y_val = evaluar_expr(v2_y, executor, funcs);
            let v3_x_val = evaluar_expr(v3_x, executor, funcs);
            let v3_y_val = evaluar_expr(v3_y, executor, funcs);
            let color_val = ColorRydit::from_str(color);

            if let (Valor::Num(v1_x), Valor::Num(v1_y), Valor::Num(v2_x), Valor::Num(v2_y), Valor::Num(v3_x), Valor::Num(v3_y)) = 
                (v1_x_val, v1_y_val, v2_x_val, v2_y_val, v3_x_val, v3_y_val) {
                println!("[DRAW] triangle({}, {}, {}, {}, {}, {}, {:?})", v1_x, v1_y, v2_x, v2_y, v3_x, v3_y, color_val);
            } else {
                println!("[ERROR] draw.triangle requiere números");
            }
        }
        Stmt::DrawRing { center_x, center_y, inner_radius, outer_radius, color } => {
            let center_x_val = evaluar_expr(center_x, executor, funcs);
            let center_y_val = evaluar_expr(center_y, executor, funcs);
            let inner_radius_val = evaluar_expr(inner_radius, executor, funcs);
            let outer_radius_val = evaluar_expr(outer_radius, executor, funcs);
            let color_val = ColorRydit::from_str(color);

            if let (Valor::Num(cx), Valor::Num(cy), Valor::Num(ir), Valor::Num(or)) = 
                (center_x_val, center_y_val, inner_radius_val, outer_radius_val) {
                println!("[DRAW] ring({}, {}, {}, {}, {:?})", cx, cy, ir, or, color_val);
            } else {
                println!("[ERROR] draw.ring requiere números");
            }
        }
        Stmt::DrawRectangleLines { x, y, ancho, alto, color } => {
            let x_val = evaluar_expr(x, executor, funcs);
            let y_val = evaluar_expr(y, executor, funcs);
            let ancho_val = evaluar_expr(ancho, executor, funcs);
            let alto_val = evaluar_expr(alto, executor, funcs);
            let color_val = ColorRydit::from_str(color);

            if let (Valor::Num(x), Valor::Num(y), Valor::Num(ancho), Valor::Num(alto)) = (x_val, y_val, ancho_val, alto_val) {
                println!("[DRAW] rectangle_lines({}, {}, {}, {}, {:?})", x, y, ancho, alto, color_val);
            } else {
                println!("[ERROR] draw.rectangle_lines requiere números");
            }
        }
        Stmt::DrawEllipse { center_x, center_y, radius_h, radius_v, color } => {
            let center_x_val = evaluar_expr(center_x, executor, funcs);
            let center_y_val = evaluar_expr(center_y, executor, funcs);
            let radius_h_val = evaluar_expr(radius_h, executor, funcs);
            let radius_v_val = evaluar_expr(radius_v, executor, funcs);
            let color_val = ColorRydit::from_str(color);

            if let (Valor::Num(cx), Valor::Num(cy), Valor::Num(rh), Valor::Num(rv)) = 
                (center_x_val, center_y_val, radius_h_val, radius_v_val) {
                println!("[DRAW] ellipse({}, {}, {}, {}, {:?})", cx, cy, rh, rv, color_val);
            } else {
                println!("[ERROR] draw.ellipse requiere números");
            }
        }
        Stmt::DrawLineThick { x1, y1, x2, y2, thick, color } => {
            let x1_val = evaluar_expr(x1, executor, funcs);
            let y1_val = evaluar_expr(y1, executor, funcs);
            let x2_val = evaluar_expr(x2, executor, funcs);
            let y2_val = evaluar_expr(y2, executor, funcs);
            let thick_val = evaluar_expr(thick, executor, funcs);
            let color_val = ColorRydit::from_str(color);

            if let (Valor::Num(x1), Valor::Num(y1), Valor::Num(x2), Valor::Num(y2), Valor::Num(thick)) = 
                (x1_val, y1_val, x2_val, y2_val, thick_val) {
                println!("[DRAW] line_thick({}, {}, {}, {}, {}, {:?})", x1, y1, x2, y2, thick, color_val);
            } else {
                println!("[ERROR] draw.line_thick requiere números");
            }
        }
    }
    (None, None)  // No break, no return value
}

// ============================================================================
// EJECUTOR GRÁFICO (con DrawHandle)
// ============================================================================

use rydit_gfx::DrawHandle;

// Estado del input para Snake y juegos
struct InputEstado {
    arrow_up: bool,
    arrow_down: bool,
    arrow_left: bool,
    arrow_right: bool,
    space: bool,
    enter: bool,
    // Mouse v0.3.0
    mouse_x: i32,
    mouse_y: i32,
    mouse_left: bool,
    mouse_right: bool,
    mouse_middle: bool,
}

impl InputEstado {
    fn new() -> Self {
        Self {
            arrow_up: false,
            arrow_down: false,
            arrow_left: false,
            arrow_right: false,
            space: false,
            enter: false,
            mouse_x: 0,
            mouse_y: 0,
            mouse_left: false,
            mouse_right: false,
            mouse_middle: false,
        }
    }

    fn actualizar(&mut self, gfx: &RyditGfx) {
        self.arrow_up = gfx.is_key_pressed(Key::ArrowUp);
        self.arrow_down = gfx.is_key_pressed(Key::ArrowDown);
        self.arrow_left = gfx.is_key_pressed(Key::ArrowLeft);
        self.arrow_right = gfx.is_key_pressed(Key::ArrowRight);
        self.space = gfx.is_key_pressed(Key::Space);
        self.enter = gfx.is_key_pressed(Key::Enter);
        
        // Actualizar mouse v0.3.0
        let mouse_pos = gfx.get_mouse_position();
        self.mouse_x = mouse_pos.0;
        self.mouse_y = mouse_pos.1;
        self.mouse_left = gfx.is_mouse_button_pressed(0);
        self.mouse_right = gfx.is_mouse_button_pressed(1);
        self.mouse_middle = gfx.is_mouse_button_pressed(2);
    }

    fn es_presionada(&self, tecla: &str) -> bool {
        match tecla.to_lowercase().as_str() {
            "arrow_up" | "arriba" => self.arrow_up,
            "arrow_down" | "abajo" => self.arrow_down,
            "arrow_left" | "izquierda" => self.arrow_left,
            "arrow_right" | "derecha" => self.arrow_right,
            "space" | "espacio" => self.space,
            "enter" | "entrada" => self.enter,
            _ => false,
        }
    }
}

fn ejecutar_stmt_gfx(
    stmt: &Stmt,
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
    d: &mut DrawHandle,
    input: &mut InputEstado,
    loaded_modules: &mut HashSet<String>,
    importing_stack: &mut Vec<String>,
) -> Option<bool> {
    match stmt {
        Stmt::Init => {
            // shield.init en modo gráfico - no hacer nada especial
        }
        Stmt::Command(cmd) => {
            executor.ejecutar(cmd);
        }
        Stmt::Assign { name, value } => {
            let valor = evaluar_expr_gfx(value, executor, input, funcs);
            executor.guardar(name, valor);
        }
        Stmt::IndexAssign { array, index, value } => {
            // Asignación a índice de array: arr[index] = value
            let index_val = evaluar_expr_gfx(index, executor, input, funcs);
            let valor = evaluar_expr_gfx(value, executor, input, funcs);
            
            // Obtener el array actual
            if let Some(Valor::Array(arr)) = executor.leer(array) {
                // Calcular índice
                let idx = match index_val {
                    Valor::Num(n) => n as usize,
                    _ => {
                        println!("[ERROR] Índice debe ser número");
                        return None;
                    }
                };
                
                // Verificar límites
                if idx >= arr.len() {
                    println!("[ERROR] Índice {} fuera de límites (array de {} elementos)", idx, arr.len());
                    return None;
                }
                
                // Modificar el array
                let mut nuevo_arr = arr.clone();
                nuevo_arr[idx] = valor;
                executor.guardar(array, Valor::Array(nuevo_arr));
            } else {
                println!("[ERROR] '{}' no es un array", array);
                return None;
            }
        }
        Stmt::If { condition, then_body, else_body } => {
            let cond_val = evaluar_expr_gfx(condition, executor, input, funcs);
            let es_verdad = match cond_val {
                Valor::Num(n) => n != 0.0,
                Valor::Bool(b) => b,
                _ => false,
            };

            if es_verdad {
                for s in then_body {
                    ejecutar_stmt_gfx(s, executor, funcs, d, input, loaded_modules, importing_stack);
                }
            } else if let Some(else_stmts) = else_body {
                for s in else_stmts {
                    ejecutar_stmt_gfx(s, executor, funcs, d, input, loaded_modules, importing_stack);
                }
            }
        }
        Stmt::While { condition, body } => {
            // Cuidado: while en modo gráfico puede causar loop infinito
            // Usar solo con condiciones controladas
            let mut iterations = 0;
            while iterations < 10 {  // Límite estricto en modo gráfico
                let cond_val = evaluar_expr_gfx(condition, executor, input, funcs);
                let es_verdad = match cond_val {
                    Valor::Num(n) => n != 0.0,
                    Valor::Bool(b) => b,
                    _ => false,
                };

                if !es_verdad {
                    break;
                }

                for s in body {
                    ejecutar_stmt_gfx(s, executor, funcs, d, input, loaded_modules, importing_stack);
                }
                iterations += 1;
            }
        }
        Stmt::ForEach { var, iterable, body } => {
            let iterable_val = evaluar_expr_gfx(iterable, executor, input, funcs);
            if let Valor::Array(arr) = iterable_val {
                for item in arr {
                    executor.guardar(var, item.clone());
                    for s in body {
                        ejecutar_stmt_gfx(s, executor, funcs, d, input, loaded_modules, importing_stack);
                    }
                }
            }
        }
        Stmt::Block(stmts) => {
            for s in stmts {
                ejecutar_stmt_gfx(s, executor, funcs, d, input, loaded_modules, importing_stack);
            }
        }
        Stmt::Function { name, params, body } => {
            funcs.insert(name.clone(), (params.clone(), body.clone()));
        }
        Stmt::Call { name, args } => {
            // Verificar si es tecla_presionada("tecla")
            if name == "tecla_presionada" && args.len() == 1 {
                if let Expr::Texto(tecla) = &args[0] {
                    let presionada = input.es_presionada(tecla);
                    executor.guardar("__RESULT__", Valor::Num(if presionada { 1.0 } else { 0.0 }));
                }
            } else {
                // Función de usuario - clonar datos para evitar borrow checker issues
                let func_data = funcs.get(name).map(|(p, b)| (p.clone(), b.clone()));
                
                if let Some((_params, body)) = func_data {
                    // Evaluar argumentos
                    let mut arg_values = vec![];
                    for arg in args {
                        arg_values.push(evaluar_expr_gfx(arg, executor, input, funcs));
                    }
                    // Ejecutar body con argumentos (sin scope real por ahora)
                    let _ = arg_values;  // Por ahora no usamos los args
                    for s in &body {
                        ejecutar_stmt_gfx(s, executor, funcs, d, input, loaded_modules, importing_stack);
                    }
                }
            }
        }
        Stmt::Return(expr) => {
            if let Some(e) = expr {
                let val = evaluar_expr(e, executor, funcs);
                executor.voz(&val);
            }
        }
        Stmt::Expr(expr) => {
            let val = evaluar_expr(expr, executor, funcs);
            executor.voz(&val);
        }
        Stmt::Import { module, alias } => {
            // Importar módulo en modo gráfico (mismo comportamiento con fixes)
            let module_path = format!("crates/modules/{}.rydit", module);

            // DEUDA #2 FIX: Detectar import cíclico
            if importing_stack.contains(&module) {
                println!("[ERROR] Importe cíclico detectado: '{}'", module);
                println!("[ERROR] Stack de imports: {} -> {}", 
                    importing_stack.join(" -> "), module);
                return None;
            }

            // DEUDA #1 FIX: Verificar si ya está cargado (evitar re-ejecución)
            if loaded_modules.contains(module.as_str()) {
                println!("[IMPORT] Módulo '{}' ya cargado (usando cache)", module);
                // Solo renombrar funciones existentes
                let prefix = if let Some(alias_name) = alias {
                    alias_name.clone()
                } else {
                    module.clone()
                };
                
                // Copiar funciones con nuevo nombre desde el cache
                let mut funcs_to_copy: Vec<(String, String)> = Vec::new();
                for (func_name, _) in funcs.iter() {
                    if func_name.starts_with(&format!("{}::", module)) {
                        let orig_name = func_name.strip_prefix(&format!("{}::", module)).unwrap();
                        let new_name = format!("{}::{}", prefix, orig_name);
                        funcs_to_copy.push((func_name.clone(), new_name));
                    }
                }
                
                for (old_name, new_name) in funcs_to_copy {
                    if let Some(func_data) = funcs.get(&old_name) {
                        funcs.insert(new_name, func_data.clone());
                    }
                }
                
                if let Some(alias_name) = alias {
                    println!("[IMPORT] Módulo '{}' disponible como '{}'", module, alias_name);
                }
                return None;
            }

            if let Ok(content) = std::fs::read_to_string(&module_path) {
                println!("[IMPORT] Cargando módulo '{}' desde '{}'", module, module_path);

                // Agregar al stack de imports en progreso
                importing_stack.push(module.clone());

                // Lexer + Parser
                let tokens = Lizer::new(&content).scan();
                let mut parser = Parser::new(tokens);

                let program = match parser.parse() {
                    Ok(p) => p,
                    Err(e) => {
                        println!("[ERROR] Error parseando módulo '{}': {}", module, e);
                        importing_stack.pop();
                        return None;
                    }
                };

                // Recolectar nombres de funciones originales
                let mut original_funcs: Vec<String> = Vec::new();
                for s in &program.statements {
                    if let Stmt::Function { name, .. } = s {
                        original_funcs.push(name.clone());
                    }
                }

                for s in &program.statements {
                    ejecutar_stmt_gfx(s, executor, funcs, d, input, loaded_modules, importing_stack);
                }

                // Remover del stack de imports en progreso
                importing_stack.pop();

                // Marcar módulo como cargado
                loaded_modules.insert(module.clone());

                // Renombrar funciones con el prefijo del módulo
                let prefix = if let Some(alias_name) = alias {
                    alias_name.clone()
                } else {
                    module.clone()
                };

                // Copiar funciones con nuevo nombre
                for orig_name in &original_funcs {
                    if let Some(func_data) = funcs.get(orig_name) {
                        let new_name = format!("{}::{}", prefix, orig_name);
                        funcs.insert(new_name, func_data.clone());
                    }
                }

                // DEUDA #3 FIX: Eliminar funciones originales SOLO si no hay alias
                if alias.is_none() {
                    // Sin alias: eliminar funciones originales
                    for orig_name in &original_funcs {
                        funcs.remove(orig_name);
                    }
                }
                // Con alias: las funciones originales se mantienen como module::func

                if let Some(alias_name) = alias {
                    println!("[IMPORT] Módulo '{}' disponible como '{}'", module, alias_name);
                } else {
                    println!("[IMPORT] Módulo '{}' disponible", module);
                }
            } else {
                println!("[ERROR] Módulo '{}' no encontrado en '{}'", module, module_path);
            }
        }
        // Comandos de dibujo - dibujan realmente en la ventana
        Stmt::DrawCircle { x, y, radio, color } => {
            let x_val = evaluar_expr(x, executor, funcs);
            let y_val = evaluar_expr(y, executor, funcs);
            let radio_val = evaluar_expr(radio, executor, funcs);
            let color_val = ColorRydit::from_str(color);
            
            if let (Valor::Num(x), Valor::Num(y), Valor::Num(radio)) = (x_val, y_val, radio_val) {
                d.draw_circle(x as i32, y as i32, radio as i32, color_val);
            }
        }
        Stmt::DrawRect { x, y, ancho, alto, color } => {
            let x_val = evaluar_expr(x, executor, funcs);
            let y_val = evaluar_expr(y, executor, funcs);
            let ancho_val = evaluar_expr(ancho, executor, funcs);
            let alto_val = evaluar_expr(alto, executor, funcs);
            let color_val = ColorRydit::from_str(color);
            
            if let (Valor::Num(x), Valor::Num(y), Valor::Num(ancho), Valor::Num(alto)) = (x_val, y_val, ancho_val, alto_val) {
                d.draw_rectangle(x as i32, y as i32, ancho as i32, alto as i32, color_val);
            }
        }
        Stmt::DrawLine { x1, y1, x2, y2, color } => {
            let x1_val = evaluar_expr(x1, executor, funcs);
            let y1_val = evaluar_expr(y1, executor, funcs);
            let x2_val = evaluar_expr(x2, executor, funcs);
            let y2_val = evaluar_expr(y2, executor, funcs);
            let color_val = ColorRydit::from_str(color);
            
            if let (Valor::Num(x1), Valor::Num(y1), Valor::Num(x2), Valor::Num(y2)) = (x1_val, y1_val, x2_val, y2_val) {
                d.draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, color_val);
            }
        }
        Stmt::DrawText { texto, x, y, tamano, color } => {
            let x_val = evaluar_expr(x, executor, funcs);
            let y_val = evaluar_expr(y, executor, funcs);
            let tamano_val = evaluar_expr(tamano, executor, funcs);
            let color_val = ColorRydit::from_str(color);

            if let (Valor::Num(x), Valor::Num(y), Valor::Num(tamano)) = (x_val, y_val, tamano_val) {
                d.draw_text(texto, x as i32, y as i32, tamano as i32, color_val);
            }
        }
        // Statements v0.2.0 - Nuevas formas (gráficos reales)
        Stmt::DrawTriangle { v1_x, v1_y, v2_x, v2_y, v3_x, v3_y, color } => {
            let v1_x_val = evaluar_expr(v1_x, executor, funcs);
            let v1_y_val = evaluar_expr(v1_y, executor, funcs);
            let v2_x_val = evaluar_expr(v2_x, executor, funcs);
            let v2_y_val = evaluar_expr(v2_y, executor, funcs);
            let v3_x_val = evaluar_expr(v3_x, executor, funcs);
            let v3_y_val = evaluar_expr(v3_y, executor, funcs);
            let color_val = ColorRydit::from_str(color);

            if let (Valor::Num(v1_x), Valor::Num(v1_y), Valor::Num(v2_x), Valor::Num(v2_y), Valor::Num(v3_x), Valor::Num(v3_y)) = 
                (v1_x_val, v1_y_val, v2_x_val, v2_y_val, v3_x_val, v3_y_val) {
                d.draw_triangle((v1_x as i32, v1_y as i32), (v2_x as i32, v2_y as i32), (v3_x as i32, v3_y as i32), color_val);
            }
        }
        Stmt::DrawRing { center_x, center_y, inner_radius, outer_radius, color } => {
            let center_x_val = evaluar_expr(center_x, executor, funcs);
            let center_y_val = evaluar_expr(center_y, executor, funcs);
            let inner_radius_val = evaluar_expr(inner_radius, executor, funcs);
            let outer_radius_val = evaluar_expr(outer_radius, executor, funcs);
            let color_val = ColorRydit::from_str(color);

            if let (Valor::Num(cx), Valor::Num(cy), Valor::Num(ir), Valor::Num(or)) = 
                (center_x_val, center_y_val, inner_radius_val, outer_radius_val) {
                d.draw_ring((cx as i32, cy as i32), ir as i32, or as i32, color_val);
            }
        }
        Stmt::DrawRectangleLines { x, y, ancho, alto, color } => {
            let x_val = evaluar_expr(x, executor, funcs);
            let y_val = evaluar_expr(y, executor, funcs);
            let ancho_val = evaluar_expr(ancho, executor, funcs);
            let alto_val = evaluar_expr(alto, executor, funcs);
            let color_val = ColorRydit::from_str(color);

            if let (Valor::Num(x), Valor::Num(y), Valor::Num(ancho), Valor::Num(alto)) = (x_val, y_val, ancho_val, alto_val) {
                d.draw_rectangle_lines(x as i32, y as i32, ancho as i32, alto as i32, color_val);
            }
        }
        Stmt::DrawEllipse { center_x, center_y, radius_h, radius_v, color } => {
            let center_x_val = evaluar_expr(center_x, executor, funcs);
            let center_y_val = evaluar_expr(center_y, executor, funcs);
            let radius_h_val = evaluar_expr(radius_h, executor, funcs);
            let radius_v_val = evaluar_expr(radius_v, executor, funcs);
            let color_val = ColorRydit::from_str(color);

            if let (Valor::Num(cx), Valor::Num(cy), Valor::Num(rh), Valor::Num(rv)) = 
                (center_x_val, center_y_val, radius_h_val, radius_v_val) {
                d.draw_ellipse((cx as i32, cy as i32), rh as i32, rv as i32, color_val);
            }
        }
        Stmt::DrawLineThick { x1, y1, x2, y2, thick, color } => {
            let x1_val = evaluar_expr(x1, executor, funcs);
            let y1_val = evaluar_expr(y1, executor, funcs);
            let x2_val = evaluar_expr(x2, executor, funcs);
            let y2_val = evaluar_expr(y2, executor, funcs);
            let thick_val = evaluar_expr(thick, executor, funcs);
            let color_val = ColorRydit::from_str(color);

            if let (Valor::Num(x1), Valor::Num(y1), Valor::Num(x2), Valor::Num(y2), Valor::Num(thick)) = 
                (x1_val, y1_val, x2_val, y2_val, thick_val) {
                d.draw_line_thick((x1 as i32, y1 as i32), (x2 as i32, y2 as i32), thick as f32, color_val);
            }
        }
        Stmt::Break => {
            return Some(true);  // Señal de break
        }
    }
    None
}

fn valor_a_bool(val: &Valor) -> bool {
    match val {
        Valor::Bool(b) => *b,
        Valor::Num(n) => *n != 0.0,
        _ => false,
    }
}

fn evaluar_expr(expr: &Expr, executor: &mut Executor, funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>) -> Valor {
    match expr {
        Expr::Num(n) => Valor::Num(*n),
        Expr::Texto(s) => Valor::Texto(s.clone()),
        Expr::Var(name) => {
            // Input especial
            if name == "__INPUT__" {
                return executor.input("> ");
            }
            executor.leer(name).unwrap_or(Valor::Vacio)
        }
        Expr::Bool(b) => Valor::Bool(*b),
        Expr::Array(elements) => {
            // Evaluar cada elemento del array
            let valores: Vec<Valor> = elements.iter().map(|e| evaluar_expr(e, executor, funcs)).collect();
            Valor::Array(valores)
        }
        Expr::Index { array, index } => {
            // Obtener el array
            let array_val = evaluar_expr(array, executor, funcs);
            let index_val = evaluar_expr(index, executor, funcs);

            // Verificar que sea un array
            if let Valor::Array(arr) = array_val {
                // Verificar que el índice sea un número
                if let Valor::Num(i) = index_val {
                    let idx = i as usize;
                    if idx < arr.len() {
                        return arr[idx].clone();
                    } else {
                        return Valor::Error(format!("Índice {} fuera de rango (len={})", idx, arr.len()));
                    }
                } else {
                    return Valor::Error("El índice debe ser un número".to_string());
                }
            } else {
                return Valor::Error("Solo se puede indexar arrays".to_string());
            }
        }
        Expr::Call { name, args } => {
            // Llamada a función builtin: tecla_presionada("tecla")
            if name == "tecla_presionada" && args.len() == 1 {
                // Función especial para input - retorna 0 por defecto (no presionada)
                // El valor real se obtiene del contexto gráfico
                return Valor::Num(0.0);
            }

            // Funciones aritméticas builtin
            if name == "sumar" && args.len() >= 2 {
                let mut suma = 0.0;
                for arg in args {
                    if let Valor::Num(n) = evaluar_expr(arg, executor, funcs) {
                        suma += n;
                    } else {
                        return Valor::Error("sumar() requiere números".to_string());
                    }
                }
                return Valor::Num(suma);
            }

            if name == "restar" && args.len() == 2 {
                let a = evaluar_expr(&args[0], executor, funcs);
                let b = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Num(a), Valor::Num(b)) = (a, b) {
                    return Valor::Num(a - b);
                } else {
                    return Valor::Error("restar() requiere números".to_string());
                }
            }

            if name == "multiplicar" && args.len() >= 2 {
                let mut producto = 1.0;
                for arg in args {
                    if let Valor::Num(n) = evaluar_expr(arg, executor, funcs) {
                        producto *= n;
                    } else {
                        return Valor::Error("multiplicar() requiere números".to_string());
                    }
                }
                return Valor::Num(producto);
            }

            if name == "dividir" && args.len() == 2 {
                let a = evaluar_expr(&args[0], executor, funcs);
                let b = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Num(a), Valor::Num(b)) = (a, b) {
                    if b != 0.0 {
                        return Valor::Num(a / b);
                    } else {
                        return Valor::Error("División por cero".to_string());
                    }
                } else {
                    return Valor::Error("dividir() requiere números".to_string());
                }
            }

            // ========================================================================
            // FUNCIONES MATH AVANZADAS - V0.3.0 (Tank Combat)
            // ========================================================================
            
            // math::sqrt(x) - Raíz cuadrada
            if (name == "__math_sqrt" || name == "math::sqrt") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    if x >= 0.0 {
                        return Valor::Num(x.sqrt());
                    } else {
                        return Valor::Error("math::sqrt() requiere número >= 0".to_string());
                    }
                } else {
                    return Valor::Error("math::sqrt() requiere número".to_string());
                }
            }

            // math::sin(x) - Seno (x en radianes)
            if (name == "__math_sin" || name == "math::sin") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.sin());
                } else {
                    return Valor::Error("math::sin() requiere número".to_string());
                }
            }

            // math::cos(x) - Coseno (x en radianes)
            if (name == "__math_cos" || name == "math::cos") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.cos());
                } else {
                    return Valor::Error("math::cos() requiere número".to_string());
                }
            }

            // math::tan(x) - Tangente (x en radianes)
            if (name == "__math_tan" || name == "math::tan") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.tan());
                } else {
                    return Valor::Error("math::tan() requiere número".to_string());
                }
            }

            // math::atan2(y, x) - Arcotangente de y/x (retorna radianes)
            if (name == "__math_atan2" || name == "math::atan2") && args.len() == 2 {
                let y_val = evaluar_expr(&args[0], executor, funcs);
                let x_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Num(y), Valor::Num(x)) = (y_val, x_val) {
                    return Valor::Num(y.atan2(x));
                } else {
                    return Valor::Error("math::atan2() requiere dos números".to_string());
                }
            }

            // math::deg2rad(x) - Convertir grados a radianes
            if (name == "__math_deg2rad" || name == "math::deg2rad") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.to_radians());
                } else {
                    return Valor::Error("math::deg2rad() requiere número".to_string());
                }
            }

            // math::rad2deg(x) - Convertir radianes a grados
            if (name == "__math_rad2deg" || name == "math::rad2deg") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.to_degrees());
                } else {
                    return Valor::Error("math::rad2deg() requiere número".to_string());
                }
            }

            // ========== FUNCIONES STRING (v0.1.2) ==========
            // Soporte para strings::length, strings::upper, etc.
            if (name == "__str_length" || name == "strings::length") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(s.len() as f64);
                } else {
                    return Valor::Error("strings::length() requiere string".to_string());
                }
            }

            if (name == "__str_upper" || name == "strings::upper") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Texto(s.to_uppercase());
                } else {
                    return Valor::Error("strings::upper() requiere string".to_string());
                }
            }

            if (name == "__str_lower" || name == "strings::lower") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Texto(s.to_lowercase());
                } else {
                    return Valor::Error("strings::lower() requiere string".to_string());
                }
            }

            if (name == "__str_concat" || name == "strings::concat") && args.len() == 2 {
                let a = evaluar_expr(&args[0], executor, funcs);
                let b = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(a), Valor::Texto(b)) = (a, b) {
                    return Valor::Texto(format!("{}{}", a, b));
                } else {
                    return Valor::Error("strings::concat() requiere dos strings".to_string());
                }
            }

            if (name == "__str_trim" || name == "strings::trim") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Texto(s.trim().to_string());
                } else {
                    return Valor::Error("strings::trim() requiere string".to_string());
                }
            }

            if (name == "__str_substr" || name == "strings::substr") && args.len() == 3 {
                let s_val = evaluar_expr(&args[0], executor, funcs);
                let start_val = evaluar_expr(&args[1], executor, funcs);
                let len_val = evaluar_expr(&args[2], executor, funcs);
                if let (Valor::Texto(s), Valor::Num(start), Valor::Num(len)) = (s_val, start_val, len_val) {
                    let start_idx = start as usize;
                    let length = len as usize;
                    if start_idx + length <= s.len() {
                        return Valor::Texto(s[start_idx..start_idx + length].to_string());
                    } else {
                        return Valor::Error("strings::substr(): índices fuera de rango".to_string());
                    }
                } else {
                    return Valor::Error("strings::substr() requiere (string, inicio, longitud)".to_string());
                }
            }

            if (name == "__str_replace" || name == "strings::replace") && args.len() == 3 {
                let s_val = evaluar_expr(&args[0], executor, funcs);
                let buscar_val = evaluar_expr(&args[1], executor, funcs);
                let reemplazar_val = evaluar_expr(&args[2], executor, funcs);
                if let (Valor::Texto(s), Valor::Texto(buscar), Valor::Texto(reemplazar)) = (s_val, buscar_val, reemplazar_val) {
                    return Valor::Texto(s.replace(&buscar, &reemplazar));
                } else {
                    return Valor::Error("strings::replace() requiere tres strings".to_string());
                }
            }

            // ========== NUEVAS FUNCIONES STRINGS (v0.1.4) ==========
            if (name == "__str_split" || name == "strings::split") && args.len() == 2 {
                let s_val = evaluar_expr(&args[0], executor, funcs);
                let sep_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(s), Valor::Texto(sep)) = (s_val, sep_val) {
                    let partes: Vec<Valor> = s.split(&sep).map(|p| Valor::Texto(p.to_string())).collect();
                    return Valor::Array(partes);
                } else {
                    return Valor::Error("strings::split() requiere (string, separador)".to_string());
                }
            }

            if (name == "__str_starts_with" || name == "strings::starts_with") && args.len() == 2 {
                let s_val = evaluar_expr(&args[0], executor, funcs);
                let prefix_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(s), Valor::Texto(prefix)) = (s_val, prefix_val) {
                    return Valor::Bool(s.starts_with(&prefix));
                } else {
                    return Valor::Error("strings::starts_with() requiere dos strings".to_string());
                }
            }

            if (name == "__str_ends_with" || name == "strings::ends_with") && args.len() == 2 {
                let s_val = evaluar_expr(&args[0], executor, funcs);
                let suffix_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(s), Valor::Texto(suffix)) = (s_val, suffix_val) {
                    return Valor::Bool(s.ends_with(&suffix));
                } else {
                    return Valor::Error("strings::ends_with() requiere dos strings".to_string());
                }
            }

            if (name == "__str_replace_all" || name == "strings::replace_all") && args.len() == 3 {
                let s_val = evaluar_expr(&args[0], executor, funcs);
                let buscar_val = evaluar_expr(&args[1], executor, funcs);
                let reemplazar_val = evaluar_expr(&args[2], executor, funcs);
                if let (Valor::Texto(s), Valor::Texto(buscar), Valor::Texto(reemplazar)) = (s_val, buscar_val, reemplazar_val) {
                    return Valor::Texto(s.replace(&buscar, &reemplazar));
                } else {
                    return Valor::Error("strings::replace_all() requiere tres strings".to_string());
                }
            }

            if (name == "__str_join" || name == "strings::join") && args.len() == 2 {
                let sep_val = evaluar_expr(&args[0], executor, funcs);
                let arr_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(sep), Valor::Array(arr)) = (sep_val, arr_val) {
                    let strs: Result<Vec<String>, _> = arr.iter().map(|v| {
                        if let Valor::Texto(s) = v {
                            Ok(s.clone())
                        } else {
                            Err("strings::join() requiere array de strings")
                        }
                    }).collect();
                    match strs {
                        Ok(parts) => return Valor::Texto(parts.join(&sep)),
                        Err(msg) => return Valor::Error(msg.to_string()),
                    }
                } else {
                    return Valor::Error("strings::join() requiere (separador, array)".to_string());
                }
            }

            // ========== FUNCIONES IO (v0.1.2) ==========
            if (name == "__file_read" || name == "io::read_file") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr(&args[0], executor, funcs) {
                    match std::fs::read_to_string(&path) {
                        Ok(content) => return Valor::Texto(content),
                        Err(e) => return Valor::Error(format!("io::read_file(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::read_file() requiere path (string)".to_string());
                }
            }

            if (name == "__file_write" || name == "io::write_file") && args.len() == 2 {
                let path_val = evaluar_expr(&args[0], executor, funcs);
                let content_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(path), Valor::Texto(content)) = (path_val, content_val) {
                    match std::fs::write(&path, &content) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::write_file(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::write_file() requiere (path, content)".to_string());
                }
            }

            if (name == "__file_exists" || name == "io::file_exists") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Bool(std::path::Path::new(&path).exists());
                } else {
                    return Valor::Error("io::file_exists() requiere path (string)".to_string());
                }
            }

            // ========== NUEVAS FUNCIONES IO (v0.1.4) ==========
            if (name == "__dir_mkdir" || name == "io::mkdir") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr(&args[0], executor, funcs) {
                    match std::fs::create_dir_all(&path) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::mkdir(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::mkdir() requiere path (string)".to_string());
                }
            }

            if (name == "__file_remove" || name == "io::remove") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr(&args[0], executor, funcs) {
                    match std::fs::remove_file(&path) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => {
                            // Intentar como directorio
                            match std::fs::remove_dir_all(&path) {
                                Ok(_) => return Valor::Num(1.0),
                                Err(_) => return Valor::Error(format!("io::remove(): {}", e)),
                            }
                        }
                    }
                } else {
                    return Valor::Error("io::remove() requiere path (string)".to_string());
                }
            }

            if (name == "__file_rename" || name == "io::rename") && args.len() == 2 {
                let old_val = evaluar_expr(&args[0], executor, funcs);
                let new_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(old), Valor::Texto(new)) = (old_val, new_val) {
                    match std::fs::rename(&old, &new) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::rename(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::rename() requiere (old, new)".to_string());
                }
            }

            if (name == "__file_copy" || name == "io::copy") && args.len() == 2 {
                let src_val = evaluar_expr(&args[0], executor, funcs);
                let dst_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(src), Valor::Texto(dst)) = (src_val, dst_val) {
                    match std::fs::copy(&src, &dst) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::copy(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::copy() requiere (src, dst)".to_string());
                }
            }

            // ========== NUEVAS FUNCIONES ARRAYS (v0.1.4) ==========
            if (name == "__array_push" || name == "arrays::push") && args.len() == 2 {
                let arr_val = evaluar_expr(&args[0], executor, funcs);
                let elem_val = evaluar_expr(&args[1], executor, funcs);
                if let Valor::Array(mut arr) = arr_val {
                    arr.push(elem_val);
                    return Valor::Array(arr);
                } else {
                    return Valor::Error("arrays::push() requiere (array, elemento)".to_string());
                }
            }

            if (name == "__array_pop" || name == "arrays::pop") && args.len() == 1 {
                if let Valor::Array(mut arr) = evaluar_expr(&args[0], executor, funcs) {
                    if arr.is_empty() {
                        return Valor::Error("arrays::pop(): array vacío".to_string());
                    }
                    let last = arr.pop().unwrap();
                    return last;
                } else {
                    return Valor::Error("arrays::pop() requiere array".to_string());
                }
            }

            if (name == "__array_shift" || name == "arrays::shift") && args.len() == 1 {
                if let Valor::Array(mut arr) = evaluar_expr(&args[0], executor, funcs) {
                    if arr.is_empty() {
                        return Valor::Error("arrays::shift(): array vacío".to_string());
                    }
                    let first = arr.remove(0);
                    return first;
                } else {
                    return Valor::Error("arrays::shift() requiere array".to_string());
                }
            }

            if (name == "__array_unshift" || name == "arrays::unshift") && args.len() == 2 {
                let arr_val = evaluar_expr(&args[0], executor, funcs);
                let elem_val = evaluar_expr(&args[1], executor, funcs);
                if let Valor::Array(mut arr) = arr_val {
                    arr.insert(0, elem_val);
                    return Valor::Array(arr);
                } else {
                    return Valor::Error("arrays::unshift() requiere (array, elemento)".to_string());
                }
            }

            if (name == "__array_slice" || name == "arrays::slice") && args.len() == 3 {
                let arr_val = evaluar_expr(&args[0], executor, funcs);
                let start_val = evaluar_expr(&args[1], executor, funcs);
                let end_val = evaluar_expr(&args[2], executor, funcs);
                if let (Valor::Array(arr), Valor::Num(start), Valor::Num(end)) = (arr_val, start_val, end_val) {
                    let s = start as usize;
                    let e = end as usize;
                    if s <= e && e <= arr.len() {
                        let sliced: Vec<Valor> = arr[s..e].to_vec();
                        return Valor::Array(sliced);
                    } else {
                        return Valor::Error("arrays::slice(): índices inválidos".to_string());
                    }
                } else {
                    return Valor::Error("arrays::slice() requiere (array, inicio, fin)".to_string());
                }
            }

            if (name == "__array_reverse" || name == "arrays::reverse") && args.len() == 1 {
                if let Valor::Array(mut arr) = evaluar_expr(&args[0], executor, funcs) {
                    arr.reverse();
                    return Valor::Array(arr);
                } else {
                    return Valor::Error("arrays::reverse() requiere array".to_string());
                }
            }

            // ========== FUNCIONES RANDOM (v0.1.6) ==========
            // PRNG xorshift - sin dependencias externas
            if (name == "__random_int" || name == "random::int") && args.len() == 2 {
                let min_val = evaluar_expr(&args[0], executor, funcs);
                let max_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Num(min), Valor::Num(max)) = (min_val, max_val) {
                    let seed = executor.leer("__random_seed").unwrap_or(Valor::Num(12345.0));
                    let mut s = if let Valor::Num(n) = seed { n as u32 } else { 12345 };
                    // xorshift PRNG
                    s ^= s << 13;
                    s ^= s >> 17;
                    s ^= s << 5;
                    executor.guardar("__random_seed", Valor::Num(s as f64));
                    // Mapear a rango [min, max] - TRUNCAR A ENTERO
                    let range = (max - min).abs() + 1.0;
                    let random_val = (s as f64 / u32::MAX as f64) * range;
                    // Truncar a entero y asegurar que esté en rango
                    let entero = (min + random_val.floor()) as i64;
                    let min_i = min.min(max) as i64;
                    let max_i = max.min(max) as i64;
                    let result = entero.max(min_i).min(max_i);
                    return Valor::Num(result as f64);
                } else {
                    return Valor::Error("random::int() requiere (min, max) números".to_string());
                }
            }

            if (name == "__random_float" || name == "random::float") && args.len() == 0 {
                let seed = executor.leer("__random_seed").unwrap_or(Valor::Num(12345.0));
                let mut s = if let Valor::Num(n) = seed { n as u32 } else { 12345 };
                // xorshift PRNG
                s ^= s << 13;
                s ^= s >> 17;
                s ^= s << 5;
                executor.guardar("__random_seed", Valor::Num(s as f64));
                return Valor::Num(s as f64 / u32::MAX as f64);
            }

            if (name == "__random_choice" || name == "random::choice") && args.len() == 1 {
                let arr_val = evaluar_expr(&args[0], executor, funcs);
                if let Valor::Array(arr) = arr_val {
                    if arr.is_empty() {
                        return Valor::Error("random::choice(): array vacío".to_string());
                    }
                    let seed = executor.leer("__random_seed").unwrap_or(Valor::Num(12345.0));
                    let mut s = if let Valor::Num(n) = seed { n as u32 } else { 12345 };
                    // xorshift PRNG
                    s ^= s << 13;
                    s ^= s >> 17;
                    s ^= s << 5;
                    executor.guardar("__random_seed", Valor::Num(s as f64));
                    let idx = (s as usize) % arr.len();
                    return arr[idx].clone();
                } else {
                    return Valor::Error("random::choice() requiere array".to_string());
                }
            }

            // ========== FUNCIONES JSON (v0.1.5) ==========
            if (name == "__json_parse" || name == "json::parse") && args.len() == 1 {
                if let Valor::Texto(json_str) = evaluar_expr(&args[0], executor, funcs) {
                    match serde_json::from_str::<serde_json::Value>(&json_str) {
                        Ok(val) => {
                            // Convertir serde_json::Value a Valor::Rydit
                            return valor_serde_a_rydit(&val);
                        }
                        Err(e) => return Valor::Error(format!("json::parse(): {}", e)),
                    }
                } else {
                    return Valor::Error("json::parse() requiere string JSON".to_string());
                }
            }

            if (name == "__json_stringify" || name == "json::stringify") && args.len() == 1 {
                let val = evaluar_expr(&args[0], executor, funcs);
                match valor_rydit_a_serde(&val) {
                    Ok(serde_val) => {
                        match serde_json::to_string(&serde_val) {
                            Ok(json_str) => return Valor::Texto(json_str),
                            Err(e) => return Valor::Error(format!("json::stringify(): {}", e)),
                        }
                    }
                    Err(e) => return Valor::Error(format!("json::stringify(): {}", e)),
                }
            }

            // ========== FUNCIONES TIME (v0.1.6) ==========
            if (name == "__time_now" || name == "time::now") && args.len() == 0 {
                use std::time::{SystemTime, UNIX_EPOCH};
                match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(duration) => return Valor::Num(duration.as_secs_f64()),
                    Err(e) => return Valor::Error(format!("time::now(): {}", e)),
                }
            }

            if (name == "__time_sleep" || name == "time::sleep") && args.len() == 1 {
                use std::{thread, time::Duration};
                let ms_val = evaluar_expr(&args[0], executor, funcs);
                if let Valor::Num(ms) = ms_val {
                    thread::sleep(Duration::from_millis(ms as u64));
                    return Valor::Vacio;
                } else {
                    return Valor::Error("time::sleep() requiere milisegundos (número)".to_string());
                }
            }

            // ========== FUNCIONES REGEX (v0.6.2) ==========
            if (name == "__regex_match" || name == "regex::match") && args.len() == 2 {
                if let (Valor::Texto(pattern), Valor::Texto(text)) = (&evaluar_expr(&args[0], executor, funcs), &evaluar_expr(&args[1], executor, funcs)) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => return Valor::Bool(re.is_match(text)),
                        Err(e) => return Valor::Error(format!("regex::match(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::match() requiere (patrón, texto)".to_string());
                }
            }

            if (name == "__regex_replace" || name == "regex::replace") && args.len() == 3 {
                if let (Valor::Texto(pattern), Valor::Texto(replacement), Valor::Texto(text)) = (
                    &evaluar_expr(&args[0], executor, funcs),
                    &evaluar_expr(&args[1], executor, funcs),
                    &evaluar_expr(&args[2], executor, funcs)
                ) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => return Valor::Texto(re.replace_all(text, replacement.as_str()).to_string()),
                        Err(e) => return Valor::Error(format!("regex::replace(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::replace() requiere (patrón, reemplazo, texto)".to_string());
                }
            }

            if (name == "__regex_split" || name == "regex::split") && args.len() == 2 {
                if let (Valor::Texto(pattern), Valor::Texto(text)) = (&evaluar_expr(&args[0], executor, funcs), &evaluar_expr(&args[1], executor, funcs)) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => {
                            let parts: Vec<Valor> = re.split(text).map(|s| Valor::Texto(s.to_string())).collect();
                            return Valor::Array(parts);
                        }
                        Err(e) => return Valor::Error(format!("regex::split(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::split() requiere (patrón, texto)".to_string());
                }
            }

            if (name == "__regex_find_all" || name == "regex::find_all") && args.len() == 2 {
                if let (Valor::Texto(pattern), Valor::Texto(text)) = (&evaluar_expr(&args[0], executor, funcs), &evaluar_expr(&args[1], executor, funcs)) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => {
                            let matches: Vec<Valor> = re.find_iter(text).map(|m| Valor::Texto(m.as_str().to_string())).collect();
                            return Valor::Array(matches);
                        }
                        Err(e) => return Valor::Error(format!("regex::find_all(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::find_all() requiere (patrón, texto)".to_string());
                }
            }

            if (name == "__regex_capture" || name == "regex::capture") && args.len() == 2 {
                if let (Valor::Texto(pattern), Valor::Texto(text)) = (&evaluar_expr(&args[0], executor, funcs), &evaluar_expr(&args[1], executor, funcs)) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => {
                            if let Some(caps) = re.captures(text) {
                                // Retornar array: [match completo, grupo1, grupo2, ...]
                                let mut result: Vec<Valor> = Vec::new();
                                // Match completo
                                result.push(Valor::Texto(caps.get(0).unwrap().as_str().to_string()));
                                // Grupos de captura
                                for i in 1..caps.len() {
                                    if let Some(m) = caps.get(i) {
                                        result.push(Valor::Texto(m.as_str().to_string()));
                                    } else {
                                        result.push(Valor::Vacio);
                                    }
                                }
                                return Valor::Array(result);
                            } else {
                                return Valor::Array(vec![]); // No match
                            }
                        }
                        Err(e) => return Valor::Error(format!("regex::capture(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::capture() requiere (patrón, texto)".to_string());
                }
            }

            // Función de usuario - ejecutar y capturar retorno
            // Ahora tenemos &mut Executor, podemos ejecutar la función
            // Clonar datos para evitar borrow checker issues
            
            // Primero buscar con nombre completo (math::sumar)
            // Si no existe, intentar buscar solo el nombre después de ::
            let func_name = if name.contains("::") {
                // Es nombre con namespace, intentar con nombre completo primero
                if funcs.contains_key(name) {
                    name.clone()
                } else {
                    // Extraer nombre después de ::
                    name.split("::").last().unwrap_or(&name).to_string()
                }
            } else {
                name.clone()
            };
            
            let func_data = funcs.get(&func_name).map(|(p, b)| (p.clone(), b.clone()));
            
            if let Some((params, body)) = func_data {
                // Evaluar argumentos
                let mut arg_values = vec![];
                for arg in args {
                    arg_values.push(evaluar_expr(arg, executor, funcs));
                }

                // Crear scope local para la función
                executor.push_scope();

                // Mapear parámetros → valores de argumentos en scope local
                for (i, param) in params.iter().enumerate() {
                    if i < arg_values.len() {
                        executor.guardar_local(param, arg_values[i].clone());
                    }
                }

                // Ejecutar body de la función y capturar retorno
                // Nota: las funciones no pueden tener imports, así que usamos contextos vacíos
                let mut empty_loaded: HashSet<String> = HashSet::new();
                let mut empty_stack: Vec<String> = Vec::new();
                
                let mut return_value: Option<Valor> = None;
                for s in &body {
                    match ejecutar_stmt(s, executor, funcs, &mut empty_loaded, &mut empty_stack) {
                        (Some(true), _) => {
                            // Break en función - salir
                            executor.pop_scope();
                            return Valor::Error("Break no permitido en función de expresión".to_string());
                        }
                        (_, Some(val)) => {
                            return_value = Some(val);
                            break;  // Hay retorno
                        }
                        _ => {}
                    }
                }

                // Pop scope al finalizar
                executor.pop_scope();

                // Retornar valor o Vacio si no hubo return explícito
                return return_value.unwrap_or(Valor::Vacio);
            }

            Valor::Error(format!("Función '{}' no soportada en expresiones", name))
        }
        Expr::BinOp { left, op, right } => {
            let left_val = evaluar_expr(left, executor, funcs);
            let right_val = evaluar_expr(right, executor, funcs);

            // Operadores lógicos (usan referencias, no mueven valores)
            match op {
                lizer::BinOp::And => {
                    let l_bool = valor_a_bool(&left_val);
                    let r_bool = valor_a_bool(&right_val);
                    return Valor::Bool(l_bool && r_bool);
                }
                lizer::BinOp::Or => {
                    let l_bool = valor_a_bool(&left_val);
                    let r_bool = valor_a_bool(&right_val);
                    return Valor::Bool(l_bool || r_bool);
                }
                _ => {}
            }

            // Concatenación de strings con + (con coerción automática de números)
            if matches!(op, lizer::BinOp::Suma) {
                match (&left_val, &right_val) {
                    (Valor::Texto(l), Valor::Texto(r)) => {
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Texto(l), Valor::Num(r)) => {
                        // "texto" + numero -> "texto123"
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Num(l), Valor::Texto(r)) => {
                        // numero + "texto" -> "123texto"
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Num(_), Valor::Num(_)) => {
                        // numero + numero -> suma aritmética (comportamiento normal, se maneja abajo)
                    }
                    _ => {}
                }
            }

            // Operadores aritméticos/comparación (mueven valores Num)
            if let (Valor::Num(l), Valor::Num(r)) = (left_val, right_val) {
                return match op {
                    lizer::BinOp::Suma => Valor::Num(l + r),
                    lizer::BinOp::Resta => Valor::Num(l - r),
                    lizer::BinOp::Mult => Valor::Num(l * r),
                    lizer::BinOp::Div => {
                        if r != 0.0 { Valor::Num(l / r) } else { Valor::Error("División por cero".to_string()) }
                    }
                    lizer::BinOp::Mayor => Valor::Bool(l > r),
                    lizer::BinOp::Menor => Valor::Bool(l < r),
                    lizer::BinOp::Igual => Valor::Bool((l - r).abs() < 0.0001),
                    lizer::BinOp::MayorIgual => Valor::Bool(l >= r),
                    lizer::BinOp::MenorIgual => Valor::Bool(l <= r),
                    _ => Valor::Error("Operador no soportado".to_string())
                };
            }

            Valor::Error("Operación inválida".to_string())
        }
        Expr::Unary { op, expr } => {
            let val = evaluar_expr(expr, executor, funcs);
            match op {
                lizer::UnaryOp::Not => {
                    let b = valor_a_bool(&val);
                    Valor::Bool(!b)
                }
                lizer::UnaryOp::Neg => {
                    if let Valor::Num(n) = val {
                        Valor::Num(-n)
                    } else {
                        Valor::Error("Neg requiere número".to_string())
                    }
                }
            }
        }
    }
}

// ============================================================================
// EVALUAR EXPRESIÓN (MODO GRÁFICO CON INPUT)
// ============================================================================

fn evaluar_expr_gfx(expr: &Expr, executor: &mut Executor, input: &InputEstado, funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>) -> Valor {
    match expr {
        Expr::Num(n) => Valor::Num(*n),
        Expr::Texto(s) => Valor::Texto(s.clone()),
        Expr::Var(name) => {
            if name == "__INPUT__" {
                return executor.input("> ");
            }
            executor.leer(name).unwrap_or(Valor::Vacio)
        }
        Expr::Bool(b) => Valor::Bool(*b),
        Expr::Array(elements) => {
            let valores: Vec<Valor> = elements.iter().map(|e| evaluar_expr_gfx(e, executor, input, funcs)).collect();
            Valor::Array(valores)
        }
        Expr::Index { array, index } => {
            let array_val = evaluar_expr_gfx(array, executor, input, funcs);
            let index_val = evaluar_expr_gfx(index, executor, input, funcs);

            if let Valor::Array(arr) = array_val {
                if let Valor::Num(i) = index_val {
                    let idx = i as usize;
                    if idx < arr.len() {
                        return arr[idx].clone();
                    } else {
                        return Valor::Error(format!("Índice {} fuera de rango (len={})", idx, arr.len()));
                    }
                } else {
                    return Valor::Error("El índice debe ser un número".to_string());
                }
            } else {
                return Valor::Error("Solo se puede indexar arrays".to_string());
            }
        }
        Expr::Call { name, args } => {
            // tecla_presionada("tecla") - retorna 1.0 si presionada, 0.0 si no
            if name == "tecla_presionada" && args.len() == 1 {
                if let Expr::Texto(tecla) = &args[0] {
                    let presionada = input.es_presionada(tecla);
                    return Valor::Num(if presionada { 1.0 } else { 0.0 });
                }
            }

            // ========================================================================
            // INPUT MOUSE - V0.3.0 (Tank Combat)
            // ========================================================================
            
            // input::mouse_x() - Retorna posición X del mouse
            if name == "input::mouse_x" || name == "__input_mouse_x" {
                return Valor::Num(input.mouse_x as f64);
            }

            // input::mouse_y() - Retorna posición Y del mouse
            if name == "input::mouse_y" || name == "__input_mouse_y" {
                return Valor::Num(input.mouse_y as f64);
            }

            // input::mouse_position() - Retorna [x, y]
            if name == "input::mouse_position" || name == "__input_mouse_position" {
                return Valor::Array(vec![Valor::Num(input.mouse_x as f64), Valor::Num(input.mouse_y as f64)]);
            }

            // input::is_mouse_button_pressed(button) - 0=izq, 1=der, 2=medio
            if (name == "input::is_mouse_button_pressed" || name == "__input_is_mouse_button_pressed") && args.len() == 1 {
                if let Valor::Num(button) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    // Usamos el input estado para botones de mouse
                    let presionado = match button as i32 {
                        0 => input.mouse_left,
                        1 => input.mouse_right,
                        2 => input.mouse_middle,
                        _ => false,
                    };
                    return Valor::Num(if presionado { 1.0 } else { 0.0 });
                }
            }

            // Funciones aritméticas builtin
            if name == "sumar" && args.len() >= 2 {
                let mut suma = 0.0;
                for arg in args {
                    if let Valor::Num(n) = evaluar_expr_gfx(arg, executor, input, funcs) {
                        suma += n;
                    } else {
                        return Valor::Error("sumar() requiere números".to_string());
                    }
                }
                return Valor::Num(suma);
            }
            
            if name == "restar" && args.len() == 2 {
                let a = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let b = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Num(a), Valor::Num(b)) = (a, b) {
                    return Valor::Num(a - b);
                } else {
                    return Valor::Error("restar() requiere números".to_string());
                }
            }
            
            if name == "multiplicar" && args.len() >= 2 {
                let mut producto = 1.0;
                for arg in args {
                    if let Valor::Num(n) = evaluar_expr_gfx(arg, executor, input, funcs) {
                        producto *= n;
                    } else {
                        return Valor::Error("multiplicar() requiere números".to_string());
                    }
                }
                return Valor::Num(producto);
            }
            
            if name == "dividir" && args.len() == 2 {
                let a = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let b = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Num(a), Valor::Num(b)) = (a, b) {
                    if b != 0.0 {
                        return Valor::Num(a / b);
                    } else {
                        return Valor::Error("División por cero".to_string());
                    }
                } else {
                    return Valor::Error("dividir() requiere números".to_string());
                }
            }

            // ========================================================================
            // FUNCIONES MATH AVANZADAS - V0.3.0 (Tank Combat)
            // ========================================================================
            
            // math::sqrt(x) - Raíz cuadrada
            if (name == "__math_sqrt" || name == "math::sqrt") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    if x >= 0.0 {
                        return Valor::Num(x.sqrt());
                    } else {
                        return Valor::Error("math::sqrt() requiere número >= 0".to_string());
                    }
                } else {
                    return Valor::Error("math::sqrt() requiere número".to_string());
                }
            }

            // math::sin(x) - Seno (x en radianes)
            if (name == "__math_sin" || name == "math::sin") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Num(x.sin());
                } else {
                    return Valor::Error("math::sin() requiere número".to_string());
                }
            }

            // math::cos(x) - Coseno (x en radianes)
            if (name == "__math_cos" || name == "math::cos") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Num(x.cos());
                } else {
                    return Valor::Error("math::cos() requiere número".to_string());
                }
            }

            // math::tan(x) - Tangente (x en radianes)
            if (name == "__math_tan" || name == "math::tan") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Num(x.tan());
                } else {
                    return Valor::Error("math::tan() requiere número".to_string());
                }
            }

            // math::atan2(y, x) - Arcotangente de y/x (retorna radianes)
            if (name == "__math_atan2" || name == "math::atan2") && args.len() == 2 {
                let y_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let x_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Num(y), Valor::Num(x)) = (y_val, x_val) {
                    return Valor::Num(y.atan2(x));
                } else {
                    return Valor::Error("math::atan2() requiere dos números".to_string());
                }
            }

            // math::deg2rad(x) - Convertir grados a radianes
            if (name == "__math_deg2rad" || name == "math::deg2rad") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Num(x.to_radians());
                } else {
                    return Valor::Error("math::deg2rad() requiere número".to_string());
                }
            }

            // math::rad2deg(x) - Convertir radianes a grados
            if (name == "__math_rad2deg" || name == "math::rad2deg") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Num(x.to_degrees());
                } else {
                    return Valor::Error("math::rad2deg() requiere número".to_string());
                }
            }

            // ========== FUNCIONES STRING (v0.1.2) ==========
            // Soporte para strings::length, strings::upper, etc.
            if (name == "__str_length" || name == "strings::length") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Num(s.len() as f64);
                } else {
                    return Valor::Error("strings::length() requiere string".to_string());
                }
            }

            if (name == "__str_upper" || name == "strings::upper") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Texto(s.to_uppercase());
                } else {
                    return Valor::Error("strings::upper() requiere string".to_string());
                }
            }

            if (name == "__str_lower" || name == "strings::lower") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Texto(s.to_lowercase());
                } else {
                    return Valor::Error("strings::lower() requiere string".to_string());
                }
            }

            if (name == "__str_concat" || name == "strings::concat") && args.len() == 2 {
                let a = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let b = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Texto(a), Valor::Texto(b)) = (a, b) {
                    return Valor::Texto(format!("{}{}", a, b));
                } else {
                    return Valor::Error("strings::concat() requiere dos strings".to_string());
                }
            }

            if (name == "__str_trim" || name == "strings::trim") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Texto(s.trim().to_string());
                } else {
                    return Valor::Error("strings::trim() requiere string".to_string());
                }
            }

            if (name == "__str_substr" || name == "strings::substr") && args.len() == 3 {
                let s_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let start_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                let len_val = evaluar_expr_gfx(&args[2], executor, input, funcs);
                if let (Valor::Texto(s), Valor::Num(start), Valor::Num(len)) = (s_val, start_val, len_val) {
                    let start_idx = start as usize;
                    let length = len as usize;
                    if start_idx + length <= s.len() {
                        return Valor::Texto(s[start_idx..start_idx + length].to_string());
                    } else {
                        return Valor::Error("strings::substr(): índices fuera de rango".to_string());
                    }
                } else {
                    return Valor::Error("strings::substr() requiere (string, inicio, longitud)".to_string());
                }
            }

            if (name == "__str_replace" || name == "strings::replace") && args.len() == 3 {
                let s_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let buscar_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                let reemplazar_val = evaluar_expr_gfx(&args[2], executor, input, funcs);
                if let (Valor::Texto(s), Valor::Texto(buscar), Valor::Texto(reemplazar)) = (s_val, buscar_val, reemplazar_val) {
                    return Valor::Texto(s.replace(&buscar, &reemplazar));
                } else {
                    return Valor::Error("strings::replace() requiere tres strings".to_string());
                }
            }

            // ========== NUEVAS FUNCIONES STRINGS (v0.1.4) ==========
            if (name == "__str_split" || name == "strings::split") && args.len() == 2 {
                let s_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let sep_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Texto(s), Valor::Texto(sep)) = (s_val, sep_val) {
                    let partes: Vec<Valor> = s.split(&sep).map(|p| Valor::Texto(p.to_string())).collect();
                    return Valor::Array(partes);
                } else {
                    return Valor::Error("strings::split() requiere (string, separador)".to_string());
                }
            }

            if (name == "__str_starts_with" || name == "strings::starts_with") && args.len() == 2 {
                let s_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let prefix_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Texto(s), Valor::Texto(prefix)) = (s_val, prefix_val) {
                    return Valor::Bool(s.starts_with(&prefix));
                } else {
                    return Valor::Error("strings::starts_with() requiere dos strings".to_string());
                }
            }

            if (name == "__str_ends_with" || name == "strings::ends_with") && args.len() == 2 {
                let s_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let suffix_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Texto(s), Valor::Texto(suffix)) = (s_val, suffix_val) {
                    return Valor::Bool(s.ends_with(&suffix));
                } else {
                    return Valor::Error("strings::ends_with() requiere dos strings".to_string());
                }
            }

            if (name == "__str_replace_all" || name == "strings::replace_all") && args.len() == 3 {
                let s_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let buscar_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                let reemplazar_val = evaluar_expr_gfx(&args[2], executor, input, funcs);
                if let (Valor::Texto(s), Valor::Texto(buscar), Valor::Texto(reemplazar)) = (s_val, buscar_val, reemplazar_val) {
                    return Valor::Texto(s.replace(&buscar, &reemplazar));
                } else {
                    return Valor::Error("strings::replace_all() requiere tres strings".to_string());
                }
            }

            if (name == "__str_join" || name == "strings::join") && args.len() == 2 {
                let sep_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let arr_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Texto(sep), Valor::Array(arr)) = (sep_val, arr_val) {
                    let strs: Result<Vec<String>, _> = arr.iter().map(|v| {
                        if let Valor::Texto(s) = v {
                            Ok(s.clone())
                        } else {
                            Err("strings::join() requiere array de strings")
                        }
                    }).collect();
                    match strs {
                        Ok(parts) => return Valor::Texto(parts.join(&sep)),
                        Err(msg) => return Valor::Error(msg.to_string()),
                    }
                } else {
                    return Valor::Error("strings::join() requiere (separador, array)".to_string());
                }
            }

            // ========== FUNCIONES IO (v0.1.2) ==========
            if (name == "__file_read" || name == "io::read_file") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    match std::fs::read_to_string(&path) {
                        Ok(content) => return Valor::Texto(content),
                        Err(e) => return Valor::Error(format!("io::read_file(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::read_file() requiere path (string)".to_string());
                }
            }

            if (name == "__file_write" || name == "io::write_file") && args.len() == 2 {
                let path_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let content_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Texto(path), Valor::Texto(content)) = (path_val, content_val) {
                    match std::fs::write(&path, &content) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::write_file(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::write_file() requiere (path, content)".to_string());
                }
            }

            if (name == "__file_exists" || name == "io::file_exists") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    return Valor::Bool(std::path::Path::new(&path).exists());
                } else {
                    return Valor::Error("io::file_exists() requiere path (string)".to_string());
                }
            }

            // ========== NUEVAS FUNCIONES IO (v0.1.4) ==========
            if (name == "__dir_mkdir" || name == "io::mkdir") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    match std::fs::create_dir_all(&path) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::mkdir(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::mkdir() requiere path (string)".to_string());
                }
            }

            if (name == "__file_remove" || name == "io::remove") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    match std::fs::remove_file(&path) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => {
                            match std::fs::remove_dir_all(&path) {
                                Ok(_) => return Valor::Num(1.0),
                                Err(_) => return Valor::Error(format!("io::remove(): {}", e)),
                            }
                        }
                    }
                } else {
                    return Valor::Error("io::remove() requiere path (string)".to_string());
                }
            }

            if (name == "__file_rename" || name == "io::rename") && args.len() == 2 {
                let old_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let new_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Texto(old), Valor::Texto(new)) = (old_val, new_val) {
                    match std::fs::rename(&old, &new) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::rename(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::rename() requiere (old, new)".to_string());
                }
            }

            if (name == "__file_copy" || name == "io::copy") && args.len() == 2 {
                let src_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let dst_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Texto(src), Valor::Texto(dst)) = (src_val, dst_val) {
                    match std::fs::copy(&src, &dst) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::copy(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::copy() requiere (src, dst)".to_string());
                }
            }

            // ========== NUEVAS FUNCIONES ARRAYS (v0.1.4) ==========
            if (name == "__array_push" || name == "arrays::push") && args.len() == 2 {
                let arr_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let elem_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let Valor::Array(mut arr) = arr_val {
                    arr.push(elem_val);
                    return Valor::Array(arr);
                } else {
                    return Valor::Error("arrays::push() requiere (array, elemento)".to_string());
                }
            }

            if (name == "__array_pop" || name == "arrays::pop") && args.len() == 1 {
                if let Valor::Array(mut arr) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    if arr.is_empty() {
                        return Valor::Error("arrays::pop(): array vacío".to_string());
                    }
                    let last = arr.pop().unwrap();
                    return last;
                } else {
                    return Valor::Error("arrays::pop() requiere array".to_string());
                }
            }

            if (name == "__array_shift" || name == "arrays::shift") && args.len() == 1 {
                if let Valor::Array(mut arr) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    if arr.is_empty() {
                        return Valor::Error("arrays::shift(): array vacío".to_string());
                    }
                    let first = arr.remove(0);
                    return first;
                } else {
                    return Valor::Error("arrays::shift() requiere array".to_string());
                }
            }

            if (name == "__array_unshift" || name == "arrays::unshift") && args.len() == 2 {
                let arr_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let elem_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let Valor::Array(mut arr) = arr_val {
                    arr.insert(0, elem_val);
                    return Valor::Array(arr);
                } else {
                    return Valor::Error("arrays::unshift() requiere (array, elemento)".to_string());
                }
            }

            if (name == "__array_slice" || name == "arrays::slice") && args.len() == 3 {
                let arr_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let start_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                let end_val = evaluar_expr_gfx(&args[2], executor, input, funcs);
                if let (Valor::Array(arr), Valor::Num(start), Valor::Num(end)) = (arr_val, start_val, end_val) {
                    let s = start as usize;
                    let e = end as usize;
                    if s <= e && e <= arr.len() {
                        let sliced: Vec<Valor> = arr[s..e].to_vec();
                        return Valor::Array(sliced);
                    } else {
                        return Valor::Error("arrays::slice(): índices inválidos".to_string());
                    }
                } else {
                    return Valor::Error("arrays::slice() requiere (array, inicio, fin)".to_string());
                }
            }

            if (name == "__array_reverse" || name == "arrays::reverse") && args.len() == 1 {
                if let Valor::Array(mut arr) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    arr.reverse();
                    return Valor::Array(arr);
                } else {
                    return Valor::Error("arrays::reverse() requiere array".to_string());
                }
            }

            // ========== FUNCIONES RANDOM (v0.1.6) ==========
            if (name == "__random_int" || name == "random::int") && args.len() == 2 {
                let min_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                let max_val = evaluar_expr_gfx(&args[1], executor, input, funcs);
                if let (Valor::Num(min), Valor::Num(max)) = (min_val, max_val) {
                    let seed = executor.leer("__random_seed").unwrap_or(Valor::Num(12345.0));
                    let mut s = if let Valor::Num(n) = seed { n as u32 } else { 12345 };
                    s ^= s << 13;
                    s ^= s >> 17;
                    s ^= s << 5;
                    executor.guardar("__random_seed", Valor::Num(s as f64));
                    let range = (max - min).abs() + 1.0;
                    let random_val = (s as f64 / u32::MAX as f64) * range;
                    return Valor::Num(min + random_val);
                } else {
                    return Valor::Error("random::int() requiere (min, max) números".to_string());
                }
            }

            if (name == "__random_float" || name == "random::float") && args.len() == 0 {
                let seed = executor.leer("__random_seed").unwrap_or(Valor::Num(12345.0));
                let mut s = if let Valor::Num(n) = seed { n as u32 } else { 12345 };
                s ^= s << 13;
                s ^= s >> 17;
                s ^= s << 5;
                executor.guardar("__random_seed", Valor::Num(s as f64));
                return Valor::Num(s as f64 / u32::MAX as f64);
            }

            if (name == "__random_choice" || name == "random::choice") && args.len() == 1 {
                let arr_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                if let Valor::Array(arr) = arr_val {
                    if arr.is_empty() {
                        return Valor::Error("random::choice(): array vacío".to_string());
                    }
                    let seed = executor.leer("__random_seed").unwrap_or(Valor::Num(12345.0));
                    let mut s = if let Valor::Num(n) = seed { n as u32 } else { 12345 };
                    s ^= s << 13;
                    s ^= s >> 17;
                    s ^= s << 5;
                    executor.guardar("__random_seed", Valor::Num(s as f64));
                    let idx = (s as usize) % arr.len();
                    return arr[idx].clone();
                } else {
                    return Valor::Error("random::choice() requiere array".to_string());
                }
            }

            // ========== FUNCIONES JSON (v0.1.5) ==========
            if (name == "__json_parse" || name == "json::parse") && args.len() == 1 {
                if let Valor::Texto(json_str) = evaluar_expr_gfx(&args[0], executor, input, funcs) {
                    match serde_json::from_str::<serde_json::Value>(&json_str) {
                        Ok(val) => {
                            return valor_serde_a_rydit(&val);
                        }
                        Err(e) => return Valor::Error(format!("json::parse(): {}", e)),
                    }
                } else {
                    return Valor::Error("json::parse() requiere string JSON".to_string());
                }
            }

            if (name == "__json_stringify" || name == "json::stringify") && args.len() == 1 {
                let val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                match valor_rydit_a_serde(&val) {
                    Ok(serde_val) => {
                        match serde_json::to_string(&serde_val) {
                            Ok(json_str) => return Valor::Texto(json_str),
                            Err(e) => return Valor::Error(format!("json::stringify(): {}", e)),
                        }
                    }
                    Err(e) => return Valor::Error(format!("json::stringify(): {}", e)),
                }
            }

            // ========== FUNCIONES TIME (v0.1.6) ==========
            if (name == "__time_now" || name == "time::now") && args.len() == 0 {
                use std::time::{SystemTime, UNIX_EPOCH};
                match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(duration) => return Valor::Num(duration.as_secs_f64()),
                    Err(e) => return Valor::Error(format!("time::now(): {}", e)),
                }
            }

            if (name == "__time_sleep" || name == "time::sleep") && args.len() == 1 {
                use std::{thread, time::Duration};
                let ms_val = evaluar_expr_gfx(&args[0], executor, input, funcs);
                if let Valor::Num(ms) = ms_val {
                    thread::sleep(Duration::from_millis(ms as u64));
                    return Valor::Vacio;
                } else {
                    return Valor::Error("time::sleep() requiere milisegundos (número)".to_string());
                }
            }

            // ========== FUNCIONES REGEX (v0.6.2) ==========
            if (name == "__regex_match" || name == "regex::match") && args.len() == 2 {
                if let (Valor::Texto(pattern), Valor::Texto(text)) = (&evaluar_expr_gfx(&args[0], executor, input, funcs), &evaluar_expr_gfx(&args[1], executor, input, funcs)) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => return Valor::Bool(re.is_match(text)),
                        Err(e) => return Valor::Error(format!("regex::match(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::match() requiere (patrón, texto)".to_string());
                }
            }

            if (name == "__regex_replace" || name == "regex::replace") && args.len() == 3 {
                if let (Valor::Texto(pattern), Valor::Texto(replacement), Valor::Texto(text)) = (
                    &evaluar_expr_gfx(&args[0], executor, input, funcs),
                    &evaluar_expr_gfx(&args[1], executor, input, funcs),
                    &evaluar_expr_gfx(&args[2], executor, input, funcs)
                ) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => return Valor::Texto(re.replace_all(text, replacement.as_str()).to_string()),
                        Err(e) => return Valor::Error(format!("regex::replace(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::replace() requiere (patrón, reemplazo, texto)".to_string());
                }
            }

            if (name == "__regex_split" || name == "regex::split") && args.len() == 2 {
                if let (Valor::Texto(pattern), Valor::Texto(text)) = (&evaluar_expr_gfx(&args[0], executor, input, funcs), &evaluar_expr_gfx(&args[1], executor, input, funcs)) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => {
                            let parts: Vec<Valor> = re.split(text).map(|s| Valor::Texto(s.to_string())).collect();
                            return Valor::Array(parts);
                        }
                        Err(e) => return Valor::Error(format!("regex::split(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::split() requiere (patrón, texto)".to_string());
                }
            }

            if (name == "__regex_find_all" || name == "regex::find_all") && args.len() == 2 {
                if let (Valor::Texto(pattern), Valor::Texto(text)) = (&evaluar_expr_gfx(&args[0], executor, input, funcs), &evaluar_expr_gfx(&args[1], executor, input, funcs)) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => {
                            let matches: Vec<Valor> = re.find_iter(text).map(|m| Valor::Texto(m.as_str().to_string())).collect();
                            return Valor::Array(matches);
                        }
                        Err(e) => return Valor::Error(format!("regex::find_all(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::find_all() requiere (patrón, texto)".to_string());
                }
            }

            if (name == "__regex_capture" || name == "regex::capture") && args.len() == 2 {
                if let (Valor::Texto(pattern), Valor::Texto(text)) = (&evaluar_expr_gfx(&args[0], executor, input, funcs), &evaluar_expr_gfx(&args[1], executor, input, funcs)) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => {
                            if let Some(caps) = re.captures(text) {
                                let mut result: Vec<Valor> = Vec::new();
                                result.push(Valor::Texto(caps.get(0).unwrap().as_str().to_string()));
                                for i in 1..caps.len() {
                                    if let Some(m) = caps.get(i) {
                                        result.push(Valor::Texto(m.as_str().to_string()));
                                    } else {
                                        result.push(Valor::Vacio);
                                    }
                                }
                                return Valor::Array(result);
                            } else {
                                return Valor::Array(vec![]);
                            }
                        }
                        Err(e) => return Valor::Error(format!("regex::capture(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::capture() requiere (patrón, texto)".to_string());
                }
            }

            Valor::Error(format!("Función '{}' no soportada", name))
        }
        Expr::BinOp { left, op, right } => {
            let left_val = evaluar_expr_gfx(left, executor, input, funcs);
            let right_val = evaluar_expr_gfx(right, executor, input, funcs);

            match op {
                lizer::BinOp::And => {
                    let l_bool = valor_a_bool(&left_val);
                    let r_bool = valor_a_bool(&right_val);
                    return Valor::Bool(l_bool && r_bool);
                }
                lizer::BinOp::Or => {
                    let l_bool = valor_a_bool(&left_val);
                    let r_bool = valor_a_bool(&right_val);
                    return Valor::Bool(l_bool || r_bool);
                }
                _ => {}
            }

            // Concatenación de strings con + (con coerción automática de números)
            if matches!(op, lizer::BinOp::Suma) {
                match (&left_val, &right_val) {
                    (Valor::Texto(l), Valor::Texto(r)) => {
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Texto(l), Valor::Num(r)) => {
                        // "texto" + numero -> "texto123"
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Num(l), Valor::Texto(r)) => {
                        // numero + "texto" -> "123texto"
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Num(_), Valor::Num(_)) => {
                        // numero + numero -> suma aritmética (comportamiento normal, se maneja abajo)
                    }
                    _ => {}
                }
            }

            if let (Valor::Num(l), Valor::Num(r)) = (left_val, right_val) {
                return match op {
                    lizer::BinOp::Suma => Valor::Num(l + r),
                    lizer::BinOp::Resta => Valor::Num(l - r),
                    lizer::BinOp::Mult => Valor::Num(l * r),
                    lizer::BinOp::Div => {
                        if r != 0.0 { Valor::Num(l / r) } else { Valor::Error("División por cero".to_string()) }
                    }
                    lizer::BinOp::Mayor => Valor::Bool(l > r),
                    lizer::BinOp::Menor => Valor::Bool(l < r),
                    lizer::BinOp::Igual => Valor::Bool((l - r).abs() < 0.0001),
                    lizer::BinOp::MayorIgual => Valor::Bool(l >= r),
                    lizer::BinOp::MenorIgual => Valor::Bool(l <= r),
                    _ => Valor::Error("Operador no soportado".to_string())
                };
            }

            Valor::Error("Operación inválida".to_string())
        }
        Expr::Unary { op, expr } => {
            let val = evaluar_expr_gfx(expr, executor, input, funcs);
            match op {
                lizer::UnaryOp::Not => {
                    let b = valor_a_bool(&val);
                    Valor::Bool(!b)
                }
                lizer::UnaryOp::Neg => {
                    if let Valor::Num(n) = val {
                        Valor::Num(-n)
                    } else {
                        Valor::Error("Neg requiere número".to_string())
                    }
                }
            }
        }
    }
}

fn repl_mode() {
    println!("=== RYDIT REPL v0.0.3 ===");
    println!("Escribe comandos RyDit y presiona Enter");
    println!("Comandos: 'help', 'mem', 'clear', 'exit'");
    println!();

    let mut executor = Executor::nuevo();
    let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
    let stdin = io::stdin();  // No necesita ser mutable

    loop {
        print!("rydit> ");
        // Manejar error de flush en REPL
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
                        ejecutar_programa(&program, &mut executor, &mut funcs);
                    }
                    Err(e) => {
                        println!("[ERROR] {}", e);
                    }
                }
            }
        }
    }
}

// ============================================================================
// EVALUAR EXPRESION MODO MIGUI
// ============================================================================

fn evaluar_expr_migui(
    expr: &Expr,
    executor: &mut Executor,
    gui: &mut Migui,
    checkbox_states: &mut HashMap<String, bool>,
    slider_states: &mut HashMap<String, f32>,
    textbox_states: &mut HashMap<String, String>,
    window_states: &mut HashMap<String, bool>,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>
) -> Valor {
    match expr {
        Expr::Num(n) => Valor::Num(*n),
        Expr::Texto(s) => Valor::Texto(s.clone()),
        Expr::Var(name) => {
            if name == "__INPUT__" {
                return executor.input("> ");
            }
            executor.leer(name).unwrap_or(Valor::Vacio)
        }
        Expr::Bool(b) => Valor::Bool(*b),
        Expr::Array(elements) => {
            let valores: Vec<Valor> = elements.iter()
                .map(|e| evaluar_expr_migui(e, executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs))
                .collect();
            Valor::Array(valores)
        }
        Expr::Index { array, index } => {
            let array_val = evaluar_expr_migui(array, executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs);
            let index_val = evaluar_expr_migui(index, executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs);

            if let Valor::Array(arr) = array_val {
                if let Valor::Num(i) = index_val {
                    let idx = i as usize;
                    if idx < arr.len() {
                        return arr[idx].clone();
                    } else {
                        return Valor::Error(format!("Índice {} fuera de rango (len={})", idx, arr.len()));
                    }
                } else {
                    return Valor::Error("El índice debe ser un número".to_string());
                }
            } else {
                return Valor::Error("Solo se puede indexar arrays".to_string());
            }
        }
        Expr::Call { name, args } => {
            println!("[MIGUI DEBUG] Llamada a funcion: {}", name);
            // ========================================================================
            // FUNCIONES MIGUI - V0.4.0 (Immediate Mode GUI)
            // ========================================================================

            // migui::button(id, text, x, y, w, h) -> bool
            if (name == "migui::button" || name == "__migui_button") && args.len() == 6 {
                if let (
                    Valor::Texto(id),
                    Valor::Texto(text),
                    Valor::Num(x),
                    Valor::Num(y),
                    Valor::Num(w),
                    Valor::Num(h)
                ) = (
                    evaluar_expr_migui(&args[0], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[1], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[2], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[3], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[4], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[5], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                ) {
                    let clicked = gui.button(WidgetId::new(&id), Rect::new(x as f32, y as f32, w as f32, h as f32), &text);
                    return Valor::Bool(clicked);
                } else {
                    return Valor::Error("migui::button() requiere (id, text, x, y, w, h)".to_string());
                }
            }

            // migui::label(id, text, x, y, w, h)
            if (name == "migui::label" || name == "__migui_label") && args.len() == 6 {
                if let (
                    Valor::Texto(id),
                    Valor::Texto(text),
                    Valor::Num(x),
                    Valor::Num(y),
                    Valor::Num(w),
                    Valor::Num(h)
                ) = (
                    evaluar_expr_migui(&args[0], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[1], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[2], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[3], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[4], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[5], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                ) {
                    gui.label(WidgetId::new(&id), &text, Rect::new(x as f32, y as f32, w as f32, h as f32));
                    return Valor::Vacio;
                } else {
                    return Valor::Error("migui::label() requiere (id, text, x, y, w, h)".to_string());
                }
            }

            // migui::checkbox(id, text, checked, x, y, w, h) -> bool
            if (name == "migui::checkbox" || name == "__migui_checkbox") && args.len() == 7 {
                if let (
                    Valor::Texto(id),
                    Valor::Texto(text),
                    Valor::Bool(checked),
                    Valor::Num(x),
                    Valor::Num(y),
                    Valor::Num(w),
                    Valor::Num(h)
                ) = (
                    evaluar_expr_migui(&args[0], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[1], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[2], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[3], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[4], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[5], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[6], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                ) {
                    let state = checkbox_states.entry(id.clone()).or_insert(checked);
                    let changed = gui.checkbox(WidgetId::new(&id), &text, state, Rect::new(x as f32, y as f32, w as f32, h as f32));
                    return Valor::Bool(changed);
                } else {
                    return Valor::Error("migui::checkbox() requiere (id, text, checked, x, y, w, h)".to_string());
                }
            }

            // migui::slider(id, value, min, max, x, y, w, h) -> f32
            if (name == "migui::slider" || name == "__migui_slider") && args.len() == 8 {
                if let (
                    Valor::Texto(id),
                    Valor::Num(value),
                    Valor::Num(min),
                    Valor::Num(max),
                    Valor::Num(x),
                    Valor::Num(y),
                    Valor::Num(w),
                    Valor::Num(h)
                ) = (
                    evaluar_expr_migui(&args[0], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[1], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[2], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[3], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[4], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[5], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[6], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[7], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                ) {
                    let state = slider_states.entry(id.clone()).or_insert(value as f32);
                    *state = gui.slider(WidgetId::new(&id), *state, min as f32, max as f32, Rect::new(x as f32, y as f32, w as f32, h as f32));
                    return Valor::Num(*state as f64);
                } else {
                    return Valor::Error("migui::slider() requiere (id, value, min, max, x, y, w, h)".to_string());
                }
            }

            // migui::panel(id, x, y, w, h, color)
            if (name == "migui::panel" || name == "__migui_panel") && args.len() == 6 {
                if let (
                    Valor::Texto(id),
                    Valor::Num(x),
                    Valor::Num(y),
                    Valor::Num(w),
                    Valor::Num(h),
                    Valor::Texto(color_str)
                ) = (
                    evaluar_expr_migui(&args[0], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[1], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[2], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[3], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[4], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[5], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                ) {
                    let color = MiguiColor::from_str(&color_str);
                    gui.panel(WidgetId::new(&id), Rect::new(x as f32, y as f32, w as f32, h as f32), color);
                    return Valor::Vacio;
                } else {
                    return Valor::Error("migui::panel() requiere (id, x, y, w, h, color)".to_string());
                }
            }

            // migui::textbox(id, x, y, w, h) -> String
            if (name == "migui::textbox" || name == "__migui_textbox") && args.len() == 5 {
                if let (
                    Valor::Texto(id),
                    Valor::Num(x),
                    Valor::Num(y),
                    Valor::Num(w),
                    Valor::Num(h)
                ) = (
                    evaluar_expr_migui(&args[0], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[1], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[2], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[3], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[4], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                ) {
                    gui.set_textbox_text(&id, String::new());
                    gui.textbox(WidgetId::new(&id), Rect::new(x as f32, y as f32, w as f32, h as f32));
                    return Valor::Texto(gui.textbox_states.get(&id).map(|s| s.text.clone()).unwrap_or_default());
                } else {
                    return Valor::Error("migui::textbox() requiere (id, x, y, w, h)".to_string());
                }
            }

            // migui::window(id, title, open, x, y, w, h) -> bool
            if (name == "migui::window" || name == "__migui_window") && args.len() == 7 {
                if let (
                    Valor::Texto(id),
                    Valor::Texto(title),
                    Valor::Bool(open),
                    Valor::Num(x),
                    Valor::Num(y),
                    Valor::Num(w),
                    Valor::Num(h)
                ) = (
                    evaluar_expr_migui(&args[0], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[1], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[2], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[3], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[4], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[5], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[6], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                ) {
                    let state = window_states.entry(id.clone()).or_insert(open);
                    let is_open = gui.window(WidgetId::new(&id), &title, Rect::new(x as f32, y as f32, w as f32, h as f32), state);
                    return Valor::Bool(is_open);
                } else {
                    return Valor::Error("migui::window() requiere (id, title, open, x, y, w, h)".to_string());
                }
            }

            // migui::message_box(title, message, buttons, x, y, w, h) -> i32
            if (name == "migui::message_box" || name == "__migui_message_box") && args.len() == 7 {
                if let (
                    Valor::Texto(title),
                    Valor::Texto(message),
                    Valor::Array(buttons_arr),
                    Valor::Num(x),
                    Valor::Num(y),
                    Valor::Num(w),
                    Valor::Num(h)
                ) = (
                    evaluar_expr_migui(&args[0], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[1], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[2], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[3], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[4], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[5], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                    evaluar_expr_migui(&args[6], executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs),
                ) {
                    let buttons: Vec<&str> = buttons_arr.iter()
                        .filter_map(|v| if let Valor::Texto(s) = v { Some(s.as_str()) } else { None })
                        .collect();
                    let result = gui.message_box(&title, &message, &buttons, Rect::new(x as f32, y as f32, w as f32, h as f32));
                    return Valor::Num(result as f64);
                } else {
                    return Valor::Error("migui::message_box() requiere (title, message, buttons, x, y, w, h)".to_string());
                }
            }

            // migui::mouse_x() -> f64
            if name == "migui::mouse_x" || name == "__migui_mouse_x" {
                return Valor::Num(gui.mouse_x() as f64);
            }

            // migui::mouse_y() -> f64
            if name == "migui::mouse_y" || name == "__migui_mouse_y" {
                return Valor::Num(gui.mouse_y() as f64);
            }

            // migui::mouse_position() -> [x, y]
            if name == "migui::mouse_position" || name == "__migui_mouse_position" {
                let (x, y) = gui.mouse_position();
                return Valor::Array(vec![Valor::Num(x as f64), Valor::Num(y as f64)]);
            }

            // migui::is_mouse_button_pressed() -> bool
            if name == "migui::is_mouse_button_pressed" || name == "__migui_is_mouse_button_pressed" {
                return Valor::Bool(gui.is_mouse_pressed());
            }

            // Funciones definidas por el usuario
            let func_name = if name.contains("::") {
                if funcs.contains_key(name) {
                    name.clone()
                } else {
                    name.split("::").last().unwrap_or(&name).to_string()
                }
            } else {
                name.clone()
            };

            let func_data = funcs.get(&func_name).map(|(p, b)| (p.clone(), b.clone()));

            if let Some((params, body)) = func_data {
                let mut arg_values = vec![];
                for arg in args {
                    arg_values.push(evaluar_expr_migui(arg, executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs));
                }

                executor.push_scope();
                for (i, param) in params.iter().enumerate() {
                    if i < arg_values.len() {
                        executor.guardar_local(param, arg_values[i].clone());
                    }
                }

                let mut empty_loaded: HashSet<String> = HashSet::new();
                let mut empty_stack: Vec<String> = Vec::new();

                let mut return_value: Option<Valor> = None;
                for s in &body {
                    match ejecutar_stmt_migui(s, executor, funcs, gui, &mut empty_loaded, &mut empty_stack, checkbox_states, slider_states, textbox_states, window_states) {
                        (Some(true), _) => {
                            executor.pop_scope();
                            return Valor::Error("Break no permitido en función de expresión".to_string());
                        }
                        (_, Some(val)) => {
                            return_value = Some(val);
                            break;
                        }
                        _ => {}
                    }
                }

                executor.pop_scope();
                return return_value.unwrap_or(Valor::Vacio);
            }

            Valor::Error(format!("Función '{}' no soportada en expresiones", name))
        }
        Expr::BinOp { left, op, right } => {
            let left_val = evaluar_expr_migui(left, executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs);
            let right_val = evaluar_expr_migui(right, executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs);

            match op {
                lizer::BinOp::And => {
                    let l_bool = valor_a_bool(&left_val);
                    let r_bool = valor_a_bool(&right_val);
                    return Valor::Bool(l_bool && r_bool);
                }
                lizer::BinOp::Or => {
                    let l_bool = valor_a_bool(&left_val);
                    let r_bool = valor_a_bool(&right_val);
                    return Valor::Bool(l_bool || r_bool);
                }
                _ => {}
            }

            if matches!(op, lizer::BinOp::Suma) {
                match (&left_val, &right_val) {
                    (Valor::Texto(l), Valor::Texto(r)) => {
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Texto(l), Valor::Num(r)) => {
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Num(l), Valor::Texto(r)) => {
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Num(l), Valor::Num(r)) => {
                        return Valor::Num(l + r);
                    }
                    _ => {}
                }
            }

            if let (Valor::Num(l), Valor::Num(r)) = (&left_val, &right_val) {
                match op {
                    lizer::BinOp::Suma => return Valor::Num(l + r),
                    lizer::BinOp::Resta => return Valor::Num(l - r),
                    lizer::BinOp::Mult => return Valor::Num(l * r),
                    lizer::BinOp::Div => {
                        if *r != 0.0 {
                            return Valor::Num(l / r);
                        } else {
                            return Valor::Error("División por cero".to_string());
                        }
                    }
                    // Mod y Exp no existen en lizer::BinOp
                    lizer::BinOp::Igual => return Valor::Bool((l - r).abs() < f64::EPSILON),
                    lizer::BinOp::Menor => return Valor::Bool(l < r),
                    lizer::BinOp::MenorIgual => return Valor::Bool(l <= r),
                    lizer::BinOp::Mayor => return Valor::Bool(l > r),
                    lizer::BinOp::MayorIgual => return Valor::Bool(l >= r),
                    _ => {}
                }
            }

            match (&left_val, &right_val) {
                (Valor::Texto(l), Valor::Texto(r)) => {
                    match op {
                        lizer::BinOp::Igual => return Valor::Bool(l == r),
                        // lizer::BinOp::Ne (no existe) => return Valor::Bool(l != r),
                        _ => {}
                    }
                }
                (Valor::Bool(l), Valor::Bool(r)) => {
                    match op {
                        lizer::BinOp::Igual => return Valor::Bool(l == r),
                        // lizer::BinOp::Ne (no existe) => return Valor::Bool(l != r),
                        _ => {}
                    }
                }
                _ => {}
            }

            Valor::Error(format!("Operación no soportada: {:?} entre {:?} y {:?}", op, left_val, right_val))
        }
        Expr::Unary { op, expr: inner } => {
            let val = evaluar_expr_migui(inner, executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs);
            match op {
                lizer::UnaryOp::Neg => {
                    if let Valor::Num(n) = val {
                        return Valor::Num(-n);
                    } else {
                        return Valor::Error("Unary - requires number".to_string());
                    }
                }
                lizer::UnaryOp::Not => {
                    let b = valor_a_bool(&val);
                    return Valor::Bool(!b);
                }
            }
        }
    }
}

// ============================================================================
// EJECUTAR STATEMENT MODO MIGUI
// ============================================================================

fn ejecutar_stmt_migui(
    stmt: &Stmt,
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
    gui: &mut Migui,
    loaded_modules: &mut HashSet<String>,
    importing_stack: &mut Vec<String>,
    checkbox_states: &mut HashMap<String, bool>,
    slider_states: &mut HashMap<String, f32>,
    textbox_states: &mut HashMap<String, String>,
    window_states: &mut HashMap<String, bool>,
) -> (Option<bool>, Option<Valor>) {
    match stmt {
        Stmt::Init => {}
        Stmt::Command(cmd) => {
            executor.ejecutar(cmd);
        }
        Stmt::Assign { name, value } => {
            let valor = evaluar_expr_migui(value, executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs);
            executor.guardar(name, valor);
        }
        Stmt::IndexAssign { array, index, value } => {
            let index_val = evaluar_expr_migui(index, executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs);
            let valor = evaluar_expr_migui(value, executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs);

            if let Some(Valor::Array(arr)) = executor.leer(array) {
                let idx = match index_val {
                    Valor::Num(n) => n as usize,
                    _ => {
                        println!("[ERROR] Índice debe ser número");
                        return (None, None);
                    }
                };

                if idx >= arr.len() {
                    println!("[ERROR] Índice {} fuera de límites (array de {} elementos)", idx, arr.len());
                    return (None, None);
                }

                let mut nuevo_arr = arr.clone();
                nuevo_arr[idx] = valor;
                executor.guardar(array, Valor::Array(nuevo_arr));
            } else {
                println!("[ERROR] '{}' no es un array", array);
            }
        }
        Stmt::If { condition, then_body, else_body } => {
            let cond_val = evaluar_expr_migui(condition, executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs);
            let es_verdad = match cond_val {
                Valor::Num(n) => n != 0.0,
                Valor::Bool(b) => b,
                _ => false,
            };

            if es_verdad {
                for s in then_body {
                    ejecutar_stmt_migui(s, executor, funcs, gui, loaded_modules, importing_stack, checkbox_states, slider_states, textbox_states, window_states);
                }
            } else if let Some(else_stmts) = else_body {
                for s in else_stmts {
                    ejecutar_stmt_migui(s, executor, funcs, gui, loaded_modules, importing_stack, checkbox_states, slider_states, textbox_states, window_states);
                }
            }
        }
        Stmt::While { condition, body } => {
            let mut iterations = 0;
            while iterations < 10 {
                let cond_val = evaluar_expr_migui(condition, executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs);
                let es_verdad = match cond_val {
                    Valor::Num(n) => n != 0.0,
                    Valor::Bool(b) => b,
                    _ => false,
                };

                if !es_verdad {
                    break;
                }

                for s in body {
                    ejecutar_stmt_migui(s, executor, funcs, gui, loaded_modules, importing_stack, checkbox_states, slider_states, textbox_states, window_states);
                }
                iterations += 1;
            }
        }
        Stmt::ForEach { var, iterable, body } => {
            let iterable_val = evaluar_expr_migui(iterable, executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs);
            if let Valor::Array(arr) = iterable_val {
                for item in arr {
                    executor.guardar(var, item.clone());
                    for s in body {
                        ejecutar_stmt_migui(s, executor, funcs, gui, loaded_modules, importing_stack, checkbox_states, slider_states, textbox_states, window_states);
                    }
                }
            } else {
                println!("[ERROR] 'cada' requiere un array");
            }
        }
        Stmt::Block(stmts) => {
            println!("[MIGUI DEBUG] Ejecutando bloque con {} statements", stmts.len());
            for (i, s) in stmts.iter().enumerate() {
                println!("[MIGUI DEBUG] Statement {}: {:?}", i, match s {
                    Stmt::Expr(_) => "Expr",
                    Stmt::Assign { .. } => "Assign",
                    Stmt::Call { .. } => "Call",
                    Stmt::Function { .. } => "Function",
                    _ => "Other",
                });
                ejecutar_stmt_migui(s, executor, funcs, gui, loaded_modules, importing_stack, checkbox_states, slider_states, textbox_states, window_states);
            }
        }
        Stmt::Function { name, params, body } => {
            funcs.insert(name.clone(), (params.clone(), body.clone()));
        }
        Stmt::Call { name, args } => {
            // Para migui, evaluar como expresión (las funciones migui generan draw commands)
            let _ = evaluar_expr_migui(&Expr::Call { name: name.clone(), args: args.clone() }, executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs);
            return (None, None);
        }
        Stmt::Import { module, alias } => {
            let module_path = format!("crates/modules/{}.rydit", module);

            if importing_stack.contains(&module) {
                println!("[ERROR] Importe cíclico detectado: '{}'", module);
                return (None, None);
            }

            if loaded_modules.contains(module.as_str()) {
                let prefix = if let Some(alias_name) = alias {
                    alias_name.clone()
                } else {
                    module.clone()
                };

                let mut funcs_to_copy: Vec<(String, String)> = Vec::new();
                for (func_name, _) in funcs.iter() {
                    if func_name.starts_with(&format!("{}::", module)) {
                        let orig_name = func_name.strip_prefix(&format!("{}::", module)).unwrap();
                        let new_name = format!("{}::{}", prefix, orig_name);
                        funcs_to_copy.push((func_name.clone(), new_name));
                    }
                }

                for (old_name, new_name) in funcs_to_copy {
                    if let Some(func_data) = funcs.get(&old_name) {
                        funcs.insert(new_name, func_data.clone());
                    }
                }
                return (None, None);
            }

            if let Ok(content) = std::fs::read_to_string(&module_path) {
                importing_stack.push(module.clone());

                let tokens = Lizer::new(&content).scan();
                let mut parser = Parser::new(tokens);

                let program = match parser.parse() {
                    Ok(p) => p,
                    Err(e) => {
                        println!("[ERROR] Error parseando módulo '{}': {}", module, e);
                        importing_stack.pop();
                        return (None, None);
                    }
                };

                let mut original_funcs: Vec<String> = Vec::new();
                for s in &program.statements {
                    if let Stmt::Function { name, .. } = s {
                        original_funcs.push(name.clone());
                    }
                }

                for s in &program.statements {
                    match ejecutar_stmt(s, executor, funcs, loaded_modules, importing_stack) {
                        (Some(true), _) => {
                            println!("[ERROR] break no permitido en módulo '{}'", module);
                            break;
                        },
                        (_, Some(_)) => {
                            println!("[ERROR] return no permitido en módulo '{}'", module);
                            break;
                        },
                        _ => {}
                    }
                }

                importing_stack.pop();
                loaded_modules.insert(module.clone());

                let prefix = if let Some(alias_name) = alias {
                    alias_name.clone()
                } else {
                    module.clone()
                };

                for orig_name in &original_funcs {
                    if let Some(func_data) = funcs.get(orig_name) {
                        let new_name = format!("{}::{}", prefix, orig_name);
                        funcs.insert(new_name, func_data.clone());
                    }
                }

                if alias.is_none() {
                    for orig_name in &original_funcs {
                        funcs.remove(orig_name);
                    }
                }
            } else {
                println!("[ERROR] Módulo '{}' no encontrado", module);
            }
        }
        Stmt::Return(opt_expr) => {
            if let Some(val) = opt_expr {
                let valor = evaluar_expr_migui(val, executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs);
                println!("[RETURN] {}", match valor {
                    Valor::Num(n) => format!("{}", n),
                    Valor::Texto(s) => s,
                    Valor::Bool(b) => format!("{}", b),
                    _ => format!("{:?}", valor),
                });
            }
        }
        Stmt::Expr(expr) => {
            let valor = evaluar_expr_migui(expr, executor, gui, checkbox_states, slider_states, textbox_states, window_states, funcs);
            executor.voz(&valor);
        }
        _ => {}
    }
    (None, None)
}

// ==================== TESTS DE WARNINGS ====================

#[cfg(test)]
mod warning_tests {
    use super::*;

    #[test]
    fn test_division_por_cero() {
        // Verificar que división por cero retorna Error, no panic
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let left = Expr::Num(10.0);
        let right = Expr::Num(0.0);
        let expr = Expr::BinOp {
            left: Box::new(left),
            op: lizer::BinOp::Div,
            right: Box::new(right),
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert!(matches!(result, Valor::Error(_)));
    }

    #[test]
    fn test_division_normal() {
        // Verificar que división normal funciona
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let left = Expr::Num(10.0);
        let right = Expr::Num(2.0);
        let expr = Expr::BinOp {
            left: Box::new(left),
            op: lizer::BinOp::Div,
            right: Box::new(right),
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert!(matches!(result, Valor::Num(5.0)));
    }

    // ========================================================================
    // TESTS V0.1.9 - CONCATENACIÓN Y SÍMBOLOS
    // ========================================================================

    #[test]
    fn test_concatenacion_string_numero() {
        // "x=" + 42 -> "x=42"
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let left = Expr::Texto("x=".to_string());
        let right = Expr::Num(42.0);
        let expr = Expr::BinOp {
            left: Box::new(left),
            op: lizer::BinOp::Suma,
            right: Box::new(right),
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Texto("x=42".to_string()));
    }

    #[test]
    fn test_concatenacion_numero_string() {
        // 42 + "x" -> "42x"
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let left = Expr::Num(42.0);
        let right = Expr::Texto("x".to_string());
        let expr = Expr::BinOp {
            left: Box::new(left),
            op: lizer::BinOp::Suma,
            right: Box::new(right),
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Texto("42x".to_string()));
    }

    #[test]
    fn test_concatenacion_multiple() {
        // "a"+1+"b"+2 -> "a1b2"
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        
        // "a" + 1 -> "a1"
        let expr1 = Expr::BinOp {
            left: Box::new(Expr::Texto("a".to_string())),
            op: lizer::BinOp::Suma,
            right: Box::new(Expr::Num(1.0)),
        };
        let result1 = evaluar_expr(&expr1, &mut executor, &mut funcs);
        assert_eq!(result1, Valor::Texto("a1".to_string()));
        
        // "a1" + "b" -> "a1b"
        let expr2 = Expr::BinOp {
            left: Box::new(Expr::Texto("a1".to_string())),
            op: lizer::BinOp::Suma,
            right: Box::new(Expr::Texto("b".to_string())),
        };
        let result2 = evaluar_expr(&expr2, &mut executor, &mut funcs);
        assert_eq!(result2, Valor::Texto("a1b".to_string()));
        
        // "a1b" + 2 -> "a1b2"
        let expr3 = Expr::BinOp {
            left: Box::new(Expr::Texto("a1b".to_string())),
            op: lizer::BinOp::Suma,
            right: Box::new(Expr::Num(2.0)),
        };
        let result3 = evaluar_expr(&expr3, &mut executor, &mut funcs);
        assert_eq!(result3, Valor::Texto("a1b2".to_string()));
    }

    #[test]
    fn test_concatenacion_con_expresion() {
        // "total: " + (2+3)*4 -> "total: 20"
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        
        // (2+3)*4 = 20
        let inner = Expr::BinOp {
            left: Box::new(Expr::Num(2.0)),
            op: lizer::BinOp::Suma,
            right: Box::new(Expr::Num(3.0)),
        };
        let expr_mult = Expr::BinOp {
            left: Box::new(inner),
            op: lizer::BinOp::Mult,
            right: Box::new(Expr::Num(4.0)),
        };
        
        // "total: " + 20
        let expr = Expr::BinOp {
            left: Box::new(Expr::Texto("total: ".to_string())),
            op: lizer::BinOp::Suma,
            right: Box::new(expr_mult),
        };
        
        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Texto("total: 20".to_string()));
    }

    #[test]
    fn test_variable_dolar_asignacion() {
        // $x = 10 debe guardarse correctamente
        let mut executor = Executor::nuevo();
        executor.guardar("$x", Valor::Num(10.0));
        let result = executor.leer("$x");
        assert_eq!(result, Some(Valor::Num(10.0)));
    }

    #[test]
    fn test_variable_arroba_lectura() {
        // @user debe leerse correctamente
        let mut executor = Executor::nuevo();
        executor.guardar("@user", Valor::Texto("alucard18".to_string()));
        let result = executor.leer("@user");
        assert_eq!(result, Some(Valor::Texto("alucard18".to_string())));
    }

    #[test]
    fn test_variable_porcentaje_expresion() {
        // %p = 50 + 25 -> 75
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        
        let expr = Expr::BinOp {
            left: Box::new(Expr::Num(50.0)),
            op: lizer::BinOp::Suma,
            right: Box::new(Expr::Num(25.0)),
        };
        
        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        executor.guardar("%p", result);
        
        assert_eq!(executor.leer("%p"), Some(Valor::Num(75.0)));
    }

    #[test]
    fn test_simbolos_en_array() {
        // [$a, $b] debe evaluarse como array
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        
        executor.guardar("$a", Valor::Num(1.0));
        executor.guardar("$b", Valor::Num(2.0));
        
        let expr = Expr::Array(vec![
            Expr::Var("$a".to_string()),
            Expr::Var("$b".to_string()),
        ]);
        
        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        
        if let Valor::Array(arr) = result {
            assert_eq!(arr.len(), 2);
            assert_eq!(arr[0], Valor::Num(1.0));
            assert_eq!(arr[1], Valor::Num(2.0));
        } else {
            panic!("Expected Array, got {:?}", result);
        }
    }

    #[test]
    fn test_concatenacion_string_string() {
        // "hello" + "world" -> "helloworld"
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let left = Expr::Texto("hello".to_string());
        let right = Expr::Texto("world".to_string());
        let expr = Expr::BinOp {
            left: Box::new(left),
            op: lizer::BinOp::Suma,
            right: Box::new(right),
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Texto("helloworld".to_string()));
    }

    #[test]
    fn test_suma_aritmetica_no_se_afecta() {
        // 2 + 3 debe seguir siendo 5 (no concatenación)
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let left = Expr::Num(2.0);
        let right = Expr::Num(3.0);
        let expr = Expr::BinOp {
            left: Box::new(left),
            op: lizer::BinOp::Suma,
            right: Box::new(right),
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Num(5.0));
    }

    // ========================================================================
    // TESTS V0.6.2 - MÓDULO REGEX
    // ========================================================================

    #[test]
    fn test_regex_match_valido() {
        // regex::match("[a-z]+", "hola") -> true
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let args = vec![
            Expr::Texto("[a-z]+".to_string()),
            Expr::Texto("hola".to_string()),
        ];
        let expr = Expr::Call {
            name: "regex::match".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Bool(true));
    }

    #[test]
    fn test_regex_match_invalido() {
        // regex::match("\\d+", "abc") -> false
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let args = vec![
            Expr::Texto("\\d+".to_string()),
            Expr::Texto("abc".to_string()),
        ];
        let expr = Expr::Call {
            name: "regex::match".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Bool(false));
    }

    #[test]
    fn test_regex_replace() {
        // regex::replace("[aeiou]", "*", "hola") -> "h*l*"
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let args = vec![
            Expr::Texto("[aeiou]".to_string()),
            Expr::Texto("*".to_string()),
            Expr::Texto("hola".to_string()),
        ];
        let expr = Expr::Call {
            name: "regex::replace".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Texto("h*l*".to_string()));
    }

    #[test]
    fn test_regex_split() {
        // regex::split(",", "uno,dos,tres") -> ["uno", "dos", "tres"]
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let args = vec![
            Expr::Texto(",".to_string()),
            Expr::Texto("uno,dos,tres".to_string()),
        ];
        let expr = Expr::Call {
            name: "regex::split".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        if let Valor::Array(arr) = result {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], Valor::Texto("uno".to_string()));
            assert_eq!(arr[1], Valor::Texto("dos".to_string()));
            assert_eq!(arr[2], Valor::Texto("tres".to_string()));
        } else {
            panic!("Expected Array, got {:?}", result);
        }
    }

    #[test]
    fn test_regex_find_all() {
        // regex::find_all("\\d+", "a1b23c456") -> ["1", "23", "456"]
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let args = vec![
            Expr::Texto("\\d+".to_string()),
            Expr::Texto("a1b23c456".to_string()),
        ];
        let expr = Expr::Call {
            name: "regex::find_all".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        if let Valor::Array(arr) = result {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], Valor::Texto("1".to_string()));
            assert_eq!(arr[1], Valor::Texto("23".to_string()));
            assert_eq!(arr[2], Valor::Texto("456".to_string()));
        } else {
            panic!("Expected Array, got {:?}", result);
        }
    }

    #[test]
    fn test_regex_capture() {
        // regex::capture("([a-z]+):(\\d+)", "edad:25") -> ["edad:25", "edad", "25"]
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let args = vec![
            Expr::Texto("([a-z]+):(\\d+)".to_string()),
            Expr::Texto("edad:25".to_string()),
        ];
        let expr = Expr::Call {
            name: "regex::capture".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        if let Valor::Array(arr) = result {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], Valor::Texto("edad:25".to_string()));
            assert_eq!(arr[1], Valor::Texto("edad".to_string()));
            assert_eq!(arr[2], Valor::Texto("25".to_string()));
        } else {
            panic!("Expected Array, got {:?}", result);
        }
    }

    #[test]
    fn test_regex_email_validation() {
        // Validar email real
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let args = vec![
            Expr::Texto("[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}".to_string()),
            Expr::Texto("usuario@ejemplo.com".to_string()),
        ];
        let expr = Expr::Call {
            name: "regex::match".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Bool(true));
    }
}
