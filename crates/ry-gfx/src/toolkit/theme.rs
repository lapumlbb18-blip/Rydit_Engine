// crates/rydit-gfx/src/toolkit/theme.rs
// Theme - Estilos y colores para UI

use sdl2::pixels::Color;

/// Tema de colores para UI
pub struct Theme {
    // Colores de botones
    pub button_normal: Color,
    pub button_hover: Color,
    pub button_border: Color,

    // Colores de paneles
    pub panel_bg: Color,
    pub panel_border: Color,
    pub title_bar_bg: Color,
    pub title_text: Color,

    // Colores de texto
    pub text_color: Color,
}

impl Theme {
    /// Tema oscuro (default)
    pub fn dark() -> Self {
        Self {
            button_normal: Color::RGB(60, 60, 80),
            button_hover: Color::RGB(80, 80, 100),
            button_border: Color::RGB(100, 100, 120),
            panel_bg: Color::RGB(40, 40, 50),
            panel_border: Color::RGB(80, 80, 90),
            title_bar_bg: Color::RGB(50, 50, 70),
            title_text: Color::RGB(255, 255, 255),
            text_color: Color::RGB(255, 255, 255),
        }
    }

    /// Tema claro
    pub fn light() -> Self {
        Self {
            button_normal: Color::RGB(200, 200, 220),
            button_hover: Color::RGB(220, 220, 240),
            button_border: Color::RGB(180, 180, 200),
            panel_bg: Color::RGB(240, 240, 250),
            panel_border: Color::RGB(200, 200, 210),
            title_bar_bg: Color::RGB(220, 220, 230),
            title_text: Color::RGB(0, 0, 0),
            text_color: Color::RGB(0, 0, 0),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
