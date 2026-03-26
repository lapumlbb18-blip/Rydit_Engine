//! RyDit Physics - Módulo de Física para RyDit
//!
//! Proporciona funcionalidad de:
//! - Proyectil 2D (trayectoria, altura máxima, alcance)
//! - Gravedad N-cuerpos (2 cuerpos)

use rydit_core::{ModuleError, ModuleResult, RyditModule};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Módulo de Física - Proyectil y Gravedad
pub struct PhysicsModule;

impl RyditModule for PhysicsModule {
    fn name(&self) -> &'static str {
        "physics"
    }

    fn version(&self) -> &'static str {
        "0.7.3"
    }

    fn register(&self) -> HashMap<&'static str, &'static str> {
        let mut cmds = HashMap::new();
        cmds.insert("projectile", "Simulación de proyectil");
        cmds.insert("nbody_2", "Simulación N-cuerpos (2 cuerpos)");
        cmds
    }

    fn execute(&self, command: &str, params: Value) -> ModuleResult {
        match command {
            "projectile" => self.projectile(params),
            "nbody_2" => self.nbody_2(params),
            _ => Err(ModuleError {
                code: "UNKNOWN_COMMAND".to_string(),
                message: format!("Comando desconocido: {}", command),
            }),
        }
    }
}

impl PhysicsModule {
    /// Simulación de proyectil 2D
    ///
    /// # Params
    /// - x0, y0: Posición inicial
    /// - v0: Velocidad inicial (m/s)
    /// - angle: Ángulo en grados
    ///
    /// # Returns
    /// [x_final, y_final, flight_time, max_height, range]
    fn projectile(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 4 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "physics::projectile requires 4 params: x0, y0, v0, angle".to_string(),
            });
        }

        let x0 = arr[0].as_f64().unwrap_or(0.0);
        let y0 = arr[1].as_f64().unwrap_or(0.0);
        let v0 = arr[2].as_f64().unwrap_or(0.0);
        let angle = arr[3].as_f64().unwrap_or(0.0);

        let rad = angle.to_radians();
        let vx = v0 * rad.cos();
        let vy = v0 * rad.sin();
        let g = 9.81;

        let flight_time = 2.0 * vy / g;
        let max_height = (vy * vy) / (2.0 * g);
        let range = vx * flight_time;

        Ok(json!([
            x0 + vx * flight_time, // x final
            y0,                    // y final
            flight_time,           // tiempo vuelo
            max_height,            // altura máxima
            range                  // alcance horizontal
        ]))
    }

    /// Gravedad entre 2 cuerpos (Ley de Newton)
    ///
    /// # Params
    /// - m1, m2: Masas de los cuerpos
    /// - x1, y1: Posición del cuerpo 1
    /// - x2, y2: Posición del cuerpo 2
    /// - G: Constante gravitacional (default: 6.674e-11)
    ///
    /// # Returns
    /// [fx1, fy1, fx2, fy2, distancia]
    fn nbody_2(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 7 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "physics::nbody_2 requires 7 params: m1, m2, x1, y1, x2, y2, G"
                    .to_string(),
            });
        }

        let m1 = arr[0].as_f64().unwrap_or(0.0);
        let m2 = arr[1].as_f64().unwrap_or(0.0);
        let x1 = arr[2].as_f64().unwrap_or(0.0);
        let y1 = arr[3].as_f64().unwrap_or(0.0);
        let x2 = arr[4].as_f64().unwrap_or(0.0);
        let y2 = arr[5].as_f64().unwrap_or(0.0);
        let g = arr[6].as_f64().unwrap_or(6.674e-11);

        let dx = x2 - x1;
        let dy = y2 - y1;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > 0.001 {
            let force = g * m1 * m2 / (dist * dist);
            let fx = force * dx / dist;
            let fy = force * dy / dist;

            Ok(json!([fx, fy, -fx, -fy, dist]))
        } else {
            Ok(json!([0.0, 0.0, 0.0, 0.0, dist]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_module_name() {
        let module = PhysicsModule;
        assert_eq!(module.name(), "physics");
        assert_eq!(module.version(), "0.7.3");
    }

    #[test]
    fn test_physics_register() {
        let module = PhysicsModule;
        let cmds = module.register();

        assert!(cmds.contains_key("projectile"));
        assert!(cmds.contains_key("nbody_2"));
    }

    #[test]
    fn test_projectile() {
        let module = PhysicsModule;
        // x0=0, y0=0, v0=10, angle=45
        let params = json!([0.0, 0.0, 10.0, 45.0]);
        let result = module.execute("projectile", params).unwrap();

        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 5);
        // flight_time ≈ 2*vy/g = 2*(10*sin(45))/9.81 ≈ 1.44s
        let flight_time = arr[2].as_f64().unwrap();
        assert!(flight_time > 1.4 && flight_time < 1.5);
    }

    #[test]
    fn test_nbody_2() {
        let module = PhysicsModule;
        // m1=100, m2=200, x1=0, y1=0, x2=10, y2=0, G=1.0
        let params = json!([100.0, 200.0, 0.0, 0.0, 10.0, 0.0, 1.0]);
        let result = module.execute("nbody_2", params).unwrap();

        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 5);
        // fx = G*m1*m2/dist^2 * dx/dist = 1*100*200/100 * 10/10 = 200
        let fx = arr[0].as_f64().unwrap();
        assert!((fx - 200.0).abs() < 0.01);
    }

    #[test]
    fn test_nbody_2_close() {
        let module = PhysicsModule;
        // Cuerpos muy cercanos (dist < 0.001)
        let params = json!([100.0, 200.0, 0.0, 0.0, 0.0001, 0.0, 1.0]);
        let result = module.execute("nbody_2", params).unwrap();

        let arr = result.as_array().unwrap();
        assert_eq!(arr[0].as_f64().unwrap(), 0.0);
        assert_eq!(arr[1].as_f64().unwrap(), 0.0);
    }

    #[test]
    fn test_unknown_command() {
        let module = PhysicsModule;
        let result = module.execute("unknown", json!([]));

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "UNKNOWN_COMMAND");
    }
}
