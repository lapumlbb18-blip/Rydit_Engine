// crates/ry-anim/src/transitions.rs
// Sistema de Transiciones Avanzadas - v0.18.0
// Inspirado en editores de video: 15+ transiciones + extensible
//
// Transiciones incluidas:
// - FadeIn, FadeOut, CrossFade
// - SlideLeft, SlideRight, SlideUp, SlideDown
// - WipeLeft, WipeRight, WipeUp, WipeDown
// - ZoomIn, ZoomOut
// - CircleOpen, CircleClose
// - BlindsHorizontal, BlindsVertical
// - Dissolve
// - Spiral
// - Checkerboard
// - Pixelate

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

// ============================================================================
// TIPO DE TRANSICIÓN
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionType {
    // Fade
    FadeIn,
    FadeOut,
    CrossFade,

    // Slide
    SlideLeft,
    SlideRight,
    SlideUp,
    SlideDown,

    // Wipe
    WipeLeft,
    WipeRight,
    WipeUp,
    WipeDown,

    // Zoom
    ZoomIn,
    ZoomOut,

    // Circle
    CircleOpen,
    CircleClose,

    // Blinds
    BlindsHorizontal,
    BlindsVertical,

    // Special
    Dissolve,
    Spiral,
    Checkerboard,
    Pixelate,
}

// ============================================================================
// TRANSICIÓN
// ============================================================================

pub struct Transition {
    pub kind: TransitionType,
    pub duration: f32,
    pub elapsed: f32,
    pub progress: f32, // 0.0 -> 1.0
    pub done: bool,

    // Texturas para CrossFade y otras
    pub texture_a: Option<Texture<'static>>,
    pub texture_b: Option<Texture<'static>>,

    // Parámetros configurables
    pub fade_color: Color,
    pub zoom_center: (f32, f32),
    pub spiral_turns: f32,
    pub checkerboard_size: u32,
    pub pixelate_block: u32,
}

impl Transition {
    /// Crear nueva transición
    pub fn new(kind: TransitionType, duration: f32) -> Self {
        Self {
            kind,
            duration: duration.max(0.05),
            elapsed: 0.0,
            progress: 0.0,
            done: false,
            texture_a: None,
            texture_b: None,
            fade_color: Color::RGB(0, 0, 0),
            zoom_center: (400.0, 300.0),
            spiral_turns: 3.0,
            checkerboard_size: 40,
            pixelate_block: 8,
        }
    }

    /// Actualizar transición
    pub fn update(&mut self, dt: f32) {
        if self.done { return; }

        self.elapsed += dt;
        self.progress = (self.elapsed / self.duration).min(1.0);

        if self.progress >= 1.0 {
            self.done = true;
            self.progress = 1.0;
        }
    }

    /// Verificar si la transición está activa
    pub fn is_active(&self) -> bool {
        !self.done
    }

    /// Obtener progreso normalizado (0.0 - 1.0)
    pub fn progress(&self) -> f32 {
        self.progress
    }

    /// Aplicar easing suave
    pub fn eased_progress(&self) -> f32 {
        // Smoothstep: 3t² - 2t³
        let t = self.progress;
        t * t * (3.0 - 2.0 * t)
    }

    /// Renderizar transición sobre un canvas
    pub fn render(
        &self,
        canvas: &mut Canvas<Window>,
        screen_w: u32,
        screen_h: u32,
        current_texture: Option<&Texture>,
    ) {
        if self.done { return; }

        match self.kind {
            TransitionType::FadeIn => self.render_fade_in(canvas, screen_w, screen_h, current_texture),
            TransitionType::FadeOut => self.render_fade_out(canvas, screen_w, screen_h, current_texture),
            TransitionType::CrossFade => self.render_cross_fade(canvas, screen_w, screen_h, current_texture),
            TransitionType::SlideLeft => self.render_slide(canvas, screen_w, screen_h, current_texture, -1.0, 0.0),
            TransitionType::SlideRight => self.render_slide(canvas, screen_w, screen_h, current_texture, 1.0, 0.0),
            TransitionType::SlideUp => self.render_slide(canvas, screen_w, screen_h, current_texture, 0.0, -1.0),
            TransitionType::SlideDown => self.render_slide(canvas, screen_w, screen_h, current_texture, 0.0, 1.0),
            TransitionType::WipeLeft => self.render_wipe(canvas, screen_w, screen_h, current_texture, -1.0, 0.0),
            TransitionType::WipeRight => self.render_wipe(canvas, screen_w, screen_h, current_texture, 1.0, 0.0),
            TransitionType::WipeUp => self.render_wipe(canvas, screen_w, screen_h, current_texture, 0.0, -1.0),
            TransitionType::WipeDown => self.render_wipe(canvas, screen_w, screen_h, current_texture, 0.0, 1.0),
            TransitionType::ZoomIn => self.render_zoom_in(canvas, screen_w, screen_h, current_texture),
            TransitionType::ZoomOut => self.render_zoom_out(canvas, screen_w, screen_h, current_texture),
            TransitionType::CircleOpen => self.render_circle_open(canvas, screen_w, screen_h, current_texture),
            TransitionType::CircleClose => self.render_circle_close(canvas, screen_w, screen_h, current_texture),
            TransitionType::BlindsHorizontal => self.render_blinds(canvas, screen_w, screen_h, current_texture, true),
            TransitionType::BlindsVertical => self.render_blinds(canvas, screen_w, screen_h, current_texture, false),
            TransitionType::Dissolve => self.render_dissolve(canvas, screen_w, screen_h, current_texture),
            TransitionType::Spiral => self.render_spiral(canvas, screen_w, screen_h, current_texture),
            TransitionType::Checkerboard => self.render_checkerboard(canvas, screen_w, screen_h, current_texture),
            TransitionType::Pixelate => self.render_pixelate(canvas, screen_w, screen_h, current_texture),
        }
    }

    // ========================================================================
    // RENDERIZADORES DE TRANSICIONES
    // ========================================================================

    fn render_fade_in(&self, canvas: &mut Canvas<Window>, w: u32, h: u32, tex: Option<&Texture>) {
        let p = 1.0 - self.eased_progress();
        let alpha = (p * 255.0) as u8;

        if let Some(t) = tex {
            let _ = canvas.copy(t, None, Rect::new(0, 0, w, h));
        }

        canvas.set_draw_color(Color::RGBA(
            self.fade_color.r,
            self.fade_color.g,
            self.fade_color.b,
            alpha,
        ));
        let _ = canvas.fill_rect(Rect::new(0, 0, w, h));
    }

    fn render_fade_out(&self, canvas: &mut Canvas<Window>, w: u32, h: u32, tex: Option<&Texture>) {
        let p = self.eased_progress();
        let alpha = (p * 255.0) as u8;

        if let Some(t) = tex {
            let _ = canvas.copy(t, None, Rect::new(0, 0, w, h));
        }

        canvas.set_draw_color(Color::RGBA(
            self.fade_color.r,
            self.fade_color.g,
            self.fade_color.b,
            alpha,
        ));
        let _ = canvas.fill_rect(Rect::new(0, 0, w, h));
    }

    fn render_cross_fade(&self, canvas: &mut Canvas<Window>, w: u32, h: u32, _tex: Option<&Texture>) {
        let p = self.eased_progress();

        if let Some(a) = &self.texture_a {
            canvas.set_draw_color(Color::RGBA(255, 255, 255, ((1.0 - p) * 255.0) as u8));
            let _ = canvas.copy(a, None, Rect::new(0, 0, w, h));
        }

        if let Some(b) = &self.texture_b {
            canvas.set_draw_color(Color::RGBA(255, 255, 255, (p * 255.0) as u8));
            let _ = canvas.copy(b, None, Rect::new(0, 0, w, h));
        }
    }

    fn render_slide(&self, canvas: &mut Canvas<Window>, w: u32, h: u32, tex: Option<&Texture>, dx: f32, dy: f32) {
        let p = self.eased_progress();
        let offset_x = (dx * p * w as f32) as i32;
        let offset_y = (dy * p * h as f32) as i32;

        if let Some(t) = tex {
            let _ = canvas.copy(t, None, Rect::new(offset_x, offset_y, w, h));
        }
    }

    fn render_wipe(&self, canvas: &mut Canvas<Window>, w: u32, h: u32, tex: Option<&Texture>, dx: f32, dy: f32) {
        let p = self.eased_progress();

        if let Some(t) = tex {
            // Calcular rect visible según dirección
            let rect = if dx < 0.0 {
                // Wipe izquierda: mostrar desde derecha
                let visible_w = (w as f32 * (1.0 - p)) as u32;
                Rect::new(w as i32 - visible_w as i32, 0, visible_w, h)
            } else if dx > 0.0 {
                // Wipe derecha: mostrar desde izquierda
                let visible_w = (w as f32 * p) as u32;
                Rect::new(0, 0, visible_w, h)
            } else if dy < 0.0 {
                // Wipe arriba
                let visible_h = (h as f32 * (1.0 - p)) as u32;
                Rect::new(0, h as i32 - visible_h as i32, w, visible_h)
            } else {
                // Wipe abajo
                let visible_h = (h as f32 * p) as u32;
                Rect::new(0, 0, w, visible_h)
            };

            let _ = canvas.copy(t, Some(rect), rect);
        }
    }

    fn render_zoom_in(&self, canvas: &mut Canvas<Window>, w: u32, h: u32, tex: Option<&Texture>) {
        let p = self.eased_progress();
        let scale = 1.0 + p * 2.0;

        if let Some(t) = tex {
            let new_w = (w as f32 * scale) as u32;
            let new_h = (h as f32 * scale) as u32;
            let x = ((w as f32 - new_w as f32) / 2.0) as i32;
            let y = ((h as f32 - new_h as f32) / 2.0) as i32;

            let _ = canvas.copy(t, None, Rect::new(x, y, new_w, new_h));
        }

        // Fade out mientras zoom
        let alpha = (p * 255.0) as u8;
        canvas.set_draw_color(Color::RGBA(0, 0, 0, alpha));
        let _ = canvas.fill_rect(Rect::new(0, 0, w, h));
    }

    fn render_zoom_out(&self, canvas: &mut Canvas<Window>, w: u32, h: u32, tex: Option<&Texture>) {
        let p = self.eased_progress();
        let scale = 3.0 - p * 2.0;

        if let Some(t) = tex {
            let new_w = (w as f32 * scale) as u32;
            let new_h = (h as f32 * scale) as u32;
            let x = ((w as f32 - new_w as f32) / 2.0) as i32;
            let y = ((h as f32 - new_h as f32) / 2.0) as i32;

            let _ = canvas.copy(t, None, Rect::new(x, y, new_w, new_h));
        }

        // Fade in mientras zoom
        let alpha = ((1.0 - p) * 255.0) as u8;
        canvas.set_draw_color(Color::RGBA(0, 0, 0, alpha));
        let _ = canvas.fill_rect(Rect::new(0, 0, w, h));
    }

    fn render_circle_open(&self, canvas: &mut Canvas<Window>, w: u32, h: u32, tex: Option<&Texture>) {
        let p = self.eased_progress();
        let max_radius = ((w * w + h * h) as f32).sqrt() / 2.0;
        let radius = max_radius * p;

        if let Some(t) = tex {
            let _ = canvas.copy(t, None, Rect::new(0, 0, w, h));
        }

        // Dibujar círculos concéntricos negros desde afuera
        canvas.set_draw_color(self.fade_color);
        let steps = 20;
        for i in 0..steps {
            let r = radius + (max_radius - radius) * (i as f32 / steps as f32);
            let size = r as i32 * 2;
            let cx = (w as i32 / 2) - r as i32;
            let cy = (h as i32 / 2) - r as i32;

            // Solo dibujar si está fuera del círculo
            if r > radius {
                let thickness = ((max_radius - radius) / steps as f32) as u32;
                let _ = canvas.fill_rect(Rect::new(cx, cy, size as u32, thickness));
            }
        }
    }

    fn render_circle_close(&self, canvas: &mut Canvas<Window>, w: u32, h: u32, tex: Option<&Texture>) {
        let p = self.eased_progress();
        let max_radius = ((w * w + h * h) as f32).sqrt() / 2.0;
        let radius = max_radius * (1.0 - p);

        if let Some(t) = tex {
            let _ = canvas.copy(t, None, Rect::new(0, 0, w, h));
        }

        // Círculo negro que se cierra
        canvas.set_draw_color(self.fade_color);
        let size = (radius * 2.0) as i32;
        let x = (w as i32 / 2) - size / 2;
        let y = (h as i32 / 2) - size / 2;
        let _ = canvas.fill_rect(Rect::new(x, y, size as u32, size as u32));
    }

    fn render_blinds(&self, canvas: &mut Canvas<Window>, w: u32, h: u32, tex: Option<&Texture>, horizontal: bool) {
        let p = self.eased_progress();
        let num_blinds = 10;

        if let Some(t) = tex {
            let _ = canvas.copy(t, None, Rect::new(0, 0, w, h));
        }

        canvas.set_draw_color(self.fade_color);

        if horizontal {
            let strip_h = h / num_blinds;
            for i in 0..num_blinds {
                let y = (i * strip_h) as i32;
                let visible_h = (strip_h as f32 * (1.0 - p)) as u32;
                let remain = strip_h.saturating_sub(visible_h);
                let _ = canvas.fill_rect(Rect::new(0, y + visible_h as i32, w, remain));
            }
        } else {
            let strip_w = w / num_blinds;
            for i in 0..num_blinds {
                let x = (i * strip_w) as i32;
                let visible_w = (strip_w as f32 * (1.0 - p)) as u32;
                let remain = strip_w.saturating_sub(visible_w);
                let _ = canvas.fill_rect(Rect::new(x + visible_w as i32, 0, remain, h));
            }
        }
    }

    fn render_dissolve(&self, canvas: &mut Canvas<Window>, w: u32, h: u32, tex: Option<&Texture>) {
        let p = self.eased_progress();

        if let Some(t) = tex {
            let _ = canvas.copy(t, None, Rect::new(0, 0, w, h));
        }

        // Dibujar bloques aleatorios que aparecen
        canvas.set_draw_color(self.fade_color);
        let block_size = 12;
        let cols = w / block_size;
        let rows = h / block_size;

        // Patrón determinista basado en progreso
        let threshold = (p * 255.0) as u8;
        for row in 0..rows {
            for col in 0..cols {
                let hash = ((col * 73 + row * 137) % 256) as u8;
                if hash < threshold {
                    let x = (col * block_size) as i32;
                    let y = (row * block_size) as i32;
                    let _ = canvas.fill_rect(Rect::new(x, y, block_size, block_size));
                }
            }
        }
    }

    fn render_spiral(&self, canvas: &mut Canvas<Window>, w: u32, h: u32, tex: Option<&Texture>) {
        let p = self.eased_progress();

        if let Some(t) = tex {
            let _ = canvas.copy(t, None, Rect::new(0, 0, w, h));
        }

        canvas.set_draw_color(self.fade_color);

        let cx = w as f32 / 2.0;
        let cy = h as f32 / 2.0;
        let max_r = ((w * w + h * h) as f32).sqrt() / 2.0;

        // Dibujar espiral que cubre
        let steps = (p * 360.0 * self.spiral_turns) as i32;
        for i in 0..steps {
            let angle = (i as f32 / 180.0) * std::f32::consts::PI;
            let r = (i as f32 / (360.0 * self.spiral_turns)) * max_r;
            let x = cx + angle.cos() * r;
            let y = cy + angle.sin() * r;

            let size = 8 + (r / max_r * 16.0) as i32;
            let _ = canvas.fill_rect(Rect::new(x as i32 - size/2, y as i32 - size/2, size as u32, size as u32));
        }
    }

    fn render_checkerboard(&self, canvas: &mut Canvas<Window>, w: u32, h: u32, tex: Option<&Texture>) {
        let p = self.eased_progress();
        let size = self.checkerboard_size;
        let cols = w / size;
        let rows = h / size;

        if let Some(t) = tex {
            let _ = canvas.copy(t, None, Rect::new(0, 0, w, h));
        }

        canvas.set_draw_color(self.fade_color);

        let total = cols * rows;
        let reveal_count = (p * total as f32) as u32;

        for i in 0..reveal_count {
            let col = i % cols;
            let row = i / cols;
            let x = (col * size) as i32;
            let y = (row * size) as i32;
            let _ = canvas.fill_rect(Rect::new(x, y, size, size));
        }
    }

    fn render_pixelate(&self, canvas: &mut Canvas<Window>, w: u32, h: u32, tex: Option<&Texture>) {
        let p = self.eased_progress();
        let max_block = 64;
        let min_block = 1;
        let block_size = max_block - (p * (max_block - min_block) as f32) as u32;

        if let Some(t) = tex {
            // Dibujar versión pixelada
            // Simplificación: dibujar bloques de color
            let _ = canvas.copy(t, None, Rect::new(0, 0, w, h));
        }

        canvas.set_draw_color(self.fade_color);

        let cols = w / block_size.max(1);
        let rows = h / block_size.max(1);

        for row in 0..rows {
            for col in 0..cols {
                let hash = ((col * 31 + row * 71) % 100) as f32;
                if hash < p * 100.0 {
                    let x = (col * block_size) as i32;
                    let y = (row * block_size) as i32;
                    let _ = canvas.fill_rect(Rect::new(x, y, block_size, block_size));
                }
            }
        }
    }
}

// ============================================================================
// TRANSITION MANAGER - Gestiona múltiples transiciones en secuencia
// ============================================================================

pub struct TransitionManager {
    pub active: Option<Transition>,
    pub queue: Vec<Transition>,
}

impl TransitionManager {
    pub fn new() -> Self {
        Self {
            active: None,
            queue: Vec::new(),
        }
    }

    /// Agregar transición a la cola
    pub fn push(&mut self, transition: Transition) {
        self.queue.push(transition);
    }

    /// Iniciar transición inmediata
    pub fn start(&mut self, transition: Transition) {
        self.active = Some(transition);
    }

    /// Actualizar y obtener transición activa
    pub fn update(&mut self, dt: f32) {
        if let Some(ref mut t) = self.active {
            t.update(dt);
            if t.done {
                self.active = None;
            }
        }

        // Si no hay activa y hay cola, sacar la siguiente
        if self.active.is_none() && !self.queue.is_empty() {
            self.active = Some(self.queue.remove(0));
        }
    }

    /// Obtener transición activa
    pub fn active(&self) -> Option<&Transition> {
        self.active.as_ref()
    }

    /// Verificar si está ocupado
    pub fn is_busy(&self) -> bool {
        self.active.is_some() || !self.queue.is_empty()
    }

    /// Limpiar cola
    pub fn clear(&mut self) {
        self.queue.clear();
        self.active = None;
    }
}

impl Default for TransitionManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// HELPER: Crear transiciones comunes rápido
// ============================================================================

pub fn fade_in(duration: f32) -> Transition {
    Transition::new(TransitionType::FadeIn, duration)
}

pub fn fade_out(duration: f32) -> Transition {
    Transition::new(TransitionType::FadeOut, duration)
}

pub fn cross_fade(duration: f32) -> Transition {
    Transition::new(TransitionType::CrossFade, duration)
}

pub fn slide_left(duration: f32) -> Transition {
    Transition::new(TransitionType::SlideLeft, duration)
}

pub fn slide_right(duration: f32) -> Transition {
    Transition::new(TransitionType::SlideRight, duration)
}

pub fn slide_up(duration: f32) -> Transition {
    Transition::new(TransitionType::SlideUp, duration)
}

pub fn slide_down(duration: f32) -> Transition {
    Transition::new(TransitionType::SlideDown, duration)
}

pub fn wipe_right(duration: f32) -> Transition {
    Transition::new(TransitionType::WipeRight, duration)
}

pub fn wipe_down(duration: f32) -> Transition {
    Transition::new(TransitionType::WipeDown, duration)
}

pub fn zoom_in(duration: f32) -> Transition {
    Transition::new(TransitionType::ZoomIn, duration)
}

pub fn zoom_out(duration: f32) -> Transition {
    Transition::new(TransitionType::ZoomOut, duration)
}

pub fn circle_open(duration: f32) -> Transition {
    Transition::new(TransitionType::CircleOpen, duration)
}

pub fn circle_close(duration: f32) -> Transition {
    Transition::new(TransitionType::CircleClose, duration)
}

pub fn dissolve(duration: f32) -> Transition {
    Transition::new(TransitionType::Dissolve, duration)
}

pub fn spiral(duration: f32) -> Transition {
    Transition::new(TransitionType::Spiral, duration)
}

pub fn checkerboard(duration: f32) -> Transition {
    Transition::new(TransitionType::Checkerboard, duration)
}

pub fn pixelate(duration: f32) -> Transition {
    Transition::new(TransitionType::Pixelate, duration)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_progress() {
        let mut t = Transition::new(TransitionType::FadeIn, 1.0);
        assert_eq!(t.progress(), 0.0);
        assert!(!t.done);

        t.update(0.5);
        assert_eq!(t.progress(), 0.5);
        assert!(!t.done);

        t.update(0.5);
        assert_eq!(t.progress(), 1.0);
        assert!(t.done);
    }

    #[test]
    fn test_eased_progress() {
        let t = Transition::new(TransitionType::FadeIn, 1.0);
        // smoothstep debe dar valor suave
        assert!(t.eased_progress() >= 0.0);
        assert!(t.eased_progress() <= 1.0);
    }

    #[test]
    fn test_transition_manager() {
        let mut tm = TransitionManager::new();
        assert!(!tm.is_busy());

        tm.push(fade_in(0.5));
        tm.push(fade_out(0.5));
        assert!(tm.is_busy());

        tm.update(0.6); // Primera termina
        assert!(tm.active.is_some());
        assert_eq!(tm.queue.len(), 1);
    }

    #[test]
    fn test_helper_functions() {
        let t = fade_in(1.0);
        assert_eq!(t.kind, TransitionType::FadeIn);
        assert_eq!(t.duration, 1.0);

        let t = slide_right(0.5);
        assert_eq!(t.kind, TransitionType::SlideRight);

        let t = dissolve(2.0);
        assert_eq!(t.kind, TransitionType::Dissolve);
    }
}
