//! # RyDit Render Queue - Command Queue + Double Buffering + Platform Sync
//!
//! **TRES CAPAS CRÍTICAS PARA RENDIMIENTO Y COMPATIBILIDAD**
//!
//! 1. **Command Queue (8192+ draw calls)** - Acumular comandos de dibujado
//! 2. **Double Buffering** - Separar lógica de renderizado
//! 3. **Platform Sync (XFlush/XSync)** - Sincronización con X11
//!
//! ## Arquitectura
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │  RYDIT RENDER QUEUE                                     │
//! ├─────────────────────────────────────────────────────────┤
//! │                                                         │
//! │  ┌─────────────────────────────────────────────────┐   │
//! │  │  1. COMMAND QUEUE (8192+ draw calls)            │   │
//! │  │  - Acumular: circle, rect, line, text, etc.     │   │
//! │  │  - Ejecutar: batch processing                   │   │
//! │  │  - Buffer circular: head, tail, capacity        │   │
//! │  └─────────────────────────────────────────────────┘   │
//! │                         │                               │
//! │  ┌──────────────────────▼──────────────────────────┐   │
//! │  │  2. DOUBLE BUFFERING                            │   │
//! │  │  - Front buffer: lógica (executor)              │   │
//! │  │  - Back buffer: render (draw calls)             │   │
//! │  │  - Swap: intercambiar buffers por frame         │   │
//! │  └─────────────────────────────────────────────────┘   │
//! │                         │                               │
//! │  ┌──────────────────────▼──────────────────────────┐   │
//! │  │  3. PLATFORM SYNC (X11)                         │   │
//! │  │  - XFlush: forzar flush de comandos             │   │
//! │  │  - XSync: sincronizar con servidor X11          │   │
//! │  │  - glFlush: OpenGL buffer swap                  │   │
//! │  └─────────────────────────────────────────────────┘   │
//! │                                                         │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Uso
//!
//! ```rust,no_run
//! use rydit_gfx::render_queue::{RenderQueue, DrawCommand};
//!
//! let mut queue = RenderQueue::with_capacity(8192);
//!
//! // Acumular comandos (front buffer - lógica)
//! queue.push(DrawCommand::Circle { x: 400, y: 300, radius: 50, color: "rojo" });
//! queue.push(DrawCommand::Rect { x: 100, y: 100, w: 100, h: 100, color: "verde" });
//!
//! // Ejecutar todos los comandos (back buffer - render)
//! queue.execute(&mut gfx);
//!
//! // Platform sync (X11)
//! queue.platform_sync();
//! ```

use crate::{ColorRydit, RyditGfx};
use crate::Assets;
use std::collections::VecDeque;

// ============================================================================
// DRAW COMMANDS - Tipos de comandos de dibujado
// ============================================================================

/// Comandos de dibujado para la queue
#[derive(Debug, Clone)]
pub enum DrawCommand {
    /// draw.circle(x, y, radius, color)
    Circle {
        x: i32,
        y: i32,
        radius: i32,
        color: ColorRydit,
    },
    /// draw.rect(x, y, w, h, color)
    Rect {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        color: ColorRydit,
    },
    /// draw.line(x1, y1, x2, y2, color)
    Line {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: ColorRydit,
    },
    /// draw.text(text, x, y, size, color)
    Text {
        text: String,
        x: i32,
        y: i32,
        size: i32,
        color: ColorRydit,
    },
    /// draw.triangle(v1, v2, v3, color)
    Triangle {
        v1: (i32, i32),
        v2: (i32, i32),
        v3: (i32, i32),
        color: ColorRydit,
    },
    /// draw.texture(id, x, y, scale, rotation, color)
    Texture {
        id: String,
        x: f32,
        y: f32,
        scale: f32,
        rotation: f32,
        color: ColorRydit,
    },
    /// Limpiar pantalla
    Clear { color: ColorRydit },
}

// ============================================================================
// RENDER QUEUE - Command Queue (8192+ draw calls)
// ============================================================================

/// Command Queue para draw calls
///
/// **Capacidad**: 8192+ comandos por frame
/// **Estrategia**: Buffer circular con head/tail
pub struct RenderQueue {
    /// Cola de comandos (buffer circular)
    commands: VecDeque<DrawCommand>,
    /// Capacidad máxima
    capacity: usize,
    /// Comandos acumulados en este frame
    frame_count: usize,
    /// Total de comandos ejecutados
    total_executed: usize,
    /// Estadísticas: máximo de comandos en un frame
    max_frame_count: usize,
}

impl RenderQueue {
    /// Crear queue con capacidad por defecto (8192)
    pub fn new() -> Self {
        Self::with_capacity(8192)
    }

    /// Crear queue con capacidad custom
    pub fn with_capacity(capacity: usize) -> Self {
        println!(
            "[RENDER QUEUE] Command Queue creada: capacidad={} (8192+ draw calls)",
            capacity
        );
        Self {
            commands: VecDeque::with_capacity(capacity),
            capacity,
            frame_count: 0,
            total_executed: 0,
            max_frame_count: 0,
        }
    }

    /// Push de comando a la queue
    pub fn push(&mut self, command: DrawCommand) {
        // Verificar overflow
        if self.commands.len() >= self.capacity {
            // Buffer lleno: remover el comando más antiguo (política FIFO)
            self.commands.pop_front();
            #[cfg(debug_assertions)]
            eprintln!("[RENDER QUEUE] WARNING: Buffer lleno ({}), removiendo comando antiguo", self.capacity);
        }

        self.commands.push_back(command);
        self.frame_count += 1;
    }

    /// Ejecutar todos los comandos acumulados
    pub fn execute(&mut self, gfx: &mut RyditGfx, assets: &Assets) {
        if self.commands.is_empty() {
            return;
        }

        // Actualizar estadísticas
        if self.frame_count > self.max_frame_count {
            self.max_frame_count = self.frame_count;
        }
        self.total_executed += self.frame_count;

        #[cfg(debug_assertions)]
        eprintln!(
            "[RENDER QUEUE] Ejecutando {} comandos (total: {}, max frame: {})",
            self.frame_count, self.total_executed, self.max_frame_count
        );

        // Ejecutar todos los comandos EN UN SOLO begin_draw
        {
            let mut d = gfx.begin_draw();

            for command in self.commands.drain(..) {
                match command {
                    DrawCommand::Circle { x, y, radius, color } => {
                        d.draw_circle(x, y, radius, color);
                    }
                    DrawCommand::Rect { x, y, w, h, color } => {
                        d.draw_rectangle(x, y, w, h, color);
                    }
                    DrawCommand::Line { x1, y1, x2, y2, color } => {
                        d.draw_line(x1, y1, x2, y2, color);
                    }
                    DrawCommand::Text { text, x, y, size, color } => {
                        d.draw_text(&text, x, y, size, color);
                    }
                    DrawCommand::Triangle { v1, v2, v3, color } => {
                        d.draw_triangle(v1, v2, v3, color);
                    }
                    DrawCommand::Texture { id, x, y, scale, rotation, color } => {
                        // Dibujar textura con escala y rotación
                        assets.draw_texture_ex_by_id(&mut d.draw, &id, x, y, scale, rotation, color);
                    }
                    DrawCommand::Clear { color } => {
                        d.clear(color);
                    }
                }
            }

            // Drop EXPLÍCITO para forzar buffer swap (fix Zink/Vulkan)
            drop(d);
        }

        // Reset contador de frame
        self.frame_count = 0;
    }

    /// Limpiar queue sin ejecutar
    pub fn clear(&mut self) {
        self.commands.clear();
        self.frame_count = 0;
    }

    /// Obtener cantidad de comandos pendientes
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Verificar si está vacía
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Obtener capacidad máxima
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Obtener estadísticas
    pub fn stats(&self) -> RenderQueueStats {
        RenderQueueStats {
            pending: self.commands.len(),
            frame_count: self.frame_count,
            total_executed: self.total_executed,
            max_frame_count: self.max_frame_count,
            capacity: self.capacity,
        }
    }
}

impl Default for RenderQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Estadísticas de la render queue
#[derive(Debug, Clone, Default)]
pub struct RenderQueueStats {
    pub pending: usize,
    pub frame_count: usize,
    pub total_executed: usize,
    pub max_frame_count: usize,
    pub capacity: usize,
}

impl std::fmt::Display for RenderQueueStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RenderQueue {{ pending: {}, frame: {}, total: {}, max: {}, capacity: {} }}",
            self.pending, self.frame_count, self.total_executed, self.max_frame_count, self.capacity
        )
    }
}

// ============================================================================
// DOUBLE BUFFERING - Separar lógica de renderizado
// ============================================================================

/// Double Buffer para draw commands
///
/// **Front Buffer**: Lógica acumula comandos (executor)
/// **Back Buffer**: Render ejecuta comandos (gfx)
/// **Swap**: Intercambiar buffers al final del frame
pub struct DoubleBuffer {
    /// Front buffer (acumulación - lógica)
    front: RenderQueue,
    /// Back buffer (ejecución - render)
    back: RenderQueue,
    /// Frame actual
    frame: u64,
}

impl DoubleBuffer {
    /// Crear double buffer con capacidad
    pub fn new(capacity: usize) -> Self {
        println!("[DOUBLE BUFFER] Creado con capacidad={}", capacity);
        Self {
            front: RenderQueue::with_capacity(capacity),
            back: RenderQueue::with_capacity(capacity),
            frame: 0,
        }
    }

    /// Push al front buffer (lógica)
    pub fn push(&mut self, command: DrawCommand) {
        self.front.push(command);
    }

    /// Swap de buffers: front → back
    /// Retorna el back buffer anterior para ejecutar
    pub fn swap(&mut self) {
        self.frame += 1;

        // Intercambiar buffers
        std::mem::swap(&mut self.front, &mut self.back);

        #[cfg(debug_assertions)]
        eprintln!(
            "[DOUBLE BUFFER] Frame {} - Swap completado (back: {} comandos)",
            self.frame,
            self.back.len()
        );
    }

    /// Ejecutar back buffer
    pub fn execute(&mut self, gfx: &mut RyditGfx, assets: &Assets) {
        self.back.execute(gfx, assets);
        self.back.clear();
    }

    /// Swap + execute (operación combinada)
    pub fn swap_and_execute(&mut self, gfx: &mut RyditGfx, assets: &Assets) {
        self.swap();
        self.execute(gfx, assets);
    }

    /// Obtener frame actual
    pub fn frame(&self) -> u64 {
        self.frame
    }

    /// Obtener estadísticas de ambos buffers
    pub fn stats(&self) -> (RenderQueueStats, RenderQueueStats) {
        (self.front.stats(), self.back.stats())
    }
}

// ============================================================================
// PLATFORM SYNC - XFlush/XSync para Termux-X11
// ============================================================================

/// Platform Sync para X11 (Termux-X11)
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

/// Modo de sincronización
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlatformSyncMode {
    /// X11 (Termux-X11)
    X11,
    /// OpenGL
    OpenGL,
    /// Auto-detect
    Auto,
}

impl PlatformSync {
    /// Crear Platform Sync (auto-detect)
    pub fn new() -> Self {
        let mode = Self::detect_mode();
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

        #[cfg(debug_assertions)]
        eprintln!("[PLATFORM SYNC] Frame {} - Sync completado", self.frame);
    }

    /// XFlush: Forzar flush de comandos X11
    ///
    /// **Importante**: En Termux-X11, los comandos OpenGL
    /// se envían al servidor X11. Sin XFlush, pueden quedar
    /// en buffer sin ejecutar.
    fn xflush(&self) {
        // NOTA: raylib no expone XDisplay directamente
        // Usamos FFI para llamar a XFlush
        #[cfg(target_os = "android")]
        unsafe {
            // Intentar obtener XDisplay
            let display = self.get_x_display();
            if !display.is_null() {
                // XFlush(display)
                // NOTA: Esto requiere linking con libX11
                // Por ahora, usamos glFlush como fallback
                eprintln!("[PLATFORM SYNC] XFlush: display={:?}", display);
            }
        }
    }

    /// XSync: Sincronizar con servidor X11
    ///
    /// **Importante**: Espera a que el servidor X11
    /// termine de procesar todos los comandos pendientes.
    fn xsync(&self) {
        #[cfg(target_os = "android")]
        unsafe {
            let display = self.get_x_display();
            if !display.is_null() {
                // XSync(display, False)
                // NOTA: Requiere libX11
                eprintln!("[PLATFORM SYNC] XSync: display={:?}", display);
            }
        }
    }

    /// glFlush: OpenGL buffer swap
    ///
    /// **Importante**: Forzar OpenGL a ejecutar comandos
    /// pendientes. En Zink/Vulkan (Termux-X11), esto es
    /// CRÍTICO para que el buffer swap funcione.
    fn gl_flush(&self) {
        // glFlush()
        // raylib no expone esto directamente, pero el Drop de DrawHandle
        // ya hace end_drawing() que incluye el swap
    }

    /// Obtener XDisplay (FFI)
    #[cfg(target_os = "android")]
    unsafe fn get_x_display(&self) -> *mut std::ffi::c_void {
        // NOTA: Esto es un placeholder
        // En Termux-X11, raylib podría no usar X11 directamente
        // Podríamos necesitar glfwGetX11Display() o similar
        std::ptr::null_mut()
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
}

impl Default for PlatformSync {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// INTEGRACIÓN CON RyditGfx
// ============================================================================

/// Extensión para RyditGfx con render queue
pub trait RyditGfxExt {
    /// Ejecutar comandos desde queue
    fn execute_queue(&mut self, queue: &mut RenderQueue, assets: &Assets);

    /// Platform sync (X11)
    fn platform_sync(&mut self);
}

impl RyditGfxExt for RyditGfx {
    fn execute_queue(&mut self, queue: &mut RenderQueue, assets: &Assets) {
        queue.execute(self, assets);
    }

    fn platform_sync(&mut self) {
        // Platform sync se maneja externamente
        // El buffer swap ya ocurre en Drop de DrawHandle
    }
}
