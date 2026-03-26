//! RyDit Core - Trait y Registro para módulos
//!
//! Proporciona la interfaz común que todos los módulos deben implementar.

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

/// Trait que todos los módulos deben implementar
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
}

/// Registro de módulos disponibles
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

    /// Lista todos los módulos registrados
    pub fn list(&self) -> Vec<&str> {
        self.modules.keys().map(|s| s.as_str()).collect()
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
}
