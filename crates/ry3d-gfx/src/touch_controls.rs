// crates/ry3d-gfx/src/touch_controls.rs
// Controles táctiles en pantalla — estilo RayGunz/Emuladores Android
//
// Sin dependencias de SDL2. Todo con raylib FFI puro.
// Inspirado en RayGunzEngine DrawTouchControls()

use raylib::ffi::{
    Vector2, Rectangle, Color,
    DrawCircle, DrawCircleLines,
    DrawRectangleRounded,
};

fn color_alpha(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r, g, b, a }
}

// ============================================================================
// JOYSTICK VIRTUAL
// ============================================================================

#[derive(Debug, Clone)]
pub struct VirtualJoystick {
    pub center: Vector2,
    pub radius: f32,
    pub knob: Vector2,
    pub active: bool,
}

impl VirtualJoystick {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        Self {
            center: Vector2 { x, y },
            radius,
            knob: Vector2 { x, y },
            active: false,
        }
    }

    /// Actualizar posición del knob según touch
    pub fn update(&mut self, touch_pos: Vector2) {
        let dx = touch_pos.x - self.center.x;
        let dy = touch_pos.y - self.center.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist <= self.radius {
            self.knob = touch_pos;
            self.active = true;
        } else {
            let angle = dy.atan2(dx);
            self.knob.x = self.center.x + angle.cos() * self.radius;
            self.knob.y = self.center.y + angle.sin() * self.radius;
            self.active = true;
        }
    }

    /// Resetear joystick
    pub fn reset(&mut self) {
        self.knob = self.center;
        self.active = false;
    }

    /// Obtener valor normalizado (-1.0 a 1.0)
    pub fn axis(&self) -> (f32, f32) {
        if !self.active { return (0.0, 0.0); }
        let dx = (self.knob.x - self.center.x) / self.radius;
        let dy = (self.knob.y - self.center.y) / self.radius;
        (dx.clamp(-1.0, 1.0), dy.clamp(-1.0, 1.0))
    }

    /// Dibujar joystick
    pub fn draw(&self, base_color: Color, knob_color: Color) {
        unsafe {
            DrawCircleLines(self.center.x as i32, self.center.y as i32, self.radius, base_color);
            let bg = color_alpha(base_color.r, base_color.g, base_color.b, 50);
            DrawCircle(self.center.x as i32, self.center.y as i32, self.radius, bg);

            if self.active {
                DrawCircle(self.knob.x as i32, self.knob.y as i32, self.radius * 0.4, knob_color);
            } else {
                let dim = color_alpha(base_color.r, base_color.g, base_color.b, 128);
                DrawCircle(self.knob.x as i32, self.knob.y as i32, self.radius * 0.3, dim);
            }
        }
    }
}

// ============================================================================
// BOTÓN VIRTUAL
// ============================================================================

#[derive(Debug, Clone)]
pub struct VirtualButton {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub text: String,
    pub pressed: bool,
    pub just_pressed: bool,
}

impl VirtualButton {
    pub fn new(x: f32, y: f32, w: f32, h: f32, text: &str) -> Self {
        Self { x, y, w, h, text: text.to_string(), pressed: false, just_pressed: false }
    }

    /// Verificar si el punto está dentro del botón
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.w && py >= self.y && py <= self.y + self.h
    }

    /// Actualizar estado según touch
    pub fn update(&mut self, tx: f32, ty: f32, touching: bool) {
        let was = self.pressed;
        self.pressed = touching && self.contains(tx, ty);
        self.just_pressed = self.pressed && !was;
    }

    /// Limpiar just_pressed
    pub fn clear(&mut self) { self.just_pressed = false; }

    /// Dibujar botón
    pub fn draw(&self, normal: Color, pressed: Color, text_c: Color) {
        let c = if self.pressed { pressed } else { normal };
        let roundness = 0.3;
        unsafe {
            DrawRectangleRounded(
                Rectangle { x: self.x, y: self.y, width: self.w, height: self.h },
                roundness, 8, c,
            );
            if !self.text.is_empty() {
                let ts = (self.h * 0.4) as i32;
                let tx = self.x + self.w / 2.0 - 10.0;
                let ty = self.y + self.h / 2.0 - ts as f32 / 2.0;
                // Texto omitido por ahora — botones visuales sin texto
            }
        }
    }
}

// ============================================================================
// TOUCH CONTROLS — Conjunto completo
// ============================================================================

pub struct TouchControls {
    pub joy_left: VirtualJoystick,
    pub joy_right: VirtualJoystick,
    pub btn_a: VirtualButton,
    pub btn_b: VirtualButton,
    pub visible: bool,
}

impl TouchControls {
    pub fn new(screen_w: f32, screen_h: f32) -> Self {
        Self {
            joy_left: VirtualJoystick::new(120.0, screen_h - 120.0, 70.0),
            joy_right: VirtualJoystick::new(screen_w - 120.0, screen_h - 120.0, 70.0),
            btn_a: VirtualButton::new(screen_w - 200.0, screen_h - 80.0, 80.0, 50.0, "A"),
            btn_b: VirtualButton::new(screen_w - 110.0, screen_h - 80.0, 80.0, 50.0, "B"),
            visible: true,
        }
    }

    pub fn update(&mut self, touching: bool, tx: f32, ty: f32) {
        if !self.visible { return; }

        if touching {
            self.joy_left.update(Vector2 { x: tx, y: ty });
            self.joy_right.update(Vector2 { x: tx, y: ty });
        } else {
            self.joy_left.reset();
            self.joy_right.reset();
        }

        self.btn_a.update(tx, ty, touching);
        self.btn_b.update(tx, ty, touching);
        self.btn_a.clear();
        self.btn_b.clear();
    }

    pub fn draw(&self) {
        if !self.visible { return; }

        let joy_base = color_alpha(100, 100, 150, 150);
        let joy_knob = color_alpha(150, 150, 200, 200);
        let btn_n = color_alpha(80, 80, 120, 180);
        let btn_p = color_alpha(150, 150, 200, 220);
        let txt_c = color_alpha(255, 255, 255, 200);

        self.joy_left.draw(joy_base, joy_knob);
        self.joy_right.draw(joy_base, joy_knob);
        self.btn_a.draw(btn_n, btn_p, txt_c);
        self.btn_b.draw(btn_n, btn_p, txt_c);
    }

    pub fn toggle(&mut self) { self.visible = !self.visible; }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joystick_default() {
        let joy = VirtualJoystick::new(100.0, 100.0, 50.0);
        assert_eq!(joy.axis(), (0.0, 0.0));
        assert!(!joy.active);
    }

    #[test]
    fn test_button_default() {
        let btn = VirtualButton::new(0.0, 0.0, 80.0, 40.0, "A");
        assert!(!btn.pressed);
        assert!(!btn.just_pressed);
    }
}

// ============================================================================
// JOYSTICK VIRTUAL
// ============================================================================
