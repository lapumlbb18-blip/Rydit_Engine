// crates/rydit-gfx/src/toolkit/widgets/panel.rs
// Widget Panel - Contenedor/ventana

use crate::backend_sdl2::Sdl2Backend;
use crate::toolkit::theme::Theme;
use sdl2::rect::Rect;

/// Panel - Contenedor/ventana
pub struct Panel {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    title: Option<String>,
}

impl Panel {
    /// Crear nuevo panel
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            height,
            title: None,
        }
    }

    /// Establecer posición
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Establecer título
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Renderizar panel
    pub fn render(&self, backend: &mut Sdl2Backend, theme: &Theme) {
        // Dibujar fondo
        backend.canvas.set_draw_color(theme.panel_bg);
        let rect = Rect::new(self.x, self.y, self.width as u32, self.height as u32);
        let _ = backend.canvas.fill_rect(rect);

        // Dibujar borde
        backend.canvas.set_draw_color(theme.panel_border);
        let _ = backend.canvas.draw_rect(rect);

        // Dibujar título (si existe)
        if let Some(ref title) = self.title {
            // Barra de título
            let title_height = 24;
            backend.canvas.set_draw_color(theme.title_bar_bg);
            let title_rect = Rect::new(self.x, self.y, self.width as u32, title_height as u32);
            let _ = backend.canvas.fill_rect(title_rect);

            // Texto del título (placeholder)
            let text_width = title.len() as i32 * 8;
            let text_x = self.x + (self.width - text_width) / 2;
            let text_y = self.y + 4;

            backend.canvas.set_draw_color(theme.title_text);
            let text_rect = Rect::new(text_x, text_y, text_width as u32, 16);
            let _ = backend.canvas.fill_rect(text_rect);
        }
    }
}
