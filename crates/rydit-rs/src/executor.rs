// crates/rydit-rs/src/executor.rs
// Ejecución de programas en diferentes modos (comandante, gfx, migui)

use std::collections::{HashMap, HashSet};
use std::time::Instant;

use blast_core::Executor;
use rydit_parser::{Program, Stmt};
use migui::Migui;
use rydit_gfx::render_queue::{DrawCommand, RenderQueue};
use rydit_gfx::RyditGfx;

use crate::{
    ejecutar_stmt, ejecutar_stmt_gfx, ejecutar_stmt_migui, evaluar_expr_migui, InputEstado,
};

// 🆕 RyBot - Inspector + Registry
use crate::rybot::RyBot;

/// Ejecutar programa en modo comandante (sin gráficos)
pub fn ejecutar_programa<'a>(
    program: &Program<'a>,
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
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
pub fn ejecutar_programa_gfx<'a>(
    program: &Program<'a>,
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
    gfx: &mut RyditGfx,
) {
    // Inicializar debug log
    rydit_gfx::debug_log::debug_init(
        "/data/data/com.termux/files/home/shield-project/rydit_debug.log",
    );
    rydit_gfx::debug_log::debug_info("=== INICIANDO GAME LOOP ===");

    // Estado del input
    let mut input = InputEstado::new();

    // 🆕 RyBot - Inspector + Registry
    let mut rybot = RyBot::new();
    rybot.info("RyBot", "Game loop iniciado");

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
    // NOTA: Ahora usamos RenderQueue, así que solo inicializamos variables
    for stmt in &program.statements {
        match stmt {
            // While y Blocks son game loops - NO ejecutar aquí
            Stmt::While { .. } | Stmt::Block(_) => {
                rydit_gfx::debug_log::debug_info("Statement es game loop (While/Block)");
            }
            // Todo lo demás es inicialización
            _ => {
                rydit_gfx::debug_log::debug_log("Ejecutando statement inicial");
                // Inicialización sin dibujar
            }
        }
    }

    // Crear Render Queue (8192+ draw calls)
    let mut queue = RenderQueue::with_capacity(8192);
    eprintln!(
        "[EXECUTOR GFX] Render Queue creada: capacidad={}",
        queue.capacity()
    );

    // Delta time para físicas consistentes
    use std::time::Instant;
    let mut last_time = Instant::now();

    // Buscar el primer While o Block como game loop principal
    let mut found_loop = false;
    for stmt in &program.statements {
        match stmt {
            Stmt::While { condition, body } => {
                found_loop = true;
                rydit_gfx::debug_log::debug_info("=== ENCONTRADO GAME LOOP WHILE ===");
                rydit_gfx::debug_log::debug_log(&format!("Body tiene {} statements", body.len()));

                // El While ES el game loop principal
                let mut frame_count = 0;
                let mut frame_start = Instant::now();

                loop {
                    // 🆕 RyBot begin frame
                    rybot.begin_frame();

                    // Input primero
                    input.actualizar(gfx);
                    let escape = gfx.is_key_pressed(rydit_gfx::Key::Escape);

                    // Calcular delta time (para físicas y partículas)
                    let now = Instant::now();
                    let dt = now.duration_since(last_time).as_secs_f32();
                    last_time = now;

                    // Guardar delta time para acceso desde .rydit
                    executor.guardar("__DT__", blast_core::Valor::Num(dt as f64));

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

                    // === FASE 1: Acumular comandos en Render Queue ===

                    // Clear screen
                    queue.push(DrawCommand::Clear {
                        color: crate::ColorRydit::Negro,
                    });

                    eprintln!("[EXECUTOR] Body tiene {} statements", body.len());
                    for (i, s) in body.iter().enumerate() {
                        eprintln!("[EXECUTOR] Statement {}: {:?}", i, s);
                    }

                    // Ejecutar statements del body (acumulan en queue)
                    for s in body {
                        eprintln!("[EXECUTOR] Ejecutando statement (queue)...");
                        let break_signal = ejecutar_stmt_gfx(
                            s,
                            executor,
                            funcs,
                            &mut queue, // ← Usar RenderQueue en vez de DrawHandle
                            &mut input,
                            &mut loaded_modules,
                            &mut importing_stack,
                        );
                        if break_signal == Some(true) {
                            rydit_gfx::debug_log::debug_log("Break detectado en body");
                            break;
                        }
                    }

                    // FPS counter + debug info (también va a la queue)
                    queue.push(DrawCommand::Text {
                        text: "RyDit v0.9.1".to_string(),
                        x: 10,
                        y: 10,
                        size: 20,
                        color: crate::ColorRydit::Blanco,
                    });
                    queue.push(DrawCommand::Text {
                        text: format!("Frame: {}", frame_count),
                        x: 10,
                        y: 40,
                        size: 20,
                        color: crate::ColorRydit::Verde,
                    });

                    // === FASE 2: Ejecutar Render Queue ===
                    // Obtener assets para texturas
                    use crate::modules::assets;
                    let assets_ref = assets::get_assets();
                    let assets_borrow = assets_ref.borrow();

                    // ✅ v0.10.4: Ejecutar queue y dibujar partículas en el mismo begin_draw
                    {
                        let mut d = gfx.begin_draw();

                        // Ejecutar RenderQueue con handle existente
                        queue.execute_with_handle(&mut d, &assets_borrow);

                        // ✅ v0.10.4: Dibujar partículas (misma sesión de dibujado)
                        use crate::modules::particles;
                        particles::draw_particles_with_handle(&mut d);

                        // Drop explícito para forzar buffer swap
                        drop(d);
                    }

                    eprintln!(
                        "[EXECUTOR] Frame {} completado - Queue ejecutada (stats: {})",
                        frame_count,
                        queue.stats()
                    );

                    // 🆕 RyBot end frame
                    let frame_time = frame_start.elapsed().as_secs_f32() * 1000.0;
                    rybot.record_render(frame_time);
                    rybot.set_entity_count(frame_count); // Placeholder
                    rybot.end_frame(frame_time);

                    // Verificar módulos no usados cada 100 frames
                    if frame_count % 100 == 0 {
                        rybot.check_unused_modules();
                    }

                    frame_count += 1;

                    // Log cada 10 frames
                    if frame_count % 10 == 0 {
                        rydit_gfx::debug_log::debug_log_frame(&format!(
                            "Frame {} completado - Stats: {}",
                            frame_count,
                            queue.stats()
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
            Stmt::Block(stmts) => {
                found_loop = true;
                // Block es game loop - ejecutar en cada frame con RenderQueue
                while !gfx.should_close() {
                    input.actualizar(gfx);
                    let escape = gfx.is_key_pressed(rydit_gfx::Key::Escape);

                    // Calcular delta time
                    let now = Instant::now();
                    let dt = now.duration_since(last_time).as_secs_f32();
                    last_time = now;
                    executor.guardar("__DT__", blast_core::Valor::Num(dt as f64));

                    // Clear queue
                    queue.clear();

                    // Clear screen
                    queue.push(DrawCommand::Clear {
                        color: crate::ColorRydit::Negro,
                    });

                    // Ejecutar statements (acumulan en queue)
                    for s in stmts {
                        let break_signal = ejecutar_stmt_gfx(
                            s,
                            executor,
                            funcs,
                            &mut queue, // ← Usar RenderQueue
                            &mut input,
                            &mut loaded_modules,
                            &mut importing_stack,
                        );
                        if break_signal == Some(true) {
                            break;
                        }
                    }

                    // FPS counter
                    queue.push(DrawCommand::Text {
                        text: "RyDit v0.9.1".to_string(),
                        x: 10,
                        y: 10,
                        size: 20,
                        color: crate::ColorRydit::Blanco,
                    });

                    // ✅ v0.10.4: Ejecutar queue con assets y partículas en mismo begin_draw
                    use crate::modules::assets;
                    let assets_ref = assets::get_assets();
                    let assets_borrow = assets_ref.borrow();

                    {
                        let mut d = gfx.begin_draw();
                        queue.execute_with_handle(&mut d, &assets_borrow);

                        // ✅ v0.10.4: Dibujar partículas
                        use crate::modules::particles;
                        particles::draw_particles_with_handle(&mut d);

                        drop(d);
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

            // Clear queue
            queue.clear();

            // Clear screen
            queue.push(DrawCommand::Clear {
                color: crate::ColorRydit::Negro,
            });

            // Ejecutar todos los statements que no son While/Block
            for stmt in &program.statements {
                match stmt {
                    Stmt::While { .. } | Stmt::Block(_) => {}
                    _ => {
                        ejecutar_stmt_gfx(
                            stmt,
                            executor,
                            funcs,
                            &mut queue, // ← Usar RenderQueue
                            &mut input,
                            &mut loaded_modules,
                            &mut importing_stack,
                        );
                    }
                }
            }

            // FPS counter
            queue.push(DrawCommand::Text {
                text: "RyDit v0.9.1".to_string(),
                x: 10,
                y: 10,
                size: 20,
                color: crate::ColorRydit::Blanco,
            });

            // ✅ v0.10.4: Ejecutar queue con assets y partículas en mismo begin_draw
            use crate::modules::assets;
            let assets_ref = assets::get_assets();
            let assets_borrow = assets_ref.borrow();

            {
                let mut d = gfx.begin_draw();
                queue.execute_with_handle(&mut d, &assets_borrow);

                // ✅ v0.10.4: Dibujar partículas
                use crate::modules::particles;
                particles::draw_particles_with_handle(&mut d);

                drop(d);
            }

            if escape {
                break;
            }
        }
    }
}

/// Ejecutar programa en modo migui (GUI)
pub fn ejecutar_programa_migui<'a>(
    program: &Program<'a>,
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
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
            Stmt::Function { name, params, body } => {
                funcs.insert(name.clone(), (params.clone(), body.clone()));
            }
            Stmt::Assign { name, value } => {
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
    let frame_stmts: Vec<&Stmt<'a>> = program
        .statements
        .iter()
        .filter(|s| matches!(s, Stmt::Block(_)))
        .flat_map(|s| {
            if let Stmt::Block(stmts) = s {
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
