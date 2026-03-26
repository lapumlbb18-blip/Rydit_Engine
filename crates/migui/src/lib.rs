//! # migui - Immediate Mode GUI puro en Rust
//!
//! **Sistema de GUI independiente sin dependencias gráficas**
//!
//! ## Filosofía
//! - **Immediate Mode**: Cada frame se evalúa desde cero
//! - **Sin dependencias**: Funciona en cualquier plataforma
//! - **Backend agnóstico**: Se conecta a raylib, terminal, web, etc.
//!
//! ## Ejemplo
//! ```rust
//! use migui::{Migui, Event, WidgetId};
//!
//! let mut gui = Migui::new();
//! let mut contador = 0;
//!
//! // En tu game loop:
//! // gui.begin_frame();
//! // if gui.button(WidgetId::new("btn"), rect(10, 10, 100, 30)) {
//! //     contador += 1;
//! // }
//! // gui.end_frame();
//! ```

use std::str::FromStr;

// ============================================================================
// TIPOS BÁSICOS
// ============================================================================

/// Identificador único para widgets
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WidgetId(pub String);

impl WidgetId {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Rectángulo para layout
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.w && py >= self.y && py <= self.y + self.h
    }
}

/// Colores básicos
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub const BLACK: Color = Color::new(0, 0, 0, 255);
    pub const WHITE: Color = Color::new(255, 255, 255, 255);
    pub const RED: Color = Color::new(230, 41, 55, 255);
    pub const GREEN: Color = Color::new(117, 203, 100, 255);
    pub const BLUE: Color = Color::new(51, 122, 206, 255);
    pub const YELLOW: Color = Color::new(253, 249, 0, 255);
    pub const GRAY: Color = Color::new(128, 128, 128, 255);
    pub const BG: Color = Color::new(30, 30, 30, 255);
    pub const PANEL: Color = Color::new(50, 50, 50, 255);
    pub const BUTTON: Color = Color::new(70, 70, 70, 255);
    pub const BUTTON_HOVER: Color = Color::new(90, 90, 90, 255);
    pub const BUTTON_ACTIVE: Color = Color::new(110, 110, 110, 255);
    pub const BORDER: Color = Color::new(100, 100, 100, 255);
    pub const TEXT: Color = Color::new(240, 240, 240, 255);
    pub const ACCENT: Color = Color::new(51, 122, 206, 255);
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rojo" | "red" => Ok(Color::RED),
            "verde" | "green" => Ok(Color::GREEN),
            "azul" | "blue" => Ok(Color::BLUE),
            "amarillo" | "yellow" => Ok(Color::YELLOW),
            "blanco" | "white" => Ok(Color::WHITE),
            "negro" | "black" => Ok(Color::BLACK),
            "gris" | "gray" => Ok(Color::GRAY),
            "panel" => Ok(Color::PANEL),
            "boton" | "button" => Ok(Color::BUTTON),
            "borde" | "border" => Ok(Color::BORDER),
            "texto" | "text" => Ok(Color::TEXT),
            "acento" | "accent" => Ok(Color::ACCENT),
            _ => Ok(Color::WHITE),
        }
    }
}

// ============================================================================
// EVENTOS
// ============================================================================

/// Eventos de entrada
#[derive(Debug, Clone)]
pub enum Event {
    MouseMove { x: f32, y: f32 },
    MouseDown { button: MouseButton, x: f32, y: f32 },
    MouseUp { button: MouseButton, x: f32, y: f32 },
    KeyDown { key: Key },
    KeyUp { key: Key },
    CharTyped { ch: char },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Key {
    Escape,
    Enter,
    Backspace,
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

// ============================================================================
// ESTADO DE WIDGETS
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct WidgetState {
    pub hovered: bool,
    pub active: bool,
    pub clicked: bool,
}

#[derive(Debug, Clone, Default)]
pub struct WindowState {
    pub x: f32,
    pub y: f32,
    pub dragging: bool,
    pub drag_offset_x: f32,
    pub drag_offset_y: f32,
    pub open: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TextboxState {
    pub text: String,
    pub cursor_pos: usize,
    pub selected: bool,
}

/// Estado para ListBox - v0.5.2
#[derive(Debug, Clone)]
pub struct ListboxState {
    pub items: Vec<String>,
    pub selected: Option<usize>,
    pub scroll_offset: usize,
    pub item_height: f32,
}

impl Default for ListboxState {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selected: None,
            scroll_offset: 0,
            item_height: 25.0,
        }
    }
}

/// Estado para Layout - v0.5.2
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutDir {
    Vertical,
    Horizontal,
}

/// Estado para contenedor Layout - v0.5.2
#[derive(Debug, Clone)]
pub struct LayoutState {
    pub direction: LayoutDir,
    pub spacing: f32,
    pub padding: f32,
    pub current_pos: f32,
}

impl Default for LayoutState {
    fn default() -> Self {
        Self {
            direction: LayoutDir::Vertical,
            spacing: 5.0,
            padding: 10.0,
            current_pos: 0.0,
        }
    }
}

// ============================================================================
// COMANDOS DE DIBUJO (para el backend)
// ============================================================================

/// Comandos que el backend debe ejecutar
#[derive(Debug, Clone)]
pub enum DrawCommand {
    Clear {
        color: Color,
    },
    DrawRect {
        rect: Rect,
        color: Color,
    },
    DrawText {
        text: String,
        x: f32,
        y: f32,
        size: u32,
        color: Color,
    },
    DrawLine {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        color: Color,
        thickness: f32,
    },
}

// ============================================================================
// BACKEND TRAIT
// ============================================================================

/// Trait para backends de renderizado
///
/// Permite conectar migui con diferentes sistemas gráficos:
/// - Raylib (rydit-gfx)
/// - Terminal (futuro)
/// - Web (futuro)
pub trait MiguiBackend {
    /// Limpiar pantalla con un color
    fn clear(&mut self, color: Color);

    /// Dibujar rectángulo
    fn draw_rect(&mut self, rect: Rect, color: Color);

    /// Dibujar texto
    fn draw_text(&mut self, text: &str, x: f32, y: f32, size: f32, color: Color);

    /// Dibujar línea
    fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color, thickness: f32);

    /// Ejecutar todos los comandos de dibujo de un frame
    fn render_commands(&mut self, commands: &[DrawCommand]) {
        for cmd in commands {
            match cmd {
                DrawCommand::Clear { color } => self.clear(*color),
                DrawCommand::DrawRect { rect, color } => self.draw_rect(*rect, *color),
                DrawCommand::DrawText {
                    text,
                    x,
                    y,
                    size,
                    color,
                } => {
                    self.draw_text(text, *x, *y, *size as f32, *color);
                }
                DrawCommand::DrawLine {
                    x1,
                    y1,
                    x2,
                    y2,
                    color,
                    thickness,
                } => {
                    self.draw_line(*x1, *y1, *x2, *y2, *color, *thickness);
                }
            }
        }
    }
}

// ============================================================================
// MIGUI - MAIN STRUCT
// ============================================================================

pub struct Migui {
    mouse_x: f32,
    mouse_y: f32,
    mouse_down: bool,
    mouse_pressed: bool,
    mouse_released: bool,

    pub widget_states: std::collections::HashMap<String, WidgetState>,
    pub window_states: std::collections::HashMap<String, WindowState>,
    pub textbox_states: std::collections::HashMap<String, TextboxState>,
    pub listbox_states: std::collections::HashMap<String, ListboxState>,
    pub layout_states: std::collections::HashMap<String, LayoutState>,

    draw_commands: Vec<DrawCommand>,
    frame_count: u64,
}

impl Migui {
    pub fn new() -> Self {
        Self {
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_down: false,
            mouse_pressed: false,
            mouse_released: false,
            widget_states: std::collections::HashMap::new(),
            window_states: std::collections::HashMap::new(),
            textbox_states: std::collections::HashMap::new(),
            listbox_states: std::collections::HashMap::new(),
            layout_states: std::collections::HashMap::new(),
            draw_commands: Vec::new(),
            frame_count: 0,
        }
    }

    // ========================================================================
    // FRAME MANAGEMENT
    // ========================================================================

    pub fn begin_frame(&mut self) {
        self.draw_commands.clear();
        self.mouse_pressed = false;
        self.mouse_released = false;
        self.frame_count += 1;
    }

    pub fn end_frame(&mut self) {
        // Los comandos están listos para el backend
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::MouseMove { x, y } => {
                self.mouse_x = x;
                self.mouse_y = y;
            }
            Event::MouseDown {
                button: MouseButton::Left,
                x,
                y,
            } => {
                self.mouse_x = x;
                self.mouse_y = y;
                self.mouse_down = true;
                self.mouse_pressed = true;
            }
            Event::MouseUp {
                button: MouseButton::Left,
                x,
                y,
            } => {
                self.mouse_x = x;
                self.mouse_y = y;
                self.mouse_down = false;
                self.mouse_released = true;
            }
            Event::KeyDown { key } => {
                // Manejo de teclado para textbox
                if let Some(ts) = self.textbox_states.values_mut().find(|t| t.selected) {
                    if key == Key::Backspace && ts.cursor_pos > 0 {
                        ts.cursor_pos -= 1;
                        ts.text.remove(ts.cursor_pos);
                    }
                }
            }
            Event::CharTyped { ch } => {
                if let Some(ts) = self.textbox_states.values_mut().find(|t| t.selected) {
                    ts.text.insert(ts.cursor_pos, ch);
                    ts.cursor_pos += 1;
                }
            }
            _ => {}
        }
    }

    // ========================================================================
    // QUERY METHODS
    // ========================================================================

    pub fn mouse_x(&self) -> f32 {
        self.mouse_x
    }
    pub fn mouse_y(&self) -> f32 {
        self.mouse_y
    }
    pub fn mouse_position(&self) -> (f32, f32) {
        (self.mouse_x, self.mouse_y)
    }
    pub fn is_mouse_pressed(&self) -> bool {
        self.mouse_pressed
    }
    pub fn is_mouse_down(&self) -> bool {
        self.mouse_down
    }

    pub fn draw_commands(&self) -> &[DrawCommand] {
        &self.draw_commands
    }

    // ========================================================================
    // WIDGETS
    // ========================================================================

    /// Button - retorna true si fue clickeado en este frame
    pub fn button(&mut self, id: WidgetId, rect: Rect, label: &str) -> bool {
        let state = self.widget_states.entry(id.0).or_default();

        state.hovered = rect.contains(self.mouse_x, self.mouse_y);
        if state.hovered && self.mouse_pressed {
            state.active = true;
        }
        let clicked = state.hovered && state.active && self.mouse_released;
        if !self.mouse_down {
            state.active = false;
        }

        // Comando de dibujo
        let color = if state.active {
            Color::BUTTON_ACTIVE
        } else if state.hovered {
            Color::BUTTON_HOVER
        } else {
            Color::BUTTON
        };

        self.draw_commands
            .push(DrawCommand::DrawRect { rect, color });
        self.draw_commands.push(DrawCommand::DrawLine {
            x1: rect.x,
            y1: rect.y,
            x2: rect.x + rect.w,
            y2: rect.y,
            color: Color::BORDER,
            thickness: 2.0,
        });
        // ... más líneas del borde

        // Texto centrado
        self.draw_commands.push(DrawCommand::DrawText {
            text: label.to_string(),
            x: rect.x + (rect.w - label.len() as f32 * 8.0) / 2.0,
            y: rect.y + rect.h / 2.0 - 8.0,
            size: 16,
            color: Color::TEXT,
        });

        clicked
    }

    /// Label - texto estático
    pub fn label(&mut self, _id: WidgetId, text: &str, rect: Rect) {
        self.draw_commands.push(DrawCommand::DrawText {
            text: text.to_string(),
            x: rect.x,
            y: rect.y + rect.h / 4.0,
            size: 16,
            color: Color::TEXT,
        });
    }

    /// Checkbox - retorna true si cambió el estado
    pub fn checkbox(&mut self, id: WidgetId, label: &str, checked: &mut bool, rect: Rect) -> bool {
        let state = self.widget_states.entry(id.0.clone()).or_default();

        let cb_rect = Rect::new(rect.x + 4.0, rect.y + 4.0, rect.h - 8.0, rect.h - 8.0);
        state.hovered = cb_rect.contains(self.mouse_x, self.mouse_y);

        let clicked = state.hovered && self.mouse_pressed && !state.active;
        if self.mouse_pressed {
            state.active = true;
        }
        if self.mouse_released {
            state.active = false;
        }

        // Dibujar checkbox
        let bg = if state.hovered {
            Color::BUTTON_HOVER
        } else {
            Color::BUTTON
        };
        self.draw_commands.push(DrawCommand::DrawRect {
            rect: cb_rect,
            color: bg,
        });
        self.draw_commands.push(DrawCommand::DrawLine {
            x1: cb_rect.x,
            y1: cb_rect.y,
            x2: cb_rect.x + cb_rect.w,
            y2: cb_rect.y,
            color: Color::BORDER,
            thickness: 2.0,
        });

        if *checked {
            let margin = 4.0;
            self.draw_commands.push(DrawCommand::DrawRect {
                rect: Rect::new(
                    cb_rect.x + margin,
                    cb_rect.y + margin,
                    cb_rect.w - margin * 2.0,
                    cb_rect.h - margin * 2.0,
                ),
                color: Color::ACCENT,
            });
        }

        // Label
        self.draw_commands.push(DrawCommand::DrawText {
            text: label.to_string(),
            x: rect.x + cb_rect.w + 8.0,
            y: rect.y + rect.h / 4.0,
            size: 16,
            color: Color::TEXT,
        });

        if clicked {
            *checked = !*checked;
        }
        clicked
    }

    /// Slider - retorna el valor actual
    pub fn slider(&mut self, id: WidgetId, value: f32, min: f32, max: f32, rect: Rect) -> f32 {
        let state = self.widget_states.entry(id.0).or_default();

        let track_h = 8.0f32;
        let track_y = rect.y + (rect.h - track_h) / 2.0;
        let range = max - min;
        let norm = if range > 0.0 {
            (value - min) / range
        } else {
            0.0
        };
        let knob_w = track_h;
        let knob_x = rect.x + norm * (rect.w - knob_w);

        state.hovered = rect.contains(self.mouse_x, self.mouse_y);
        let knob_hovered =
            Rect::new(knob_x, track_y, knob_w, track_h).contains(self.mouse_x, self.mouse_y);

        if (knob_hovered || state.active) && self.mouse_pressed {
            state.active = true;
        }

        let mut new_value = value;
        if state.active && self.mouse_x >= rect.x && self.mouse_x <= rect.x + rect.w {
            new_value = min + ((self.mouse_x - rect.x) / rect.w) * range;
            new_value = new_value.clamp(min, max);
        }
        if !self.mouse_down {
            state.active = false;
        }

        // Dibujar track
        self.draw_commands.push(DrawCommand::DrawRect {
            rect: Rect::new(rect.x, track_y, rect.w, track_h),
            color: Color::BUTTON,
        });

        // Dibujar knob
        let knob_color = if knob_hovered || state.active {
            Color::BUTTON_HOVER
        } else {
            Color::BUTTON
        };
        self.draw_commands.push(DrawCommand::DrawRect {
            rect: Rect::new(knob_x, track_y, knob_w, track_h),
            color: knob_color,
        });

        // Valor
        self.draw_commands.push(DrawCommand::DrawText {
            text: format!("{:.1}", new_value),
            x: rect.x + rect.w - 50.0,
            y: rect.y + rect.h / 4.0,
            size: 14,
            color: Color::TEXT,
        });

        new_value
    }

    /// Panel - contenedor visual
    pub fn panel(&mut self, _id: WidgetId, rect: Rect, color: Color) {
        self.draw_commands
            .push(DrawCommand::DrawRect { rect, color });
        self.draw_commands.push(DrawCommand::DrawLine {
            x1: rect.x,
            y1: rect.y,
            x2: rect.x + rect.w,
            y2: rect.y,
            color: Color::BORDER,
            thickness: 2.0,
        });
    }

    /// Textbox - retorna referencia al texto
    pub fn textbox(&mut self, id: WidgetId, rect: Rect) -> &str {
        let state = self.widget_states.entry(id.0.clone()).or_default();
        let ts = self.textbox_states.entry(id.0).or_default();

        state.hovered = rect.contains(self.mouse_x, self.mouse_y);
        if state.hovered && self.mouse_pressed {
            ts.selected = true;
        } else if self.mouse_pressed {
            ts.selected = false;
        }

        let bg = if ts.selected {
            Color::ACCENT
        } else if state.hovered {
            Color::BUTTON_HOVER
        } else {
            Color::BUTTON
        };

        self.draw_commands
            .push(DrawCommand::DrawRect { rect, color: bg });

        let display = if ts.selected {
            format!("{}_", ts.text)
        } else {
            ts.text.clone()
        };
        self.draw_commands.push(DrawCommand::DrawText {
            text: display,
            x: rect.x + 5.0,
            y: rect.y + rect.h / 4.0,
            size: 16,
            color: Color::TEXT,
        });

        &ts.text
    }

    pub fn set_textbox_text(&mut self, id: &str, text: String) {
        if let Some(ts) = self.textbox_states.get_mut(id) {
            ts.text = text;
        }
    }

    /// Window - ventana arrastrable, retorna true si está abierta
    pub fn window(&mut self, id: WidgetId, title: &str, rect: Rect, open: &mut bool) -> bool {
        if !*open {
            return false;
        }

        let ws = self
            .window_states
            .entry(id.0.clone())
            .or_insert_with(|| WindowState {
                x: rect.x,
                y: rect.y,
                ..Default::default()
            });

        if ws.dragging {
            if self.mouse_down {
                ws.x = self.mouse_x - ws.drag_offset_x;
                ws.y = self.mouse_y - ws.drag_offset_y;
            } else {
                ws.dragging = false;
            }
        }

        let header_h = 30.0f32;
        let header_rect = Rect::new(ws.x, ws.y, rect.w, header_h);
        let header_hovered = header_rect.contains(self.mouse_x, self.mouse_y);

        if header_hovered && self.mouse_pressed {
            ws.dragging = true;
            ws.drag_offset_x = self.mouse_x - ws.x;
            ws.drag_offset_y = self.mouse_y - ws.y;
        }

        // Cuerpo de ventana
        self.draw_commands.push(DrawCommand::DrawRect {
            rect: Rect::new(ws.x, ws.y, rect.w, rect.h),
            color: Color::PANEL,
        });

        // Header
        self.draw_commands.push(DrawCommand::DrawRect {
            rect: header_rect,
            color: Color::ACCENT,
        });

        // Título
        self.draw_commands.push(DrawCommand::DrawText {
            text: title.to_string(),
            x: ws.x + 10.0,
            y: ws.y + 7.0,
            size: 18,
            color: Color::WHITE,
        });

        // Botón cerrar
        let close_x = ws.x + rect.w - 25.0;
        let close_rect = Rect::new(close_x, ws.y + 5.0, 20.0, 20.0);
        let close_hovered = close_rect.contains(self.mouse_x, self.mouse_y);

        self.draw_commands.push(DrawCommand::DrawRect {
            rect: close_rect,
            color: if close_hovered {
                Color::RED
            } else {
                Color::new(128, 0, 0, 255)
            },
        });

        self.draw_commands.push(DrawCommand::DrawText {
            text: "X".to_string(),
            x: close_x + 5.0,
            y: ws.y + 3.0,
            size: 18,
            color: Color::WHITE,
        });

        if close_hovered && self.mouse_pressed {
            *open = false;
        }
        true
    }

    /// Dropdown - lista desplegable, retorna true si se seleccionó una opción
    /// API: dropdown(id, options[], selected_index, x, y, w, h) -> bool (cambió selección)
    pub fn dropdown(
        &mut self,
        id: WidgetId,
        options: &[&str],
        selected: &mut usize,
        rect: Rect,
    ) -> bool {
        let state = self.widget_states.entry(id.0.clone()).or_default();

        // Verificar si está abierto
        let is_open = state.active;

        // Hover detection
        state.hovered = rect.contains(self.mouse_x, self.mouse_y);

        // Click para abrir/cerrar
        if state.hovered && self.mouse_pressed {
            state.active = !state.active;
            return false;
        }

        // Si está abierto y click fuera, cerrar
        if is_open && self.mouse_pressed && !state.hovered {
            state.active = false;
            return false;
        }

        // Dibujar botón principal
        let btn_color = if state.hovered {
            Color::BUTTON_HOVER
        } else {
            Color::BUTTON
        };
        self.draw_commands.push(DrawCommand::DrawRect {
            rect,
            color: btn_color,
        });

        // Borde
        self.draw_commands.push(DrawCommand::DrawLine {
            x1: rect.x,
            y1: rect.y,
            x2: rect.x + rect.w,
            y2: rect.y,
            color: Color::BORDER,
            thickness: 2.0,
        });

        // Texto seleccionado
        let selected_text = if *selected < options.len() {
            options[*selected]
        } else {
            "Seleccionar"
        };
        self.draw_commands.push(DrawCommand::DrawText {
            text: selected_text.to_string(),
            x: rect.x + 8.0,
            y: rect.y + rect.h / 4.0,
            size: 16,
            color: Color::TEXT,
        });

        // Flecha
        let arrow_x = rect.x + rect.w - 20.0;
        let arrow_y = rect.y + rect.h / 3.0;
        self.draw_commands.push(DrawCommand::DrawLine {
            x1: arrow_x,
            y1: arrow_y,
            x2: arrow_x + 10.0,
            y2: arrow_y,
            color: Color::TEXT,
            thickness: 2.0,
        });

        let mut changed = false;

        // Si está abierto, dibujar lista desplegada
        if is_open {
            let item_h = 30.0f32;
            let list_h = options.len() as f32 * item_h;
            let list_rect = Rect::new(rect.x, rect.y + rect.h, rect.w, list_h);

            // Fondo de lista
            self.draw_commands.push(DrawCommand::DrawRect {
                rect: list_rect,
                color: Color::PANEL,
            });

            // Borde de lista
            self.draw_commands.push(DrawCommand::DrawLine {
                x1: list_rect.x,
                y1: list_rect.y,
                x2: list_rect.x + list_rect.w,
                y2: list_rect.y,
                color: Color::BORDER,
                thickness: 2.0,
            });

            // Items
            for (i, option) in options.iter().enumerate() {
                let item_rect =
                    Rect::new(rect.x, rect.y + rect.h + i as f32 * item_h, rect.w, item_h);
                let item_hovered = item_rect.contains(self.mouse_x, self.mouse_y);

                // Hover en item
                if item_hovered {
                    self.draw_commands.push(DrawCommand::DrawRect {
                        rect: item_rect,
                        color: Color::BUTTON_HOVER,
                    });
                }

                // Texto del item
                self.draw_commands.push(DrawCommand::DrawText {
                    text: option.to_string(),
                    x: rect.x + 8.0,
                    y: item_rect.y + item_rect.h / 4.0,
                    size: 16,
                    color: Color::TEXT,
                });

                // Click en item
                if item_hovered && self.mouse_pressed {
                    *selected = i;
                    changed = true;
                    state.active = false;
                }
            }
        }

        changed
    }

    /// Progress Bar - barra de progreso, vertical u horizontal
    /// API: progress_bar(id, value, min, max, x, y, w, h, vertical) -> ()
    pub fn progress_bar(
        &mut self,
        _id: WidgetId,
        value: f32,
        min: f32,
        max: f32,
        rect: Rect,
        vertical: bool,
    ) {
        // Normalizar valor
        let range = max - min;
        let norm = if range > 0.0 {
            (value - min) / range
        } else {
            0.0
        };
        let norm = norm.clamp(0.0, 1.0);

        // Fondo (track)
        self.draw_commands.push(DrawCommand::DrawRect {
            rect,
            color: Color::BUTTON,
        });

        // Borde
        self.draw_commands.push(DrawCommand::DrawLine {
            x1: rect.x,
            y1: rect.y,
            x2: rect.x + rect.w,
            y2: rect.y,
            color: Color::BORDER,
            thickness: 2.0,
        });

        // Barra de progreso
        if vertical {
            // Vertical: llena de abajo hacia arriba
            let fill_h = norm * rect.h;
            let fill_rect = Rect::new(rect.x + 4.0, rect.y + rect.h - fill_h, rect.w - 8.0, fill_h);
            self.draw_commands.push(DrawCommand::DrawRect {
                rect: fill_rect,
                color: Color::ACCENT,
            });
        } else {
            // Horizontal: llena de izquierda a derecha
            let fill_w = norm * rect.w;
            let fill_rect = Rect::new(rect.x + 4.0, rect.y + 4.0, fill_w, rect.h - 8.0);
            self.draw_commands.push(DrawCommand::DrawRect {
                rect: fill_rect,
                color: Color::GREEN,
            });
        }

        // Texto de porcentaje
        let percent = (norm * 100.0) as i32;
        let text = format!("{}%", percent);
        self.draw_commands.push(DrawCommand::DrawText {
            text,
            x: rect.x + (rect.w - 40.0) / 2.0,
            y: rect.y + rect.h / 4.0,
            size: 14,
            color: Color::TEXT,
        });
    }

    /// Message box - retorna índice del botón presionado
    pub fn message_box(&mut self, title: &str, message: &str, buttons: &[&str], rect: Rect) -> i32 {
        // Fondo
        self.draw_commands.push(DrawCommand::DrawRect {
            rect,
            color: Color::PANEL,
        });

        // Título
        self.draw_commands.push(DrawCommand::DrawText {
            text: title.to_string(),
            x: rect.x + 10.0,
            y: rect.y + 10.0,
            size: 18,
            color: Color::ACCENT,
        });

        // Mensaje
        self.draw_commands.push(DrawCommand::DrawText {
            text: message.to_string(),
            x: rect.x + 10.0,
            y: rect.y + 35.0,
            size: 16,
            color: Color::TEXT,
        });

        // Botones
        let btn_w = 80.0f32;
        let btn_h = 35.0f32;
        let btn_y = rect.y + rect.h - btn_h - 10.0;
        let total_w = buttons.len() as f32 * (btn_w + 10.0) - 10.0;
        let mut btn_x = rect.x + (rect.w - total_w) / 2.0;

        for (i, btn_text) in buttons.iter().enumerate() {
            if self.button(
                WidgetId::new(&format!("msgbox_{}_{}", title, i)),
                Rect::new(btn_x, btn_y, btn_w, btn_h),
                btn_text,
            ) {
                return i as i32;
            }
            btn_x += btn_w + 10.0;
        }

        -1
    }

    // ========================================================================
    // LISTBOX - v0.5.2
    // ========================================================================

    /// ListBox - lista de items seleccionables
    /// Retorna el índice seleccionado o None si no hay selección
    pub fn listbox(&mut self, id: WidgetId, items: &[String], rect: Rect) -> Option<usize> {
        let state = self
            .listbox_states
            .entry(id.0.clone())
            .or_insert_with(|| ListboxState {
                items: items.to_vec(),
                selected: None,
                scroll_offset: 0,
                item_height: 25.0,
            });

        // Actualizar items si cambiaron
        if state.items.len() != items.len() {
            state.items = items.to_vec();
        }

        // Fondo
        self.draw_commands.push(DrawCommand::DrawRect {
            rect,
            color: Color::BUTTON,
        });

        // Borde
        self.draw_commands.push(DrawCommand::DrawLine {
            x1: rect.x,
            y1: rect.y,
            x2: rect.x + rect.w,
            y2: rect.y,
            color: Color::BORDER,
            thickness: 2.0,
        });
        self.draw_commands.push(DrawCommand::DrawLine {
            x1: rect.x + rect.w,
            y1: rect.y,
            x2: rect.x + rect.w,
            y2: rect.y + rect.h,
            color: Color::BORDER,
            thickness: 2.0,
        });
        self.draw_commands.push(DrawCommand::DrawLine {
            x1: rect.x + rect.w,
            y1: rect.y + rect.h,
            x2: rect.x,
            y2: rect.y + rect.h,
            color: Color::BORDER,
            thickness: 2.0,
        });
        self.draw_commands.push(DrawCommand::DrawLine {
            x1: rect.x,
            y1: rect.y + rect.h,
            x2: rect.x,
            y2: rect.y,
            color: Color::BORDER,
            thickness: 2.0,
        });

        // Items visibles
        let visible_items = ((rect.h - 10.0) / state.item_height) as usize;
        let _max_scroll = state.items.len().saturating_sub(visible_items);

        for i in 0..visible_items.min(state.items.len().saturating_sub(state.scroll_offset)) {
            let item_idx = i + state.scroll_offset;
            let y = rect.y + 5.0 + (i as f32 * state.item_height);
            let item_rect = Rect::new(rect.x + 5.0, y, rect.w - 10.0, state.item_height - 2.0);

            let hovered = item_rect.contains(self.mouse_x, self.mouse_y);

            // Fondo del item
            let bg_color = if Some(item_idx) == state.selected {
                Color::ACCENT
            } else if hovered {
                Color::BUTTON_HOVER
            } else {
                Color::BUTTON
            };

            self.draw_commands.push(DrawCommand::DrawRect {
                rect: item_rect,
                color: bg_color,
            });

            // Texto del item
            self.draw_commands.push(DrawCommand::DrawText {
                text: state.items[item_idx].clone(),
                x: item_rect.x + 5.0,
                y: y + 5.0,
                size: 16,
                color: Color::TEXT,
            });

            // Click en item
            if hovered && self.mouse_pressed {
                state.selected = Some(item_idx);
            }
        }

        // Scroll simple con rueda del mouse (futuro: scrollbar)
        if rect.contains(self.mouse_x, self.mouse_y) && state.items.len() > visible_items {
            // Se puede agregar scroll con rueda aquí
        }

        state.selected
    }

    // ========================================================================
    // LAYOUTS - v0.5.2
    // ========================================================================

    /// Layout vertical - organiza widgets en columna
    pub fn begin_vertical(&mut self, id: WidgetId, rect: Rect, spacing: f32) {
        let state = self
            .layout_states
            .entry(id.0.clone())
            .or_insert_with(|| LayoutState {
                direction: LayoutDir::Vertical,
                spacing,
                padding: 5.0,
                current_pos: rect.y + 5.0,
            });

        state.current_pos = rect.y + state.padding;
        state.direction = LayoutDir::Vertical;
        state.spacing = spacing;

        // Fondo opcional (descomentar para debug)
        // self.draw_commands.push(DrawCommand::DrawRect { rect, color: Color::new(40, 40, 40, 255) });
    }

    /// Obtener posición Y para siguiente widget en layout vertical
    pub fn next_y(&mut self, id: WidgetId, height: f32) -> f32 {
        if let Some(state) = self.layout_states.get_mut(&id.0) {
            let y = state.current_pos;
            state.current_pos += height + state.spacing;
            y
        } else {
            0.0
        }
    }

    /// Finalizar layout vertical
    pub fn end_vertical(&mut self, _id: WidgetId) {
        // Limpieza opcional
    }

    /// Layout horizontal - organiza widgets en fila
    pub fn begin_horizontal(&mut self, id: WidgetId, rect: Rect, spacing: f32) {
        let state = self
            .layout_states
            .entry(id.0.clone())
            .or_insert_with(|| LayoutState {
                direction: LayoutDir::Horizontal,
                spacing,
                padding: 5.0,
                current_pos: rect.x + 5.0,
            });

        state.current_pos = rect.x + state.padding;
        state.direction = LayoutDir::Horizontal;
        state.spacing = spacing;
    }

    /// Obtener posición X para siguiente widget en layout horizontal
    pub fn next_x(&mut self, id: WidgetId, width: f32) -> f32 {
        if let Some(state) = self.layout_states.get_mut(&id.0) {
            let x = state.current_pos;
            state.current_pos += width + state.spacing;
            x
        } else {
            0.0
        }
    }

    /// Finalizar layout horizontal
    pub fn end_horizontal(&mut self, _id: WidgetId) {
        // Limpieza opcional
    }
}

impl Default for Migui {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_contains() {
        let r = Rect::new(0.0, 0.0, 100.0, 50.0);
        assert!(r.contains(50.0, 25.0));
        assert!(!r.contains(150.0, 25.0));
    }

    #[test]
    fn test_button_click() {
        let mut gui = Migui::new();
        gui.begin_frame();
        gui.handle_event(Event::MouseMove { x: 50.0, y: 50.0 });
        gui.handle_event(Event::MouseDown {
            button: MouseButton::Left,
            x: 50.0,
            y: 50.0,
        });
        gui.handle_event(Event::MouseUp {
            button: MouseButton::Left,
            x: 50.0,
            y: 50.0,
        });

        let clicked = gui.button(
            WidgetId::new("btn"),
            Rect::new(0.0, 0.0, 100.0, 100.0),
            "Click",
        );
        assert!(clicked);
    }

    #[test]
    fn test_slider_value() {
        let mut gui = Migui::new();
        gui.begin_frame();
        gui.handle_event(Event::MouseMove { x: 150.0, y: 50.0 });
        gui.handle_event(Event::MouseDown {
            button: MouseButton::Left,
            x: 150.0,
            y: 50.0,
        });

        let value = gui.slider(
            WidgetId::new("sld"),
            0.5,
            0.0,
            1.0,
            Rect::new(100.0, 40.0, 200.0, 20.0),
        );
        assert!(value >= 0.0 && value <= 1.0);
    }

    #[test]
    fn test_dropdown_select() {
        let mut gui = Migui::new();
        let mut selected = 0usize;
        let options = ["Opción 1", "Opción 2", "Opción 3"];

        gui.begin_frame();
        // Renderizar dropdown cerrado
        let changed = gui.dropdown(
            WidgetId::new("dd"),
            &options,
            &mut selected,
            Rect::new(0.0, 0.0, 200.0, 40.0),
        );

        assert!(!changed);
        assert_eq!(selected, 0);
        // Debería haber comandos de dibujo (botón, borde, texto)
        assert!(gui.draw_commands().len() >= 3);
    }

    #[test]
    fn test_dropdown_closed() {
        let mut gui = Migui::new();
        let mut selected = 0usize;
        let options = ["Opción 1", "Opción 2", "Opción 3"];

        gui.begin_frame();
        // No hacer click, solo renderizar
        let changed = gui.dropdown(
            WidgetId::new("dd"),
            &options,
            &mut selected,
            Rect::new(0.0, 0.0, 200.0, 40.0),
        );

        assert!(!changed);
        assert_eq!(selected, 0);
    }

    #[test]
    fn test_progress_bar_horizontal() {
        let mut gui = Migui::new();
        gui.begin_frame();

        gui.progress_bar(
            WidgetId::new("pb"),
            50.0,
            0.0,
            100.0,
            Rect::new(0.0, 0.0, 200.0, 30.0),
            false,
        );

        // Verificar que se generaron comandos de dibujo
        assert!(!gui.draw_commands().is_empty());
    }

    #[test]
    fn test_progress_bar_vertical() {
        let mut gui = Migui::new();
        gui.begin_frame();

        gui.progress_bar(
            WidgetId::new("pb"),
            75.0,
            0.0,
            100.0,
            Rect::new(0.0, 0.0, 30.0, 200.0),
            true,
        );

        // Verificar que se generaron comandos de dibujo
        assert!(!gui.draw_commands().is_empty());
    }

    #[test]
    fn test_progress_bar_bounds() {
        let mut gui = Migui::new();
        gui.begin_frame();

        // Valor fuera de rango (debería clampear)
        gui.progress_bar(
            WidgetId::new("pb"),
            150.0,
            0.0,
            100.0,
            Rect::new(0.0, 0.0, 200.0, 30.0),
            false,
        );
        assert!(!gui.draw_commands().is_empty());

        gui.begin_frame();
        // Valor negativo
        gui.progress_bar(
            WidgetId::new("pb"),
            -10.0,
            0.0,
            100.0,
            Rect::new(0.0, 0.0, 200.0, 30.0),
            false,
        );
        assert!(!gui.draw_commands().is_empty());
    }
}
