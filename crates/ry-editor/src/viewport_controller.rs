use migui::{Rect, WidgetId, Migui};

pub struct ViewportController {
    pub zoom: f32,
    pub pan_x: f32,
    pub pan_y: f32,
    pub show_grid: bool,
}

impl ViewportController {
    pub fn new() -> Self {
        Self {
            zoom: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
            show_grid: true,
        }
    }

    /// Dibuja los controles de navegación sobre el viewport
    pub fn draw_controls(&mut self, gui: &mut Migui, viewport_rect: Rect) {
        let btn_size = 24.0;
        let margin = 5.0;
        
        // Panel de controles en la esquina superior derecha del viewport
        let controls_x = viewport_rect.x + viewport_rect.w - (btn_size * 2.0 + margin * 3.0);
        let controls_y = viewport_rect.y + margin;
        
        // Botones de Zoom
        if gui.button(WidgetId::new("vp_zoom_in"), Rect::new(controls_x, controls_y, btn_size, btn_size), "+") {
            self.zoom *= 1.1;
        }
        if gui.button(WidgetId::new("vp_zoom_out"), Rect::new(controls_x + btn_size + margin, controls_y, btn_size, btn_size), "-") {
            self.zoom /= 1.1;
        }

        // Botones de Paneo (debajo del zoom)
        let pan_y = controls_y + btn_size + margin;
        if gui.button(WidgetId::new("vp_pan_up"), Rect::new(controls_x + btn_size/2.0 + margin/2.0, pan_y, btn_size, btn_size), "↑") {
            self.pan_y -= 10.0 / self.zoom;
        }
        
        let pan_row2_y = pan_y + btn_size + margin;
        if gui.button(WidgetId::new("vp_pan_left"), Rect::new(controls_x, pan_row2_y, btn_size, btn_size), "←") {
            self.pan_x -= 10.0 / self.zoom;
        }
        if gui.button(WidgetId::new("vp_pan_right"), Rect::new(controls_x + btn_size + margin, pan_row2_y, btn_size, btn_size), "→") {
            self.pan_x += 10.0 / self.zoom;
        }

        let pan_row3_y = pan_row2_y + btn_size + margin;
        if gui.button(WidgetId::new("vp_pan_down"), Rect::new(controls_x + btn_size/2.0 + margin/2.0, pan_row3_y, btn_size, btn_size), "↓") {
            self.pan_y += 10.0 / self.zoom;
        }

        // Toggle Grid
        let grid_y = pan_row3_y + btn_size + margin;
        let mut grid_active = self.show_grid;
        if gui.checkbox(WidgetId::new("vp_grid_toggle"), "Grid", &mut grid_active, Rect::new(controls_x, grid_y, btn_size * 2.0 + margin, 20.0)) {
            self.show_grid = grid_active;
        }
    }

    /// Maneja el input de mouse para mover el viewport (Pan) y Gizmos
    pub fn handle_input(&mut self, mouse_wheel: f32, is_mouse_down: bool, dx: i32, dy: i32) {
        // Zoom con scroll
        if mouse_wheel != 0.0 {
            if mouse_wheel > 0.0 {
                self.zoom *= 1.1;
            } else {
                self.zoom /= 1.1;
            }
        }

        // Paneo con botón medio (estilo motor profesional) o arrastre simple
        if is_mouse_down {
            self.pan_x += dx as f32;
            self.pan_y += dy as f32;
        }
    }
}
