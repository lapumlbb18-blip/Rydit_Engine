// crates/migui/src/backend_sdl2.rs
// Backend SDL2 para MiGUI - v0.13.0
// Conecta MiGUI con ry-backend (Sdl2Core) para render + input + TTF profesional

#[cfg(feature = "sdl2")]
use ry_backend::sdl2_core::{Sdl2Core, MouseEvent, TtfFont};
#[cfg(feature = "sdl2")]
use ry_backend::sdl2;

use crate::{Color, DrawCommand, Event, Key, Migui, MouseButton};

#[cfg(feature = "sdl2")]
use ry_backend::sdl2::keyboard::Keycode;

// ============================================================================
// MIGUI BACKEND CON RY-BACKEND
// ============================================================================

/// Backend SDL2 para MiGUI usando ry-backend Sdl2Core
#[cfg(feature = "sdl2")]
pub struct MiguiSdl2Backend {
    core: Sdl2Core,

}

#[cfg(feature = "sdl2")]
impl MiguiSdl2Backend {
    /// Crear nuevo backend SDL2 para MiGUI
    pub fn new(title: &str, width: i32, height: i32) -> Result<Self, String> {
        let core = Sdl2Core::new(title, width, height)?;
        
        println!("[MIGUI] Backend SDL2 creado con ry-backend Sdl2Core");
        Ok(Self { core })
    }

    /// Procesar eventos SDL2 y actualizar estado de MiGUI
    pub fn process_events(&mut self, gui: &mut Migui) -> bool {
        let mut should_close = false;

        for event in self.core.event_pump.poll_iter() {
            match &event {
                ry_backend::sdl2::event::Event::Quit { .. } => {
                    should_close = true;
                }
                ry_backend::sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    repeat: false,
                    ..
                } => {
                    should_close = true;
                }
                ry_backend::sdl2::event::Event::KeyDown {
                    keycode: Some(kc),
                    repeat: false,
                    ..
                } => {
                    if let Some(key) = keycode_to_migui_key(*kc) {
                        gui.handle_event(Event::KeyDown { key });
                    }
                }
                ry_backend::sdl2::event::Event::KeyUp { keycode: Some(kc), .. } => {
                    if let Some(key) = keycode_to_migui_key(*kc) {
                        gui.handle_event(Event::KeyUp { key });
                    }
                }
                ry_backend::sdl2::event::Event::TextInput { text, .. } => {
                    for ch in text.chars() {
                        gui.handle_event(Event::CharTyped { ch });
                    }
                }
                ry_backend::sdl2::event::Event::MouseMotion { x, y, .. } => {
                    gui.handle_event(Event::MouseMove { x: *x as f32, y: *y as f32 });
                }
                ry_backend::sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                    gui.handle_event(Event::MouseDown {
                        button: MouseButton::Left,
                        x: *x as f32,
                        y: *y as f32,
                    });
                }
                ry_backend::sdl2::event::Event::MouseButtonUp { x, y, .. } => {
                    gui.handle_event(Event::MouseUp {
                        button: MouseButton::Left,
                        x: *x as f32,
                        y: *y as f32,
                    });
                }
                ry_backend::sdl2::event::Event::MouseWheel { x, y, .. } => {
                    // Scroll events
                    gui.handle_event(Event::MouseMove {
                        x: self.core.mouse.x as f32,
                        y: self.core.mouse.y as f32,
                    });
                }
                ry_backend::sdl2::event::Event::FingerDown { x, y, .. } => {
                    gui.handle_event(Event::MouseMove { x: *x * 800.0, y: *y * 600.0 });
                    gui.handle_event(Event::MouseDown {
                        button: MouseButton::Left,
                        x: *x * 800.0,
                        y: *y * 600.0,
                    });
                }
                ry_backend::sdl2::event::Event::FingerMotion { x, y, .. } => {
                    gui.handle_event(Event::MouseMove { x: *x * 800.0, y: *y * 600.0 });
                }
                ry_backend::sdl2::event::Event::FingerUp { x, y, .. } => {
                    gui.handle_event(Event::MouseUp {
                        button: MouseButton::Left,
                        x: *x * 800.0,
                        y: *y * 600.0,
                    });
                }
                ry_backend::sdl2::event::Event::Window { win_event, .. } => {
                    if let sdl2::event::WindowEvent::SizeChanged(w, h) = win_event {
                        self.core.width = *w;
                        self.core.height = *h;
                    }
                }
                _ => {}
            }

            // Procesar eventos de mouse del Sdl2Core
            if let Some(me) = self.core.mouse.process_event(&event) {
                match me {
                    MouseEvent::LeftDoubleClick { x, y } => {
                        // Doble click = acción especial
                        gui.handle_event(Event::MouseDown {
                            button: MouseButton::Left,
                            x: x as f32,
                            y: y as f32,
                        });
                        gui.handle_event(Event::MouseUp {
                            button: MouseButton::Left,
                            x: x as f32,
                            y: y as f32,
                        });
                        gui.handle_event(Event::MouseDown {
                            button: MouseButton::Left,
                            x: x as f32,
                            y: y as f32,
                        });
                        gui.handle_event(Event::MouseUp {
                            button: MouseButton::Left,
                            x: x as f32,
                            y: y as f32,
                        });
                    }
                    MouseEvent::RightClick { x, y } => {
                        // Click derecho = menú contextual
                        gui.handle_event(Event::MouseDown {
                            button: MouseButton::Right,
                            x: x as f32,
                            y: y as f32,
                        });
                    }
                    MouseEvent::Wheel { x, y, .. } => {
                        // Scroll = navegación
                        gui.handle_event(Event::MouseMove {
                            x: self.core.mouse.x as f32,
                            y: self.core.mouse.y as f32,
                        });
                    }
                    _ => {}
                }
            }
        }

        should_close
    }

    /// Renderizar MiGUI
    pub fn render(&mut self, gui: &mut Migui) {
        // Limpiar
        self.core.canvas.set_draw_color(ry_backend::sdl2::pixels::Color::RGB(25, 25, 35));
        let _ = self.core.canvas.clear();

        // Renderizar comandos de MiGUI
        for cmd in gui.draw_commands() {
            match cmd {
                DrawCommand::DrawRect { rect, color } => {
                    self.core.canvas.set_draw_color(ry_backend::sdl2::pixels::Color::RGBA(
                        color.r, color.g, color.b, color.a,
                    ));
                    let _ = self.core.canvas.fill_rect(ry_backend::sdl2::rect::Rect::new(
                        rect.x as i32, rect.y as i32,
                        rect.w.max(1.0) as u32, rect.h.max(1.0) as u32,
                    ));
                }
                DrawCommand::DrawLine { x1, y1, x2, y2, color, thickness: _ } => {
                    self.core.canvas.set_draw_color(ry_backend::sdl2::pixels::Color::RGBA(
                        color.r, color.g, color.b, color.a,
                    ));
                    let _ = self.core.canvas.draw_line(
                        (*x1 as i32, *y1 as i32), (*x2 as i32, *y2 as i32),
                    );
                }
                DrawCommand::DrawText { text, x, y, size, color } => {
                    // Texto real con TTF profesional (ry-backend)
                    if let Some(ref font) = self.core.font {
                        let sdl_color = ry_backend::sdl2::pixels::Color::RGBA(color.r, color.g, color.b, color.a);
                        if let Some(tex) = font.render_text(text, sdl_color) {
                            let _ = self.core.canvas.copy(
                                &tex.texture,
                                None,
                                ry_backend::sdl2::rect::Rect::new(*x as i32, *y as i32, tex.width, tex.height),
                            );
                        }
                    }
                }
                DrawCommand::Clear { color } => {
                    self.core.canvas.set_draw_color(ry_backend::sdl2::pixels::Color::RGBA(
                        color.r, color.g, color.b, color.a,
                    ));
                    let _ = self.core.canvas.clear();
                }
                DrawCommand::DrawViewport3D { .. } => {
                    // El renderizado de viewports 3D se delega a ry-gfx (Patrón Fusional)
                }
            }
        }

        self.core.canvas.present();
    }

    /// Obtener posición del mouse
    pub fn mouse_pos(&self) -> (i32, i32) {
        (self.core.mouse.x, self.core.mouse.y)
    }

    /// Acceder al Sdl2Core para operaciones avanzadas
    pub fn core(&self) -> &Sdl2Core {
        &self.core
    }

    /// Acceder al Sdl2Core mutable
    pub fn core_mut(&mut self) -> &mut Sdl2Core {
        &mut self.core
    }
}

/// Convertir Keycode SDL2 → migui Key
#[cfg(feature = "sdl2")]
fn keycode_to_migui_key(kc: Keycode) -> Option<Key> {
    match kc {
        Keycode::Escape => Some(Key::Escape),
        Keycode::Return | Keycode::Return2 => Some(Key::Enter),
        Keycode::Backspace => Some(Key::Backspace),
        Keycode::Up => Some(Key::ArrowUp),
        Keycode::Down => Some(Key::ArrowDown),
        Keycode::Left => Some(Key::ArrowLeft),
        Keycode::Right => Some(Key::ArrowRight),
        Keycode::A => Some(Key::A), Keycode::B => Some(Key::B),
        Keycode::C => Some(Key::C), Keycode::D => Some(Key::D),
        Keycode::E => Some(Key::E), Keycode::F => Some(Key::F),
        Keycode::G => Some(Key::G), Keycode::H => Some(Key::H),
        Keycode::I => Some(Key::I), Keycode::J => Some(Key::J),
        Keycode::K => Some(Key::K), Keycode::L => Some(Key::L),
        Keycode::M => Some(Key::M), Keycode::N => Some(Key::N),
        Keycode::O => Some(Key::O), Keycode::P => Some(Key::P),
        Keycode::Q => Some(Key::Q), Keycode::R => Some(Key::R),
        Keycode::S => Some(Key::S), Keycode::T => Some(Key::T),
        Keycode::U => Some(Key::U), Keycode::V => Some(Key::V),
        Keycode::W => Some(Key::W), Keycode::X => Some(Key::X),
        Keycode::Y => Some(Key::Y), Keycode::Z => Some(Key::Z),
        _ => None,
    }
}
