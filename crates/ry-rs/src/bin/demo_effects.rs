// Demo: Efectos Especiales
// Uso: cargo run --bin demo_effects --release

use ry_gfx::{ColorRydit, RyditGfx};
use ry_anim::effects;

fn main() {
    let mut gfx = RyditGfx::new("Efectos Especiales - Ry-Dit", 800, 600);
    gfx.set_target_fps(30);

    let mut current = 0;
    let mut t = 0.0;
    let names = [
        "Neon Glow",
        "Motion Blur",
        "Chromatic Aberration",
        "Bloom",
        "Particle Trails",
        "Morphing",
    ];

    // Particulas para trails
    let mut particles: Vec<(f64, f64, f64, f64)> = Vec::new();
    for i in 0..5 {
        let angle = i as f64 * 1.2;
        particles.push((400.0, 300.0, angle.cos() * 3.0, angle.sin() * 3.0));
    }

    println!("[DEMO] Efectos Especiales - 1-6 cambiar, ESC salir");

    while !gfx.should_close() {
        for (i, key) in [ry_gfx::Key::Num1, ry_gfx::Key::Num2, ry_gfx::Key::Num3,
                         ry_gfx::Key::Num4, ry_gfx::Key::Num5, ry_gfx::Key::Num6].iter().enumerate() {
            if gfx.is_key_pressed(*key) {
                current = i;
                t = 0.0;
            }
        }
        if gfx.is_key_pressed(ry_gfx::Key::Escape) { break; }

        t += 0.016;

        // Actualizar particulas
        for p in &mut particles {
            p.0 += p.2;
            p.1 += p.3;
            // Rebotar en bordes
            if p.0 < 50.0 || p.0 > 750.0 { p.2 *= -1.0; }
            if p.1 < 50.0 || p.1 > 550.0 { p.3 *= -1.0; }
        }

        {
            let mut d = gfx.begin_draw();
            d.clear(ColorRydit::Negro);

            match current {
                0 => draw_neon(&mut d, t),
                1 => draw_blur(&mut d, t),
                2 => draw_chromatic(&mut d, t),
                3 => draw_bloom(&mut d, t),
                4 => draw_trails(&mut d, t, &particles),
                5 => draw_morph(&mut d, t),
                _ => {}
            }

            d.draw_text(&format!("{} (t={:.1})", names[current], t), 10, 15, 16, ColorRydit::Blanco);
            d.draw_text("1-6 cambiar | ESC salir", 10, 575, 12, ColorRydit::Gris);
        }
    }
    println!("[DEMO] Efectos completados");
}

fn draw_neon(d: &mut ry_gfx::DrawHandle, t: f64) {
    let cx = 400.0 + 100.0 * (t * 0.7).sin();
    let cy = 300.0 + 80.0 * (t * 1.1).cos();
    let layers = effects::neon_glow(cx, cy, 15.0, 6, 2.2, 0.9, "#FF00FF", t);

    for layer in &layers {
        let (Some(x), Some(y), Some(r)) = (
            layer.get("x").and_then(|v| v.as_f64()),
            layer.get("y").and_then(|v| v.as_f64()),
            layer.get("radius").and_then(|v| v.as_f64()),
        ) else { continue };
        let _alpha = layer.get("alpha").and_then(|v| v.as_f64()).unwrap_or(1.0);
        let c = if layer.get("type").and_then(|v| v.as_str()) == Some("core") {
            ColorRydit::Blanco
        } else {
            ColorRydit::Magenta
        };
        d.draw_circle(x as i32, y as i32, r as i32, c);
    }
}

fn draw_blur(d: &mut ry_gfx::DrawHandle, t: f64) {
    // Simular movimiento circular
    let mut prev: Vec<(f64, f64)> = Vec::new();
    for i in 1..=12 {
        let angle = t * 2.0 - i as f64 * 0.15;
        prev.push((400.0 + 150.0 * angle.cos(), 300.0 + 100.0 * angle.sin()));
    }
    let cx = 400.0 + 150.0 * (t * 2.0).cos();
    let cy = 300.0 + 100.0 * (t * 2.0).sin();

    let blurs = effects::motion_blur(&prev, (cx, cy), 0.8, 0.85);

    for b in &blurs {
        let (Some(x), Some(y)) = (b.get("x").and_then(|v| v.as_f64()), b.get("y").and_then(|v| v.as_f64())) else { continue };
        let _alpha = b.get("alpha").and_then(|v| v.as_f64()).unwrap_or(1.0);
        let r = if b.get("type").and_then(|v| v.as_str()) == Some("sharp") { 12 } else { 14 };
        d.draw_circle(x as i32, y as i32, r, ColorRydit::Cyan);
    }
}

fn draw_chromatic(d: &mut ry_gfx::DrawHandle, t: f64) {
    let channels = effects::chromatic_aberration(400.0, 300.0, 60.0, 15.0, t * 0.5, "circle");

    for ch in &channels {
        let (Some(x), Some(y), Some(r)) = (
            ch.get("x").and_then(|v| v.as_f64()),
            ch.get("y").and_then(|v| v.as_f64()),
            ch.get("radius").and_then(|v| v.as_f64()),
        ) else { continue };
        let c = match ch.get("channel").and_then(|v| v.as_str()) {
            Some("red") => ColorRydit::Rojo,
            Some("green") => ColorRydit::Verde,
            _ => ColorRydit::Azul,
        };
        d.draw_circle(x as i32, y as i32, r as i32, c);
    }
}

fn draw_bloom(d: &mut ry_gfx::DrawHandle, t: f64) {
    // Fuentes de luz moviendose
    let sources: Vec<(f64, f64, f64, f64)> = (0..5).map(|i| {
        let angle = t * 0.8 + i as f64 * 1.25;
        let x = 400.0 + 200.0 * angle.cos();
        let y = 300.0 + 150.0 * angle.sin();
        let intensity = 0.5 + 0.5 * (t * 2.0 + i as f64).sin().abs();
        (x, y, intensity, 8.0)
    }).collect();

    let blooms = effects::bloom_effect(&sources, 60.0, 0.7, t);

    for b in &blooms {
        let (Some(x), Some(y), Some(r)) = (
            b.get("x").and_then(|v| v.as_f64()),
            b.get("y").and_then(|v| v.as_f64()),
            b.get("radius").and_then(|v| v.as_f64()),
        ) else { continue };
        let c = if b.get("type").and_then(|v| v.as_str()) == Some("bloom_halo") {
            ColorRydit::Amarillo
        } else {
            ColorRydit::Blanco
        };
        d.draw_circle(x as i32, y as i32, r as i32, c);
    }
}

fn draw_trails(d: &mut ry_gfx::DrawHandle, _t: f64, particles: &[(f64, f64, f64, f64)]) {
    let trails = effects::particle_trails(particles, 15, 0.88, "#FF6600");

    for pt in &trails {
        let (Some(x), Some(y), Some(s)) = (
            pt.get("x").and_then(|v| v.as_f64()),
            pt.get("y").and_then(|v| v.as_f64()),
            pt.get("size").and_then(|v| v.as_f64()),
        ) else { continue };
        let c = if pt.get("type").and_then(|v| v.as_str()) == Some("particle") {
            ColorRydit::Blanco
        } else {
            ColorRydit::Naranja
        };
        d.draw_circle(x as i32, y as i32, s as i32, c);
    }
}

fn draw_morph(d: &mut ry_gfx::DrawHandle, t: f64) {
    // Triangulo → Cuadrado → Triangulo
    let triangle = vec![(0.0, -80.0), (70.0, 60.0), (-70.0, 60.0)];
    let square = vec![(-60.0, -60.0), (60.0, -60.0), (60.0, 60.0), (-60.0, 60.0)];

    let morph_t = (t * 0.5).sin() * 0.5 + 0.5; // Oscila entre 0 y 1
    let points = effects::morph_shapes(&triangle, &square, morph_t, "ease_in_out");

    // Dibujar líneas entre puntos consecutivos
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        let (Some(x1), Some(x2)) = (
            points[i].get("x").and_then(|v| v.as_f64()),
            points[j].get("x").and_then(|v| v.as_f64()),
        ) else { continue };
        let y1 = points[i].get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let y2 = points[j].get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
        d.draw_line(400 + x1 as i32, 300 + y1 as i32, 400 + x2 as i32, 300 + y2 as i32, ColorRydit::Verde);
    }

    // Labels
    d.draw_text(&format!("Morph: {:.0}%", morph_t * 100.0), 350, 500, 14, ColorRydit::Blanco);
}
