//! RyDit Science - Módulo de Ciencia para RyDit
//!
//! Proporciona funcionalidad de:
//! - Curvas Bezier (lineal, cuadrática, cúbica)
//! - Estadísticas (media, mediana, mínimo, máximo)
//! - Geometría (ilusiones ópticas: Penrose, Cubo imposible, Espiral)

pub mod geometry;

use rydit_core::{ModuleError, ModuleResult, RyditModule};
use serde_json::{json, Value};
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
        cmds.insert("geometry::penrose", "Triángulo de Penrose");
        cmds.insert("geometry::impossible_cube", "Cubo imposible");
        cmds.insert("geometry::spiral", "Espiral óptica");
        cmds.insert("geometry::muller_lyer", "Ilusión Müller-Lyer");
        cmds.insert("geometry::ponzo", "Ilusión de Ponzo");
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
            "geometry::penrose" => self.geometry_penrose(params),
            "geometry::impossible_cube" => self.geometry_impossible_cube(params),
            "geometry::spiral" => self.geometry_spiral(params),
            "geometry::muller_lyer" => self.geometry_muller_lyer(params),
            "geometry::ponzo" => self.geometry_ponzo(params),
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
        let t = arr[4].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);

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
                message:
                    "bezier::quadratic requires 7 params: p0_x, p0_y, p1_x, p1_y, p2_x, p2_y, t"
                        .to_string(),
            });
        }

        let p0_x = arr[0].as_f64().unwrap_or(0.0);
        let p0_y = arr[1].as_f64().unwrap_or(0.0);
        let p1_x = arr[2].as_f64().unwrap_or(0.0);
        let p1_y = arr[3].as_f64().unwrap_or(0.0);
        let p2_x = arr[4].as_f64().unwrap_or(0.0);
        let p2_y = arr[5].as_f64().unwrap_or(0.0);
        let t = arr[6].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);

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
        let t = arr[8].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);

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

        let median = if nums.len().is_multiple_of(2) {
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

    // ================================================================
    // Funciones de Geometría - Ilusiones Ópticas
    // ================================================================

    /// Triángulo de Penrose (tribar imposible)
    fn geometry_penrose(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 3 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "geometry::penrose requires 3 params: center_x, center_y, size"
                    .to_string(),
            });
        }

        let center_x = arr[0].as_f64().unwrap_or(400.0);
        let center_y = arr[1].as_f64().unwrap_or(300.0);
        let size = arr[2].as_f64().unwrap_or(100.0);

        Ok(geometry::penrose(center_x, center_y, size))
    }

    /// Cubo imposible (Necker cube)
    fn geometry_impossible_cube(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 3 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "geometry::impossible_cube requires 3 params: center_x, center_y, size"
                    .to_string(),
            });
        }

        let center_x = arr[0].as_f64().unwrap_or(400.0);
        let center_y = arr[1].as_f64().unwrap_or(300.0);
        let size = arr[2].as_f64().unwrap_or(100.0);

        Ok(geometry::impossible_cube(center_x, center_y, size))
    }

    /// Espiral óptica (Arquímedes)
    fn geometry_spiral(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 5 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message:
                    "geometry::spiral requires 5 params: center_x, center_y, turns, radius, points"
                        .to_string(),
            });
        }

        let center_x = arr[0].as_f64().unwrap_or(400.0);
        let center_y = arr[1].as_f64().unwrap_or(300.0);
        let turns = arr[2].as_i64().unwrap_or(3) as i32;
        let radius = arr[3].as_f64().unwrap_or(100.0);
        let points = arr[4].as_i64().unwrap_or(20) as i32;

        Ok(geometry::spiral(center_x, center_y, turns, radius, points))
    }

    /// Ilusión de Müller-Lyer (flechas)
    fn geometry_muller_lyer(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 3 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "geometry::muller_lyer requires 3 params: center_x, center_y, length"
                    .to_string(),
            });
        }

        let center_x = arr[0].as_f64().unwrap_or(400.0);
        let center_y = arr[1].as_f64().unwrap_or(300.0);
        let length = arr[2].as_f64().unwrap_or(200.0);

        Ok(geometry::muller_lyer(center_x, center_y, length))
    }

    /// Ilusión de Ponzo (perspectiva)
    fn geometry_ponzo(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 5 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "geometry::ponzo requires 5 params: center_x, center_y, height, width_top, width_bottom".to_string(),
            });
        }

        let center_x = arr[0].as_f64().unwrap_or(400.0);
        let center_y = arr[1].as_f64().unwrap_or(300.0);
        let height = arr[2].as_f64().unwrap_or(300.0);
        let width_top = arr[3].as_f64().unwrap_or(100.0);
        let width_bottom = arr[4].as_f64().unwrap_or(300.0);

        Ok(geometry::ponzo(
            center_x,
            center_y,
            height,
            width_top,
            width_bottom,
        ))
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
        assert!(cmds.contains_key("geometry::penrose"));
        assert!(cmds.contains_key("geometry::impossible_cube"));
        assert!(cmds.contains_key("geometry::spiral"));
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

    // Tests de Geometría
    #[test]
    fn test_geometry_penrose() {
        let module = ScienceModule;
        let params = json!([400.0, 300.0, 100.0]);
        let result = module.execute("geometry::penrose", params).unwrap();

        let lines = result.as_array().unwrap();
        assert!(!lines.is_empty());
        assert!(lines.len() >= 10);
    }

    #[test]
    fn test_geometry_impossible_cube() {
        let module = ScienceModule;
        let params = json!([400.0, 300.0, 100.0]);
        let result = module.execute("geometry::impossible_cube", params).unwrap();

        let lines = result.as_array().unwrap();
        assert!(!lines.is_empty());
        assert!(lines.len() >= 12);
    }

    #[test]
    fn test_geometry_spiral() {
        let module = ScienceModule;
        let params = json!([400.0, 300.0, 3, 100.0, 20]);
        let result = module.execute("geometry::spiral", params).unwrap();

        let points = result.as_array().unwrap();
        assert_eq!(points.len(), 60); // 3 turns * 20 points
    }

    #[test]
    fn test_geometry_muller_lyer() {
        let module = ScienceModule;
        let params = json!([400.0, 300.0, 200.0]);
        let result = module.execute("geometry::muller_lyer", params).unwrap();

        let lines = result.as_array().unwrap();
        assert_eq!(lines.len(), 10);
    }

    #[test]
    fn test_geometry_ponzo() {
        let module = ScienceModule;
        let params = json!([400.0, 300.0, 300.0, 100.0, 300.0]);
        let result = module.execute("geometry::ponzo", params).unwrap();

        let lines = result.as_array().unwrap();
        assert_eq!(lines.len(), 6); // 2 rieles + 4 horizontales
    }
}
