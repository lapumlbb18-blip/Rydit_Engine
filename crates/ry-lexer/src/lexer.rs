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
/// use ry_lexer::Lexer;
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
        let bytes = self.source.as_bytes();
        let mut i = 0;
        let mut line = 1;
        let mut column = 1;

        while i < bytes.len() {
            let start_line = line;
            let start_col = column;
            let start_pos = i;

            // Saltar whitespace
            if bytes[i].is_ascii_whitespace() {
                if bytes[i] == b'\n' {
                    line += 1;
                    column = 1;
                } else {
                    column += 1;
                }
                i += 1;
                continue;
            }

            // Comentario: # ...
            if bytes[i] == b'#' {
                let start = i + 1;
                i += 1;
                column += 1;
                while i < bytes.len() && bytes[i] != b'\n' {
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
            if bytes[i] == b'"' || bytes[i] == b'\'' {
                let quote_char = bytes[i];
                let start = i;
                i += 1;
                column += 1;

                while i < bytes.len() && bytes[i] != quote_char {
                    // Soporte para escapes
                    if bytes[i] == b'\\' && i + 1 < bytes.len() {
                        i += 2;
                        column += 2;
                    } else {
                        i += 1;
                        column += 1;
                    }
                }

                if i < bytes.len() {
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
            if bytes[i].is_ascii_digit()
                || (bytes[i] == b'-' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit())
            {
                let start = i;
                if bytes[i] == b'-' {
                    i += 1;
                    column += 1;
                }
                while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
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
            if bytes[i].is_ascii_alphabetic()
                || bytes[i].is_ascii_alphanumeric()
                || bytes[i] == b'_'
                || bytes[i] == b'@'
                || bytes[i] == b'$'
                || bytes[i] == b'%'
            {
                let start = i;
                while i < bytes.len()
                    && (bytes[i].is_ascii_alphanumeric()
                        || bytes[i] == b'.'
                        || bytes[i] == b'_'
                        || bytes[i] == b'@'
                        || bytes[i] == b'$'
                        || bytes[i] == b'%')
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
            match bytes[i] {
                b'=' => {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
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
                b'+' => {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
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
                b'-' => {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
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
                b'*' => {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
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
                b'/' => {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
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
                b'>' => {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
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
                b'<' => {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
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
                b'!' => {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
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
                b'{' => {
                    tokens.push(Token::new(
                        TokenKind::LlaveIzq,
                        "{",
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
                b'}' => {
                    tokens.push(Token::new(
                        TokenKind::LlaveDer,
                        "}",
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
                b'(' => {
                    tokens.push(Token::new(
                        TokenKind::ParentIzq,
                        "(",
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
                b')' => {
                    tokens.push(Token::new(
                        TokenKind::ParentDer,
                        ")",
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
                b'[' => {
                    tokens.push(Token::new(
                        TokenKind::CorcheteIzq,
                        "[",
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
                b']' => {
                    tokens.push(Token::new(
                        TokenKind::CorcheteDer,
                        "]",
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
                b'.' => {
                    tokens.push(Token::new(
                        TokenKind::Punto,
                        ".",
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
                b':' => {
                    if i + 1 < bytes.len() && bytes[i + 1] == b':' {
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
                b',' => {
                    tokens.push(Token::new(
                        TokenKind::Coma,
                        ",",
                        Span::new(start_pos, i + 1, start_line, start_col),
                    ));
                    i += 1;
                    column += 1;
                }
                _ => {
                    // Caracter desconocido - intentar avanzar 1 byte
                    // Para caracteres UTF-8 multi-byte, avanzar al siguiente byte ASCII
                    let byte_len = 1;
                    tokens.push(Token::new(
                        TokenKind::Error,
                        &self.source[i..i + byte_len.min(self.source.len() - i)],
                        Span::new(start_pos, i + byte_len, start_line, start_col),
                    ));
                    i += byte_len;
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

    // ============================================================
    // TESTS DE COMENTARIOS (Fase 1: Reproducción del bug)
    // ============================================================

    #[test]
    fn test_comentario_al_inicio_ascii() {
        // Bug original: comentario al inicio corrupta tokens siguientes
        let tokens = Lexer::new("# Comentario al inicio\ndark.slot x = 10").scan();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Comentario));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::DarkSlot && t.lexeme == "dark.slot"));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Ident && t.lexeme == "x"));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Num && t.lexeme == "10"));
    }

    #[test]
    fn test_multiples_comentarios() {
        let tokens =
            Lexer::new("# Primer comentario\n# Segundo comentario\ndark.slot x = 10").scan();
        let comments: Vec<_> = tokens.iter().filter(|t| t.kind == TokenKind::Comentario).collect();
        assert_eq!(comments.len(), 2);
        assert!(tokens.iter().any(|t| t.kind == TokenKind::DarkSlot && t.lexeme == "dark.slot"));
    }

    #[test]
    fn test_comentario_vacio() {
        let tokens = Lexer::new("#\ndark.slot x = 10").scan();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Comentario && t.lexeme == ""));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::DarkSlot && t.lexeme == "dark.slot"));
    }

    #[test]
    fn test_comentario_final_sin_newline() {
        let tokens = Lexer::new("dark.slot x = 10 # comentario final").scan();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::DarkSlot && t.lexeme == "dark.slot"));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Comentario));
    }

    #[test]
    fn test_solo_comentarios() {
        let tokens = Lexer::new("# Solo comentario\n# Otro comentario").scan();
        assert_eq!(tokens.len(), 2);
        assert!(tokens.iter().all(|t| t.kind == TokenKind::Comentario));
    }

    #[test]
    fn test_span_despues_de_comentario() {
        let tokens = Lexer::new("# comentario\ndark.slot x = 10").scan();
        let dark_slot = tokens.iter().find(|t| t.kind == TokenKind::DarkSlot).unwrap();
        assert_eq!(dark_slot.span.line, 2);
        assert_eq!(dark_slot.span.column, 1);
    }

    // ============================================================
    // TESTS ALTERNATIVOS DE VERIFICACIÓN (por si los 6 principales fallan)
    // ============================================================

    #[test]
    fn test_alternativo_comentario_simple() {
        // Test ultra-simple: solo verificar que comentario no rompe
        let tokens = Lexer::new("# hola\nx").scan();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Comentario);
        assert_eq!(tokens[1].kind, TokenKind::Ident);
        assert_eq!(tokens[1].lexeme, "x");
    }

    #[test]
    fn test_alternativo_comentario_y_numero() {
        // Verificar que numero despues de comentario funciona
        let tokens = Lexer::new("# test\n42").scan();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[1].kind, TokenKind::Num);
        assert_eq!(tokens[1].lexeme, "42");
    }

    #[test]
    fn test_alternativo_comentario_y_string() {
        // Verificar que string despues de comentario funciona
        let tokens = Lexer::new("# test\n\"hola\"").scan();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[1].kind, TokenKind::Texto);
        assert_eq!(tokens[1].lexeme, "\"hola\"");
    }

    #[test]
    fn test_alternativo_comentario_y_operador() {
        // Verificar que operador despues de comentario funciona
        let tokens = Lexer::new("# test\n+").scan();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[1].kind, TokenKind::Mas);
    }

    #[test]
    fn test_alternativo_comentario_y_llaves() {
        // Verificar que llaves despues de comentario funcionan
        let tokens = Lexer::new("# test\n{ }").scan();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[1].kind, TokenKind::LlaveIzq);
        assert_eq!(tokens[2].kind, TokenKind::LlaveDer);
    }

    #[test]
    fn test_alternativo_comentario_con_emoji() {
        // Edge case: emoji en comentario (UTF-8 multi-byte)
        let tokens = Lexer::new("# 🚀 rocket\ndark.slot x = 10").scan();
        // El emoji genera tokens Error pero dark.slot debe parsearse bien
        let dark_slot = tokens.iter().find(|t| t.kind == TokenKind::DarkSlot);
        assert!(
            dark_slot.is_some(),
            "dark.slot debe tokenizarse correctamente despues de comentario con emoji"
        );
    }

    #[test]
    fn test_alternativo_comentario_con_unicode() {
        // Comentario con texto unicode (acentos, eñes)
        let tokens = Lexer::new("# acción y configuración\ndark.slot x = 10").scan();
        let dark_slot = tokens.iter().find(|t| t.kind == TokenKind::DarkSlot);
        assert!(
            dark_slot.is_some(),
            "dark.slot debe tokenizarse correctamente despues de comentario con unicode"
        );
    }

    #[test]
    fn test_alternativo_cero_copy_preservado() {
        // Verificar que zero-copy funciona despues de comentarios
        let source = String::from("# comentario\ndark.slot x = 10");
        let tokens = Lexer::new(&source).scan();
        let dark_slot = tokens.iter().find(|t| t.kind == TokenKind::DarkSlot).unwrap();
        // Verificar que el lexeme es una referencia al source (mismo buffer base)
        let source_ptr = source.as_ptr();
        let lexeme_ptr = dark_slot.lexeme.as_ptr();
        // El lexeme debe estar dentro del buffer del source
        let offset = unsafe { lexeme_ptr.offset_from(source_ptr) };
        assert!(
            offset >= 0 && (offset as usize) < source.len(),
            "El lexeme debe ser una referencia dentro del source original"
        );
    }

    #[test]
    fn test_alternativo_script_completo_con_comentarios() {
        // Script realista con comentarios intercalados
        let script = r#"# Juego Snake
dark.slot x = 400
dark.slot y = 300
# Game loop
ryda x < 500 {
    voz "Frame"
    dark.slot x = x + 1
}
# Fin"#;
        let tokens = Lexer::new(script).scan();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::DarkSlot));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Ryda));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Voz));
        // Verificar que los comentarios existen
        let comments: Vec<_> = tokens.iter().filter(|t| t.kind == TokenKind::Comentario).collect();
        assert_eq!(comments.len(), 3);
    }

    #[test]
    fn test_alternativo_comentarios_consecutivos() {
        // Muchos comentarios seguidos no deben romper el lexer
        let script = "# 1\n# 2\n# 3\n# 4\n# 5\nx";
        let tokens = Lexer::new(script).scan();
        let comments: Vec<_> = tokens.iter().filter(|t| t.kind == TokenKind::Comentario).collect();
        assert_eq!(comments.len(), 5);
        assert_eq!(tokens.last().unwrap().lexeme, "x");
    }
}
