// crates/rydit-rs/src/executor.rs
// Ejecución de programas en diferentes modos (comandante, gfx, migui)

use std::collections::{HashMap, HashSet};

use blast_core::Executor;
use lizer::Program;
use migui::Migui;
use rydit_gfx::RyditGfx;

use crate::{
    ejecutar_stmt, ejecutar_stmt_gfx, ejecutar_stmt_migui, evaluar_expr_migui, InputEstado,
};

/// Ejecutar programa en modo comandante (sin gráficos)
pub fn ejecutar_programa(
    program: &Program,
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<lizer::Stmt>)>,
) {
    // Contexto de imports: módulos cargados y stack de imports en progreso
    let mut loaded_modules: HashSet<String> = HashSet::new();
    let mut importing_stack: Vec<String> = Vec::new();

    for stmt in &program.statements {
        let (break_flag, return_val) = ejecutar_stmt(
            stmt,
            executor,
            funcs,
            &mut loaded_modules,
            &mut importing_stack,
        );

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

/// Ejecutar programa en modo gráfico (gfx)
pub fn ejecutar_programa_gfx(
    program: &Program,
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<lizer::Stmt>)>,
    gfx: &mut RyditGfx,
) {
    // Inicializar debug log
    rydit_gfx::debug_log::debug_init(
        "/data/data/com.termux/files/home/shield-project/rydit_debug.log",
    );
    rydit_gfx::debug_log::debug_info("=== INICIANDO GAME LOOP ===");

    // Estado del input
    let mut input = InputEstado::new();

    // Contexto de imports: módulos cargados y stack de imports en progreso
    let mut loaded_modules: HashSet<String> = HashSet::new();
    let mut importing_stack: Vec<String> = Vec::new();

    rydit_gfx::debug_log::debug_log(&format!(
        "Program has {} statements",
        program.statements.len()
    ));

    eprintln!(
        "[EXECUTOR GFX] Program has {} statements",
        program.statements.len()
    );
    for (i, stmt) in program.statements.iter().enumerate() {
        eprintln!("[EXECUTOR GFX] Statement {}: {:?}", i, stmt);
    }

    // Ejecutar statements iniciales (UNA vez) EXCEPTO While/Blocks que son game loops
    for stmt in &program.statements {
        match stmt {
            // While y Blocks son game loops - NO ejecutar aquí
            lizer::Stmt::While { .. } | lizer::Stmt::Block(_) => {
                rydit_gfx::debug_log::debug_info("Statement es game loop (While/Block)");
            }
            // Todo lo demás es inicialización
            _ => {
                rydit_gfx::debug_log::debug_log("Ejecutando statement inicial");
                ejecutar_stmt_gfx(
                    stmt,
                    executor,
                    funcs,
                    &mut gfx.begin_draw(),
                    &mut input,
                    &mut loaded_modules,
                    &mut importing_stack,
                );
            }
        }
    }

    // Buscar el primer While o Block como game loop principal
    let mut found_loop = false;
    for stmt in &program.statements {
        match stmt {
            lizer::Stmt::While { condition, body } => {
                found_loop = true;
                rydit_gfx::debug_log::debug_info("=== ENCONTRADO GAME LOOP WHILE ===");
                rydit_gfx::debug_log::debug_log(&format!("Body tiene {} statements", body.len()));

                // El While ES el game loop principal
                let mut frame_count = 0;
                loop {
                    // Input primero
                    input.actualizar(gfx);
                    let escape = gfx.is_key_pressed(rydit_gfx::Key::Escape);

                    // Verificar condición del While
                    let cond_val =
                        crate::evaluar_expr_gfx_for_loop(condition, executor, &input, funcs);
                    let es_verdad = match cond_val {
                        blast_core::Valor::Num(n) => n != 0.0,
                        blast_core::Valor::Bool(b) => b,
                        _ => false,
                    };

                    if !es_verdad || escape {
                        rydit_gfx::debug_log::debug_log(&format!(
                            "Saliendo del loop - frame={}, escape={}",
                            frame_count, escape
                        ));
                        break;
                    }

                    // Ejecutar body del While
                    {
                        {
                            let mut d = gfx.begin_draw();
                            d.clear(crate::ColorRydit::Negro);

                            eprintln!("[EXECUTOR] Body tiene {} statements", body.len());
                            for (i, s) in body.iter().enumerate() {
                                eprintln!("[EXECUTOR] Statement {}: {:?}", i, s);
                            }

                            for s in body {
                                eprintln!("[EXECUTOR] Ejecutando statement...");
                                let break_signal = ejecutar_stmt_gfx(
                                    s,
                                    executor,
                                    funcs,
                                    &mut d,
                                    &mut input,
                                    &mut loaded_modules,
                                    &mut importing_stack,
                                );
                                if break_signal == Some(true) {
                                    rydit_gfx::debug_log::debug_log("Break detectado en body");
                                    break;
                                }
                            }

                            // FPS counter + debug info
                            d.draw_text("RyDit v0.8.5", 10, 10, 20, crate::ColorRydit::Blanco);
                            d.draw_text(
                                &format!("Frame: {}", frame_count),
                                10,
                                40,
                                20,
                                crate::ColorRydit::Verde,
                            );

                            // Drop explícito del DrawHandle para forzar buffer swap
                            drop(d);
                        }
                        eprintln!(
                            "[EXECUTOR] Frame {} completado - DrawHandle dropped",
                            frame_count
                        );
                    }

                    frame_count += 1;

                    // Log cada 10 frames
                    if frame_count % 10 == 0 {
                        rydit_gfx::debug_log::debug_log_frame(&format!(
                            "Frame {} completado",
                            frame_count
                        ));
                    }

                    if escape {
                        break;
                    }
                }
                rydit_gfx::debug_log::debug_log(&format!(
                    "Game loop terminado - frames totales: {}",
                    frame_count
                ));
                break; // Solo un game loop principal
            }
            lizer::Stmt::Block(stmts) => {
                found_loop = true;
                // Block es game loop - ejecutar en cada frame
                while !gfx.should_close() {
                    input.actualizar(gfx);
                    let escape = gfx.is_key_pressed(rydit_gfx::Key::Escape);

                    {
                        let mut d = gfx.begin_draw();
                        d.clear(crate::ColorRydit::Negro);

                        for s in stmts {
                            let break_signal = ejecutar_stmt_gfx(
                                s,
                                executor,
                                funcs,
                                &mut d,
                                &mut input,
                                &mut loaded_modules,
                                &mut importing_stack,
                            );
                            if break_signal == Some(true) {
                                break;
                            }
                        }

                        d.draw_text("RyDit v0.8.5", 10, 10, 20, crate::ColorRydit::Blanco);
                    }

                    if escape {
                        break;
                    }
                }
                break; // Solo un game loop principal
            }
            _ => {}
        }
    }

    // Si no hay game loop explícito, usar el game loop por defecto
    if !found_loop {
        while !gfx.should_close() {
            input.actualizar(gfx);
            let escape = gfx.is_key_pressed(rydit_gfx::Key::Escape);

            {
                let mut d = gfx.begin_draw();
                d.clear(crate::ColorRydit::Negro);

                // Ejecutar todos los statements que no son While/Block
                for stmt in &program.statements {
                    match stmt {
                        lizer::Stmt::While { .. } | lizer::Stmt::Block(_) => {}
                        _ => {
                            ejecutar_stmt_gfx(
                                stmt,
                                executor,
                                funcs,
                                &mut d,
                                &mut input,
                                &mut loaded_modules,
                                &mut importing_stack,
                            );
                        }
                    }
                }

                d.draw_text("RyDit v0.8.5", 10, 10, 20, crate::ColorRydit::Blanco);
            }

            if escape {
                break;
            }
        }
    }
}

/// Ejecutar programa en modo migui (GUI)
pub fn ejecutar_programa_migui(
    program: &Program,
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<lizer::Stmt>)>,
    gui: &mut Migui,
    gfx: &mut RyditGfx,
) {
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
            lizer::Stmt::Function { name, params, body } => {
                funcs.insert(name.clone(), (params.clone(), body.clone()));
            }
            lizer::Stmt::Assign { name, value } => {
                let valor = evaluar_expr_migui(
                    value,
                    executor,
                    gui,
                    &mut checkbox_states,
                    &mut slider_states,
                    &mut textbox_states,
                    &mut window_states,
                    funcs,
                );
                executor.guardar(name, valor);
            }
            _ => {}
        }
    }

    // Guardar el bloque de código para ejecutar en cada frame
    let frame_stmts: Vec<&lizer::Stmt> = program
        .statements
        .iter()
        .filter(|s| matches!(s, lizer::Stmt::Block(_)))
        .flat_map(|s| {
            if let lizer::Stmt::Block(stmts) = s {
                stmts.iter().collect()
            } else {
                vec![]
            }
        })
        .collect();

    // Game loop principal con migui + backend
    while !gfx.should_close() {
        // Input de teclado para salir
        if gfx.is_key_pressed(GfxKey::Escape) {
            break;
        }

        // Input de mouse para migui
        let (mx, my) = gfx.get_mouse_position();
        gui.handle_event(Event::MouseMove {
            x: mx as f32,
            y: my as f32,
        });

        if gfx.is_mouse_button_pressed(0) {
            gui.handle_event(Event::MouseDown {
                button: MouseButton::Left,
                x: mx as f32,
                y: my as f32,
            });
        }
        if !gfx.is_mouse_button_pressed(0) && gui.is_mouse_down() {
            gui.handle_event(Event::MouseUp {
                button: MouseButton::Left,
                x: mx as f32,
                y: my as f32,
            });
        }

        // Iniciar frame de migui
        gui.begin_frame();

        // Ejecutar statements del bloque en cada frame
        for stmt in &frame_stmts {
            ejecutar_stmt_migui(
                stmt,
                executor,
                funcs,
                gui,
                &mut loaded_modules,
                &mut importing_stack,
                &mut checkbox_states,
                &mut slider_states,
                &mut textbox_states,
                &mut window_states,
            );
        }

        gui.end_frame();

        // Debug: mostrar comandos generados
        if !gui.draw_commands().is_empty() {
            println!("[MIGUI] {} comandos generados", gui.draw_commands().len());
        }

        // Renderizar con el backend optimizado
        gfx.render_migui_frame(gui.draw_commands());
    }
}
