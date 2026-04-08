//! 🛡️ V-Shield - Platform Layer + Sync Primitives for Ry-Dit
//!
//! **V-Shield** provee:
//!
//! - 🎨 **Colores** constantes para RyDit (con o sin raylib)
//! - 🔒 **Sync Primitives** multiplataforma (Mutex, RwLock, Barrier, Condvar)
//! - 🖥️ **Platform Detection** (Linux, Windows, macOS, Android, iOS, WASM)
//! - 🔄 **Platform Sync** para sincronización de renderizado
//!
//! # Features
//!
//! | Feature | Descripción | Dependencias |
//! |---------|-------------|--------------|
//! | `native` (default) | Sync nativo (std::sync) | Ninguna extra |
//! | `wasm` | Sync para WASM | Ninguna extra |
//! | `graphics` (default) | Colores raylib + window init | raylib |
//! | `async-tokio` | Wrappers async | tokio (solo sync) |
//! | `rt-linux` | Linux real-time | rtsc |
//!
//! # Ejemplo rápido
//!
//! ```rust
//! use v_shield::sync::{Mutex, RwLock};
//! use v_shield::platform::{current_platform, PlatformConfig};
//! use v_shield::platform_sync::PlatformSync;
//!
//! // Platform detection
//! let p = current_platform();
//! println!("Running on: {}", p.name());
//!
//! // Config defaults
//! let config = PlatformConfig::for_current();
//!
//! // Sync primitives
//! let data = Mutex::new(vec![1, 2, 3]);
//! let cache = RwLock::new(String::new());
//!
//! // Platform sync para renderizado
//! let mut sync = PlatformSync::new();
//! // sync.sync(); // al final de cada frame
//! ```
//!
//! # Para Termux/Android
//!
//! Funciona sin cambios en Termux. Solo necesitas:
//! ```bash
//! pkg install clang libx11-dev libxcb-dev
//! ```

// ============================================================================
// Platform Detection
// ============================================================================
pub mod platform;

// ============================================================================
// Platform Sync (renderizado)
// ============================================================================
pub mod platform_sync;

// ============================================================================
// Sync Primitives
// ============================================================================
pub mod sync;

// ============================================================================
// Graphics (colores + window) - solo con feature "graphics"
// ============================================================================
#[cfg(feature = "graphics")]
mod graphics {
    pub use raylib;
    pub use raylib::consts::KeyboardKey::*;
    pub use raylib::prelude::*;

    use std::str::FromStr;

    // Definir colores manualmente (raylib nobuild no incluye colors::prelude)
    pub const RED: Color = Color {
        r: 230, g: 41, b: 55, a: 255,
    };
    pub const GREEN: Color = Color {
        r: 117, g: 203, b: 100, a: 255,
    };
    pub const BLUE: Color = Color {
        r: 51, g: 122, b: 206, a: 255,
    };
    pub const YELLOW: Color = Color {
        r: 253, g: 249, b: 0, a: 255,
    };
    pub const WHITE: Color = Color {
        r: 255, g: 255, b: 255, a: 255,
    };
    pub const BLACK: Color = Color {
        r: 0, g: 0, b: 0, a: 255,
    };
    pub const CYAN: Color = Color {
        r: 0, g: 255, b: 255, a: 255,
    };
    pub const MAGENTA: Color = Color {
        r: 255, g: 0, b: 255, a: 255,
    };
    pub const ORANGE: Color = Color {
        r: 255, g: 165, b: 0, a: 255,
    };
    pub const PINK: Color = Color {
        r: 255, g: 192, b: 203, a: 255,
    };
    pub const PURPLE: Color = Color {
        r: 128, g: 0, b: 128, a: 255,
    };
    pub const BROWN: Color = Color {
        r: 165, g: 42, b: 42, a: 255,
    };
    pub const GRAY: Color = Color {
        r: 128, g: 128, b: 128, a: 255,
    };
    pub const LIME: Color = Color {
        r: 0, g: 255, b: 0, a: 255,
    };
    pub const NAVY: Color = Color {
        r: 0, g: 0, b: 128, a: 255,
    };
    pub const OLIVE: Color = Color {
        r: 128, g: 128, b: 0, a: 255,
    };
    pub const TEAL: Color = Color {
        r: 0, g: 128, b: 128, a: 255,
    };
    pub const MAROON: Color = Color {
        r: 128, g: 0, b: 0, a: 255,
    };

    /// Colores básicos para RyDit
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum ColorRyDit {
        Rojo, Verde, Azul, Amarillo, Blanco, Negro,
        Cyan, Magenta, Naranja, Rosa, Morado, Cafe,
        Gris, Lima, AzulOscuro, Oliva, Turquesa, Vino,
    }

    impl ColorRyDit {
        pub fn to_color(&self) -> Color {
            match self {
                Self::Rojo => RED, Self::Verde => GREEN, Self::Azul => BLUE,
                Self::Amarillo => YELLOW, Self::Blanco => WHITE, Self::Negro => BLACK,
                Self::Cyan => CYAN, Self::Magenta => MAGENTA, Self::Naranja => ORANGE,
                Self::Rosa => PINK, Self::Morado => PURPLE, Self::Cafe => BROWN,
                Self::Gris => GRAY, Self::Lima => LIME, Self::AzulOscuro => NAVY,
                Self::Oliva => OLIVE, Self::Turquesa => TEAL, Self::Vino => MAROON,
            }
        }
    }

    impl FromStr for ColorRyDit {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "rojo" | "red" => Ok(Self::Rojo),
                "verde" | "green" => Ok(Self::Verde),
                "azul" | "blue" => Ok(Self::Azul),
                "amarillo" | "yellow" => Ok(Self::Amarillo),
                "blanco" | "white" => Ok(Self::Blanco),
                "negro" | "black" => Ok(Self::Negro),
                "cyan" | "celeste" => Ok(Self::Cyan),
                "magenta" | "fucsia" => Ok(Self::Magenta),
                "naranja" | "orange" => Ok(Self::Naranja),
                "rosa" | "pink" => Ok(Self::Rosa),
                "morado" | "purple" | "violeta" => Ok(Self::Morado),
                "cafe" | "brown" | "marron" => Ok(Self::Cafe),
                "gris" | "gray" | "grey" => Ok(Self::Gris),
                "lima" | "lime" => Ok(Self::Lima),
                "azuloscuro" | "navy" | "azul oscuro" => Ok(Self::AzulOscuro),
                "oliva" | "olive" => Ok(Self::Oliva),
                "turquesa" | "teal" => Ok(Self::Turquesa),
                "vino" | "maroon" | "granate" => Ok(Self::Vino),
                _ => Ok(Self::Negro),
            }
        }
    }

    /// Inicializar ventana
    pub fn init_window(titulo: &str, w: i32, h: i32) -> (raylib::RaylibHandle, raylib::RaylibThread) {
        raylib::init().size(w, h).title(titulo).build()
    }
}

#[cfg(feature = "graphics")]
pub use graphics::*;

// Re-exports convenience
pub use platform::{current_platform, Platform, PlatformConfig};
pub use platform_sync::{PlatformSync, PlatformSyncMode};
pub use sync::{Mutex, RwLock};

/// Versión del crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// ============================================================================
// Tests de integración
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.2.0");
    }

    #[test]
    fn test_exports() {
        // Verificar que los re-exports funcionan
        let _ = current_platform();
        let _ = PlatformConfig::for_current();
        let _ = PlatformSync::new();
        let _ = Mutex::new(0);
        let _ = RwLock::new(0);
    }

    // Tests de colores (solo con feature graphics)
    #[cfg(feature = "graphics")]
    mod color_tests {
        use super::*;
        use std::str::FromStr;

        #[test]
        fn test_color_from_str() {
            assert_eq!(ColorRyDit::from_str("rojo").unwrap(), ColorRyDit::Rojo);
            assert_eq!(ColorRyDit::from_str("RED").unwrap(), ColorRyDit::Rojo);
            assert_eq!(ColorRyDit::from_str("verde").unwrap(), ColorRyDit::Verde);
            assert_eq!(ColorRyDit::from_str("azul").unwrap(), ColorRyDit::Azul);
            assert_eq!(ColorRyDit::from_str("blanco").unwrap(), ColorRyDit::Blanco);
            assert_eq!(ColorRyDit::from_str("otro").unwrap(), ColorRyDit::Negro);
            assert_eq!(ColorRyDit::from_str("cyan").unwrap(), ColorRyDit::Cyan);
            assert_eq!(ColorRyDit::from_str("magenta").unwrap(), ColorRyDit::Magenta);
        }

        #[test]
        fn test_colores_constantes() {
            assert_eq!(RED.r, 230); assert_eq!(RED.g, 41); assert_eq!(RED.b, 55);
            assert_eq!(GREEN.r, 117); assert_eq!(GREEN.g, 203); assert_eq!(GREEN.b, 100);
            assert_eq!(BLUE.r, 51); assert_eq!(BLUE.g, 122); assert_eq!(BLUE.b, 206);
            assert_eq!(WHITE.r, 255); assert_eq!(WHITE.g, 255); assert_eq!(WHITE.b, 255);
            assert_eq!(BLACK.r, 0); assert_eq!(BLACK.g, 0); assert_eq!(BLACK.b, 0);
        }

        #[test]
        fn test_color_to_color() {
            assert_eq!(ColorRyDit::Rojo.to_color().r, 230);
            assert_eq!(ColorRyDit::Verde.to_color().r, 117);
            assert_eq!(ColorRyDit::Azul.to_color().r, 51);
        }

        #[test]
        fn test_color_desconocido_retorna_negro() {
            assert_eq!(ColorRyDit::from_str("color_raro").unwrap(), ColorRyDit::Negro);
            assert_eq!(ColorRyDit::from_str("").unwrap(), ColorRyDit::Negro);
        }
    }
}
