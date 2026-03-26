//! RyDit Science - Módulo de Ciencia para RyDit
//! 
//! Proporciona funcionalidad de:
//! - Curvas Bezier (lineal, cuadrática, cúbica)
//! - Estadísticas (media, mediana, mínimo, máximo)

use rydit_core::{RyditModule, ModuleResult, ModuleError};
use serde_json::{Value, json};
use std::collections::HashMap;

/// Módulo de Ciencia - Bezier y Estadísticas
pub struct ScienceModule;

impl RyditModule for ScienceModule {
    fn name(&self) -> &'static str {
        "science"
    }

    fn version(&self) -> &'static str {
        "0.7.3"
    }

    fn register(&self) -> HashMap<&'static str, &'static str> {
        let mut cmds = HashMap::new();
        cmds.insert("bezier::linear", "Curva Bezier lineal");
        cmds.insert("bezier::quadratic", "Curva Bezier cuadrática");
        cmds.insert("bezier::cubic", "Curva Bezier cúbica");
        cmds.insert("stats::mean", "Media aritmética");
        cmds.insert("stats::median", "Mediana");
        cmds.insert("stats::min", "Valor mínimo");
        cmds.insert("stats::max", "Valor máximo");
        cmds
    }

    fn execute(&self, command: &str, params: Value) -> ModuleResult {
        match command {
            "bezier::linear" => self.bezier_linear(params),
            "bezier::quadratic" => self.bezier_quadratic(params),
            "bezier::cubic" => self.bezier_cubic(params),
            "stats::mean" => self.stats_mean(params),
            "stats::median" => self.stats_median(params),
            "stats::min" => self.stats_min(params),
            "stats::max" => self.stats_max(params),
            _ => Err(ModuleError {
                code: "UNKNOWN_COMMAND".to_string(),
                message: format!("Comando desconocido: {}", command),
            }),
        }
    }
}

impl ScienceModule {
    /// Curva Bezier lineal: P(t) = (1-t)*P0 + t*P1
    fn bezier_linear(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 5 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "bezier::linear requires 5 params: p0_x, p0_y, p1_x, p1_y, t".to_string(),
            });
        }

        let p0_x = arr[0].as_f64().unwrap_or(0.0);
        let p0_y = arr[1].as_f64().unwrap_or(0.0);
        let p1_x = arr[2].as_f64().unwrap_or(0.0);
        let p1_y = arr[3].as_f64().unwrap_or(0.0);
        let t = arr[4].as_f64().unwrap_or(0.0).max(0.0).min(1.0);

        let x = (1.0 - t) * p0_x + t * p1_x;
        let y = (1.0 - t) * p0_y + t * p1_y;

        Ok(json!([x, y]))
    }

    /// Curva Bezier cuadrática: P(t) = (1-t)²*P0 + 2(1-t)t*P1 + t²*P2
    fn bezier_quadratic(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 7 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "bezier::quadratic requires 7 params: p0_x, p0_y, p1_x, p1_y, p2_x, p2_y, t".to_string(),
            });
        }

        let p0_x = arr[0].as_f64().unwrap_or(0.0);
        let p0_y = arr[1].as_f64().unwrap_or(0.0);
        let p1_x = arr[2].as_f64().unwrap_or(0.0);
        let p1_y = arr[3].as_f64().unwrap_or(0.0);
        let p2_x = arr[4].as_f64().unwrap_or(0.0);
        let p2_y = arr[5].as_f64().unwrap_or(0.0);
        let t = arr[6].as_f64().unwrap_or(0.0).max(0.0).min(1.0);

        let mt = 1.0 - t;
        let x = mt * mt * p0_x + 2.0 * mt * t * p1_x + t * t * p2_x;
        let y = mt * mt * p0_y + 2.0 * mt * t * p1_y + t * t * p2_y;

        Ok(json!([x, y]))
    }

    /// Curva Bezier cúbica: P(t) = (1-t)³*P0 + 3(1-t)²t*P1 + 3(1-t)t²*P2 + t³*P3
    fn bezier_cubic(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 9 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "bezier::cubic requires 9 params: p0_x, p0_y, p1_x, p1_y, p2_x, p2_y, p3_x, p3_y, t".to_string(),
            });
        }

        let p0_x = arr[0].as_f64().unwrap_or(0.0);
        let p0_y = arr[1].as_f64().unwrap_or(0.0);
        let p1_x = arr[2].as_f64().unwrap_or(0.0);
        let p1_y = arr[3].as_f64().unwrap_or(0.0);
        let p2_x = arr[4].as_f64().unwrap_or(0.0);
        let p2_y = arr[5].as_f64().unwrap_or(0.0);
        let p3_x = arr[6].as_f64().unwrap_or(0.0);
        let p3_y = arr[7].as_f64().unwrap_or(0.0);
        let t = arr[8].as_f64().unwrap_or(0.0).max(0.0).min(1.0);

        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let t2 = t * t;

        let x = mt2 * mt * p0_x + 3.0 * mt2 * t * p1_x + 3.0 * mt * t2 * p2_x + t2 * t * p3_x;
        let y = mt2 * mt * p0_y + 3.0 * mt2 * t * p1_y + 3.0 * mt * t2 * p2_y + t2 * t * p3_y;

        Ok(json!([x, y]))
    }

    /// Media aritmética: sum / n
    fn stats_mean(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.is_empty() {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "Empty array".to_string(),
            });
        }

        let sum: f64 = arr.iter().filter_map(|v| v.as_f64()).sum();
        Ok(json!(sum / arr.len() as f64))
    }

    /// Mediana: valor central de array ordenado
    fn stats_median(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        let mut nums: Vec<f64> = arr.iter().filter_map(|v| v.as_f64()).collect();

        if nums.is_empty() {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "Empty array or no numbers".to_string(),
            });
        }

        nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = nums.len() / 2;

        let median = if nums.len() % 2 == 0 {
            (nums[mid - 1] + nums[mid]) / 2.0
        } else {
            nums[mid]
        };

        Ok(json!(median))
    }

    /// Valor mínimo de un array
    fn stats_min(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        let mut min_val = f64::MAX;
        let mut found = false;

        for v in arr {
            if let Some(n) = v.as_f64() {
                if n < min_val {
                    min_val = n;
                }
                found = true;
            }
        }

        if found {
            Ok(json!(min_val))
        } else {
            Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "No numbers in array".to_string(),
            })
        }
    }

    /// Valor máximo de un array
    fn stats_max(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        let mut max_val = f64::MIN;
        let mut found = false;

        for v in arr {
            if let Some(n) = v.as_f64() {
                if n > max_val {
                    max_val = n;
                }
                found = true;
            }
        }

        if found {
            Ok(json!(max_val))
        } else {
            Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "No numbers in array".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_science_module_name() {
        let module = ScienceModule;
        assert_eq!(module.name(), "science");
        assert_eq!(module.version(), "0.7.3");
    }

    #[test]
    fn test_science_register() {
        let module = ScienceModule;
        let cmds = module.register();
        
        assert!(cmds.contains_key("bezier::linear"));
        assert!(cmds.contains_key("bezier::cubic"));
        assert!(cmds.contains_key("stats::mean"));
        assert!(cmds.contains_key("stats::median"));
    }

    #[test]
    fn test_bezier_linear() {
        let module = ScienceModule;
        let params = json!([0.0, 0.0, 100.0, 100.0, 0.5]);
        let result = module.execute("bezier::linear", params).unwrap();
        
        assert_eq!(result, json!([50.0, 50.0]));
    }

    #[test]
    fn test_bezier_cubic() {
        let module = ScienceModule;
        // p0=(0,0), p1=(30,100), p2=(70,100), p3=(100,0), t=0.5
        let params = json!([0.0, 0.0, 30.0, 100.0, 70.0, 100.0, 100.0, 0.0, 0.5]);
        let result = module.execute("bezier::cubic", params).unwrap();
        
        assert_eq!(result, json!([50.0, 75.0]));
    }

    #[test]
    fn test_stats_mean() {
        let module = ScienceModule;
        let params = json!([1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = module.execute("stats::mean", params).unwrap();
        
        assert_eq!(result, json!(3.0));
    }

    #[test]
    fn test_stats_median_odd() {
        let module = ScienceModule;
        let params = json!([1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = module.execute("stats::median", params).unwrap();
        
        assert_eq!(result, json!(3.0));
    }

    #[test]
    fn test_stats_median_even() {
        let module = ScienceModule;
        let params = json!([1.0, 2.0, 3.0, 4.0]);
        let result = module.execute("stats::median", params).unwrap();
        
        assert_eq!(result, json!(2.5));
    }

    #[test]
    fn test_stats_min_max() {
        let module = ScienceModule;
        let params = json!([3.0, 1.0, 4.0, 1.0, 5.0]);
        
        let min_result = module.execute("stats::min", params.clone()).unwrap();
        assert_eq!(min_result, json!(1.0));
        
        let max_result = module.execute("stats::max", params).unwrap();
        assert_eq!(max_result, json!(5.0));
    }

    #[test]
    fn test_unknown_command() {
        let module = ScienceModule;
        let result = module.execute("unknown", json!([]));
        
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "UNKNOWN_COMMAND");
    }
}
