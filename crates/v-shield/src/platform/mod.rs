//! 🛡️ V-Shield Platform Detection
//!
//! Detección automática de plataforma con config defaults.
//!
//! # Plataformas soportadas
//!
//! | Plataforma | `target_os` | Sync | Graphics |
//! |------------|------------|------|----------|
//! | Linux | `linux` | `std::sync` | X11/Wayland |
//! | Windows | `windows` | `std::sync` | DirectX |
//! | macOS | `macos` | `std::sync` | Metal |
//! | Android | `android` | `std::sync` | EGL/OpenGL ES |
//! | WASM | `wasm32` | `wasm_sync` | WebGL |
//! | iOS | `ios` | `std::sync` | Metal |
//!
//! # Ejemplo
//!
//! ```rust
//! use v_shield::platform::{Platform, current_platform};
//!
//! let p = current_platform();
//! println!("Running on: {:?}", p);
//! assert!(p.is_desktop() || p.is_mobile());
//! ```

/// Plataforma detectada
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    /// Linux (Desktop)
    Linux,
    /// Windows (Desktop)
    Windows,
    /// macOS (Desktop)
    MacOS,
    /// Android (Mobile)
    Android,
    /// iOS (Mobile)
    IOS,
    /// WebAssembly (Browser)
    Wasm,
}

impl Platform {
    /// ¿Es desktop? (Linux, Windows, macOS)
    pub fn is_desktop(self) -> bool {
        matches!(self, Self::Linux | Self::Windows | Self::MacOS)
    }

    /// ¿Es mobile? (Android, iOS)
    pub fn is_mobile(self) -> bool {
        matches!(self, Self::Android | Self::IOS)
    }

    /// ¿Es WASM?
    pub fn is_wasm(self) -> bool {
        matches!(self, Self::Wasm)
    }

    /// ¿Soporta X11?
    pub fn supports_x11(self) -> bool {
        matches!(self, Self::Linux)
    }

    /// ¿Soporta Wayland?
    pub fn supports_wayland(self) -> bool {
        matches!(self, Self::Linux)
    }

    /// ¿Tiene GPU disponible? (aproximación)
    pub fn has_gpu(self) -> bool {
        !matches!(self, Self::Wasm)
    }

    /// Nombre legible
    pub fn name(self) -> &'static str {
        match self {
            Self::Linux => "Linux",
            Self::Windows => "Windows",
            Self::MacOS => "macOS",
            Self::Android => "Android",
            Self::IOS => "iOS",
            Self::Wasm => "WebAssembly",
        }
    }

    /// Arquitectura del target
    pub fn arch(self) -> &'static str {
        if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else if cfg!(target_arch = "aarch64") {
            "aarch64"
        } else if cfg!(target_arch = "arm") {
            "arm"
        } else if cfg!(target_arch = "wasm32") {
            "wasm32"
        } else {
            "unknown"
        }
    }

    /// Info completa para debug
    pub fn debug_info(self) -> String {
        format!(
            "Platform: {} | Arch: {} | Desktop: {} | Mobile: {} | GPU: {}",
            self.name(),
            self.arch(),
            self.is_desktop(),
            self.is_mobile(),
            self.has_gpu()
        )
    }
}

/// Detecta la plataforma actual en compile-time
#[inline]
pub const fn current_platform() -> Platform {
    if cfg!(target_os = "linux") {
        Platform::Linux
    } else if cfg!(target_os = "windows") {
        Platform::Windows
    } else if cfg!(target_os = "macos") {
        Platform::MacOS
    } else if cfg!(target_os = "android") {
        Platform::Android
    } else if cfg!(target_os = "ios") {
        Platform::IOS
    } else if cfg!(target_arch = "wasm32") {
        Platform::Wasm
    } else {
        // Fallback genérico
        Platform::Linux
    }
}

/// Configuración defaults por plataforma
#[derive(Debug, Clone)]
pub struct PlatformConfig {
    /// ¿Usar VSync?
    pub vsync: bool,
    /// ¿Usar anti-aliasing?
    pub antialiasing: bool,
    /// Resolución por defecto (ancho)
    pub default_width: u32,
    /// Resolución por defecto (alto)
    pub default_height: u32,
    /// ¿Habilitar FSR auto?
    pub fsr_auto: bool,
    /// Umbral FPS para FSR auto
    pub fsr_threshold: u32,
}

impl PlatformConfig {
    /// Config defaults para la plataforma actual
    pub fn for_current() -> Self {
        let platform = current_platform();
        match platform {
            Platform::Linux => Self {
                vsync: true,
                antialiasing: false, // Termux-X11 no lo soporta bien
                default_width: 1280,
                default_height: 720,
                fsr_auto: true,
                fsr_threshold: 30,
            },
            Platform::Windows | Platform::MacOS => Self {
                vsync: true,
                antialiasing: true,
                default_width: 1920,
                default_height: 1080,
                fsr_auto: false,
                fsr_threshold: 60,
            },
            Platform::Android => Self {
                vsync: true,
                antialiasing: false,
                default_width: 1280,
                default_height: 720,
                fsr_auto: true,
                fsr_threshold: 25,
            },
            Platform::IOS => Self {
                vsync: true,
                antialiasing: false,
                default_width: 1170,
                default_height: 2532,
                fsr_auto: true,
                fsr_threshold: 30,
            },
            Platform::Wasm => Self {
                vsync: true,
                antialiasing: false,
                default_width: 800,
                default_height: 600,
                fsr_auto: false,
                fsr_threshold: 30,
            },
        }
    }

    /// Preset low-end (para hardware limitado)
    pub fn low_end() -> Self {
        Self {
            vsync: false,
            antialiasing: false,
            default_width: 960,
            default_height: 540,
            fsr_auto: true,
            fsr_threshold: 25,
        }
    }

    /// Preset high-end (para hardware potente)
    pub fn high_end() -> Self {
        Self {
            vsync: true,
            antialiasing: true,
            default_width: 2560,
            default_height: 1440,
            fsr_auto: false,
            fsr_threshold: 120,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_platform() {
        let p = current_platform();
        // En Termux/Android debería ser Android o Linux
        // En desktop debería ser Linux/Windows/MacOS
        assert!(p.is_desktop() || p.is_mobile() || p.is_wasm());
    }

    #[test]
    fn test_platform_name() {
        let p = current_platform();
        let name = p.name();
        assert!(!name.is_empty());
        assert!(name.len() < 20);
    }

    #[test]
    fn test_platform_config_defaults() {
        let cfg = PlatformConfig::for_current();
        assert!(cfg.vsync);
        assert!(cfg.default_width > 0);
        assert!(cfg.default_height > 0);
    }

    #[test]
    fn test_platform_config_low_end() {
        let cfg = PlatformConfig::low_end();
        assert!(!cfg.vsync);
        assert!(!cfg.antialiasing);
        assert_eq!(cfg.default_width, 960);
        assert_eq!(cfg.default_height, 540);
        assert!(cfg.fsr_auto);
    }

    #[test]
    fn test_platform_config_high_end() {
        let cfg = PlatformConfig::high_end();
        assert!(cfg.vsync);
        assert!(cfg.antialiasing);
        assert_eq!(cfg.default_width, 2560);
        assert_eq!(cfg.default_height, 1440);
    }

    #[test]
    fn test_platform_debug_info() {
        let p = current_platform();
        let info = p.debug_info();
        assert!(info.contains("Platform:"));
        assert!(info.contains("Arch:"));
    }
}
