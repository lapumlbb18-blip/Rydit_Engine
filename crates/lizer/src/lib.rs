// crates/lizer/src/lib.rs
// Lizer - Wrapper para compatibilidad backward
//
// ⚠️ DEPRECATED: Usar rydit-lexer y rydit-parser directamente.
// Este crate existe solo para compatibilidad con código existente.

// Re-exportar rydit-lexer
pub use rydit_lexer::*;

// Re-exportar rydit-parser
pub use rydit_parser::*;

/// Versión del crate
pub const VERSION: &str = "0.11.2";

/// Parse_cached (compatibilidad) - ahora usa rybot/cache
pub fn parse_cached(source: &str) -> Result<Program<'_>, RyDitError> {
    let mut parser = Parser::from_source(source);
    let (program, errors) = parser.parse();

    if !errors.is_empty() {
        Err(errors[0].clone())
    } else {
        Ok(program)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lizer_wrapper() {
        // Verificar que los re-exports funcionan
        let tokens = Lexer::new("shield.init").scan();
        assert_eq!(tokens.len(), 1);
    }

    #[test]
    fn test_parser_wrapper() {
        let mut parser = Parser::from_source("dark.slot x = 100");
        let (program, errors) = parser.parse();
        assert!(errors.is_empty());
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn test_parse_cached_compat() {
        let result = parse_cached("shield.init");
        assert!(result.is_ok());
    }
}
