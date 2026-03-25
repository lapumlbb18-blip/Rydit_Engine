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
    // Estado del input
    let mut input = InputEstado::new();

    // Contexto de imports: módulos cargados y stack de imports en progreso
    let mut loaded_modules: HashSet<String> = HashSet::new();
    let mut importing_stack: Vec<String> = Vec::new();

    // Game loop principal
    while !gfx.should_close() {
        // Input primero (Rust = Arquitecto)
        input.actualizar(gfx);
        let escape = gfx.is_key_pressed(rydit_gfx::Key::Escape);

        // Iniciar dibujo
        {
            let mut d = gfx.begin_draw();
            d.clear(crate::ColorRydit::Negro);

            // Ejecutar programa en cada frame
            for stmt in &program.statements {
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

            // FPS counter
            d.draw_text("RyDit v0.0.9", 10, 10, 20, crate::ColorRydit::Blanco);
        }
        // end_draw automático cuando d sale de scope

        if escape {
            break;
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
