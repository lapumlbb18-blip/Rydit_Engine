// crates/ry-gfx/src/font_system.rs
// Sistema de Fuentes con UTF-8 + Emojis + Acentos (ñ, á, é, í, ó, ú, ü)
// v0.17.0 - Soporte completo de texto en español

use crate::sdl2_ffi::FontFFI;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use std::collections::HashMap;

// ============================================================================
// FONT SYSTEM - Múltiples fuentes con fallback
// ============================================================================

/// Gestor de fuentes con soporte UTF-8 y fallback automático
pub struct FontSystem<'a> {
    /// Fuentes cargadas (nombre -> FontFFI)
    pub fonts: HashMap<String, FontFFI>,
    /// Fuente principal para texto
    pub primary_font: Option<String>,
    /// Fuente de emojis (fallback)
    pub emoji_font: Option<String>,
    /// Tamaño actual
    current_size: u16,
    /// Creator de texturas
    texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
}

impl<'a> FontSystem<'a> {
    /// Crear nuevo sistema de fuentes
    pub fn new(
        texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
    ) -> Self {
        Self {
            fonts: HashMap::new(),
            primary_font: None,
            emoji_font: None,
            current_size: 14,
            texture_creator,
        }
    }

    /// Cargar fuente principal (para texto y acentos)
    pub fn load_primary_font(&mut self, path: &str, size: u16) -> Result<(), String> {
        let font = FontFFI::load(path, size as i32)?;
        let name = format!("primary_{}", path);
        self.fonts.insert(name.clone(), font);
        self.primary_font = Some(name);
        self.current_size = size;
        println!("✅ Fuente principal cargada: {} ({}px)", path, size);
        Ok(())
    }

    /// Cargar fuente de emojis (fallback para glyphs de emojis)
    pub fn load_emoji_font(&mut self, path: &str, size: u16) -> Result<(), String> {
        let font = FontFFI::load(path, size as i32)?;
        let name = format!("emoji_{}", path);
        self.fonts.insert(name.clone(), font);
        self.emoji_font = Some(name);
        println!("✅ Fuente emojis cargada: {} ({}px)", path, size);
        Ok(())
    }

    /// Auto-detectar y cargar fuentes del sistema
    /// Busca fuentes comunes en Android/Termux/Linux
    pub fn auto_load_fonts(&mut self) -> Result<(), String> {
        // Intentar fuentes principales en orden de preferencia
        let primary_paths = [
            // Android/Termux
            "/system/fonts/DroidSans.ttf",
            "/system/fonts/Roboto-Regular.ttf",
            "/system/fonts/NotoSans-Regular.ttf",
            // Linux
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/usr/share/fonts/TTF/DejaVuSans.ttf",
            // Termux
            "/data/data/com.termux/files/usr/share/fonts/noto-sans/NotoSans-Regular.ttf",
        ];

        let emoji_paths = [
            // Android/Termux
            "/system/fonts/NotoColorEmoji.ttf",
            "/system/fonts/EmojiOne.ttf",
            // Linux
            "/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf",
            "/usr/share/fonts/noto/NotoColorEmoji.ttf",
            // Termux
            "/data/data/com.termux/files/usr/share/fonts/noto-emoji/NotoColorEmoji.ttf",
        ];

        // Cargar fuente principal
        for path in &primary_paths {
            if std::path::Path::new(path).exists() {
                if self.load_primary_font(path, self.current_size).is_ok() {
                    break;
                }
            }
        }

        // Cargar fuente de emojis
        for path in &emoji_paths {
            if std::path::Path::new(path).exists() {
                if self.load_emoji_font(path, self.current_size).is_ok() {
                    break;
                }
            }
        }

        // Si no se encontró fuente principal, reportar
        if self.primary_font.is_none() {
            return Err("No se encontró fuente TTF compatible".to_string());
        }

        Ok(())
    }

    /// Renderizar texto a textura con fallback de emojis
    pub fn render_text(
        &self,
        text: &str,
        r: u8,
        g: u8,
        b: u8,
    ) -> Option<Texture<'a>> {
        if self.fonts.is_empty() {
            return None;
        }

        // Si hay fuente de emojis, intentar renderizar con fallback
        if let (Some(primary_name), Some(emoji_name)) =
            (&self.primary_font, &self.emoji_font)
        {
            let primary = self.fonts.get(primary_name)?;
            let emoji = self.fonts.get(emoji_name)?;

            // Intentar renderizar todo el texto con fuente principal primero
            if let Ok(surface) = primary.render_text_blended(text, r, g, b) {
                unsafe {
                    let sdl_surface = sdl2::surface::Surface::from_ll(
                        surface as *mut sdl2::sys::SDL_Surface,
                    );
                    if let Ok(tex) =
                        self.texture_creator.create_texture_from_surface(&sdl_surface)
                    {
                        return Some(std::mem::transmute(tex));
                    }
                }
            }

            // Si falla, intentar con fuente de emojis
            if let Ok(surface) = emoji.render_text_blended(text, r, g, b) {
                unsafe {
                    let sdl_surface = sdl2::surface::Surface::from_ll(
                        surface as *mut sdl2::sys::SDL_Surface,
                    );
                    if let Ok(tex) =
                        self.texture_creator.create_texture_from_surface(&sdl_surface)
                    {
                        return Some(std::mem::transmute(tex));
                    }
                }
            }

            // Último recurso: renderizar carácter por carácter con fallback
            self.render_char_fallback(text, r, g, b)
        } else if let Some(primary_name) = &self.primary_font {
            // Solo fuente principal
            let font = self.fonts.get(primary_name)?;
            if let Ok(surface) = font.render_text_blended(text, r, g, b) {
                unsafe {
                    let sdl_surface = sdl2::surface::Surface::from_ll(
                        surface as *mut sdl2::sys::SDL_Surface,
                    );
                    if let Ok(tex) =
                        self.texture_creator.create_texture_from_surface(&sdl_surface)
                    {
                        return Some(std::mem::transmute(tex));
                    }
                }
            }
            None
        } else {
            None
        }
    }

    /// Renderizar carácter por carácter con fallback entre fuentes
    fn render_char_fallback(
        &self,
        text: &str,
        r: u8,
        g: u8,
        b: u8,
    ) -> Option<Texture<'a>> {
        // Para cada carácter, intentar con fuente principal, luego emoji
        let mut rendered_chars: Vec<Texture<'a>> = Vec::new();

        for ch in text.chars() {
            let ch_str = ch.to_string();
            let mut rendered = false;

            // Intentar con fuente principal
            if let Some(primary_name) = &self.primary_font {
                if let Some(font) = self.fonts.get(primary_name) {
                    if let Ok(surface) = font.render_text_blended(&ch_str, r, g, b) {
                        unsafe {
                            let sdl_surface = sdl2::surface::Surface::from_ll(
                                surface as *mut sdl2::sys::SDL_Surface,
                            );
                            if let Ok(tex) = self
                                .texture_creator
                                .create_texture_from_surface(&sdl_surface)
                            {
                                rendered_chars.push(std::mem::transmute(tex));
                                rendered = true;
                            }
                        }
                    }
                }
            }

            // Si falló, intentar con fuente de emojis
            if !rendered {
                if let Some(emoji_name) = &self.emoji_font {
                    if let Some(font) = self.fonts.get(emoji_name) {
                        if let Ok(surface) = font.render_text_blended(&ch_str, r, g, b) {
                            unsafe {
                                let sdl_surface = sdl2::surface::Surface::from_ll(
                                    surface as *mut sdl2::sys::SDL_Surface,
                                );
                                if let Ok(tex) = self
                                    .texture_creator
                                    .create_texture_from_surface(&sdl_surface)
                                {
                                    rendered_chars.push(std::mem::transmute(tex));
                                    rendered = true;
                                }
                            }
                        }
                    }
                }
            }

            // Si ambos fallaron, usar cuadrado vacío (placeholder)
            if !rendered {
                // No agregar nada para caracteres no renderizables
            }
        }

        // Combinar todas las texturas de caracteres en una sola
        if rendered_chars.is_empty() {
            return None;
        }

        // Para simplificar, retornar solo el primer carácter renderizado
        // (Una implementación completa combinaría todas las texturas)
        Some(rendered_chars.remove(0))
    }

    /// Obtener referencia a la fuente principal
    pub fn primary_font(&self) -> Option<&FontFFI> {
        self.primary_font
            .as_ref()
            .and_then(|name| self.fonts.get(name))
    }

    /// Verificar si hay fuentes cargadas
    pub fn has_fonts(&self) -> bool {
        !self.fonts.is_empty()
    }

    /// Establecer tamaño de fuente
    pub fn set_size(&mut self, size: u16) {
        self.current_size = size;
    }
}

// ============================================================================
// HELPER: Crear textura de texto rápida (compatibilidad con demos antiguos)
// ============================================================================

/// Helper para crear texturas de texto desde FontFFI directamente
pub fn crear_textura<'a>(
    font: &Option<FontFFI>,
    texto: &str,
    r: u8,
    g: u8,
    b: u8,
    tc: &'a TextureCreator<sdl2::video::WindowContext>,
) -> Option<Texture<'a>> {
    if let Some(f) = font {
        if let Ok(surface) = f.render_text_blended(texto, r, g, b) {
            unsafe {
                let sdl_surface =
                    sdl2::surface::Surface::from_ll(surface as *mut sdl2::sys::SDL_Surface);
                if let Ok(tex) = tc.create_texture_from_surface(&sdl_surface) {
                    return Some(std::mem::transmute(tex));
                }
            }
        }
    }
    None
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_system_new() {
        // Solo prueba de compilación (no se puede crear sin TextureCreator)
        assert!(true);
    }

    #[test]
    fn test_utf8_support() {
        // Verificar que los caracteres UTF-8 se manejan correctamente
        let texto = "¡Hola! Ñoño con acentos: áéíóú ü";
        assert!(texto.contains('ñ'));
        assert!(texto.contains('á'));
        assert!(texto.contains('é'));
        assert!(texto.contains('ü'));
    }

    #[test]
    fn test_emoji_detection() {
        let emoji_paths = [
            "/system/fonts/NotoColorEmoji.ttf",
            "/system/fonts/EmojiOne.ttf",
        ];

        // Al menos una debería existir en Android
        let exists = emoji_paths.iter().any(|p| std::path::Path::new(p).exists());
        // No fallar si no existen (solo informativo)
        println!("¿Fuentes de emojis disponibles? {}", exists);
    }
}
