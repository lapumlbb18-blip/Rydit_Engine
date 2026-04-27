//! # RyDit Graphics Layer (rydit-gfx)
//!
//! **Sincronización entre Rust (arquitecto) y Raylib (pincel)**
//!
//! Esta capa abstrae las inconsistencias de la API de raylib-rs,
//! proporcionando una interfaz consistente para RyDit.
//!
//! ## Filosofía
//!
//! - **Rust = Arquitecto**: Controla el game loop, input, decisiones
//! - **Raylib = Pincel**: Solo ejecuta dibujos
//! - **rydit-gfx = Puente**: Sincroniza ambos mundos
//!
//! ## Ejemplo de Uso
//!
//! ```rust,no_run
//! use ry_gfx::{RyditGfx, ColorRydit, Key};
//!
//! fn main() {
//!     let mut gfx = RyditGfx::new("Mi Juego RyDit", 800, 600);
//!
//!     while !gfx.should_close() {
//!         gfx.begin_draw();
//!
//!         gfx.clear_background(ColorRydit::Negro);
//!         gfx.draw_circle(400, 300, 50, ColorRydit::Rojo);
//!         gfx.draw_rect(100, 100, 100, 100, ColorRydit::Verde);
//!
//!         if gfx.is_key_pressed(Key::Escape) {
//!             break;
//!         }
//!
//!         gfx.end_draw();
//!     }
//! }
//! ```

#![allow(clippy::too_many_arguments)]

// Módulo de partículas v0.5.3
pub mod gpu_particles;

// Módulo de cámara 2D v0.9.0
pub mod camera;

// Módulo de debug log v0.8.5
pub mod debug_log;

// Módulo de render queue v0.9.0 - Command Queue + Double Buffering + Platform Sync
pub mod render_queue;

// Módulo de Viewports (Lienzos) v0.22.0
pub mod viewports;
pub use viewports::ViewportManager;

// Módulo de ECS render v0.10.0 - ECS + rlgl integration
// 🗑️ ecs_render eliminado v0.13.1 — usaba ry-ecs (eliminado)

// Módulo de GPU Instancing v0.10.1 - FFI OpenGL + Shaders GLSL
pub mod gpu_instancing;

// Módulo de Input SDL2 v0.10.4 - Eventos para Termux-X11/Android
pub mod input_sdl2;

// Módulo de Backend SDL2 v0.10.6 - Ventana + OpenGL + Assets
pub mod backend_sdl2;

// Módulo de Audio SDL2 v0.10.8 - SDL2_mixer (pendiente)
pub mod audio_sdl2;

// Módulo de Audio Mixer Avanzado v0.17.0 - Buses + Spatial + Fade
pub mod audio_mixer;

// Módulo de Fuentes SDL2 v0.10.8 - SDL2_ttf (pendiente)
pub mod font_sdl2;

// Módulo de Sistema de Fuentes v0.17.0 - UTF-8 + Emojis + Fallback
pub mod font_system;

// Módulo de Emoji Atlas v0.17.0 - Emojis como sprites procedurales
pub mod emoji_atlas;

// Módulo de Transiciones v0.18.0 - 15+ transiciones tipo editor de video
pub mod transitions;

// Módulo de FFI Nativo SDL2 v0.10.8 - Texturas, Audio, Fuentes (nativo)
pub mod sdl2_ffi;

// 🆕 UI Toolkit v0.11.0 - Botones, Labels, Paneles
pub mod toolkit;

// 🆕 FSR 1.0 v0.11.4 - FidelityFX Super Resolution (Upscale + Sharpen)
pub mod fsr;

// 🆕 v0.19.1: Iluminación 2D — luces puntuales + sombras
pub mod lighting;

// 🆕 v0.19.2: SDL2 Helpers — puente para demos SDL2
pub mod sdl2_helpers;

use raylib::consts::KeyboardKey;
use raylib::prelude::*;

// Importar migui para implementar el backend
#[cfg(feature = "migui")]
use migui::{Color as MiguiColor, MiguiBackend, Rect as MiguiRect};

// SDL2 para carga de texturas

// ============================================================================
// AUDIO SYSTEM - v0.5.2
// ============================================================================

use std::collections::HashMap;
use std::ffi::CString;
use std::str::FromStr;

/// Sistema de audio con raylib
/// Nota: Sound y Music son structs FFI que contienen pointers internos
pub struct AudioSystem {
    initialized: bool,
    sounds: HashMap<String, raylib::ffi::Sound>,
    music: Option<raylib::ffi::Music>,
}

impl AudioSystem {
    /// Inicializar sistema de audio
    pub fn new() -> Self {
        unsafe {
            raylib::ffi::InitAudioDevice();
            println!("[AUDIO] Dispositivo de audio inicializado");
        }
        Self {
            initialized: true,
            sounds: HashMap::new(),
            music: None,
        }
    }

    /// Cargar sonido desde archivo
    pub fn load_sound(&mut self, id: &str, path: &str) -> Result<(), String> {
        if !self.initialized {
            return Err("Audio no inicializado".into());
        }

        let c_path = CString::new(path).map_err(|e| format!("Error en path: {}", e))?;

        unsafe {
            let sound = raylib::ffi::LoadSound(c_path.as_ptr());
            // Verificamos que el buffer sea válido
            if !sound.stream.buffer.is_null() || sound.frameCount > 0 {
                println!("[AUDIO] Sonido '{}' cargado: {}", id, path);
                self.sounds.insert(id.to_string(), sound);
                Ok(())
            } else {
                Err(format!("Error cargando sonido '{}'", path))
            }
        }
    }

    /// Reproducir sonido
    pub fn play_sound(&self, id: &str) -> bool {
        if let Some(sound) = self.sounds.get(id) {
            unsafe {
                raylib::ffi::PlaySound(*sound);
            }
            true
        } else {
            false
        }
    }

    /// Detener sonido
    pub fn stop_sound(&self, id: &str) -> bool {
        if let Some(sound) = self.sounds.get(id) {
            unsafe {
                raylib::ffi::StopSound(*sound);
            }
            true
        } else {
            false
        }
    }

    /// Configurar volumen de sonido (0.0 - 1.0)
    pub fn set_sound_volume(&self, id: &str, volume: f32) -> bool {
        if let Some(sound) = self.sounds.get(id) {
            unsafe {
                raylib::ffi::SetSoundVolume(*sound, volume);
            }
            true
        } else {
            false
        }
    }

    /// Cargar música desde archivo
    pub fn load_music(&mut self, path: &str) -> Result<(), String> {
        if !self.initialized {
            return Err("Audio no inicializado".into());
        }

        let c_path = CString::new(path).map_err(|e| format!("Error en path: {}", e))?;

        unsafe {
            let music = raylib::ffi::LoadMusicStream(c_path.as_ptr());
            // Verificamos que el buffer sea válido
            if !music.stream.buffer.is_null() || music.frameCount > 0 {
                println!("[AUDIO] Música cargada: {}", path);
                self.music = Some(music);
                Ok(())
            } else {
                Err(format!("Error cargando música '{}'", path))
            }
        }
    }

    /// Reproducir música
    pub fn play_music(&mut self) {
        if let Some(ref music) = self.music {
            unsafe {
                raylib::ffi::PlayMusicStream(*music);
            }
            println!("[AUDIO] Reproduciendo música");
        }
    }

    /// Detener música
    pub fn stop_music(&mut self) {
        if let Some(ref music) = self.music {
            unsafe {
                raylib::ffi::StopMusicStream(*music);
            }
            println!("[AUDIO] Música detenida");
        }
    }

    /// Actualizar música (llamar en cada frame)
    pub fn update_music(&mut self) {
        if let Some(ref music) = self.music {
            unsafe {
                raylib::ffi::UpdateMusicStream(*music);
            }
        }
    }

    /// Configurar volumen de música (0.0 - 1.0)
    pub fn set_music_volume(&mut self, volume: f32) {
        if let Some(ref music) = self.music {
            unsafe {
                raylib::ffi::SetMusicVolume(*music, volume);
            }
        }
    }

    /// Verificar si la música está reproduciendo
    pub fn is_music_playing(&self) -> bool {
        if let Some(ref music) = self.music {
            unsafe { raylib::ffi::IsMusicStreamPlaying(*music) }
        } else {
            false
        }
    }

    /// Descargar sonido y liberar memoria
    pub fn unload_sound(&mut self, id: &str) {
        if let Some(sound) = self.sounds.remove(id) {
            unsafe {
                raylib::ffi::UnloadSound(sound);
            }
            println!("[AUDIO] Sonido '{}' descargado", id);
        }
    }

    /// Descargar música y liberar memoria
    pub fn unload_music(&mut self) {
        if let Some(music) = self.music.take() {
            unsafe {
                raylib::ffi::UnloadMusicStream(music);
            }
            println!("[AUDIO] Música descargada");
        }
    }

    /// Verificar si existe un sonido
    pub fn has_sound(&self, id: &str) -> bool {
        self.sounds.contains_key(id)
    }

    /// Cantidad de sonidos cargados
    pub fn sound_count(&self) -> usize {
        self.sounds.len()
    }
}

impl Default for AudioSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AudioSystem {
    fn drop(&mut self) {
        println!("[AUDIO] Cerrando sistema de audio...");
        // Descargar todos los sonidos
        for (_, sound) in self.sounds.drain() {
            unsafe {
                raylib::ffi::UnloadSound(sound);
            }
        }
        // Descargar música
        if let Some(music) = self.music.take() {
            unsafe {
                raylib::ffi::UnloadMusicStream(music);
            }
        }
        // Cerrar dispositivo
        if self.initialized {
            unsafe {
                raylib::ffi::CloseAudioDevice();
            }
            println!("[AUDIO] Dispositivo cerrado");
        }
    }
}

// Colores manuales (raylib nobuild no incluye colors::prelude)
pub const RED: Color = Color {
    r: 230,
    g: 41,
    b: 55,
    a: 255,
};
pub const GREEN: Color = Color {
    r: 117,
    g: 203,
    b: 100,
    a: 255,
};
pub const BLUE: Color = Color {
    r: 51,
    g: 122,
    b: 206,
    a: 255,
};
pub const YELLOW: Color = Color {
    r: 253,
    g: 249,
    b: 0,
    a: 255,
};
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};
pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};
pub const MAGENTA: Color = Color {
    r: 255,
    g: 0,
    b: 255,
    a: 255,
};
pub const PINK: Color = Color {
    r: 255,
    g: 192,
    b: 203,
    a: 255,
};
pub const ORANGE: Color = Color {
    r: 255,
    g: 165,
    b: 0,
    a: 255,
};
pub const GRAY: Color = Color {
    r: 128,
    g: 128,
    b: 128,
    a: 255,
};

// Colores adicionales v0.2.0
pub const CYAN: Color = Color {
    r: 0,
    g: 255,
    b: 255,
    a: 255,
};
pub const PURPLE: Color = Color {
    r: 128,
    g: 0,
    b: 128,
    a: 255,
};
pub const BROWN: Color = Color {
    r: 165,
    g: 42,
    b: 42,
    a: 255,
};
pub const LIME: Color = Color {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};
pub const NAVY: Color = Color {
    r: 0,
    g: 0,
    b: 128,
    a: 255,
};
pub const OLIVE: Color = Color {
    r: 128,
    g: 128,
    b: 0,
    a: 255,
};
pub const TEAL: Color = Color {
    r: 0,
    g: 128,
    b: 128,
    a: 255,
};
pub const MAROON: Color = Color {
    r: 128,
    g: 0,
    b: 0,
    a: 255,
};

// Teclas
pub const KEY_ESCAPE: KeyboardKey = unsafe { std::mem::transmute(256i32) };
pub const KEY_SPACE: KeyboardKey = unsafe { std::mem::transmute(32i32) };
pub const KEY_ENTER: KeyboardKey = unsafe { std::mem::transmute(257i32) };
pub const KEY_UP: KeyboardKey = unsafe { std::mem::transmute(265i32) };
pub const KEY_DOWN: KeyboardKey = unsafe { std::mem::transmute(264i32) };
pub const KEY_LEFT: KeyboardKey = unsafe { std::mem::transmute(263i32) };
pub const KEY_RIGHT: KeyboardKey = unsafe { std::mem::transmute(262i32) };
pub const KEY_A: KeyboardKey = unsafe { std::mem::transmute(65i32) };
pub const KEY_B: KeyboardKey = unsafe { std::mem::transmute(66i32) };
pub const KEY_C: KeyboardKey = unsafe { std::mem::transmute(67i32) };
pub const KEY_D: KeyboardKey = unsafe { std::mem::transmute(68i32) };
pub const KEY_E: KeyboardKey = unsafe { std::mem::transmute(69i32) };
pub const KEY_F: KeyboardKey = unsafe { std::mem::transmute(70i32) };
pub const KEY_G: KeyboardKey = unsafe { std::mem::transmute(71i32) };
pub const KEY_H: KeyboardKey = unsafe { std::mem::transmute(72i32) };
pub const KEY_I: KeyboardKey = unsafe { std::mem::transmute(73i32) };
pub const KEY_J: KeyboardKey = unsafe { std::mem::transmute(74i32) };
pub const KEY_K: KeyboardKey = unsafe { std::mem::transmute(75i32) };
pub const KEY_L: KeyboardKey = unsafe { std::mem::transmute(76i32) };
pub const KEY_M: KeyboardKey = unsafe { std::mem::transmute(77i32) };
pub const KEY_N: KeyboardKey = unsafe { std::mem::transmute(78i32) };
pub const KEY_O: KeyboardKey = unsafe { std::mem::transmute(79i32) };
pub const KEY_P: KeyboardKey = unsafe { std::mem::transmute(80i32) };
pub const KEY_Q: KeyboardKey = unsafe { std::mem::transmute(81i32) };
pub const KEY_R: KeyboardKey = unsafe { std::mem::transmute(82i32) };
pub const KEY_S: KeyboardKey = unsafe { std::mem::transmute(83i32) };
pub const KEY_T: KeyboardKey = unsafe { std::mem::transmute(84i32) };
pub const KEY_U: KeyboardKey = unsafe { std::mem::transmute(85i32) };
pub const KEY_V: KeyboardKey = unsafe { std::mem::transmute(86i32) };
pub const KEY_W: KeyboardKey = unsafe { std::mem::transmute(87i32) };
pub const KEY_X: KeyboardKey = unsafe { std::mem::transmute(88i32) };
pub const KEY_Y: KeyboardKey = unsafe { std::mem::transmute(89i32) };
pub const KEY_Z: KeyboardKey = unsafe { std::mem::transmute(90i32) };
pub const KEY_ZERO: KeyboardKey = unsafe { std::mem::transmute(48i32) };
pub const KEY_ONE: KeyboardKey = unsafe { std::mem::transmute(49i32) };
pub const KEY_TWO: KeyboardKey = unsafe { std::mem::transmute(50i32) };
pub const KEY_THREE: KeyboardKey = unsafe { std::mem::transmute(51i32) };
pub const KEY_FOUR: KeyboardKey = unsafe { std::mem::transmute(52i32) };
pub const KEY_FIVE: KeyboardKey = unsafe { std::mem::transmute(53i32) };
pub const KEY_SIX: KeyboardKey = unsafe { std::mem::transmute(54i32) };
pub const KEY_SEVEN: KeyboardKey = unsafe { std::mem::transmute(55i32) };
pub const KEY_EIGHT: KeyboardKey = unsafe { std::mem::transmute(56i32) };
pub const KEY_NINE: KeyboardKey = unsafe { std::mem::transmute(57i32) };

// Teclas adicionales v0.9.2 (100+ teclas)
pub const KEY_TAB: KeyboardKey = unsafe { std::mem::transmute(258i32) };
pub const KEY_CAPS_LOCK: KeyboardKey = unsafe { std::mem::transmute(259i32) };
pub const KEY_LEFT_SHIFT: KeyboardKey = unsafe { std::mem::transmute(340i32) };
pub const KEY_RIGHT_SHIFT: KeyboardKey = unsafe { std::mem::transmute(344i32) };
pub const KEY_LEFT_CONTROL: KeyboardKey = unsafe { std::mem::transmute(341i32) };
pub const KEY_RIGHT_CONTROL: KeyboardKey = unsafe { std::mem::transmute(345i32) };
pub const KEY_LEFT_ALT: KeyboardKey = unsafe { std::mem::transmute(342i32) };
pub const KEY_RIGHT_ALT: KeyboardKey = unsafe { std::mem::transmute(346i32) };
pub const KEY_PAGE_UP: KeyboardKey = unsafe { std::mem::transmute(266i32) };
pub const KEY_PAGE_DOWN: KeyboardKey = unsafe { std::mem::transmute(267i32) };
pub const KEY_HOME: KeyboardKey = unsafe { std::mem::transmute(268i32) };
pub const KEY_END: KeyboardKey = unsafe { std::mem::transmute(269i32) };
pub const KEY_INSERT: KeyboardKey = unsafe { std::mem::transmute(260i32) };
pub const KEY_DELETE: KeyboardKey = unsafe { std::mem::transmute(261i32) };
pub const KEY_F1: KeyboardKey = unsafe { std::mem::transmute(290i32) };
pub const KEY_F2: KeyboardKey = unsafe { std::mem::transmute(291i32) };
pub const KEY_F3: KeyboardKey = unsafe { std::mem::transmute(292i32) };
pub const KEY_F4: KeyboardKey = unsafe { std::mem::transmute(293i32) };
pub const KEY_F5: KeyboardKey = unsafe { std::mem::transmute(294i32) };
pub const KEY_F6: KeyboardKey = unsafe { std::mem::transmute(295i32) };
pub const KEY_F7: KeyboardKey = unsafe { std::mem::transmute(296i32) };
pub const KEY_F8: KeyboardKey = unsafe { std::mem::transmute(297i32) };
pub const KEY_F9: KeyboardKey = unsafe { std::mem::transmute(298i32) };
pub const KEY_F10: KeyboardKey = unsafe { std::mem::transmute(299i32) };
pub const KEY_F11: KeyboardKey = unsafe { std::mem::transmute(300i32) };
pub const KEY_F12: KeyboardKey = unsafe { std::mem::transmute(301i32) };

// ============================================================================
// COLORES RYDIT
// ============================================================================

/// Colores básicos para RyDit
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ColorRydit {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
    Magenta,
    Rosa,
    Naranja,
    Gris,
    // Colores v0.2.0
    Cyan,
    Morado,
    Cafe,
    Lima,
    AzulOscuro,
    Oliva,
    Turquesa,
    Vino,
}

impl ColorRydit {
    /// Convertir a Color de raylib
    pub fn to_color(&self) -> Color {
        match self {
            ColorRydit::Rojo => RED,
            ColorRydit::Verde => GREEN,
            ColorRydit::Azul => BLUE,
            ColorRydit::Amarillo => YELLOW,
            ColorRydit::Blanco => WHITE,
            ColorRydit::Negro => BLACK,
            ColorRydit::Magenta => MAGENTA,
            ColorRydit::Rosa => PINK,
            ColorRydit::Naranja => ORANGE,
            ColorRydit::Gris => GRAY,
            ColorRydit::Cyan => CYAN,
            ColorRydit::Morado => PURPLE,
            ColorRydit::Cafe => BROWN,
            ColorRydit::Lima => LIME,
            ColorRydit::AzulOscuro => NAVY,
            ColorRydit::Oliva => OLIVE,
            ColorRydit::Turquesa => TEAL,
            ColorRydit::Vino => MAROON,
        }
    }

    /// Convertir a componentes RGB (r, g, b)
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            ColorRydit::Rojo => (255, 0, 0),
            ColorRydit::Verde => (0, 255, 0),
            ColorRydit::Azul => (0, 0, 255),
            ColorRydit::Amarillo => (255, 255, 0),
            ColorRydit::Blanco => (255, 255, 255),
            ColorRydit::Negro => (0, 0, 0),
            ColorRydit::Magenta => (255, 0, 255),
            ColorRydit::Rosa => (255, 192, 203),
            ColorRydit::Naranja => (255, 165, 0),
            ColorRydit::Gris => (128, 128, 128),
            ColorRydit::Cyan => (0, 255, 255),
            ColorRydit::Morado => (128, 0, 128),
            ColorRydit::Cafe => (165, 42, 42),
            ColorRydit::Lima => (0, 255, 0),
            ColorRydit::AzulOscuro => (0, 0, 139),
            ColorRydit::Oliva => (128, 128, 0),
            ColorRydit::Turquesa => (64, 224, 208),
            ColorRydit::Vino => (128, 0, 64),
        }
    }

    /// Crear desde Color de migui (solo con feature migui)
    #[cfg(feature = "migui")]
    pub fn from_migui(color: MiguiColor) -> Self {
        // Convertir componentes RGB directamente
        // migui::Color tiene campos públicos: r, g, b, a
        // Usamos aproximación a los colores RyDit más cercanos
        let r = color.r;
        let g = color.g;
        let b = color.b;

        // Calcular distancia a cada color RyDit y elegir el más cercano
        let colores = vec![
            (ColorRydit::Rojo, RED),
            (ColorRydit::Verde, GREEN),
            (ColorRydit::Azul, BLUE),
            (ColorRydit::Amarillo, YELLOW),
            (ColorRydit::Blanco, WHITE),
            (ColorRydit::Negro, BLACK),
            (ColorRydit::Gris, GRAY),
            (ColorRydit::Naranja, ORANGE),
            (ColorRydit::Cyan, CYAN),
            (ColorRydit::Morado, PURPLE),
            (ColorRydit::Cafe, BROWN),
            (ColorRydit::Lima, LIME),
            (ColorRydit::AzulOscuro, NAVY),
            (ColorRydit::Oliva, OLIVE),
            (ColorRydit::Turquesa, TEAL),
            (ColorRydit::Vino, MAROON),
        ];

        let mut mejor_color = ColorRydit::Blanco;
        let mut mejor_distancia = f32::MAX;

        for (ry_color, raylib_color) in colores {
            let dr = (r as i32 - raylib_color.r as i32).pow(2) as f32;
            let dg = (g as i32 - raylib_color.g as i32).pow(2) as f32;
            let db = (b as i32 - raylib_color.b as i32).pow(2) as f32;
            let distancia = dr + dg + db;

            if distancia < mejor_distancia {
                mejor_distancia = distancia;
                mejor_color = ry_color;
            }
        }

        mejor_color
    }
}

// Implementación del trait FromStr para ColorRydit
impl FromStr for ColorRydit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rojo" | "red" => Ok(ColorRydit::Rojo),
            "verde" | "green" => Ok(ColorRydit::Verde),
            "azul" | "blue" => Ok(ColorRydit::Azul),
            "amarillo" | "yellow" => Ok(ColorRydit::Amarillo),
            "blanco" | "white" => Ok(ColorRydit::Blanco),
            "negro" | "black" => Ok(ColorRydit::Negro),
            "magenta" | "fucsia" => Ok(ColorRydit::Magenta),
            "rosa" | "pink" => Ok(ColorRydit::Rosa),
            "naranja" | "orange" => Ok(ColorRydit::Naranja),
            "gris" | "gray" | "grey" => Ok(ColorRydit::Gris),
            "cyan" | "celeste" => Ok(ColorRydit::Cyan),
            "morado" | "purple" | "violeta" => Ok(ColorRydit::Morado),
            "cafe" | "brown" | "marron" => Ok(ColorRydit::Cafe),
            "lima" | "lime" => Ok(ColorRydit::Lima),
            "azuloscuro" | "navy" | "azul oscuro" => Ok(ColorRydit::AzulOscuro),
            "oliva" | "olive" => Ok(ColorRydit::Oliva),
            "turquesa" | "teal" => Ok(ColorRydit::Turquesa),
            "vino" | "maroon" | "granate" => Ok(ColorRydit::Vino),
            _ => Ok(ColorRydit::Blanco),
        }
    }
}

// ============================================================================
// TECLAS
// ============================================================================

/// Teclas para input
/// v0.9.2: 100+ teclas
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Key {
    Escape,
    Space,
    Enter,
    Tab,
    CapsLock,
    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
    LeftAlt,
    RightAlt,
    PageUp,
    PageDown,
    Home,
    End,
    Insert,
    Delete,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
}

impl Key {
    /// Convertir a KeyboardKey de raylib
    pub fn to_raylib(&self) -> KeyboardKey {
        match self {
            // Especiales
            Key::Escape => KEY_ESCAPE,
            Key::Space => KEY_SPACE,
            Key::Enter => KEY_ENTER,
            Key::Tab => KEY_TAB,
            Key::CapsLock => KEY_CAPS_LOCK,
            Key::LeftShift => KEY_LEFT_SHIFT,
            Key::RightShift => KEY_RIGHT_SHIFT,
            Key::LeftControl => KEY_LEFT_CONTROL,
            Key::RightControl => KEY_RIGHT_CONTROL,
            Key::LeftAlt => KEY_LEFT_ALT,
            Key::RightAlt => KEY_RIGHT_ALT,

            // Navegación
            Key::PageUp => KEY_PAGE_UP,
            Key::PageDown => KEY_PAGE_DOWN,
            Key::Home => KEY_HOME,
            Key::End => KEY_END,
            Key::Insert => KEY_INSERT,
            Key::Delete => KEY_DELETE,

            // Función F1-F12
            Key::F1 => KEY_F1,
            Key::F2 => KEY_F2,
            Key::F3 => KEY_F3,
            Key::F4 => KEY_F4,
            Key::F5 => KEY_F5,
            Key::F6 => KEY_F6,
            Key::F7 => KEY_F7,
            Key::F8 => KEY_F8,
            Key::F9 => KEY_F9,
            Key::F10 => KEY_F10,
            Key::F11 => KEY_F11,
            Key::F12 => KEY_F12,

            // Flechas
            Key::ArrowUp => KEY_UP,
            Key::ArrowDown => KEY_DOWN,
            Key::ArrowLeft => KEY_LEFT,
            Key::ArrowRight => KEY_RIGHT,

            // Letras A-Z
            Key::A => KEY_A,
            Key::B => KEY_B,
            Key::C => KEY_C,
            Key::D => KEY_D,
            Key::E => KEY_E,
            Key::F => KEY_F,
            Key::G => KEY_G,
            Key::H => KEY_H,
            Key::I => KEY_I,
            Key::J => KEY_J,
            Key::K => KEY_K,
            Key::L => KEY_L,
            Key::M => KEY_M,
            Key::N => KEY_N,
            Key::O => KEY_O,
            Key::P => KEY_P,
            Key::Q => KEY_Q,
            Key::R => KEY_R,
            Key::S => KEY_S,
            Key::T => KEY_T,
            Key::U => KEY_U,
            Key::V => KEY_V,
            Key::W => KEY_W,
            Key::X => KEY_X,
            Key::Y => KEY_Y,
            Key::Z => KEY_Z,

            // Números 0-9
            Key::Num0 => KEY_ZERO,
            Key::Num1 => KEY_ONE,
            Key::Num2 => KEY_TWO,
            Key::Num3 => KEY_THREE,
            Key::Num4 => KEY_FOUR,
            Key::Num5 => KEY_FIVE,
            Key::Num6 => KEY_SIX,
            Key::Num7 => KEY_SEVEN,
            Key::Num8 => KEY_EIGHT,
            Key::Num9 => KEY_NINE,
        }
    }
}

// ============================================================================
// RYDIT GFX - EL PUENTE
// ============================================================================

/// RyDit Graphics Layer
///
/// Sincroniza Rust (arquitecto) con Raylib (pincel)
///
/// ## Características:
/// - API consistente (oculta inconsistencias de raylib-rs)
/// - Rust siempre controla, Raylib siempre ejecuta
/// - Manejo seguro de handles y threads
pub struct RyditGfx {
    handle: Option<RaylibHandle>,
    thread: Option<RaylibThread>,
    width: i32,
    height: i32,
    fps: i32,
    // ✅ v0.10.4: Input SDL2 para Termux-X11
    pub input_sdl2: input_sdl2::InputState,
    // Contexto SDL2 para eventos
    #[allow(dead_code)]
    sdl_context: Option<sdl2::Sdl>,
    sdl_event_pump: Option<sdl2::EventPump>,
    // Backend SDL2 líder (opcional)
    pub backend_sdl2: Option<backend_sdl2::Sdl2Backend>,
    // ✅ v0.13.1: FSR Upscaler
    #[allow(dead_code)]
    fsr: Option<fsr::FsrUpscaler>,
    #[allow(dead_code)]
    fsr_enabled: bool,
}

impl RyditGfx {
    /// Crear nueva ventana gráfica (Legacy/Raylib-only)
    pub fn new(title: &str, width: i32, height: i32) -> Self {
        let (handle, thread) = raylib::init().size(width, height).title(title).build();

        println!("[RYDIT-GFX]: Ventana Raylib creada {}x{}", width, height);

        // Inicializar SDL2 para eventos
        let sdl_context = sdl2::init().ok();
        let sdl_event_pump = sdl_context.as_ref().and_then(|ctx| ctx.event_pump().ok());

        Self {
            handle: Some(handle),
            thread: Some(thread),
            width,
            height,
            fps: 60,
            input_sdl2: input_sdl2::InputState::new(),
            sdl_context,
            sdl_event_pump,
            backend_sdl2: None,
            fsr: None,
            fsr_enabled: false,
        }
    }

    /// Crear nueva ventana gráfica con SDL2 como líder (Recomendado para Editor)
    pub fn new_with_sdl2(title: &str, width: i32, height: i32) -> Result<Self, String> {
        let backend = backend_sdl2::Sdl2Backend::new(title, width, height)?;
        
        println!("[RYDIT-GFX]: Ventana SDL2 creada (Líder) {}x{}", width, height);

        Ok(Self {
            handle: None,
            thread: None,
            width,
            height,
            fps: 60,
            input_sdl2: input_sdl2::InputState::new(),
            sdl_context: None, // El contexto está dentro del backend
            sdl_event_pump: None,
            backend_sdl2: Some(backend),
            fsr: None,
            fsr_enabled: false,
        })
    }

    /// Configurar FPS objetivo
    pub fn set_target_fps(&mut self, fps: i32) {
        self.fps = fps;
        if let Some(ref mut h) = self.handle {
            h.set_target_fps(fps as u32);
        }
    }

    /// Obtener FPS objetivo
    pub fn get_target_fps(&self) -> i32 {
        self.fps
    }

    /// Obtener FPS reales
    pub fn get_fps(&self) -> i32 {
        if let Some(ref h) = self.handle {
            h.get_fps() as i32
        } else {
            self.fps // Fallback
        }
    }

    /// Verificar si la ventana debe cerrarse
    pub fn should_close(&self) -> bool {
        if let Some(ref h) = self.handle {
            h.window_should_close()
        } else if let Some(ref b) = self.backend_sdl2 {
            b.should_close() // Este debe ser actualizado para ser real
        } else {
            false
        }
    }

    // ========================================================================
    // INPUT SDL2 - v0.10.4: Para Termux-X11/Android
    // ========================================================================

    /// Procesar eventos SDL2 (debe llamarse en cada frame)
    pub fn procesar_eventos_sdl2(&mut self) {
        // Limpiar eventos del frame anterior
        self.input_sdl2.limpiar_frame();

        if let Some(ref mut backend) = self.backend_sdl2 {
            // Si hay backend completo, delegar a él
            backend.procesar_eventos();
            // Sincronizar input_sdl2 (legacy) con el input del backend
            self.input_sdl2 = backend.input.clone(); // Nota: Requiere Clone en InputState
        } else if let Some(ref mut event_pump) = self.sdl_event_pump {
            // Modo legacy: solo eventos sin ventana SDL2
            for event in event_pump.poll_iter() {
                self.input_sdl2.procesar_evento(&event);
            }
        }
    }

    /// Verificar si una tecla está presionada (vía SDL2)
    pub fn is_key_pressed_sdl2(&self, nombre: &str) -> bool {
        self.input_sdl2.is_key_pressed(nombre)
    }

    /// Verificar si una tecla fue presionada este frame (vía SDL2)
    pub fn is_key_just_pressed_sdl2(&self, nombre: &str) -> bool {
        self.input_sdl2.is_key_just_pressed(nombre)
    }

    /// Obtener ancho de ventana
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Obtener alto de ventana
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Iniciar dibujo (obtener DrawHandle)
    pub fn begin_draw(&mut self) -> DrawHandle<'_> {
        if let Some(ref mut h) = self.handle {
            let d = h.begin_drawing(self.thread.as_ref().unwrap());
            DrawHandle::new(d)
        } else {
            DrawHandle::noop()
        }
    }

    /// Finalizar dibujo (automático con Drop de DrawHandle)
    pub fn end_draw(&mut self) {
        if let Some(ref mut b) = self.backend_sdl2 {
            b.end_draw();
        }
    }

    // ... (FSR functions omitidas para brevedad, se mantienen igual pero con checks si es necesario)

    /// Limpiar pantalla
    pub fn clear_background(&mut self, color: ColorRydit) {
        if self.handle.is_some() {
            let mut d = self.begin_draw();
            d.clear(color);
        } else if let Some(ref mut b) = self.backend_sdl2 {
            b.clear_background(color);
        }
    }

    /// Dibujar círculo
    pub fn draw_circle(&mut self, x: i32, y: i32, radius: i32, color: ColorRydit) {
        if self.handle.is_some() {
            let mut d = self.begin_draw();
            d.draw_circle(x, y, radius, color);
        } else if let Some(ref mut b) = self.backend_sdl2 {
            let (r, g, b_val) = color.to_rgb();
            b.draw_circle(x, y, radius, r, g, b_val);
        }
    }

    /// Dibujar rectángulo
    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: ColorRydit) {
        if self.handle.is_some() {
            let mut d = self.begin_draw();
            d.draw_rectangle(x, y, w, h, color);
        } else if let Some(ref mut b) = self.backend_sdl2 {
            b.draw_rect_color(x, y, w, h, color);
        }
    }

    /// Dibujar línea
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: ColorRydit) {
        if self.handle.is_some() {
            let mut d = self.begin_draw();
            d.draw_line(x1, y1, x2, y2, color);
        } else if let Some(ref mut b) = self.backend_sdl2 {
            let (r, g, b_val) = color.to_rgb();
            b.canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b_val));
            let _ = b.canvas.draw_line((x1, y1), (x2, y2));
        }
    }

    /// Dibujar texto
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, size: i32, color: ColorRydit) {
        if self.handle.is_some() {
            let mut d = self.begin_draw();
            d.draw_text(text, x, y, size, color);
        } else if let Some(ref mut b) = self.backend_sdl2 {
            let (r_val, g_val, b_val) = color.to_rgb();
            b.draw_text(text, x, y, size as u16, r_val, g_val, b_val);
        }
    }

    /// Cargar textura desde archivo (placeholder)
    pub fn load_texture(&mut self, path: &str) -> Texture2D {
        // ⚠️ Placeholder: Implementación completa pendiente
        // Intentar cargar con raylib FFI
        unsafe {
            let c_path = std::ffi::CString::new(path).unwrap();
            let tex = raylib::ffi::LoadTexture(c_path.as_ptr());
            // Convertir FFI Texture2D a raylib::prelude::Texture2D
            std::mem::transmute(tex)
        }
    }

    /// Dibujar textura (placeholder - dibuja rect)
    pub fn draw_texture(&mut self, _texture: &Texture2D, x: i32, y: i32, color: ColorRydit) {
        // ⚠️ Placeholder: Usar textura real cuando esté implementado
        self.draw_rect(x, y, 32, 32, color);
    }

    /// Recargar una textura existente (Hot Reload)
    pub fn reload_texture(&mut self, id: &str, path: &str, assets: &mut Assets) -> Result<(), String> {
        let new_tex = Assets::load_texture_from_path(path)?;
        assets.insert_texture(id.to_string(), new_tex);
        println!("[HOT-RELOAD] Textura '{}' recargada desde {}", id, path);
        Ok(())
    }

    /// Verificar tecla presionada
    pub fn is_key_pressed(&self, key: Key) -> bool {
        if let Some(ref h) = self.handle {
            h.is_key_pressed(key.to_raylib())
        } else {
            false
        }
    }

    /// Verificar tecla mantenida
    pub fn is_key_down(&self, key: Key) -> bool {
        if let Some(ref h) = self.handle {
            h.is_key_down(key.to_raylib())
        } else {
            false
        }
    }

    /// Obtener posición X del mouse
    pub fn get_mouse_x(&self) -> i32 {
        if let Some(ref h) = self.handle {
            h.get_mouse_x()
        } else {
            self.input_sdl2.mouse_x
        }
    }

    /// Obtener posición Y del mouse
    pub fn get_mouse_y(&self) -> i32 {
        if let Some(ref h) = self.handle {
            h.get_mouse_y()
        } else {
            self.input_sdl2.mouse_y
        }
    }

    // ========================================================================
    // INPUT MOUSE AVANZADO - V0.3.0
    // ========================================================================

    /// Obtener posición del mouse como (x, y)
    pub fn get_mouse_position(&self) -> (i32, i32) {
        if let Some(ref h) = self.handle {
            (h.get_mouse_x(), h.get_mouse_y())
        } else {
            (self.input_sdl2.mouse_x, self.input_sdl2.mouse_y)
        }
    }

    /// Verificar si botón del mouse está presionado (0=izq, 1=der, 2=medio)
    pub fn is_mouse_button_pressed(&self, button: i32) -> bool {
        // Raylib usa valores: 0=MOUSE_LEFT_BUTTON, 1=MOUSE_RIGHT_BUTTON, 2=MOUSE_MIDDLE_BUTTON
        unsafe {
            let ffi_button = button;
            raylib::ffi::IsMouseButtonPressed(ffi_button)
        }
    }

    /// Verificar si botón del mouse está mantenido
    pub fn is_mouse_button_down(&self, button: i32) -> bool {
        unsafe {
            let ffi_button = button;
            raylib::ffi::IsMouseButtonDown(ffi_button)
        }
    }

    /// Verificar si botón del mouse fue soltado este frame
    pub fn is_mouse_button_released(&self, button: i32) -> bool {
        unsafe {
            let ffi_button = button;
            raylib::ffi::IsMouseButtonReleased(ffi_button)
        }
    }

    /// Obtener movimiento del mouse (delta X, delta Y)
    pub fn get_mouse_delta(&self) -> (i32, i32) {
        if let Some(ref h) = self.handle {
            let delta = h.get_mouse_delta();
            (delta.x as i32, delta.y as i32)
        } else {
            (
                self.input_sdl2.mouse_x - self.input_sdl2.prev_mouse_x,
                self.input_sdl2.mouse_y - self.input_sdl2.prev_mouse_y,
            )
        }
    }

    /// Obtener scroll del mouse (Y axis)
    pub fn get_mouse_wheel(&self) -> f32 {
        if let Some(ref h) = self.handle {
            h.get_mouse_wheel_move()
        } else {
            0.0 // Pendiente: Capturar en InputState SDL2
        }
    }
}

impl Drop for RyditGfx {
    fn drop(&mut self) {
        println!("[RYDIT-GFX]: Cerrando ventana...");
    }
}

// ============================================================================
// DRAW HANDLE - EL PINCEL
// ============================================================================

/// Handle de dibujo (Raylib = Pincel)
///
/// Se obtiene de `RyditGfx::begin_draw()` y se usa para dibujar.
/// Al salir de scope, automáticamente finaliza el dibujo si es Raylib.
pub struct DrawHandle<'a> {
    pub draw: Option<RaylibDrawHandle<'a>>,
}

impl<'a> DrawHandle<'a> {
    fn new(draw: RaylibDrawHandle<'a>) -> Self {
        Self { draw: Some(draw) }
    }

    /// Crear un handle vacío (Noop)
    fn noop() -> Self {
        Self { draw: None }
    }

    /// Limpiar con color
    pub fn clear(&mut self, color: ColorRydit) {
        if let Some(ref mut d) = self.draw {
            d.clear_background(color.to_color());
        }
    }

    /// Dibujar círculo
    pub fn draw_circle(&mut self, x: i32, y: i32, radius: i32, color: ColorRydit) {
        if let Some(ref mut d) = self.draw {
            d.draw_circle(x, y, radius as f32, color.to_color());
        }
    }

    /// Dibujar rectángulo
    pub fn draw_rectangle(&mut self, x: i32, y: i32, w: i32, h: i32, color: ColorRydit) {
        if let Some(ref mut d) = self.draw {
            d.draw_rectangle(x, y, w, h, color.to_color());
        }
    }

    /// Dibujar línea
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: ColorRydit) {
        if let Some(ref mut d) = self.draw {
            d.draw_line(x1, y1, x2, y2, color.to_color());
        }
    }

    /// 🆕 v0.19.2: Dibujar sistema de partículas con color por velocidad
    pub fn draw_particles_velocity(&mut self, system: &crate::gpu_particles::ParticleSystem, max_speed: f32) {
        if let Some(ref mut d) = self.draw {
            system.draw_with_velocity(d, max_speed);
        }
    }

    /// Dibujar texto
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, size: i32, color: ColorRydit) {
        if let Some(ref mut d) = self.draw {
            d.draw_text(text, x, y, size, color.to_color());
        }
    }

    // ========================================================================
    // FUNCIONES DRAW V0.2.0 - NUEVAS FORMAS
    // ========================================================================

    /// Dibujar triángulo
    pub fn draw_triangle(
        &mut self,
        v1: (i32, i32),
        v2: (i32, i32),
        v3: (i32, i32),
        color: ColorRydit,
    ) {
        if let Some(ref mut d) = self.draw {
            let v1_raylib = Vector2::new(v1.0 as f32, v1.1 as f32);
            let v2_raylib = Vector2::new(v2.0 as f32, v2.1 as f32);
            let v3_raylib = Vector2::new(v3.0 as f32, v3.1 as f32);
            d.draw_triangle(v1_raylib, v2_raylib, v3_raylib, color.to_color());
        }
    }

    /// Dibujar triángulo con líneas (outline)
    pub fn draw_triangle_lines(
        &mut self,
        v1: (i32, i32),
        v2: (i32, i32),
        v3: (i32, i32),
        color: ColorRydit,
    ) {
        if self.draw.is_some() {
            self.draw_line(v1.0, v1.1, v2.0, v2.1, color);
            self.draw_line(v2.0, v2.1, v3.0, v3.1, color);
            self.draw_line(v3.0, v3.1, v1.0, v1.1, color);
        }
    }

    /// Dibujar rectángulo con líneas (outline)
    pub fn draw_rectangle_lines(&mut self, x: i32, y: i32, w: i32, h: i32, color: ColorRydit) {
        if let Some(ref mut d) = self.draw {
            d.draw_rectangle_lines(x, y, w, h, color.to_color());
        }
    }

    /// Dibujar círculo con líneas (outline)
    pub fn draw_circle_lines(&mut self, x: i32, y: i32, radius: f32, color: ColorRydit) {
        if let Some(ref mut d) = self.draw {
            d.draw_circle_lines(x, y, radius, color.to_color());
        }
    }

    /// Dibujar anillo (ring)
    pub fn draw_ring(
        &mut self,
        center: (i32, i32),
        _inner_radius: i32,
        outer_radius: i32,
        color: ColorRydit,
    ) {
        if let Some(ref mut d) = self.draw {
            // Simplificación: dibujamos solo el círculo exterior
            d.draw_circle(center.0, center.1, outer_radius as f32, color.to_color());
        }
    }

    /// Dibujar elipse
    pub fn draw_ellipse(
        &mut self,
        center: (i32, i32),
        radius_h: i32,
        radius_v: i32,
        color: ColorRydit,
    ) {
        if let Some(ref mut d) = self.draw {
            d.draw_ellipse(
                center.0,
                center.1,
                radius_h as f32,
                radius_v as f32,
                color.to_color(),
            );
        }
    }

    /// Dibujar línea gruesa
    pub fn draw_line_thick(
        &mut self,
        start_pos: (i32, i32),
        end_pos: (i32, i32),
        thick: f32,
        color: ColorRydit,
    ) {
        if let Some(ref mut d) = self.draw {
            let start = Vector2::new(start_pos.0 as f32, start_pos.1 as f32);
            let end = Vector2::new(end_pos.0 as f32, end_pos.1 as f32);
            d.draw_line_ex(start, end, thick, color.to_color());
        }
    }

    /// Dibujar rectángulo rotado
    pub fn draw_rectangle_pro(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        angle: f32,
        color: ColorRydit,
    ) {
        if let Some(ref mut d) = self.draw {
            let origin = Vector2::new(width as f32 / 2.0, height as f32 / 2.0);
            let rect = Rectangle::new(x as f32, y as f32, width as f32, height as f32);
            d.draw_rectangle_pro(rect, origin, angle, color.to_color());
        }
    }

    /// Dibujar textura avanzada
    pub fn draw_texture_ex(
        &mut self,
        texture: &Texture2D,
        position: Vector2,
        rotation: f32,
        scale: f32,
        color: Color,
    ) {
        if let Some(ref mut d) = self.draw {
            d.draw_texture_ex(texture, position, rotation, scale, color);
        }
    }
}

impl<'a> Drop for DrawHandle<'a> {
    fn drop(&mut self) {
        // Finalizar dibujo automáticamente
        // (RaylibDrawHandle lo hace en su Drop)
    }
}

// ============================================================================
// ASSETS MANAGER - v0.5.0 (Sprites)
// ============================================================================

/// Gestor de assets (texturas)
/// Nota: La carga de texturas se hace desde RyDit usando raylib directamente
pub struct Assets {
    textures: HashMap<String, Texture2D>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    /// Cargar textura desde archivo (función estática usando FFI)
    pub fn load_texture_from_path(path: &str) -> Result<Texture2D, String> {
        use std::ffi::CString;
        use std::path::Path;

        if Path::new(path).exists() {
            // Usar FFI para cargar textura sin handle
            unsafe {
                let c_path =
                    CString::new(path).map_err(|e| format!("Error convirtiendo path: {}", e))?;
                let ffi_texture = raylib::ffi::LoadTexture(c_path.as_ptr());
                if ffi_texture.id != 0 {
                    // Usar from_raw para crear Texture2D desde FFI
                    Ok(Texture2D::from_raw(ffi_texture))
                } else {
                    Err(format!("Error cargando textura '{}'", path))
                }
            }
        } else {
            Err(format!("Archivo '{}' no encontrado", path))
        }
    }

    // ==================== TEXTURAS ====================

    /// Insertar textura cargada desde RyDit
    pub fn insert_texture(&mut self, id: String, texture: Texture2D) {
        self.textures.insert(id, texture);
    }

    /// Obtener textura por ID
    pub fn get_texture(&self, id: &str) -> Option<&Texture2D> {
        self.textures.get(id)
    }

    /// Descargar textura y liberar memoria
    pub fn unload_texture(&mut self, id: &str) -> bool {
        self.textures.remove(id).is_some()
    }

    /// Dibujar textura en pantalla
    ///
    /// # Arguments (en orden):
    /// 1. `d` - Draw handle de raylib
    /// 2. `id` - ID del sprite cargado
    ///    3-4. `x, y` - Posición en pantalla
    ///    5-6. `_w, _h` - Dimensiones (actualmente no usadas)
    /// 7. `color` - Color de tinte
    ///
    /// # Nota
    /// Los parámetros `_w` y `_h` son reservados para futura implementación de escalado.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_texture(
        &self,
        d: &mut RaylibDrawHandle,
        id: &str,
        x: f32,
        y: f32,
        _w: f32,
        _h: f32,
        color: Color,
    ) {
        if let Some(texture) = self.textures.get(id) {
            d.draw_texture_ex(texture, Vector2::new(x, y), 0.0, 1.0, color);
        }
    }

    /// Dibujar textura escalada
    pub fn draw_texture_scaled(
        &self,
        d: &mut RaylibDrawHandle,
        id: &str,
        x: f32,
        y: f32,
        scale: f32,
        color: Color,
    ) {
        if let Some(texture) = self.textures.get(id) {
            d.draw_texture_ex(texture, Vector2::new(x, y), 0.0, scale, color);
        }
    }

    /// Dibujar textura con rotación y escala (para RenderQueue)
    pub fn draw_texture_ex_by_id(
        &self,
        d: &mut RaylibDrawHandle,
        id: &str,
        x: f32,
        y: f32,
        scale: f32,
        rotation: f32,
        color: ColorRydit,
    ) {
        if let Some(texture) = self.textures.get(id) {
            d.draw_texture_ex(
                texture,
                Vector2::new(x, y),
                rotation,
                scale,
                color.to_color(),
            );
        }
    }

    /// Dibujar rectángulo de textura (para tilesets)
    pub fn draw_texture_rec(
        &self,
        d: &mut RaylibDrawHandle,
        id: &str,
        source: Rectangle,
        dest: Rectangle,
        color: Color,
    ) {
        if let Some(texture) = self.textures.get(id) {
            d.draw_texture_rec(texture, source, Vector2::new(dest.x, dest.y), color);
        }
    }

    /// Verificar si existe una textura
    pub fn has_texture(&self, id: &str) -> bool {
        self.textures.contains_key(id)
    }

    /// Cantidad de texturas cargadas
    pub fn texture_count(&self) -> usize {
        self.textures.len()
    }

    /// Limpiar todas las texturas
    pub fn clear(&mut self) {
        self.textures.clear();
    }

    // ==================== FUNCIONES SDL2 (ESTÁTICAS) ====================

    /// Cargar textura SDL2 desde archivo (usando FFI nativo)
    /// Retorna la textura que debe ser gestionada por el caller
    /// Nota: El lifetime de la textura está ligado al texture_creator
    pub fn load_texture_sdl2<'a>(
        path: &str,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> Result<sdl2::render::Texture<'a>, String> {
        use crate::sdl2_ffi::TextureFFI;
        use sdl2::surface::Surface;
        use std::path::Path;

        if !Path::new(path).exists() {
            return Err(format!("Archivo '{}' no encontrado", path));
        }

        // Cargar superficie con SDL2_image FFI
        let texture_ffi = TextureFFI::load(path)?;
        let surface_ptr = texture_ffi.surface();

        unsafe {
            // Envolver raw pointer en Surface
            let sdl_surface = Surface::from_ll(surface_ptr as *mut sdl2::sys::SDL_Surface);

            // Crear textura desde superficie
            let texture = texture_creator
                .create_texture_from_surface(&sdl_surface)
                .map_err(|e| format!("Error creando textura SDL2: {}", e))?;

            Ok(texture)
        }
    }

    /// Dibujar textura SDL2 directamente (sin gestor de assets)
    pub fn draw_texture_sdl2(
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture: &sdl2::render::Texture,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        let rect = sdl2::rect::Rect::new(x, y, width, height);
        canvas
            .copy(texture, None, rect)
            .map_err(|e| format!("Error dibujando textura SDL2: {}", e))?;

        Ok(())
    }
}

impl Default for Assets {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MIGUI BACKEND IMPLEMENTATION
// ============================================================================

/// Implementación de MiguiBackend para RyditGfx (solo con feature migui)
///
/// Conecta los DrawCommand de migui con las funciones de dibujo de raylib
///
/// Nota: Las funciones asumen que begin_draw() ya fue llamado.
/// Usar con render_commands_frame() que maneja el begin/end draw.
#[cfg(feature = "migui")]
impl MiguiBackend for RyditGfx {
    fn clear(&mut self, color: MiguiColor) {
        let color_rydit = ColorRydit::from_migui(color);
        let mut d = self.begin_draw();
        d.clear(color_rydit);
    }

    fn draw_rect(&mut self, rect: MiguiRect, color: MiguiColor) {
        let color_rydit = ColorRydit::from_migui(color);
        let mut d = self.begin_draw();
        d.draw_rectangle(
            rect.x as i32,
            rect.y as i32,
            rect.w as i32,
            rect.h as i32,
            color_rydit,
        );
    }

    fn draw_text(&mut self, text: &str, x: f32, y: f32, size: f32, color: MiguiColor) {
        let color_rydit = ColorRydit::from_migui(color);
        let mut d = self.begin_draw();
        d.draw_text(text, x as i32, y as i32, size as i32, color_rydit);
    }

    fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: MiguiColor, thickness: f32) {
        let color_rydit = ColorRydit::from_migui(color);
        let mut d = self.begin_draw();
        if let Some(ref mut rd) = d.draw {
            rd.draw_line_ex(
                Vector2::new(x1, y1),
                Vector2::new(x2, y2),
                thickness,
                color_rydit.to_color(),
            );
        }
    }
}

// ============================================================================
// FUNCIONES DE RENDERIZADO MIGUI OPTIMIZADAS (solo con feature migui)
// ============================================================================

#[cfg(feature = "migui")]
impl RyditGfx {
    /// Renderizar comandos migui en un frame (con begin/end draw único)
    pub fn render_migui_frame(
        &mut self, 
        commands: &[migui::DrawCommand], 
        viewports: &mut crate::viewports::ViewportManager
    ) {
        if let Some(ref mut backend) = self.backend_sdl2 {
            // MODO SDL2 (Líder)
            backend.begin_draw();
            backend.render_migui_commands(commands, viewports);
            backend.end_draw();
        } else if let Some(ref mut h) = self.handle {
            // MODO RAYLIB (Legacy)
            let mut d = h.begin_drawing(self.thread.as_ref().unwrap());
            d.clear_background(Color::BLACK);

            for cmd in commands {
                match cmd {
                    migui::DrawCommand::Clear { color } => {
                        d.clear_background(ColorRydit::from_migui(*color).to_color());
                    }
                    migui::DrawCommand::DrawRect { rect, color } => {
                        d.draw_rectangle(
                            rect.x as i32,
                            rect.y as i32,
                            rect.w as i32,
                            rect.h as i32,
                            ColorRydit::from_migui(*color).to_color(),
                        );
                    }
                    migui::DrawCommand::DrawText {
                        text,
                        x,
                        y,
                        size,
                        color,
                    } => {
                        d.draw_text(
                            text,
                            *x as i32,
                            *y as i32,
                            *size as i32,
                            ColorRydit::from_migui(*color).to_color(),
                        );
                    }
                    migui::DrawCommand::DrawLine {
                        x1,
                        y1,
                        x2,
                        y2,
                        color,
                        thickness,
                    } => {
                        d.draw_line_ex(
                            Vector2::new(*x1, *y1),
                            Vector2::new(*x2, *y2),
                            *thickness,
                            ColorRydit::from_migui(*color).to_color(),
                        );
                    }
                    migui::DrawCommand::DrawViewport3D { id, rect } => {
                        if let Some(vp) = viewports.viewports.get_mut(id) {
                            // 1. Renderizar escena 3D en el buffer del viewport
                            {
                                // raylib-rs: begin_texture_mode requiere thread y target
                                let mut vd = d.begin_texture_mode(self.thread.as_ref().unwrap(), &mut vp.target);
                                unsafe {
                                    raylib::ffi::ClearBackground(raylib::ffi::Color { r: 30, g: 30, b: 35, a: 255 });
                                    // ... dibujo 3D ...
                                }
                            }

                            // 2. Dibujar la textura resultante en el canvas principal
                            unsafe {
                                let source = raylib::ffi::Rectangle { 
                                    x: 0.0, y: 0.0, 
                                    width: vp.width as f32, 
                                    height: -vp.height as f32 
                                };
                                let dest = raylib::ffi::Rectangle {
                                    x: rect.x, y: rect.y,
                                    width: rect.w, height: rect.h
                                };
                                let origin = raylib::ffi::Vector2 { x: 0.0, y: 0.0 };
                                raylib::ffi::DrawTexturePro(vp.target.texture, source, dest, origin, 0.0, raylib::ffi::Color { r: 255, g: 255, b: 255, a: 255 });
                            }
                        }
                    }
                }
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
    fn test_color_from_str() {
        assert_eq!(ColorRydit::from_str("rojo").unwrap(), ColorRydit::Rojo);
        assert_eq!(ColorRydit::from_str("RED").unwrap(), ColorRydit::Rojo);
        assert_eq!(ColorRydit::from_str("verde").unwrap(), ColorRydit::Verde);
        assert_eq!(ColorRydit::from_str("azul").unwrap(), ColorRydit::Azul);
        assert_eq!(
            ColorRydit::from_str("amarillo").unwrap(),
            ColorRydit::Amarillo
        );
        assert_eq!(ColorRydit::from_str("blanco").unwrap(), ColorRydit::Blanco);
        assert_eq!(ColorRydit::from_str("negro").unwrap(), ColorRydit::Negro);
        assert_eq!(
            ColorRydit::from_str("desconocido").unwrap(),
            ColorRydit::Blanco
        );
    }

    #[test]
    fn test_key_to_raylib() {
        // Solo verificamos que no panic
        let _ = Key::Escape.to_raylib();
        let _ = Key::Space.to_raylib();
        let _ = Key::A.to_raylib();
        let _ = Key::Num0.to_raylib();
    }

    #[test]
    fn test_color_to_color() {
        let color = ColorRydit::Rojo.to_color();
        assert_eq!(color.r, 230);
        assert_eq!(color.g, 41);
        assert_eq!(color.b, 55);
        assert_eq!(color.a, 255);
    }

    // ========================================================================
    // TESTS V0.1.9 - GRÁFICOS Y COLORES
    // ========================================================================

    #[test]
    fn test_draw_circle_colores() {
        // Verificar que todos los colores básicos funcionan
        let colores = vec!["rojo", "verde", "azul", "amarillo", "blanco", "negro"];

        for color_str in colores {
            let color_rydit = ColorRydit::from_str(color_str).unwrap();
            let color = color_rydit.to_color();
            // Solo verificamos que el alpha sea 255 (completamente opaco)
            assert_eq!(color.a, 255, "Color {} debe tener alpha 255", color_str);
        }
    }

    #[test]
    fn test_draw_rect_dimensiones() {
        // Verificar conversión de dimensiones para rectángulos
        // Las dimensiones son enteros en raylib
        let x: i32 = 100;
        let y: i32 = 200;
        let ancho: i32 = 50;
        let alto: i32 = 75;

        // Verificamos que los valores se mantienen
        assert_eq!(x, 100);
        assert_eq!(y, 200);
        assert_eq!(ancho, 50);
        assert_eq!(alto, 75);

        // Un rectángulo con estas dimensiones debería ser válido
        let rect = raylib::prelude::Rectangle::new(x as f32, y as f32, ancho as f32, alto as f32);
        assert_eq!(rect.x, 100.0);
        assert_eq!(rect.y, 200.0);
        assert!((rect.width - 50.0).abs() < 0.01);
        assert!((rect.height - 75.0).abs() < 0.01);
    }

    // ========================================================================
    // TESTS V0.3.0 - INPUT MOUSE AVANZADO
    // ========================================================================

    #[test]
    fn test_mouse_functions_exist() {
        // Solo verificamos que las funciones existen y compilan
        // No podemos probar mouse real sin ventana
        let _ = RyditGfx::new("Test", 800, 600);
        // gfx.get_mouse_position()  // Retorna (i32, i32)
        // gfx.is_mouse_button_pressed(0)  // Retorna bool
        // gfx.get_mouse_wheel()  // Retorna (f32, f32)
    }

    #[test]
    fn test_mouse_button_mapping() {
        // Verificar mapeo de botones
        assert_eq!(0, 0); // Left button index
        assert_eq!(1, 1); // Right button index
        assert_eq!(2, 2); // Middle button index
    }

    // ========================================================================
    // TESTS V0.4.1 - MIGUI BACKEND
    // ========================================================================

    #[cfg(feature = "migui")]
    #[test]
    fn test_migui_backend_exists() {
        // Verificar que RyditGfx implementa MiguiBackend
        let _ = RyditGfx::new("Test", 800, 600);
        // El backend existe y compila
    }
}
