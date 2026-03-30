//! Módulo de Geometría - Ilusiones Ópticas
//!
//! Implementa ilusiones ópticas clásicas usando matemáticas simples:
//! - Triángulo de Penrose (tribar)
//! - Cubo imposible (Necker cube)
//! - Espiral óptica

use serde_json::{json, Value};

/// Genera las coordenadas para el Triángulo de Penrose
///
/// # Parámetros
/// - `center_x`: Centro X en pantalla
/// - `center_y`: Centro Y en pantalla  
/// - `size`: Tamaño del triángulo
///
/// # Retorna
/// Array de líneas: [[x1, y1, x2, y2], ...]
pub fn penrose(center_x: f64, center_y: f64, size: f64) -> Value {
    let s = size;

    // Coordenadas de los 3 vértices principales
    let v1_x = center_x;
    let v1_y = center_y - s * 0.577; // cos(30°)

    let v2_x = center_x - s * 0.5;
    let v2_y = center_y + s * 0.289;

    let v3_x = center_x + s * 0.5;
    let v3_y = center_y + s * 0.289;

    // Grosor de las barras
    let thick = s * 0.15;

    // Barras del triángulo imposible
    // Cada barra tiene 2 líneas paralelas para dar grosor
    let mut lines = Vec::new();

    // Barra 1: v1 -> v2 (con "quiebre" imposible)
    lines.push(json!([
        v1_x - thick * 0.3,
        v1_y - thick * 0.5,
        v2_x + thick * 0.5,
        v2_y - thick * 0.3
    ]));
    lines.push(json!([
        v1_x + thick * 0.3,
        v1_y - thick * 0.5,
        v2_x - thick * 0.5,
        v2_y + thick * 0.3
    ]));

    // Barra 2: v2 -> v3
    lines.push(json!([
        v2_x + thick * 0.5,
        v2_y - thick * 0.3,
        v3_x - thick * 0.5,
        v3_y - thick * 0.3
    ]));
    lines.push(json!([
        v2_x + thick * 0.5,
        v2_y + thick * 0.3,
        v3_x - thick * 0.5,
        v3_y + thick * 0.3
    ]));

    // Barra 3: v3 -> v1 (con "quiebre" imposible)
    lines.push(json!([
        v3_x + thick * 0.3,
        v3_y - thick * 0.5,
        v1_x + thick * 0.3,
        v1_y + thick * 0.5
    ]));
    lines.push(json!([
        v3_x - thick * 0.3,
        v3_y + thick * 0.5,
        v1_x - thick * 0.3,
        v1_y + thick * 0.5
    ]));

    // Líneas de conexión "imposibles" en las esquinas
    // Esquina v1
    lines.push(json!([
        v1_x - thick * 0.3,
        v1_y - thick * 0.5,
        v1_x - thick * 0.3,
        v1_y + thick * 0.5
    ]));
    lines.push(json!([
        v1_x + thick * 0.3,
        v1_y - thick * 0.5,
        v1_x + thick * 0.3,
        v1_y + thick * 0.5
    ]));

    // Esquina v2
    lines.push(json!([
        v2_x - thick * 0.5,
        v2_y - thick * 0.3,
        v2_x - thick * 0.5,
        v2_y + thick * 0.3
    ]));
    lines.push(json!([
        v2_x + thick * 0.5,
        v2_y - thick * 0.3,
        v2_x + thick * 0.5,
        v2_y + thick * 0.3
    ]));

    // Esquina v3
    lines.push(json!([
        v3_x - thick * 0.5,
        v3_y - thick * 0.3,
        v3_x - thick * 0.5,
        v3_y + thick * 0.3
    ]));
    lines.push(json!([
        v3_x + thick * 0.5,
        v3_y - thick * 0.3,
        v3_x + thick * 0.5,
        v3_y + thick * 0.3
    ]));

    json!(lines)
}

/// Genera las coordenadas para el Cubo Imposible
///
/// # Parámetros
/// - `center_x`: Centro X en pantalla
/// - `center_y`: Centro Y en pantalla
/// - `size`: Tamaño del cubo
///
/// # Retorna
/// Array de líneas: [[x1, y1, x2, y2], ...]
pub fn impossible_cube(center_x: f64, center_y: f64, size: f64) -> Value {
    let s = size * 0.5;

    // Cubo frontal
    let f_bl_x = center_x - s; // front bottom-left
    let f_bl_y = center_y + s;
    let f_br_x = center_x + s; // front bottom-right
    let f_br_y = center_y + s;
    let f_tl_x = center_x - s; // front top-left
    let f_tl_y = center_y - s;
    let f_tr_x = center_x + s; // front top-right
    let f_tr_y = center_y - s;

    // Cubo trasero (desplazado)
    let offset = s * 0.6;
    let b_bl_x = center_x - s + offset; // back bottom-left
    let b_bl_y = center_y + s - offset;
    let b_br_x = center_x + s + offset; // back bottom-right
    let b_br_y = center_y + s - offset;
    let b_tl_x = center_x - s + offset; // back top-left
    let b_tl_y = center_y - s - offset;
    let b_tr_x = center_x + s + offset; // back top-right
    let b_tr_y = center_y - s - offset;

    let mut lines = Vec::new();

    // Cara frontal
    lines.push(json!([f_bl_x, f_bl_y, f_br_x, f_br_y])); // bottom
    lines.push(json!([f_br_x, f_br_y, f_tr_x, f_tr_y])); // right
    lines.push(json!([f_tr_x, f_tr_y, f_tl_x, f_tl_y])); // top
    lines.push(json!([f_tl_x, f_tl_y, f_bl_x, f_bl_y])); // left

    // Cara trasera
    lines.push(json!([b_bl_x, b_bl_y, b_br_x, b_br_y])); // bottom
    lines.push(json!([b_br_x, b_br_y, b_tr_x, b_tr_y])); // right
    lines.push(json!([b_tr_x, b_tr_y, b_tl_x, b_tl_y])); // top
    lines.push(json!([b_tl_x, b_tl_y, b_bl_x, b_bl_y])); // left

    // Conexiones frontal-trasera (algunas "imposibles")
    lines.push(json!([f_bl_x, f_bl_y, b_bl_x, b_bl_y]));
    lines.push(json!([f_br_x, f_br_y, b_br_x, b_br_y]));
    lines.push(json!([f_tl_x, f_tl_y, b_tl_x, b_tl_y]));
    lines.push(json!([f_tr_x, f_tr_y, b_tr_x, b_tr_y]));

    // Líneas adicionales para efecto imposible
    // Conexión cruzada que crea la imposibilidad
    lines.push(json!([f_bl_x + s * 0.3, f_bl_y, b_bl_x - s * 0.3, b_bl_y]));
    lines.push(json!([f_tr_x - s * 0.3, f_tr_y, b_tr_x + s * 0.3, b_tr_y]));

    json!(lines)
}

/// Genera las coordenadas para la Espiral Óptica
///
/// # Parámetros
/// - `center_x`: Centro X en pantalla
/// - `center_y`: Centro Y en pantalla
/// - `turns`: Número de vueltas
/// - `radius`: Radio máximo
/// - `points`: Puntos por vuelta
///
/// # Retorna
/// Array de puntos: [[x1, y1], [x2, y2], ...]
pub fn spiral(center_x: f64, center_y: f64, turns: i32, radius: f64, points: i32) -> Value {
    let mut points_arr = Vec::new();
    let total_points = turns * points;

    for i in 0..total_points {
        let t = (i as f64) / (total_points as f64); // 0.0 a 1.0
        let angle = t * turns as f64 * 2.0 * std::f64::consts::PI;
        let r = t * radius;

        let x = center_x + r * angle.cos();
        let y = center_y + r * angle.sin();

        points_arr.push(json!([x, y]));
    }

    json!(points_arr)
}

/// Genera la ilusión de Müller-Lyer
///
/// # Parámetros
/// - `center_x`: Centro X en pantalla
/// - `center_y`: Centro Y en pantalla
/// - `length`: Longitud de la línea principal
///
/// # Retorna
/// Array de líneas: [[x1, y1, x2, y2], ...]
pub fn muller_lyer(center_x: f64, center_y: f64, length: f64) -> Value {
    let half = length / 2.0;
    let arrow_size = length * 0.15;

    let mut lines = Vec::new();

    // Línea 1: Flechas hacia adentro (>)
    let y1 = center_y - length * 0.3;
    lines.push(json!([center_x - half, y1, center_x + half, y1])); // línea principal

    // Flecha izquierda adentro
    lines.push(json!([
        center_x - half,
        y1,
        center_x - half + arrow_size,
        y1 - arrow_size * 0.6
    ]));
    lines.push(json!([
        center_x - half,
        y1,
        center_x - half + arrow_size,
        y1 + arrow_size * 0.6
    ]));

    // Flecha derecha adentro
    lines.push(json!([
        center_x + half,
        y1,
        center_x + half - arrow_size,
        y1 - arrow_size * 0.6
    ]));
    lines.push(json!([
        center_x + half,
        y1,
        center_x + half - arrow_size,
        y1 + arrow_size * 0.6
    ]));

    // Línea 2: Flechas hacia afuera (<)
    let y2 = center_y + length * 0.3;
    lines.push(json!([center_x - half, y2, center_x + half, y2])); // línea principal

    // Flecha izquierda afuera
    lines.push(json!([
        center_x - half,
        y2,
        center_x - half - arrow_size,
        y2 - arrow_size * 0.6
    ]));
    lines.push(json!([
        center_x - half,
        y2,
        center_x - half - arrow_size,
        y2 + arrow_size * 0.6
    ]));

    // Flecha derecha afuera
    lines.push(json!([
        center_x + half,
        y2,
        center_x + half + arrow_size,
        y2 - arrow_size * 0.6
    ]));
    lines.push(json!([
        center_x + half,
        y2,
        center_x + half + arrow_size,
        y2 + arrow_size * 0.6
    ]));

    json!(lines)
}

/// Genera la ilusión de Ponzo (perspectiva)
///
/// # Parámetros
/// - `center_x`: Centro X en pantalla
/// - `center_y`: Centro Y en pantalla
/// - `height`: Altura de la perspectiva
/// - `width_top`: Ancho superior
/// - `width_bottom`: Ancho inferior
///
/// # Retorna
/// Array de líneas: [[x1, y1, x2, y2], ...]
pub fn ponzo(
    center_x: f64,
    center_y: f64,
    height: f64,
    width_top: f64,
    width_bottom: f64,
) -> Value {
    let mut lines = Vec::new();

    // Líneas de perspectiva (rieles)
    let top_y = center_y - height / 2.0;
    let bottom_y = center_y + height / 2.0;

    lines.push(json!([
        center_x - width_top / 2.0,
        top_y,
        center_x - width_bottom / 2.0,
        bottom_y
    ]));
    lines.push(json!([
        center_x + width_top / 2.0,
        top_y,
        center_x + width_bottom / 2.0,
        bottom_y
    ]));

    // Líneas horizontales (la de arriba parece más larga)
    let top_line_width = width_top * 0.8;
    let bottom_line_width = width_bottom * 0.8;

    // Línea superior
    lines.push(json!([
        center_x - top_line_width / 2.0,
        top_y + height * 0.2,
        center_x + top_line_width / 2.0,
        top_y + height * 0.2
    ]));

    // Línea inferior (misma longitud real, parece más corta)
    lines.push(json!([
        center_x - bottom_line_width / 2.0,
        bottom_y - height * 0.2,
        center_x + bottom_line_width / 2.0,
        bottom_y - height * 0.2
    ]));

    // Líneas adicionales para reforzar la perspectiva
    let mid_y = (top_y + bottom_y) / 2.0;
    let mid_line_width = (width_top + width_bottom) * 0.4;
    lines.push(json!([
        center_x - mid_line_width / 2.0,
        mid_y,
        center_x + mid_line_width / 2.0,
        mid_y
    ]));
    lines.push(json!([
        center_x - mid_line_width / 2.0 * 0.5,
        mid_y + height * 0.15,
        center_x + mid_line_width / 2.0 * 0.5,
        mid_y + height * 0.15
    ]));

    json!(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_penrose_returns_lines() {
        let result = penrose(400.0, 300.0, 100.0);
        let lines = result.as_array().unwrap();

        assert!(!lines.is_empty());
        assert!(lines.len() >= 10); // Al menos 10 líneas

        // Cada línea es [x1, y1, x2, y2]
        let first_line = lines[0].as_array().unwrap();
        assert_eq!(first_line.len(), 4);
    }

    #[test]
    fn test_impossible_cube_returns_lines() {
        let result = impossible_cube(400.0, 300.0, 100.0);
        let lines = result.as_array().unwrap();

        assert!(!lines.is_empty());
        assert!(lines.len() >= 12); // Cubo tiene 12 aristas + extra

        let first_line = lines[0].as_array().unwrap();
        assert_eq!(first_line.len(), 4);
    }

    #[test]
    fn test_spiral_returns_points() {
        let result = spiral(400.0, 300.0, 3, 100.0, 20);
        let points = result.as_array().unwrap();

        assert_eq!(points.len(), 60); // 3 turns * 20 points

        let first_point = points[0].as_array().unwrap();
        assert_eq!(first_point.len(), 2); // [x, y]
    }

    #[test]
    fn test_muller_lyer_returns_lines() {
        let result = muller_lyer(400.0, 300.0, 200.0);
        let lines = result.as_array().unwrap();

        assert_eq!(lines.len(), 10); // 2 líneas principales + 8 flechas

        let first_line = lines[0].as_array().unwrap();
        assert_eq!(first_line.len(), 4);
    }

    #[test]
    fn test_ponzo_returns_lines() {
        let result = ponzo(400.0, 300.0, 300.0, 100.0, 300.0);
        let lines = result.as_array().unwrap();

        assert_eq!(lines.len(), 6); // 2 rieles + 4 horizontales

        let first_line = lines[0].as_array().unwrap();
        assert_eq!(first_line.len(), 4);
    }

    #[test]
    fn test_spiral_center_point() {
        // El primer punto debe estar cerca del centro
        let result = spiral(400.0, 300.0, 1, 100.0, 10);
        let points = result.as_array().unwrap();
        let first_point = points[0].as_array().unwrap();

        let x = first_point[0].as_f64().unwrap();
        let y = first_point[1].as_f64().unwrap();

        assert!((x - 400.0).abs() < 1.0); // Cerca del centro
        assert!((y - 300.0).abs() < 1.0);
    }

    #[test]
    fn test_spiral_outer_point() {
        // El último punto debe estar cerca del radio máximo
        let result = spiral(400.0, 300.0, 1, 100.0, 10);
        let points = result.as_array().unwrap();
        let last_point = points[points.len() - 1].as_array().unwrap();

        let x = last_point[0].as_f64().unwrap();
        let y = last_point[1].as_f64().unwrap();

        // Distancia desde el centro debería ser ~100 (radio máximo)
        // Nota: como es espiral de 1 vuelta, el último punto está cerca del radio máximo
        let dist = ((x - 400.0).powi(2) + (y - 300.0).powi(2)).sqrt();
        assert!(dist > 80.0 && dist < 120.0); // Rango razonable para espiral de 1 vuelta
    }
}
