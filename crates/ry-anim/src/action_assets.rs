//! Action Assets — Sistema de Animación de Sprites
//!
//! Gestión de sprites animados con estados, transiciones y blending.
//!
//! ## Funcionalidades
//!
//! - Frame-based animation (animación cuadro por cuadro)
//! - Sprite sheet parser (parsear hojas de sprites)
//! - Animation states (idle, run, jump, attack, etc.)
//! - Animation blending (transiciones suaves entre estados)
//! - Sprite events (eventos al cambiar de estado o frame)
//! - Flip horizontal/vertical
//! - Loop control (loop, once, ping-pong)

use serde_json::{json, Value};

// ============================================================================
// FRAME ANIMATION — Animación cuadro por cuadro
// ============================================================================

/// Frame Animation — Genera frames de una animación sprite
///
/// # Args
/// - frame_count: número total de frames
/// - frame_duration: duración de cada frame en segundos
/// - t: tiempo actual de la animación
/// - loop_mode: "loop", "once", "ping_pong"
///
/// # Retorna
/// {current_frame, progress, is_finished, flipped}
pub fn frame_animation(frame_count: usize, frame_duration: f64, t: f64, loop_mode: &str) -> Value {
    let frame_count = frame_count.max(1);
    let frame_duration = frame_duration.max(0.01);
    let total_duration = frame_count as f64 * frame_duration;
    let mut progress = (t % total_duration) / total_duration;
    let mut flipped = false;
    let mut is_finished = false;

    match loop_mode {
        "once" => {
            if t >= total_duration {
                progress = 1.0;
                is_finished = true;
            } else {
                progress = t / total_duration;
            }
        }
        "ping_pong" => {
            let cycle = (t / total_duration).floor() as usize;
            if cycle % 2 == 1 {
                progress = 1.0 - progress;
                flipped = true;
            }
        }
        _ => {} // loop (default)
    }

    let current_frame = (progress * frame_count as f64).floor() as usize;
    let current_frame = current_frame.min(frame_count - 1);

    json!({
        "current_frame": current_frame,
        "progress": progress,
        "is_finished": is_finished,
        "flipped": flipped,
        "frame_count": frame_count,
        "total_duration": total_duration
    })
}

// ============================================================================
// SPRITE SHEET PARSER — Parsear hojas de sprites
// ============================================================================

/// Sprite Sheet Parse — Calcula rects de frames en un sprite sheet
///
/// # Args
/// - sheet_width: ancho de la hoja de sprites
/// - sheet_height: alto de la hoja
/// - frame_width: ancho de cada frame
/// - frame_height: alto de cada frame
/// - frame_index: índice del frame a obtener (0-based)
/// - columns: número de columnas (0 = auto-calcular)
///
/// # Retorna
/// {x, y, w, h} rect del frame en el sprite sheet
pub fn sprite_sheet_parse(sheet_width: f64, sheet_height: f64, frame_width: f64,
                          frame_height: f64, frame_index: usize, columns: usize) -> Value {
    let cols = if columns > 0 { columns } else { (sheet_width / frame_width).floor() as usize };
    let rows = (sheet_height / frame_height).floor() as usize;

    let row = frame_index / cols;
    let col = frame_index % cols;

    if row >= rows || col >= cols {
        return json!({ "error": "Frame index out of bounds" });
    }

    json!({
        "x": col as f64 * frame_width,
        "y": row as f64 * frame_height,
        "w": frame_width,
        "h": frame_height,
        "frame_index": frame_index,
        "row": row,
        "col": col,
        "total_frames": cols * rows
    })
}

// ============================================================================
/// ANIMATION STATES — Máquina de estados de animación
/// ============================================================================

/// Animation State Manager — Gestiona estados y transiciones
///
/// # Args
/// - current_state: estado actual ("idle", "run", "jump", etc.)
/// - states: array de nombres de estados disponibles
/// - state_durations: array de duraciones por estado
/// - t: tiempo en el estado actual
/// - trigger: evento que dispara transición (o "" para none)
///
/// # Retorna
/// {state, progress, transitioning, next_state, time_in_state}
pub fn animation_state_machine(current_state: &str, states: &[String],
                                state_durations: &[f64], t: f64, trigger: &str) -> Value {
    let state_idx = states.iter().position(|s| s == current_state).unwrap_or(0);
    let duration = state_durations.get(state_idx).copied().unwrap_or(1.0);
    let progress = (t % duration) / duration;

    // Determinar si hay transición
    let mut next_state = current_state.to_string();
    let mut transitioning = false;

    // Transiciones automáticas al finalizar
    if t >= duration && !trigger.is_empty() {
        transitioning = true;
        // Buscar estado que coincida con trigger
        if let Some(idx) = states.iter().position(|s| s == trigger) {
            next_state = states[idx].clone();
        }
    }

    json!({
        "state": current_state,
        "progress": progress,
        "time_in_state": t,
        "duration": duration,
        "transitioning": transitioning,
        "next_state": next_state,
        "state_index": state_idx,
        "total_states": states.len()
    })
}

// ============================================================================
/// ANIMATION BLENDING — Transición suave entre animaciones
/// ============================================================================

/// Animation Blend — Interpola entre dos estados de animación
///
/// # Args
/// - state_a_progress: progreso del estado A (0.0-1.0)
/// - state_b_progress: progreso del estado B (0.0-1.0)
/// - blend_factor: cuánto de B vs A (0.0 = 100% A, 1.0 = 100% B)
/// - blend_duration: duración de la transición
/// - t: tiempo actual del blend
///
/// # Retorna
/// {blended_progress, blend_factor, from_state, to_state, is_complete}
pub fn animation_blend(state_a_progress: f64, state_b_progress: f64,
                       blend_factor: f64, blend_duration: f64, t: f64) -> Value {
    let blend_factor = blend_factor.clamp(0.0, 1.0);
    let blend_progress = (t / blend_duration).min(1.0);
    let current_blend = blend_factor * blend_progress;

    // Ease in-out para suavidad
    let eased = if current_blend < 0.5 {
        2.0 * current_blend * current_blend
    } else {
        1.0 - 2.0 * (1.0 - current_blend) * (1.0 - current_blend)
    };

    let blended_progress = state_a_progress * (1.0 - eased) + state_b_progress * eased;
    let is_complete = blend_progress >= 1.0;

    json!({
        "blended_progress": blended_progress,
        "blend_factor": eased,
        "is_complete": is_complete,
        "progress": blend_progress
    })
}

// ============================================================================
/// SPRITE EVENTS — Eventos al cambiar frame o estado
/// ============================================================================

/// Sprite Events — Genera eventos basados en animación
///
/// # Args
/// - event_type: tipo de evento ("frame_change", "state_change", "loop_complete", "animation_end")
/// - current_frame: frame actual
/// - total_frames: total de frames
/// - current_state: estado actual
/// - progress: progreso de la animación (0.0-1.0)
///
/// # Retorna
/// {event_type, triggered, data}
pub fn sprite_events(event_type: &str, current_frame: usize, total_frames: usize,
                     current_state: &str, progress: f64) -> Value {
    let mut triggered = false;
    let mut data = json!({});

    match event_type {
        "frame_change" => {
            triggered = true;
            data = json!({ "frame": current_frame, "total": total_frames });
        }
        "state_change" => {
            triggered = progress >= 1.0;
            data = json!({ "from_state": current_state });
        }
        "loop_complete" => {
            triggered = progress >= 1.0 && current_frame == total_frames - 1;
            data = json!({ "loops_completed": (progress).floor() as usize });
        }
        "animation_end" => {
            triggered = progress >= 1.0;
            data = json!({ "final_frame": current_frame, "state": current_state });
        }
        _ => {}
    }

    json!({
        "event_type": event_type,
        "triggered": triggered,
        "data": data
    })
}

// ============================================================================
/// FLIP & MIRROR — Volteo de sprites
/// ============================================================================

/// Sprite Flip — Calcula transformaciones de flip
///
/// # Args
/// - flip_horizontal: voltear horizontalmente
/// - flip_vertical: voltear verticalmente
/// - origin_x: origen X para flip (default 0.5 = centro)
/// - origin_y: origen Y para flip (default 0.5 = centro)
///
/// # Retorna
/// {scale_x, scale_y, origin_x, origin_y, is_flipped}
pub fn sprite_flip(flip_horizontal: bool, flip_vertical: bool,
                   origin_x: f64, origin_y: f64) -> Value {
    let scale_x = if flip_horizontal { -1.0 } else { 1.0 };
    let scale_y = if flip_vertical { -1.0 } else { 1.0 };
    let is_flipped = flip_horizontal || flip_vertical;

    json!({
        "scale_x": scale_x,
        "scale_y": scale_y,
        "origin_x": origin_x,
        "origin_y": origin_y,
        "is_flipped": is_flipped,
        "flip_horizontal": flip_horizontal,
        "flip_vertical": flip_vertical
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_animation_loop() {
        let result = frame_animation(4, 0.25, 0.5, "loop");
        assert_eq!(result.get("current_frame").unwrap().as_u64().unwrap(), 2);
        assert!(!result.get("is_finished").unwrap().as_bool().unwrap());
    }

    #[test]
    fn test_frame_animation_once() {
        let result = frame_animation(4, 0.25, 1.5, "once");
        assert!(result.get("is_finished").unwrap().as_bool().unwrap());
    }

    #[test]
    fn test_frame_animation_ping_pong() {
        let result = frame_animation(4, 0.25, 1.2, "ping_pong");
        assert!(result.get("flipped").unwrap().as_bool().unwrap());
    }

    #[test]
    fn test_sprite_sheet_parse() {
        let result = sprite_sheet_parse(256.0, 256.0, 64.0, 64.0, 5, 4);
        assert_eq!(result.get("row").unwrap().as_u64().unwrap(), 1);
        assert_eq!(result.get("col").unwrap().as_u64().unwrap(), 1);
        assert_eq!(result.get("total_frames").unwrap().as_u64().unwrap(), 16);
    }

    #[test]
    fn test_animation_state_machine() {
        let states = vec!["idle".to_string(), "run".to_string(), "jump".to_string()];
        let durations = vec![2.0, 1.0, 0.5];
        let result = animation_state_machine("idle", &states, &durations, 1.0, "");
        assert_eq!(result.get("state").unwrap().as_str().unwrap(), "idle");
        assert!((result.get("progress").unwrap().as_f64().unwrap() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_animation_blend() {
        let result = animation_blend(0.0, 1.0, 1.0, 1.0, 0.5);
        assert!(result.get("blended_progress").unwrap().as_f64().unwrap() > 0.0);
        assert!(!result.get("is_complete").unwrap().as_bool().unwrap());
    }

    #[test]
    fn test_sprite_events() {
        let result = sprite_events("frame_change", 2, 4, "run", 0.5);
        assert!(result.get("triggered").unwrap().as_bool().unwrap());
    }

    #[test]
    fn test_sprite_flip() {
        let result = sprite_flip(true, false, 0.5, 0.5);
        assert!((result.get("scale_x").unwrap().as_f64().unwrap() - (-1.0)).abs() < 0.01);
        assert!(result.get("flip_horizontal").unwrap().as_bool().unwrap());
    }
}
