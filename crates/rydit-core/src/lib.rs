//! RyDit Core - Trait y Registro para módulos
//!
//! Proporciona la interfaz común que todos los módulos deben implementar.
//!
//! # Versión
//! **v0.8.2** - Sistema Universal Ry (metadata + hot reload hooks)

use serde_json::Value;
use std::collections::HashMap;

/// Resultado de operación de módulo
pub type ModuleResult = Result<Value, ModuleError>;

/// Error de módulo
#[derive(Debug, Clone)]
pub struct ModuleError {
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for ModuleError {}

/// Metadata de un módulo (v0.8.2+)
///
/// Información descriptiva para carga dinámica y sistema de plugins
#[derive(Debug, Clone, Default)]
pub struct ModuleMetadata {
    /// Nombre del módulo
    pub name: &'static str,
    /// Versión del módulo
    pub version: &'static str,
    /// Autores del módulo
    pub authors: Vec<&'static str>,
    /// Descripción del módulo
    pub description: &'static str,
    /// Licencia (ej: "MIT", "Apache-2.0")
    pub license: &'static str,
    /// Dependencias de otros módulos
    pub dependencies: Vec<&'static str>,
}

impl ModuleMetadata {
    /// Crea una nueva metadata vacía
    pub fn new() -> Self {
        Self::default()
    }

    /// Establece el nombre del módulo
    pub fn with_name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }

    /// Establece la versión del módulo
    pub fn with_version(mut self, version: &'static str) -> Self {
        self.version = version;
        self
    }

    /// Establece los autores del módulo
    pub fn with_authors(mut self, authors: Vec<&'static str>) -> Self {
        self.authors = authors;
        self
    }

    /// Establece la descripción del módulo
    pub fn with_description(mut self, description: &'static str) -> Self {
        self.description = description;
        self
    }

    /// Establece la licencia del módulo
    pub fn with_license(mut self, license: &'static str) -> Self {
        self.license = license;
        self
    }

    /// Establece las dependencias del módulo
    pub fn with_dependencies(mut self, deps: Vec<&'static str>) -> Self {
        self.dependencies = deps;
        self
    }
}

/// Trait que todos los módulos deben implementar
///
/// # Ejemplo
/// ```rust
/// use rydit_core::{RyditModule, ModuleResult, ModuleMetadata};
/// use serde_json::Value;
/// use std::collections::HashMap;
///
/// struct MiModulo;
///
/// impl RyditModule for MiModulo {
///     fn name(&self) -> &'static str { "mi_modulo" }
///     fn version(&self) -> &'static str { "1.0.0" }
///     fn register(&self) -> HashMap<&'static str, &'static str> {
///         let mut cmds = HashMap::new();
///         cmds.insert("saludar", "Saluda al usuario");
///         cmds
///     }
///     fn execute(&self, command: &str, params: Value) -> ModuleResult {
///         match command {
///             "saludar" => Ok(Value::String("Hola!".to_string())),
///             _ => Err(rydit_core::ModuleError {
///                 code: "UNKNOWN_COMMAND".to_string(),
///                 message: format!("Comando desconocido: {}", command),
///             }),
///         }
///     }
/// }
/// ```
pub trait RyditModule: Send + Sync {
    /// Nombre único del módulo
    fn name(&self) -> &'static str;

    /// Versión del módulo
    fn version(&self) -> &'static str;

    /// Registro de comandos disponibles
    /// Retorna: HashMap<nombre_comando, descripción>
    fn register(&self) -> HashMap<&'static str, &'static str>;

    /// Ejecuta un comando con parámetros
    ///
    /// # Arguments
    /// * `command` - Nombre del comando
    /// * `params` - Parámetros JSON
    fn execute(&self, command: &str, params: Value) -> ModuleResult;

    /// Metadata del módulo (v0.8.2+)
    ///
    /// Proporciona información descriptiva para el sistema de plugins
    /// y carga dinámica. Por defecto retorna metadata básica.
    fn metadata(&self) -> ModuleMetadata {
        ModuleMetadata {
            name: self.name(),
            version: self.version(),
            authors: vec![],
            description: "",
            license: "MIT",
            dependencies: vec![],
        }
    }

    /// Hook llamado antes de recargar el módulo (hot reload)
    ///
    /// Permite limpiar recursos o guardar estado antes de una recarga.
    /// Por defecto no hace nada.
    fn on_reload(&mut self) {}

    /// Hook llamado al descargar el módulo
    ///
    /// Permite limpiar recursos asignados.
    /// Por defecto no hace nada.
    fn on_unload(&mut self) {}
}

/// Registro de módulos disponibles (v0.8.2+)
///
/// Soporta carga dinámica, hot reload y metadata de módulos
#[derive(Default)]
pub struct ModuleRegistry {
    modules: HashMap<String, Box<dyn RyditModule>>,
}

impl ModuleRegistry {
    /// Crea un nuevo registro vacío
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    /// Registra un módulo
    pub fn register<M: RyditModule + 'static>(&mut self, module: M) {
        let name = module.name().to_string();
        self.modules.insert(name, Box::new(module));
    }

    /// Obtiene un módulo por nombre
    pub fn get(&self, name: &str) -> Option<&dyn RyditModule> {
        self.modules.get(name).map(|b| b.as_ref())
    }

    /// Obtiene un módulo mutable por nombre (para hot reload)
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Box<dyn RyditModule>> {
        self.modules.get_mut(name)
    }

    /// Lista todos los módulos registrados
    pub fn list(&self) -> Vec<&str> {
        self.modules.keys().map(|s| s.as_str()).collect()
    }

    /// Lista todos los módulos con su metadata (v0.8.2+)
    pub fn list_with_metadata(&self) -> Vec<(&str, ModuleMetadata)> {
        self.modules
            .values()
            .map(|m| (m.name(), m.metadata()))
            .collect()
    }

    /// Recarga un módulo (hot reload) (v0.8.2+)
    ///
    /// Llama al hook `on_reload()` del módulo.
    pub fn reload(&mut self, name: &str) {
        if let Some(module) = self.modules.get_mut(name) {
            module.on_reload();
        }
    }

    /// Descarga un módulo (v0.8.2+)
    ///
    /// Llama al hook `on_unload()` y luego remueve el módulo.
    pub fn unload(&mut self, name: &str) {
        if let Some(mut module) = self.modules.remove(name) {
            module.on_unload();
        }
    }

    /// Verifica si un módulo está registrado
    pub fn contains(&self, name: &str) -> bool {
        self.modules.contains_key(name)
    }

    /// Obtiene el número de módulos registrados
    pub fn len(&self) -> usize {
        self.modules.len()
    }

    /// Verifica si el registro está vacío
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Módulo de prueba para tests
    struct TestModule;

    impl RyditModule for TestModule {
        fn name(&self) -> &'static str {
            "test"
        }

        fn version(&self) -> &'static str {
            "1.0.0"
        }

        fn register(&self) -> HashMap<&'static str, &'static str> {
            let mut cmds = HashMap::new();
            cmds.insert("ping", "Test ping command");
            cmds.insert("echo", "Test echo command");
            cmds
        }

        fn execute(&self, command: &str, params: Value) -> ModuleResult {
            match command {
                "ping" => Ok(Value::String("pong".to_string())),
                "echo" => Ok(params),
                _ => Err(ModuleError {
                    code: "UNKNOWN_COMMAND".to_string(),
                    message: format!("Unknown command: {}", command),
                }),
            }
        }

        fn metadata(&self) -> ModuleMetadata {
            ModuleMetadata::new()
                .with_name("test")
                .with_version("1.0.0")
                .with_description("Módulo de prueba para tests")
                .with_license("MIT")
        }
    }

    #[test]
    fn test_module_registry() {
        let mut registry = ModuleRegistry::new();
        registry.register(TestModule);

        assert_eq!(registry.list().len(), 1);
        assert!(registry.get("test").is_some());
        assert!(registry.get("unknown").is_none());
    }

    #[test]
    fn test_module_execute_ping() {
        let mut registry = ModuleRegistry::new();
        registry.register(TestModule);

        let module = registry.get("test").unwrap();
        let result = module.execute("ping", Value::Null).unwrap();
        assert_eq!(result, Value::String("pong".to_string()));
    }

    #[test]
    fn test_module_execute_echo() {
        let mut registry = ModuleRegistry::new();
        registry.register(TestModule);

        let module = registry.get("test").unwrap();
        let input = Value::String("hello".to_string());
        let result = module.execute("echo", input.clone()).unwrap();
        assert_eq!(result, input);
    }

    #[test]
    fn test_module_error() {
        let mut registry = ModuleRegistry::new();
        registry.register(TestModule);

        let module = registry.get("test").unwrap();
        let result = module.execute("unknown", Value::Null);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.code, "UNKNOWN_COMMAND");
    }

    #[test]
    fn test_module_metadata() {
        let mut registry = ModuleRegistry::new();
        registry.register(TestModule);

        let module = registry.get("test").unwrap();
        let metadata = module.metadata();

        assert_eq!(metadata.name, "test");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.description, "Módulo de prueba para tests");
        assert_eq!(metadata.license, "MIT");
    }

    #[test]
    fn test_module_registry_with_metadata() {
        let mut registry = ModuleRegistry::new();
        registry.register(TestModule);

        let list = registry.list_with_metadata();
        assert_eq!(list.len(), 1);

        let (name, metadata) = &list[0];
        assert_eq!(*name, "test");
        assert_eq!(metadata.name, "test");
    }

    #[test]
    fn test_module_reload() {
        let mut registry = ModuleRegistry::new();
        registry.register(TestModule);

        // El reload debería funcionar (on_reload por defecto no hace nada)
        registry.reload("test");
        assert!(registry.contains("test"));

        // Reload en módulo inexistente no hace nada
        registry.reload("nonexistent");
        assert!(!registry.contains("nonexistent"));
    }

    #[test]
    fn test_module_unload() {
        let mut registry = ModuleRegistry::new();
        registry.register(TestModule);

        assert!(registry.contains("test"));

        // Unload debería funcionar
        registry.unload("test");
        assert!(!registry.contains("test"));

        // Unload de módulo inexistente no hace nada
        registry.unload("nonexistent");
        assert!(!registry.contains("nonexistent"));
    }

    #[test]
    fn test_module_registry_len() {
        let mut registry = ModuleRegistry::new();
        assert!(registry.is_empty());

        registry.register(TestModule);
        assert_eq!(registry.len(), 1);
        assert!(!registry.is_empty());

        registry.unload("test");
        assert!(registry.is_empty());
    }
}
