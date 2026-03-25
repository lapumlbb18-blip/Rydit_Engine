// crates/lizer/src/lib.rs
// Lexer + Parser para RyDit - v0.0.2

use std::fmt;

// ============================================================================
// ERRORES
// ============================================================================

/// Error de compilación RyDit
#[derive(Debug, Clone)]
pub struct RyDitError {
    pub kind: ErrorKind,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub source: String,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    UnexpectedChar,
    InvalidNumber,
    UndefinedCommand,
    SyntaxError,
    UnterminatedString,
    // Nuevos tipos de error para v0.5.1
    UnexpectedToken,
    MissingToken,
    DuplicateDefinition,
    TypeMismatch,
    DivisionByZero,
    IndexOutOfBounds,
    UndefinedVariable,
    CircularImport,
    ModuleNotFound,
    // Nuevos tipos de error para v0.5.2 - Assets
    TextureNotFound,
    SoundNotFound,
}

impl fmt::Display for RyDitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind_str = match self.kind {
            ErrorKind::UnexpectedChar => "Carácter inesperado",
            ErrorKind::InvalidNumber => "Número inválido",
            ErrorKind::UndefinedCommand => "Comando no definido",
            ErrorKind::SyntaxError => "Error de sintaxis",
            ErrorKind::UnterminatedString => "String sin cerrar",
            ErrorKind::UnexpectedToken => "Token inesperado",
            ErrorKind::MissingToken => "Token faltante",
            ErrorKind::DuplicateDefinition => "Definición duplicada",
            ErrorKind::TypeMismatch => "Tipo incompatible",
            ErrorKind::DivisionByZero => "División por cero",
            ErrorKind::IndexOutOfBounds => "Índice fuera de rango",
            ErrorKind::UndefinedVariable => "Variable no definida",
            ErrorKind::CircularImport => "Importe cíclico",
            ErrorKind::ModuleNotFound => "Módulo no encontrado",
            ErrorKind::TextureNotFound => "Textura no encontrada",
            ErrorKind::SoundNotFound => "Sonido no encontrado",
        };

        writeln!(f)?;
        writeln!(
            f,
            "  ╔══════════════════════════════════════════════════════╗"
        )?;
        writeln!(
            f,
            "  ║  🔴 ERROR DE COMPILACIÓN                             ║"
        )?;
        writeln!(
            f,
            "  ╠══════════════════════════════════════════════════════╣"
        )?;
        writeln!(f, "  ║  Tipo: {}", kind_str)?;
        writeln!(
            f,
            "  ║  Ubicación: línea {}, columna {}",
            self.line, self.column
        )?;
        writeln!(
            f,
            "  ╠══════════════════════════════════════════════════════╣"
        )?;

        // Mostrar código fuente con línea y marcador
        if !self.source.is_empty() {
            writeln!(
                f,
                "  ║  Código:                                           ║"
            )?;
            writeln!(f, "  ║    {}", self.source)?;

            // Crear marcador visual
            let marker = "→".to_string() + &" ".repeat(self.column.saturating_sub(1)) + "^";
            writeln!(f, "  ║    {}", marker)?;
        }

        writeln!(
            f,
            "  ╠══════════════════════════════════════════════════════╣"
        )?;
        writeln!(f, "  ║  Mensaje: {}", self.message)?;

        // Agregar sugerencias basadas en el tipo de error
        match self.kind {
            ErrorKind::UnterminatedString => {
                writeln!(
                    f,
                    "  ╠══════════════════════════════════════════════════════╣"
                )?;
                writeln!(
                    f,
                    "  ║  💡 Sugerencia: Agrega comillas al final del string  ║"
                )?;
            }
            ErrorKind::MissingToken => {
                writeln!(
                    f,
                    "  ╠══════════════════════════════════════════════════════╣"
                )?;
                writeln!(
                    f,
                    "  ║  💡 Sugerencia: Verifica que todos los paréntesis   ║"
                )?;
                writeln!(
                    f,
                    "  ║     y llaves estén cerrados correctamente            ║"
                )?;
            }
            ErrorKind::UndefinedVariable => {
                writeln!(
                    f,
                    "  ╠══════════════════════════════════════════════════════╣"
                )?;
                writeln!(
                    f,
                    "  ║  💡 Sugerencia: Verifica el nombre de la variable   ║"
                )?;
                writeln!(
                    f,
                    "  ║     o defínela antes de usarla                       ║"
                )?;
            }
            ErrorKind::CircularImport => {
                writeln!(
                    f,
                    "  ╠══════════════════════════════════════════════════════╣"
                )?;
                writeln!(
                    f,
                    "  ║  💡 Sugerencia: Reestructura los módulos para       ║"
                )?;
                writeln!(
                    f,
                    "  ║     evitar dependencias circulares                   ║"
                )?;
            }
            ErrorKind::ModuleNotFound => {
                writeln!(
                    f,
                    "  ╠══════════════════════════════════════════════════════╣"
                )?;
                writeln!(
                    f,
                    "  ║  💡 Sugerencia: Verifica que el archivo existe en   ║"
                )?;
                writeln!(
                    f,
                    "  ║     crates/modules/                                  ║"
                )?;
            }
            ErrorKind::TextureNotFound => {
                writeln!(
                    f,
                    "  ╠══════════════════════════════════════════════════════╣"
                )?;
                writeln!(
                    f,
                    "  ║  💡 Sugerencia: Carga la textura con                ║"
                )?;
                writeln!(
                    f,
                    "  ║     assets::load_texture() antes de usarla          ║"
                )?;
            }
            ErrorKind::SoundNotFound => {
                writeln!(
                    f,
                    "  ╠══════════════════════════════════════════════════════╣"
                )?;
                writeln!(
                    f,
                    "  ║  💡 Sugerencia: Carga el sonido con                 ║"
                )?;
                writeln!(
                    f,
                    "  ║     audio::load_sound() antes de reproducirlo       ║"
                )?;
            }
            _ => {}
        }

        writeln!(
            f,
            "  ╚══════════════════════════════════════════════════════╝"
        )?;
        writeln!(f)
    }
}

pub type Result<T> = std::result::Result<T, RyDitError>;

// ============================================================================
// TOKENS (Lexer)
// ============================================================================
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Comandos principales
    ShieldInit,
    OndaCore,
    Onif,
    Blelse,
    RyPrime,
    Ryda,               // while/ciclo
    Cada,               // for/iteración
    En,                 // "en" - para cada x en lista
    Rytmo,              // function/definición
    Return,             // return valor
    Voz,                // print/voz
    DrawCircle,         // draw.circle
    DrawRect,           // draw.rect
    DrawLine,           // draw.line
    DrawText,           // draw.text
    DrawTriangle,       // draw.triangle
    DrawRing,           // draw.ring
    DrawRectangleLines, // draw.rectangle_lines
    DrawEllipse,        // draw.ellipse
    DrawLineThick,      // draw.line_thick
    Import,             // import modulo
    As,                 // as alias - para alias de módulos
    // Input se maneja como identificador especial "input"

    // Variables y valores
    DarkSlot,      // dark.slot
    Ident(String), // nombres: delta.flow, x, jugador
    Igual,         // =
    Num(f64),      // números: 100, 0.5
    Texto(String), // strings: "hola"

    // Operadores
    Mas,   // +
    Menos, // -
    Por,   // *
    Div,   // /

    // Comparación
    Mayor,      // >
    Menor,      // <
    MayorIgual, // >=
    MenorIgual, // <=

    // Lógicos
    And, // and
    Or,  // or
    Not, // not

    // Control de flujo
    Break, // break - salir de loop

    // Delimitadores de bloque
    LlaveIzq,       // {
    LlaveDer,       // }
    ParentIzq,      // (
    ParentDer,      // )
    CorcheteIzq,    // [
    CorcheteDer,    // ]
    Punto,          // .
    DobleDosPuntos, // :: para namespaces
    Coma,           // ,

    // Comentarios
    Comentario(String), // # comentario

    // Errores
    Error(String),
}

// ============================================================================
// AST (Parser)
// ============================================================================

/// Expresiones en RyDit
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    Texto(String),
    Var(String),
    Bool(bool),
    Array(Vec<Expr>), // Array literal: [1, 2, 3]
    Index {
        array: Box<Expr>, // Expression del array
        index: Box<Expr>, // Expression del índice
    }, // Indexación: lista[0]
    BinOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        name: String,    // Nombre de la función
        args: Vec<Expr>, // Argumentos
    }, // Llamada a función: tecla_presionada("arrow_up")
}

/// Operadores binarios
#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    // Aritméticos
    Suma,  // +
    Resta, // -
    Mult,  // *
    Div,   // /

    // Comparación
    Mayor,      // >
    Menor,      // <
    Igual,      // == o =
    MayorIgual, // >=
    MenorIgual, // <=

    // Lógicos
    And,
    Or,
}

/// Operadores unarios
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
    Neg,
}

/// Statements (declaraciones) en RyDit
#[derive(Debug, Clone)]
pub enum Stmt {
    Init,            // shield.init
    Command(String), // onda.core, ryprime
    Assign {
        name: String,
        value: Expr,
    },
    IndexAssign {
        array: String,
        index: Expr,
        value: Expr,
    },
    If {
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Block(Vec<Stmt>), // { ... } - bloque de statements
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    }, // rytmo nombre(params) { ... }
    Call {
        name: String,
        args: Vec<Expr>,
    }, // nombre(args) - llamada a función
    Return(Option<Expr>), // return valor
    Expr(Expr),       // Expresión como statement
    ForEach {
        var: String,     // variable del iterador
        iterable: Expr,  // expresión a iterar (array)
        body: Vec<Stmt>, // cuerpo del loop
    }, // cada x en lista { ... }
    Break,            // break - salir de loop
    Import {
        module: String,        // nombre del módulo
        alias: Option<String>, // alias opcional (import arrays as arr)
    }, // import arrays [as arr]
    DrawCircle {
        x: Expr,
        y: Expr,
        radio: Expr,
        color: String,
    }, // draw.circle(x, y, radio, "red")
    DrawRect {
        x: Expr,
        y: Expr,
        ancho: Expr,
        alto: Expr,
        color: String,
    }, // draw.rect(x, y, ancho, alto, "red")
    DrawLine {
        x1: Expr,
        y1: Expr,
        x2: Expr,
        y2: Expr,
        color: String,
    }, // draw.line(x1, y1, x2, y2, "red")
    DrawText {
        texto: String,
        x: Expr,
        y: Expr,
        tamano: Expr,
        color: String,
    }, // draw.text("Hola", x, y, tamano, "red")
    // Statements v0.2.0 - Nuevas formas
    DrawTriangle {
        v1_x: Expr,
        v1_y: Expr,
        v2_x: Expr,
        v2_y: Expr,
        v3_x: Expr,
        v3_y: Expr,
        color: String,
    }, // draw.triangle(x1, y1, x2, y2, x3, y3, "red")
    DrawRing {
        center_x: Expr,
        center_y: Expr,
        inner_radius: Expr,
        outer_radius: Expr,
        color: String,
    }, // draw.ring(x, y, inner, outer, "red")
    DrawRectangleLines {
        x: Expr,
        y: Expr,
        ancho: Expr,
        alto: Expr,
        color: String,
    }, // draw.rectangle_lines(x, y, w, h, "red")
    DrawEllipse {
        center_x: Expr,
        center_y: Expr,
        radius_h: Expr,
        radius_v: Expr,
        color: String,
    }, // draw.ellipse(x, y, radius_h, radius_v, "red")
    DrawLineThick {
        x1: Expr,
        y1: Expr,
        x2: Expr,
        y2: Expr,
        thick: Expr,
        color: String,
    }, // draw.line_thick(x1, y1, x2, y2, thick, "red")
}

/// Programa completo
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

// ============================================================================
// LIZER (Lexer)
// ============================================================================

/// Lexer para RyDit
///
/// Convierte código fuente en tokens.
///
/// # Ejemplos
///
/// ```
/// use lizer::Lizer;
///
/// // Tokenizar shield.init
/// let tokens = Lizer::new("shield.init").scan();
/// assert!(tokens.contains(&lizer::Token::ShieldInit));
/// ```
///
/// ```
/// use lizer::Lizer;
///
/// // Tokenizar dark.slot x = 10
/// let tokens = Lizer::new("dark.slot x = 10").scan();
/// assert!(tokens.contains(&lizer::Token::DarkSlot));
/// assert!(tokens.contains(&lizer::Token::Num(10.0)));
/// ```
pub struct Lizer<'a> {
    source: &'a str,
}

impl<'a> Lizer<'a> {
    /// Crea un nuevo Lizer para el código fuente dado
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use lizer::Lizer;
    ///
    /// let lizer = Lizer::new("voz \"hola\"");
    /// let tokens = lizer.scan();
    /// assert!(tokens.contains(&lizer::Token::Voz));
    /// ```
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    /// Escanea todo el source y retorna tokens o errores
    pub fn scan(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = self.source.chars().collect();
        let mut i = 0;
        let mut line = 1;
        let mut column = 1;

        while i < chars.len() {
            // Saltar whitespace
            if chars[i].is_whitespace() {
                if chars[i] == '\n' {
                    line += 1;
                    column = 1;
                } else {
                    column += 1;
                }
                i += 1;
                continue;
            }

            // Comentario: # ...
            if chars[i] == '#' {
                let mut comment = String::new();
                let start_col = column;
                i += 1;
                column += 1;
                while i < chars.len() && chars[i] != '\n' {
                    comment.push(chars[i]);
                    i += 1;
                    column += 1;
                }
                tokens.push(Token::Comentario(format!(
                    "{} (col {})",
                    comment, start_col
                )));
                continue;
            }

            // String: "..." o '...'
            if chars[i] == '"' || chars[i] == '\'' {
                let quote_char = chars[i];
                let start_line = line;
                let start_col = column;
                i += 1;
                column += 1;
                let mut text = String::new();
                while i < chars.len() && chars[i] != quote_char {
                    // Soporte para escapes
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        match chars[i + 1] {
                            '\'' if quote_char == '\'' => {
                                text.push('\'');
                                i += 2;
                                column += 2;
                            }
                            '"' if quote_char == '"' => {
                                text.push('"');
                                i += 2;
                                column += 2;
                            }
                            '\\' => {
                                text.push('\\');
                                i += 2;
                                column += 2;
                            }
                            'n' => {
                                text.push('\n');
                                i += 2;
                                column += 2;
                            }
                            't' => {
                                text.push('\t');
                                i += 2;
                                column += 2;
                            }
                            'r' => {
                                text.push('\r');
                                i += 2;
                                column += 2;
                            }
                            _ => {
                                // Escape desconocido - tratar como carácter literal
                                text.push(chars[i]);
                                i += 1;
                                column += 1;
                            }
                        }
                    } else {
                        text.push(chars[i]);
                        i += 1;
                        column += 1;
                    }
                }
                if i >= chars.len() {
                    // String sin cerrar - error
                    tokens.push(Token::Error(format!(
                        "String sin cerrar en línea {}, columna {}",
                        start_line, start_col
                    )));
                    continue;
                }
                i += 1; // cerrar quote
                column += 1;
                tokens.push(Token::Texto(text));
                continue;
            }

            // Número: 100, 0.5, -5 (soporte UTF-8 para dígitos Unicode)
            if chars[i].is_numeric()
                || (chars[i] == '-' && i + 1 < chars.len() && chars[i + 1].is_numeric())
            {
                let start_col = column;
                let mut num_str = String::new();
                if chars[i] == '-' {
                    num_str.push(chars[i]);
                    i += 1;
                    column += 1;
                }
                while i < chars.len() && (chars[i].is_numeric() || chars[i] == '.') {
                    num_str.push(chars[i]);
                    i += 1;
                    column += 1;
                }
                if let Ok(num) = num_str.parse::<f64>() {
                    tokens.push(Token::Num(num));
                } else {
                    tokens.push(Token::Error(format!(
                        "Número inválido '{}' en columna {}",
                        num_str, start_col
                    )));
                }
                continue;
            }

            // Identificador o comando: shield.init, delta.flow, x (soporte UTF-8)
            if chars[i].is_alphabetic() || chars[i].is_alphanumeric() || chars[i] == '_' {
                let _start_col = column; // Reservado para futuros errores detallados
                let mut ident = String::new();
                while i < chars.len()
                    && (chars[i].is_alphanumeric() || chars[i] == '.' || chars[i] == '_')
                {
                    ident.push(chars[i]);
                    i += 1;
                    column += 1;
                }

                // Verificar si es comando especial
                let token = match ident.as_str() {
                    "shield.init" => Token::ShieldInit,
                    "onda.core" => Token::OndaCore,
                    "onif" => Token::Onif,
                    "blelse" => Token::Blelse,
                    "ryprime" => Token::RyPrime,
                    "dark.slot" => Token::DarkSlot,
                    "ryda" => Token::Ryda,
                    "cada" => Token::Cada,
                    "en" => Token::En,
                    "rytmo" => Token::Rytmo,
                    "return" => Token::Return,
                    "voz" => Token::Voz,
                    "draw.circle" => Token::DrawCircle,
                    "draw.rect" => Token::DrawRect,
                    "draw.line" => Token::DrawLine,
                    "draw.text" => Token::DrawText,
                    "draw.triangle" => Token::DrawTriangle,
                    "draw.ring" => Token::DrawRing,
                    "draw.rectangle_lines" => Token::DrawRectangleLines,
                    "draw.ellipse" => Token::DrawEllipse,
                    "draw.line_thick" => Token::DrawLineThick,
                    // "input" se maneja como Ident especial
                    "and" => Token::And,
                    "or" => Token::Or,
                    "not" => Token::Not,
                    "break" => Token::Break,
                    "import" => Token::Import,
                    "as" => Token::As,
                    _ => Token::Ident(ident), // Permitir identificadores genéricos
                };
                tokens.push(token);
                continue;
            }

            // Operadores y símbolos
            let start_col = column;
            match chars[i] {
                '=' => {
                    // Verificar si es == (comparación)
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        tokens.push(Token::Igual); // Token de comparación
                        i += 2;
                        column += 2;
                        continue;
                    } else {
                        tokens.push(Token::Igual); // Token de asignación (=)
                        i += 1;
                        column += 1;
                    }
                }
                '+' => tokens.push(Token::Mas),
                '-' => tokens.push(Token::Menos),
                '*' => tokens.push(Token::Por),
                '/' => tokens.push(Token::Div),
                '>' => {
                    // Verificar si es >=
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        tokens.push(Token::MayorIgual);
                        i += 2;
                        column += 2;
                        continue;
                    } else {
                        tokens.push(Token::Mayor);
                        i += 1;
                        column += 1;
                    }
                }
                '<' => {
                    // Verificar si es <=
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        tokens.push(Token::MenorIgual);
                        i += 2;
                        column += 2;
                        continue;
                    } else {
                        tokens.push(Token::Menor);
                        i += 1;
                        column += 1;
                    }
                }
                '{' => tokens.push(Token::LlaveIzq),
                '}' => tokens.push(Token::LlaveDer),
                '(' => tokens.push(Token::ParentIzq),
                ')' => tokens.push(Token::ParentDer),
                '[' => tokens.push(Token::CorcheteIzq),
                ']' => tokens.push(Token::CorcheteDer),
                '.' => tokens.push(Token::Punto),
                ':' => {
                    // Verificar si es ::
                    if i + 1 < chars.len() && chars[i + 1] == ':' {
                        tokens.push(Token::DobleDosPuntos);
                        i += 2;
                        column += 2;
                        continue;
                    } else {
                        tokens.push(Token::Error(format!(
                            "Carácter ':' no reconocido (col {})",
                            column
                        )));
                        i += 1;
                        column += 1;
                    }
                }
                ',' => tokens.push(Token::Coma),
                '!' => {
                    // Verificar si es != (diferente de)
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        tokens.push(Token::Menor); // Usaremos Not + Igual para !=
                        i += 2;
                        column += 2;
                        continue;
                    } else {
                        tokens.push(Token::Not);
                        i += 1;
                        column += 1;
                    }
                }
                '@' | '$' | '%' | '&' | '|' | '^' | '~' | '`' => {
                    // Símbolos permitidos en identificadores (para futuros usos)
                    let mut ident = String::new();
                    ident.push(chars[i]);
                    i += 1;
                    column += 1;
                    // Continuar leyendo si está seguido de alfanuméricos
                    while i < chars.len()
                        && (chars[i].is_alphanumeric() || chars[i] == '.' || chars[i] == '_')
                    {
                        ident.push(chars[i]);
                        i += 1;
                        column += 1;
                    }
                    tokens.push(Token::Ident(ident));
                }
                c => tokens.push(Token::Error(format!(
                    "Carácter '{}' no reconocido (col {})",
                    c, start_col
                ))),
            }
            i += 1;
            column += 1;
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shield_init() {
        let tokens = Lizer::new("shield.init").scan();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::ShieldInit);
    }

    #[test]
    fn test_dark_slot_numero() {
        let tokens = Lizer::new("dark.slot x = 100").scan();
        assert!(tokens.contains(&Token::DarkSlot));
        assert!(tokens.contains(&Token::Ident("x".to_string())));
        assert!(tokens.contains(&Token::Igual));
        assert!(tokens.contains(&Token::Num(100.0)));
    }

    #[test]
    fn test_dark_slot_decimal() {
        let tokens = Lizer::new("dark.slot delta.flow = 0.5").scan();
        assert!(tokens.contains(&Token::DarkSlot));
        assert!(tokens.contains(&Token::Ident("delta.flow".to_string())));
        assert!(tokens.contains(&Token::Igual));
        assert!(tokens.contains(&Token::Num(0.5)));
    }

    #[test]
    fn test_comentario() {
        let tokens = Lizer::new("shield.init # esto es un comentario").scan();
        assert!(tokens.iter().any(|t| matches!(t, Token::Comentario(_))));
    }

    #[test]
    fn test_string() {
        let tokens = Lizer::new("\"hola mundo\"").scan();
        assert!(tokens.contains(&Token::Texto("hola mundo".to_string())));
    }

    #[test]
    fn test_error_caracter_raro() {
        // Ahora @ $ % & | ^ ~ ` son válidos, usamos caracteres realmente inválidos
        let tokens = Lizer::new("€£¥").scan();
        assert!(tokens.iter().any(|t| matches!(t, Token::Error(_))));
    }

    #[test]
    fn test_string_sin_cerrar() {
        let tokens = Lizer::new("\"hola sin cerrar").scan();
        assert!(tokens.iter().any(|t| matches!(t, Token::Error(_))));
    }

    #[test]
    fn test_numero_valido() {
        let tokens = Lizer::new("123.456").scan();
        assert!(tokens.contains(&Token::Num(123.456)));
    }

    #[test]
    fn test_script_completo() {
        let tokens = Lizer::new("shield.init dark.slot x = 100 onda.core").scan();
        assert!(tokens.contains(&Token::ShieldInit));
        assert!(tokens.contains(&Token::DarkSlot));
        assert!(tokens.contains(&Token::OndaCore));
        assert!(tokens.contains(&Token::Num(100.0)));
    }

    #[test]
    fn test_multiples_lineas() {
        let tokens = Lizer::new("shield.init\nonda.core\nryprime").scan();
        assert_eq!(tokens.len(), 3);
        assert!(tokens.contains(&Token::ShieldInit));
        assert!(tokens.contains(&Token::OndaCore));
        assert!(tokens.contains(&Token::RyPrime));
    }

    #[test]
    fn test_numeros_negativos() {
        let tokens = Lizer::new("-5 -10.5").scan();
        assert!(tokens.contains(&Token::Num(-5.0)));
        assert!(tokens.contains(&Token::Num(-10.5)));
    }

    #[test]
    fn test_operadores() {
        let tokens = Lizer::new("+ - * /").scan();
        assert!(tokens.contains(&Token::Mas));
        assert!(tokens.contains(&Token::Menos));
        assert!(tokens.contains(&Token::Por));
        assert!(tokens.contains(&Token::Div));
    }

    // ========================================================================
    // TESTS V0.1.8 - MADURACIÓN
    // ========================================================================

    #[test]
    fn test_string_escape_newline() {
        let tokens = Lizer::new("\"hola\\nmundo\"").scan();
        assert!(tokens.contains(&Token::Texto("hola\nmundo".to_string())));
    }

    #[test]
    fn test_string_escape_tab() {
        let tokens = Lizer::new("\"hola\\tmundo\"").scan();
        assert!(tokens.contains(&Token::Texto("hola\tmundo".to_string())));
    }

    #[test]
    fn test_string_escape_backslash() {
        let tokens = Lizer::new("\"hola\\\\mundo\"").scan();
        assert!(tokens.contains(&Token::Texto("hola\\mundo".to_string())));
    }

    #[test]
    fn test_string_escape_carriage_return() {
        let tokens = Lizer::new("\"hola\\rmundo\"").scan();
        assert!(tokens.contains(&Token::Texto("hola\rmundo".to_string())));
    }

    #[test]
    fn test_string_escape_multiple() {
        let tokens = Lizer::new("\"linea1\\nlinea2\\ttab\"").scan();
        assert!(tokens.contains(&Token::Texto("linea1\nlinea2\ttab".to_string())));
    }

    #[test]
    fn test_string_comillas_simples() {
        let tokens = Lizer::new("'hola mundo'").scan();
        assert!(tokens.contains(&Token::Texto("hola mundo".to_string())));
    }

    #[test]
    fn test_string_comillas_simples_escape() {
        let tokens = Lizer::new("'hola\\nmundo'").scan();
        assert!(tokens.contains(&Token::Texto("hola\nmundo".to_string())));
    }

    #[test]
    fn test_string_comillas_simples_con_doble_dentro() {
        let tokens = Lizer::new("'dijo \"hola\"'").scan();
        assert!(tokens.contains(&Token::Texto("dijo \"hola\"".to_string())));
    }

    #[test]
    fn test_string_comillas_dobles_con_simple_dentro() {
        let tokens = Lizer::new("\"dijo 'hola'\"").scan();
        assert!(tokens.contains(&Token::Texto("dijo 'hola'".to_string())));
    }

    #[test]
    fn test_simbolos_identificadores() {
        let tokens = Lizer::new("@variable $valor %porcentaje").scan();
        assert!(tokens.contains(&Token::Ident("@variable".to_string())));
        assert!(tokens.contains(&Token::Ident("$valor".to_string())));
        assert!(tokens.contains(&Token::Ident("%porcentaje".to_string())));
    }

    #[test]
    fn test_simbolos_variados() {
        let tokens = Lizer::new("&amper |pipe ^caret ~tilde").scan();
        assert!(tokens.contains(&Token::Ident("&amper".to_string())));
        assert!(tokens.contains(&Token::Ident("|pipe".to_string())));
        assert!(tokens.contains(&Token::Ident("^caret".to_string())));
        assert!(tokens.contains(&Token::Ident("~tilde".to_string())));
    }

    #[test]
    fn test_not_operador() {
        let tokens = Lizer::new("!true").scan();
        assert!(tokens.contains(&Token::Not));
    }

    // ========================================================================
    // TESTS V0.1.9 - FIX CONCATENACIÓN STRING + NÚMERO
    // ========================================================================

    #[test]
    fn test_precedencia_operadores() {
        // 2 + 3 * 4 = 14 (no 20)
        let mut parser = Parser::new(Lizer::new("dark.slot x = 2 + 3 * 4").scan());
        let program = parser.parse().unwrap();

        // Verificar que parsea correctamente
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_precedencia_con_parentesis() {
        // (2 + 3) * 4 = 20
        let mut parser = Parser::new(Lizer::new("dark.slot x = (2 + 3) * 4").scan());
        let program = parser.parse().unwrap();

        // Verificar que parsea correctamente
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_precedencia_multiples_operadores() {
        // 10 - 2 * 3 + 8 / 4 = 6
        let mut parser = Parser::new(Lizer::new("dark.slot x = 10 - 2 * 3 + 8 / 4").scan());
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_simbolos_en_expresiones() {
        // Variables con símbolos deben parsear correctamente
        let mut parser = Parser::new(Lizer::new("dark.slot $precio = 100").scan());
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_concatenacion_string_mas_numero() {
        // "texto" + 123 debe parsear como expresión BinOp
        let mut parser = Parser::new(Lizer::new("dark.slot x = \"precio: \" + $precio").scan());
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_parentesis_anidados() {
        // ((2+3)*(4+5)) = 45
        let mut parser = Parser::new(Lizer::new("dark.slot x = ((2 + 3) * (4 + 5))").scan());
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_expresiones_complejas() {
        // 2+3*4-10/2 = 2+12-5 = 9
        let mut parser = Parser::new(Lizer::new("dark.slot x = 2 + 3 * 4 - 10 / 2").scan());
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_simbolos_multiple() {
        // Variables con diferentes símbolos (no múltiples del mismo)
        let tokens = Lizer::new("dark.slot $x @y %z = 10").scan();
        assert!(tokens.contains(&Token::Ident("$x".to_string())));
        assert!(tokens.contains(&Token::Ident("@y".to_string())));
        assert!(tokens.contains(&Token::Ident("%z".to_string())));
    }

    #[test]
    fn test_simbolos_con_numeros() {
        // Símbolos seguidos de números
        let tokens = Lizer::new("dark.slot $precio1 = 100").scan();
        assert!(tokens.contains(&Token::Ident("$precio1".to_string())));
        assert!(tokens.contains(&Token::Num(100.0)));
    }

    #[test]
    fn test_concatenacion_multiple() {
        // "a" + x + "b" + y
        let mut parser =
            Parser::new(Lizer::new("dark.slot x = \"a\" + var1 + \"b\" + var2").scan());
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_expresion_con_namespace() {
        // "valor: " + random::int(1, 10)
        let mut parser =
            Parser::new(Lizer::new("dark.slot x = \"valor: \" + random::int(1, 10)").scan());
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_array_con_simbolos() {
        // Arrays con variables (los símbolos se parsean como identificadores)
        let mut parser = Parser::new(Lizer::new("dark.slot arr = [a, b, c]").scan());
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_negacion_multiple() {
        // --5, ---5
        let mut parser = Parser::new(Lizer::new("dark.slot x = not not true").scan());
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_operador_and_or() {
        // true and false or true
        let mut parser = Parser::new(Lizer::new("dark.slot x = true and false or true").scan());
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_comparacion_encadenada() {
        // x > 5 and x < 10
        let mut parser = Parser::new(Lizer::new("dark.slot x = 7 > 5 and 7 < 10").scan());
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
    }
}

// ============================================================================
// PARSER
// ============================================================================

/// Parser para RyDit
///
/// Convierte tokens en un AST (Program).
///
/// # Ejemplos
///
/// ```
/// use lizer::{Lizer, Parser};
///
/// // Parsear un programa simple
/// let tokens = Lizer::new("shield.init").scan();
/// let mut parser = Parser::new(tokens);
/// let program = parser.parse().unwrap();
/// assert_eq!(program.statements.len(), 1);
/// ```
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parsea todo el programa
    pub fn parse(&mut self) -> Result<Program> {
        let mut statements = Vec::new();

        while self.pos < self.tokens.len() {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            } else {
                // Token no es un statement válido, avanzar
                self.pos += 1;
            }
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Option<Stmt>> {
        if self.pos >= self.tokens.len() {
            return Ok(None);
        }

        match &self.tokens[self.pos] {
            Token::ShieldInit => {
                self.pos += 1;
                Ok(Some(Stmt::Init))
            }
            Token::OndaCore => {
                self.pos += 1;
                Ok(Some(Stmt::Command("onda.core".to_string())))
            }
            Token::RyPrime => {
                self.pos += 1;
                Ok(Some(Stmt::Command("ryprime".to_string())))
            }
            Token::Ryda => self.parse_while(),
            Token::Cada => self.parse_foreach(),
            Token::Onif => self.parse_if(),
            Token::DarkSlot => self.parse_assignment(),
            Token::LlaveIzq => self.parse_block(),
            Token::Rytmo => self.parse_function(),
            Token::Return => self.parse_return(),
            Token::Voz => self.parse_voz(),
            Token::DrawCircle => self.parse_draw_circle(),
            Token::DrawRect => self.parse_draw_rect(),
            Token::DrawLine => self.parse_draw_line(),
            Token::DrawText => self.parse_draw_text(),
            Token::DrawTriangle => self.parse_draw_triangle(),
            Token::DrawRing => self.parse_draw_ring(),
            Token::DrawRectangleLines => self.parse_draw_rectangle_lines(),
            Token::DrawEllipse => self.parse_draw_ellipse(),
            Token::DrawLineThick => self.parse_draw_line_thick(),
            Token::Break => {
                self.pos += 1;
                Ok(Some(Stmt::Break))
            }
            Token::Import => self.parse_import(),
            Token::Ident(name) => {
                // Verificar si es input() especial
                if name == "input" {
                    self.parse_input()
                } else {
                    // Podría ser una llamada a función o una variable
                    // Solo es statement si es llamada con () o indexación []
                    self.parse_call_or_ident(name.clone())
                }
            }
            Token::Comentario(_) => {
                // No avanzar self.pos aquí - el bucle de parse() ya lo hace cuando retornamos Ok(None)
                Ok(None)
            }
            _ => {
                // Token no es un statement válido, avanzar
                self.pos += 1;
                Ok(None)
            }
        }
    }

    fn parse_if(&mut self) -> Result<Option<Stmt>> {
        // onif <condition> <then-body> [blelse <else-body>]
        self.pos += 1; // consumir onif

        // Parsear condición
        let condition = self.parse_expression()?;

        // Parsear cuerpo del then: puede ser bloque { } o statements sueltos
        let mut then_body = Vec::new();

        // Verificar si hay un bloque explícito { }
        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::LlaveIzq) {
            // Es un bloque explícito
            if let Some(Stmt::Block(stmts)) = self.parse_block()? {
                then_body = stmts;
            }
        } else {
            // Statements sueltos (hasta blelse o comando de control)
            while self.pos < self.tokens.len() {
                // blelse termina el then
                if matches!(self.tokens[self.pos], Token::Blelse) {
                    break;
                }
                // Otros comandos de control también terminan el then
                if matches!(
                    self.tokens[self.pos],
                    Token::Onif | Token::Ryda | Token::Cada
                ) {
                    break;
                }
                if let Some(stmt) = self.parse_statement()? {
                    then_body.push(stmt);
                } else {
                    break;
                }
            }
        }

        // Parsear cuerpo del else (si existe)
        let else_body = if self.pos < self.tokens.len()
            && matches!(self.tokens[self.pos], Token::Blelse)
        {
            self.pos += 1; // consumir blelse

            // Verificar si hay un bloque explícito { }
            if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::LlaveIzq) {
                // Es un bloque explícito
                if let Some(Stmt::Block(stmts)) = self.parse_block()? {
                    Some(stmts)
                } else {
                    None
                }
            } else {
                // Statements sueltos
                let mut body = Vec::new();
                while self.pos < self.tokens.len() {
                    if matches!(
                        self.tokens[self.pos],
                        Token::Onif | Token::Ryda | Token::Cada | Token::Rytmo
                    ) {
                        break;
                    }
                    if let Some(stmt) = self.parse_statement()? {
                        body.push(stmt);
                    } else {
                        break;
                    }
                }
                Some(body)
            }
        } else {
            None
        };

        Ok(Some(Stmt::If {
            condition,
            then_body,
            else_body,
        }))
    }

    fn parse_while(&mut self) -> Result<Option<Stmt>> {
        // ryda <condition> <body...>
        self.pos += 1; // consumir ryda

        // Parsear condición
        let condition = self.parse_expression()?;

        // Parsear cuerpo: puede ser un bloque { } o un solo statement
        let body =
            if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::LlaveIzq) {
                // Es un bloque - usar parse_block
                if let Some(Stmt::Block(stmts)) = self.parse_block()? {
                    stmts
                } else {
                    vec![]
                }
            } else {
                // Solo un statement
                if let Some(stmt) = self.parse_statement()? {
                    vec![stmt]
                } else {
                    vec![]
                }
            };

        Ok(Some(Stmt::While { condition, body }))
    }

    fn parse_foreach(&mut self) -> Result<Option<Stmt>> {
        // cada <var> en <iterable> { <body> }
        self.pos += 1; // consumir cada

        // Obtener nombre de la variable
        if self.pos >= self.tokens.len() {
            return Err(RyDitError {
                kind: ErrorKind::SyntaxError,
                message: "Se esperaba variable después de cada".to_string(),
                line: 1,
                column: self.pos,
                source: "cada".to_string(),
            });
        }

        let var = if let Token::Ident(v) = &self.tokens[self.pos] {
            v.clone()
        } else {
            return Err(RyDitError {
                kind: ErrorKind::SyntaxError,
                message: "Se esperaba nombre de variable".to_string(),
                line: 1,
                column: self.pos,
                source: format!("{:?}", self.tokens[self.pos]),
            });
        };
        self.pos += 1;

        // Consumir "en"
        if self.pos >= self.tokens.len() || !matches!(self.tokens[self.pos], Token::En) {
            return Err(RyDitError {
                kind: ErrorKind::SyntaxError,
                message: "Se esperaba 'en' después de la variable".to_string(),
                line: 1,
                column: self.pos,
                source: "cada".to_string(),
            });
        }
        self.pos += 1;

        // Parsear iterable (array)
        let iterable = self.parse_expression()?;

        // Parsear cuerpo { ... }
        let body =
            if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::LlaveIzq) {
                if let Some(Stmt::Block(stmts)) = self.parse_block()? {
                    stmts
                } else {
                    vec![]
                }
            } else {
                return Err(RyDitError {
                    kind: ErrorKind::SyntaxError,
                    message: "Se esperaba '{' para el cuerpo del cada".to_string(),
                    line: 1,
                    column: self.pos,
                    source: "cada".to_string(),
                });
            };

        Ok(Some(Stmt::ForEach {
            var,
            iterable,
            body,
        }))
    }

    fn parse_block(&mut self) -> Result<Option<Stmt>> {
        // { <statements...> }
        self.pos += 1; // consumir {

        let mut statements = Vec::new();

        // Parsear statements hasta encontrar }
        while self.pos < self.tokens.len() {
            if matches!(self.tokens[self.pos], Token::LlaveDer) {
                self.pos += 1; // consumir }
                return Ok(Some(Stmt::Block(statements)));
            }

            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            } else {
                // Token no es un statement válido, avanzar al siguiente
                // Esto puede pasar con tokens de expresión sueltos
                self.pos += 1;
            }
        }

        // Si llegamos aquí, no se encontró }
        Err(RyDitError {
            kind: ErrorKind::SyntaxError,
            message: "Se esperaba '}' para cerrar el bloque".to_string(),
            line: 1,
            column: self.pos,
            source: "{".to_string(),
        })
    }

    fn parse_function(&mut self) -> Result<Option<Stmt>> {
        // rytmo <name> ( params... ) { body... }
        self.pos += 1; // consumir rytmo

        // Obtener nombre de la función
        if self.pos >= self.tokens.len() {
            return Err(RyDitError {
                kind: ErrorKind::SyntaxError,
                message: "Se esperaba nombre de función después de rytmo".to_string(),
                line: 1,
                column: self.pos,
                source: "rytmo".to_string(),
            });
        }

        let name = if let Token::Ident(n) = &self.tokens[self.pos] {
            n.clone()
        } else {
            return Err(RyDitError {
                kind: ErrorKind::SyntaxError,
                message: "Nombre de función inválido".to_string(),
                line: 1,
                column: self.pos,
                source: format!("{:?}", self.tokens[self.pos]),
            });
        };
        self.pos += 1;

        // Parsear parámetros ( )
        let mut params = vec![];
        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentIzq) {
            self.pos += 1; // consumir (

            // Parsear parámetros hasta encontrar )
            while self.pos < self.tokens.len() && !matches!(self.tokens[self.pos], Token::ParentDer)
            {
                if let Token::Ident(p) = &self.tokens[self.pos] {
                    params.push(p.clone());
                }
                self.pos += 1;
            }

            // Consumir )
            if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentDer) {
                self.pos += 1;
            }
        }

        // Parsear cuerpo { ... }
        let body =
            if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::LlaveIzq) {
                if let Some(Stmt::Block(stmts)) = self.parse_block()? {
                    stmts
                } else {
                    vec![]
                }
            } else {
                vec![]
            };

        Ok(Some(Stmt::Function { name, params, body }))
    }

    fn parse_voz(&mut self) -> Result<Option<Stmt>> {
        // voz <expr>
        self.pos += 1; // consumir voz

        // Parsear expresión a imprimir
        if self.pos < self.tokens.len() {
            let expr = self.parse_expression()?;
            return Ok(Some(Stmt::Expr(expr))); // Reusamos Expr statement
        }

        Ok(None)
    }

    fn parse_draw_circle(&mut self) -> Result<Option<Stmt>> {
        // draw.circle(x, y, radio, "color")
        self.pos += 1; // consumir draw.circle

        // Consumir (
        if self.pos >= self.tokens.len() || !matches!(self.tokens[self.pos], Token::ParentIzq) {
            return Err(self.error_syntax("Se esperaba '(' después de draw.circle"));
        }
        self.pos += 1;

        // Parsear x
        let x = self.parse_expression()?;
        self.skip_comma();

        // Parsear y
        let y = self.parse_expression()?;
        self.skip_comma();

        // Parsear radio
        let radio = self.parse_expression()?;
        self.skip_comma();

        // Parsear color (string)
        let color = if self.pos < self.tokens.len() {
            if let Token::Texto(c) = &self.tokens[self.pos] {
                self.pos += 1;
                c.clone()
            } else {
                "negro".to_string()
            }
        } else {
            "negro".to_string()
        };

        // Consumir )
        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentDer) {
            self.pos += 1;
        }

        Ok(Some(Stmt::DrawCircle { x, y, radio, color }))
    }

    fn parse_draw_rect(&mut self) -> Result<Option<Stmt>> {
        // draw.rect(x, y, ancho, alto, "color")
        self.pos += 1; // consumir draw.rect

        if self.pos >= self.tokens.len() || !matches!(self.tokens[self.pos], Token::ParentIzq) {
            return Err(self.error_syntax("Se esperaba '(' después de draw.rect"));
        }
        self.pos += 1;

        let x = self.parse_expression()?;
        self.skip_comma();
        let y = self.parse_expression()?;
        self.skip_comma();
        let ancho = self.parse_expression()?;
        self.skip_comma();
        let alto = self.parse_expression()?;
        self.skip_comma();

        let color = if self.pos < self.tokens.len() {
            if let Token::Texto(c) = &self.tokens[self.pos] {
                self.pos += 1;
                c.clone()
            } else {
                "negro".to_string()
            }
        } else {
            "negro".to_string()
        };

        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentDer) {
            self.pos += 1;
        }

        Ok(Some(Stmt::DrawRect {
            x,
            y,
            ancho,
            alto,
            color,
        }))
    }

    fn parse_draw_line(&mut self) -> Result<Option<Stmt>> {
        // draw.line(x1, y1, x2, y2, "color")
        self.pos += 1; // consumir draw.line

        if self.pos >= self.tokens.len() || !matches!(self.tokens[self.pos], Token::ParentIzq) {
            return Err(self.error_syntax("Se esperaba '(' después de draw.line"));
        }
        self.pos += 1;

        let x1 = self.parse_expression()?;
        self.skip_comma();
        let y1 = self.parse_expression()?;
        self.skip_comma();
        let x2 = self.parse_expression()?;
        self.skip_comma();
        let y2 = self.parse_expression()?;
        self.skip_comma();

        let color = if self.pos < self.tokens.len() {
            if let Token::Texto(c) = &self.tokens[self.pos] {
                self.pos += 1;
                c.clone()
            } else {
                "negro".to_string()
            }
        } else {
            "negro".to_string()
        };

        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentDer) {
            self.pos += 1;
        }

        Ok(Some(Stmt::DrawLine {
            x1,
            y1,
            x2,
            y2,
            color,
        }))
    }

    fn parse_draw_text(&mut self) -> Result<Option<Stmt>> {
        // draw.text("texto", x, y, tamano, "color")
        self.pos += 1; // consumir draw.text

        if self.pos >= self.tokens.len() || !matches!(self.tokens[self.pos], Token::ParentIzq) {
            return Err(self.error_syntax("Se esperaba '(' después de draw.text"));
        }
        self.pos += 1;

        // Parsear texto (string)
        let texto = if self.pos < self.tokens.len() {
            if let Token::Texto(t) = &self.tokens[self.pos] {
                self.pos += 1;
                t.clone()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };
        self.skip_comma();

        let x = self.parse_expression()?;
        self.skip_comma();
        let y = self.parse_expression()?;
        self.skip_comma();
        let tamano = self.parse_expression()?;
        self.skip_comma();

        let color = if self.pos < self.tokens.len() {
            if let Token::Texto(c) = &self.tokens[self.pos] {
                self.pos += 1;
                c.clone()
            } else {
                "blanco".to_string()
            }
        } else {
            "blanco".to_string()
        };

        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentDer) {
            self.pos += 1;
        }

        Ok(Some(Stmt::DrawText {
            texto,
            x,
            y,
            tamano,
            color,
        }))
    }

    // ========================================================================
    // FUNCIONES DE PARSING V0.2.0 - NUEVAS FORMAS
    // ========================================================================

    fn parse_draw_triangle(&mut self) -> Result<Option<Stmt>> {
        // draw.triangle(x1, y1, x2, y2, x3, y3, "color")
        self.pos += 1;
        if self.pos >= self.tokens.len() || !matches!(self.tokens[self.pos], Token::ParentIzq) {
            return Err(self.error_syntax("Se esperaba '(' después de draw.triangle"));
        }
        self.pos += 1;

        let v1_x = self.parse_expression()?;
        self.skip_comma();
        let v1_y = self.parse_expression()?;
        self.skip_comma();
        let v2_x = self.parse_expression()?;
        self.skip_comma();
        let v2_y = self.parse_expression()?;
        self.skip_comma();
        let v3_x = self.parse_expression()?;
        self.skip_comma();
        let v3_y = self.parse_expression()?;
        self.skip_comma();

        let color = if self.pos < self.tokens.len() {
            if let Token::Texto(c) = &self.tokens[self.pos] {
                self.pos += 1;
                c.clone()
            } else {
                "negro".to_string()
            }
        } else {
            "negro".to_string()
        };

        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentDer) {
            self.pos += 1;
        }

        Ok(Some(Stmt::DrawTriangle {
            v1_x,
            v1_y,
            v2_x,
            v2_y,
            v3_x,
            v3_y,
            color,
        }))
    }

    fn parse_draw_ring(&mut self) -> Result<Option<Stmt>> {
        // draw.ring(x, y, inner_radius, outer_radius, "color")
        self.pos += 1;
        if self.pos >= self.tokens.len() || !matches!(self.tokens[self.pos], Token::ParentIzq) {
            return Err(self.error_syntax("Se esperaba '(' después de draw.ring"));
        }
        self.pos += 1;

        let center_x = self.parse_expression()?;
        self.skip_comma();
        let center_y = self.parse_expression()?;
        self.skip_comma();
        let inner_radius = self.parse_expression()?;
        self.skip_comma();
        let outer_radius = self.parse_expression()?;
        self.skip_comma();

        let color = if self.pos < self.tokens.len() {
            if let Token::Texto(c) = &self.tokens[self.pos] {
                self.pos += 1;
                c.clone()
            } else {
                "negro".to_string()
            }
        } else {
            "negro".to_string()
        };

        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentDer) {
            self.pos += 1;
        }

        Ok(Some(Stmt::DrawRing {
            center_x,
            center_y,
            inner_radius,
            outer_radius,
            color,
        }))
    }

    fn parse_draw_rectangle_lines(&mut self) -> Result<Option<Stmt>> {
        // draw.rectangle_lines(x, y, ancho, alto, "color")
        self.pos += 1;
        if self.pos >= self.tokens.len() || !matches!(self.tokens[self.pos], Token::ParentIzq) {
            return Err(self.error_syntax("Se esperaba '(' después de draw.rectangle_lines"));
        }
        self.pos += 1;

        let x = self.parse_expression()?;
        self.skip_comma();
        let y = self.parse_expression()?;
        self.skip_comma();
        let ancho = self.parse_expression()?;
        self.skip_comma();
        let alto = self.parse_expression()?;
        self.skip_comma();

        let color = if self.pos < self.tokens.len() {
            if let Token::Texto(c) = &self.tokens[self.pos] {
                self.pos += 1;
                c.clone()
            } else {
                "negro".to_string()
            }
        } else {
            "negro".to_string()
        };

        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentDer) {
            self.pos += 1;
        }

        Ok(Some(Stmt::DrawRectangleLines {
            x,
            y,
            ancho,
            alto,
            color,
        }))
    }

    fn parse_draw_ellipse(&mut self) -> Result<Option<Stmt>> {
        // draw.ellipse(x, y, radius_h, radius_v, "color")
        self.pos += 1;
        if self.pos >= self.tokens.len() || !matches!(self.tokens[self.pos], Token::ParentIzq) {
            return Err(self.error_syntax("Se esperaba '(' después de draw.ellipse"));
        }
        self.pos += 1;

        let center_x = self.parse_expression()?;
        self.skip_comma();
        let center_y = self.parse_expression()?;
        self.skip_comma();
        let radius_h = self.parse_expression()?;
        self.skip_comma();
        let radius_v = self.parse_expression()?;
        self.skip_comma();

        let color = if self.pos < self.tokens.len() {
            if let Token::Texto(c) = &self.tokens[self.pos] {
                self.pos += 1;
                c.clone()
            } else {
                "negro".to_string()
            }
        } else {
            "negro".to_string()
        };

        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentDer) {
            self.pos += 1;
        }

        Ok(Some(Stmt::DrawEllipse {
            center_x,
            center_y,
            radius_h,
            radius_v,
            color,
        }))
    }

    fn parse_draw_line_thick(&mut self) -> Result<Option<Stmt>> {
        // draw.line_thick(x1, y1, x2, y2, thick, "color")
        self.pos += 1;
        if self.pos >= self.tokens.len() || !matches!(self.tokens[self.pos], Token::ParentIzq) {
            return Err(self.error_syntax("Se esperaba '(' después de draw.line_thick"));
        }
        self.pos += 1;

        let x1 = self.parse_expression()?;
        self.skip_comma();
        let y1 = self.parse_expression()?;
        self.skip_comma();
        let x2 = self.parse_expression()?;
        self.skip_comma();
        let y2 = self.parse_expression()?;
        self.skip_comma();
        let thick = self.parse_expression()?;
        self.skip_comma();

        let color = if self.pos < self.tokens.len() {
            if let Token::Texto(c) = &self.tokens[self.pos] {
                self.pos += 1;
                c.clone()
            } else {
                "negro".to_string()
            }
        } else {
            "negro".to_string()
        };

        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentDer) {
            self.pos += 1;
        }

        Ok(Some(Stmt::DrawLineThick {
            x1,
            y1,
            x2,
            y2,
            thick,
            color,
        }))
    }

    fn skip_comma(&mut self) {
        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::Coma) {
            self.pos += 1;
        }
    }

    fn error_syntax(&self, msg: &str) -> RyDitError {
        RyDitError {
            kind: ErrorKind::SyntaxError,
            message: msg.to_string(),
            line: 1,
            column: self.pos,
            source: "".to_string(),
        }
    }

    fn parse_input(&mut self) -> Result<Option<Stmt>> {
        // input() - leer del usuario
        self.pos += 1; // consumir input

        // Consumir ( y ) si existen
        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentIzq) {
            self.pos += 1;
            if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentDer) {
                self.pos += 1;
            }
        }

        // Retornamos un statement especial que el executor reconocerá
        Ok(Some(Stmt::Expr(Expr::Var("__INPUT__".to_string()))))
    }

    fn parse_call_or_ident(&mut self, name: String) -> Result<Option<Stmt>> {
        // Un Ident como statement solo es válido si:
        // 1. Es llamada a función: nombre() o nombre(args)
        // 2. Es llamada con namespace: modulo::funcion()
        // 3. Es indexación: nombre[indice] (expresión como statement)
        // Si es solo "nombre", NO es statement válido - retornar Ok(None)

        // Primero consumir el token Ident
        self.pos += 1;

        // Verificar si hay :: para namespace (modulo::funcion)
        let module_name = name.clone();
        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::DobleDosPuntos) {
            self.pos += 1; // consumir ::

            // Ahora esperamos un identificador (nombre de función)
            if self.pos < self.tokens.len() {
                if let Token::Ident(func_name) = &self.tokens[self.pos] {
                    let func_full_name = format!("{}::{}", module_name, func_name);
                    self.pos += 1;

                    // Verificar si hay () para llamada a función
                    if self.pos < self.tokens.len()
                        && matches!(self.tokens[self.pos], Token::ParentIzq)
                    {
                        return self.parse_call_with_name(func_full_name);
                    }

                    // Función sin llamar - no es statement válido
                    return Ok(None);
                }
            }
        }

        // Verificar si hay () para llamada a función normal
        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentIzq) {
            return self.parse_call_with_name(name);
        }

        // Verificar si es indexación: nombre[indice]
        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::CorcheteIzq) {
            // Es indexación, retornar como expresión
            let index_expr = self.parse_index(Expr::Var(name))?;
            return Ok(Some(Stmt::Expr(index_expr)));
        }

        // Solo "nombre" sin () o [] - NO es statement válido
        // Retornar Ok(None) para que el parser continúe
        Ok(None)
    }

    // Helper para parsear llamada con nombre ya determinado
    fn parse_call_with_name(&mut self, name: String) -> Result<Option<Stmt>> {
        self.pos += 1; // consumir (

        // Parsear argumentos
        let mut args = vec![];
        while self.pos < self.tokens.len() && !matches!(self.tokens[self.pos], Token::ParentDer) {
            let arg = self.parse_expression()?;
            args.push(arg);

            // Saltar coma si existe
            if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::Coma) {
                self.pos += 1;
            }
        }

        // Consumir )
        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentDer) {
            self.pos += 1;
        }

        Ok(Some(Stmt::Call { name, args }))
    }

    fn parse_return(&mut self) -> Result<Option<Stmt>> {
        // return <expr>
        self.pos += 1; // consumir return

        // Si hay una expresión después, la parseamos
        if self.pos < self.tokens.len() && !matches!(self.tokens[self.pos], Token::LlaveDer) {
            let expr = self.parse_expression()?;
            return Ok(Some(Stmt::Return(Some(expr))));
        }

        Ok(Some(Stmt::Return(None)))
    }

    fn parse_import(&mut self) -> Result<Option<Stmt>> {
        // import <modulo> [as <alias>]
        self.pos += 1; // consumir import

        // Obtener nombre del módulo
        if self.pos >= self.tokens.len() {
            return Err(RyDitError {
                kind: ErrorKind::SyntaxError,
                message: "Se esperaba nombre de módulo después de import".to_string(),
                line: 1,
                column: self.pos,
                source: "import".to_string(),
            });
        }

        let module = if let Token::Ident(name) = &self.tokens[self.pos] {
            name.clone()
        } else {
            return Err(RyDitError {
                kind: ErrorKind::SyntaxError,
                message: "Nombre de módulo inválido".to_string(),
                line: 1,
                column: self.pos,
                source: "import".to_string(),
            });
        };
        self.pos += 1;

        // Verificar si hay alias (as <alias>)
        let alias = if self.pos < self.tokens.len() {
            if let Token::As = &self.tokens[self.pos] {
                self.pos += 1; // consumir as
                if self.pos < self.tokens.len() {
                    if let Token::Ident(alias_name) = &self.tokens[self.pos] {
                        self.pos += 1;
                        Some(alias_name.clone())
                    } else {
                        return Err(RyDitError {
                            kind: ErrorKind::SyntaxError,
                            message: "Se esperaba nombre de alias después de as".to_string(),
                            line: 1,
                            column: self.pos,
                            source: "as".to_string(),
                        });
                    }
                } else {
                    return Err(RyDitError {
                        kind: ErrorKind::SyntaxError,
                        message: "Se esperaba nombre de alias después de as".to_string(),
                        line: 1,
                        column: self.pos,
                        source: "as".to_string(),
                    });
                }
            } else {
                None
            }
        } else {
            None
        };

        Ok(Some(Stmt::Import { module, alias }))
    }

    fn parse_assignment(&mut self) -> Result<Option<Stmt>> {
        // dark.slot <name> = <value>
        // dark.slot <name>[index] = <value>
        self.pos += 1; // consumir dark.slot

        // Saltar comentarios antes del nombre de variable
        while self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::Comentario(_))
        {
            self.pos += 1;
        }

        if self.pos >= self.tokens.len() {
            return Err(RyDitError {
                kind: ErrorKind::SyntaxError,
                message: "Se esperaba nombre de variable después de dark.slot".to_string(),
                line: 1,
                column: self.pos,
                source: "dark.slot".to_string(),
            });
        }

        let name = if let Token::Ident(n) = &self.tokens[self.pos] {
            self.pos += 1;
            n.clone()
        } else {
            return Err(RyDitError {
                kind: ErrorKind::SyntaxError,
                message: "Se esperaba nombre de variable".to_string(),
                line: 1,
                column: self.pos,
                source: format!("{:?}", self.tokens[self.pos]),
            });
        };

        // Verificar si hay indexación [index]
        let is_indexed =
            self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::CorcheteIzq);

        if is_indexed {
            // Es una asignación indexada: arr[index] = value
            self.pos += 1; // consumir [
            let index = self.parse_expression()?;

            if self.pos >= self.tokens.len() || !matches!(self.tokens[self.pos], Token::CorcheteDer)
            {
                return Err(RyDitError {
                    kind: ErrorKind::SyntaxError,
                    message: "Se esperaba ']' después del índice".to_string(),
                    line: 1,
                    column: self.pos,
                    source: "[".to_string(),
                });
            }
            self.pos += 1; // consumir ]

            // Esperar =
            if self.pos >= self.tokens.len() || !matches!(self.tokens[self.pos], Token::Igual) {
                return Err(RyDitError {
                    kind: ErrorKind::SyntaxError,
                    message: "Se esperaba '=' después de array[index]".to_string(),
                    line: 1,
                    column: self.pos,
                    source: "]".to_string(),
                });
            }
            self.pos += 1; // consumir =

            // Parsear valor
            let value = self.parse_expression()?;

            Ok(Some(Stmt::IndexAssign {
                array: name,
                index,
                value,
            }))
        } else {
            // Es una asignación simple: name = value
            // Esperar =
            if self.pos >= self.tokens.len() || !matches!(self.tokens[self.pos], Token::Igual) {
                return Err(RyDitError {
                    kind: ErrorKind::SyntaxError,
                    message: "Se esperaba '=' después del nombre de variable".to_string(),
                    line: 1,
                    column: self.pos,
                    source: "dark.slot".to_string(),
                });
            }
            self.pos += 1; // consumir =

            // Parsear valor
            let value = self.parse_expression()?;

            Ok(Some(Stmt::Assign { name, value }))
        }
    }

    fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr> {
        let mut left = self.parse_and()?;

        while self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::Or) {
            self.pos += 1;
            let right = self.parse_and()?;
            left = Expr::BinOp {
                left: Box::new(left),
                op: BinOp::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr> {
        let mut left = self.parse_comparison()?;

        while self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::And) {
            self.pos += 1;
            let right = self.parse_comparison()?;
            left = Expr::BinOp {
                left: Box::new(left),
                op: BinOp::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr> {
        let mut left = self.parse_additive()?;

        loop {
            let op = if self.pos < self.tokens.len() {
                match &self.tokens[self.pos] {
                    Token::Mayor => BinOp::Mayor,
                    Token::Menor => BinOp::Menor,
                    Token::MayorIgual => BinOp::MayorIgual,
                    Token::MenorIgual => BinOp::MenorIgual,
                    Token::Igual => BinOp::Igual,
                    _ => break,
                }
            } else {
                break;
            };

            self.pos += 1;
            let right = self.parse_additive()?;

            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let op = if self.pos < self.tokens.len() {
                match &self.tokens[self.pos] {
                    Token::Mas => BinOp::Suma,
                    Token::Menos => BinOp::Resta,
                    _ => break,
                }
            } else {
                break;
            };

            self.pos += 1;
            let right = self.parse_multiplicative()?;

            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr> {
        let mut left = self.parse_primary()?;

        loop {
            let op = if self.pos < self.tokens.len() {
                match &self.tokens[self.pos] {
                    Token::Por => BinOp::Mult,
                    Token::Div => BinOp::Div,
                    _ => break,
                }
            } else {
                break;
            };

            self.pos += 1;
            let right = self.parse_primary()?;

            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        if self.pos >= self.tokens.len() {
            return Err(RyDitError {
                kind: ErrorKind::SyntaxError,
                message: "Se esperaba expresión".to_string(),
                line: 1,
                column: self.pos,
                source: "".to_string(),
            });
        }

        // not <expr>
        if matches!(self.tokens[self.pos], Token::Not) {
            self.pos += 1;
            let expr = self.parse_primary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(expr),
            });
        }

        // Paréntesis para agrupación: ( expr )
        if matches!(self.tokens[self.pos], Token::ParentIzq) {
            self.pos += 1; // consumir (
            let expr = self.parse_expression()?;
            // Consumir )
            if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentDer) {
                self.pos += 1;
                return Ok(expr);
            } else {
                return Err(RyDitError {
                    kind: ErrorKind::SyntaxError,
                    message: "Se esperaba ')' para cerrar paréntesis".to_string(),
                    line: 1,
                    column: self.pos,
                    source: "(".to_string(),
                });
            }
        }

        // Array literal: [ ... ]
        if matches!(self.tokens[self.pos], Token::CorcheteIzq) {
            return self.parse_array();
        }

        match &self.tokens[self.pos] {
            Token::Num(n) => {
                let val = *n;
                self.pos += 1;
                Ok(Expr::Num(val))
            }
            Token::Texto(s) => {
                let val = s.clone();
                self.pos += 1;
                Ok(Expr::Texto(val))
            }
            Token::Ident(s) => {
                let name = s.clone();
                self.pos += 1;

                // Verificar si es namespace: modulo::funcion(...)
                if self.pos < self.tokens.len()
                    && matches!(self.tokens[self.pos], Token::DobleDosPuntos)
                {
                    self.pos += 1; // consumir ::

                    // Esperar identificador (nombre de función)
                    if self.pos < self.tokens.len() {
                        if let Token::Ident(func_name) = &self.tokens[self.pos] {
                            let func_full_name = format!("{}::{}", name, func_name);
                            self.pos += 1;

                            // Verificar si es llamada a función: modulo::funcion(...)
                            if self.pos < self.tokens.len()
                                && matches!(self.tokens[self.pos], Token::ParentIzq)
                            {
                                return self.parse_call_expr(func_full_name);
                            }

                            // Variable con namespace: modulo::variable
                            return Ok(Expr::Var(func_full_name));
                        }
                    }
                }

                // Verificar si es llamada a función normal: nombre(...)
                if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentIzq)
                {
                    return self.parse_call_expr(name);
                }

                // Verificar si es indexación: lista[0]
                if self.pos < self.tokens.len()
                    && matches!(self.tokens[self.pos], Token::CorcheteIzq)
                {
                    return self.parse_index(Expr::Var(name));
                }

                Ok(Expr::Var(name))
            }
            _ => Err(RyDitError {
                kind: ErrorKind::SyntaxError,
                message: format!("Expresión no válida: {:?}", self.tokens[self.pos]),
                line: 1,
                column: self.pos,
                source: "".to_string(),
            }),
        }
    }

    fn parse_array(&mut self) -> Result<Expr> {
        // [ elem1, elem2, ... ]
        self.pos += 1; // consumir [

        let mut elements = Vec::new();

        // Array vacío: []
        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::CorcheteDer) {
            self.pos += 1;
            return Ok(Expr::Array(elements));
        }

        // Parsear elementos separados por coma
        loop {
            let elem = self.parse_expression()?;
            elements.push(elem);

            // Si hay coma, continuar con el siguiente elemento
            if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::Coma) {
                self.pos += 1;
            } else {
                break;
            }
        }

        // Consumir ]
        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::CorcheteDer) {
            self.pos += 1;
            Ok(Expr::Array(elements))
        } else {
            Err(RyDitError {
                kind: ErrorKind::SyntaxError,
                message: "Se esperaba ']' para cerrar el array".to_string(),
                line: 1,
                column: self.pos,
                source: "[".to_string(),
            })
        }
    }

    fn parse_call_expr(&mut self, name: String) -> Result<Expr> {
        // nombre(arg1, arg2, ...)
        self.pos += 1; // consumir (

        let mut args = Vec::new();

        // Verificar si es llamada sin argumentos: nombre()
        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentDer) {
            self.pos += 1;
            return Ok(Expr::Call { name, args });
        }

        // Parsear argumentos separados por coma
        loop {
            let arg = self.parse_expression()?;
            args.push(arg);

            // Si hay coma, continuar con el siguiente argumento
            if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::Coma) {
                self.pos += 1;
            } else {
                break;
            }
        }

        // Consumir )
        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::ParentDer) {
            self.pos += 1;
            Ok(Expr::Call { name, args })
        } else {
            Err(RyDitError {
                kind: ErrorKind::SyntaxError,
                message: "Se esperaba ')' después de los argumentos".to_string(),
                line: 1,
                column: self.pos,
                source: "(".to_string(),
            })
        }
    }

    fn parse_index(&mut self, array_expr: Expr) -> Result<Expr> {
        // array[indice]
        self.pos += 1; // consumir [

        let index = self.parse_expression()?;

        // Consumir ]
        if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::CorcheteDer) {
            self.pos += 1;
            let result = Expr::Index {
                array: Box::new(array_expr),
                index: Box::new(index),
            };

            // Verificar si hay otra indexación: array[1][2]
            if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::CorcheteIzq) {
                return self.parse_index(result);
            }

            Ok(result)
        } else {
            Err(RyDitError {
                kind: ErrorKind::SyntaxError,
                message: "Se esperaba ']' después del índice".to_string(),
                line: 1,
                column: self.pos,
                source: "[".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_parse_init() {
        let tokens = Lizer::new("shield.init").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
        assert!(matches!(program.statements[0], Stmt::Init));
    }

    #[test]
    fn test_parse_assignment() {
        let tokens = Lizer::new("dark.slot x = 100").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
        if let Stmt::Assign { name, value } = &program.statements[0] {
            assert_eq!(name, "x");
            assert!(matches!(value, Expr::Num(100.0)));
        } else {
            panic!("No es un Assign");
        }
    }

    #[test]
    fn test_parse_multiple() {
        let tokens = Lizer::new("shield.init dark.slot x = 100 onda.core").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 3);
    }

    #[test]
    fn test_parse_not() {
        // not x solo no es un statement válido
        // Verificamos que los tokens se generen correctamente
        let tokens = Lizer::new("not x").scan();
        assert_eq!(tokens.len(), 2);
        assert!(tokens.contains(&Token::Not));
        // El parser no genera statements para expresiones sueltas
    }

    #[test]
    fn test_onif_not() {
        let tokens = Lizer::new("onif not x shield.init").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        // Debería haber 1 statement (el If)
        assert_eq!(program.statements.len(), 1);

        // Verificar que es un If con condición Unary Not
        if let Stmt::If {
            condition,
            then_body,
            ..
        } = &program.statements[0]
        {
            assert!(matches!(
                condition,
                Expr::Unary {
                    op: UnaryOp::Not,
                    ..
                }
            ));
            assert_eq!(then_body.len(), 1); // shield.init en el body
        } else {
            panic!("No es un If");
        }
    }

    #[test]
    fn test_block_tokens() {
        let tokens = Lizer::new("{ }").scan();
        assert!(tokens.contains(&Token::LlaveIzq));
        assert!(tokens.contains(&Token::LlaveDer));
    }

    #[test]
    fn test_block_parse() {
        let tokens = Lizer::new("{ shield.init onda.core }").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
        if let Stmt::Block(stmts) = &program.statements[0] {
            assert_eq!(stmts.len(), 2);
        } else {
            panic!("No es un Block");
        }
    }

    // ==================== TESTS DE ARRAYS ====================

    #[test]
    fn test_array_tokens() {
        let tokens = Lizer::new("[1, 2, 3]").scan();
        assert!(tokens.contains(&Token::CorcheteIzq));
        assert!(tokens.contains(&Token::CorcheteDer));
        assert!(tokens.contains(&Token::Num(1.0)));
        assert!(tokens.contains(&Token::Num(2.0)));
        assert!(tokens.contains(&Token::Num(3.0)));
    }

    #[test]
    fn test_array_vacio() {
        let tokens = Lizer::new("[]").scan();
        assert!(tokens.contains(&Token::CorcheteIzq));
        assert!(tokens.contains(&Token::CorcheteDer));
    }

    #[test]
    fn test_array_basico() {
        let tokens = Lizer::new("dark.slot lista = [1, 2, 3]").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        if let Stmt::Assign { name, value } = &program.statements[0] {
            assert_eq!(name, "lista");
            if let Expr::Array(elements) = value {
                assert_eq!(elements.len(), 3);
            } else {
                panic!("No es un Array");
            }
        } else {
            panic!("No es un Assign");
        }
    }

    #[test]
    fn test_indexacion_basica() {
        let tokens = Lizer::new("lista[0]").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        if let Stmt::Expr(Expr::Index { array, index }) = &program.statements[0] {
            if let Expr::Var(name) = array.as_ref() {
                assert_eq!(name, "lista");
            } else {
                panic!("Array no es Var");
            }
            if let Expr::Num(i) = index.as_ref() {
                assert_eq!(*i, 0.0);
            } else {
                panic!("Índice no es Num");
            }
        } else {
            panic!("No es una indexación");
        }
    }

    #[test]
    fn test_array_en_memoria() {
        let tokens = Lizer::new("dark.slot x = [10, 20, 30]").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        if let Stmt::Assign { value, .. } = &program.statements[0] {
            if let Expr::Array(elements) = value {
                assert_eq!(elements.len(), 3);
                if let Expr::Num(n) = &elements[0] {
                    assert_eq!(*n, 10.0);
                }
            }
        }
    }

    #[test]
    fn test_array_multidimensional() {
        // [[0, 0], [0, 0]]
        let tokens = Lizer::new("dark.slot tablero = [[0, 0], [0, 0]]").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        if let Stmt::Assign { value, .. } = &program.statements[0] {
            if let Expr::Array(outer) = value {
                assert_eq!(outer.len(), 2); // 2 filas
                if let Expr::Array(inner) = &outer[0] {
                    assert_eq!(inner.len(), 2); // 2 columnas
                }
            }
        }
    }

    // ==================== TESTS DE CADA (FOREACH) ====================

    #[test]
    fn test_cada_tokens() {
        let tokens = Lizer::new("cada x en lista").scan();
        assert!(tokens.contains(&Token::Cada));
        assert!(tokens.contains(&Token::En));
        assert!(tokens.contains(&Token::Ident("x".to_string())));
        assert!(tokens.contains(&Token::Ident("lista".to_string())));
    }

    #[test]
    fn test_cada_basico() {
        let tokens = Lizer::new("cada x en lista { voz x }").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        if let Stmt::ForEach {
            var,
            iterable,
            body,
        } = &program.statements[0]
        {
            assert_eq!(var, "x");
            if let Expr::Var(name) = iterable {
                assert_eq!(name, "lista");
            } else {
                panic!("Iterable no es Var");
            }
            assert_eq!(body.len(), 1);
        } else {
            panic!("No es un ForEach");
        }
    }

    #[test]
    fn test_cada_con_array_literal() {
        let tokens = Lizer::new("cada x en [1, 2, 3] { voz x }").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        if let Stmt::ForEach { iterable, .. } = &program.statements[0] {
            if let Expr::Array(elements) = iterable {
                assert_eq!(elements.len(), 3);
            } else {
                panic!("Iterable no es Array");
            }
        }
    }

    #[test]
    fn test_cada_multiple_statements() {
        let tokens = Lizer::new("cada i en nums { voz i dark.slot x = i }").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        if let Stmt::ForEach { body, .. } = &program.statements[0] {
            assert_eq!(body.len(), 2); // voz + dark.slot
        }
    }

    // ========================================================================
    // TESTS DE REGRESIÓN - v0.0.10 Parser Loop Bug Fix
    // ========================================================================

    #[test]
    fn test_regresion_ident_sin_parentesis_no_causa_loop() {
        // Bug: Un Ident sin () causaba loop infinito en el parser
        // Fix: parse_call_or_ident retorna Ok(None) si no hay () o []
        let tokens = Lizer::new("lista").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        // "lista" solo no es un statement válido, debería ser ignorado
        assert_eq!(program.statements.len(), 0);
    }

    #[test]
    fn test_regresion_voz_string_en_repl() {
        // Test del bug reportado en advertencia.txt
        // voz "hola" debería parsear correctamente sin loop
        let tokens = Lizer::new("voz \"hola\"").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        if let Stmt::Expr(Expr::Texto(text)) = &program.statements[0] {
            assert_eq!(text, "hola");
        } else {
            panic!("No es un statement de texto");
        }
    }

    #[test]
    fn test_regresion_funcion_con_cuerpo() {
        // rytmo test(x) { voz x } debería parsear sin loop
        let tokens = Lizer::new("rytmo test(x) { voz x }").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        if let Stmt::Function { name, params, body } = &program.statements[0] {
            assert_eq!(name, "test");
            assert_eq!(params.len(), 1);
            assert_eq!(params[0], "x");
            assert_eq!(body.len(), 1);
        } else {
            panic!("No es una función");
        }
    }

    #[test]
    fn test_regresion_input_con_parentesis() {
        // input() debería parsear correctamente
        let tokens = Lizer::new("input()").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        if let Stmt::Expr(Expr::Var(name)) = &program.statements[0] {
            assert_eq!(name, "__INPUT__");
        } else {
            panic!("No es un statement de input");
        }
    }

    #[test]
    fn test_regresion_llamada_funcion_con_args() {
        // foo(1, 2, 3) debería parsear como llamada
        let tokens = Lizer::new("foo(1, 2, 3)").scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        if let Stmt::Call { name, args } = &program.statements[0] {
            assert_eq!(name, "foo");
            assert_eq!(args.len(), 3);
        } else {
            panic!("No es una llamada a función");
        }
    }

    #[test]
    fn test_regresion_script_completo_repl() {
        // Script completo como el de advertencia.txt
        let script = r#"
voz "hola"
rytmo test(x) { voz x }
"#;
        let tokens = Lizer::new(script).scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        // Debería tener 2 statements: voz + función
        assert_eq!(program.statements.len(), 2);
    }

    #[test]
    fn test_funcion_multilinea() {
        // Función con múltiples líneas como en los módulos
        let script = r#"rytmo sumar(a, b) {
    return a + b
}

rytmo restar(a, b) {
    return a - b
}"#;
        let tokens = Lizer::new(script).scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse();

        // Debería parsear correctamente
        assert!(program.is_ok(), "Error: {:?}", program.err());
        let program = program.unwrap();

        // Debería tener 2 statements: 2 funciones
        assert_eq!(program.statements.len(), 2);

        // Verificar que son funciones
        if let Stmt::Function { name, params, .. } = &program.statements[0] {
            assert_eq!(name, "sumar");
            assert_eq!(params.len(), 2);
        } else {
            panic!("No es una función");
        }
    }

    #[test]
    fn test_regresion_comentarios_no_saltan_statements() {
        // Bug v0.1.3: Los comentarios hacían que se salte el siguiente statement
        // El parser avanzaba 2 veces: una en Comentario y otra en el bucle parse()
        let script = r#"shield.init
# comentario
dark.slot x = 10
voz x"#;
        let tokens = Lizer::new(script).scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        // Debería tener 3 statements: shield.init, assign, voz
        assert_eq!(program.statements.len(), 3);

        // Verificar que el segundo statement es la asignación
        if let Stmt::Assign { name, value } = &program.statements[1] {
            assert_eq!(name, "x");
            assert!(matches!(value, Expr::Num(10.0)));
        } else {
            panic!("El segundo statement debería ser Assign");
        }
    }

    #[test]
    fn test_regresion_multiples_comentarios() {
        // Múltiples comentarios no deberían romper el parsing
        let script = r#"# comentario 1
shield.init
dark.slot x = 5
voz x"#;
        let tokens = Lizer::new(script).scan();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 3);
    }

    // ========================================================================
    // TESTS V0.5.1 - PRECEDENCIA AVANZADA
    // ========================================================================

    #[test]
    fn test_precedencia_and_or() {
        // true AND false OR true = (true AND false) OR true = false OR true = true
        let mut parser = Parser::new(Lizer::new("dark.slot x = true and false or true").scan());
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_precedencia_comparacion_and() {
        // x > 5 AND x < 10
        let mut parser = Parser::new(Lizer::new("dark.slot x = 7 > 5 and 7 < 10").scan());
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_precedencia_not() {
        // not true AND false = (not true) AND false = false AND false = false
        let mut parser = Parser::new(Lizer::new("dark.slot x = not true and false").scan());
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_precedencia_expresion_compleja() {
        // (5 + 3) * 2 > 10 AND not false = 16 > 10 AND true = true AND true = true
        let mut parser =
            Parser::new(Lizer::new("dark.slot x = (5 + 3) * 2 > 10 and not false").scan());
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_parentesis_anidados_profundos() {
        // (((1 + 2) + 3) + 4) = 10
        let mut parser = Parser::new(Lizer::new("dark.slot x = (((1 + 2) + 3) + 4)").scan());
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    // ========================================================================
    // TESTS V0.5.1 - ERRORES
    // ========================================================================

    #[test]
    fn test_error_string_sin_cerrar() {
        let tokens = Lizer::new("\"hola mundo").scan();
        // Debería generar error de string sin cerrar
        assert!(tokens.iter().any(|t| matches!(t, Token::Error(_))));
    }

    #[test]
    fn test_error_parentesis_sin_cerrar() {
        let mut parser = Parser::new(Lizer::new("dark.slot x = (2 + 3").scan());
        let result = parser.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_error_llave_sin_cerrar() {
        let mut parser = Parser::new(Lizer::new("si true { voz \"hola\"").scan());
        let result = parser.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_error_caracter_invalido() {
        // € £ ¥ son caracteres realmente inválidos
        let tokens = Lizer::new("€£¥").scan();
        // Debería generar error de carácter inesperado
        assert!(tokens.iter().any(|t| matches!(t, Token::Error(_))));
    }
}
