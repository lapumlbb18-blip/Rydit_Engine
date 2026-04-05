//! RyDit Anim - Módulo de Animación para Ry-Dit
//!
//! Implementa principios de animación de Disney:
//! - Principio #1: Squash & Stretch (Deformación)
//! - Principio #2: Anticipation (Anticipación)
//! - Principio #3: Staging (en progreso)
//! - Principio #4: Follow Through & Overlapping Action
//! - Principio #5: Straight Ahead vs Pose-to-Pose
//! - Principio #6: Slow In & Slow Out (Easing)
//! - Principio #7: Arcs
//! - Principio #8: Secondary Action
//! - Principio #9: Timing
//! - Principio #10: Exaggeration
//! - Principio #11: Solid Drawing
//! - Principio #12: Appeal

pub mod particles;
pub mod disney;
pub mod illusions;
pub mod effects;
pub mod science_anim;

pub use disney::{
    appeal, arc_path, exaggerate, follow_through, overlapping_action, pose_to_pose,
    secondary_action, solid_rotation, timing,
};

pub use illusions::{
    cafe_wall, motion_induced_blindness, pulsing_star, rotating_snakes,
    troxler_fading, zollner_effect,
};

pub use effects::{
    bloom_effect, chromatic_aberration, morph_shapes, motion_blur,
    neon_glow, particle_trails,
};

pub use science_anim::{
    cell_division, chemical_crystallization, flight_pattern,
    lsystem_tree, pendulum_waves, tusi_couple, walk_cycle, wave_interference,
};

use ry_core::{ModuleError, ModuleResult, RyditModule};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Módulo de Animación - 12 principios de Disney
pub struct AnimModule;

impl RyditModule for AnimModule {
    fn name(&self) -> &'static str { "anim" }
    fn version(&self) -> &'static str { "0.11.0" }

    fn register(&self) -> HashMap<&'static str, &'static str> {
        let mut cmds = HashMap::new();
        cmds.insert("ease_in", "Easing In - comienza lento, acelera");
        cmds.insert("ease_out", "Easing Out - comienza rápido, frena");
        cmds.insert("ease_in_out", "Easing In-Out - combina ambos");
        cmds.insert("squash", "Squash - aplasta (mantiene área)");
        cmds.insert("stretch", "Stretch - estira (mantiene área)");
        cmds.insert("anticipate", "Anticipation - retrocede antes de avanzar");
        cmds.insert("follow_through", "Follow Through - partes siguen moviéndose");
        cmds.insert("overlapping_action", "Overlapping Action - partes a distintas velocidades");
        cmds.insert("arc_path", "Arcs - trayectoria curva entre puntos");
        cmds.insert("secondary_action", "Secondary Action - movimiento secundario");
        cmds.insert("timing", "Timing - interpolación entre keyframes");
        cmds.insert("exaggerate", "Exaggeration - exagerar movimientos");
        cmds.insert("solid_rotation", "Solid Drawing - rotación 3D con perspectiva");
        cmds.insert("appeal", "Appeal - hacer forma más atractiva");
        cmds.insert("pose_to_pose", "Pose-to-Pose - interpolación entre poses clave");
        // ✅ v0.9.0: Ilusiones ópticas animadas
        cmds.insert("rotating_snakes", "Rotating Snakes - ilusión de movimiento circular");
        cmds.insert("cafe_wall", "Cafe Wall - líneas paralelas que parecen inclinadas");
        cmds.insert("troxler_fading", "Troxler Fading - desvanecimiento por fijación");
        cmds.insert("pulsing_star", "Pulsing Star - estrella que pulsa");
        cmds.insert("zollner_effect", "Zöllner Effect - líneas que parecen no ser paralelas");
        cmds.insert("motion_blindness", "Motion-Induced Blindness - puntos que desaparecen");
        // ✅ v0.10.0: Efectos especiales
        cmds.insert("neon_glow", "Neon Glow - resplandor neón configurable");
        cmds.insert("motion_blur", "Motion Blur - desenfoque de movimiento");
        cmds.insert("chromatic_aberration", "Chromatic Aberration - separación RGB");
        cmds.insert("bloom_effect", "Bloom - brillo difuso en zonas claras");
        cmds.insert("particle_trails", "Particle Trails - estelas de partículas");
        cmds.insert("morph_shapes", "Morphing - transición entre formas");
        // ✅ v0.11.0: Ciencia animada
        cmds.insert("chemical_crystallization", "Chemical - cristalización animada");
        cmds.insert("cell_division", "Biological - división celular");
        cmds.insert("walk_cycle", "Fauna - ciclo de caminata");
        cmds.insert("flight_pattern", "Fauna - aleteo de aves");
        cmds.insert("lsystem_tree", "Flora - árbol L-System animado");
        cmds.insert("tusi_couple", "Historical - Pareja de Tusi");
        cmds.insert("pendulum_waves", "Physics - ondas de péndulos");
        cmds.insert("wave_interference", "Physics - interferencia de ondas");
        cmds
    }

    fn execute(&self, command: &str, params: Value) -> ModuleResult {
        let invalid = || ModuleError { code: "INVALID_PARAMS".to_string(), message: format!("Parámetros inválidos para {}", command) };
        match command {
            "ease_in" => self.ease_in(params),
            "ease_out" => self.ease_out(params),
            "ease_in_out" => self.ease_in_out(params),
            "squash" => self.squash(params),
            "stretch" => self.stretch(params),
            "anticipate" => self.anticipate(params),
            "follow_through" => self.follow_through(params),
            "overlapping_action" => self.overlapping_action(params),
            "arc_path" => self.arc_path(params),
            "secondary_action" => self.secondary_action(params),
            "timing" => self.timing(params),
            "exaggerate" => self.exaggerate(params),
            "solid_rotation" => self.solid_rotation(params),
            "appeal" => self.appeal(params),
            "pose_to_pose" => self.pose_to_pose(params),
            // ✅ v0.9.0: Ilusiones ópticas
            "rotating_snakes" => self.rotating_snakes(params),
            "cafe_wall" => self.cafe_wall(params),
            "troxler_fading" => self.troxler_fading(params),
            "pulsing_star" => self.pulsing_star(params),
            "zollner_effect" => self.zollner_effect(params),
            "motion_blindness" => self.motion_blindness(params),
            // ✅ v0.10.0: Efectos especiales
            "neon_glow" => self.neon_glow(params),
            "motion_blur" => self.motion_blur(params),
            "chromatic_aberration" => self.chromatic_aberration(params),
            "bloom_effect" => self.bloom_effect(params),
            "particle_trails" => self.particle_trails(params),
            "morph_shapes" => self.morph_shapes(params),
            // ✅ v0.11.0: Ciencia animada
            "chemical_crystallization" => self.chemical_crystallization(params),
            "cell_division" => self.cell_division(params),
            "walk_cycle" => self.walk_cycle(params),
            "flight_pattern" => self.flight_pattern(params),
            "lsystem_tree" => self.lsystem_tree(params),
            "tusi_couple" => self.tusi_couple(params),
            "pendulum_waves" => self.pendulum_waves(params),
            "wave_interference" => self.wave_interference(params),
            _ => Err(ModuleError { code: "UNKNOWN_COMMAND".to_string(), message: format!("Comando desconocido: {}", command) }),
        }
    }
}

impl AnimModule {
    fn ease_in(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "ease_in requiere [t]".to_string() })?;
        if a.len() != 1 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "ease_in requiere 1 param".to_string() }); }
        let t = a[0].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);
        Ok(json!(t * t))
    }

    fn ease_out(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "ease_out requiere [t]".to_string() })?;
        if a.len() != 1 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "ease_out requiere 1 param".to_string() }); }
        let t = a[0].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);
        Ok(json!(t * (2.0 - t)))
    }

    fn ease_in_out(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "ease_in_out requiere [t]".to_string() })?;
        if a.len() != 1 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "ease_in_out requiere 1 param".to_string() }); }
        let t = a[0].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);
        let r = if t < 0.5 { 2.0 * t * t } else { 1.0 - 2.0 * (1.0 - t) * (1.0 - t) };
        Ok(json!(r))
    }

    fn squash(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "squash requiere [factor]".to_string() })?;
        if a.len() != 1 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "squash requiere 1 param".to_string() }); }
        let f = a[0].as_f64().unwrap_or(1.0).clamp(0.5, 2.0);
        Ok(json!([f, 1.0 / f]))
    }

    fn stretch(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "stretch requiere [factor]".to_string() })?;
        if a.len() != 1 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "stretch requiere 1 param".to_string() }); }
        let f = a[0].as_f64().unwrap_or(1.0).clamp(0.5, 2.0);
        Ok(json!([1.0 / f, f]))
    }

    fn anticipate(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "anticipate requiere [pos, target, amount]".to_string() })?;
        if a.len() != 3 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "anticipate requiere 3 params".to_string() }); }
        let pos = a[0].as_f64().unwrap_or(0.0);
        let target = a[1].as_f64().unwrap_or(0.0);
        let amount = a[2].as_f64().unwrap_or(0.0);
        let dir = if target > pos { -1.0 } else { 1.0 };
        Ok(json!(pos + dir * amount))
    }

    // ===== v0.8.0: 9 PRINCIPIOS NUEVOS =====

    fn follow_through(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "follow_through requiere [amp, decay, freq, t]".to_string() })?;
        if a.len() != 4 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "follow_through requiere 4 params".to_string() }); }
        Ok(json!(disney::follow_through(a[0].as_f64().unwrap_or(1.0), a[1].as_f64().unwrap_or(1.0), a[2].as_f64().unwrap_or(5.0), a[3].as_f64().unwrap_or(0.0))))
    }

    fn overlapping_action(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "overlapping_action requiere [base, offsets, t]".to_string() })?;
        if a.len() != 3 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "overlapping_action requiere 3 params".to_string() }); }
        let base = a[0].as_f64().unwrap_or(0.0);
        let offsets: Vec<(f64, f64)> = a[1].as_array().map(|arr| arr.iter().filter_map(|v| v.as_array().map(|p| (p[0].as_f64().unwrap_or(0.0), p[1].as_f64().unwrap_or(0.0)))).collect()).unwrap_or_default();
        let t = a[2].as_f64().unwrap_or(0.0);
        Ok(json!(disney::overlapping_action(base, &offsets, t)))
    }

    fn arc_path(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "arc_path requiere [sx, sy, ex, ey, curvature, t]".to_string() })?;
        if a.len() != 6 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "arc_path requiere 6 params".to_string() }); }
        let (x, y) = disney::arc_path((a[0].as_f64().unwrap_or(0.0), a[1].as_f64().unwrap_or(0.0)), (a[2].as_f64().unwrap_or(10.0), a[3].as_f64().unwrap_or(0.0)), a[4].as_f64().unwrap_or(5.0), a[5].as_f64().unwrap_or(0.0));
        Ok(json!([x, y]))
    }

    fn secondary_action(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "secondary_action requiere [primary, offset, amp, t]".to_string() })?;
        if a.len() != 4 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "secondary_action requiere 4 params".to_string() }); }
        let (pr, sc) = disney::secondary_action(a[0].as_f64().unwrap_or(0.0), a[1].as_f64().unwrap_or(0.2), a[2].as_f64().unwrap_or(0.5), a[3].as_f64().unwrap_or(0.0));
        Ok(json!([pr, sc]))
    }

    fn timing(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "timing requiere [keyframes, frame]".to_string() })?;
        if a.len() != 2 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "timing requiere 2 params".to_string() }); }
        let kfs: Vec<(f64, f64)> = a[0].as_array().map(|arr| arr.iter().filter_map(|v| v.as_array().map(|p| (p[0].as_f64().unwrap_or(0.0), p[1].as_f64().unwrap_or(0.0)))).collect()).unwrap_or_default();
        Ok(json!(disney::timing(&kfs, a[1].as_f64().unwrap_or(0.0))))
    }

    fn exaggerate(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "exaggerate requiere [base, factor, t]".to_string() })?;
        if a.len() != 3 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "exaggerate requiere 3 params".to_string() }); }
        Ok(json!(disney::exaggerate(a[0].as_f64().unwrap_or(0.0), a[1].as_f64().unwrap_or(1.5), a[2].as_f64().unwrap_or(0.0))))
    }

    fn solid_rotation(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "solid_rotation requiere [x,y,z,rx,ry,rz,fov]".to_string() })?;
        if a.len() != 7 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "solid_rotation requiere 7 params".to_string() }); }
        let (x, y, s) = disney::solid_rotation((a[0].as_f64().unwrap_or(0.0), a[1].as_f64().unwrap_or(0.0), a[2].as_f64().unwrap_or(0.0)), (a[3].as_f64().unwrap_or(0.0), a[4].as_f64().unwrap_or(0.0), a[5].as_f64().unwrap_or(0.0)), a[6].as_f64().unwrap_or(60.0));
        Ok(json!([x, y, s]))
    }

    fn appeal(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "appeal requiere [w, h, charm, t]".to_string() })?;
        if a.len() != 4 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "appeal requiere 4 params".to_string() }); }
        let (w, h, r) = disney::appeal((a[0].as_f64().unwrap_or(10.0), a[1].as_f64().unwrap_or(10.0)), a[2].as_f64().unwrap_or(0.5), a[3].as_f64().unwrap_or(0.0));
        Ok(json!([w, h, r]))
    }

    fn pose_to_pose(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "pose_to_pose requiere [kfs, time]".to_string() })?;
        if a.len() != 2 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "pose_to_pose requiere 2 params".to_string() }); }
        let kfs: Vec<(f64, f64, f64, f64, f64)> = a[0].as_array().map(|arr| arr.iter().filter_map(|v| v.as_array().map(|p| (p[0].as_f64().unwrap_or(0.0), p[1].as_f64().unwrap_or(0.0), p[2].as_f64().unwrap_or(0.0), p[3].as_f64().unwrap_or(1.0), p[4].as_f64().unwrap_or(0.0)))).collect()).unwrap_or_default();
        let (x, y, s, r) = disney::pose_to_pose(&kfs, a[1].as_f64().unwrap_or(0.0));
        Ok(json!([x, y, s, r]))
    }

    // ===== v0.9.0: ILUSIONES ÓPTICAS =====

    fn rotating_snakes(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "rotating_snakes requiere [cx, cy, radius, segments, t]".to_string() })?;
        if a.len() < 5 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "rotating_snakes requiere 5+ params".to_string() }); }
        let colors: Vec<String> = a.get(5).and_then(|v| v.as_array()).map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()).unwrap_or_default();
        Ok(json!(illusions::rotating_snakes(a[0].as_f64().unwrap_or(400.0), a[1].as_f64().unwrap_or(300.0), a[2].as_f64().unwrap_or(100.0), a[3].as_f64().unwrap_or(16.0) as usize, a[4].as_f64().unwrap_or(0.0), &colors)))
    }

    fn cafe_wall(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "cafe_wall requiere [sx, sy, rows, cols, bw, bh, mortar, t]".to_string() })?;
        if a.len() < 8 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "cafe_wall requiere 8 params".to_string() }); }
        Ok(json!(illusions::cafe_wall(a[0].as_f64().unwrap_or(0.0), a[1].as_f64().unwrap_or(0.0), a[2].as_f64().unwrap_or(8.0) as usize, a[3].as_f64().unwrap_or(12.0) as usize, a[4].as_f64().unwrap_or(30.0), a[5].as_f64().unwrap_or(15.0), a[6].as_f64().unwrap_or(2.0), a[7].as_f64().unwrap_or(0.0))))
    }

    fn troxler_fading(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "troxler_fading requiere [cx, cy, num, radius, size, t]".to_string() })?;
        if a.len() < 6 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "troxler_fading requiere 6 params".to_string() }); }
        Ok(json!(illusions::troxler_fading(a[0].as_f64().unwrap_or(400.0), a[1].as_f64().unwrap_or(300.0), a[2].as_f64().unwrap_or(12.0) as usize, a[3].as_f64().unwrap_or(100.0), a[4].as_f64().unwrap_or(10.0), a[5].as_f64().unwrap_or(0.0))))
    }

    fn pulsing_star(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "pulsing_star requiere [cx, cy, outer, inner, points, t]".to_string() })?;
        if a.len() < 6 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "pulsing_star requiere 6 params".to_string() }); }
        Ok(json!(illusions::pulsing_star(a[0].as_f64().unwrap_or(400.0), a[1].as_f64().unwrap_or(300.0), a[2].as_f64().unwrap_or(50.0), a[3].as_f64().unwrap_or(25.0), a[4].as_f64().unwrap_or(5.0) as usize, a[5].as_f64().unwrap_or(0.0))))
    }

    fn zollner_effect(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "zollner_effect requiere [sx, sy, len, spacing, lines, tick_len, angle, t]".to_string() })?;
        if a.len() < 8 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "zollner_effect requiere 8 params".to_string() }); }
        Ok(json!(illusions::zollner_effect(a[0].as_f64().unwrap_or(50.0), a[1].as_f64().unwrap_or(50.0), a[2].as_f64().unwrap_or(700.0), a[3].as_f64().unwrap_or(40.0), a[4].as_f64().unwrap_or(10.0) as usize, a[5].as_f64().unwrap_or(15.0), a[6].as_f64().unwrap_or(0.5), a[7].as_f64().unwrap_or(0.0))))
    }

    fn motion_blindness(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "motion_blindness requiere [cx, cy, grid, spacing, size, t]".to_string() })?;
        if a.len() < 6 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "motion_blindness requiere 6 params".to_string() }); }
        Ok(json!(illusions::motion_induced_blindness(a[0].as_f64().unwrap_or(400.0), a[1].as_f64().unwrap_or(300.0), a[2].as_f64().unwrap_or(10.0) as usize, a[3].as_f64().unwrap_or(30.0), a[4].as_f64().unwrap_or(5.0), a[5].as_f64().unwrap_or(0.0))))
    }

    // ===== v0.10.0: EFECTOS ESPECIALES =====

    fn neon_glow(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "neon_glow requiere [cx, cy, radius, layers, spread, intensity, color, t]".to_string() })?;
        if a.len() < 8 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "neon_glow requiere 8 params".to_string() }); }
        let color = a.get(6).and_then(|v| v.as_str()).unwrap_or("#FF00FF");
        Ok(json!(effects::neon_glow(a[0].as_f64().unwrap_or(400.0), a[1].as_f64().unwrap_or(300.0), a[2].as_f64().unwrap_or(20.0), a[3].as_f64().unwrap_or(5.0) as usize, a[4].as_f64().unwrap_or(2.0), a[5].as_f64().unwrap_or(0.8), color, a[7].as_f64().unwrap_or(0.0))))
    }

    fn motion_blur(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "motion_blur requiere [prev_positions, cx, cy, intensity, fade]".to_string() })?;
        if a.len() < 5 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "motion_blur requiere 5 params".to_string() }); }
        let prev: Vec<(f64, f64)> = a[0].as_array().map(|arr| arr.iter().filter_map(|v| v.as_array().map(|p| (p[0].as_f64().unwrap_or(0.0), p[1].as_f64().unwrap_or(0.0)))).collect()).unwrap_or_default();
        Ok(json!(effects::motion_blur(&prev, (a[1].as_f64().unwrap_or(0.0), a[2].as_f64().unwrap_or(0.0)), a[3].as_f64().unwrap_or(0.8), a[4].as_f64().unwrap_or(0.8))))
    }

    fn chromatic_aberration(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "chromatic_aberration requiere [cx, cy, radius, sep, t, shape]".to_string() })?;
        if a.len() < 6 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "chromatic_aberration requiere 6 params".to_string() }); }
        let shape = a.get(5).and_then(|v| v.as_str()).unwrap_or("circle");
        Ok(json!(effects::chromatic_aberration(a[0].as_f64().unwrap_or(400.0), a[1].as_f64().unwrap_or(300.0), a[2].as_f64().unwrap_or(30.0), a[3].as_f64().unwrap_or(10.0), a[4].as_f64().unwrap_or(0.0), shape)))
    }

    fn bloom_effect(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "bloom_effect requiere [sources, radius, intensity, t]".to_string() })?;
        if a.len() < 4 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "bloom_effect requiere 4 params".to_string() }); }
        let sources: Vec<(f64, f64, f64, f64)> = a[0].as_array().map(|arr| arr.iter().filter_map(|v| v.as_array().map(|p| (p[0].as_f64().unwrap_or(0.0), p[1].as_f64().unwrap_or(0.0), p[2].as_f64().unwrap_or(1.0), p[3].as_f64().unwrap_or(10.0)))).collect()).unwrap_or_default();
        Ok(json!(effects::bloom_effect(&sources, a[1].as_f64().unwrap_or(50.0), a[2].as_f64().unwrap_or(0.8), a[3].as_f64().unwrap_or(0.0))))
    }

    fn particle_trails(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "particle_trails requiere [positions, length, fade, color]".to_string() })?;
        if a.len() < 4 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "particle_trails requiere 4 params".to_string() }); }
        let positions: Vec<(f64, f64, f64, f64)> = a[0].as_array().map(|arr| arr.iter().filter_map(|v| v.as_array().map(|p| (p[0].as_f64().unwrap_or(0.0), p[1].as_f64().unwrap_or(0.0), p[2].as_f64().unwrap_or(0.0), p[3].as_f64().unwrap_or(0.0)))).collect()).unwrap_or_default();
        let color = a.get(3).and_then(|v| v.as_str()).unwrap_or("#FFAA00");
        Ok(json!(effects::particle_trails(&positions, a[1].as_f64().unwrap_or(10.0) as usize, a[2].as_f64().unwrap_or(0.85), color)))
    }

    fn morph_shapes(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "morph_shapes requiere [shape_a, shape_b, t, easing]".to_string() })?;
        if a.len() < 4 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "morph_shapes requiere 4 params".to_string() }); }
        let sa: Vec<(f64, f64)> = a[0].as_array().map(|arr| arr.iter().filter_map(|v| v.as_array().map(|p| (p[0].as_f64().unwrap_or(0.0), p[1].as_f64().unwrap_or(0.0)))).collect()).unwrap_or_default();
        let sb: Vec<(f64, f64)> = a[1].as_array().map(|arr| arr.iter().filter_map(|v| v.as_array().map(|p| (p[0].as_f64().unwrap_or(0.0), p[1].as_f64().unwrap_or(0.0)))).collect()).unwrap_or_default();
        let easing = a.get(3).and_then(|v| v.as_str()).unwrap_or("linear");
        Ok(json!(effects::morph_shapes(&sa, &sb, a[2].as_f64().unwrap_or(0.0), easing)))
    }

    // ===== v0.11.0: CIENCIA ANIMADA =====

    fn chemical_crystallization(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "chemical_crystallization requiere [cx, cy, num, radius, t, growth]".to_string() })?;
        if a.len() < 6 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "chemical_crystallization requiere 6 params".to_string() }); }
        Ok(json!(science_anim::chemical_crystallization(a[0].as_f64().unwrap_or(400.0), a[1].as_f64().unwrap_or(300.0), a[2].as_f64().unwrap_or(12.0) as usize, a[3].as_f64().unwrap_or(100.0), a[4].as_f64().unwrap_or(0.0), a[5].as_f64().unwrap_or(1.5))))
    }

    fn cell_division(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "cell_division requiere [cx, cy, radius, div_time, max_div, t]".to_string() })?;
        if a.len() < 6 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "cell_division requiere 6 params".to_string() }); }
        Ok(json!(science_anim::cell_division(a[0].as_f64().unwrap_or(400.0), a[1].as_f64().unwrap_or(300.0), a[2].as_f64().unwrap_or(30.0), a[3].as_f64().unwrap_or(1.0), a[4].as_f64().unwrap_or(3.0) as usize, a[5].as_f64().unwrap_or(0.0))))
    }

    fn walk_cycle(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "walk_cycle requiere [cx, cy, body, legs, stride, t, phase]".to_string() })?;
        if a.len() < 7 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "walk_cycle requiere 7 params".to_string() }); }
        Ok(json!(science_anim::walk_cycle(a[0].as_f64().unwrap_or(400.0), a[1].as_f64().unwrap_or(300.0), a[2].as_f64().unwrap_or(20.0), a[3].as_f64().unwrap_or(4.0) as usize, a[4].as_f64().unwrap_or(15.0), a[5].as_f64().unwrap_or(0.0), a[6].as_f64().unwrap_or(0.25))))
    }

    fn flight_pattern(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "flight_pattern requiere [cx, cy, wingspan, flap_speed, t]".to_string() })?;
        if a.len() < 5 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "flight_pattern requiere 5 params".to_string() }); }
        Ok(json!(science_anim::flight_pattern(a[0].as_f64().unwrap_or(400.0), a[1].as_f64().unwrap_or(300.0), a[2].as_f64().unwrap_or(80.0), a[3].as_f64().unwrap_or(5.0), a[4].as_f64().unwrap_or(0.0))))
    }

    fn lsystem_tree(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "lsystem_tree requiere [bx, by, trunk, angle, ratio, depth, t]".to_string() })?;
        if a.len() < 7 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "lsystem_tree requiere 7 params".to_string() }); }
        Ok(json!(science_anim::lsystem_tree(a[0].as_f64().unwrap_or(400.0), a[1].as_f64().unwrap_or(500.0), a[2].as_f64().unwrap_or(80.0), a[3].as_f64().unwrap_or(0.5), a[4].as_f64().unwrap_or(0.7), a[5].as_f64().unwrap_or(4.0) as usize, a[6].as_f64().unwrap_or(0.0))))
    }

    fn tusi_couple(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "tusi_couple requiere [cx, cy, large_radius, t]".to_string() })?;
        if a.len() < 4 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "tusi_couple requiere 4 params".to_string() }); }
        Ok(json!(science_anim::tusi_couple(a[0].as_f64().unwrap_or(400.0), a[1].as_f64().unwrap_or(300.0), a[2].as_f64().unwrap_or(100.0), a[3].as_f64().unwrap_or(0.0))))
    }

    fn pendulum_waves(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "pendulum_waves requiere [base_x, base_y, num, length, freq_spread, t]".to_string() })?;
        if a.len() < 6 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "pendulum_waves requiere 6 params".to_string() }); }
        Ok(json!(science_anim::pendulum_waves(a[0].as_f64().unwrap_or(400.0), a[1].as_f64().unwrap_or(100.0), a[2].as_f64().unwrap_or(12.0) as usize, a[3].as_f64().unwrap_or(100.0), a[4].as_f64().unwrap_or(0.05), a[5].as_f64().unwrap_or(0.0))))
    }

    fn wave_interference(&self, p: Value) -> ModuleResult {
        let a = p.as_array().ok_or_else(|| ModuleError { code: "INVALID_PARAMS".to_string(), message: "wave_interference requiere [cx1, cy1, cx2, cy2, wavelength, amp, grid, t]".to_string() })?;
        if a.len() < 8 { return Err(ModuleError { code: "INVALID_PARAMS".to_string(), message: "wave_interference requiere 8 params".to_string() }); }
        Ok(json!(science_anim::wave_interference(a[0].as_f64().unwrap_or(250.0), a[1].as_f64().unwrap_or(300.0), a[2].as_f64().unwrap_or(550.0), a[3].as_f64().unwrap_or(300.0), a[4].as_f64().unwrap_or(40.0), a[5].as_f64().unwrap_or(1.0), a[6].as_f64().unwrap_or(15.0) as usize, a[7].as_f64().unwrap_or(0.0))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anim_module_name() {
        let m = AnimModule;
        assert_eq!(m.name(), "anim");
        assert_eq!(m.version(), "0.11.0");
    }

    #[test]
    fn test_anim_register() {
        let m = AnimModule;
        let cmds = m.register();
        assert!(cmds.contains_key("ease_in"));
        assert!(cmds.contains_key("follow_through"));
        assert!(cmds.contains_key("arc_path"));
        assert!(cmds.contains_key("pose_to_pose"));
    }

    #[test]
    fn test_ease_in() {
        let m = AnimModule;
        let r = m.execute("ease_in", json!([0.5])).unwrap();
        assert!((r.as_f64().unwrap() - 0.25).abs() < 0.001);
    }

    #[test]
    fn test_ease_out() {
        let m = AnimModule;
        let r = m.execute("ease_out", json!([0.5])).unwrap();
        assert!((r.as_f64().unwrap() - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_ease_in_out() {
        let m = AnimModule;
        let r = m.execute("ease_in_out", json!([0.5])).unwrap();
        assert!((r.as_f64().unwrap() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_squash() {
        let m = AnimModule;
        let r = m.execute("squash", json!([2.0])).unwrap();
        let arr = r.as_array().unwrap();
        assert!((arr[0].as_f64().unwrap() - 2.0).abs() < 0.001);
        assert!((arr[1].as_f64().unwrap() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_stretch() {
        let m = AnimModule;
        let r = m.execute("stretch", json!([2.0])).unwrap();
        let arr = r.as_array().unwrap();
        assert!((arr[0].as_f64().unwrap() - 0.5).abs() < 0.001);
        assert!((arr[1].as_f64().unwrap() - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_anticipate() {
        let m = AnimModule;
        let r = m.execute("anticipate", json!([100.0, 200.0, 20.0])).unwrap();
        assert!((r.as_f64().unwrap() - 80.0).abs() < 0.001);
    }

    #[test]
    fn test_follow_through() {
        let m = AnimModule;
        let r = m.execute("follow_through", json!([1.0, 1.0, 5.0, 0.1])).unwrap();
        assert!(r.as_f64().unwrap().abs() > 0.0);
    }

    #[test]
    fn test_arc_path() {
        let m = AnimModule;
        let r = m.execute("arc_path", json!([0.0, 0.0, 10.0, 0.0, 5.0, 0.5])).unwrap();
        let arr = r.as_array().unwrap();
        assert!((arr[0].as_f64().unwrap() - 5.0).abs() < 0.1);
        assert!(arr[1].as_f64().unwrap() > 0.0);
    }

    #[test]
    fn test_secondary_action() {
        let m = AnimModule;
        let r = m.execute("secondary_action", json!([1.0, 0.2, 0.5, 0.5])).unwrap();
        let arr = r.as_array().unwrap();
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn test_timing() {
        let m = AnimModule;
        let r = m.execute("timing", json!([[[0.0, 0.0], [10.0, 100.0]], 5.0])).unwrap();
        let v = r.as_f64().unwrap();
        assert!(v > 40.0 && v < 60.0);
    }

    #[test]
    fn test_exaggerate() {
        let m = AnimModule;
        let r = m.execute("exaggerate", json!([1.0, 2.0, 0.5])).unwrap();
        assert!((r.as_f64().unwrap() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_solid_rotation() {
        let m = AnimModule;
        let r = m.execute("solid_rotation", json!([0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 60.0])).unwrap();
        let arr = r.as_array().unwrap();
        assert!((arr[0].as_f64().unwrap() - 0.0).abs() < 0.01);
        assert!(arr[2].as_f64().unwrap() > 0.0);
    }

    #[test]
    fn test_appeal() {
        let m = AnimModule;
        let r = m.execute("appeal", json!([10.0, 10.0, 0.5, 1.0])).unwrap();
        let arr = r.as_array().unwrap();
        assert!(arr[0].as_f64().unwrap() > 10.0);
    }

    #[test]
    fn test_pose_to_pose() {
        let m = AnimModule;
        let r = m.execute("pose_to_pose", json!([[[0.0, 0.0, 0.0, 1.0, 0.0], [1.0, 10.0, 5.0, 1.5, 0.5]], 0.5])).unwrap();
        let arr = r.as_array().unwrap();
        assert!(arr[0].as_f64().unwrap() > 2.0 && arr[0].as_f64().unwrap() < 8.0);
    }

    #[test]
    fn test_unknown_command() {
        let m = AnimModule;
        assert!(m.execute("unknown", json!([])).is_err());
    }
}
