// crates/rydit-gfx/src/toolkit/widgets/button.rs
// Widget Button - Botón clicable

use crate::backend_sdl2::Sdl2Backend;
use crate::toolkit::theme::Theme;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

/// Botón clicable
pub struct Button {
    text: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    hovered: bool,
}

impl Button {
    /// Crear nuevo botón
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            x: 0,
            y: 0,
            width: 150,
            height: 40,
            hovered: false,
        }
    }

    /// Establecer posición
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Establecer tamaño
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Renderizar botón
    pub fn render(&self, backend: &mut Sdl2Backend, theme: &Theme) {
        // Color según estado
        let color = if self.hovered {
            theme.button_hover
        } else {
            theme.button_normal
        };

        // Dibujar fondo
        backend.canvas.set_draw_color(color);
        let rect = Rect::new(self.x, self.y, self.width as u32, self.height as u32);
        let _ = backend.canvas.fill_rect(rect);

        // Dibujar borde
        backend.canvas.set_draw_color(theme.button_border);
        let _ = backend.canvas.draw_rect(rect);

        // Dibujar texto (placeholder - rectángulo blanco)
        let text_width = self.text.len() as i32 * 8;
        let text_x = self.x + (self.width - text_width) / 2;
        let text_y = self.y + (self.height - 16) / 2;

        backend.canvas.set_draw_color(Color::RGB(255, 255, 255));
        let text_rect = Rect::new(text_x, text_y, text_width as u32, 16);
        let _ = backend.canvas.fill_rect(text_rect);
    }

    /// Verificar si el click está en el botón
    pub fn is_clicked(&self, x: i32, y: i32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}
