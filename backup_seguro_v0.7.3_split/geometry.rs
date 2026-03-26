// crates/rydit-rs/src/geometry.rs
// Módulo de Geometría - Ilusiones Ópticas
// Implementa el trait RyditModule de rydit-core

use rydit_core::{RyditModule, ModuleResult, ModuleError};
use serde_json::{Value, json};
use std::collections::HashMap;

pub struct GeometryModule;

impl RyditModule for GeometryModule {
    fn name(&self) -> &'static str {
        "geometry"
    }

    fn version(&self) -> &'static str {
        "0.7.3"
    }

    fn register(&self) -> HashMap<&'static str, &'static str> {
        let mut cmds = HashMap::new();
        cmds.insert("penrose", "Triángulo de Penrose");
        cmds.insert("impossible_cube", "Cubo imposible");
        cmds.insert("spiral", "Espiral óptica");
        cmds
    }

    fn execute(&self, command: &str, params: Value) -> ModuleResult {
        match command {
            "penrose" => self.penrose(params),
            "impossible_cube" => self.impossible_cube(params),
            "spiral" => self.spiral(params),
            _ => Err(ModuleError {
                code: "UNKNOWN_COMMAND".to_string(),
                message: format!("Comando desconocido: {}", command),
            }),
        }
    }
}

impl GeometryModule {
    fn penrose(&self, _params: Value) -> ModuleResult {
        // TODO: Implementar triángulo de Penrose
        Ok(json!({
            "status": "not_implemented",
            "message": "Triángulo de Penrose - Próximamente"
        }))
    }

    fn impossible_cube(&self, _params: Value) -> ModuleResult {
        // TODO: Implementar cubo imposible
        Ok(json!({
            "status": "not_implemented",
            "message": "Cubo imposible - Próximamente"
        }))
    }

    fn spiral(&self, _params: Value) -> ModuleResult {
        // TODO: Implementar espiral óptica
        Ok(json!({
            "status": "not_implemented",
            "message": "Espiral óptica - Próximamente"
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometry_module_name() {
        let module = GeometryModule;
        assert_eq!(module.name(), "geometry");
        assert_eq!(module.version(), "0.7.3");
    }

    #[test]
    fn test_geometry_register() {
        let module = GeometryModule;
        let cmds = module.register();
        
        assert!(cmds.contains_key("penrose"));
        assert!(cmds.contains_key("impossible_cube"));
        assert!(cmds.contains_key("spiral"));
    }

    #[test]
    fn test_geometry_not_implemented() {
        let module = GeometryModule;
        let result = module.execute("penrose", json!([])).unwrap();
        
        let obj = result.as_object().unwrap();
        assert_eq!(obj.get("status").unwrap(), "not_implemented");
    }

    #[test]
    fn test_unknown_command() {
        let module = GeometryModule;
        let result = module.execute("unknown", json!([]));
        
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "UNKNOWN_COMMAND");
    }
}
