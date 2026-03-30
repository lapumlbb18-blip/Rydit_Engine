// crates/rydit-rs/src/lazos.rs
// Protocolo LAZOS - Sistema universal de comunicación

use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

use crate::get_loader;

/// Loop principal del Protocolo LAZOS
/// Lee comandos JSON desde stdin, ejecuta, responde por stdout
pub fn lazos_loop() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    eprintln!("[LAZOS] Protocolo iniciado - esperando comandos...");
    eprintln!("[LAZOS] Modo: stdin/stdout JSON-RPC");
    eprintln!("[LAZOS] Presiona Ctrl+D para salir");

    for line in stdin.lock().lines().map_while(Result::ok) {
        // Parsear request JSON
        let request: Value = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(e) => {
                let error = json!({
                    "jsonrpc": "2.0",
                    "error": format!("Invalid JSON: {e}"),
                    "id": null
                });
                println!("{error}");
                continue;
            }
        };

        // Ejecutar comando
        let response = ejecutar_comando_lazos(&request);

        // Responder
        writeln!(stdout, "{}", response).unwrap();
        stdout.flush().unwrap();
    }

    eprintln!("[LAZOS] Protocolo finalizado");
}

/// Ejecutar un comando LAZOS
fn ejecutar_comando_lazos(request: &Value) -> Value {
    let method = request["method"].as_str().unwrap_or("");
    let empty_array = vec![];
    let params = request["params"].as_array().unwrap_or(&empty_array);
    let id = request["id"].clone();

    // Routing de comandos
    let result = match method {
        // === SYSTEM ===
        "system::version" => json!("v0.7.3-lazos"),
        "system::ping" => json!("pong"),
        "system::info" => {
            json!({
                "name": "RyDit Engine",
                "version": "v0.8.2-sistema-universal",
                "protocol": "LAZOS v1.0",
                "commands": [
                    "system::version", "system::ping", "system::info",
                    "science::bezier::linear", "science::bezier::quadratic", "science::bezier::cubic",
                    "science::stats::mean", "science::stats::median", "science::stats::min", "science::stats::max",
                    "physics::projectile", "physics::nbody_2",
                    "anim::ease_in", "anim::ease_out", "anim::ease_in_out",
                    "anim::squash", "anim::stretch", "anim::anticipate",
                    "module::list", "module::info"
                ]
            })
        }

        // === BEZIER ===
        "science::bezier::linear" => bezier_linear(params),
        "science::bezier::quadratic" => bezier_quadratic(params),
        "science::bezier::cubic" => bezier_cubic(params),

        // === ESTADÍSTICAS ===
        "science::stats::mean" => stats_mean(params),
        "science::stats::median" => stats_median(params),
        "science::stats::min" => stats_min(params),
        "science::stats::max" => stats_max(params),

        // === FÍSICA ===
        "physics::projectile" => physics_projectile(params),
        "physics::nbody_2" => physics_nbody_2(params),

        // === ANIMACIÓN ===
        "anim::ease_in" => anim_ease_in(params),
        "anim::ease_out" => anim_ease_out(params),
        "anim::ease_in_out" => anim_ease_in_out(params),
        "anim::squash" => anim_squash(params),
        "anim::stretch" => anim_stretch(params),
        "anim::anticipate" => anim_anticipate(params),

        // === MÓDULOS DINÁMICOS (v0.8.2) ===
        "module::list" => {
            if let Some(loader_mutex) = get_loader() {
                let loader = loader_mutex.lock().unwrap();
                let modules = loader.list_modules();
                json!({"modules": modules, "count": modules.len()})
            } else {
                json!({"error": "Loader not initialized"})
            }
        }
        "module::info" => {
            if params.is_empty() {
                json!({"error": "module::info requires module name"})
            } else if let Some(module_name) = params[0].as_str() {
                if let Some(loader_mutex) = get_loader() {
                    let loader = loader_mutex.lock().unwrap();
                    if let Some(info) = loader.get_module_info(module_name) {
                        json!({
                            "name": info.name,
                            "version": info.metadata.version,
                            "path": info.path,
                            "loaded_at": info.loaded_at
                        })
                    } else {
                        json!({"error": format!("Module '{}' not found", module_name)})
                    }
                } else {
                    json!({"error": "Loader not initialized"})
                }
            } else {
                json!({"error": "Invalid module name"})
            }
        }

        // === ERROR ===
        _ => json!({"error": format!("Unknown method: {}", method)}),
    };

    json!({
        "jsonrpc": "2.0",
        "result": result,
        "id": id
    })
}

// ============================================================================
// COMANDOS DE BEZIER
// ============================================================================

fn bezier_linear(params: &[Value]) -> Value {
    if params.len() != 5 {
        return json!({"error": "bezier::linear requires 5 params: p0_x, p0_y, p1_x, p1_y, t"});
    }

    let p0_x = params[0].as_f64().unwrap_or(0.0);
    let p0_y = params[1].as_f64().unwrap_or(0.0);
    let p1_x = params[2].as_f64().unwrap_or(0.0);
    let p1_y = params[3].as_f64().unwrap_or(0.0);
    let t = params[4].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);

    let x = (1.0 - t) * p0_x + t * p1_x;
    let y = (1.0 - t) * p0_y + t * p1_y;

    json!([x, y])
}

fn bezier_quadratic(params: &[Value]) -> Value {
    if params.len() != 7 {
        return json!({"error": "bezier::quadratic requires 7 params: p0_x, p0_y, p1_x, p1_y, p2_x, p2_y, t"});
    }

    let p0_x = params[0].as_f64().unwrap_or(0.0);
    let p0_y = params[1].as_f64().unwrap_or(0.0);
    let p1_x = params[2].as_f64().unwrap_or(0.0);
    let p1_y = params[3].as_f64().unwrap_or(0.0);
    let p2_x = params[4].as_f64().unwrap_or(0.0);
    let p2_y = params[5].as_f64().unwrap_or(0.0);
    let t = params[6].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);

    let mt = 1.0 - t;
    let x = mt * mt * p0_x + 2.0 * mt * t * p1_x + t * t * p2_x;
    let y = mt * mt * p0_y + 2.0 * mt * t * p1_y + t * t * p2_y;

    json!([x, y])
}

fn bezier_cubic(params: &[Value]) -> Value {
    if params.len() != 9 {
        return json!({"error": "bezier::cubic requires 9 params: p0_x, p0_y, p1_x, p1_y, p2_x, p2_y, p3_x, p3_y, t"});
    }

    let p0_x = params[0].as_f64().unwrap_or(0.0);
    let p0_y = params[1].as_f64().unwrap_or(0.0);
    let p1_x = params[2].as_f64().unwrap_or(0.0);
    let p1_y = params[3].as_f64().unwrap_or(0.0);
    let p2_x = params[4].as_f64().unwrap_or(0.0);
    let p2_y = params[5].as_f64().unwrap_or(0.0);
    let p3_x = params[6].as_f64().unwrap_or(0.0);
    let p3_y = params[7].as_f64().unwrap_or(0.0);
    let t = params[8].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);

    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let t2 = t * t;

    let x = mt2 * mt * p0_x + 3.0 * mt2 * t * p1_x + 3.0 * mt * t2 * p2_x + t2 * t * p3_x;
    let y = mt2 * mt * p0_y + 3.0 * mt2 * t * p1_y + 3.0 * mt * t2 * p2_y + t2 * t * p3_y;

    json!([x, y])
}

// ============================================================================
// COMANDOS DE FÍSICA
// ============================================================================

fn physics_projectile(params: &[Value]) -> Value {
    if params.len() != 4 {
        return json!({"error": "physics::projectile requires 4 params: x0, y0, v0, angle"});
    }

    let x0 = params[0].as_f64().unwrap_or(0.0);
    let y0 = params[1].as_f64().unwrap_or(0.0);
    let v0 = params[2].as_f64().unwrap_or(0.0);
    let angle = params[3].as_f64().unwrap_or(0.0);

    let rad = angle.to_radians();
    let vx = v0 * rad.cos();
    let vy = v0 * rad.sin();
    let g = 9.81;

    let flight_time = 2.0 * vy / g;
    let max_height = (vy * vy) / (2.0 * g);
    let range = vx * flight_time;

    json!([
        x0 + vx * flight_time, // x final
        y0,                    // y final
        flight_time,           // tiempo vuelo
        max_height,            // altura máxima
        range                  // alcance horizontal
    ])
}

fn physics_nbody_2(params: &[Value]) -> Value {
    if params.len() != 7 {
        return json!({"error": "physics::nbody_2 requires 7 params: m1, m2, x1, y1, x2, y2, G"});
    }

    let m1 = params[0].as_f64().unwrap_or(0.0);
    let m2 = params[1].as_f64().unwrap_or(0.0);
    let x1 = params[2].as_f64().unwrap_or(0.0);
    let y1 = params[3].as_f64().unwrap_or(0.0);
    let x2 = params[4].as_f64().unwrap_or(0.0);
    let y2 = params[5].as_f64().unwrap_or(0.0);
    let g = params[6].as_f64().unwrap_or(6.674e-11);

    let dx = x2 - x1;
    let dy = y2 - y1;
    let dist = (dx * dx + dy * dy).sqrt();

    if dist > 0.001 {
        let force = g * m1 * m2 / (dist * dist);
        let fx = force * dx / dist;
        let fy = force * dy / dist;

        json!([fx, fy, -fx, -fy, dist])
    } else {
        json!([0.0, 0.0, 0.0, 0.0, dist])
    }
}

// ============================================================================
// COMANDOS DE ESTADÍSTICAS
// ============================================================================

fn stats_mean(params: &[Value]) -> Value {
    if params.len() != 1 {
        return json!({"error": "stats::mean requires 1 param: [array]"});
    }

    let empty_array = vec![];
    let arr = params[0].as_array().unwrap_or(&empty_array);

    if arr.is_empty() {
        return json!({"error": "Empty array"});
    }

    let sum: f64 = arr.iter().filter_map(|v| v.as_f64()).sum();

    json!(sum / arr.len() as f64)
}

fn stats_median(params: &[Value]) -> Value {
    if params.len() != 1 {
        return json!({"error": "stats::median requires 1 param: [array]"});
    }

    let empty_array = vec![];
    let arr = params[0].as_array().unwrap_or(&empty_array);
    let mut nums: Vec<f64> = arr.iter().filter_map(|v| v.as_f64()).collect();

    if nums.is_empty() {
        return json!({"error": "Empty array or no numbers"});
    }

    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = nums.len() / 2;

    let median = if nums.len().is_multiple_of(2) {
        (nums[mid - 1] + nums[mid]) / 2.0
    } else {
        nums[mid]
    };

    json!(median)
}

fn stats_min(params: &[Value]) -> Value {
    if params.len() != 1 {
        return json!({"error": "stats::min requires 1 param: [array]"});
    }

    let empty_array = vec![];
    let arr = params[0].as_array().unwrap_or(&empty_array);
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
        json!(min_val)
    } else {
        json!({"error": "No numbers in array"})
    }
}

fn stats_max(params: &[Value]) -> Value {
    if params.len() != 1 {
        return json!({"error": "stats::max requires 1 param: [array]"});
    }

    let empty_array = vec![];
    let arr = params[0].as_array().unwrap_or(&empty_array);
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
        json!(max_val)
    } else {
        json!({"error": "No numbers in array"})
    }
}

// ============================================================================
// COMANDOS DE ANIMACIÓN (v0.7.3)
// ============================================================================

fn anim_ease_in(params: &[Value]) -> Value {
    if params.len() != 1 {
        return json!({"error": "anim::ease_in requires 1 param: t (0.0-1.0)"});
    }

    let t = params[0].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);
    json!(t * t)
}

fn anim_ease_out(params: &[Value]) -> Value {
    if params.len() != 1 {
        return json!({"error": "anim::ease_out requires 1 param: t (0.0-1.0)"});
    }

    let t = params[0].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);
    json!(t * (2.0 - t))
}

fn anim_ease_in_out(params: &[Value]) -> Value {
    if params.len() != 1 {
        return json!({"error": "anim::ease_in_out requires 1 param: t (0.0-1.0)"});
    }

    let t = params[0].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);
    let result = if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - 2.0 * (1.0 - t) * (1.0 - t)
    };
    json!(result)
}

fn anim_squash(params: &[Value]) -> Value {
    if params.len() != 1 {
        return json!({"error": "anim::squash requires 1 param: factor (0.5-2.0)"});
    }

    let factor = params[0].as_f64().unwrap_or(1.0).clamp(0.5, 2.0);
    json!([factor, 1.0 / factor])
}

fn anim_stretch(params: &[Value]) -> Value {
    if params.len() != 1 {
        return json!({"error": "anim::stretch requires 1 param: factor (0.5-2.0)"});
    }

    let factor = params[0].as_f64().unwrap_or(1.0).clamp(0.5, 2.0);
    json!([1.0 / factor, factor])
}

fn anim_anticipate(params: &[Value]) -> Value {
    if params.len() != 3 {
        return json!({"error": "anim::anticipate requires 3 params: pos, target, amount"});
    }

    let pos = params[0].as_f64().unwrap_or(0.0);
    let target = params[1].as_f64().unwrap_or(0.0);
    let amount = params[2].as_f64().unwrap_or(0.0);

    let dir = if target > pos { -1.0 } else { 1.0 };
    json!(pos + dir * amount)
}
