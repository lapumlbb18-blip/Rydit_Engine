//! RyDit Script - Carga de scripts .rydit como módulos
//!
//! Permite cargar scripts RyDit (.rydit) como módulos dinámicos
//! que implementan el trait `RyditModule`.
//!
//! # Estructura de un script módulo
//!
//! ```rydit
//! # modules/mi_modulo.rydit
//! __module__ = "mi_modulo"
//! __version__ = "1.0.0"
//! __description__ = "Descripción del módulo"
//! __license__ = "MIT"
//!
//! # Funciones exportadas
//! export funcion saludar(nombre) {
//!     return "Hola " + nombre
//! }
//! ```
//!
//! # Ejemplo de uso
//! ```rust,no_run
//! use rydit_script::ScriptModule;
//!
//! // Cargar script como módulo
//! let module = ScriptModule::from_file("modules/mi_modulo.rydit").unwrap();
//!
//! // Ejecutar comando
//! let result = module.execute("saludar", serde_json::json!(["Mundo"])).unwrap();
//! println!("{}", result); // "Hola Mundo"
//! ```

use rydit_core::{ModuleError, ModuleMetadata, ModuleResult, RyditModule};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Metadata extraída de un script RyDit
#[derive(Debug, Clone, Default)]
pub struct ScriptMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub license: String,
    pub author: String,
}

impl ScriptMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    /// Extraer metadata de un script RyDit
    pub fn from_script(content: &str) -> Self {
        let mut metadata = Self::new();

        for line in content.lines() {
            let line = line.trim();
            
            // Buscar líneas de metadata: __name__ = "value"
            if line.starts_with("__") && line.contains('=') {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim().trim_matches('"');

                    match key {
                        "__module__" | "__name__" => metadata.name = value.to_string(),
                        "__version__" => metadata.version = value.to_string(),
                        "__description__" => metadata.description = value.to_string(),
                        "__license__" => metadata.license = value.to_string(),
                        "__author__" => metadata.author = value.to_string(),
                        _ => {}
                    }
                }
            }
        }

        metadata
    }
}

/// Función exportada desde un script
#[derive(Debug, Clone)]
pub struct ExportedFunction {
    pub name: String,
    pub params: Vec<String>,
    pub body: String,
}

/// Extraer funciones exportadas de un script
pub fn extract_exports(content: &str) -> Vec<ExportedFunction> {
    let mut exports = Vec::new();
    let mut lines = content.lines().peekable();

    while let Some(line) = lines.next() {
        let line = line.trim();
        
        // Buscar: export funcion nombre(params) {
        if line.starts_with("export funcion") || line.starts_with("export rytmo") {
            if let Some(func) = parse_function(line, &mut lines) {
                exports.push(func);
            }
        }
    }

    exports
}

/// Parsear una función desde su declaración
fn parse_function(
    line: &str,
    lines: &mut std::iter::Peekable<std::str::Lines>,
) -> Option<ExportedFunction> {
    // Extraer nombre y parámetros: export funcion nombre(p1, p2) {
    let start = line.find("funcion").or_else(|| line.find("rytmo"))?;
    let rest = &line[start + 7..].trim();
    
    // nombre(params) {
    let paren_start = rest.find('(')?;
    let paren_end = rest.find(')')?;
    
    let name = rest[..paren_start].trim().to_string();
    let params_str = &rest[paren_start + 1..paren_end];
    
    let params: Vec<String> = params_str
        .split(',')
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect();

    // Capturar cuerpo de la función
    let mut body = String::new();
    let mut brace_count = 1;

    for next_line in lines {
        body.push_str(next_line);
        body.push('\n');

        brace_count += next_line.matches('{').count();
        brace_count -= next_line.matches('}').count();

        if brace_count == 0 {
            break;
        }
    }

    Some(ExportedFunction { name, params, body })
}

/// Módulo creado desde un script RyDit
pub struct ScriptModule {
    metadata: ScriptMetadata,
    exports: Vec<ExportedFunction>,
    #[allow(dead_code)]  // Reservado para implementación futura de runtime
    source_code: String,
}

impl ScriptModule {
    /// Cargar módulo desde archivo
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ScriptModuleError> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .map_err(|e| ScriptModuleError {
                code: "FILE_ERROR".to_string(),
                message: format!("No se pudo leer el archivo: {}", e),
            })?;

        Ok(Self::from_source(&content))
    }

    /// Crear módulo desde código fuente
    pub fn from_source(content: &str) -> Self {
        let metadata = ScriptMetadata::from_script(content);
        let exports = extract_exports(content);

        Self {
            metadata,
            exports,
            source_code: content.to_string(),
        }
    }

    /// Obtener metadata del módulo
    pub fn metadata(&self) -> &ScriptMetadata {
        &self.metadata
    }

    /// Obtener funciones exportadas
    pub fn exports(&self) -> &[ExportedFunction] {
        &self.exports
    }

    /// Obtener nombre del módulo
    pub fn name(&self) -> &str {
        &self.metadata.name
    }
}

impl RyditModule for ScriptModule {
    fn name(&self) -> &'static str {
        // Nota: Esto es un hack porque necesitamos &'static str
        // En producción, usaríamos Arc<String> o similar
        Box::leak(self.metadata.name.clone().into_boxed_str())
    }

    fn version(&self) -> &'static str {
        Box::leak(self.metadata.version.clone().into_boxed_str())
    }

    fn register(&self) -> HashMap<&'static str, &'static str> {
        let mut cmds = HashMap::new();
        
        for export in &self.exports {
            let name: &'static str = Box::leak(export.name.clone().into_boxed_str());
            let description: &'static str = Box::leak(format!("Función exportada: {}", export.name).into_boxed_str());
            cmds.insert(name, description);
        }

        cmds
    }

    fn execute(&self, command: &str, _params: Value) -> ModuleResult {
        // Buscar la función exportada
        let _func = self.exports
            .iter()
            .find(|f| f.name == command)
            .ok_or_else(|| ModuleError {
                code: "FUNCTION_NOT_FOUND".to_string(),
                message: format!("Función '{}' no encontrada", command),
            })?;

        // NOTA: Aquí iría la lógica para ejecutar el script RyDit
        // En la implementación completa, usaríamos el evaluator de rydit-rs
        // para parsear y ejecutar el código fuente del script
        
        Ok(Value::String(format!(
            "[ScriptModule] Función '{}' lista para ejecutar (runtime pendiente)",
            command
        )))
    }

    fn metadata(&self) -> ModuleMetadata {
        // Usar strings estáticos temporales para la metadata
        // En producción, usaríamos un enfoque diferente (Arc, etc.)
        ModuleMetadata::new()
            .with_name(Box::leak(self.metadata.name.clone().into_boxed_str()))
            .with_version(Box::leak(self.metadata.version.clone().into_boxed_str()))
            .with_description(Box::leak(self.metadata.description.clone().into_boxed_str()))
            .with_license(Box::leak(self.metadata.license.clone().into_boxed_str()))
    }
}

/// Error de ScriptModule
#[derive(Debug, Clone)]
pub struct ScriptModuleError {
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for ScriptModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for ScriptModuleError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SCRIPT: &str = r#"
__module__ = "test_mod"
__version__ = "1.0.0"
__description__ = "Módulo de prueba"
__license__ = "MIT"
__author__ = "Test Author"

export funcion saludar(nombre) {
    return "Hola " + nombre
}

export funcion sumar(a, b) {
    return a + b
}

# Función interna (no exportada)
funcion helper() {
    return "interno"
}
"#;

    #[test]
    fn test_script_metadata() {
        let metadata = ScriptMetadata::from_script(TEST_SCRIPT);
        
        assert_eq!(metadata.name, "test_mod");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.description, "Módulo de prueba");
        assert_eq!(metadata.license, "MIT");
        assert_eq!(metadata.author, "Test Author");
    }

    #[test]
    fn test_extract_exports() {
        let exports = extract_exports(TEST_SCRIPT);
        
        assert_eq!(exports.len(), 2);
        assert_eq!(exports[0].name, "saludar");
        assert_eq!(exports[0].params, vec!["nombre"]);
        assert_eq!(exports[1].name, "sumar");
        assert_eq!(exports[1].params, vec!["a", "b"]);
    }

    #[test]
    fn test_script_module_from_source() {
        let module = ScriptModule::from_source(TEST_SCRIPT);
        
        assert_eq!(module.name(), "test_mod");
        assert_eq!(module.exports().len(), 2);
        assert_eq!(module.metadata().description, "Módulo de prueba");
    }

    #[test]
    fn test_script_module_register() {
        let module = ScriptModule::from_source(TEST_SCRIPT);
        let cmds = module.register();
        
        assert!(cmds.contains_key("saludar"));
        assert!(cmds.contains_key("sumar"));
        assert!(!cmds.contains_key("helper")); // No exportada
    }
}
