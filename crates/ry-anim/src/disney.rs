//! Principios de Animación Disney — Implementación en Rust
//!
//! 12 principios clásicos de animación de Disney, implementados como
//! funciones matemáticas puras para uso en Ry-Dit.
//!
//! ## Principios implementados
//!
//! ✅ #1 Squash & Stretch (ya existe en AnimModule)
//! ✅ #2 Anticipation (ya existe en AnimModule)
//! ✅ #6 Slow In & Slow Out (ya existe como easing)
//! 🔄 #3 Staging — Presentación clara de una idea
//! 🔄 #4 Follow Through & Overlapping Action
//! 🔄 #5 Straight Ahead vs Pose-to-Pose
//! 🔄 #7 Arcs
//! 🔄 #8 Secondary Action
//! 🔄 #9 Timing
//! 🔄 #10 Exaggeration
//! 🔄 #11 Solid Drawing
//! 🔄 #12 Appeal

use serde_json::{json, Value};

// ============================================================================
// PRINCIPIO #4: FOLLOW THROUGH & OVERLAPPING ACTION
// ============================================================================

/// Follow Through — Partes del cuerpo continúan moviéndose después de parar
///
/// Cuando un objeto se detiene, sus partes "sueltas" continúan por inercia
/// y luego regresan. Simula el efecto de "cola" o "orejitas" que siguen
/// moviéndose.
///
/// # Fórmula
/// `offset(t) = amplitude * e^(-decay * t) * sin(frequency * t)`
///
/// # Args
/// - amplitude: cuánto se mueve la parte suelta
/// - decay: qué tan rápido se detiene (0.1-2.0)
/// - frequency: oscilaciones (1.0-10.0)
/// - t: tiempo normalizado 0.0-1.0
///
/// # Retorna
/// offset actual de la parte que sigue moviéndose
pub fn follow_through(amplitude: f64, decay: f64, frequency: f64, t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    amplitude * (-decay * t).exp() * (frequency * t).sin()
}

/// Overlapping Action — Diferentes partes se mueven a diferentes velocidades
///
/// Cuando un personaje se mueve, no todo se mueve al mismo tiempo.
/// El cuerpo principal lidera, luego los brazos, luego la cabeza, etc.
///
/// # Args
/// - base_value: valor base del movimiento principal
/// - offsets: array de [offset_time, amplitude] por cada parte
/// - t: tiempo normalizado 0.0-1.0
///
/// # Retorna
/// Array de valores actuales para cada parte [base, part1, part2, ...]
pub fn overlapping_action(base_value: f64, offsets: &[(f64, f64)], t: f64) -> Vec<f64> {
    let t = t.clamp(0.0, 1.0);
    let mut result = vec![base_value]; // Valor base (movimiento principal)

    for (offset_t, amplitude) in offsets {
        let delayed_t = (t - offset_t).max(0.0); // Retraso temporal
        let eased = ease_in_out(delayed_t);
        result.push(base_value + amplitude * eased);
    }

    result
}

// ============================================================================
// PRINCIPIO #7: ARCS
// ============================================================================

/// Arc Path — Trayectoria curva entre dos puntos
///
/// Los movimientos naturales siguen arcos, no líneas rectas.
/// Este genera un punto en un arco entre start y end con curvatura controlable.
///
/// # Fórmula
/// Punto en arco cuadrático: P(t) = (1-t)²·start + 2(1-t)t·control + t²·end
///
/// # Args
/// - start: punto inicial (x, y)
/// - end: punto final (x, y)
/// - curvature: control point offset (cuánto se curva el arco)
/// - t: tiempo normalizado 0.0-1.0
///
/// # Retorna
/// [x, y] posición actual en el arco
pub fn arc_path(start: (f64, f64), end: (f64, f64), curvature: f64, t: f64) -> (f64, f64) {
    let t = t.clamp(0.0, 1.0);

    // Control point: punto medio + offset de curvatura perpendicular
    let mid_x = (start.0 + end.0) / 2.0;
    let mid_y = (start.1 + end.1) / 2.0;

    // Vector perpendicular
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    let perp_x = -dy / len * curvature;
    let perp_y = dx / len * curvature;

    let control_x = mid_x + perp_x;
    let control_y = mid_y + perp_y;

    // Bézier cuadrática
    let mt = 1.0 - t;
    let x = mt * mt * start.0 + 2.0 * mt * t * control_x + t * t * end.0;
    let y = mt * mt * start.1 + 2.0 * mt * t * control_y + t * t * end.1;

    (x, y)
}

// ============================================================================
// PRINCIPIO #8: SECONDARY ACTION
// ============================================================================

/// Secondary Action — Movimiento secundario que refuerza el principal
///
/// Mientras un personaje camina (acción principal), puede silbar, mover
/// los brazos, etc. (acción secundaria). La acción secundaria tiene
/// su propio timing y no debe distraer de la principal.
///
/// # Args
/// - primary: valor de la acción principal
/// - secondary_offset: desfase temporal (0.0-0.5 recomendado)
/// - secondary_amplitude: intensidad de la acción secundaria
/// - t: tiempo normalizado 0.0-1.0
///
/// # Retorna
/// [primary_value, secondary_value]
pub fn secondary_action(
    primary: f64,
    secondary_offset: f64,
    secondary_amplitude: f64,
    t: f64,
) -> (f64, f64) {
    let t = t.clamp(0.0, 1.0);
    let secondary_t = (t - secondary_offset).max(0.0);
    let eased = ease_in_out(secondary_t);
    let secondary = primary + secondary_amplitude * eased;

    (primary, secondary)
}

// ============================================================================
// PRINCIPIO #9: TIMING
// ============================================================================

/// Timing — Número de frames determina la velocidad percibida
///
/// El timing controla cuántos frames se usan para una acción,
/// determinando si parece rápida o lenta.
///
/// # Args
/// - duration: duración total en frames
/// - keyframes: array de [(frame, value), ...]
/// - current_frame: frame actual
///
/// # Retorna
/// valor interpolado en el frame actual
pub fn timing(keyframes: &[(f64, f64)], current_frame: f64) -> f64 {
    if keyframes.is_empty() {
        return 0.0;
    }

    // Si estamos antes del primer keyframe
    if current_frame <= keyframes[0].0 {
        return keyframes[0].1;
    }

    // Si estamos después del último keyframe
    if current_frame >= keyframes.last().unwrap().0 {
        return keyframes.last().unwrap().1;
    }

    // Buscar entre qué keyframes estamos e interpolar
    for i in 0..keyframes.len() - 1 {
        let (f1, v1) = keyframes[i];
        let (f2, v2) = keyframes[i + 1];

        if current_frame >= f1 && current_frame <= f2 {
            let t = (current_frame - f1) / (f2 - f1);
            let eased_t = ease_in_out(t);
            return v1 + (v2 - v1) * eased_t;
        }
    }

    keyframes.last().unwrap().1
}

// ============================================================================
// PRINCIPIO #10: EXAGGERATION
// ============================================================================

/// Exaggeration — Exagerar movimientos para mayor claridad visual
///
/// En animación, los movimientos sutiles se pierden. Se exageran
/// para que sean claros y entretenidos.
///
/// # Args
/// - base_value: valor original del movimiento
/// - factor: cuánto exagerar (1.0 = normal, 2.0 = doble, 0.5 = mitad)
/// - t: tiempo normalizado 0.0-1.0 (para aplicar easing)
///
/// # Retorna
/// valor exagerado
pub fn exaggerate(base_value: f64, factor: f64, t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    let eased = ease_in_out(t);
    base_value * factor * eased
}

// ============================================================================
// PRINCIPIO #11: SOLID DRAWING
// ============================================================================

/// Solid Drawing — Rotación 3D con perspectiva correcta
///
/// Genera coordenadas 2D proyectadas desde rotación 3D,
/// manteniendo la sensación de volumen y peso.
///
/// # Args
/// - point: punto 3D (x, y, z)
/// - rotation: rotación (rx, ry, rz) en radianes
/// - fov: campo de visión (default 60°)
///
/// # Retorna
/// [screen_x, screen_y, scale_factor]
pub fn solid_rotation(
    point: (f64, f64, f64),
    rotation: (f64, f64, f64),
    fov: f64,
) -> (f64, f64, f64) {
    let (rx, ry, rz) = rotation;
    let (x, y, z) = point;

    // Rotación Y
    let cos_y = ry.cos();
    let sin_y = ry.sin();
    let x1 = x * cos_y - z * sin_y;
    let z1 = x * sin_y + z * cos_y;

    // Rotación X
    let cos_x = rx.cos();
    let sin_x = rx.sin();
    let y1 = y * cos_x - z1 * sin_x;
    let z2 = y * sin_x + z1 * cos_x;

    // Rotación Z
    let cos_z = rz.cos();
    let sin_z = rz.sin();
    let x2 = x1 * cos_z - y1 * sin_z;
    let y2 = x1 * sin_z + y1 * cos_z;

    // Proyección perspectiva
    let fov_rad = fov.to_radians();
    let scale = 1.0 / (fov_rad * z2).max(0.1);

    (x2 * scale, y2 * scale, scale)
}

// ============================================================================
// PRINCIPIO #12: APPEAL
// ============================================================================

/// Appeal — Hacer un diseño más atractivo visualmente
///
/// Aplica transformaciones que hacen una forma más "appealing":
/// - Proporción áurea en dimensiones
/// - Suavizado de curvas
/// - Asimetría controlada
///
/// # Args
/// - base_shape: dimensiones base [width, height]
/// - charm_factor: cuánto "charm" aplicar (0.0-1.0)
/// - t: tiempo para animación
///
/// # Retorna
/// [appealing_width, appealing_height, rotation]
pub fn appeal(base_shape: (f64, f64), charm_factor: f64, t: f64) -> (f64, f64, f64) {
    let (w, h) = base_shape;
    let t = t.clamp(0.0, 1.0);

    // Proporción áurea (1.618)
    let golden = 1.618033988749;
    let target_ratio = golden;
    let current_ratio = w / h.max(0.001);

    // Interpolar hacia proporción áurea
    let appeal_ratio = current_ratio + (target_ratio - current_ratio) * charm_factor * t;

    let new_w = w * (1.0 + 0.05 * charm_factor * t);
    let new_h = new_w / appeal_ratio;

    // Leve rotación para dinamismo
    let rotation = charm_factor * 3.0 * t.sin() * t;

    (new_w, new_h, rotation)
}

// ============================================================================
// PRINCIPIO #5: STRAIGHT AHEAD vs POSE-TO-POSE
// ============================================================================

/// Pose-to-Pose — Interpolación entre poses clave
///
/// Define poses clave y genera frames intermedios automáticamente.
/// Método clásico de animación Disney.
///
/// # Args
/// - keyframes: array de [time, x, y, scale, rotation]
/// - current_time: tiempo actual
///
/// # Retorna
/// [x, y, scale, rotation] interpolado
pub fn pose_to_pose(keyframes: &[(f64, f64, f64, f64, f64)], current_time: f64) -> (f64, f64, f64, f64) {
    if keyframes.is_empty() {
        return (0.0, 0.0, 1.0, 0.0);
    }

    if current_time <= keyframes[0].0 {
        let kf = &keyframes[0];
        return (kf.1, kf.2, kf.3, kf.4);
    }

    if current_time >= keyframes.last().unwrap().0 {
        let kf = keyframes.last().unwrap();
        return (kf.1, kf.2, kf.3, kf.4);
    }

    for i in 0..keyframes.len() - 1 {
        let (t1, x1, y1, s1, r1) = keyframes[i];
        let (t2, x2, y2, s2, r2) = keyframes[i + 1];

        if current_time >= t1 && current_time <= t2 {
            let t = (current_time - t1) / (t2 - t1);
            let et = ease_in_out(t);
            return (
                x1 + (x2 - x1) * et,
                y1 + (y2 - y1) * et,
                s1 + (s2 - s1) * et,
                r1 + (r2 - r1) * et,
            );
        }
    }

    let kf = keyframes.last().unwrap();
    (kf.1, kf.2, kf.3, kf.4)
}

// ============================================================================
// HELPERS
// ============================================================================

/// Easing In-Out — 2t² (t<0.5) o 1-2(1-t)² (t>=0.5)
fn ease_in_out(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - 2.0 * (1.0 - t) * (1.0 - t)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow_through() {
        let result = follow_through(1.0, 1.0, 5.0, 0.0);
        assert!((result - 0.0).abs() < 0.01);

        let result = follow_through(1.0, 1.0, 5.0, 0.1);
        assert!(result.abs() > 0.0); // Debe tener movimiento

        let result = follow_through(1.0, 2.0, 5.0, 1.0);
        assert!(result.abs() < 0.5); // Debe decaer
    }

    #[test]
    fn test_overlapping_action() {
        let offsets = [(0.1, 0.5), (0.2, 0.3)];
        let result = overlapping_action(1.0, &offsets, 0.5);
        assert_eq!(result.len(), 3); // base + 2 partes
        assert!((result[0] - 1.0).abs() < 0.01); // Base sin cambio
    }

    #[test]
    fn test_arc_path() {
        let (x, y) = arc_path((0.0, 0.0), (10.0, 0.0), 5.0, 0.5);
        assert!((x - 5.0).abs() < 0.1); // Centro del arco
        assert!(y > 0.0); // Debe curvarse hacia arriba
    }

    #[test]
    fn test_secondary_action() {
        let (primary, secondary) = secondary_action(1.0, 0.2, 0.5, 0.5);
        assert!((primary - 1.0).abs() < 0.01);
        assert!((secondary - 1.0).abs() > 0.01); // Debe ser diferente
    }

    #[test]
    fn test_timing() {
        let keyframes = [(0.0, 0.0), (10.0, 100.0)];
        let result = timing(&keyframes, 5.0);
        assert!(result > 40.0 && result < 60.0); // ~50 con easing
    }

    #[test]
    fn test_exaggerate() {
        let result = exaggerate(1.0, 2.0, 0.5);
        assert!((result - 1.0).abs() < 0.01); // 1.0 * 2.0 * 0.5 = 1.0
    }

    #[test]
    fn test_solid_rotation() {
        let (x, y, scale) = solid_rotation((0.0, 0.0, 1.0), (0.0, 0.0, 0.0), 60.0);
        assert!((x - 0.0).abs() < 0.01);
        assert!((y - 0.0).abs() < 0.01);
        assert!(scale > 0.0);
    }

    #[test]
    fn test_appeal() {
        let (w, h, rot) = appeal((10.0, 10.0), 0.5, 1.0);
        assert!(w > 10.0); // Debe crecer ligeramente
        assert!(h > 0.0);
        assert!(rot.abs() <= 3.0);
    }

    #[test]
    fn test_pose_to_pose() {
        let keyframes = [
            (0.0, 0.0, 0.0, 1.0, 0.0),
            (1.0, 10.0, 5.0, 1.5, 0.5),
        ];
        let (x, y, s, r) = pose_to_pose(&keyframes, 0.5);
        assert!(x > 2.0 && x < 8.0);
        assert!(y > 1.0 && y < 4.0);
        assert!(s > 1.0 && s < 1.5);
    }
}
