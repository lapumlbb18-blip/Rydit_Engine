// crates/rydit-gfx/src/toolkit/widgets/label.rs
// Widget Label - Texto estático

use crate::backend_sdl2::Sdl2Backend;
use crate::toolkit::theme::Theme;
use sdl2::rect::Rect;

/// Label - Texto estático
pub struct Label {
    text: String,
    x: i32,
    y: i32,
    font_size: u16,
    color: Option<(u8, u8, u8)>,
}

impl Label {
    /// Crear nuevo label
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            x: 0,
            y: 0,
            font_size: 16,
            color: None,
        }
    }

    /// Establecer posición
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Establecer tamaño de fuente
    pub fn size(mut self, size: u16) -> Self {
        self.font_size = size;
        self
    }

    /// Establecer color (R, G, B)
    pub fn color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = Some((r, g, b));
        self
    }

    /// Renderizar label
    pub fn render(&self, backend: &mut Sdl2Backend, _theme: &Theme) {
        let (r, g, b) = self.color.unwrap_or((255, 255, 255));
        backend.draw_text(&self.text, self.x, self.y, self.font_size, r, g, b);
    }

    /// Obtener bounds
    pub fn bounds(&self) -> Rect {
        let width = self.text.len() as i32 * (self.font_size as i32 / 2);
        Rect::new(self.x, self.y, width as u32, self.font_size as u32)
    }
}
