// crates/rydit-lexer/src/lib.rs
// RyDit Lexer - Zero-Copy Tokenizer
//
// Lexer zero-copy con lifetimes para RyDit Engine.
// Los tokens referencian el source original, no copian.

pub mod lexer;
pub mod token;

pub use lexer::Lexer;
pub use token::{Span, Token, TokenKind};

/// Versión del crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Escanear source code y retornar tokens (zero-copy)
///
/// Función convenience para `Lexer::new(source).scan()`.
///
/// # Ejemplos
///
/// ```
/// let tokens = rydit_lexer::scan("shield.init dark.slot x = 100");
/// assert!(tokens.iter().any(|t| t.kind == rydit_lexer::TokenKind::ShieldInit));
/// ```
pub fn scan(source: &str) -> Vec<Token<'_>> {
    Lexer::new(source).scan()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_function() {
        let tokens = scan("shield.init");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::ShieldInit);
    }

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
