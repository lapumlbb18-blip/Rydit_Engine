// crates/rydit-gfx/src/font_sdl2.rs
// Fuentes SDL2 para RyDit - v0.10.8
// ⚠️ PENDIENTE: Linking de SDL2_ttf

/// Gestor de fuentes con SDL2_ttf
pub struct FontManager {
    initialized: bool,
}

impl FontManager {
    pub fn new() -> Self {
        Self { initialized: false }
    }

    /// Inicializar SDL2_ttf
    pub fn init(&mut self) -> Result<(), String> {
        // ⚠️ PENDIENTE: El linking de SDL2_ttf es complejo
        // Esto se implementará en v0.10.8
        Err("SDL2_ttf linking pendiente - v0.10.8".to_string())
    }

    /// Cargar fuente desde archivo
    pub fn load_font(&mut self, _path: &str, _size: u16) -> Result<(), String> {
        Err("SDL2_ttf linking pendiente - v0.10.8".to_string())
    }

    /// Renderizar texto a superficie
    pub fn render_text(&self, _text: &str, _color: (u8, u8, u8)) -> Result<(), String> {
        Err("SDL2_ttf linking pendiente - v0.10.8".to_string())
    }

    /// Verificar si está inicializado
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Default for FontManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_manager_new() {
        let font = FontManager::new();
        assert!(!font.is_initialized());
    }
}
