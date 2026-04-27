// Demo Menu Bar - Migui Dear ImGui style con input SDL2 completo
// Soporte: teclado, mouse, touch Android, Termux-X11
//
// Uso: cargo run --bin demo_menu_bar --release

use migui::{Event, Key, Menu, MenuBar, MenuItem, Migui, MouseButton};
use sdl2::event::Event as SdlEvent;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color as SdlColor;
use sdl2::rect::Rect as SdlRect;

fn main() {
    // ========================================================================
    // HINTS PARA TERMUX-X11 / ANDROID (pipeline: Zink/DRI3 → OpenGL ES → VirGL)
    // ========================================================================
    sdl2::hint::set("SDL_VIDEODRIVER", "x11");
    sdl2::hint::set("SDL_RENDER_DRIVER", "opengles2");
    sdl2::hint::set("SDL_RENDER_VSYNC", "1");
    sdl2::hint::set("SDL_VIDEO_X11_FORCE_EGL", "1");
    sdl2::hint::set("SDL_HINT_ANDROID_SEPARATE_MOUSE_AND_TOUCH", "1");
    sdl2::hint::set("SDL_HINT_TOUCH_MOUSE_EVENTS", "1");
    sdl2::hint::set("SDL_HINT_ENABLE_SCREEN_KEYBOARD", "1");
    sdl2::hint::set("SDL_HINT_IME_SHOW_UI", "1");

    println!("[DEMO MENU BAR] migui Dear ImGui style + SDL2 input completo");
    println!("[DEMO MENU BAR] Pipeline: Zink/DRI3 -> OpenGL ES -> VirGL fallback");
    println!("[DEMO MENU BAR] ESC = Salir | Click = Abrir menús | Teclado = Input texto");

    // ========================================================================
    // INIT SDL2
    // ========================================================================
    let sdl = sdl2::init().expect("SDL2 init failed");
    let video = sdl.video().expect("SDL2 video failed");
    let mut event_pump = sdl.event_pump().expect("SDL2 event_pump failed");

    let window = video
        .window("migui Menu Bar Demo - ry-dit v0.13.0", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Window build failed");

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Canvas build failed");

    // Activar text input para teclado Android (SDL2 0.37 usa sys directo)
    unsafe {
        sdl2::sys::SDL_StartTextInput();
    }

    // ========================================================================
    // CREAR MENÚS (File, Edit, View, Emulation, Help)
    // ========================================================================
    let mut menu_bar = MenuBar::new(vec![
        Menu::new("File", vec![
            MenuItem::new("New").shortcut("Ctrl+N"),
            MenuItem::new("Open...").shortcut("Ctrl+O"),
            MenuItem::new("Save").shortcut("Ctrl+S"),
            MenuItem::separator(),
            MenuItem::new("Settings..."),
            MenuItem::separator(),
            MenuItem::new("Exit").shortcut("Alt+F4"),
        ]),
        Menu::new("Edit", vec![
            MenuItem::new("Undo").shortcut("Ctrl+Z"),
            MenuItem::new("Redo").shortcut("Ctrl+Y"),
            MenuItem::separator(),
            MenuItem::new("Cut").shortcut("Ctrl+X"),
            MenuItem::new("Copy").shortcut("Ctrl+C"),
            MenuItem::new("Paste").shortcut("Ctrl+V"),
        ]),
        Menu::new("View", vec![
            MenuItem::new("CPU Registers"),
            MenuItem::new("Memory Viewer"),
            MenuItem::new("Screen"),
            MenuItem::separator(),
            MenuItem::new("Zoom In").shortcut("Ctrl++"),
            MenuItem::new("Zoom Out").shortcut("Ctrl+-"),
        ]),
        Menu::new("Emulation", vec![
            MenuItem::new("Start").shortcut("F5"),
            MenuItem::new("Pause").shortcut("F6"),
            MenuItem::new("Stop").shortcut("F7"),
            MenuItem::separator(),
            MenuItem::new("Reset").shortcut("Ctrl+R"),
        ]),
        Menu::new("Help", vec![
            MenuItem::new("About"),
            MenuItem::new("Documentation"),
        ]),
    ]);

    let mut gui = Migui::new();
    let mut log_lines: Vec<String> = vec![
        "[INFO] migui v0.4.1 initialized".into(),
        "[INFO] MenuBar created with 5 menus".into(),
        "[INFO] SDL2 input enabled (keyboard + mouse + touch)".into(),
        "[INFO] Waiting for input...".into(),
    ];
    let mut frame: u64 = 0;
    let mut fps_counter: u64 = 0;
    let mut fps_display: f64 = 0.0;
    let mut last_fps_time = std::time::Instant::now();

    // ========================================================================
    // GAME LOOP
    // ========================================================================
    'running: loop {
        gui.begin_frame();

        // ====================================================================
        // PROCESAR EVENTOS SDL2
        // ====================================================================
        for event in event_pump.poll_iter() {
            match event {
                // --- Salir ---
                SdlEvent::Quit { .. } => break 'running,

                // --- Teclado (KeyDown sin repeat) ---
                SdlEvent::KeyDown {
                    keycode: Some(kc),
                    repeat: false,
                    ..
                } => {
                    // Mapear Keycode → migui Key
                    let migui_key = keycode_to_migui_key(kc);
                    if let Some(key) = migui_key {
                        gui.handle_event(Event::KeyDown { key });
                    }

                    // Atajos directos
                    match kc {
                        Keycode::Escape => break 'running,
                        Keycode::F5 => add_log(&mut log_lines, "[EMULATION] Start (F5)"),
                        Keycode::F6 => add_log(&mut log_lines, "[EMULATION] Pause (F6)"),
                        Keycode::F7 => add_log(&mut log_lines, "[EMULATION] Stop (F7)"),
                        _ => {}
                    }
                }

                // --- Teclado (KeyUp) ---
                SdlEvent::KeyUp {
                    keycode: Some(kc), ..
                } => {
                    if let Some(key) = keycode_to_migui_key(kc) {
                        gui.handle_event(Event::KeyUp { key });
                    }
                }

                // --- Text Input (teclado Android / virtual) ---
                SdlEvent::TextInput { text, .. } => {
                    for ch in text.chars() {
                        gui.handle_event(Event::CharTyped { ch });
                        add_log(&mut log_lines, &format!("[INPUT] CharTyped: '{}'", ch));
                    }
                }

                // --- Text Editing (IME composition) ---
                SdlEvent::TextEditing { text, .. } => {
                    if !text.is_empty() {
                        add_log(&mut log_lines, &format!("[INPUT] TextEditing: '{}'", text));
                    }
                }

                // --- Mouse Move ---
                SdlEvent::MouseMotion { x, y, .. } => {
                    gui.handle_event(Event::MouseMove {
                        x: x as f32,
                        y: y as f32,
                    });
                }

                // --- Mouse Down ---
                SdlEvent::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    let btn = match mouse_btn {
                        sdl2::mouse::MouseButton::Left => MouseButton::Left,
                        sdl2::mouse::MouseButton::Right => MouseButton::Right,
                        sdl2::mouse::MouseButton::Middle => MouseButton::Middle,
                        _ => MouseButton::Left,
                    };
                    gui.handle_event(Event::MouseDown {
                        button: btn,
                        x: x as f32,
                        y: y as f32,
                    });
                    add_log(&mut log_lines, &format!("[MOUSE] Down {:?} at ({}, {})", btn, x, y));
                }

                // --- Mouse Up ---
                SdlEvent::MouseButtonUp {
                    mouse_btn, x, y, ..
                } => {
                    let btn = match mouse_btn {
                        sdl2::mouse::MouseButton::Left => MouseButton::Left,
                        sdl2::mouse::MouseButton::Right => MouseButton::Right,
                        sdl2::mouse::MouseButton::Middle => MouseButton::Middle,
                        _ => MouseButton::Left,
                    };
                    gui.handle_event(Event::MouseUp {
                        button: btn,
                        x: x as f32,
                        y: y as f32,
                    });
                }

                // --- Mouse Wheel ---
                SdlEvent::MouseWheel { x, y, .. } => {
                    add_log(&mut log_lines, &format!("[MOUSE] Wheel ({}, {})", x, y));
                }

                // --- Touch (Android) ---
                SdlEvent::FingerDown { x, y, .. } => {
                    gui.handle_event(Event::MouseMove {
                        x: x * 800.0,
                        y: y * 600.0,
                    });
                    gui.handle_event(Event::MouseDown {
                        button: MouseButton::Left,
                        x: x * 800.0,
                        y: y * 600.0,
                    });
                    add_log(&mut log_lines, &format!("[TOUCH] Down at ({:.0}, {:.0})", x * 800.0, y * 600.0));
                }
                SdlEvent::FingerMotion { x, y, .. } => {
                    gui.handle_event(Event::MouseMove {
                        x: x * 800.0,
                        y: y * 600.0,
                    });
                }
                SdlEvent::FingerUp { x, y, .. } => {
                    gui.handle_event(Event::MouseUp {
                        button: MouseButton::Left,
                        x: x * 800.0,
                        y: y * 600.0,
                    });
                }

                // --- Window Events ---
                SdlEvent::Window { win_event, .. } => {
                    match win_event {
                        sdl2::event::WindowEvent::SizeChanged(w, h) => {
                            add_log(&mut log_lines, &format!("[WINDOW] Resized to {}x{}", w, h));
                        }
                        sdl2::event::WindowEvent::FocusGained => {
                            add_log(&mut log_lines, "[WINDOW] Focus gained");
                        }
                        sdl2::event::WindowEvent::FocusLost => {
                            add_log(&mut log_lines, "[WINDOW] Focus lost");
                        }
                        _ => {}
                    }
                }

                _ => {}
            }
        }

        // ====================================================================
        // RENDERIZAR MENÚ BAR
        // ====================================================================
        gui.menu_bar(&mut menu_bar, 0.0, 0.0, 800.0);

        // ====================================================================
        // RENDERIZAR COMANDOS DE DIBUJO
        // ====================================================================
        canvas.set_draw_color(SdlColor::RGB(25, 25, 35));
        canvas.clear();

        for cmd in gui.draw_commands() {
            match cmd {
                migui::DrawCommand::DrawRect { rect, color } => {
                    canvas.set_draw_color(SdlColor::RGBA(color.r, color.g, color.b, color.a));
                    let _ = canvas.fill_rect(SdlRect::new(
                        rect.x as i32, rect.y as i32,
                        rect.w.max(1.0) as u32, rect.h.max(1.0) as u32,
                    ));
                }
                migui::DrawCommand::DrawLine { x1, y1, x2, y2, color, thickness: _ } => {
                    canvas.set_draw_color(SdlColor::RGBA(color.r, color.g, color.b, color.a));
                    let _ = canvas.draw_line(
                        (*x1 as i32, *y1 as i32), (*x2 as i32, *y2 as i32),
                    );
                }
                migui::DrawCommand::DrawText { text, x, y, size, color } => {
                    // Block text (hasta tener TTF real)
                    let char_w = (*size as i32 / 2).max(4);
                    for (ci, _ch) in text.chars().enumerate().take(120) {
                        canvas.set_draw_color(SdlColor::RGBA(color.r, color.g, color.b, color.a));
                        let _ = canvas.fill_rect(SdlRect::new(
                            *x as i32 + ci as i32 * (char_w + 1),
                            *y as i32,
                            char_w as u32,
                            *size as u32,
                        ));
                    }
                }
                migui::DrawCommand::Clear { color } => {
                    canvas.set_draw_color(SdlColor::RGBA(color.r, color.g, color.b, color.a));
                    let _ = canvas.clear();
                }
                migui::DrawCommand::DrawViewport3D { .. } => {
                    // Ignorado en este demo de UI simple
                }
            }
        }

        // ====================================================================
        // LOG PANEL (abajo)
        // ====================================================================
        let log_y = 400i32;
        canvas.set_draw_color(SdlColor::RGB(30, 30, 40));
        let _ = canvas.fill_rect(SdlRect::new(0, log_y, 800, 170));

        // Borde superior
        canvas.set_draw_color(SdlColor::RGB(60, 60, 80));
        let _ = canvas.draw_line((0, log_y), (800, log_y));

        // Título
        canvas.set_draw_color(SdlColor::RGB(200, 200, 220));
        let title = "=== Event Log (SDL2 Input + Touch + Keyboard) ===";
        for (ci, _ch) in title.chars().enumerate().take(60) {
            let _ = canvas.fill_rect(SdlRect::new(10 + ci as i32 * 5, log_y + 4, 4, 12));
        }

        // Líneas de log (últimas 8)
        let start = if log_lines.len() > 8 { log_lines.len() - 8 } else { 0 };
        for (i, line) in log_lines.iter().enumerate().skip(start) {
            let ly = log_y + 22 + (i - start) as i32 * 18;
            let color = if line.starts_with("[MOUSE]") || line.starts_with("[TOUCH]") {
                SdlColor::RGB(100, 180, 255)
            } else if line.starts_with("[INPUT]") {
                SdlColor::RGB(180, 255, 100)
            } else if line.starts_with("[EMULATION]") {
                SdlColor::RGB(255, 200, 100)
            } else if line.starts_with("[WINDOW]") {
                SdlColor::RGB(200, 150, 255)
            } else {
                SdlColor::RGB(180, 180, 200)
            };
            canvas.set_draw_color(color);
            for (ci, _ch) in line.chars().enumerate().take(100) {
                let _ = canvas.fill_rect(SdlRect::new(10 + ci as i32 * 4, ly as i32, 3, 10));
            }
        }

        // ====================================================================
        // STATUS BAR (arriba del log)
        // ====================================================================
        // FPS counter
        fps_counter += 1;
        let now = std::time::Instant::now();
        if now.duration_since(last_fps_time).as_secs_f64() >= 1.0 {
            fps_display = fps_counter as f64 / now.duration_since(last_fps_time).as_secs_f64();
            fps_counter = 0;
            last_fps_time = now;
        }

        canvas.set_draw_color(SdlColor::RGB(40, 40, 55));
        let _ = canvas.fill_rect(SdlRect::new(0, 24, 800, 20));

        let status = format!("FPS: {:.1} | Frame: {} | Events: {}", fps_display, frame, log_lines.len());
        canvas.set_draw_color(SdlColor::RGB(200, 200, 210));
        for (ci, _ch) in status.chars().enumerate().take(60) {
            let _ = canvas.fill_rect(SdlRect::new(10 + ci as i32 * 4, 28, 3, 10));
        }

        canvas.present();
        frame += 1;

        // Limitar ~30fps
        std::thread::sleep(std::time::Duration::from_millis(33));
    }

    // Cleanup
    unsafe {
        sdl2::sys::SDL_StopTextInput();
    }
    println!("[DEMO MENU BAR] Shutdown. Frames: {} | Final FPS: {:.1}", frame, fps_display);
}

// ============================================================================
// HELPERS
// ============================================================================

fn keycode_to_migui_key(kc: Keycode) -> Option<Key> {
    match kc {
        Keycode::Escape => Some(Key::Escape),
        Keycode::Return | Keycode::Return2 => Some(Key::Enter),
        Keycode::Backspace => Some(Key::Backspace),
        Keycode::Up => Some(Key::ArrowUp),
        Keycode::Down => Some(Key::ArrowDown),
        Keycode::Left => Some(Key::ArrowLeft),
        Keycode::Right => Some(Key::ArrowRight),
        Keycode::A => Some(Key::A),
        Keycode::B => Some(Key::B),
        Keycode::C => Some(Key::C),
        Keycode::D => Some(Key::D),
        Keycode::E => Some(Key::E),
        Keycode::F => Some(Key::F),
        Keycode::G => Some(Key::G),
        Keycode::H => Some(Key::H),
        Keycode::I => Some(Key::I),
        Keycode::J => Some(Key::J),
        Keycode::K => Some(Key::K),
        Keycode::L => Some(Key::L),
        Keycode::M => Some(Key::M),
        Keycode::N => Some(Key::N),
        Keycode::O => Some(Key::O),
        Keycode::P => Some(Key::P),
        Keycode::Q => Some(Key::Q),
        Keycode::R => Some(Key::R),
        Keycode::S => Some(Key::S),
        Keycode::T => Some(Key::T),
        Keycode::U => Some(Key::U),
        Keycode::V => Some(Key::V),
        Keycode::W => Some(Key::W),
        Keycode::X => Some(Key::X),
        Keycode::Y => Some(Key::Y),
        Keycode::Z => Some(Key::Z),
        Keycode::Num1 => Some(Key::Num1),
        Keycode::Num2 => Some(Key::Num2),
        Keycode::Num3 => Some(Key::Num3),
        Keycode::Num4 => Some(Key::Num4),
        Keycode::Num5 => Some(Key::Num5),
        _ => None,
    }
}

fn add_log(lines: &mut Vec<String>, msg: &str) {
    lines.push(msg.to_string());
    if lines.len() > 50 {
        lines.remove(0);
    }
}
