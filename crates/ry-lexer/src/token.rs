// crates/rydit-lexer/src/token.rs
// Token<'a> zero-copy con lifetimes

use std::fmt;

/// Span: posición en el source code
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

impl Default for Span {
    fn default() -> Self {
        Self {
            start: 0,
            end: 0,
            line: 1,
            column: 1,
        }
    }
}

/// Tipo de token (kind) - no contiene datos
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
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
    DarkSlot,           // dark.slot
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
    As,                 // as alias
    Break,              // break

    // Literales y valores (zero-copy: &'a str)
    Ident, // nombres: x, jugador, delta.flow
    Num,   // números: 100, 0.5
    Texto, // strings: "hola"

    // Operadores aritméticos
    Mas,        // +
    Menos,      // -
    Por,        // *
    Div,        // /
    MasIgual,   // +=
    MenosIgual, // -=
    PorIgual,   // *=
    DivIgual,   // /=

    // Operadores de comparación
    Mayor,      // >
    Menor,      // <
    Igual,      // ==
    Asignar,    // =
    MayorIgual, // >=
    MenorIgual, // <=
    Diferente,  // !=

    // Operadores lógicos
    And, // and
    Or,  // or
    Not, // not

    // Delimitadores
    LlaveIzq,       // {
    LlaveDer,       // }
    ParentIzq,      // (
    ParentDer,      // )
    CorcheteIzq,    // [
    CorcheteDer,    // ]
    Punto,          // .
    Coma,           // ,
    DobleDosPuntos, // ::

    // Comentarios (zero-copy: &'a str)
    Comentario,

    // Errores
    Error,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenKind::ShieldInit => "shield.init",
            TokenKind::OndaCore => "onda.core",
            TokenKind::Onif => "onif",
            TokenKind::Blelse => "blelse",
            TokenKind::RyPrime => "ryprime",
            TokenKind::Ryda => "ryda",
            TokenKind::Cada => "cada",
            TokenKind::En => "en",
            TokenKind::Rytmo => "rytmo",
            TokenKind::Return => "return",
            TokenKind::Voz => "voz",
            TokenKind::DarkSlot => "dark.slot",
            TokenKind::DrawCircle => "draw.circle",
            TokenKind::DrawRect => "draw.rect",
            TokenKind::DrawLine => "draw.line",
            TokenKind::DrawText => "draw.text",
            TokenKind::DrawTriangle => "draw.triangle",
            TokenKind::DrawRing => "draw.ring",
            TokenKind::DrawRectangleLines => "draw.rectangle_lines",
            TokenKind::DrawEllipse => "draw.ellipse",
            TokenKind::DrawLineThick => "draw.line_thick",
            TokenKind::Import => "import",
            TokenKind::As => "as",
            TokenKind::Break => "break",
            TokenKind::Ident => "ident",
            TokenKind::Num => "num",
            TokenKind::Texto => "texto",
            TokenKind::Mas => "+",
            TokenKind::Menos => "-",
            TokenKind::Por => "*",
            TokenKind::Div => "/",
            TokenKind::MasIgual => "+=",
            TokenKind::MenosIgual => "-=",
            TokenKind::PorIgual => "*=",
            TokenKind::DivIgual => "/=",
            TokenKind::Mayor => ">",
            TokenKind::Menor => "<",
            TokenKind::Asignar => "=",
            TokenKind::Igual => "==",
            TokenKind::MayorIgual => ">=",
            TokenKind::MenorIgual => "<=",
            TokenKind::Diferente => "!=",
            TokenKind::And => "and",
            TokenKind::Or => "or",
            TokenKind::Not => "not",
            TokenKind::LlaveIzq => "{",
            TokenKind::LlaveDer => "}",
            TokenKind::ParentIzq => "(",
            TokenKind::ParentDer => ")",
            TokenKind::CorcheteIzq => "[",
            TokenKind::CorcheteDer => "]",
            TokenKind::Punto => ".",
            TokenKind::Coma => ",",
            TokenKind::DobleDosPuntos => "::",
            TokenKind::Comentario => "comentario",
            TokenKind::Error => "error",
        };
        write!(f, "{}", s)
    }
}

/// Token zero-copy con lifetime
///
/// Contiene referencias al source original, no copias.
/// El lifetime 'a asegura que el token no vive más que el source.
#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str, // Zero-copy: referencia al source
    pub span: Span,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, lexeme: &'a str, span: Span) -> Self {
        Self { kind, lexeme, span }
    }

    /// Obtener valor numérico si es TokenKind::Num
    pub fn as_num(&self) -> Option<f64> {
        if self.kind == TokenKind::Num {
            self.lexeme.parse().ok()
        } else {
            None
        }
    }

    /// Obtener identificador si es TokenKind::Ident
    pub fn as_ident(&self) -> Option<&'a str> {
        if self.kind == TokenKind::Ident {
            Some(self.lexeme)
        } else {
            None
        }
    }

    /// Verificar si es EOF o fin del scan
    pub fn is_eof(&self) -> bool {
        self.kind == TokenKind::Error && self.lexeme.is_empty()
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}('{}', line {} col {})",
            self.kind, self.lexeme, self.span.line, self.span.column
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_kind_display() {
        assert_eq!(format!("{}", TokenKind::ShieldInit), "shield.init");
        assert_eq!(format!("{}", TokenKind::Mas), "+");
        assert_eq!(format!("{}", TokenKind::Ident), "ident");
    }

    #[test]
    fn test_span_new() {
        let span = Span::new(0, 5, 1, 1);
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 5);
        assert_eq!(span.length(), 5);
    }

    #[test]
    fn test_span_default() {
        let span = Span::default();
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 0);
        assert_eq!(span.line, 1);
    }

    #[test]
    fn test_token_new() {
        let span = Span::new(0, 5, 1, 1);
        let token = Token::new(TokenKind::Ident, "x", span);
        assert_eq!(token.kind, TokenKind::Ident);
        assert_eq!(token.lexeme, "x");
        assert_eq!(token.span, span);
    }

    #[test]
    fn test_token_as_num() {
        let span = Span::default();
        let token = Token::new(TokenKind::Num, "100.5", span);
        assert_eq!(token.as_num(), Some(100.5));

        let token2 = Token::new(TokenKind::Ident, "x", span);
        assert_eq!(token2.as_num(), None);
    }

    #[test]
    fn test_token_as_ident() {
        let span = Span::default();
        let token = Token::new(TokenKind::Ident, "jugador", span);
        assert_eq!(token.as_ident(), Some("jugador"));

        let token2 = Token::new(TokenKind::Num, "100", span);
        assert_eq!(token2.as_ident(), None);
    }

    #[test]
    fn test_token_display() {
        let span = Span::new(0, 1, 1, 1);
        let token = Token::new(TokenKind::Ident, "x", span);
        let display = format!("{}", token);
        assert!(display.contains("x"));
        assert!(display.contains("line 1"));
    }

    #[test]
    fn test_token_zero_copy() {
        // Verificar que el token referencia el source original
        let source = String::from("x = 100");
        let span = Span::default();
        let token = Token::new(TokenKind::Ident, &source[0..1], span);

        // El token referencia una parte del source, no copia
        assert_eq!(token.lexeme, "x");
    }
}
