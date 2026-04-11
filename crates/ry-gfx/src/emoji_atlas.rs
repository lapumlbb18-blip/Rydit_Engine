// crates/ry-gfx/src/emoji_atlas.rs
// Atlas de Emojis como Sprites PNG - v0.17.0
// Emojis renderizados proceduralmente como texturas SDL2

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use std::collections::HashMap;

// ============================================================================
// EMOJI REGISTRO - Mapeo char -> color/forma
// ============================================================================

struct EmojiDef {
    bg_color: (u8, u8, u8),
    symbol: char,
    symbol_color: (u8, u8, u8),
}

impl EmojiDef {
    fn new(bg: (u8, u8, u8), sym: char, sym_c: (u8, u8, u8)) -> Self {
        Self {
            bg_color: bg,
            symbol: sym,
            symbol_color: sym_c,
        }
    }
}

fn get_emoji_defs() -> HashMap<char, EmojiDef> {
    let mut map = HashMap::new();
    
    // Caras
    map.insert('😀', EmojiDef::new((255, 220, 50), '☺', (255, 150, 0)));   // Sonrisa
    map.insert('😃', EmojiDef::new((255, 220, 50), '▃', (200, 50, 50)));    // Sonrisa grande
    map.insert('😄', EmojiDef::new((255, 220, 50), '◉', (200, 50, 50)));    // Ojos sonriendo
    map.insert('😁', EmojiDef::new((255, 220, 50), '▬', (200, 100, 0)));    // Sonrisa amplia
    map.insert('😆', EmojiDef::new((255, 220, 50), '✚', (200, 50, 50)));    // Risa
    map.insert('😅', EmojiDef::new((150, 200, 255), '💧', (50, 100, 255))); // Nervioso
    map.insert('😂', EmojiDef::new((255, 220, 50), '▼', (50, 150, 255)));   // Lágrimas de risa
    map.insert('🤣', EmojiDef::new((255, 220, 50), '↻', (200, 50, 50)));    // Rodando de risa
    map.insert('😊', EmojiDef::new((255, 180, 120), '◕', (255, 100, 100))); // Sonrojado
    map.insert('👍', EmojiDef::new((255, 220, 50), '👆', (200, 150, 50))); // Pulgar arriba
    
    // Corazones
    map.insert('❤', EmojiDef::new((255, 220, 220), '♥', (255, 0, 0)));     // Corazón rojo
    map.insert('💯', EmojiDef::new((255, 50, 50), '1', (255, 255, 255)));   // 100 puntos
    
    // Fuego/estrellas
    map.insert('🔥', EmojiDef::new((255, 150, 0), '▲', (255, 255, 100)));  // Fuego
    map.insert('⭐', EmojiDef::new((255, 255, 200), '★', (255, 200, 0)));   // Estrella
    map.insert('🌟', EmojiDef::new((255, 255, 150), '✦', (255, 255, 255))); // Estrella brillante
    map.insert('✅', EmojiDef::new((50, 200, 50), '✓', (255, 255, 255)));   // Check verde
    map.insert('❌', EmojiDef::new((200, 50, 50), '✕', (255, 255, 255)));   // X roja
    map.insert('⚡', EmojiDef::new((255, 255, 100), '⚡', (255, 100, 0)));  // Rayo
    
    // Juegos/tech
    map.insert('🎮', EmojiDef::new((50, 50, 150), '⊞', (100, 200, 255)));   // Gamepad
    map.insert('🎯', EmojiDef::new((255, 255, 255), '◎', (255, 0, 0)));     // Diana
    map.insert('🎨', EmojiDef::new((200, 100, 200), '◐', (255, 255, 255))); // Arte
    map.insert('🎬', EmojiDef::new((50, 50, 50), '▣', (255, 255, 255)));    // Claqueta
    map.insert('🚀', EmojiDef::new((100, 100, 200), '▲', (255, 200, 100))); // Cohete
    
    // Música
    map.insert('🎵', EmojiDef::new((100, 100, 200), '♪', (255, 255, 255))); // Nota
    map.insert('🎶', EmojiDef::new((100, 100, 200), '♫', (255, 255, 255))); // Notas
    
    // Otros
    map.insert('💡', EmojiDef::new((255, 255, 200), '💡', (255, 200, 0)));  // Bombilla
    map.insert('🎉', EmojiDef::new((200, 50, 200), '✦', (255, 255, 100))); // Confeti
    
    map
}

// ============================================================================
// EMOJI ATLAS - Texturas de emojis generadas proceduralmente
// ============================================================================

pub struct EmojiAtlas {
    pub textures: HashMap<char, Texture<'static>>,
    emoji_size: u32,
}

impl EmojiAtlas {
    /// Crear atlas de emojis procedural
    pub fn new<'a>(
        tc: &'a TextureCreator<sdl2::video::WindowContext>,
        size: u32,
    ) -> Result<Self, String> {
        let emoji_defs = get_emoji_defs();
        let mut textures = HashMap::new();

        for (ch, def) in &emoji_defs {
            let tex = Self::create_emoji_texture(tc, *ch, def, size)?;
            textures.insert(*ch, unsafe { std::mem::transmute(tex) });
        }

        println!("✅ Emoji Atlas creado: {} emojis ({}x{}px)", textures.len(), size, size);

        Ok(Self {
            textures,
            emoji_size: size,
        })
    }

    /// Crear textura de emoji procedural
    fn create_emoji_texture<'a>(
        tc: &'a TextureCreator<sdl2::video::WindowContext>,
        _ch: char,
        def: &EmojiDef,
        size: u32,
    ) -> Result<Texture<'a>, String> {
        let mut surface = Surface::new(size, size, sdl2::pixels::PixelFormatEnum::RGBA8888)
            .map_err(|e| e.to_string())?;

        // Fondo circular
        let center = size as i32 / 2;
        let radius = (size as i32 / 2) - 2;

        // Dibujar círculo con rects (aproximación)
        for y in -(radius)..=radius {
            for x in -(radius)..=radius {
                if x * x + y * y <= radius * radius {
                    let px = center + x;
                    let py = center + y;
                    if px >= 0 && px < size as i32 && py >= 0 && py < size as i32 {
                        let _ = surface.fill_rect(
                            Rect::new(px, py, 1, 1),
                            Color::RGB(def.bg_color.0, def.bg_color.1, def.bg_color.2),
                        );
                    }
                }
            }
        }

        // Símbolo central
        let sym_size = size / 2;
        let sym_x = (size - sym_size) / 2;
        let sym_y = (size - sym_size) / 2;

        // Dibujar símbolo simple (rectángulo de color)
        let _ = surface.fill_rect(
            Rect::new(sym_x as i32, sym_y as i32, sym_size, sym_size),
            Color::RGB(def.symbol_color.0, def.symbol_color.1, def.symbol_color.2),
        );

        // Borde
        let _ = surface.fill_rect(
            Rect::new(0, 0, size, 2),
            Color::RGBA(0, 0, 0, 100),
        );
        let _ = surface.fill_rect(
            Rect::new(0, size as i32 - 2, size, 2),
            Color::RGBA(0, 0, 0, 100),
        );

        tc.create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
    }

    /// Obtener textura de emoji
    pub fn get(&self, ch: char) -> Option<&Texture<'static>> {
        self.textures.get(&ch)
    }

    /// Verificar si un carácter es emoji
    pub fn is_emoji(&self, ch: char) -> bool {
        self.textures.contains_key(&ch)
    }

    /// Tamaño de emojis
    pub fn size(&self) -> u32 {
        self.emoji_size
    }

    /// Dibujar emoji en canvas
    pub fn draw<'a>(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        ch: char,
        x: i32,
        y: i32,
    ) {
        if let Some(tex) = self.get(ch) {
            let s = self.emoji_size as i32;
            let _ = canvas.copy(tex, None, Rect::new(x, y, s as u32, s as u32));
        }
    }
}

// ============================================================================
// TEXT RENDERER CON EMOJI SUPPORT
// ============================================================================

/// Renderizar texto mezclando texto normal + emojis
pub fn render_text_with_emojis<'a>(
    text: &str,
    font: &Option<crate::sdl2_ffi::FontFFI>,
    emoji_atlas: &EmojiAtlas,
    r: u8,
    g: u8,
    b: u8,
    tc: &'a TextureCreator<sdl2::video::WindowContext>,
) -> Option<Texture<'a>> {
    // Para simplificar: detectar emojis y renderizar texto normal donde no hay emojis
    let mut has_emoji = false;
    for ch in text.chars() {
        if emoji_atlas.is_emoji(ch) {
            has_emoji = true;
            break;
        }
    }
    
    if !has_emoji {
        // Solo texto normal
        if let Some(f) = font {
            if let Ok(surface) = f.render_text_blended(text, r, g, b) {
                unsafe {
                    let s = sdl2::surface::Surface::from_ll(surface as *mut sdl2::sys::SDL_Surface);
                    if let Ok(tex) = tc.create_texture_from_surface(&s) {
                        return Some(std::mem::transmute(tex));
                    }
                }
            }
        }
        return None;
    }
    
    // Tiene emojis: crear textura combinada placeholder
    let mut width = 0;
    for ch in text.chars() {
        if emoji_atlas.is_emoji(ch) {
            width += emoji_atlas.size();
        } else {
            width += 10; // Aproximación por carácter
        }
    }
    
    let height = emoji_atlas.size().max(16);
    let mut surface = Surface::new(width, height, sdl2::pixels::PixelFormatEnum::RGBA8888).ok()?;
    let _ = surface.fill_rect(None, Color::RGBA(0, 0, 0, 0));
    
    let mut x_offset = 0;
    for ch in text.chars() {
        if emoji_atlas.is_emoji(ch) {
            if let Some(tex) = emoji_atlas.get(ch) {
                let q = tex.query();
                let s = q.width.min(emoji_atlas.size());
                let _ = surface.fill_rect(
                    Rect::new(x_offset, 0, s, s.min(height)),
                    Color::RGB(255, 220, 50),
                );
                x_offset += s as i32;
            }
        } else {
            x_offset += 10;
        }
    }
    
    let combined_tex = tc.create_texture_from_surface(&surface).ok()?;
    Some(unsafe { std::mem::transmute(combined_tex) })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji_detection() {
        let defs = get_emoji_defs();
        assert!(defs.contains_key(&'😀'));
        assert!(defs.contains_key(&'🎮'));
        assert!(defs.contains_key(&'❤'));
        assert!(defs.contains_key(&'🔥'));
    }

    #[test]
    fn test_emoji_count() {
        let defs = get_emoji_defs();
        assert!(defs.len() >= 20);
    }
}
