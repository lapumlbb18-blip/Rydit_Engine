//! Animaciones Científicas
//!
//! Generadores de animaciones basadas en fenómenos científicos reales:
//! química, biología, zoología, botánica, física histórica.
//!
//! ## Animaciones implementadas
//!
//! - Chemical Reactions (cristalización, crecimiento)
//! - Biological Cycles (división celular, curvas de crecimiento)
//! - Fauna Movement (ciclos de caminata, patrones de vuelo)
//! - Flora Growth (L-systems animados)
//! - Tusi Couple (pareja de Tusi - animación histórica)
//! - Pendulum Waves (ondas de péndulos)
//! - Wave Interference (interferencia de ondas)

use serde_json::{json, Value};

// ============================================================================
// CHEMICAL REACTIONS — Cristalización
// ============================================================================

/// Chemical Crystallization — Simula crecimiento de cristales
///
/// # Args
/// - cx, cy: centro de cristalización
/// - num_crystals: número de cristales (5-30)
/// - max_radius: radio máximo de crecimiento
/// - t: tiempo de animación (0.0-1.0)
/// - growth_rate: velocidad de crecimiento (0.5-3.0)
///
/// # Retorna
/// Array de cristales [{x, y, size, angle, alpha}, ...]
pub fn chemical_crystallization(cx: f64, cy: f64, num_crystals: usize,
                                 max_radius: f64, t: f64, growth_rate: f64) -> Vec<Value> {
    let num_crystals = num_crystals.clamp(5, 40);
    let t = t.clamp(0.0, 1.0);
    let growth_rate = growth_rate.clamp(0.3, 5.0);
    let mut result = Vec::new();

    for i in 0..num_crystals {
        let angle = (i as f64 / num_crystals as f64) * std::f64::consts::PI * 2.0 + i as f64 * 0.5;
        // Crecimiento no lineal — cristales externos crecen más lento
        let dist_factor = i as f64 / num_crystals as f64;
        let radius = max_radius * t.powf(growth_rate) * (1.0 - dist_factor * 0.5);
        let x = cx + angle.cos() * radius;
        let y = cy + angle.sin() * radius;
        let size = 3.0 + 8.0 * t.powf(growth_rate * 0.8);
        let alpha = (t * 2.0).min(1.0);

        result.push(json!({
            "type": "crystal",
            "x": x, "y": y,
            "size": size,
            "angle": angle,
            "alpha": alpha,
            "color": "#AADDFF"
        }));
    }

    result
}

// ============================================================================
// BIOLOGICAL CYCLES — División celular
// ============================================================================

/// Cell Division — Simula división celular con crecimiento
///
/// # Args
/// - cx, cy: centro
/// - initial_radius: radio de la célula inicial
/// - division_time: tiempo en el que ocurre la división (1.0 = primera división)
/// - max_divisions: número máximo de divisiones (1-6)
/// - t: tiempo de animación
///
/// # Retorna
/// Array de células [{x, y, radius, dividing, alpha}, ...]
pub fn cell_division(cx: f64, cy: f64, initial_radius: f64,
                     division_time: f64, max_divisions: usize, t: f64) -> Vec<Value> {
    let max_divisions = max_divisions.clamp(1, 8);
    let mut cells = vec![(cx, cy, initial_radius, 0usize)];

    for div in 0..max_divisions {
        let div_t = division_time * (div as f64 + 1.0);
        if t >= div_t {
            let progress = ((t - div_t) / division_time).min(1.0);
            let mut new_cells = Vec::new();

            for (ccx, ccy, r, _) in &cells {
                if progress > 0.5 {
                    // División completada — crear 2 células hijas
                    let offset = r * 0.3 * (progress - 0.5) * 2.0;
                    let new_r = r * 0.7;
                    new_cells.push((*ccx - offset, *ccy, new_r, div + 1));
                    new_cells.push((*ccx + offset, *ccy, new_r, div + 1));
                } else {
                    // Célula creciendo antes de dividirse
                    let new_r = r * (1.0 + progress * 0.5);
                    new_cells.push((*ccx, *ccy, new_r, div));
                }
            }

            cells = new_cells;
        }
    }

    cells.iter().map(|(x, y, r, _)| json!({
        "type": "cell",
        "x": x, "y": y,
        "radius": r,
        "alpha": 0.8,
        "color": "#44DD88"
    })).collect()
}

// ============================================================================
// FAUNA MOVEMENT — Ciclo de caminata (walk cycle)
// ============================================================================

/// Walk Cycle — Genera posiciones de patas para ciclo de caminata
///
/// # Args
/// - cx, cy: posición del cuerpo
/// - body_size: tamaño del cuerpo
/// - num_legs: número de patas (2, 4, 6, 8)
/// - stride: largo del paso
/// - t: tiempo del ciclo (0.0-1.0 = un paso completo)
/// - leg_phase_offset: desfase entre patas (0.0-1.0)
///
/// # Retorna
/// Array de puntos [{type: "body" | "leg", x, y, angle}, ...]
pub fn walk_cycle(cx: f64, cy: f64, body_size: f64, num_legs: usize,
                  stride: f64, t: f64, leg_phase_offset: f64) -> Vec<Value> {
    let num_legs = num_legs.clamp(2, 8);
    let t = t % 1.0;
    let mut result = Vec::new();

    // Cuerpo con ligero balanceo
    let body_y = cy + 3.0 * (t * std::f64::consts::PI * 4.0).sin();
    result.push(json!({
        "type": "body",
        "x": cx, "y": body_y,
        "size": body_size,
        "color": "#CC8844"
    }));

    // Patas
    for i in 0..num_legs {
        let phase = t + (i as f64 / num_legs as f64) * leg_phase_offset;
        let leg_angle = (phase * std::f64::consts::PI * 2.0).sin() * stride;
        let _side = if i < num_legs / 2 { 1.0 } else { -1.0 };
        let leg_idx = i % (num_legs / 2);
        let leg_x = cx + (leg_idx as f64 - (num_legs / 4) as f64) * body_size * 0.4;

        let foot_x = leg_x + leg_angle;
        let foot_y = body_y + body_size + leg_angle.abs() * 0.3;

        result.push(json!({
            "type": "leg",
            "x1": leg_x, "y1": body_y,
            "x2": foot_x, "y2": foot_y,
            "color": "#AA6633"
        }));
    }

    result
}

// ============================================================================
// FAUNA MOVEMENT — Patrón de vuelo (flapping wings)
// ============================================================================

/// Flight Pattern — Simula aleteo de aves
///
/// # Args
/// - cx, cy: posición del cuerpo
/// - wingspan: envergadura
/// - flap_speed: velocidad del aleteo
/// - t: tiempo
///
/// # Retorna
/// Array de líneas [{type: "body" | "wing", x1, y1, x2, y2}, ...]
pub fn flight_pattern(cx: f64, cy: f64, wingspan: f64, flap_speed: f64, t: f64) -> Vec<Value> {
    let flap = (t * flap_speed * std::f64::consts::PI * 2.0).sin();
    let wing_angle = flap * 0.6; // ±60 grados

    let mut result = Vec::new();

    // Cuerpo
    result.push(json!({
        "type": "body",
        "x": cx, "y": cy,
        "size": 10.0,
        "color": "#4488CC"
    }));

    // Ala izquierda
    let left_wing_x = cx - wingspan / 2.0 * wing_angle.cos();
    let left_wing_y = cy - wingspan / 2.0 * wing_angle.sin();
    result.push(json!({
        "type": "wing",
        "x1": cx, "y1": cy,
        "x2": left_wing_x, "y2": left_wing_y,
        "color": "#66AADD"
    }));

    // Ala derecha
    let right_wing_x = cx + wingspan / 2.0 * wing_angle.cos();
    let right_wing_y = cy - wingspan / 2.0 * wing_angle.sin();
    result.push(json!({
        "type": "wing",
        "x1": cx, "y1": cy,
        "x2": right_wing_x, "y2": right_wing_y,
        "color": "#66AADD"
    }));

    result
}

// ============================================================================
// FLORA GROWTH — L-System animado (árbol simple)
// ============================================================================

/// L-System Tree — Árbol fractal animado
///
/// # Args
/// - base_x, base_y: base del tronco
/// - trunk_length: largo del tronco
/// - branch_angle: ángulo de ramificación (radianes)
/// - length_ratio: ratio de reducción por nivel (0.6-0.8)
/// - max_depth: profundidad máxima de recursión (3-8)
/// - t: tiempo de animación (0.0 = solo tronco, 1.0 = árbol completo)
///
/// # Retorna
/// Array de segmentos [{x1, y1, x2, y2, depth, color}, ...]
pub fn lsystem_tree(base_x: f64, base_y: f64, trunk_length: f64,
                     branch_angle: f64, length_ratio: f64, max_depth: usize, t: f64) -> Vec<Value> {
    let max_depth = max_depth.clamp(2, 10);
    let t = t.clamp(0.0, 1.0);
    let mut result = Vec::new();

    fn grow_branch(x: f64, y: f64, angle: f64, length: f64,
                   depth: usize, max_depth: usize, t: f64,
                   branch_angle: f64, length_ratio: f64,
                   result: &mut Vec<Value>) {
        if depth == 0 || t < (depth as f64 / max_depth as f64) {
            return;
        }

        let progress = ((t - (depth as f64 / max_depth as f64)) * max_depth as f64).min(1.0);
        let current_length = length * progress;

        let end_x = x + angle.cos() * current_length;
        let end_y = y + angle.sin() * current_length;

        // Color: tronco marrón, ramas verdes
        let color = if depth > max_depth / 2 { "#8B4513" } else { "#228B22" };

        result.push(json!({
            "type": "branch",
            "x1": x, "y1": y,
            "x2": end_x, "y2": end_y,
            "depth": depth,
            "color": color
        }));

        if depth > 1 {
            grow_branch(end_x, end_y, angle - branch_angle,
                       length * length_ratio, depth - 1, max_depth, t,
                       branch_angle, length_ratio, result);
            grow_branch(end_x, end_y, angle + branch_angle,
                       length * length_ratio, depth - 1, max_depth, t,
                       branch_angle, length_ratio, result);
        }
    }

    grow_branch(base_x, base_y, -std::f64::consts::PI / 2.0,
               trunk_length, max_depth, max_depth, t, branch_angle, length_ratio, &mut result);

    result
}

// ============================================================================
// TUSI COUPLE — Pareja de Tusi (animación histórica)
// ============================================================================

/// Tusi Couple — Círculo pequeño rodando dentro de uno grande
///
/// Genera movimiento lineal a partir de movimiento circular.
/// Inventado por Nasir al-Din al-Tusi (~1250 d.C.)
///
/// # Args
/// - cx, cy: centro
/// - large_radius: radio del círculo grande
/// - t: tiempo de animación
///
/// # Retorna
/// Array de elementos [{type: "large_circle" | "small_circle" | "point" | "trace", ...}, ...]
pub fn tusi_couple(cx: f64, cy: f64, large_radius: f64, t: f64) -> Vec<Value> {
    let small_radius = large_radius / 2.0;
    let mut result = Vec::new();

    // Círculo grande
    result.push(json!({
        "type": "large_circle",
        "x": cx, "y": cy,
        "radius": large_radius,
        "color": "#444466"
    }));

    // Centro del círculo pequeño (orbitando)
    let small_cx = cx + small_radius * (t * 2.0).cos();
    let small_cy = cy + small_radius * (t * 2.0).sin();

    // Círculo pequeño
    result.push(json!({
        "type": "small_circle",
        "x": small_cx, "y": small_cy,
        "radius": small_radius,
        "color": "#666688"
    }));

    // Punto en el borde del círculo pequeño (se mueve linealmente!)
    let point_x = small_cx + small_radius * (-t * 2.0).cos();
    let point_y = small_cy + small_radius * (-t * 2.0).sin();

    result.push(json!({
        "type": "point",
        "x": point_x, "y": point_y,
        "size": 6.0,
        "color": "#FF4444"
    }));

    // Línea de traza (movimiento lineal)
    result.push(json!({
        "type": "trace_line",
        "x1": cx - large_radius, "y1": cy,
        "x2": cx + large_radius, "y2": cy,
        "color": "#FF8888"
    }));

    result
}

// ============================================================================
// PENDULUM WAVES — Ondas de péndulos
// ============================================================================

/// Pendulum Waves — Múltiples péndulos con frecuencias ligeramente distintas
///
/// # Args
/// - base_x, base_y: línea de soporte
/// - num_pendulums: número de péndulos (8-20)
/// - pendulum_length: largo de cada péndulo
/// - freq_spread: diferencia de frecuencia entre péndulos
/// - t: tiempo
///
/// # Retorna
/// Array de péndulos [{x1, y1, x2, y2, bob_x, bob_y}, ...]
pub fn pendulum_waves(base_x: f64, base_y: f64, num_pendulums: usize,
                      pendulum_length: f64, freq_spread: f64, t: f64) -> Vec<Value> {
    let num_pendulums = num_pendulums.clamp(6, 30);
    let mut result = Vec::new();
    let spacing = (base_x * 2.0 / num_pendulums as f64).min(40.0);

    for i in 0..num_pendulums {
        let px = base_x + i as f64 * spacing - (num_pendulums as f64 * spacing / 2.0);
        let freq = 1.0 + i as f64 * freq_spread;
        let angle = 0.5 * (t * freq * std::f64::consts::PI * 2.0).sin();

        let bob_x = px + pendulum_length * angle.sin();
        let bob_y = base_y + pendulum_length * angle.cos();

        // Color basado en la posición (efecto arcoíris)
        let hue = (i as f64 / num_pendulums as f64 + t * 0.1) % 1.0;
        let color = format!("hsl({}, 80%, 60%)", (hue * 360.0) as usize);

        result.push(json!({
            "type": "pendulum",
            "x1": px, "y1": base_y,
            "x2": bob_x, "y2": bob_y,
            "bob_x": bob_x, "bob_y": bob_y,
            "color": color
        }));
    }

    result
}

// ============================================================================
// WAVE INTERFERENCE — Interferencia de ondas
// ============================================================================

/// Wave Interference — Dos fuentes de ondas interfiriéndose
///
/// # Args
/// - cx1, cy1: centro de fuente 1
/// - cx2, cy2: centro de fuente 2
/// - wavelength: longitud de onda
/// - amplitude: amplitud
/// - grid_resolution: resolución de la cuadrícula (10-50)
/// - t: tiempo
///
/// # Retorna
/// Array de puntos de la cuadrícula [{x, y, amplitude, color}, ...]
pub fn wave_interference(cx1: f64, cy1: f64, cx2: f64, cy2: f64,
                           wavelength: f64, amplitude: f64,
                           grid_resolution: usize, t: f64) -> Vec<Value> {
    let grid_resolution = grid_resolution.clamp(8, 60);
    let mut result = Vec::new();

    let width = 600.0;
    let height = 400.0;
    let start_x = 100.0;
    let start_y = 100.0;
    let step_x = width / grid_resolution as f64;
    let step_y = height / grid_resolution as f64;

    for iy in 0..=grid_resolution {
        for ix in 0..=grid_resolution {
            let x = start_x + ix as f64 * step_x;
            let y = start_y + iy as f64 * step_y;

            // Distancia a cada fuente
            let d1 = ((x - cx1) * (x - cx1) + (y - cy1) * (y - cy1)).sqrt();
            let d2 = ((x - cx2) * (x - cx2) + (y - cy2) * (y - cy2)).sqrt();

            // Ondas individuales
            let w1 = amplitude * (d1 * std::f64::consts::PI * 2.0 / wavelength - t * 3.0).sin();
            let w2 = amplitude * (d2 * std::f64::consts::PI * 2.0 / wavelength - t * 3.0).sin();

            // Interferencia
            let combined = (w1 + w2) / 2.0;
            let normalized = (combined / amplitude + 1.0) / 2.0; // 0.0-1.0

            // Color: azul (destructiva) → blanco → rojo (constructiva)
            let color = if normalized < 0.5 {
                let t = normalized * 2.0;
                format!("rgb({}, {}, {})", (t * 100.0) as u8, (t * 100.0) as u8, 255)
            } else {
                let t = (normalized - 0.5) * 2.0;
                format!("rgb(255, {}, {})", (255.0 - t * 155.0) as u8, (255.0 - t * 200.0) as u8)
            };

            result.push(json!({
                "type": "wave_point",
                "x": x, "y": y,
                "amplitude": combined,
                "color": color
            }));
        }
    }

    // Fuentes
    result.push(json!({ "type": "source", "x": cx1, "y": cy1, "color": "#00FFFF" }));
    result.push(json!({ "type": "source", "x": cx2, "y": cy2, "color": "#FF00FF" }));

    result
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chemical_crystallization() {
        let result = chemical_crystallization(400.0, 300.0, 12, 100.0, 0.5, 1.5);
        assert_eq!(result.len(), 12);
        assert!(result[0].get("type").is_some());
    }

    #[test]
    fn test_cell_division() {
        let result = cell_division(400.0, 300.0, 30.0, 1.0, 3, 2.5);
        assert!(result.len() >= 2); // Al menos 2 células después de dos divisiones
    }

    #[test]
    fn test_walk_cycle() {
        let result = walk_cycle(400.0, 300.0, 20.0, 4, 15.0, 0.3, 0.25);
        assert!(result.len() > 4); // Cuerpo + 4 patas
    }

    #[test]
    fn test_flight_pattern() {
        let result = flight_pattern(400.0, 300.0, 80.0, 5.0, 0.25);
        assert_eq!(result.len(), 3); // Cuerpo + 2 alas
    }

    #[test]
    fn test_lsystem_tree() {
        let result = lsystem_tree(400.0, 500.0, 80.0, 0.5, 0.7, 4, 1.0);
        assert!(!result.is_empty());
        assert!(result.len() >= 3); // Tronco + al menos 2 ramas
    }

    #[test]
    fn test_tusi_couple() {
        let result = tusi_couple(400.0, 300.0, 100.0, 0.5);
        assert!(result.len() >= 4); // large_circle, small_circle, point, trace_line
    }

    #[test]
    fn test_pendulum_waves() {
        let result = pendulum_waves(400.0, 100.0, 12, 100.0, 0.05, 0.5);
        assert_eq!(result.len(), 12);
    }

    #[test]
    fn test_wave_interference() {
        let result = wave_interference(250.0, 300.0, 550.0, 300.0, 40.0, 1.0, 15, 0.5);
        assert!(!result.is_empty());
        // Debe tener puntos de onda + 2 fuentes
        assert!(result.len() > 2);
    }
}
