//! # ry-input
//!
//! Input map configurable para Ry-Dit.
//!
//! ## Arquitectura
//!
//! ```text
//! InputSource          → Tecla, botón ratón, touch, gamepad
//! InputMap             → Parser de .rydit-input + mapeo acción → sources
//! InputState           → Estado en tiempo real (pressed, just_pressed, just_released)
//! ```
//!
//! ## Ejemplo
//!
//! ```rust
//! use ry_input::{InputMap, InputSource, InputState, K};
//!
//! let mut map = InputMap::new();
//! map.add_action("move_up", vec![
//!     K!("W"),
//!     K!("Up"),
//! ]);
//!
//! let mut state = InputState::new(&map);
//! // Cada frame: state.update_key("W", true);
//! if state.is_action_pressed("move_up") {
//!     // jugador mueve arriba
//! }
//! ```

#![allow(missing_docs)]

use std::collections::HashMap;
use std::fmt;

// ============================================================================
// INPUT SOURCE — Qué puede disparar una acción
// ============================================================================

/// Fuente de input individual
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputSource {
    /// Tecla del teclado: `W`, `Up`, `Space`, `Escape`, etc.
    Key(String),
    /// Botón del ratón: `MouseLeft`, `MouseRight`, `MouseMiddle`
    MouseButton(String),
    /// Eje del ratón (delta): `MouseX`, `MouseY`
    MouseAxis(String),
    /// Zona táctil virtual (para Android/Termux)
    TouchZone(String),
    /// Botón de gamepad: `A`, `B`, `X`, `Y`, `LB`, `RB`, `Start`, etc.
    GamepadButton(String),
    /// Eje de gamepad (analógico): `LeftX`, `LeftY`, `RightX`, `RightY`, `LT`, `RT`
    GamepadAxis(String),
}

/// Helper para crear InputSource con &str sin `.to_string()`
#[macro_export]
macro_rules! src {
    (key $k:ident) => { InputSource::Key(stringify!($k).to_string()) };
    (mouse $b:ident) => { InputSource::MouseButton(stringify!($b).to_string()) };
    (axis $a:ident) => { InputSource::MouseAxis(stringify!($a).to_string()) };
    (touch $z:ident) => { InputSource::TouchZone(stringify!($z).to_string()) };
    (pad $b:ident) => { InputSource::GamepadButton(stringify!($b).to_string()) };
    (padaxis $a:ident) => { InputSource::GamepadAxis(stringify!($a).to_string()) };
}

/// Helper string: `K!("W")`, `M!("MouseLeft")`, `P!("A")`, `PA!("LeftX")`
#[macro_export]
macro_rules! K {
    ($k:literal) => { InputSource::Key($k.to_string()) };
}
#[macro_export]
macro_rules! M {
    ($b:literal) => { InputSource::MouseButton($b.to_string()) };
}
#[macro_export]
macro_rules! P {
    ($b:literal) => { InputSource::GamepadButton($b.to_string()) };
}
#[macro_export]
macro_rules! PA {
    ($a:literal) => { InputSource::GamepadAxis($a.to_string()) };
}

impl InputSource {
    /// Parsear desde string (ej: "W" → Key("W"), "MouseLeft" → MouseButton("MouseLeft"))
    pub fn parse(s: &str) -> Self {
        let s = s.trim();
        match s {
            // Mouse buttons
            "MouseLeft" | "MouseRight" | "MouseMiddle" => InputSource::MouseButton(s.to_string()),
            // Mouse axes
            "MouseX" | "MouseY" => InputSource::MouseAxis(s.to_string()),
            // Gamepad axes
            "LeftX" | "LeftY" | "RightX" | "RightY" | "LT" | "RT" | "LX" | "LY" | "RX" | "RY" => {
                InputSource::GamepadAxis(s.to_string())
            }
            // Gamepad buttons
            "A" | "B" | "X" | "Y" | "LB" | "RB" | "LS" | "RS" | "Start" | "Back" | "Guide"
            | "DPadUp" | "DPadDown" | "DPadLeft" | "DPadRight" => {
                InputSource::GamepadButton(s.to_string())
            }
            // Touch
            s if s.starts_with("Touch_") => InputSource::TouchZone(s.to_string()),
            // Everything else is a keyboard key
            _ => InputSource::Key(s.to_string()),
        }
    }

    /// Nombre legible del source
    pub fn label(&self) -> &str {
        match self {
            InputSource::Key(k) => k,
            InputSource::MouseButton(b) => b,
            InputSource::MouseAxis(a) => a,
            InputSource::TouchZone(z) => z,
            InputSource::GamepadButton(b) => b,
            InputSource::GamepadAxis(a) => a,
        }
    }
}

impl fmt::Display for InputSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

// ============================================================================
// INPUT MAP — Mapeo acción → fuentes
// ============================================================================

/// Mapa de acciones → fuentes de input
///
/// Carga desde `.rydit-input` o se construye programáticamente.
#[derive(Debug, Clone)]
pub struct InputMap {
    /// acción → vec de sources
    actions: HashMap<String, Vec<InputSource>>,
    /// Orden de inserción (para iterar estable)
    order: Vec<String>,
}

impl Default for InputMap {
    fn default() -> Self {
        Self::new()
    }
}

impl InputMap {
    /// Crear mapa vacío
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
            order: Vec::new(),
        }
    }

    /// Crear con defaults de movimiento (WASD + flechas)
    pub fn with_defaults() -> Self {
        let mut map = Self::new();
        map.add_action("move_up", vec![K!("W"), K!("Up")]);
        map.add_action("move_down", vec![K!("S"), K!("Down")]);
        map.add_action("move_left", vec![K!("A"), K!("Left")]);
        map.add_action("move_right", vec![K!("D"), K!("Right")]);
        map.add_action("jump", vec![K!("Space"), K!("W")]);
        map.add_action("pause", vec![K!("Escape"), K!("P")]);
        map.add_action("interact", vec![K!("E"), K!("Enter")]);
        map.add_action("sprint", vec![K!("LeftShift")]);
        map.add_action("crouch", vec![K!("LeftCtrl")]);
        map
    }

    /// Agregar una acción con sus sources
    pub fn add_action(&mut self, action: &str, sources: Vec<InputSource>) {
        if !self.actions.contains_key(action) {
            self.order.push(action.to_string());
        }
        self.actions.insert(action.to_string(), sources);
    }

    /// Remover una acción
    pub fn remove_action(&mut self, action: &str) {
        self.actions.remove(action);
        self.order.retain(|a| a != action);
    }

    /// Obtener sources de una acción
    pub fn get_action(&self, action: &str) -> Option<&[InputSource]> {
        self.actions.get(action).map(|v| v.as_slice())
    }

    /// Verificar si existe una acción
    pub fn has_action(&self, action: &str) -> bool {
        self.actions.contains_key(action)
    }

    /// Listar todas las acciones (en orden de inserción)
    pub fn actions(&self) -> &[String] {
        &self.order
    }

    /// Cargar desde archivo `.rydit-input`
    pub fn load(path: &str) -> Result<Self, String> {
        let contenido = std::fs::read_to_string(path)
            .map_err(|e| format!("No se pudo leer '{}': {}", path, e))?;
        Self::parse_contenido(&contenido)
    }

    /// Parsear contenido de archivo `.rydit-input`
    ///
    /// Formato:
    /// ```ini
    /// # comentario
    /// accion = Tecla1, Tecla2, MouseLeft
    /// ```
    pub fn parse_contenido(contenido: &str) -> Result<Self, String> {
        let mut map = Self::new();

        for (num, linea) in contenido.lines().enumerate() {
            let linea = linea.trim();
            if linea.is_empty() || linea.starts_with('#') {
                continue;
            }

            if let Some(idx) = linea.find('=') {
                let accion = linea[..idx].trim().to_string();
                let fuentes_str = linea[idx + 1..].trim();

                if accion.is_empty() {
                    return Err(format!("Línea {}: nombre de acción vacío", num + 1));
                }

                let sources: Vec<InputSource> = fuentes_str
                    .split(',')
                    .map(|s| InputSource::parse(s.trim()))
                    .collect();

                map.add_action(&accion, sources);
            } else {
                return Err(format!(
                    "Línea {}: formato inválido, se esperaba 'accion = fuente1, fuente2'",
                    num + 1
                ));
            }
        }

        Ok(map)
    }

    /// Guardar a archivo `.rydit-input`
    pub fn save(&self, path: &str) -> Result<(), String> {
        let contenido = self.to_contenido();
        std::fs::write(path, contenido)
            .map_err(|e| format!("No se pudo escribir '{}': {}", path, e))
    }

    /// Serializar a formato `.rydit-input`
    pub fn to_contenido(&self) -> String {
        let mut lines = vec![
            "# Ry-Dit Input Map".to_string(),
            "# Generado automáticamente por ry-input".to_string(),
            String::new(),
        ];

        for action in &self.order {
            if let Some(sources) = self.actions.get(action) {
                let fuentes: Vec<String> = sources.iter().map(|s| s.label().to_string()).collect();
                lines.push(format!("{} = {}", action, fuentes.join(", ")));
            }
        }

        lines.join("\n")
    }
}

// ============================================================================
// INPUT STATE — Estado en tiempo real
// ============================================================================

/// Estado de una tecla/botón individual
#[derive(Debug, Clone, Copy, Default)]
struct KeyState {
    pressed: bool,
    prev_pressed: bool,
}

impl KeyState {
    fn is_pressed(&self) -> bool {
        self.pressed
    }
    fn is_just_pressed(&self) -> bool {
        self.pressed && !self.prev_pressed
    }
    fn is_just_released(&self) -> bool {
        !self.pressed && self.prev_pressed
    }
}

/// Estado de input en tiempo real — se actualiza cada frame.
///
/// ## Flujo por frame
/// ```rust,ignore
/// state.begin_frame();
/// // Alimentar eventos del backend (SDL2, raylib, etc.)
/// state.update_key("W", true);
/// state.update_mouse_button("MouseLeft", false);
/// // Consultar
/// if state.is_action_pressed("jump") { ... }
/// ```
#[derive(Debug)]
pub struct InputState {
    /// Estado de cada key/botón (por label)
    keys: HashMap<String, KeyState>,
    /// Referencia al mapa de acciones
    map: InputMap,
}

impl InputState {
    /// Crear estado con un mapa de acciones
    pub fn new(map: &InputMap) -> Self {
        Self {
            keys: HashMap::new(),
            map: map.clone(),
        }
    }

    /// Obtener referencia al mapa
    pub fn map(&self) -> &InputMap {
        &self.map
    }

    /// Iniciar frame — actualiza estado previo
    pub fn begin_frame(&mut self) {
        for state in self.keys.values_mut() {
            state.prev_pressed = state.pressed;
        }
    }

    /// Actualizar estado de una tecla
    pub fn update_key(&mut self, key: &str, pressed: bool) {
        let entry = self.keys.entry(key.to_string()).or_default();
        entry.pressed = pressed;
    }

    /// Actualizar botón del ratón
    pub fn update_mouse_button(&mut self, button: &str, pressed: bool) {
        self.update_key(button, pressed);
    }

    /// Actualizar botón de gamepad
    pub fn update_gamepad_button(&mut self, button: &str, pressed: bool) {
        self.update_key(button, pressed);
    }

    /// Verificar si una acción está presionada (cualquier source)
    pub fn is_action_pressed(&self, action: &str) -> bool {
        if let Some(sources) = self.map.get_action(action) {
            sources.iter().any(|s| {
                self.keys
                    .get(s.label())
                    .map(|k| k.is_pressed())
                    .unwrap_or(false)
            })
        } else {
            false
        }
    }

    /// Verificar si una acción se acaba de presionar (este frame)
    pub fn is_action_just_pressed(&self, action: &str) -> bool {
        if let Some(sources) = self.map.get_action(action) {
            sources.iter().any(|s| {
                self.keys
                    .get(s.label())
                    .map(|k| k.is_just_pressed())
                    .unwrap_or(false)
            })
        } else {
            false
        }
    }

    /// Verificar si una acción se acaba de soltar (este frame)
    pub fn is_action_just_released(&self, action: &str) -> bool {
        if let Some(sources) = self.map.get_action(action) {
            sources.iter().any(|s| {
                self.keys
                    .get(s.label())
                    .map(|k| k.is_just_released())
                    .unwrap_or(false)
            })
        } else {
            false
        }
    }

    /// Obtener valor de un eje (mouse o gamepad) — devuelve 0.0 si no hay datos
    pub fn get_action_axis(&self, action: &str) -> f32 {
        // Para ejes: buscar sources de tipo MouseAxis o GamepadAxis
        if let Some(sources) = self.map.get_action(action) {
            for s in sources {
                match s {
                    InputSource::MouseAxis(_) | InputSource::GamepadAxis(_) => {
                        // Por ahora retorna 0.0 — en implementación real
                        // se leería del backend de input
                        return 0.0;
                    }
                    _ => {}
                }
            }
        }
        0.0
    }

    /// Remapear una acción en runtime
    pub fn rebind_action(&mut self, action: &str, sources: Vec<InputSource>) {
        self.map.add_action(action, sources);
    }

    /// Listar acciones disponibles
    pub fn list_actions(&self) -> &[String] {
        self.map.actions()
    }

    /// Verificar si una tecla o botón específico está presionado (por su label)
    pub fn is_key_pressed(&self, label: &str) -> bool {
        self.keys.get(label).map(|k| k.is_pressed()).unwrap_or(false)
    }
}

// ============================================================================
// DEFAULTS — Atajos comunes
// ============================================================================

/// Crear InputMap con configuración de juego 2D estándar
pub fn game_2d_defaults() -> InputMap {
    let mut map = InputMap::with_defaults();
    map.add_action("attack", vec![K!("J"), K!("X"), M!("MouseLeft")]);
    map.add_action("special", vec![K!("K"), K!("C"), M!("MouseRight")]);
    map.add_action("inventory", vec![K!("I"), K!("Tab")]);
    map.add_action("map", vec![K!("M")]);
    map.add_action("quick_save", vec![K!("F5")]);
    map.add_action("quick_load", vec![K!("F9")]);
    map.add_action("screenshot", vec![K!("F12")]);
    map
}

/// Crear InputMap con configuración de editor visual
pub fn editor_defaults() -> InputMap {
    let mut map = InputMap::new();
    map.add_action("select", vec![K!("Q"), M!("MouseLeft")]);
    map.add_action("move_tool", vec![K!("W")]);
    map.add_action("rotate_tool", vec![K!("E")]);
    map.add_action("scale_tool", vec![K!("R")]);
    map.add_action("undo", vec![K!("Z")]);
    map.add_action("redo", vec![K!("Y")]);
    map.add_action("delete", vec![K!("Delete"), K!("Backspace")]);
    map.add_action("duplicate", vec![K!("D")]);
    map.add_action("play", vec![K!("F5")]);
    map.add_action("stop", vec![K!("Escape")]);
    map.add_action("save", vec![K!("S")]);
    map.add_action("focus_selected", vec![K!("F")]);
    map.add_action("toggle_grid", vec![K!("G")]);
    map.add_action("toggle_snap", vec![K!("N")]);
    map.add_action("camera_reset", vec![K!("Home")]);
    map.add_action("pan_view", vec![M!("MouseMiddle")]);
    map.add_action("zoom_in", vec![K!("Equal")]);
    map.add_action("zoom_out", vec![K!("Minus")]);
    map
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key() {
        assert_eq!(InputSource::parse("W"), K!("W"));
        assert_eq!(InputSource::parse("Space"), K!("Space"));
        assert_eq!(InputSource::parse("Up"), K!("Up"));
    }

    #[test]
    fn test_parse_mouse() {
        assert_eq!(InputSource::parse("MouseLeft"), M!("MouseLeft"));
        assert_eq!(InputSource::parse("MouseX"), InputSource::MouseAxis("MouseX".to_string()));
    }

    #[test]
    fn test_parse_gamepad() {
        assert_eq!(InputSource::parse("DPadUp"), P!("DPadUp"));
        assert_eq!(InputSource::parse("LeftX"), PA!("LeftX"));
    }

    #[test]
    fn test_parse_touch() {
        assert_eq!(InputSource::parse("Touch_JoyLeft"), InputSource::TouchZone("Touch_JoyLeft".to_string()));
    }

    #[test]
    fn test_add_and_get_action() {
        let mut map = InputMap::new();
        map.add_action("move", vec![K!("W"), K!("Up")]);

        let sources = map.get_action("move").unwrap();
        assert_eq!(sources.len(), 2);
    }

    #[test]
    fn test_parse_contenido() {
        let contenido = r#"
# Movimiento
move_up = W, Up
move_down = S, Down
# Acción
attack = J, MouseLeft
"#;
        let map = InputMap::parse_contenido(contenido).unwrap();
        assert_eq!(map.actions().len(), 3);
        assert_eq!(map.get_action("attack").unwrap().len(), 2);
    }

    #[test]
    fn test_parse_contenido_invalido() {
        let contenido = "linea sin igual";
        let resultado = InputMap::parse_contenido(contenido);
        assert!(resultado.is_err());
    }

    #[test]
    fn test_save_and_load() {
        let mut map = InputMap::with_defaults();
        map.add_action("attack", vec![K!("J")]);

        let path = "test_ry_input_temp.rydit-input";
        map.save(path).unwrap();

        let loaded = InputMap::load(path).unwrap();
        assert!(loaded.has_action("move_up"));
        assert!(loaded.has_action("attack"));

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_to_contenido() {
        let mut map = InputMap::new();
        map.add_action("move_up", vec![K!("W"), K!("Up")]);

        let contenido = map.to_contenido();
        assert!(contenido.contains("move_up = W, Up"));
    }

    #[test]
    fn test_input_state_pressed() {
        let map = InputMap::with_defaults();
        let mut state = InputState::new(&map);

        state.begin_frame();
        state.update_key("W", true);

        assert!(state.is_action_pressed("move_up"));
        assert!(state.is_action_just_pressed("move_up"));
        assert!(!state.is_action_just_released("move_up"));
    }

    #[test]
    fn test_input_state_release() {
        let map = InputMap::with_defaults();
        let mut state = InputState::new(&map);

        state.begin_frame();
        state.update_key("W", true);
        state.begin_frame();
        state.update_key("W", false);

        assert!(!state.is_action_pressed("move_up"));
        assert!(!state.is_action_just_pressed("move_up"));
        assert!(state.is_action_just_released("move_up"));
    }

    #[test]
    fn test_input_state_multiple_sources() {
        let map = InputMap::with_defaults();
        let mut state = InputState::new(&map);

        state.begin_frame();
        state.update_key("Up", true);

        assert!(state.is_action_pressed("move_up"));
    }

    #[test]
    fn test_rebind_action() {
        let map = InputMap::with_defaults();
        let mut state = InputState::new(&map);

        state.rebind_action("move_up", vec![K!("I"), K!("K")]);

        state.begin_frame();
        state.update_key("I", true);

        assert!(state.is_action_pressed("move_up"));
        assert!(!state.is_action_pressed("move_down"));
    }

    #[test]
    fn test_game_2d_defaults() {
        let map = game_2d_defaults();
        assert!(map.has_action("attack"));
        assert!(map.has_action("special"));
        assert!(map.has_action("inventory"));
        assert!(map.has_action("move_up"));
        assert_eq!(map.actions().len(), 16);
    }

    #[test]
    fn test_editor_defaults() {
        let map = editor_defaults();
        assert!(map.has_action("select"));
        assert!(map.has_action("undo"));
        assert!(map.has_action("redo"));
        assert!(map.has_action("save"));
        assert!(map.has_action("play"));
    }

    #[test]
    fn test_remove_action() {
        let mut map = InputMap::with_defaults();
        assert!(map.has_action("jump"));
        map.remove_action("jump");
        assert!(!map.has_action("jump"));
    }

    #[test]
    fn test_unknown_action_returns_false() {
        let map = InputMap::new();
        let state = InputState::new(&map);

        assert!(!state.is_action_pressed("nonexistent"));
        assert!(!state.is_action_just_pressed("nonexistent"));
        assert!(!state.is_action_just_released("nonexistent"));
    }

    #[test]
    fn test_list_actions() {
        let map = InputMap::with_defaults();
        let actions = map.actions();
        assert!(!actions.is_empty());
        assert_eq!(actions[0], "move_up");
    }
}
