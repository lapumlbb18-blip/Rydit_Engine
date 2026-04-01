// crates/rydit-gfx/src/sdl2_ffi.rs
// FFI Nativo para SDL2 - Texturas y Audio
// v0.10.8 - Implementación nativa sin crates sdl2_*

#![allow(non_upper_case_globals, non_snake_case, dead_code, improper_ctypes)]

// ============================================================================
// FFI PARA SDL2_IMAGE (TEXTURAS)
// ============================================================================

#[link(name = "SDL2_image")]
extern "C" {
    fn IMG_Init(flags: c_int) -> c_int;
    fn IMG_Quit();
    fn IMG_Load(file: *const c_char) -> *mut SDL_Surface;
}

const IMG_INIT_PNG: c_int = 0x00000002;
const IMG_INIT_JPG: c_int = 0x00000004;

// ============================================================================
// FFI PARA SDL2_TTF (FUENTES)
// ============================================================================

#[link(name = "SDL2_ttf")]
extern "C" {
    fn TTF_Init() -> c_int;
    fn TTF_Quit();
    fn TTF_OpenFont(file: *const c_char, ptsize: c_int) -> *mut TTF_Font;
    fn TTF_CloseFont(font: *mut TTF_Font);
    fn TTF_RenderText_Solid(
        font: *mut TTF_Font,
        text: *const c_char,
        fg: SDL_Color,
    ) -> *mut SDL_Surface;
    fn TTF_RenderText_Blended(
        font: *mut TTF_Font,
        text: *const c_char,
        fg: SDL_Color,
    ) -> *mut SDL_Surface;
}

// ============================================================================
// FFI PARA SDL2_MIXER (AUDIO)
// ============================================================================

#[link(name = "SDL2_mixer")]
extern "C" {
    fn Mix_Init(flags: c_int) -> c_int;
    fn Mix_Quit();
    fn Mix_OpenAudio(
        frequency: c_int,
        format: u16,
        channels: c_int,
        chunksize: c_int,
    ) -> c_int;
    fn Mix_CloseAudio();
    fn Mix_LoadWAV(file: *const c_char) -> *mut Mix_Chunk;
    fn Mix_LoadMUS(file: *const c_char) -> *mut Mix_Music;
    fn Mix_PlayChannel(channel: c_int, chunk: *mut Mix_Chunk, loops: c_int) -> c_int;
    fn Mix_PlayMusic(music: *mut Mix_Music, loops: c_int) -> c_int;
    fn Mix_HaltMusic();
}

const MIX_INIT_OGG: c_int = 0x00000002;
const MIX_INIT_MP3: c_int = 0x00000008;

// ============================================================================
// TIPOS SDL2
// ============================================================================

#[repr(C)]
pub struct SDL_Surface {
    pub flags: u32,
    pub format: *mut SDL_PixelFormat,
    pub w: c_int,
    pub h: c_int,
    pub pitch: c_int,
    pub pixels: *mut c_void,
    // ... más campos (no necesitamos todos)
}

#[repr(C)]
pub struct SDL_PixelFormat {
    // Campos no usados directamente
}

#[repr(C)]
pub struct SDL_Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[repr(C)]
pub struct TTF_Font {
    // Opaco
}

#[repr(C)]
pub struct Mix_Chunk {
    // Opaco
}

#[repr(C)]
pub struct Mix_Music {
    // Opaco
}

// Tipos básicos
use std::os::raw::{c_char, c_int, c_void};
use std::ffi::CString;
use std::ptr;

// ============================================================================
// GESTOR DE TEXTURAS NATIVO
// ============================================================================

pub struct TextureFFI {
    surface: *mut SDL_Surface,
}

impl TextureFFI {
    /// Inicializar SDL2_image
    pub fn init() -> Result<(), String> {
        unsafe {
            let result = IMG_Init(IMG_INIT_PNG | IMG_INIT_JPG);
            if result == 0 {
                Err("Error inicializando SDL2_image".to_string())
            } else {
                Ok(())
            }
        }
    }

    /// Cargar textura desde archivo
    pub fn load(path: &str) -> Result<Self, String> {
        unsafe {
            let c_path = CString::new(path).map_err(|e| e.to_string())?;
            let surface = IMG_Load(c_path.as_ptr());
            
            if surface.is_null() {
                Err(format!("Error cargando textura: {}", path))
            } else {
                Ok(TextureFFI { surface })
            }
        }
    }

    /// Obtener dimensiones
    pub fn dimensions(&self) -> (i32, i32) {
        unsafe {
            ((*self.surface).w, (*self.surface).h)
        }
    }

    /// Obtener superficie raw (para crear textura SDL2)
    pub fn surface(&self) -> *mut SDL_Surface {
        self.surface
    }
}

impl Drop for TextureFFI {
    fn drop(&mut self) {
        unsafe {
            if !self.surface.is_null() {
                // SDL_FreeSurface se llama desde el backend SDL2
            }
        }
    }
}

// ============================================================================
// GESTOR DE AUDIO NATIVO
// ============================================================================

pub struct AudioFFI {
    initialized: bool,
}

impl AudioFFI {
    /// Inicializar SDL2_mixer
    pub fn init() -> Result<Self, String> {
        unsafe {
            let result = Mix_Init(MIX_INIT_OGG | MIX_INIT_MP3);
            if result == 0 {
                return Err("Error inicializando SDL2_mixer".to_string());
            }
            
            let audio_result = Mix_OpenAudio(44100, 0x8010, 2, 1024);
            if audio_result != 0 {
                Mix_Quit();
                return Err("Error abriendo audio".to_string());
            }
            
            Ok(AudioFFI { initialized: true })
        }
    }

    /// Cargar sonido
    pub fn load_sound(&self, path: &str) -> Result<*mut Mix_Chunk, String> {
        unsafe {
            let c_path = CString::new(path).map_err(|e| e.to_string())?;
            let chunk = Mix_LoadWAV(c_path.as_ptr());
            
            if chunk.is_null() {
                Err(format!("Error cargando sonido: {}", path))
            } else {
                Ok(chunk)
            }
        }
    }

    /// Cargar música
    pub fn load_music(&self, path: &str) -> Result<*mut Mix_Music, String> {
        unsafe {
            let c_path = CString::new(path).map_err(|e| e.to_string())?;
            let music = Mix_LoadMUS(c_path.as_ptr());
            
            if music.is_null() {
                Err(format!("Error cargando música: {}", path))
            } else {
                Ok(music)
            }
        }
    }

    /// Reproducir sonido (unsafe por raw pointer)
    pub unsafe fn play_sound(&self, chunk: *mut Mix_Chunk) -> Result<(), String> {
        unsafe {
            let result = Mix_PlayChannel(-1, chunk, 0);
            if result == -1 {
                Err("Error reproduciendo sonido".to_string())
            } else {
                Ok(())
            }
        }
    }

    /// Reproducir música (unsafe por raw pointer)
    pub unsafe fn play_music(&self, music: *mut Mix_Music, loops: i32) -> Result<(), String> {
        unsafe {
            let result = Mix_PlayMusic(music, loops as c_int);
            if result == -1 {
                Err("Error reproduciendo música".to_string())
            } else {
                Ok(())
            }
        }
    }

    /// Detener música
    pub fn stop_music(&self) {
        unsafe {
            Mix_HaltMusic();
        }
    }
}

impl Drop for AudioFFI {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                Mix_CloseAudio();
                Mix_Quit();
            }
        }
    }
}

// ============================================================================
// GESTOR DE FUENTES NATIVO
// ============================================================================

pub struct FontFFI {
    font: *mut TTF_Font,
}

impl FontFFI {
    /// Inicializar SDL2_ttf
    pub fn init() -> Result<(), String> {
        unsafe {
            let result = TTF_Init();
            if result == -1 {
                Err("Error inicializando SDL2_ttf".to_string())
            } else {
                Ok(())
            }
        }
    }

    /// Cargar fuente
    pub fn load(path: &str, size: i32) -> Result<Self, String> {
        unsafe {
            let c_path = CString::new(path).map_err(|e| e.to_string())?;
            let font = TTF_OpenFont(c_path.as_ptr(), size as c_int);
            
            if font.is_null() {
                Err(format!("Error cargando fuente: {}", path))
            } else {
                Ok(FontFFI { font })
            }
        }
    }

    /// Renderizar texto (Solid - rápido, sin alpha suave)
    pub fn render_text(&self, text: &str, r: u8, g: u8, b: u8) -> Result<*mut SDL_Surface, String> {
        unsafe {
            let c_text = CString::new(text).map_err(|e| e.to_string())?;
            let color = SDL_Color { r, g, b, a: 255 };

            let surface = TTF_RenderText_Solid(self.font, c_text.as_ptr(), color);

            if surface.is_null() {
                Err("Error renderizando texto".to_string())
            } else {
                Ok(surface)
            }
        }
    }

    /// Renderizar texto (Blended - más lento, con alpha suave)
    pub fn render_text_blended(&self, text: &str, r: u8, g: u8, b: u8) -> Result<*mut SDL_Surface, String> {
        unsafe {
            let c_text = CString::new(text).map_err(|e| e.to_string())?;
            let color = SDL_Color { r, g, b, a: 255 };

            let surface = TTF_RenderText_Blended(self.font, c_text.as_ptr(), color);

            if surface.is_null() {
                Err("Error renderizando texto".to_string())
            } else {
                Ok(surface)
            }
        }
    }

    /// Obtener superficie raw
    pub fn font_ptr(&self) -> *mut TTF_Font {
        self.font
    }
}

impl Drop for FontFFI {
    fn drop(&mut self) {
        unsafe {
            if !self.font.is_null() {
                TTF_CloseFont(self.font);
            }
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_ffi_init() {
        // No podemos testear sin SDL2 inicializado, pero verificamos que compile
        // Test placeholder - FFI requiere SDL2 inicializado
    }
}
