// crates/rydit-gfx/src/camera.rs
// Cámara 2D para RyDit Engine
//
// Soporte para:
// - Posición (x, y)
// - Zoom
// - Rotación
// - Seguimiento suave (lerp)
// - Límites de scroll
// - Conversión mundo↔pantalla

use crate::DrawHandle;

/// Cámara 2D
///
/// Permite transformar coordenadas del mundo a coordenadas de pantalla.
/// Útil para juegos con scroll, zoom, o cámaras que siguen al jugador.
#[derive(Debug, Clone)]
pub struct Camera2D {
    /// Posición X de la cámara en el mundo
    pub x: f32,
    /// Posición Y de la cámara en el mundo
    pub y: f32,
    /// Zoom (1.0 = normal, 2.0 = 2x zoom, 0.5 = half zoom)
    pub zoom: f32,
    /// Rotación en grados
    pub rotation: f32,
    /// Posición objetivo X (para seguimiento suave)
    pub target_x: Option<f32>,
    /// Posición objetivo Y (para seguimiento suave)
    pub target_y: Option<f32>,
    /// Factor de suavizado (0.0-1.0) para follow_smooth
    pub smooth: f32,
    /// Límite mínimo X (None = sin límite)
    pub min_x: Option<f32>,
    /// Límite mínimo Y (None = sin límite)
    pub min_y: Option<f32>,
    /// Límite máximo X (None = sin límite)
    pub max_x: Option<f32>,
    /// Límite máximo Y (None = sin límite)
    pub max_y: Option<f32>,
}

impl Camera2D {
    /// Crear nueva cámara con valores por defecto
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
            rotation: 0.0,
            target_x: None,
            target_y: None,
            smooth: 0.1,
            min_x: None,
            min_y: None,
            max_x: None,
            max_y: None,
        }
    }

    /// Crear cámara con posición inicial
    pub fn at(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            ..Default::default()
        }
    }

    /// Crear cámara con posición y zoom
    pub fn with_zoom(x: f32, y: f32, zoom: f32) -> Self {
        Self {
            x,
            y,
            zoom,
            ..Default::default()
        }
    }

    // ========================================================================
    // FUNCIONES BÁSICAS
    // ========================================================================

    /// Establecer posición de la cámara
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = self.clamp_x(x);
        self.y = self.clamp_y(y);
    }

    /// Obtener posición actual
    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    /// Establecer zoom (0.1 mínimo, 10.0 máximo)
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(0.1, 10.0);
    }

    /// Obtener zoom actual
    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    /// Establecer rotación en grados
    pub fn set_rotation(&mut self, angle: f32) {
        self.rotation = angle % 360.0;
    }

    /// Obtener rotación actual
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    // ========================================================================
    // SCROLL
    // ========================================================================

    /// Mover cámara relativamente (scroll)
    pub fn scroll(&mut self, dx: f32, dy: f32) {
        self.x = self.clamp_x(self.x + dx);
        self.y = self.clamp_y(self.y + dy);
    }

    /// Mover cámara a posición absoluta
    pub fn scroll_to(&mut self, x: f32, y: f32) {
        self.set_position(x, y);
    }

    /// Establecer límites de scroll
    pub fn set_bounds(&mut self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) {
        self.min_x = Some(min_x);
        self.min_y = Some(min_y);
        self.max_x = Some(max_x);
        self.max_y = Some(max_y);
        // Aplicar límites inmediatamente
        self.x = self.clamp_x(self.x);
        self.y = self.clamp_y(self.y);
    }

    /// Limpiar límites de scroll
    pub fn clear_bounds(&mut self) {
        self.min_x = None;
        self.min_y = None;
        self.max_x = None;
        self.max_y = None;
    }

    // ========================================================================
    // SEGUIMIENTO DEL JUGADOR
    // ========================================================================

    /// Seguir objetivo instantáneamente
    pub fn follow(&mut self, target_x: f32, target_y: f32) {
        self.target_x = Some(target_x);
        self.target_y = Some(target_y);
        self.x = self.clamp_x(target_x);
        self.y = self.clamp_y(target_y);
    }

    /// Seguir objetivo con suavizado (lerp)
    pub fn follow_smooth(&mut self, target_x: f32, target_y: f32, smooth: f32) {
        self.target_x = Some(target_x);
        self.target_y = Some(target_y);
        self.smooth = smooth.clamp(0.01, 1.0);

        // Interpolar posición actual hacia objetivo
        let dx = target_x - self.x;
        let dy = target_y - self.y;
        self.x = self.clamp_x(self.x + dx * self.smooth);
        self.y = self.clamp_y(self.y + dy * self.smooth);
    }

    /// Establecer offset para seguimiento
    pub fn set_follow_offset(&mut self, offset_x: f32, offset_y: f32) {
        if let Some(tx) = self.target_x {
            if let Some(ty) = self.target_y {
                self.x = self.clamp_x(tx + offset_x);
                self.y = self.clamp_y(ty + offset_y);
            }
        }
    }

    // ========================================================================
    // CONVERSIÓN DE COORDENADAS
    // ========================================================================

    /// Convertir coordenadas del mundo a pantalla
    pub fn world_to_screen(&self, world_x: f32, world_y: f32, screen_width: i32, screen_height: i32) -> (i32, i32) {
        // Aplicar transformación inversa: traslación → rotación → escala
        let mut x = world_x - self.x;
        let mut y = world_y - self.y;

        // Rotación inversa
        if self.rotation != 0.0 {
            let rad = -self.rotation.to_radians();
            let cos = rad.cos();
            let sin = rad.sin();
            let rx = x * cos - y * sin;
            let ry = x * sin + y * cos;
            x = rx;
            y = ry;
        }

        // Escala (zoom)
        x *= self.zoom;
        y *= self.zoom;

        // Centrar en pantalla
        let sx = (x + screen_width as f32 / 2.0) as i32;
        let sy = (y + screen_height as f32 / 2.0) as i32;

        (sx, sy)
    }

    /// Convertir coordenadas de pantalla a mundo
    pub fn screen_to_world(&self, screen_x: i32, screen_y: i32, screen_width: i32, screen_height: i32) -> (f32, f32) {
        // Aplicar transformación: escala → rotación → traslación
        let mut x = screen_x as f32 - screen_width as f32 / 2.0;
        let mut y = screen_y as f32 - screen_height as f32 / 2.0;

        // Deshacer zoom
        x /= self.zoom;
        y /= self.zoom;

        // Rotación
        if self.rotation != 0.0 {
            let rad = self.rotation.to_radians();
            let cos = rad.cos();
            let sin = rad.sin();
            let rx = x * cos - y * sin;
            let ry = x * sin + y * cos;
            x = rx;
            y = ry;
        }

        // Trasladar al mundo
        x += self.x;
        y += self.y;

        (x, y)
    }

    // ========================================================================
    // APLICAR CÁMARA AL DRAW
    // ========================================================================

    /// Aplicar transformaciones de cámara al DrawHandle
    ///
    /// Esto debe llamarse ANTES de cualquier draw call en el frame.
    /// Nota: Esta función es para uso interno del engine.
    pub fn apply(&self, _d: &mut DrawHandle) {
        // Por ahora, las transformaciones se aplican manualmente
        // en las funciones de draw usando world_to_screen
        //
        // En una implementación futura, podríamos usar
        // raylib::Camera2D nativo para transformaciones automáticas
    }

    // ========================================================================
    // UTILIDADES
    // ========================================================================

    /// Clamp X con límites
    fn clamp_x(&self, x: f32) -> f32 {
        let mut result = x;
        if let Some(min) = self.min_x {
            result = result.max(min);
        }
        if let Some(max) = self.max_x {
            result = result.min(max);
        }
        result
    }

    /// Clamp Y con límites
    fn clamp_y(&self, y: f32) -> f32 {
        let mut result = y;
        if let Some(min) = self.min_y {
            result = result.max(min);
        }
        if let Some(max) = self.max_y {
            result = result.min(max);
        }
        result
    }

    /// Resetear cámara a valores por defecto
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Centrar cámara en posición
    pub fn center_on(&mut self, x: f32, y: f32) {
        self.set_position(x, y);
    }
}

impl Default for Camera2D {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ESTADO GLOBAL DE CÁMARA
// ============================================================================

use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    /// Cámara global para uso en módulos RyDit
    static CAMERA: Rc<RefCell<Camera2D>> = Rc::new(RefCell::new(Camera2D::new()));
}

/// Obtener referencia a la cámara global
pub fn get_camera() -> Rc<RefCell<Camera2D>> {
    CAMERA.with(|c| c.clone())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_new() {
        let camera = Camera2D::new();
        assert_eq!(camera.x, 0.0);
        assert_eq!(camera.y, 0.0);
        assert_eq!(camera.zoom, 1.0);
        assert_eq!(camera.rotation, 0.0);
    }

    #[test]
    fn test_camera_set_position() {
        let mut camera = Camera2D::new();
        camera.set_position(100.0, 200.0);
        assert_eq!(camera.get_position(), (100.0, 200.0));
    }

    #[test]
    fn test_camera_set_zoom() {
        let mut camera = Camera2D::new();
        camera.set_zoom(2.0);
        assert_eq!(camera.get_zoom(), 2.0);

        // Test límites
        camera.set_zoom(0.05);
        assert_eq!(camera.get_zoom(), 0.1); // mínimo

        camera.set_zoom(15.0);
        assert_eq!(camera.get_zoom(), 10.0); // máximo
    }

    #[test]
    fn test_camera_scroll() {
        let mut camera = Camera2D::new();
        camera.scroll(10.0, 20.0);
        assert_eq!(camera.get_position(), (10.0, 20.0));

        camera.scroll(-5.0, -10.0);
        assert_eq!(camera.get_position(), (5.0, 10.0));
    }

    #[test]
    fn test_camera_follow() {
        let mut camera = Camera2D::new();
        camera.follow(100.0, 200.0);
        assert_eq!(camera.get_position(), (100.0, 200.0));
    }

    #[test]
    fn test_camera_follow_smooth() {
        let mut camera = Camera2D::new();
        camera.follow_smooth(100.0, 200.0, 0.5);

        // Después de un smooth, debería estar a mitad de camino
        let (x, y) = camera.get_position();
        assert!((x - 50.0).abs() < 1.0); // ~50% de 100
        assert!((y - 100.0).abs() < 1.0); // ~50% de 200
    }

    #[test]
    fn test_camera_bounds() {
        let mut camera = Camera2D::new();
        camera.set_bounds(0.0, 0.0, 100.0, 100.0);

        camera.set_position(150.0, 150.0);
        let (x, y) = camera.get_position();
        assert_eq!(x, 100.0); // clamp al máximo
        assert_eq!(y, 100.0);

        camera.set_position(-50.0, -50.0);
        let (x, y) = camera.get_position();
        assert_eq!(x, 0.0); // clamp al mínimo
        assert_eq!(y, 0.0);
    }

    #[test]
    fn test_camera_world_to_screen() {
        let camera = Camera2D::with_zoom(0.0, 0.0, 1.0);
        let (sx, sy) = camera.world_to_screen(0.0, 0.0, 800, 600);

        // Sin transformación, el centro del mundo debería estar en el centro de pantalla
        assert_eq!(sx, 400);
        assert_eq!(sy, 300);
    }

    #[test]
    fn test_camera_screen_to_world() {
        let camera = Camera2D::with_zoom(0.0, 0.0, 1.0);
        let (wx, wy) = camera.screen_to_world(400, 300, 800, 600);

        // Centro de pantalla debería ser centro del mundo
        assert!((wx - 0.0).abs() < 0.01);
        assert!((wy - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_camera_reset() {
        let mut camera = Camera2D::with_zoom(100.0, 200.0, 2.0);
        camera.reset();

        assert_eq!(camera.x, 0.0);
        assert_eq!(camera.y, 0.0);
        assert_eq!(camera.zoom, 1.0);
    }

    #[test]
    fn test_camera_functions_exist() {
        // Verificar que las funciones existen
        let _ = Camera2D::new;
        let _ = Camera2D::at;
        let _ = Camera2D::with_zoom;
        let _ = get_camera;
    }
}
