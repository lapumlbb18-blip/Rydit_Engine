// Demo Anime Ry - Showcase completo de ry-anim
// Binario de prueba para publicacion de crate
// Uso: cargo run --bin demo_anime_ry --release

use ry_gfx::{ColorRydit, RyditGfx};
use ry_anim::{disney, illusions, effects, science_anim, action_assets};

fn main() {
    let mut gfx = RyditGfx::new("Anime Ry - ry-anim Showcase", 800, 600);
    gfx.set_target_fps(30);

    let mut scene = 0;
    let mut t = 0.0;
    let scenes = [
        "Disney: Follow Through",
        "Disney: Arcs",
        "Disney: Solid Drawing 3D",
        "Illusion: Rotating Snakes",
        "Illusion: Cafe Wall",
        "Effect: Neon Glow",
        "Effect: Chromatic Aberration",
        "Effect: Morphing",
        "Science: Tusi Couple",
        "Science: Pendulum Waves",
        "Science: L-System Tree",
        "Science: Wave Interference",
        "Action: State Machine",
        "Action: Sprite Flip",
        "Action: Frame Animation",
    ];

    println!("[ANIME RY] Showcase de ry-anim v0.12.0");
    println!("[ANIME RY] ESPACIO = siguiente escena | ESC salir");

    while !gfx.should_close() {
        if gfx.is_key_pressed(ry_gfx::Key::Space) {
            scene = (scene + 1) % scenes.len();
            t = 0.0;
        }
        if gfx.is_key_pressed(ry_gfx::Key::Escape) { break; }

        t += 0.016;

        {
            let mut d = gfx.begin_draw();
            d.clear(ColorRydit::Negro);

            match scene {
                0 => scene_follow_through(&mut d, t),
                1 => scene_arcs(&mut d, t),
                2 => scene_solid_drawing(&mut d, t),
                3 => scene_rotating_snakes(&mut d, t),
                4 => scene_cafe_wall(&mut d, t),
                5 => scene_neon_glow(&mut d, t),
                6 => scene_chromatic(&mut d, t),
                7 => scene_morphing(&mut d, t),
                8 => scene_tusi(&mut d, t),
                9 => scene_pendulums(&mut d, t),
                10 => scene_lsystem(&mut d, t),
                11 => scene_wave_interference(&mut d, t),
                12 => scene_state_machine(&mut d, t),
                13 => scene_sprite_flip(&mut d, t),
                14 => scene_frame_animation(&mut d, t),
                _ => {}
            }

            // HUD
            d.draw_text(&format!("[{}] {}", scene + 1, scenes[scene]), 10, 15, 16, ColorRydit::Blanco);
            d.draw_text(&format!("t = {:.2}s", t), 10, 35, 12, ColorRydit::Gris);
            d.draw_text("ESPACIO = siguiente | ESC = salir", 10, 575, 12, ColorRydit::Gris);
        }
    }

    println!("[ANIME RY] Showcase completado");
    println!("[ANIME RY] ry-anim v0.12.0 - 41 funciones | 58 tests");
}

// ==================== DISNEY ====================

fn scene_follow_through(d: &mut ry_gfx::DrawHandle, t: f64) {
    let cx = 400.0;
    for i in 0..5 {
        let offset = disney::follow_through(30.0, 1.5, 8.0, t - i as f64 * 0.1);
        let x = cx + offset;
        let y = 300.0 + i as f64 * 20.0;
        let size = (15 - i * 2) as i32;
        d.draw_circle(x as i32, y as i32, size.max(2), ColorRydit::Verde);
    }
    d.draw_text("Follow Through: partes siguen moviendose", 100, 100, 14, ColorRydit::Blanco);
}

fn scene_arcs(d: &mut ry_gfx::DrawHandle, t: f64) {
    let start = (100.0, 400.0);
    let end = (700.0, 400.0);
    for i in 0..6 {
        let curve = (i as f64 - 2.5) * 60.0;
        let (x, y) = disney::arc_path(start, end, curve, (t * 0.3 + i as f64 * 0.1) % 1.0);
        d.draw_circle(x as i32, y as i32, 8, ColorRydit::Amarillo);
    }
    // Dibujar linea base
    d.draw_line(100, 400, 700, 400, ColorRydit::Gris);
    d.draw_text("Arcs: trayectoria curva entre puntos", 100, 100, 14, ColorRydit::Blanco);
}

fn scene_solid_drawing(d: &mut ry_gfx::DrawHandle, t: f64) {
    let points = vec![
        (0.0, -30.0, 0.0), (25.0, 20.0, 0.0), (-25.0, 20.0, 0.0),
    ];
    let rot = (t * 0.5, t * 0.3, 0.0);
    for p in &points {
        let (sx, sy, scale) = disney::solid_rotation(*p, rot, 60.0);
        d.draw_circle((400.0 + sx * scale) as i32, (300.0 + sy * scale) as i32, (10.0 * scale) as i32, ColorRydit::Cyan);
    }
    d.draw_text("Solid Drawing: rotacion 3D con perspectiva", 100, 100, 14, ColorRydit::Blanco);
}

// ==================== ILUSIONES ====================

fn scene_rotating_snakes(d: &mut ry_gfx::DrawHandle, t: f64) {
    let colors = vec!["#000000".to_string(), "#FFFFFF".to_string(), "#0000FF".to_string(), "#FFFF00".to_string()];
    let segs = illusions::rotating_snakes(400.0, 300.0, 120.0, 24, t, &colors);
    for s in &segs {
        let (Some(x), Some(y)) = (s.get("x").and_then(|v| v.as_f64()), s.get("y").and_then(|v| v.as_f64())) else { continue };
        d.draw_circle(x as i32, y as i32, 8, ColorRydit::Amarillo);
    }
    d.draw_text("Rotating Snakes: ilusion de movimiento circular", 80, 100, 14, ColorRydit::Blanco);
}

fn scene_cafe_wall(d: &mut ry_gfx::DrawHandle, t: f64) {
    let elems = illusions::cafe_wall(60.0, 60.0, 6, 8, 40.0, 20.0, 2.0, (t * 0.2).sin() * 0.5 + 0.5);
    for e in &elems {
        let Some(tp) = e.get("type").and_then(|v| v.as_str()) else { continue };
        if tp == "rect" {
            let (Some(x), Some(y), Some(w), Some(h)) = (e.get("x").and_then(|v| v.as_f64()), e.get("y").and_then(|v| v.as_f64()), e.get("w").and_then(|v| v.as_f64()), e.get("h").and_then(|v| v.as_f64())) else { continue };
            let c = if e.get("color").and_then(|v| v.as_str()) == Some("#FFFFFF") { ColorRydit::Blanco } else { ColorRydit::Negro };
            d.draw_rectangle(x as i32, y as i32, w as i32, h as i32, c);
        }
    }
    d.draw_text("Cafe Wall: lineas paralelas que parecen inclinadas", 80, 100, 14, ColorRydit::Blanco);
}

// ==================== EFECTOS ====================

fn scene_neon_glow(d: &mut ry_gfx::DrawHandle, t: f64) {
    let layers = effects::neon_glow(400.0, 300.0, 15.0, 6, 2.0, 0.9, "#FF00FF", t);
    for l in &layers {
        let (Some(x), Some(y), Some(r)) = (l.get("x").and_then(|v| v.as_f64()), l.get("y").and_then(|v| v.as_f64()), l.get("radius").and_then(|v| v.as_f64())) else { continue };
        let c = if l.get("type").and_then(|v| v.as_str()) == Some("core") { ColorRydit::Blanco } else { ColorRydit::Magenta };
        d.draw_circle(x as i32, y as i32, r as i32, c);
    }
    d.draw_text("Neon Glow: resplandor neon configurable", 100, 100, 14, ColorRydit::Blanco);
}

fn scene_chromatic(d: &mut ry_gfx::DrawHandle, t: f64) {
    let channels = effects::chromatic_aberration(400.0, 300.0, 50.0, 15.0, t * 0.5, "circle");
    for ch in &channels {
        let (Some(x), Some(y), Some(r)) = (ch.get("x").and_then(|v| v.as_f64()), ch.get("y").and_then(|v| v.as_f64()), ch.get("radius").and_then(|v| v.as_f64())) else { continue };
        let c = match ch.get("channel").and_then(|v| v.as_str()) {
            Some("red") => ColorRydit::Rojo,
            Some("green") => ColorRydit::Verde,
            _ => ColorRydit::Azul,
        };
        d.draw_circle(x as i32, y as i32, r as i32, c);
    }
    d.draw_text("Chromatic Aberration: separacion RGB", 100, 100, 14, ColorRydit::Blanco);
}

fn scene_morphing(d: &mut ry_gfx::DrawHandle, t: f64) {
    let tri = vec![(0.0, -60.0), (50.0, 40.0), (-50.0, 40.0)];
    let sq = vec![(-40.0, -40.0), (40.0, -40.0), (40.0, 40.0), (-40.0, 40.0)];
    let mt = (t * 0.4).sin() * 0.5 + 0.5;
    let pts = effects::morph_shapes(&tri, &sq, mt, "ease_in_out");
    for i in 0..pts.len() {
        let j = (i + 1) % pts.len();
        let (Some(x1), Some(x2)) = (pts[i].get("x").and_then(|v| v.as_f64()), pts[j].get("x").and_then(|v| v.as_f64())) else { continue };
        let y1 = pts[i].get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let y2 = pts[j].get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
        d.draw_line(400 + x1 as i32, 300 + y1 as i32, 400 + x2 as i32, 300 + y2 as i32, ColorRydit::Verde);
    }
    d.draw_text(&format!("Morphing: triangulo <-> cuadrado ({:.0}%)", mt * 100.0), 80, 100, 14, ColorRydit::Blanco);
}

// ==================== CIENCIA ====================

fn scene_tusi(d: &mut ry_gfx::DrawHandle, t: f64) {
    let elems = science_anim::tusi_couple(400.0, 300.0, 120.0, t * 0.8);
    for e in &elems {
        let Some(tp) = e.get("type").and_then(|v| v.as_str()) else { continue };
        match tp {
            "large_circle" => {
                let (Some(x), Some(y), Some(r)) = (e.get("x").and_then(|v| v.as_f64()), e.get("y").and_then(|v| v.as_f64()), e.get("radius").and_then(|v| v.as_f64())) else { continue };
                d.draw_circle(x as i32, y as i32, r as i32, ColorRydit::Gris);
            }
            "small_circle" => {
                let (Some(x), Some(y), Some(r)) = (e.get("x").and_then(|v| v.as_f64()), e.get("y").and_then(|v| v.as_f64()), e.get("radius").and_then(|v| v.as_f64())) else { continue };
                d.draw_circle(x as i32, y as i32, r as i32, ColorRydit::Azul);
            }
            "point" => {
                let (Some(x), Some(y)) = (e.get("x").and_then(|v| v.as_f64()), e.get("y").and_then(|v| v.as_f64())) else { continue };
                d.draw_circle(x as i32, y as i32, 8, ColorRydit::Rojo);
            }
            "trace_line" => {
                let (Some(x1), Some(y1), Some(x2), Some(y2)) = (e.get("x1").and_then(|v| v.as_f64()), e.get("y1").and_then(|v| v.as_f64()), e.get("x2").and_then(|v| v.as_f64()), e.get("y2").and_then(|v| v.as_f64())) else { continue };
                d.draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, ColorRydit::Rojo);
            }
            _ => {}
        }
    }
    d.draw_text("Pareja de Tusi: movimiento lineal desde circular (~1250 d.C.)", 50, 100, 14, ColorRydit::Blanco);
}

fn scene_pendulums(d: &mut ry_gfx::DrawHandle, t: f64) {
    let pendulums = science_anim::pendulum_waves(400.0, 80.0, 16, 120.0, 0.06, t);
    for p in &pendulums {
        let (Some(x1), Some(y1), Some(x2), Some(y2)) = (p.get("x1").and_then(|v| v.as_f64()), p.get("y1").and_then(|v| v.as_f64()), p.get("x2").and_then(|v| v.as_f64()), p.get("y2").and_then(|v| v.as_f64())) else { continue };
        d.draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, ColorRydit::Blanco);
        d.draw_circle(x2 as i32, y2 as i32, 6, ColorRydit::Amarillo);
    }
    d.draw_text("Pendulum Waves: ondas de pendulos", 100, 100, 14, ColorRydit::Blanco);
}

fn scene_lsystem(d: &mut ry_gfx::DrawHandle, t: f64) {
    let branches = science_anim::lsystem_tree(400.0, 520.0, 80.0, 0.5, 0.7, 5, (t * 0.08).min(1.0));
    for b in &branches {
        let (Some(x1), Some(y1), Some(x2), Some(y2)) = (b.get("x1").and_then(|v| v.as_f64()), b.get("y1").and_then(|v| v.as_f64()), b.get("x2").and_then(|v| v.as_f64()), b.get("y2").and_then(|v| v.as_f64())) else { continue };
        let c = if b.get("depth").and_then(|v| v.as_f64()).unwrap_or(0.0) > 2.0 { ColorRydit::Cafe } else { ColorRydit::Verde };
        d.draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, c);
    }
    d.draw_text("L-System Tree: arbol fractal animado", 100, 100, 14, ColorRydit::Blanco);
}

fn scene_wave_interference(d: &mut ry_gfx::DrawHandle, t: f64) {
    let waves = science_anim::wave_interference(250.0, 300.0, 550.0, 300.0, 50.0, 1.0, 18, t);
    for w in &waves {
        let Some(tp) = w.get("type").and_then(|v| v.as_str()) else { continue };
        if tp == "wave_point" {
            let (Some(x), Some(y)) = (w.get("x").and_then(|v| v.as_f64()), w.get("y").and_then(|v| v.as_f64())) else { continue };
            let amp = w.get("amplitude").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let c = if amp > 0.3 { ColorRydit::Rojo } else if amp < -0.3 { ColorRydit::Azul } else { ColorRydit::Gris };
            d.draw_circle(x as i32, y as i32, 3, c);
        } else if tp == "source" {
            let (Some(x), Some(y)) = (w.get("x").and_then(|v| v.as_f64()), w.get("y").and_then(|v| v.as_f64())) else { continue };
            d.draw_circle(x as i32, y as i32, 10, ColorRydit::Amarillo);
        }
    }
    d.draw_text("Wave Interference: interferencia de ondas", 100, 100, 14, ColorRydit::Blanco);
}

// ==================== ACTION ASSETS ====================

fn scene_state_machine(d: &mut ry_gfx::DrawHandle, t: f64) {
    let states = vec!["idle".to_string(), "walk".to_string(), "run".to_string(), "jump".to_string()];
    let durations = vec![2.0, 1.5, 1.0, 0.5];
    let cycle = (t * 0.3) as usize % 4;
    let st = t % durations[cycle];
    let result = action_assets::animation_state_machine(&states[cycle], &states, &durations, st, "");
    let progress = result.get("progress").and_then(|v| v.as_f64()).unwrap_or(0.0);

    d.draw_rectangle(100, 250, 600, 30, ColorRydit::Gris);
    d.draw_rectangle(100, 250, (600.0 * progress) as i32, 30, ColorRydit::Verde);
    d.draw_text(&format!("Estado: {} ({:.0}%)", states[cycle], progress * 100.0), 100, 200, 16, ColorRydit::Blanco);

    for (i, s) in states.iter().enumerate() {
        let c = if i == cycle { ColorRydit::Amarillo } else { ColorRydit::Gris };
        d.draw_text(&format!("{} ({:.1}s)", s, durations[i]), 100, 320 + i as i32 * 25, 14, c);
    }
}

fn scene_sprite_flip(d: &mut ry_gfx::DrawHandle, t: f64) {
    let hflip = (t * 0.5).sin() > 0.0;
    let vflip = (t * 0.3).cos() > 0.0;
    let result = action_assets::sprite_flip(hflip, vflip, 0.5, 0.5);
    let sx = result.get("scale_x").and_then(|v| v.as_f64()).unwrap_or(1.0);
    let sy = result.get("scale_y").and_then(|v| v.as_f64()).unwrap_or(1.0);

    let w = (60.0 * sx.abs()) as i32;
    let h = (80.0 * sy.abs()) as i32;
    d.draw_rectangle(370, 260, w, h, ColorRydit::Azul);
    d.draw_text(&format!("Flip H: {} | Flip V: {}", hflip, vflip), 100, 100, 16, ColorRydit::Blanco);
    d.draw_text(&format!("Scale X: {:.1} | Scale Y: {:.1}", sx, sy), 100, 130, 14, ColorRydit::Gris);
}

fn scene_frame_animation(d: &mut ry_gfx::DrawHandle, t: f64) {
    let modes = ["loop", "once", "ping_pong"];
    let colors = [ColorRydit::Verde, ColorRydit::Amarillo, ColorRydit::Rojo];
    for (i, (mode, color)) in modes.iter().zip(colors.iter()).enumerate() {
        let result = action_assets::frame_animation(8, 0.2, t, mode);
        let frame = result.get("current_frame").and_then(|v| v.as_u64()).unwrap_or(0) as i32;
        let y = 150 + i as i32 * 100;
        for f in 0..8i32 {
            let x = 100 + f * 40;
            let c = if f == frame { *color } else { ColorRydit::Gris };
            d.draw_rectangle(x, y, 30, 30, c);
        }
        d.draw_text(mode, 100, y - 20, 12, ColorRydit::Blanco);
    }
}
