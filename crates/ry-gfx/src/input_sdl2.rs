// crates/rydit-gfx/src/input_sdl2.rs
// Input backend con SDL2 para Termux-X11/Android
// ✅ v0.10.4: Usa eventos (no polling) para mejor soporte Android

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;

// ============================================================================
// ESTADO DEL INPUT SDL2
// ============================================================================

/// Estado del input - se actualiza vía eventos SDL2
pub struct InputState {
    /// Teclas actualmente presionadas
    pub teclas: HashMap<Keycode, bool>,
    /// Eventos de teclas presionadas (solo un frame)
    pub teclas_pressionadas_frame: Vec<Keycode>,
    /// Mapeo de teclas SDL2 a nombres RyDit
    pub mapeo_teclas: HashMap<Keycode, String>,
}

impl InputState {
    pub fn new() -> Self {
        let mut state = Self {
            teclas: HashMap::new(),
            teclas_pressionadas_frame: Vec::new(),
            mapeo_teclas: HashMap::new(),
        };

        // Inicializar mapeo de teclas
        state.inicializar_mapeo();

        state
    }

    /// Inicializar mapeo de teclas SDL2 → nombres RyDit
    fn inicializar_mapeo(&mut self) {
        // Teclas básicas
        self.mapeo_teclas.insert(Keycode::A, "a".to_string());
        self.mapeo_teclas.insert(Keycode::B, "b".to_string());
        self.mapeo_teclas.insert(Keycode::C, "c".to_string());
        self.mapeo_teclas.insert(Keycode::D, "d".to_string());
        self.mapeo_teclas.insert(Keycode::E, "e".to_string());
        self.mapeo_teclas.insert(Keycode::F, "f".to_string());
        self.mapeo_teclas.insert(Keycode::G, "g".to_string());
        self.mapeo_teclas.insert(Keycode::H, "h".to_string());
        self.mapeo_teclas.insert(Keycode::I, "i".to_string());
        self.mapeo_teclas.insert(Keycode::J, "j".to_string());
        self.mapeo_teclas.insert(Keycode::K, "k".to_string());
        self.mapeo_teclas.insert(Keycode::L, "l".to_string());
        self.mapeo_teclas.insert(Keycode::M, "m".to_string());
        self.mapeo_teclas.insert(Keycode::N, "n".to_string());
        self.mapeo_teclas.insert(Keycode::O, "o".to_string());
        self.mapeo_teclas.insert(Keycode::P, "p".to_string());
        self.mapeo_teclas.insert(Keycode::Q, "q".to_string());
        self.mapeo_teclas.insert(Keycode::R, "r".to_string());
        self.mapeo_teclas.insert(Keycode::S, "s".to_string());
        self.mapeo_teclas.insert(Keycode::T, "t".to_string());
        self.mapeo_teclas.insert(Keycode::U, "u".to_string());
        self.mapeo_teclas.insert(Keycode::V, "v".to_string());
        self.mapeo_teclas.insert(Keycode::W, "w".to_string());
        self.mapeo_teclas.insert(Keycode::X, "x".to_string());
        self.mapeo_teclas.insert(Keycode::Y, "y".to_string());
        self.mapeo_teclas.insert(Keycode::Z, "z".to_string());

        // Números
        self.mapeo_teclas.insert(Keycode::Num1, "1".to_string());
        self.mapeo_teclas.insert(Keycode::Num2, "2".to_string());
        self.mapeo_teclas.insert(Keycode::Num3, "3".to_string());
        self.mapeo_teclas.insert(Keycode::Num4, "4".to_string());
        self.mapeo_teclas.insert(Keycode::Num5, "5".to_string());
        self.mapeo_teclas.insert(Keycode::Num6, "6".to_string());
        self.mapeo_teclas.insert(Keycode::Num7, "7".to_string());
        self.mapeo_teclas.insert(Keycode::Num8, "8".to_string());
        self.mapeo_teclas.insert(Keycode::Num9, "9".to_string());
        self.mapeo_teclas.insert(Keycode::Num0, "0".to_string());

        // Especiales
        self.mapeo_teclas
            .insert(Keycode::Space, "space".to_string());
        self.mapeo_teclas
            .insert(Keycode::Return, "enter".to_string());
        self.mapeo_teclas
            .insert(Keycode::Escape, "escape".to_string());
        self.mapeo_teclas.insert(Keycode::Tab, "tab".to_string());
        self.mapeo_teclas
            .insert(Keycode::Backspace, "backspace".to_string());

        // Flechas
        self.mapeo_teclas
            .insert(Keycode::Up, "arrow_up".to_string());
        self.mapeo_teclas
            .insert(Keycode::Down, "arrow_down".to_string());
        self.mapeo_teclas
            .insert(Keycode::Left, "arrow_left".to_string());
        self.mapeo_teclas
            .insert(Keycode::Right, "arrow_right".to_string());

        // Modificadores
        self.mapeo_teclas
            .insert(Keycode::LShift, "shift_left".to_string());
        self.mapeo_teclas
            .insert(Keycode::RShift, "shift_right".to_string());
        self.mapeo_teclas
            .insert(Keycode::LCtrl, "ctrl_left".to_string());
        self.mapeo_teclas
            .insert(Keycode::RCtrl, "ctrl_right".to_string());
        self.mapeo_teclas
            .insert(Keycode::LAlt, "alt_left".to_string());
        self.mapeo_teclas
            .insert(Keycode::RAlt, "alt_right".to_string());

        // Función
        self.mapeo_teclas.insert(Keycode::F1, "f1".to_string());
        self.mapeo_teclas.insert(Keycode::F2, "f2".to_string());
        self.mapeo_teclas.insert(Keycode::F3, "f3".to_string());
        self.mapeo_teclas.insert(Keycode::F4, "f4".to_string());
        self.mapeo_teclas.insert(Keycode::F5, "f5".to_string());
        self.mapeo_teclas.insert(Keycode::F6, "f6".to_string());
        self.mapeo_teclas.insert(Keycode::F7, "f7".to_string());
        self.mapeo_teclas.insert(Keycode::F8, "f8".to_string());
        self.mapeo_teclas.insert(Keycode::F9, "f9".to_string());
        self.mapeo_teclas.insert(Keycode::F10, "f10".to_string());
        self.mapeo_teclas.insert(Keycode::F11, "f11".to_string());
        self.mapeo_teclas.insert(Keycode::F12, "f12".to_string());

        // Navegación
        self.mapeo_teclas.insert(Keycode::Home, "home".to_string());
        self.mapeo_teclas.insert(Keycode::End, "end".to_string());
        self.mapeo_teclas
            .insert(Keycode::PageUp, "page_up".to_string());
        self.mapeo_teclas
            .insert(Keycode::PageDown, "page_down".to_string());
        self.mapeo_teclas
            .insert(Keycode::Insert, "insert".to_string());
        self.mapeo_teclas
            .insert(Keycode::Delete, "delete".to_string());

        // Android / Termux especiales
        self.mapeo_teclas
            .insert(Keycode::VolumeUp, "volumen_up".to_string());
        self.mapeo_teclas
            .insert(Keycode::VolumeDown, "volumen_down".to_string());
    }

    /// Procesar un evento SDL2
    pub fn procesar_evento(&mut self, evento: &Event) {
        match evento {
            Event::KeyDown {
                keycode: Some(keycode),
                repeat: false,
                ..
            } => {
                self.teclas.insert(*keycode, true);
                self.teclas_pressionadas_frame.push(*keycode);
            }
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => {
                self.teclas.insert(*keycode, false);
            }
            _ => {}
        }
    }

    /// Limpiar eventos del frame anterior
    pub fn limpiar_frame(&mut self) {
        self.teclas_pressionadas_frame.clear();
    }

    /// Verificar si una tecla está presionada (por nombre RyDit)
    pub fn is_key_pressed(&self, nombre: &str) -> bool {
        for (&keycode, &presionada) in &self.teclas {
            if let Some(mapeado) = self.mapeo_teclas.get(&keycode) {
                if mapeado == nombre && presionada {
                    return true;
                }
            }
        }
        false
    }

    /// Verificar si una tecla fue presionada este frame (por nombre RyDit)
    pub fn is_key_just_pressed(&self, nombre: &str) -> bool {
        for &keycode in &self.teclas_pressionadas_frame {
            if let Some(mapeado) = self.mapeo_teclas.get(&keycode) {
                if mapeado == nombre {
                    return true;
                }
            }
        }
        false
    }

    /// Obtener nombre RyDit de una tecla SDL2
    pub fn get_key_name(&self, keycode: Keycode) -> Option<&str> {
        self.mapeo_teclas.get(&keycode).map(|s| s.as_str())
    }

    /// Verificar si hay alguna tecla presionada
    pub fn alguna_tecla_presionada(&self) -> bool {
        self.teclas.values().any(|&p| p)
    }
}

impl Default for InputState {
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
    fn test_mapeo_teclas() {
        let state = InputState::new();
        assert!(state.mapeo_teclas.contains_key(&Keycode::W));
        assert!(state.mapeo_teclas.contains_key(&Keycode::Space));
        assert!(state.mapeo_teclas.contains_key(&Keycode::Escape));
    }

    #[test]
    fn test_mapeo_flechas() {
        let state = InputState::new();
        assert_eq!(
            state.mapeo_teclas.get(&Keycode::Up),
            Some(&"arrow_up".to_string())
        );
        assert_eq!(
            state.mapeo_teclas.get(&Keycode::Down),
            Some(&"arrow_down".to_string())
        );
    }
}
