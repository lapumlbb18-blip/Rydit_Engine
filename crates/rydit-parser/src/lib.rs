// crates/rydit-parser/src/lib.rs
// RyDit Parser - Parser con Error Recovery
//
// Parser para RyDit que convierte tokens en AST,
// recuperando de errores para reportar múltiples.

pub mod ast;
pub mod error;
pub mod parser;

pub use ast::*;
pub use error::*;
pub use parser::Parser;

/// Versión del crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Parsear source code y retornar (AST, errores)
///
/// Función convenience para `Parser::from_source(source).parse()`.
///
/// # Ejemplos
///
/// ```
/// let (program, errors) = rydit_parser::parse("shield.init dark.slot x = 100");
/// assert!(errors.is_empty());
/// assert_eq!(program.len(), 2);
/// ```
pub fn parse(source: &str) -> (Program<'_>, Vec<RyDitError>) {
    let mut parser = Parser::from_source(source);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_function() {
        let (program, errors) = parse("shield.init");
        assert!(errors.is_empty());
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
