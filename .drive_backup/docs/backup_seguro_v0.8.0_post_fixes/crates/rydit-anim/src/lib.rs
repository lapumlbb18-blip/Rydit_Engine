//! RyDit Anim - Módulo de Animación para RyDit
//!
//! Implementa principios de animación de Disney:
//! - Principio #1: Squash & Stretch (Deformación)
//! - Principio #2: Anticipation (Anticipación)
//! - Principio #6: Slow In & Slow Out (Easing)

use rydit_core::{ModuleError, ModuleResult, RyditModule};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Módulo de Animación - 12 principios de Disney
pub struct AnimModule;

impl RyditModule for AnimModule {
    fn name(&self) -> &'static str {
        "anim"
    }

    fn version(&self) -> &'static str {
        "0.7.3"
    }

    fn register(&self) -> HashMap<&'static str, &'static str> {
        let mut cmds = HashMap::new();
        cmds.insert("ease_in", "Easing In - comienza lento, acelera");
        cmds.insert("ease_out", "Easing Out - comienza rápido, frena");
        cmds.insert("ease_in_out", "Easing In-Out - combina ambos");
        cmds.insert("squash", "Squash - aplasta (mantiene área)");
        cmds.insert("stretch", "Stretch - estira (mantiene área)");
        cmds.insert("anticipate", "Anticipation - retrocede antes de avanzar");
        cmds
    }

    fn execute(&self, command: &str, params: Value) -> ModuleResult {
        match command {
            "ease_in" => self.ease_in(params),
            "ease_out" => self.ease_out(params),
            "ease_in_out" => self.ease_in_out(params),
            "squash" => self.squash(params),
            "stretch" => self.stretch(params),
            "anticipate" => self.anticipate(params),
            _ => Err(ModuleError {
                code: "UNKNOWN_COMMAND".to_string(),
                message: format!("Comando desconocido: {}", command),
            }),
        }
    }
}

impl AnimModule {
    /// Easing In - Comienza lento, acelera al final
    /// Fórmula: t²
    fn ease_in(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 1 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "ease_in requires 1 param: t (0.0-1.0)".to_string(),
            });
        }

        let t = arr[0].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);
        Ok(json!(t * t))
    }

    /// Easing Out - Comienza rápido, frena al final
    /// Fórmula: t * (2 - t)
    fn ease_out(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 1 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "ease_out requires 1 param: t (0.0-1.0)".to_string(),
            });
        }

        let t = arr[0].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);
        Ok(json!(t * (2.0 - t)))
    }

    /// Easing In-Out - Combina ambos, suave en extremos
    /// Fórmula: 2t² (t<0.5) o 1-2(1-t)² (t>=0.5)
    fn ease_in_out(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 1 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "ease_in_out requires 1 param: t (0.0-1.0)".to_string(),
            });
        }

        let t = arr[0].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);
        let result = if t < 0.5 {
            2.0 * t * t
        } else {
            1.0 - 2.0 * (1.0 - t) * (1.0 - t)
        };
        Ok(json!(result))
    }

    /// Squash - Aplasta objeto (mantiene área)
    /// Retorna: [factor, 1/factor]
    fn squash(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 1 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "squash requires 1 param: factor (0.5-2.0)".to_string(),
            });
        }

        let factor = arr[0].as_f64().unwrap_or(1.0).max(0.5).min(2.0);
        Ok(json!([factor, 1.0 / factor]))
    }

    /// Stretch - Estira objeto (mantiene área)
    /// Retorna: [1/factor, factor]
    fn stretch(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 1 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "stretch requires 1 param: factor (0.5-2.0)".to_string(),
            });
        }

        let factor = arr[0].as_f64().unwrap_or(1.0).max(0.5).min(2.0);
        Ok(json!([1.0 / factor, factor]))
    }

    /// Anticipation - Retrocede antes de avanzar
    /// Retorna: pos + dir * amount (dir = -1 si target>pos, else 1)
    fn anticipate(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 3 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "anticipate requires 3 params: pos, target, amount".to_string(),
            });
        }

        let pos = arr[0].as_f64().unwrap_or(0.0);
        let target = arr[1].as_f64().unwrap_or(0.0);
        let amount = arr[2].as_f64().unwrap_or(0.0);

        let dir = if target > pos { -1.0 } else { 1.0 };
        Ok(json!(pos + dir * amount))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anim_module_name() {
        let module = AnimModule;
        assert_eq!(module.name(), "anim");
        assert_eq!(module.version(), "0.7.3");
    }

    #[test]
    fn test_anim_register() {
        let module = AnimModule;
        let cmds = module.register();

        assert!(cmds.contains_key("ease_in"));
        assert!(cmds.contains_key("ease_out"));
        assert!(cmds.contains_key("squash"));
        assert!(cmds.contains_key("anticipate"));
    }

    #[test]
    fn test_ease_in() {
        let module = AnimModule;
        // ease_in(0.5) = 0.25
        let params = json!([0.5]);
        let result = module.execute("ease_in", params).unwrap();
        assert!((result.as_f64().unwrap() - 0.25).abs() < 0.001);
    }

    #[test]
    fn test_ease_out() {
        let module = AnimModule;
        // ease_out(0.5) = 0.75
        let params = json!([0.5]);
        let result = module.execute("ease_out", params).unwrap();
        assert!((result.as_f64().unwrap() - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_ease_in_out() {
        let module = AnimModule;
        // ease_in_out(0.5) = 0.5
        let params = json!([0.5]);
        let result = module.execute("ease_in_out", params).unwrap();
        assert!((result.as_f64().unwrap() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_squash() {
        let module = AnimModule;
        // squash(2.0) = [2.0, 0.5]
        let params = json!([2.0]);
        let result = module.execute("squash", params).unwrap();
        let arr = result.as_array().unwrap();
        assert!((arr[0].as_f64().unwrap() - 2.0).abs() < 0.001);
        assert!((arr[1].as_f64().unwrap() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_stretch() {
        let module = AnimModule;
        // stretch(2.0) = [0.5, 2.0]
        let params = json!([2.0]);
        let result = module.execute("stretch", params).unwrap();
        let arr = result.as_array().unwrap();
        assert!((arr[0].as_f64().unwrap() - 0.5).abs() < 0.001);
        assert!((arr[1].as_f64().unwrap() - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_anticipate() {
        let module = AnimModule;
        // anticipate(100, 200, 20) = 80 (retrocede)
        let params = json!([100.0, 200.0, 20.0]);
        let result = module.execute("anticipate", params).unwrap();
        assert!((result.as_f64().unwrap() - 80.0).abs() < 0.001);
    }

    #[test]
    fn test_unknown_command() {
        let module = AnimModule;
        let result = module.execute("unknown", json!([]));

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "UNKNOWN_COMMAND");
    }
}
