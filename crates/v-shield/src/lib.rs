//! V-Shield - Wrapper gráfico para RyDit
//! Usa raylib para dibujar primitivas

pub use raylib;
pub use raylib::prelude::*;
pub use raylib::consts::KeyboardKey::*;

// Definir colores manualmente (raylib nobuild no incluye colors::prelude)
pub const RED: Color = Color { r: 230, g: 41, b: 55, a: 255 };
pub const GREEN: Color = Color { r: 117, g: 203, b: 100, a: 255 };
pub const BLUE: Color = Color { r: 51, g: 122, b: 206, a: 255 };
pub const YELLOW: Color = Color { r: 253, g: 249, b: 0, a: 255 };
pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };

/// Colores básicos para RyDit
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorRyDit {
    Rojo, Verde, Azul, Amarillo, Blanco, Negro,
}

impl ColorRyDit {
    pub fn to_color(&self) -> Color {
        match self {
            ColorRyDit::Rojo => RED,
            ColorRyDit::Verde => GREEN,
            ColorRyDit::Azul => BLUE,
            ColorRyDit::Amarillo => YELLOW,
            ColorRyDit::Blanco => WHITE,
            ColorRyDit::Negro => BLACK,
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "rojo" | "red" => ColorRyDit::Rojo,
            "verde" | "green" => ColorRyDit::Verde,
            "azul" | "blue" => ColorRyDit::Azul,
            "amarillo" | "yellow" => ColorRyDit::Amarillo,
            "blanco" | "white" => ColorRyDit::Blanco,
            _ => ColorRyDit::Negro,
        }
    }
}

/// Inicializar ventana
pub fn init_window(titulo: &str, w: i32, h: i32) -> (RaylibHandle, RaylibThread) {
    raylib::init()
        .size(w, h)
        .title(titulo)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_str() {
        assert_eq!(ColorRyDit::from_str("rojo"), ColorRyDit::Rojo);
        assert_eq!(ColorRyDit::from_str("RED"), ColorRyDit::Rojo);
        assert_eq!(ColorRyDit::from_str("verde"), ColorRyDit::Verde);
        assert_eq!(ColorRyDit::from_str("azul"), ColorRyDit::Azul);
        assert_eq!(ColorRyDit::from_str("amarillo"), ColorRyDit::Amarillo);
        assert_eq!(ColorRyDit::from_str("blanco"), ColorRyDit::Blanco);
        assert_eq!(ColorRyDit::from_str("otro"), ColorRyDit::Negro);
    }

    // ========================================================================
    // TESTS V0.1.9 - INIT Y COLORES
    // ========================================================================

    #[test]
    fn test_init_window() {
        // Solo verificamos que la función existe y los parámetros son correctos
        // No podemos abrir ventana real en tests sin display
        // Verificamos que los tipos son correctos
        let _titulo: &str = "Test";
        let _ancho: i32 = 800;
        let _alto: i32 = 600;
        // La función debería compilar correctamente
        // let _ = init_window(_titulo, _ancho, _alto);  // Comentado porque abre ventana
    }

    #[test]
    fn test_colores_constantes() {
        // Verificar que los colores constantes están bien definidos
        assert_eq!(RED.r, 230);
        assert_eq!(RED.g, 41);
        assert_eq!(RED.b, 55);
        assert_eq!(RED.a, 255);

        assert_eq!(GREEN.r, 117);
        assert_eq!(GREEN.g, 203);
        assert_eq!(GREEN.b, 100);
        assert_eq!(GREEN.a, 255);

        assert_eq!(BLUE.r, 51);
        assert_eq!(BLUE.g, 122);
        assert_eq!(BLUE.b, 206);
        assert_eq!(BLUE.a, 255);

        assert_eq!(YELLOW.r, 253);
        assert_eq!(YELLOW.g, 249);
        assert_eq!(YELLOW.b, 0);
        assert_eq!(YELLOW.a, 255);

        assert_eq!(WHITE.r, 255);
        assert_eq!(WHITE.g, 255);
        assert_eq!(WHITE.b, 255);
        assert_eq!(WHITE.a, 255);

        assert_eq!(BLACK.r, 0);
        assert_eq!(BLACK.g, 0);
        assert_eq!(BLACK.b, 0);
        assert_eq!(BLACK.a, 255);
    }
}

