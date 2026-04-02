// crates/rydit-lexer/src/lexer.rs
// Lexer<'a> zero-copy con lifetimes

use crate::token::{Span, Token, TokenKind};

/// Lexer zero-copy para RyDit
///
/// Convierte source code en tokens con lifetimes.
/// Los tokens referencian el source original, no copian.
///
/// # Ejemplos
///
/// ```
/// use rydit_lexer::Lexer;
///
/// let source = "shield.init dark.slot x = 100";
/// let lexer = Lexer::new(source);
/// let tokens = lexer.scan();
///
/// assert!(tokens.iter().any(|t| t.kind == TokenKind::ShieldInit));
/// ```
pub struct Lexer<'a> {
    source: &'a str,
}

impl<'a> Lexer<'a> {
    /// Crea un nuevo Lexer para el source dado
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    /// Escanea todo el source y retorna tokens
    ///
    /// Los tokens referencian el source original (zero-copy).
    pub fn scan(&self) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = self.source.chars().collect();
        let mut i = 0;
        let mut line = 1;
        let mut column = 1;

        while i < chars.len() {
            let start_line = line;
            let start_col = column;
            let start_pos = i;

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
                let start = i + 1;
                i += 1;
                column += 1;
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                    column += 1;
                }
                // Zero-copy: slice del source original
                let comment = &self.source[start..i];
                tokens.push(Token::new(
                    TokenKind::Comentario,
                    comment,
                    Span::new(start_pos, i, start_line, start_col),
                ));
                continue;
            }

            // String: "..." o '...'
            if chars[i] == '"' || chars[i] == '\'' {
                let quote_char = chars[i];
                let start = i;
                i += 1;
                column += 1;

                while i < chars.len() && chars[i] != quote_char {
                    // Soporte para escapes
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        i += 2;
                        column += 2;
                    } else {
                        i += 1;
                        column += 1;
                    }
                }

                if i < chars.len() {
                    i += 1; // cerrar quote
                    column += 1;
                }

                // Zero-copy: slice del source original (incluye comillas)
                let text = &self.source[start..i];
                tokens.push(Token::new(
                    TokenKind::Texto,
                    text,
                    Span::new(start_pos, i, start_line, start_col),
                ));
                continue;
            }

            // Número: 100, 0.5, -5
            if chars[i].is_numeric()
                || (chars[i] == '-' && i + 1 < chars.len() && chars[i + 1].is_numeric())
            {
                let start = i;
                if chars[i] == '-' {
                    i += 1;
                    column += 1;
                }
                while i < chars.len() && (chars[i].is_numeric() || chars[i] == '.') {
                    i += 1;
                    column += 1;
                }

                // Zero-copy: slice del source original
                let num_str = &self.source[start..i];
                tokens.push(Token::new(
                    TokenKind::Num,
                    num_str,
                    Span::new(start_pos, i, start_line, start_col),
                ));
                continue;
            }

            // Identificador o comando: shield.init, delta.flow, x
            if chars[i].is_alphabetic()
                || chars[i].is_alphanumeric()
                || chars[i] == '_'
                || chars[i] == '@'
                || chars[i] == '$'
                || chars[i] == '%'
            {
                let start = i;
                while i < chars.len()
                    && (chars[i].is_alphanumeric()
                        || chars[i] == '.'
                        || chars[i] == '_'
                        || chars[i] == '@'
                        || chars[i] == '$'
                        || chars[i] == '%')
                {
                    i += 1;
                    column += 1;
                }

                // Zero-copy: slice del source original
                let ident = &self.source[start..i];

                // Verificar si es comando especial
                let kind = match ident {
                    "shield.init" => TokenKind::ShieldInit,
                    "onda.core" => TokenKind::OndaCore,
                    "onif" => TokenKind::Onif,
                    "blelse" => TokenKind::Blelse,
                    "ryprime" => TokenKind::RyPrime,
                    "dark.slot" => TokenKind::DarkSlot,
                    "ryda" => TokenKind::Ryda,
                    "cada" => TokenKind::Cada,
                    "en" => TokenKind::En,
                    "rytmo" => TokenKind::Rytmo,
                    "return" => TokenKind::Return,
                    "voz" => TokenKind::Voz,
                    // Draw commands
                    "draw.circle" | "dibujar.circulo" => TokenKind::DrawCircle,
                    "draw.rect" | "dibujar.rect" | "dibujar.rectangulo" => TokenKind::DrawRect,
                    "draw.line" | "dibujar.linea" => TokenKind::DrawLine,
                    "draw.text" | "dibujar.texto" => TokenKind::DrawText,
                    "draw.triangle" | "dibujar.triangulo" => TokenKind::DrawTriangle,
                    "draw.ring" | "dibujar.anillo" => TokenKind::DrawRing,
                    "draw.rectangle_lines" | "dibujar.lineas_rectangulo" => {
                        TokenKind::DrawRectangleLines
                    }
                    "draw.ellipse" | "dibujar.elipse" => TokenKind::DrawEllipse,
                    "draw.line_thick" | "dibujar.linea_gruesa" => TokenKind::DrawLineThick,
                    // Keywords
                    "and" => TokenKind::And,
                    "or" => TokenKind::Or,
                    "not" => TokenKind::Not,
                    "break" => TokenKind::Break,
                    "import" => TokenKind::Import,
                    "as" => TokenKind::As,
                    // Input especial
                    "input" => TokenKind::Ident,
                    _ => TokenKind::Ident,
                };

                tokens.push(Token::new(
                    kind,
                    ident,
                    Span::new(start_pos, i, start_line, start_col),
                ));
                continue;
            }

            // Operadores y símbolos
            match chars[i] {
                '=' => {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        tokens.push(Token::new(
                            TokenKind::Igual,
                            "==",
                            Span::new(start_pos, i + 2, start_line, start_col),
                        ));
                        i += 2;
                        column += 2;
                    } else {
                        tokens.push(Token::new(
                            TokenKind::Igual,
                            "=",
                            Span::new(start_pos, i + 1, start_line, start_col),
                        ));
                        i += 1;
                        column += 1;
                    }
                }
                '+' => {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        tokens.push(Token::new(
                            TokenKind::MasIgual,
                            "+=",
                            Span::new(start_pos, i + 2, start_line, start_col),
                        ));
                        i += 2;
                        column += 2;
                    } else {
                        tokens.push(Token::new(
                            TokenKind::Mas,
                            "+",
                            Span::new(start_pos, i + 1, start_line, start_col),
                        ));
                        i += 1;
                        column += 1;
                    }
                }
                '-' => {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        tokens.push(Token::new(
                            TokenKind::MenosIgual,
                            "-=",
                            Span::new(start_pos, i + 2, start_line, start_col),
                        ));
                        i += 2;
                        column += 2;
                    } else {
                        tokens.push(Token::new(
                            TokenKind::Menos,
                            "-",
                            Span::new(start_pos, i + 1, start_line, start_col),
                        ));
                        i += 1;
                        column += 1;
                    }
                }
                '*' => {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        tokens.push(Token::new(
                            TokenKind::PorIgual,
                            "*=",
                            Span::new(start_pos, i + 2, start_line, start_col),
                        ));
                        i += 2;
                        column += 2;
                    } else {
                        tokens.push(Token::new(
                            TokenKind::Por,
                            "*",
                            Span::new(start_pos, i + 1, start_line, start_col),
                        ));
                        i += 1;
                        column += 1;
                    }
                }
                '/' => {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        tokens.push(Token::new(
                            TokenKind::DivIgual,
                            "/=",
                            Span::new(start_pos, i + 2, start_line, start_col),
                        ));
                        i += 2;
                        column += 2;
                    } else {
                        tokens.push(Token::new(
                            TokenKind::Div,
                            "/",
                            Span::new(start_pos, i + 1, start_line, start_col),
                        ));
                        i += 1;
                        column += 1;
                    }
                }
                '>' => {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        tokens.push(Token::new(
                            TokenKind::MayorIgual,
                            ">=",
                            Span::new(start_pos, i + 2, start_line, start_col),
                        ));
                        i += 2;
                        column += 2;
                    } else {
                        tokens.push(Token::new(
                            TokenKind::Mayor,
                            ">",
                            Span::new(start_pos, i + 1, start_line, start_col),
                        ));
                        i += 1;
                        column += 1;
                    }
                }
                '<' => {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        tokens.push(Token::new(
                            TokenKind::MenorIgual,
                            "<=",
                            Span::new(start_pos, i + 2, start_line, start_col),
                        ));
                        i += 2;
                        column += 2;
                    } else {
                        tokens.push(Token::new(
                            TokenKind::Menor,
                            "<",
                            Span::new(start_pos, i + 1, start_line, start_col),
                        ));
                        i += 1;
                        column += 1;
                    }
                }
                '!' => {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        tokens.push(Token::new(
                            TokenKind::Diferente,
                            "!=",
                            Span::new(start_pos, i + 2, start_line, start_col),
                        ));
                        i += 2;
                        column += 2;
                    } else {
                        tokens.push(Token::new(
                            TokenKind::Not,
                            "!",
                            Span::new(start_pos, i + 1, start_line, start_col),
                        ));
                        i += 1;
                        column += 1;
                    }
                }
                '{' => {
                    tokens.push(Token::new(
                        TokenKind::LlaveIzq,
                        "{",
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
                '}' => {
                    tokens.push(Token::new(
                        TokenKind::LlaveDer,
                        "}",
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
                '(' => {
                    tokens.push(Token::new(
                        TokenKind::ParentIzq,
                        "(",
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
                ')' => {
                    tokens.push(Token::new(
                        TokenKind::ParentDer,
                        ")",
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
                '[' => {
                    tokens.push(Token::new(
                        TokenKind::CorcheteIzq,
                        "[",
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
                ']' => {
                    tokens.push(Token::new(
                        TokenKind::CorcheteDer,
                        "]",
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
                '.' => {
                    tokens.push(Token::new(
                        TokenKind::Punto,
                        ".",
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
                ':' => {
                    if i + 1 < chars.len() && chars[i + 1] == ':' {
                        tokens.push(Token::new(
                            TokenKind::DobleDosPuntos,
                            "::",
                            Span::new(start_pos, i + 2, start_line, start_col),
                        ));
                        i += 2;
                        column += 2;
                    } else {
                        tokens.push(Token::new(
                            TokenKind::Error,
                            ":",
                            Span::new(start_pos, i + 1, start_line, start_col),
                        ));
                        i += 1;
                        column += 1;
                    }
                }
                ',' => {
                    tokens.push(Token::new(
                        TokenKind::Coma,
                        ",",
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
                _c => {
                    tokens.push(Token::new(
                        TokenKind::Error,
                        &self.source[i..i + 1],
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shield_init() {
        let tokens = Lexer::new("shield.init").scan();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::ShieldInit);
    }

    #[test]
    fn test_dark_slot_numero() {
        let tokens = Lexer::new("dark.slot x = 100").scan();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::DarkSlot));
        assert!(tokens
            .iter()
            .any(|t| t.kind == TokenKind::Ident && t.lexeme == "x"));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Igual));
        assert!(tokens
            .iter()
            .any(|t| t.kind == TokenKind::Num && t.lexeme == "100"));
    }

    #[test]
    fn test_comentario() {
        let tokens = Lexer::new("shield.init # comentario").scan();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Comentario));
    }

    #[test]
    fn test_string() {
        let tokens = Lexer::new("\"hola mundo\"").scan();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Texto));
    }

    #[test]
    fn test_operadores_compuestos() {
        let tokens = Lexer::new("+= -= *= /=").scan();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::MasIgual));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::MenosIgual));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::PorIgual));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::DivIgual));
    }

    #[test]
    fn test_zero_copy() {
        // Verificar zero-copy: los tokens referencian el source original
        let source = String::from("x = 100");
        let tokens = Lexer::new(&source).scan();

        // El token Ident referencia una parte del source
        let ident_token = tokens.iter().find(|t| t.kind == TokenKind::Ident).unwrap();
        assert_eq!(ident_token.lexeme, "x");

        // Verificar que es una referencia al source (mismo pointer)
        assert!(std::ptr::eq(ident_token.lexeme.as_ptr(), source.as_ptr()));
    }

    #[test]
    fn test_numeros_negativos() {
        let tokens = Lexer::new("-5 -10.5").scan();
        assert!(tokens
            .iter()
            .any(|t| t.kind == TokenKind::Num && t.lexeme == "-5"));
        assert!(tokens
            .iter()
            .any(|t| t.kind == TokenKind::Num && t.lexeme == "-10.5"));
    }

    #[test]
    fn test_operadores() {
        let tokens = Lexer::new("+ - * /").scan();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Mas));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Menos));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Por));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Div));
    }

    #[test]
    fn test_comparacion() {
        let tokens = Lexer::new("== != >= <=").scan();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Igual));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Diferente));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::MayorIgual));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::MenorIgual));
    }

    #[test]
    fn test_logicos() {
        let tokens = Lexer::new("and or not").scan();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::And));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Or));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Not));
    }

    #[test]
    fn test_simbolos_identificadores() {
        let tokens = Lexer::new("@variable $valor %porcentaje").scan();
        assert!(tokens
            .iter()
            .any(|t| t.kind == TokenKind::Ident && t.lexeme == "@variable"));
        assert!(tokens
            .iter()
            .any(|t| t.kind == TokenKind::Ident && t.lexeme == "$valor"));
        assert!(tokens
            .iter()
            .any(|t| t.kind == TokenKind::Ident && t.lexeme == "%porcentaje"));
    }
}
