//! 🛡️ V-Shield Platform Sync
//!
//! Sincronización de renderizado multiplataforma.
//!
//! Maneja las diferencias entre X11, OpenGL, y otros backends
//! para garantizar que los comandos de renderizado se ejecuten
//! correctamente en cada frame.
//!
//! # Modos
//!
//! - `X11`: Termux-X11, Linux desktop → XFlush + XSync
//! - `OpenGL`: Native GL → glFlush
//! - `Auto`: Auto-detect por env var `DISPLAY`
//!
//! # Ejemplo
//!
//! ```rust
//! use v_shield::platform_sync::PlatformSync;
//!
//! let mut sync = PlatformSync::new();
//! // ... al final de cada frame ...
//! sync.sync();
//! ```

/// Modo de sincronización
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlatformSyncMode {
    /// X11 (Termux-X11)
    X11,
    /// OpenGL nativo
    OpenGL,
    /// Auto-detect
    Auto,
}

/// Platform Sync - sincronización de frame multiplataforma
///
/// **Funciones**:
/// - `xflush()`: Forzar flush de comandos X11
/// - `xsync()`: Sincronizar con servidor X11
/// - `gl_flush()`: OpenGL buffer swap
pub struct PlatformSync {
    /// Si está habilitado
    enabled: bool,
    /// Modo: x11, gl, auto
    mode: PlatformSyncMode,
    /// Frame count
    frame: u64,
}

impl PlatformSync {
    /// Crear Platform Sync (auto-detect)
    pub fn new() -> Self {
        let mode = Self::detect_mode();
        #[cfg(debug_assertions)]
        println!(
            "[PLATFORM SYNC] Detectado modo: {:?} (Termux-X11 compatible)",
            mode
        );
        Self {
            enabled: true,
            mode,
            frame: 0,
        }
    }

    /// Detectar modo automáticamente
    fn detect_mode() -> PlatformSyncMode {
        // Si DISPLAY está seteado, asumir X11
        if std::env::var("DISPLAY").is_ok() {
            PlatformSyncMode::X11
        } else {
            PlatformSyncMode::OpenGL
        }
    }

    /// Platform sync: llamar al final de cada frame
    pub fn sync(&mut self) {
        if !self.enabled {
            return;
        }

        self.frame += 1;

        match self.mode {
            PlatformSyncMode::X11 => {
                self.xflush();
                self.xsync();
            }
            PlatformSyncMode::OpenGL => {
                self.gl_flush();
            }
            PlatformSyncMode::Auto => {
                // Auto: intentar X11 primero, fallback a GL
                self.xflush();
                self.gl_flush();
            }
        }
    }

    /// XFlush: Forzar flush de comandos X11
    fn xflush(&self) {
        // NOTA: Esto requiere FFI con libX11
        // Por ahora, es un no-op. El backend de SDL2/raylib
        // ya maneja el flush internamente.
        #[cfg(debug_assertions)]
        eprintln!("[PLATFORM SYNC] XFlush: stub (SDL2/raylib manejan internamente)");
    }

    /// XSync: Sincronizar con servidor X11
    fn xsync(&self) {
        // NOTA: Requiere libX11
        #[cfg(debug_assertions)]
        eprintln!("[PLATFORM SYNC] XSync: stub (SDL2/raylib manejan internamente)");
    }

    /// glFlush: OpenGL buffer swap
    fn gl_flush(&self) {
        // El backend de SDL2/raylib ya hace swap internamente
        // en su Drop de DrawHandle / end_drawing()
    }

    /// Habilitar Platform Sync
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Deshabilitar Platform Sync
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Cambiar modo
    pub fn set_mode(&mut self, mode: PlatformSyncMode) {
        self.mode = mode;
    }

    /// Obtener modo actual
    pub fn mode(&self) -> PlatformSyncMode {
        self.mode
    }

    /// Frame count actual
    pub fn frame_count(&self) -> u64 {
        self.frame
    }
}

impl Default for PlatformSync {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_sync_new() {
        let sync = PlatformSync::new();
        assert!(sync.enabled);
        assert_eq!(sync.frame, 0);
    }

    #[test]
    fn test_platform_sync_enable_disable() {
        let mut sync = PlatformSync::new();
        sync.disable();
        assert!(!sync.enabled);
        sync.enable();
        assert!(sync.enabled);
    }

    #[test]
    fn test_platform_sync_sync_disabled() {
        let mut sync = PlatformSync::new();
        sync.disable();
        sync.sync(); // No debe panic
        assert_eq!(sync.frame, 0);
    }

    #[test]
    fn test_platform_sync_mode() {
        let mut sync = PlatformSync::new();
        sync.set_mode(PlatformSyncMode::OpenGL);
        assert_eq!(sync.mode(), PlatformSyncMode::OpenGL);
        sync.set_mode(PlatformSyncMode::X11);
        assert_eq!(sync.mode(), PlatformSyncMode::X11);
    }

    #[test]
    fn test_platform_sync_default() {
        let sync = PlatformSync::default();
        assert!(sync.enabled);
    }
}
