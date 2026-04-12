//! Demo Rybot GUI — Muestra la interfaz visual de Rybot con migui
//!
//! ```bash
//! cargo run --bin demo_rybot_gui --release
//! ```
//!
//! Controles:
//! - ESPACIO: Toggle GUI
//! - 1: Toggle New Project
//! - 2: Toggle Inspector
//! - 3: Toggle Scene Tree
//! - 4: Toggle Stats
//! - R: Reset layout
//! - ESC: Salir

use raylib::prelude::*;
use rybot::{RybotEngine, RybotGui, ProjectTemplate};

fn main() -> Result<(), String> {
    println!("🛡️ Ry-Dit — Demo Rybot GUI v0.19.0");

    let (mut rl, thread) = raylib::init()
        .size(1200, 700)
        .title("Rybot GUI — Motor Central Ry-Dit")
        .build();

    let mut engine = RybotEngine::new();
    let mut gui_state = RybotGui::new();
    gui_state.open = true;
    gui_state.show_inspector = true;
    gui_state.show_scene_tree = true;
    gui_state.show_stats = true;

    // Crear escena de demo
    use rybot::SceneNode;
    use rybot::NodeType;
    let mut player = SceneNode::new("player", NodeType::Entity);
    player.position = (0.0, 0.0);
    engine.scene_mut().add_node(player);

    let mut camera = SceneNode::new("main_camera", NodeType::Camera);
    engine.scene_mut().add_node(camera);

    let mut light = SceneNode::new("directional_light", NodeType::Light);
    engine.scene_mut().add_node(light);

    let mut hud = SceneNode::new("hud", NodeType::UI);
    hud.add_child(SceneNode::new("health_bar", NodeType::UI));
    hud.add_child(SceneNode::new("minimap", NodeType::UI));
    engine.scene_mut().add_node(hud);

    // migui instance
    let mut migui = migui::Migui::new();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        // === INPUT ===
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            gui_state.open = !gui_state.open;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            gui_state.show_new_project = !gui_state.show_new_project;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            gui_state.show_inspector = !gui_state.show_inspector;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            gui_state.show_scene_tree = !gui_state.show_scene_tree;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
            gui_state.show_stats = !gui_state.show_stats;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            gui_state.reset();
        }

        // Update engine
        engine.update(dt);

        // Feed input to migui
        let mx = rl.get_mouse_x() as f32;
        let my = rl.get_mouse_y() as f32;
        migui.handle_event(migui::Event::MouseMove { x: mx, y: my });

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            migui.handle_event(migui::Event::MouseDown {
                button: migui::MouseButton::Left,
                x: mx,
                y: my,
            });
        }
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            migui.handle_event(migui::Event::MouseUp {
                button: migui::MouseButton::Left,
                x: mx,
                y: my,
            });
        }

        // === RENDER ===
        let fps = rl.get_fps();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(25, 25, 35, 255));

        // Fondo — grid sutil
        draw_grid_bg(&mut d, 1200, 700);

        // Engine update on migui
        migui.begin_frame();
        if gui_state.open {
            gui_state.draw(&mut migui, &engine);
        }
        migui.end_frame();

        // Render migui commands
        render_migui_commands(&mut d, migui.draw_commands());

        // HUD inferior
        d.draw_text(
            &format!(
                "Rybot v0.19.0 | Frame: {} | FPS: {} | [ESP] Toggle GUI | [1-4] Paneles | [R] Reset | [ESC] Salir",
                engine.frame(),
                fps
            ),
            10,
            680,
            12,
            Color::new(150, 150, 180, 255),
        );
    }

    println!("✅ Rybot GUI demo cerrado correctamente");
    Ok(())
}

fn draw_grid_bg(d: &mut RaylibDrawHandle, w: i32, h: i32) {
    let spacing = 40;
    let color = Color::new(35, 35, 50, 255);
    for x in (0..w).step_by(spacing) {
        d.draw_line(x, 0, x, h, color);
    }
    for y in (0..h).step_by(spacing) {
        d.draw_line(0, y, w, y, color);
    }
}

fn render_migui_commands(d: &mut RaylibDrawHandle, commands: &[migui::DrawCommand]) {
    for cmd in commands {
        match cmd {
            migui::DrawCommand::Clear { .. } => {}
            migui::DrawCommand::DrawRect { rect, color } => {
                let c = to_raylib_color(*color);
                d.draw_rectangle(rect.x as i32, rect.y as i32, rect.w as i32, rect.h as i32, c);
            }
            migui::DrawCommand::DrawText { text, x, y, size, color } => {
                let c = to_raylib_color(*color);
                d.draw_text(text, *x as i32, *y as i32, *size as i32, c);
            }
            migui::DrawCommand::DrawLine { x1, y1, x2, y2, color, thickness } => {
                let c = to_raylib_color(*color);
                d.draw_line(*x1 as i32, *y1 as i32, *x2 as i32, *y2 as i32, c);
                if *thickness > 2.0 {
                    d.draw_line(
                        *x1 as i32, *y1 as i32 + 1, *x2 as i32, *y2 as i32 + 1, c);
                }
            }
        }
    }
}

fn to_raylib_color(c: migui::Color) -> Color {
    Color::new(c.r, c.g, c.b, c.a)
}
