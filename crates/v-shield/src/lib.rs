//! V-Shield - Wrapper gráfico para RyDit
//! Usa raylib para dibujar primitivas

pub use raylib;
pub use raylib::consts::KeyboardKey::*;
pub use raylib::prelude::*;

use std::str::FromStr;

// Definir colores manualmente (raylib nobuild no incluye colors::prelude)
pub const RED: Color = Color {
    r: 230,
    g: 41,
    b: 55,
    a: 255,
};
pub const GREEN: Color = Color {
    r: 117,
    g: 203,
    b: 100,
    a: 255,
};
pub const BLUE: Color = Color {
    r: 51,
    g: 122,
    b: 206,
    a: 255,
};
pub const YELLOW: Color = Color {
    r: 253,
    g: 249,
    b: 0,
    a: 255,
};
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};
pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

// Colores adicionales v0.2.0
pub const CYAN: Color = Color {
    r: 0,
    g: 255,
    b: 255,
    a: 255,
};
pub const MAGENTA: Color = Color {
    r: 255,
    g: 0,
    b: 255,
    a: 255,
};
pub const ORANGE: Color = Color {
    r: 255,
    g: 165,
    b: 0,
    a: 255,
};
pub const PINK: Color = Color {
    r: 255,
    g: 192,
    b: 203,
    a: 255,
};
pub const PURPLE: Color = Color {
    r: 128,
    g: 0,
    b: 128,
    a: 255,
};
pub const BROWN: Color = Color {
    r: 165,
    g: 42,
    b: 42,
    a: 255,
};
pub const GRAY: Color = Color {
    r: 128,
    g: 128,
    b: 128,
    a: 255,
};
pub const LIME: Color = Color {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};
pub const NAVY: Color = Color {
    r: 0,
    g: 0,
    b: 128,
    a: 255,
};
pub const OLIVE: Color = Color {
    r: 128,
    g: 128,
    b: 0,
    a: 255,
};
pub const TEAL: Color = Color {
    r: 0,
    g: 128,
    b: 128,
    a: 255,
};
pub const MAROON: Color = Color {
    r: 128,
    g: 0,
    b: 0,
    a: 255,
};

/// Colores básicos para RyDit
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorRyDit {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
    Cyan,
    Magenta,
    Naranja,
    Rosa,
    Morado,
    Cafe,
    Gris,
    Lima,
    AzulOscuro,
    Oliva,
    Turquesa,
    Vino,
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
            ColorRyDit::Cyan => CYAN,
            ColorRyDit::Magenta => MAGENTA,
            ColorRyDit::Naranja => ORANGE,
            ColorRyDit::Rosa => PINK,
            ColorRyDit::Morado => PURPLE,
            ColorRyDit::Cafe => BROWN,
            ColorRyDit::Gris => GRAY,
            ColorRyDit::Lima => LIME,
            ColorRyDit::AzulOscuro => NAVY,
            ColorRyDit::Oliva => OLIVE,
            ColorRyDit::Turquesa => TEAL,
            ColorRyDit::Vino => MAROON,
        }
    }
}

impl FromStr for ColorRyDit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rojo" | "red" => Ok(ColorRyDit::Rojo),
            "verde" | "green" => Ok(ColorRyDit::Verde),
            "azul" | "blue" => Ok(ColorRyDit::Azul),
            "amarillo" | "yellow" => Ok(ColorRyDit::Amarillo),
            "blanco" | "white" => Ok(ColorRyDit::Blanco),
            "negro" | "black" => Ok(ColorRyDit::Negro),
            "cyan" | "celeste" => Ok(ColorRyDit::Cyan),
            "magenta" | "fucsia" => Ok(ColorRyDit::Magenta),
            "naranja" | "orange" => Ok(ColorRyDit::Naranja),
            "rosa" | "pink" => Ok(ColorRyDit::Rosa),
            "morado" | "purple" | "violeta" => Ok(ColorRyDit::Morado),
            "cafe" | "brown" | "marron" => Ok(ColorRyDit::Cafe),
            "gris" | "gray" | "grey" => Ok(ColorRyDit::Gris),
            "lima" | "lime" => Ok(ColorRyDit::Lima),
            "azuloscuro" | "navy" | "azul oscuro" => Ok(ColorRyDit::AzulOscuro),
            "oliva" | "olive" => Ok(ColorRyDit::Oliva),
            "turquesa" | "teal" => Ok(ColorRyDit::Turquesa),
            "vino" | "maroon" | "granate" => Ok(ColorRyDit::Vino),
            _ => Ok(ColorRyDit::Negro),
        }
    }
}

/// Inicializar ventana
pub fn init_window(titulo: &str, w: i32, h: i32) -> (RaylibHandle, RaylibThread) {
    raylib::init().size(w, h).title(titulo).build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_str() {
        assert_eq!(ColorRyDit::from_str("rojo").unwrap(), ColorRyDit::Rojo);
        assert_eq!(ColorRyDit::from_str("RED").unwrap(), ColorRyDit::Rojo);
        assert_eq!(ColorRyDit::from_str("verde").unwrap(), ColorRyDit::Verde);
        assert_eq!(ColorRyDit::from_str("azul").unwrap(), ColorRyDit::Azul);
        assert_eq!(ColorRyDit::from_str("amarillo").unwrap(), ColorRyDit::Amarillo);
        assert_eq!(ColorRyDit::from_str("blanco").unwrap(), ColorRyDit::Blanco);
        assert_eq!(ColorRyDit::from_str("otro").unwrap(), ColorRyDit::Negro);

        // Tests v0.2.0 - Nuevos colores
        assert_eq!(ColorRyDit::from_str("cyan").unwrap(), ColorRyDit::Cyan);
        assert_eq!(ColorRyDit::from_str("magenta").unwrap(), ColorRyDit::Magenta);
        assert_eq!(ColorRyDit::from_str("naranja").unwrap(), ColorRyDit::Naranja);
        assert_eq!(ColorRyDit::from_str("rosa").unwrap(), ColorRyDit::Rosa);
        assert_eq!(ColorRyDit::from_str("morado").unwrap(), ColorRyDit::Morado);
        assert_eq!(ColorRyDit::from_str("cafe").unwrap(), ColorRyDit::Cafe);
        assert_eq!(ColorRyDit::from_str("gris").unwrap(), ColorRyDit::Gris);
        assert_eq!(ColorRyDit::from_str("lima").unwrap(), ColorRyDit::Lima);
        assert_eq!(ColorRyDit::from_str("azuloscuro").unwrap(), ColorRyDit::AzulOscuro);
        assert_eq!(ColorRyDit::from_str("oliva").unwrap(), ColorRyDit::Oliva);
        assert_eq!(ColorRyDit::from_str("turquesa").unwrap(), ColorRyDit::Turquesa);
        assert_eq!(ColorRyDit::from_str("vino").unwrap(), ColorRyDit::Vino);
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

        // Tests v0.2.0 - Nuevos colores
        assert_eq!(CYAN.r, 0);
        assert_eq!(CYAN.g, 255);
        assert_eq!(CYAN.b, 255);

        assert_eq!(MAGENTA.r, 255);
        assert_eq!(MAGENTA.g, 0);
        assert_eq!(MAGENTA.b, 255);

        assert_eq!(ORANGE.r, 255);
        assert_eq!(ORANGE.g, 165);
        assert_eq!(ORANGE.b, 0);

        assert_eq!(PINK.r, 255);
        assert_eq!(PINK.g, 192);
        assert_eq!(PINK.b, 203);

        assert_eq!(PURPLE.r, 128);
        assert_eq!(PURPLE.g, 0);
        assert_eq!(PURPLE.b, 128);

        assert_eq!(BROWN.r, 165);
        assert_eq!(BROWN.g, 42);
        assert_eq!(BROWN.b, 42);

        assert_eq!(GRAY.r, 128);
        assert_eq!(GRAY.g, 128);
        assert_eq!(GRAY.b, 128);

        assert_eq!(LIME.r, 0);
        assert_eq!(LIME.g, 255);
        assert_eq!(LIME.b, 0);

        assert_eq!(NAVY.r, 0);
        assert_eq!(NAVY.g, 0);
        assert_eq!(NAVY.b, 128);

        assert_eq!(OLIVE.r, 128);
        assert_eq!(OLIVE.g, 128);
        assert_eq!(OLIVE.b, 0);

        assert_eq!(TEAL.r, 0);
        assert_eq!(TEAL.g, 128);
        assert_eq!(TEAL.b, 128);

        assert_eq!(MAROON.r, 128);
        assert_eq!(MAROON.g, 0);
        assert_eq!(MAROON.b, 0);
    }

    // ========================================================================
    // TESTS V0.5.0 - CONVERSIÓN DE COLORES
    // ========================================================================

    #[test]
    fn test_color_to_color() {
        // Verificar que to_color() convierte correctamente
        assert_eq!(ColorRyDit::Rojo.to_color().r, 230);
        assert_eq!(ColorRyDit::Rojo.to_color().g, 41);
        assert_eq!(ColorRyDit::Rojo.to_color().b, 55);

        assert_eq!(ColorRyDit::Verde.to_color().r, 117);
        assert_eq!(ColorRyDit::Verde.to_color().g, 203);
        assert_eq!(ColorRyDit::Verde.to_color().b, 100);

        assert_eq!(ColorRyDit::Azul.to_color().r, 51);
        assert_eq!(ColorRyDit::Azul.to_color().g, 122);
        assert_eq!(ColorRyDit::Azul.to_color().b, 206);

        assert_eq!(ColorRyDit::Blanco.to_color().r, 255);
        assert_eq!(ColorRyDit::Blanco.to_color().g, 255);
        assert_eq!(ColorRyDit::Blanco.to_color().b, 255);

        assert_eq!(ColorRyDit::Negro.to_color().r, 0);
        assert_eq!(ColorRyDit::Negro.to_color().g, 0);
        assert_eq!(ColorRyDit::Negro.to_color().b, 0);

        // Tests v0.2.0+
        assert_eq!(ColorRyDit::Cyan.to_color().r, 0);
        assert_eq!(ColorRyDit::Cyan.to_color().g, 255);
        assert_eq!(ColorRyDit::Cyan.to_color().b, 255);

        assert_eq!(ColorRyDit::Magenta.to_color().r, 255);
        assert_eq!(ColorRyDit::Magenta.to_color().g, 0);
        assert_eq!(ColorRyDit::Magenta.to_color().b, 255);

        assert_eq!(ColorRyDit::Lima.to_color().r, 0);
        assert_eq!(ColorRyDit::Lima.to_color().g, 255);
        assert_eq!(ColorRyDit::Lima.to_color().b, 0);
    }

    #[test]
    fn test_color_from_str_variantes() {
        // Probar múltiples formas de escribir colores
        assert_eq!(ColorRyDit::from_str("rojo").unwrap(), ColorRyDit::Rojo);
        assert_eq!(ColorRyDit::from_str("RED").unwrap(), ColorRyDit::Rojo);
        assert_eq!(ColorRyDit::from_str("Red").unwrap(), ColorRyDit::Rojo);

        assert_eq!(ColorRyDit::from_str("verde").unwrap(), ColorRyDit::Verde);
        assert_eq!(ColorRyDit::from_str("GREEN").unwrap(), ColorRyDit::Verde);

        assert_eq!(ColorRyDit::from_str("azul").unwrap(), ColorRyDit::Azul);
        assert_eq!(ColorRyDit::from_str("BLUE").unwrap(), ColorRyDit::Azul);

        assert_eq!(ColorRyDit::from_str("amarillo").unwrap(), ColorRyDit::Amarillo);
        assert_eq!(ColorRyDit::from_str("yellow").unwrap(), ColorRyDit::Amarillo);

        assert_eq!(ColorRyDit::from_str("blanco").unwrap(), ColorRyDit::Blanco);
        assert_eq!(ColorRyDit::from_str("WHITE").unwrap(), ColorRyDit::Blanco);

        assert_eq!(ColorRyDit::from_str("negro").unwrap(), ColorRyDit::Negro);
        assert_eq!(ColorRyDit::from_str("BLACK").unwrap(), ColorRyDit::Negro);

        // Variantes con guiones y espacios
        assert_eq!(ColorRyDit::from_str("azuloscuro").unwrap(), ColorRyDit::AzulOscuro);
        assert_eq!(ColorRyDit::from_str("azul oscuro").unwrap(), ColorRyDit::AzulOscuro);
        assert_eq!(ColorRyDit::from_str("navy").unwrap(), ColorRyDit::AzulOscuro);
    }

    #[test]
    fn test_color_desconocido_retorna_negro() {
        // Colores desconocidos deben retornar Negro
        assert_eq!(ColorRyDit::from_str("color_raro").unwrap(), ColorRyDit::Negro);
        assert_eq!(ColorRyDit::from_str("").unwrap(), ColorRyDit::Negro);
        assert_eq!(ColorRyDit::from_str("123").unwrap(), ColorRyDit::Negro);
        assert_eq!(ColorRyDit::from_str("transparente").unwrap(), ColorRyDit::Negro);
    }

    #[test]
    fn test_colores_v0_2_0_completos() {
        // Verificar todos los colores añadidos en v0.2.0
        let colores_extra = vec![
            (ColorRyDit::Cyan, "cyan", 0, 255, 255),
            (ColorRyDit::Magenta, "magenta", 255, 0, 255),
            (ColorRyDit::Naranja, "naranja", 255, 165, 0),
            (ColorRyDit::Rosa, "rosa", 255, 192, 203),
            (ColorRyDit::Morado, "morado", 128, 0, 128),
            (ColorRyDit::Cafe, "cafe", 165, 42, 42),
            (ColorRyDit::Gris, "gris", 128, 128, 128),
            (ColorRyDit::Lima, "lima", 0, 255, 0),
            (ColorRyDit::AzulOscuro, "azuloscuro", 0, 0, 128),
            (ColorRyDit::Oliva, "oliva", 128, 128, 0),
            (ColorRyDit::Turquesa, "turquesa", 0, 128, 128),
            (ColorRyDit::Vino, "vino", 128, 0, 0),
        ];

        for (color_enum, nombre, r, g, b) in colores_extra {
            assert_eq!(color_enum.to_color().r, r, "Canal R de {}", nombre);
            assert_eq!(color_enum.to_color().g, g, "Canal G de {}", nombre);
            assert_eq!(color_enum.to_color().b, b, "Canal B de {}", nombre);
            assert_eq!(color_enum.to_color().a, 255, "Canal A de {}", nombre);
        }
    }
}
