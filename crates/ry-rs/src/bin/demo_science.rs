// Demo: Ciencia Animada
// Uso: cargo run --bin demo_science --release

use ry_gfx::{ColorRydit, RyditGfx};
use ry_anim::science_anim;

fn main() {
    let mut gfx = RyditGfx::new("Ciencia Animada - Ry-Dit", 800, 600);
    gfx.set_target_fps(30);

    let mut current = 0;
    let mut t = 0.0;
    let names = [
        "Cristalizacion Quimica",
        "Division Celular",
        "Ciclo de Caminata",
        "Aleteo de Ave",
        "Arbol L-System",
        "Pareja de Tusi",
        "Ondas de Pendulos",
        "Interferencia de Ondas",
    ];

    println!("[DEMO] Ciencia Animada - 1-8 cambiar, ESC salir");

    while !gfx.should_close() {
        for (i, key) in [ry_gfx::Key::Num1, ry_gfx::Key::Num2, ry_gfx::Key::Num3, ry_gfx::Key::Num4,
                         ry_gfx::Key::Num5, ry_gfx::Key::Num6, ry_gfx::Key::Num7, ry_gfx::Key::Num8].iter().enumerate() {
            if gfx.is_key_pressed(*key) {
                current = i;
                t = 0.0;
            }
        }
        if gfx.is_key_pressed(ry_gfx::Key::Escape) { break; }

        t += 0.016;

        {
            let mut d = gfx.begin_draw();
            d.clear(ColorRydit::Negro);

            match current {
                0 => draw_crystallization(&mut d, t),
                1 => draw_cell_division(&mut d, t),
                2 => draw_walk_cycle(&mut d, t),
                3 => draw_flight(&mut d, t),
                4 => draw_tree(&mut d, t),
                5 => draw_tusi(&mut d, t),
                6 => draw_pendulums(&mut d, t),
                7 => draw_waves(&mut d, t),
                _ => {}
            }

            d.draw_text(&format!("{} (t={:.1})", names[current], t), 10, 15, 14, ColorRydit::Blanco);
            d.draw_text("1-8 cambiar | ESC salir", 10, 575, 12, ColorRydit::Gris);
        }
    }
    println!("[DEMO] Ciencia completada");
}

fn draw_crystallization(d: &mut ry_gfx::DrawHandle, t: f64) {
    let crystals = science_anim::chemical_crystallization(400.0, 300.0, 16, 120.0, (t * 0.2).min(1.0), 1.5);
    for c in &crystals {
        let (Some(x), Some(y), Some(s)) = (c.get("x").and_then(|v| v.as_f64()), c.get("y").and_then(|v| v.as_f64()), c.get("size").and_then(|v| v.as_f64())) else { continue };
        d.draw_circle(x as i32, y as i32, s as i32, ColorRydit::Cyan);
    }
}

fn draw_cell_division(d: &mut ry_gfx::DrawHandle, t: f64) {
    let cells = science_anim::cell_division(400.0, 300.0, 25.0, 2.0, 4, t * 0.15);
    for c in &cells {
        let (Some(x), Some(y), Some(r)) = (c.get("x").and_then(|v| v.as_f64()), c.get("y").and_then(|v| v.as_f64()), c.get("radius").and_then(|v| v.as_f64())) else { continue };
        d.draw_circle(x as i32, y as i32, r as i32, ColorRydit::Verde);
    }
}

fn draw_walk_cycle(d: &mut ry_gfx::DrawHandle, t: f64) {
    let elements = science_anim::walk_cycle(400.0, 250.0, 25.0, 4, 18.0, t * 0.8, 0.25);
    for e in &elements {
        let Some(tp) = e.get("type").and_then(|v| v.as_str()) else { continue };
        if tp == "body" {
            let (Some(x), Some(y), Some(s)) = (e.get("x").and_then(|v| v.as_f64()), e.get("y").and_then(|v| v.as_f64()), e.get("size").and_then(|v| v.as_f64())) else { continue };
            d.draw_circle(x as i32, y as i32, s as i32, ColorRydit::Naranja);
        } else if tp == "leg" {
            let (Some(x1), Some(y1), Some(x2), Some(y2)) = (e.get("x1").and_then(|v| v.as_f64()), e.get("y1").and_then(|v| v.as_f64()), e.get("x2").and_then(|v| v.as_f64()), e.get("y2").and_then(|v| v.as_f64())) else { continue };
            d.draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, ColorRydit::Cafe);
        }
    }
}

fn draw_flight(d: &mut ry_gfx::DrawHandle, t: f64) {
    let elements = science_anim::flight_pattern(400.0 + 100.0 * (t * 0.5).sin(), 250.0 + 50.0 * (t * 0.7).cos(), 80.0, 6.0, t);
    for e in &elements {
        let Some(tp) = e.get("type").and_then(|v| v.as_str()) else { continue };
        if tp == "body" {
            let (Some(x), Some(y)) = (e.get("x").and_then(|v| v.as_f64()), e.get("y").and_then(|v| v.as_f64())) else { continue };
            d.draw_circle(x as i32, y as i32, 8, ColorRydit::Azul);
        } else if tp == "wing" {
            let (Some(x1), Some(y1), Some(x2), Some(y2)) = (e.get("x1").and_then(|v| v.as_f64()), e.get("y1").and_then(|v| v.as_f64()), e.get("x2").and_then(|v| v.as_f64()), e.get("y2").and_then(|v| v.as_f64())) else { continue };
            d.draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, ColorRydit::Cyan);
        }
    }
}

fn draw_tree(d: &mut ry_gfx::DrawHandle, t: f64) {
    let branches = science_anim::lsystem_tree(400.0, 520.0, 80.0, 0.5, 0.7, 5, (t * 0.1).min(1.0));
    for b in &branches {
        let (Some(x1), Some(y1), Some(x2), Some(y2)) = (b.get("x1").and_then(|v| v.as_f64()), b.get("y1").and_then(|v| v.as_f64()), b.get("x2").and_then(|v| v.as_f64()), b.get("y2").and_then(|v| v.as_f64())) else { continue };
        let c = if b.get("depth").and_then(|v| v.as_f64()).unwrap_or(0.0) > 2.0 { ColorRydit::Cafe } else { ColorRydit::Verde };
        d.draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, c);
    }
}

fn draw_tusi(d: &mut ry_gfx::DrawHandle, t: f64) {
    let elements = science_anim::tusi_couple(400.0, 300.0, 120.0, t * 0.8);
    for e in &elements {
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
}

fn draw_pendulums(d: &mut ry_gfx::DrawHandle, t: f64) {
    let pendulums = science_anim::pendulum_waves(400.0, 80.0, 16, 120.0, 0.06, t);
    for p in &pendulums {
        let (Some(x1), Some(y1), Some(x2), Some(y2)) = (p.get("x1").and_then(|v| v.as_f64()), p.get("y1").and_then(|v| v.as_f64()), p.get("x2").and_then(|v| v.as_f64()), p.get("y2").and_then(|v| v.as_f64())) else { continue };
        d.draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, ColorRydit::Blanco);
        d.draw_circle(x2 as i32, y2 as i32, 6, ColorRydit::Amarillo);
    }
}

fn draw_waves(d: &mut ry_gfx::DrawHandle, t: f64) {
    let waves = science_anim::wave_interference(250.0, 300.0, 550.0, 300.0, 50.0, 1.0, 20, t);
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
}
