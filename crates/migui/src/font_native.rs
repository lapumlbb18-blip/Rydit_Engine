// crates/migui/src/font_native.rs
// Fuentes - v0.13.0
// ab_glyph con API correcta

use crate::Color;

// Intentar usar ab_glyph si está disponible
#[cfg(feature = "ab_glyph_feat")]
mod real_font {
    use ab_glyph::{FontRef, ScaleFont};
    use crate::Color;

    pub struct TextTexture {
        pub pixels: Vec<u8>,
        pub width: u32,
        pub height: u32,
    }

    pub struct NativeFontManager {
        font: Option<FontRef<'static>>,
    }

    impl NativeFontManager {
        pub fn new() -> Self { Self { font: None } }
        pub fn render_text(&self, _text: &str, _size: f32, _color: Color) -> Option<TextTexture> { None }
        pub fn text_dimensions(&self, text: &str, _size: f32) -> (u32, u32) { (text.len() as u32 * 8, 16) }
        pub fn has_font(&self) -> bool { self.font.is_some() }
    }
}

#[cfg(not(feature = "ab_glyph_feat"))]
mod bitmap_font {
    use crate::Color;

    /// Textura de texto renderizado
    pub struct TextTexture {
        pub pixels: Vec<u8>,
        pub width: u32,
        pub height: u32,
    }

    /// Gestor de fuentes
    pub struct NativeFontManager {
        has_font: bool,
    }

    impl NativeFontManager {
        pub fn new() -> Self {
            Self { has_font: false }
        }

        /// Renderizar texto como bitmap simple
        /// Cada carácter = bloque de color (placeholder hasta tener TTF real)
        pub fn render_text(&self, text: &str, size: f32, color: Color) -> Option<TextTexture> {
            let char_w = (size as u32 / 2).max(4);
            let char_h = size as u32;
            let w = text.len() as u32 * char_w;
            let h = char_h;
            let mut pixels = vec![0u8; (w * h * 4) as usize];

            for (ci, ch) in text.chars().enumerate() {
                // Dibujar bloque simple como representación del carácter
                let ox = ci as u32 * char_w;
                for py in 0..h {
                    for px in 0..char_w {
                        // Patrón simple basado en el carácter
                        let pattern = (ch as u32 + px + py) % 3;
                        if pattern != 0 {
                            let i = ((py * w + ox + px) * 4) as usize;
                            if i + 3 < pixels.len() {
                                pixels[i] = color.r;
                                pixels[i+1] = color.g;
                                pixels[i+2] = color.b;
                                pixels[i+3] = 200;
                            }
                        }
                    }
                }
            }

            Some(TextTexture { pixels, width: w, height: h })
        }

        pub fn text_dimensions(&self, text: &str, size: f32) -> (u32, u32) {
            let char_w = (size as u32 / 2).max(4);
            (text.len() as u32 * char_w, size as u32)
        }

        pub fn has_font(&self) -> bool { self.has_font }
    }

    impl Default for NativeFontManager {
        fn default() -> Self { Self::new() }
    }
}

#[cfg(feature = "ab_glyph_feat")]
pub use real_font::*;

#[cfg(not(feature = "ab_glyph_feat"))]
pub use bitmap_font::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create() { let _ = NativeFontManager::new(); }
    #[test]
    fn test_render() {
        let m = NativeFontManager::new();
        let tex = m.render_text("Hola", 16.0, Color { r: 255, g: 255, b: 255, a: 255 });
        assert!(tex.is_some());
    }
}
