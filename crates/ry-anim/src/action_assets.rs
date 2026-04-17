//! Action Assets — Sistema de Animación de Sprites
//!
//! Gestión de sprites animados con estados, transiciones y blending.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoopMode {
    Loop,
    Once,
    PingPong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameRect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteAnimation {
    pub frame_count: usize,
    pub frame_duration: f64,
    pub loop_mode: LoopMode,
}

#[derive(Debug, Clone, Serialize)]
pub struct AnimationResult {
    pub current_frame: usize,
    pub progress: f64,
    pub is_finished: bool,
    pub flipped: bool,
}

impl SpriteAnimation {
    pub fn update(&self, t: f64) -> AnimationResult {
        let frame_count = self.frame_count.max(1);
        let frame_duration = self.frame_duration.max(0.01);
        let total_duration = frame_count as f64 * frame_duration;
        let mut progress = (t % total_duration) / total_duration;
        let mut flipped = false;
        let mut is_finished = false;

        match self.loop_mode {
            LoopMode::Once => {
                if t >= total_duration {
                    progress = 1.0;
                    is_finished = true;
                } else {
                    progress = t / total_duration;
                }
            }
            LoopMode::PingPong => {
                let cycle = (t / total_duration).floor() as usize;
                if cycle % 2 == 1 {
                    progress = 1.0 - progress;
                    flipped = true;
                }
            }
            LoopMode::Loop => {}
        }

        let current_frame = (progress * frame_count as f64).floor() as usize;
        let current_frame = current_frame.min(frame_count - 1);

        AnimationResult {
            current_frame,
            progress,
            is_finished,
            flipped,
        }
    }
}

pub struct SpriteSheet {
    pub width: f64,
    pub height: f64,
    pub frame_width: f64,
    pub frame_height: f64,
}

impl SpriteSheet {
    pub fn get_frame_rect(&self, frame_index: usize, columns: usize) -> Option<FrameRect> {
        let cols = if columns > 0 { columns } else { (self.width / self.frame_width).floor() as usize };
        let rows = (self.height / self.frame_height).floor() as usize;

        let row = frame_index / cols;
        let col = frame_index % cols;

        if row >= rows || col >= cols {
            return None;
        }

        Some(FrameRect {
            x: col as f64 * self.frame_width,
            y: row as f64 * self.frame_height,
            w: self.frame_width,
            h: self.frame_height,
        })
    }
}

// ============================================================================
// COMPATIBILITY WRAPPERS (LEGACY)
// ============================================================================

pub fn frame_animation(frame_count: usize, frame_duration: f64, t: f64, loop_mode: &str) -> Value {
    let mode = match loop_mode {
        "once" => LoopMode::Once,
        "ping_pong" => LoopMode::PingPong,
        _ => LoopMode::Loop,
    };
    let anim = SpriteAnimation { frame_count, frame_duration, loop_mode: mode };
    let res = anim.update(t);
    json!(res)
}

pub fn sprite_sheet_parse(sheet_width: f64, sheet_height: f64, frame_width: f64,
                          frame_height: f64, frame_index: usize, columns: usize) -> Value {
    let sheet = SpriteSheet { width: sheet_width, height: sheet_height, frame_width, frame_height };
    if let Some(rect) = sheet.get_frame_rect(frame_index, columns) {
        json!(rect)
    } else {
        json!({ "error": "Frame index out of bounds" })
    }
}

pub fn animation_state_machine(_current_state: &str, _states: &[String], _state_durations: &[f64], _t: f64, _trigger: &str) -> Value {
    json!({ "error": "Not implemented in legacy wrapper" })
}

pub fn animation_blend(_a: f64, _b: f64, _f: f64, _d: f64, _t: f64) -> Value {
    json!({ "error": "Not implemented in legacy wrapper" })
}

pub fn sprite_events(_etype: &str, _f: usize, _tf: usize, _s: &str, _p: f64) -> Value {
    json!({ "error": "Not implemented in legacy wrapper" })
}

pub fn sprite_flip(h: bool, v: bool, ox: f64, oy: f64) -> Value {
    json!({ "scale_x": if h { -1.0 } else { 1.0 }, "scale_y": if v { -1.0 } else { 1.0 }, "is_flipped": h || v })
}
