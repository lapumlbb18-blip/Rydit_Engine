// Demo Panel Visual - ry-dit Panel de Control
// Inspirado en: bgfx_libs/console_emulator_prototype.cpp
// Stack: SDL2 + SDL2_ttf + events-ry + ry-anim
// Pipeline: Zink/DRI3 -> OpenGL ES -> VirGL fallback
//
// Uso: cargo run --bin demo_panel_visual --release
//
// Controles:
// - ESC: Salir
// - SPACE: Cambiar escena animacion
// - 1-4: Toggle paneles
// - Escribe comandos en consola + ENTER: help, load, exec, debug, echo
// - Mouse: Click en panel consola para focus

use events_ry::{InputEvent, Key};
use ry_anim::{disney, illusions, effects, science_anim};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color as SdlColor;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

// ============================================================================
// CONFIGURACION
// ============================================================================
const WIN_W: u32 = 800;
const WIN_H: u32 = 600;
const TARGET_FPS: u32 = 30;
const HEADER_H: i32 = 22;
const LINE_H: i32 = 16;
const CONSOLE_MAX_LINES: usize = 200;
const CHAR_W: i32 = 6;
const CHAR_H: i32 = 12;

// ============================================================================
// COLORES
// ============================================================================
const C_BG: SdlColor = SdlColor::RGB(18, 18, 24);
const C_PANEL_BG: SdlColor = SdlColor::RGB(28, 28, 36);
const C_HEADER_BG: SdlColor = SdlColor::RGB(40, 40, 55);
const C_TEXT: SdlColor = SdlColor::RGB(200, 200, 210);
const C_GREEN: SdlColor = SdlColor::RGB(80, 200, 120);
const C_RED: SdlColor = SdlColor::RGB(220, 80, 80);
const C_YELLOW: SdlColor = SdlColor::RGB(220, 200, 60);
const C_CYAN: SdlColor = SdlColor::RGB(80, 180, 220);
const C_BLUE: SdlColor = SdlColor::RGB(80, 120, 220);
const C_ORANGE: SdlColor = SdlColor::RGB(220, 160, 60);
const C_BORDER: SdlColor = SdlColor::RGB(50, 50, 65);

// ============================================================================
// ESTADO
// ============================================================================
struct State {
    running: bool,
    frame: u64,
    t: f64,
    scene: usize,
    scenes: Vec<&'static str>,
    show_screen: bool,
    show_console: bool,
    show_input: bool,
    show_controls: bool,
    console_input: String,
    console_lines: Vec<String>,
    keys_down: Vec<String>,
    mouse_x: i32,
    mouse_y: i32,
    fps: f64,
    fps_frames: u64,
    fps_last: std::time::Instant,
}

impl State {
    fn new() -> Self {
        Self {
            running: true,
            frame: 0,
            t: 0.0,
            scene: 0,
            scenes: vec![
                "Disney: Follow Through",
                "Disney: Arcs",
                "Illusion: Rotating Snakes",
                "Effect: Neon Glow",
                "Effect: Morphing",
                "Science: Tusi Couple",
            ],
            show_screen: true,
            show_console: true,
            show_input: true,
            show_controls: false,
            console_input: String::new(),
            console_lines: vec![
                "ry-dit Console v0.13.0".into(),
                "Escribe 'help' para ver comandos".into(),
            ],
            keys_down: Vec::new(),
            mouse_x: 0,
            mouse_y: 0,
            fps: 0.0,
            fps_frames: 0,
            fps_last: std::time::Instant::now(),
        }
    }

    fn update_fps(&mut self) {
        self.fps_frames += 1;
        let elapsed = self.fps_last.elapsed().as_secs_f64();
        if elapsed >= 1.0 {
            self.fps = self.fps_frames as f64 / elapsed;
            self.fps_frames = 0;
            self.fps_last = std::time::Instant::now();
        }
    }
}

// ============================================================================
// MAIN
// ============================================================================
fn main() {
    sdl2::hint::set("SDL_VIDEODRIVER", "x11");
    sdl2::hint::set("SDL_RENDER_DRIVER", "opengles2");
    sdl2::hint::set("SDL_RENDER_VSYNC", "1");
    sdl2::hint::set("SDL_VIDEO_X11_FORCE_EGL", "1");
    sdl2::hint::set("SDL_HINT_ANDROID_SEPARATE_MOUSE_AND_TOUCH", "1");
    sdl2::hint::set("SDL_HINT_TOUCH_MOUSE_EVENTS", "1");
    sdl2::hint::set("SDL_HINT_ENABLE_SCREEN_KEYBOARD", "1");
    sdl2::hint::set("SDL_HINT_IME_SHOW_UI", "1");

    println!("[DEMO PANEL VISUAL] ry-dit v0.13.0");
    println!("[DEMO PANEL VISUAL] Pipeline: Zink/DRI3 -> OpenGL ES -> VirGL fallback");
    println!("[DEMO PANEL VISUAL] ESC=Salir | SPACE=Escena | 1-4=Paneles | ENTER=Comando");

    let sdl = sdl2::init().expect("SDL2 init failed");
    let video = sdl.video().expect("SDL2 video failed");
    let mut event_pump = sdl.event_pump().expect("SDL2 event_pump failed");

    let window = video
        .window("ry-dit Panel v0.13.0", WIN_W, WIN_H)
        .position_centered()
        .opengl()
        .build()
        .expect("Window build failed");

    let mut canvas: Canvas<Window> = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Canvas build failed");

    // Text input para consola Android
    unsafe { sdl2::sys::SDL_StartTextInput(); }

    let mut state = State::new();
    let frame_ms = 1000.0 / TARGET_FPS as f64;

    'running: loop {
        let frame_start = std::time::Instant::now();
        state.t += 1.0 / TARGET_FPS as f64;
        state.frame += 1;
        state.update_fps();

        // EVENTOS
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(k), repeat: false, .. } => {
                    match k {
                        Keycode::Escape => break 'running,
                        Keycode::Space => {
                            state.scene = (state.scene + 1) % state.scenes.len();
                            add_line(&mut state.console_lines,
                                &format!("Scene -> {}", state.scenes[state.scene]));
                        }
                        Keycode::Num1 => state.show_screen = !state.show_screen,
                        Keycode::Num2 => state.show_console = !state.show_console,
                        Keycode::Num3 => state.show_input = !state.show_input,
                        Keycode::Num4 => state.show_controls = !state.show_controls,
                        Keycode::Return | Keycode::Return2 => {
                            if !state.console_input.is_empty() {
                                exec_cmd(&mut state);
                            }
                        }
                        Keycode::Backspace => {
                            state.console_input.pop();
                        }
                        _ => {
                            if let Some(ch) = kc_to_char(k) {
                                state.console_input.push(ch);
                            }
                        }
                    }
                }
                Event::TextInput { text, .. } => {
                    for ch in text.chars() {
                        state.console_input.push(ch);
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    state.mouse_x = x;
                    state.mouse_y = y;
                }
                Event::MouseButtonDown { x, y, .. } => {
                    // Focus consola si click en mitad inferior
                    if y as i32 > WIN_H as i32 / 2 {
                        // already focused by default
                    }
                }
                _ => {}
            }
        }

        // RENDER
        canvas.set_draw_color(C_BG);
        let _ = canvas.clear();

        let pw = WIN_W as i32 / 2;
        let ph = WIN_H as i32 / 2;
        let mut col = 0i32;
        let mut row = 0i32;

        if state.show_screen {
            let px = col * pw;
            let py = row * ph;
            draw_panel(&mut canvas, px, py, pw, ph, "Screen");
            draw_anim(&mut canvas, px + 6, py + HEADER_H + 4,
                      pw - 12, ph - HEADER_H - 10, &state);
            col += 1; if col >= 2 { col = 0; row += 1; }
        }
        if state.show_console {
            let px = col * pw;
            let py = row * ph;
            draw_panel(&mut canvas, px, py, pw, ph, "Console");
            draw_console(&mut canvas, px + 4, py + HEADER_H + 2,
                        pw - 8, ph - HEADER_H - 6, &state);
            col += 1; if col >= 2 { col = 0; row += 1; }
        }
        if state.show_input {
            let px = col * pw;
            let py = row * ph;
            draw_panel(&mut canvas, px, py, pw, ph, "Input State");
            draw_input_state(&mut canvas, px + 4, py + HEADER_H + 2,
                           pw - 8, ph - HEADER_H - 6, &state);
            col += 1; if col >= 2 { col = 0; row += 1; }
        }
        if state.show_controls {
            let px = col * pw;
            let py = row * ph;
            draw_panel(&mut canvas, px, py, pw, ph, "Controls");
            draw_controls_panel(&mut canvas, px + 4, py + HEADER_H + 2,
                               pw - 8, ph - HEADER_H - 6);
            col += 1; if col >= 2 { col = 0; row += 1; }
        }

        // Status bar
        draw_status_bar(&mut canvas, &state);

        canvas.present();

        // Frame limit
        let elapsed = frame_start.elapsed().as_millis() as f64;
        if elapsed < frame_ms {
            std::thread::sleep(std::time::Duration::from_millis(
                (frame_ms - elapsed) as u64));
        }
    }

    unsafe { sdl2::sys::SDL_StopTextInput(); }
    println!("[DEMO PANEL VISUAL] Shutdown. Frames: {} | FPS: {:.1}",
             state.frame, state.fps);
}

// ============================================================================
// HELPERS
// ============================================================================

fn kc_to_char(k: Keycode) -> Option<char> {
    match k {
        Keycode::A => Some('a'), Keycode::B => Some('b'), Keycode::C => Some('c'),
        Keycode::D => Some('d'), Keycode::E => Some('e'), Keycode::F => Some('f'),
        Keycode::G => Some('g'), Keycode::H => Some('h'), Keycode::I => Some('i'),
        Keycode::J => Some('j'), Keycode::K => Some('k'), Keycode::L => Some('l'),
        Keycode::M => Some('m'), Keycode::N => Some('n'), Keycode::O => Some('o'),
        Keycode::P => Some('p'), Keycode::Q => Some('q'), Keycode::R => Some('r'),
        Keycode::S => Some('s'), Keycode::T => Some('t'), Keycode::U => Some('u'),
        Keycode::V => Some('v'), Keycode::W => Some('w'), Keycode::X => Some('x'),
        Keycode::Y => Some('y'), Keycode::Z => Some('z'),
        Keycode::Num1 => Some('1'), Keycode::Num2 => Some('2'), Keycode::Num3 => Some('3'),
        Keycode::Num4 => Some('4'), Keycode::Num5 => Some('5'), Keycode::Num6 => Some('6'),
        Keycode::Num7 => Some('7'), Keycode::Num8 => Some('8'), Keycode::Num9 => Some('9'),
        Keycode::Num0 => Some('0'), Keycode::Space => Some(' '),
        Keycode::Comma => Some(','), Keycode::Period => Some('.'),
        Keycode::Slash => Some('/'), Keycode::Minus => Some('-'),
        Keycode::Equals => Some('='), Keycode::Backslash => Some('\\'),
        _ => None,
    }
}

fn add_line(lines: &mut Vec<String>, text: &str) {
    lines.push(text.to_string());
    if lines.len() > CONSOLE_MAX_LINES {
        lines.remove(0);
    }
}

fn exec_cmd(state: &mut State) {
    let cmd = state.console_input.trim().to_string();
    state.console_input.clear();
    add_line(&mut state.console_lines, &format!("> {}", cmd));

    let output = match cmd.as_str() {
        "help" => "Comandos: help clear echo load exec debug version status".into(),
        "clear" => { state.console_lines.clear(); String::new() }
        "version" => "events-ry v0.1.0 | ry-dit v0.13.0".into(),
        "status" => format!("Frame:{} t:{:.1}s Scene:{} FPS:{:.1}",
            state.frame, state.t, state.scenes[state.scene], state.fps),
        s if s.starts_with("echo ") => s[5..].into(),
        s if s.starts_with("load ") => format!("Asset cargado: {}", &s[5..]),
        s if s.starts_with("exec ") => format!("Ejecutado: {}", &s[5..]),
        s if s.starts_with("debug ") => format!("Debug {}: OK", &s[6..]),
        _ if !cmd.is_empty() => format!("Comando desconocido: '{}'. help.", cmd),
        _ => String::new(),
    };
    if !output.is_empty() {
        for line in output.lines() {
            add_line(&mut state.console_lines, line);
        }
    }
}

// ============================================================================
// DRAW
// ============================================================================

fn draw_block(canvas: &mut Canvas<Window>, x: i32, y: i32, c: SdlColor) {
    canvas.set_draw_color(c);
    let _ = canvas.fill_rect(Rect::new(x, y, 5, CHAR_H as u32));
}

fn draw_text(canvas: &mut Canvas<Window>, x: i32, y: i32, text: &str, color: SdlColor) {
    for (i, _ch) in text.chars().enumerate().take(120) {
        draw_block(canvas, x + i as i32 * (CHAR_W + 1), y, color);
    }
}

fn draw_panel(canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32, title: &str) {
    canvas.set_draw_color(C_PANEL_BG);
    let _ = canvas.fill_rect(Rect::new(x, y, w as u32, h as u32));
    canvas.set_draw_color(C_BORDER);
    let _ = canvas.draw_rect(Rect::new(x, y, w as u32, h as u32));
    canvas.set_draw_color(C_HEADER_BG);
    let _ = canvas.fill_rect(Rect::new(x + 1, y + 1, (w - 2) as u32, (HEADER_H - 1) as u32));
    draw_text(canvas, x + 6, y + 5, title, C_WHITE);
}

fn draw_anim(canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32, state: &State) {
    let cx = x + w / 2;
    let cy = y + h / 2;

    match state.scene {
        0 => {
            for i in 0..5 {
                let offset = disney::follow_through(40.0, 1.5, 10.0, state.t - i as f64 * 0.1);
                let px = cx as f64 + offset;
                let py = cy as f64;
                let r = (12 - i * 2).max(2);
                canvas.set_draw_color(C_GREEN);
                let _ = canvas.fill_rect(Rect::new(px as i32 - r, py as i32 - r, (r * 2) as u32, (r * 2) as u32));
            }
        }
        1 => {
            let start = (cx as f64 - 60.0, cy as f64);
            let end = (cx as f64 + 60.0, cy as f64);
            for i in 0..8 {
                let curve = (i as f64 - 3.5) * 20.0;
                let (px, py) = disney::arc_path(start, end, curve, (state.t * 0.3 + i as f64 * 0.1) % 1.0);
                canvas.set_draw_color(C_YELLOW);
                let _ = canvas.fill_rect(Rect::new(px as i32 - 4, py as i32 - 4, 8u32, 8u32));
            }
        }
        2 => {
            let colors = vec!["#000000".into(), "#FFFFFF".into(), "#0000FF".into(), "#FFFF00".into()];
            let segs = illusions::rotating_snakes(cx as f64, cy as f64, 80.0, 16, state.t, &colors);
            for s in &segs {
                let sx = s.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let sy = s.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                canvas.set_draw_color(C_YELLOW);
                let _ = canvas.fill_rect(Rect::new(sx as i32 - 3, sy as i32 - 3, 6, 6));
            }
        }
        3 => {
            let layers = effects::neon_glow(cx as f64, cy as f64, 12.0, 5, 1.5, 0.8, "#FF00FF", state.t);
            for (i, l) in layers.iter().enumerate() {
                let lx = l.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let ly = l.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let lr = l.get("radius").and_then(|v| v.as_f64()).unwrap_or(10.0) as i32;
                canvas.set_draw_color(if i == 0 { C_GREEN } else { SdlColor::RGB(180, 40, 180) });
                let _ = canvas.fill_rect(Rect::new(lx as i32 - lr, ly as i32 - lr, (lr * 2) as u32, (lr * 2) as u32));
            }
        }
        4 => {
            let tri = vec![(0.0, -40.0), (35.0, 30.0), (-35.0, 30.0)];
            let sq = vec![(-30.0, -30.0), (30.0, -30.0), (30.0, 30.0), (-30.0, 30.0)];
            let mt = (state.t * 0.4).sin() * 0.5 + 0.5;
            let pts = effects::morph_shapes(&tri, &sq, mt, "ease_in_out");
            for (i, p) in pts.iter().enumerate() {
                let px = p.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let py = p.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                canvas.set_draw_color(C_GREEN);
                let _ = canvas.fill_rect(Rect::new(cx + px as i32 - 4, cy + py as i32 - 4, 8, 8));
                let j = (i + 1) % pts.len();
                let pj = &pts[j];
                let jx = pj.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let jy = pj.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                canvas.set_draw_color(SdlColor::RGB(50, 120, 50));
                let _ = canvas.draw_line(
                    Point::new(cx + px as i32, cy + py as i32),
                    Point::new(cx + jx as i32, cy + jy as i32),
                );
            }
        }
        5 => {
            let elems = science_anim::tusi_couple(cx as f64, cy as f64, 80.0, state.t);
            for e in &elems {
                let tp = e.get("type").and_then(|v| v.as_str()).unwrap_or("");
                let ex = e.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let ey = e.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let er = e.get("radius").and_then(|v| v.as_f64()).unwrap_or(5.0) as i32;
                match tp {
                    "large_circle" => {
                        canvas.set_draw_color(SdlColor::RGB(70, 70, 90));
                        let _ = canvas.draw_rect(Rect::new(ex as i32 - er, ey as i32 - er, (er * 2) as u32, (er * 2) as u32));
                    }
                    "small_circle" | "point" => {
                        canvas.set_draw_color(if tp == "point" { C_RED } else { C_BLUE });
                        let _ = canvas.fill_rect(Rect::new(ex as i32 - 4, ey as i32 - 4, 8, 8));
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }

    draw_text(canvas, x + 4, y + 2, &format!("[{}]", state.scene + 1), C_WHITE);
}

fn draw_console(canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32, state: &State) {
    let max_lines = h / LINE_H;
    let start_line = if state.console_lines.len() > max_lines as usize {
        state.console_lines.len() - max_lines as usize
    } else {
        0
    };

    let mut ly = y;
    for line in state.console_lines.iter().skip(start_line) {
        if ly > y + h - LINE_H - CHAR_H { break; }
        let color = if line.starts_with("> ") { C_YELLOW }
                    else if line.contains("desconocido") || line.contains("Error") { C_RED }
                    else if line.contains("cargado") || line.contains("OK") { C_GREEN }
                    else if line.contains("Ejecutado") { C_CYAN }
                    else { C_TEXT };
        draw_text(canvas, x, ly, line, color);
        ly += LINE_H;
    }

    // Input line
    if ly < y + h - CHAR_H {
        draw_text(canvas, x, ly, ">", C_GREEN);
        draw_text(canvas, x + CHAR_W + 2, ly, &state.console_input,
                  SdlColor::RGB(140, 200, 140));
        // Cursor blink
        if state.frame % 30 < 15 {
            let cx = x + CHAR_W + 2 + state.console_input.len() as i32 * (CHAR_W + 1);
            canvas.set_draw_color(C_GREEN);
            let _ = canvas.fill_rect(Rect::new(cx, ly, 5u32, CHAR_H as u32));
        }
    }
}

fn draw_input_state(canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, _h: i32, state: &State) {
    let mut ly = y;
    draw_text(canvas, x, ly, "Mouse:", C_TEXT);
    ly += LINE_H + 2;
    draw_text(canvas, x + 8, ly, &format!("({}, {})", state.mouse_x, state.mouse_y), C_CYAN);
    ly += LINE_H + 6;

    draw_text(canvas, x, ly, &format!("Keys: {}", state.keys_down.len()), C_TEXT);
    ly += LINE_H + 2;

    for key in state.keys_down.iter().take(10) {
        draw_text(canvas, x + 8, ly, key, C_YELLOW);
        ly += LINE_H;
    }

    ly += 4;
    draw_text(canvas, x, ly, &format!("FPS: {:.1}", state.fps), C_GREEN);
    ly += LINE_H + 2;
    draw_text(canvas, x, ly, &format!("Frame: {}", state.frame), C_WHITE);
}

fn draw_controls_panel(canvas: &mut Canvas<Window>, x: i32, y: i32, _w: i32, _h: i32) {
    let controls = [
        ("ESC", "Salir"),
        ("SPACE", "Cambiar escena"),
        ("1", "Toggle Screen"),
        ("2", "Toggle Console"),
        ("3", "Toggle Input"),
        ("4", "Toggle Controls"),
        ("ENTER", "Ejecutar comando"),
        ("BACKSP", "Borrar texto"),
    ];
    let mut ly = y;
    draw_text(canvas, x, ly, "Key Bindings:", C_TEXT);
    ly += LINE_H + 4;
    for (key, action) in &controls {
        draw_text(canvas, x + 4, ly, *key, C_YELLOW);
        draw_text(canvas, x + 80, ly, *action, C_TEXT);
        ly += LINE_H;
    }
}

fn draw_status_bar(canvas: &mut Canvas<Window>, state: &State) {
    canvas.set_draw_color(C_HEADER_BG);
    let _ = canvas.fill_rect(Rect::new(0, 0, WIN_W, 20u32));
    draw_text(canvas, 4, 4, &format!("ry-dit v0.13.0 | FPS: {:.1}", state.fps), C_GREEN);
    draw_text(canvas, 220, 4, state.scenes[state.scene], C_CYAN);
    draw_text(canvas, 520, 4, "ESC=Exit SPACE=Scene 1-4=Panels", C_YELLOW);
}

// Need WHITE constant
const C_WHITE: SdlColor = SdlColor::RGB(240, 240, 250);
