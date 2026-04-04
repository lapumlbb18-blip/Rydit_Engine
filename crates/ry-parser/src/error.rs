// crates/rydit-parser/src/error.rs
// Error handling con error recovery
//
// Parser que recupera y continúa, no falla en el primer error.

use std::fmt;

/// Error de compilación RyDit
#[derive(Debug, Clone)]
pub struct RyDitError {
    pub kind: ErrorKind,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub source_snippet: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    // Lexer
    UnexpectedChar,
    InvalidNumber,
    UnterminatedString,

    // Parser
    UnexpectedToken,
    MissingToken,
    SyntaxError,

    // Semántico
    UndefinedVariable,
    DuplicateDefinition,
    TypeMismatch,
    DivisionByZero,
    IndexOutOfBounds,

    // Módulos
    CircularImport,
    ModuleNotFound,

    // Assets
    TextureNotFound,
    SoundNotFound,
}

impl RyDitError {
    pub fn new(kind: ErrorKind, message: String, line: usize, column: usize) -> Self {
        Self {
            kind,
            message,
            line,
            column,
            source_snippet: String::new(),
        }
    }

    pub fn with_source(mut self, source: String) -> Self {
        self.source_snippet = source;
        self
    }

    pub fn unexpected_token(expected: &str, found: &str, line: usize, column: usize) -> Self {
        Self {
            kind: ErrorKind::UnexpectedToken,
            message: format!("Se esperaba '{}', se encontró '{}'", expected, found),
            line,
            column,
            source_snippet: String::new(),
        }
    }

    pub fn missing_token(token: &str, line: usize, column: usize) -> Self {
        Self {
            kind: ErrorKind::MissingToken,
            message: format!("Se esperaba '{}'", token),
            line,
            column,
            source_snippet: String::new(),
        }
    }

    pub fn syntax_error(message: String, line: usize, column: usize) -> Self {
        Self {
            kind: ErrorKind::SyntaxError,
            message,
            line,
            column,
            source_snippet: String::new(),
        }
    }
}

impl fmt::Display for RyDitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind_str = match self.kind {
            ErrorKind::UnexpectedChar => "Carácter inesperado",
            ErrorKind::InvalidNumber => "Número inválido",
            ErrorKind::UnterminatedString => "String sin cerrar",
            ErrorKind::UnexpectedToken => "Token inesperado",
            ErrorKind::MissingToken => "Token faltante",
            ErrorKind::SyntaxError => "Error de sintaxis",
            ErrorKind::UndefinedVariable => "Variable no definida",
            ErrorKind::DuplicateDefinition => "Definición duplicada",
            ErrorKind::TypeMismatch => "Tipo incompatible",
            ErrorKind::DivisionByZero => "División por cero",
            ErrorKind::IndexOutOfBounds => "Índice fuera de rango",
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

        // Mostrar código fuente con snippet
        if !self.source_snippet.is_empty() {
            writeln!(
                f,
                "  ║  Código:                                           ║"
            )?;
            writeln!(f, "  ║    {}", self.source_snippet)?;

            // Crear marcador visual
            let marker_len = self.column.saturating_sub(1);
            let marker = "→".to_string() + &" ".repeat(marker_len) + "^";
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
            _ => {}
        }

        writeln!(
            f,
            "  ╚══════════════════════════════════════════════════════╝"
        )?;
        writeln!(f)
    }
}

/// Resultado que puede contener múltiples errores
pub type ParseResult<'a, T> = Result<T, Vec<RyDitError>>;

/// Estado del parser con error recovery
pub struct ParseState<'a> {
    pub errors: Vec<RyDitError>,
    pub recovered: bool,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> ParseState<'a> {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            recovered: false,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn add_error(&mut self, error: RyDitError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Recuperar de un error (synchronize)
    pub fn recover(&mut self) {
        self.recovered = true;
    }

    pub fn is_recovered(&self) -> bool {
        self.recovered
    }

    /// Convertir a Result
    pub fn into_result<T>(self, value: T) -> ParseResult<'a, T> {
        if self.has_errors() {
            Err(self.errors)
        } else {
            Ok(value)
        }
    }
}

impl<'a> Default for ParseState<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_new() {
        let error = RyDitError::new(ErrorKind::SyntaxError, "Error de prueba".to_string(), 1, 10);
        assert_eq!(error.kind, ErrorKind::SyntaxError);
        assert_eq!(error.message, "Error de prueba");
        assert_eq!(error.line, 1);
        assert_eq!(error.column, 10);
    }

    #[test]
    fn test_error_unexpected_token() {
        let error = RyDitError::unexpected_token("}", "identifier", 5, 20);
        assert_eq!(error.kind, ErrorKind::UnexpectedToken);
        assert!(error.message.contains("}"));
        assert!(error.message.contains("identifier"));
    }

    #[test]
    fn test_error_missing_token() {
        let error = RyDitError::missing_token(")", 3, 15);
        assert_eq!(error.kind, ErrorKind::MissingToken);
        assert!(error.message.contains(")"));
    }

    #[test]
    fn test_error_display() {
        let error = RyDitError::syntax_error("Test error".to_string(), 1, 5);
        let display = format!("{}", error);
        assert!(display.contains("ERROR DE COMPILACIÓN"));
        assert!(display.contains("Error de sintaxis"));
        assert!(display.contains("Test error"));
    }

    #[test]
    fn test_parse_state_new() {
        let state: ParseState = ParseState::new();
        assert!(!state.has_errors());
        assert_eq!(state.error_count(), 0);
        assert!(!state.is_recovered());
    }

    #[test]
    fn test_parse_state_add_error() {
        let mut state: ParseState = ParseState::new();
        let error = RyDitError::syntax_error("Test".to_string(), 1, 1);
        state.add_error(error);
        assert!(state.has_errors());
        assert_eq!(state.error_count(), 1);
    }

    #[test]
    fn test_parse_state_recover() {
        let mut state: ParseState = ParseState::new();
        state.recover();
        assert!(state.is_recovered());
    }

    #[test]
    fn test_parse_state_into_result_ok() {
        let state: ParseState = ParseState::new();
        let result: Result<i32, _> = state.into_result(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_parse_state_into_result_err() {
        let mut state: ParseState = ParseState::new();
        state.add_error(RyDitError::syntax_error("Test".to_string(), 1, 1));
        let result: Result<i32, _> = state.into_result(42);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().len(), 1);
    }
}
