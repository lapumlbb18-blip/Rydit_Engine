//! Módulo de Ejemplo para RyDit Engine
//!
//! Este módulo demuestra cómo crear un módulo dinámico
//! que puede cargarse en runtime con el Sistema Universal Ry.
//!
//! # Compilación
//! ```bash
//! cd demos/modulo_ejemplo
//! cargo build --release
//! ```
//!
//! # Carga en RyDit
//! ```bash
//! # Linux
//! cp target/release/libmodulo_ejemplo.so ../..
//! echo '{"method":"module::list"}' | rydit-rs --lazos
//! ```

use rydit_core::{ModuleError, ModuleMetadata, ModuleResult, RyditModule};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Módulo de Ejemplo
pub struct ModuloEjemplo;

impl RyditModule for ModuloEjemplo {
    fn name(&self) -> &'static str {
        "modulo_ejemplo"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn register(&self) -> HashMap<&'static str, &'static str> {
        let mut cmds = HashMap::new();
        
        cmds.insert("saludar", "Saluda a una persona");
        cmds.insert("despedir", "Despide a una persona");
        cmds.insert("sumar", "Suma dos números");
        cmds.insert("multiplicar", "Multiplica dos números");
        cmds.insert("info", "Información del módulo");
        cmds.insert("pi", "Retorna el valor de PI");
        cmds.insert("cuadrado", "Calcula el cuadrado de un número");
        
        cmds
    }

    fn execute(&self, command: &str, params: Value) -> ModuleResult {
        match command {
            "saludar" => {
                let arr = params.as_array().ok_or_else(|| ModuleError {
                    code: "INVALID_PARAMS".to_string(),
                    message: "saludar requiere [nombre]".to_string(),
                })?;
                
                let nombre = arr[0].as_str().unwrap_or("Mundo");
                Ok(json!(format!("¡Hola, {}! Bienvenido al módulo de ejemplo 🦀", nombre)))
            }
            
            "despedir" => {
                let arr = params.as_array().ok_or_else(|| ModuleError {
                    code: "INVALID_PARAMS".to_string(),
                    message: "despedir requiere [nombre]".to_string(),
                })?;
                
                let nombre = arr[0].as_str().unwrap_or("Amigo");
                Ok(json!(format!("¡Hasta luego, {}! Que tengas un gran día 👋", nombre)))
            }
            
            "sumar" => {
                let arr = params.as_array().ok_or_else(|| ModuleError {
                    code: "INVALID_PARAMS".to_string(),
                    message: "sumar requiere [a, b]".to_string(),
                })?;
                
                if arr.len() != 2 {
                    return Err(ModuleError {
                        code: "INVALID_PARAMS".to_string(),
                        message: "sumar requiere exactamente 2 números".to_string(),
                    });
                }
                
                let a = arr[0].as_f64().unwrap_or(0.0);
                let b = arr[1].as_f64().unwrap_or(0.0);
                
                Ok(json!(a + b))
            }
            
            "multiplicar" => {
                let arr = params.as_array().ok_or_else(|| ModuleError {
                    code: "INVALID_PARAMS".to_string(),
                    message: "multiplicar requiere [a, b]".to_string(),
                })?;
                
                if arr.len() != 2 {
                    return Err(ModuleError {
                        code: "INVALID_PARAMS".to_string(),
                        message: "multiplicar requiere exactamente 2 números".to_string(),
                    });
                }
                
                let a = arr[0].as_f64().unwrap_or(0.0);
                let b = arr[1].as_f64().unwrap_or(0.0);
                
                Ok(json!(a * b))
            }
            
            "info" => {
                Ok(json!({
                    "nombre": "modulo_ejemplo",
                    "version": "1.0.0",
                    "descripcion": "Módulo de ejemplo para RyDit Engine",
                    "autor": "Comunidad RyDit",
                    "license": "MIT",
                    "comandos": ["saludar", "despedir", "sumar", "multiplicar", "pi", "cuadrado"]
                }))
            }
            
            "pi" => {
                Ok(json!(std::f64::consts::PI))
            }
            
            "cuadrado" => {
                let arr = params.as_array().ok_or_else(|| ModuleError {
                    code: "INVALID_PARAMS".to_string(),
                    message: "cuadrado requiere [numero]".to_string(),
                })?;
                
                let numero = arr[0].as_f64().unwrap_or(0.0);
                Ok(json!(numero * numero))
            }
            
            _ => Err(ModuleError {
                code: "UNKNOWN_COMMAND".to_string(),
                message: format!("Comando desconocido: {}. Usa module::info para ver comandos disponibles.", command),
            })
        }
    }

    fn metadata(&self) -> ModuleMetadata {
        ModuleMetadata::new()
            .with_name("modulo_ejemplo")
            .with_version("1.0.0")
            .with_description("Módulo de ejemplo para RyDit Engine - Demuestra carga dinámica")
            .with_license("MIT")
            .with_authors(vec!["Comunidad RyDit"])
    }
}

// ============================================================================
// FUNCIÓN DE EXPORTACIÓN PARA CARGA DINÁMICA
// ============================================================================

/// Función exportada para carga dinámica
/// El loader busca este símbolo para crear una instancia del módulo
#[no_mangle]
pub extern "C" fn create_module() -> *mut dyn RyditModule {
    Box::into_raw(Box::new(ModuloEjemplo))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modulo_nombre() {
        let modulo = ModuloEjemplo;
        assert_eq!(modulo.name(), "modulo_ejemplo");
        assert_eq!(modulo.version(), "1.0.0");
    }

    #[test]
    fn test_saludar() {
        let modulo = ModuloEjemplo;
        let result = modulo.execute("saludar", json!(["Rust"])).unwrap();
        assert!(result.as_str().unwrap().contains("Rust"));
    }

    #[test]
    fn test_sumar() {
        let modulo = ModuloEjemplo;
        let result = modulo.execute("sumar", json!([5.0, 3.0])).unwrap();
        assert_eq!(result.as_f64().unwrap(), 8.0);
    }

    #[test]
    fn test_multiplicar() {
        let modulo = ModuloEjemplo;
        let result = modulo.execute("multiplicar", json!([4.0, 5.0])).unwrap();
        assert_eq!(result.as_f64().unwrap(), 20.0);
    }

    #[test]
    fn test_pi() {
        let modulo = ModuloEjemplo;
        let result = modulo.execute("pi", json!([])).unwrap();
        let pi = result.as_f64().unwrap();
        assert!((pi - std::f64::consts::PI).abs() < 0.0001);
    }

    #[test]
    fn test_cuadrado() {
        let modulo = ModuloEjemplo;
        let result = modulo.execute("cuadrado", json!([7.0])).unwrap();
        assert_eq!(result.as_f64().unwrap(), 49.0);
    }

    #[test]
    fn test_info() {
        let modulo = ModuloEjemplo;
        let result = modulo.execute("info", json!([])).unwrap();
        let info = result.as_object().unwrap();
        assert_eq!(info["nombre"], "modulo_ejemplo");
        assert_eq!(info["version"], "1.0.0");
    }

    #[test]
    fn test_register() {
        let modulo = ModuloEjemplo;
        let cmds = modulo.register();
        
        assert!(cmds.contains_key("saludar"));
        assert!(cmds.contains_key("sumar"));
        assert!(cmds.contains_key("multiplicar"));
        assert!(cmds.contains_key("pi"));
        assert!(cmds.contains_key("info"));
    }
}
