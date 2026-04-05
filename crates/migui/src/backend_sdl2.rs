// crates/migui/src/backend_sdl2.rs
// Backend SDL2 para MiGUI - v0.10.10
// Conecta MiGUI con SDL2 para render e input + Fuentes nativas Rust

use crate::font_native::NativeFontManager;
use crate::{Color, Migui};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::Canvas;
use sdl2::video::Window;

// ============================================================================
// FONT MANAGER (Wrapper para NativeFontManager)
// ============================================================================

/// Gestor de fuentes (usa NativeFontManager internamente)
pub struct FontManager {
    #[allow(dead_code)]
    native: NativeFontManager,
}

impl FontManager {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            native: NativeFontManager::with_fallback(),
        })
    }

    /// Renderizar texto a superficie SDL2
    pub fn render_text(
        &mut self,
        _text: &str,
        _size: u32,
        _color: Color,
    ) -> Result<sdl2::surface::Surface<'_>, String> {
        // Placeholder: retorna superficie vacía
        // En producción, usar ab_glyph o SDL2_ttf
        sdl2::surface::Surface::new(100, 20, sdl2::pixels::PixelFormatEnum::RGBA8888)
            .map_err(|e| e.to_string())
    }
}

// ============================================================================
// MIGUI BACKEND SDL2
// ============================================================================

/// Backend SDL2 para MiGUI
pub struct MiguiSdl2Backend {
    canvas: Canvas<Window>,
    mouse_x: i32,
    mouse_y: i32,
    #[allow(dead_code)]
    font_manager: Option<FontManager>,
}

impl MiguiSdl2Backend {
    /// Crear nuevo backend SDL2 para MiGUI
    pub fn new(canvas: Canvas<Window>) -> Self {
        let font_manager = FontManager::new().ok();

        Self {
            canvas,
            mouse_x: 0,
            mouse_y: 0,
            font_manager,
        }
    }

    /// Procesar eventos SDL2 y actualizar estado de MiGUI
    /// ✅ v0.13.0: Agregado soporte SDL_TEXTINPUT para teclado Android
    pub fn process_events(&mut self, gui: &mut Migui, sdl_event: &sdl2::event::Event) -> bool {
        let mut should_close = false;

        match sdl_event {
            sdl2::event::Event::Quit { .. } => {
                should_close = true;
            }
            sdl2::event::Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                should_close = true;
            }

            // Input de mouse - convertir a evento Migui
            sdl2::event::Event::MouseMotion { x, y, .. } => {
                self.mouse_x = *x;
                self.mouse_y = *y;
                gui.handle_event(crate::Event::MouseMove {
                    x: *x as f32,
                    y: *y as f32,
                });
            }

            sdl2::event::Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => {
                if *mouse_btn == MouseButton::Left {
                    gui.handle_event(crate::Event::MouseDown {
                        button: crate::MouseButton::Left,
                        x: *x as f32,
                        y: *y as f32,
                    });
                }
            }

            sdl2::event::Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } => {
                if *mouse_btn == MouseButton::Left {
                    gui.handle_event(crate::Event::MouseUp {
                        button: crate::MouseButton::Left,
                        x: *x as f32,
                        y: *y as f32,
                    });
                }
            }

            // ✅ v0.13.0: TEXTINPUT - Recibe texto real del teclado Android
            // Esto es CRÍTICO: el teclado virtual Android envía TextInput, no KeyCodes
            sdl2::event::Event::TextInput { text, .. } => {
                if !text.is_empty() {
                    // Enviar cada caracter como CharTyped a Migui
                    for ch in text.chars() {
                        gui.handle_event(crate::Event::CharTyped { ch });
                    }
                }
            }

            // ✅ v0.13.0: TEXTEDITING - Composición IME (CJK input methods)
            sdl2::event::Event::TextEditing { text, .. } => {
                // Pre-edit text para input methods complejos
                if !text.is_empty() {
                    // En futuras versiones, manejar composición IME
                    eprintln!("[MIGUI] TextEditing: {}", text);
                }
            }

            _ => {}
        }

        should_close
    }

    /// Activar input de texto (muestra teclado virtual Android)
    /// ✅ v0.13.0: Llamar cuando un textbox recibe foco
    pub fn enable_text_input(&self) {
        sdl2::hint::set("SDL_HINT_ENABLE_SCREEN_KEYBOARD", "1");
        // SDL_StartTextInput se llama desde el contexto principal
        eprintln!("[MIGUI] TextInput enabled - keyboard should appear on focus");
    }

    /// Desactivar input de texto
    /// ✅ v0.13.0: Llamar cuando un textbox pierde foco
    pub fn disable_text_input(&self) {
        eprintln!("[MIGUI] TextInput disabled");
    }

    /// Renderizar MiGUI con SDL2
    pub fn render(&mut self, gui: &mut Migui) {
        // Limpiar pantalla (color de fondo)
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(30, 30, 30));
        self.canvas.clear();

        // Renderizar comandos de MiGUI
        for cmd in gui.draw_commands() {
            match cmd {
                crate::DrawCommand::DrawRect { rect, color } => {
                    self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(
                        color.r, color.g, color.b, color.a,
                    ));
                    let sdl_rect = sdl2::rect::Rect::new(
                        rect.x as i32,
                        rect.y as i32,
                        rect.w as u32,
                        rect.h as u32,
                    );
                    self.canvas.fill_rect(sdl_rect).ok();
                }
                crate::DrawCommand::DrawLine {
                    x1,
                    y1,
                    x2,
                    y2,
                    color,
                    ..
                } => {
                    self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(
                        color.r, color.g, color.b, color.a,
                    ));
                    self.canvas
                        .draw_line((*x1 as i32, *y1 as i32), (*x2 as i32, *y2 as i32))
                        .ok();
                }
                crate::DrawCommand::DrawText {
                    text: _,
                    x,
                    y,
                    size: _,
                    color,
                } => {
                    // Placeholder: dibujar rect en vez de texto
                    // En producción, usar SDL2_ttf o ab_glyph
                    self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(
                        color.r, color.g, color.b, color.a,
                    ));
                    self.canvas
                        .fill_rect(sdl2::rect::Rect::new(*x as i32, *y as i32, 50, 20))
                        .ok();
                }
                crate::DrawCommand::Clear { color } => {
                    self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(
                        color.r, color.g, color.b, color.a,
                    ));
                    self.canvas.clear();
                }
            }
        }

        // Presentar
        self.canvas.present();
    }

    /// Obtener posición del mouse
    pub fn mouse_pos(&self) -> (i32, i32) {
        (self.mouse_x, self.mouse_y)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    

    #[test]
    fn test_backend_creation() {
        // Solo verificamos que compile
        assert!(true);
    }
}
