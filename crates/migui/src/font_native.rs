// crates/migui/src/font_native.rs
// Fuentes Nativas en Rust - v0.10.10
// Usa ab_glyph (100% Rust, sin FFI)

use ab_glyph::FontRef;
use std::collections::HashMap;

// ============================================================================
// FONT MANAGER NATIVO
// ============================================================================

/// Gestor de fuentes nativas en Rust
pub struct NativeFontManager {
    #[allow(dead_code)]
    fonts: HashMap<u32, FontData>,
    #[allow(dead_code)]
    default_font: &'static [u8],
}

struct FontData {
    #[allow(dead_code)]
    font: FontRef<'static>,
}

impl NativeFontManager {
    /// Crear gestor de fuentes nativas
    pub fn new() -> Result<Self, String> {
        // Fuente por defecto embebida (usamos una fuente del sistema o fallback)
        // En producción, podríamos embeber una fuente .ttf en el binario
        Ok(Self {
            fonts: HashMap::new(),
            default_font: include_bytes!(concat!(env!("OUT_DIR"), "/font.ttf")),
        })
    }

    /// Crear con fuente por defecto simple (bitmap fallback)
    pub fn with_fallback() -> Self {
        Self {
            fonts: HashMap::new(),
            default_font: &[],
        }
    }

    /// Cargar fuente desde archivo TTF
    pub fn load_font(&mut self, path: &str, _size: u32) -> Result<(), String> {
        use std::fs;

        let font_data =
            fs::read(path).map_err(|e| format!("Error leyendo fuente '{}': {}", path, e))?;

        // Necesitamos que los datos vivan lo suficiente
        // En producción, usaríamos Arc o similar
        // Por ahora, solo verificamos que se puede cargar
        let _font = FontRef::try_from_slice(&font_data)
            .map_err(|e| format!("Error parseando fuente: {}", e))?;

        // Guardamos los datos (en producción usaríamos Arc)
        // Por simplicidad, solo verificamos que funciona
        Ok(())
    }

    /// Renderizar texto a imagen (RGBA)
    pub fn render_text(
        &mut self,
        _text: &str,
        _size: u32,
        _color: (u8, u8, u8, u8),
    ) -> Result<Vec<u8>, String> {
        // Placeholder: retorna imagen vacía
        // En producción, usar ab_glyph para renderizar
        Ok(vec![0u8; 100 * 20 * 4]) // 100x20 pixels RGBA
    }

    /// Obtener dimensiones de texto
    pub fn text_dimensions(&self, text: &str, _size: u32) -> (u32, u32) {
        // Aproximación simple: 10px por carácter, 20px de alto
        (text.len() as u32 * 10, 20)
    }
}

impl Default for NativeFontManager {
    fn default() -> Self {
        Self::with_fallback()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_manager_creation() {
        let mgr = NativeFontManager::with_fallback();
        assert_eq!(mgr.fonts.len(), 0);
    }

    #[test]
    fn test_text_dimensions() {
        let mgr = NativeFontManager::with_fallback();
        let (w, h) = mgr.text_dimensions("Hola", 16);
        assert!(w > 0);
        assert!(h > 0);
    }
}
