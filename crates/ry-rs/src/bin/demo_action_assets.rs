// Demo: Action Assets - Sprite Animation System
// Uso: cargo run --bin demo_action_assets --release

use ry_gfx::{ColorRydit, RyditGfx};
use ry_anim::action_assets;

fn main() {
    let mut gfx = RyditGfx::new("Action Assets - Ry-Dit", 800, 600);
    gfx.set_target_fps(30);

    let mut current = 0;
    let mut t = 0.0;
    let names = [
        "Frame Animation",
        "Sprite Sheet Parse",
        "Animation State Machine",
        "Animation Blending",
        "Sprite Events",
        "Sprite Flip",
    ];

    println!("[DEMO] Action Assets - 1-6 cambiar, ESC salir");

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

        {
            let mut d = gfx.begin_draw();
            d.clear(ColorRydit::Negro);

            match current {
                0 => draw_frame_anim(&mut d, t),
                1 => draw_sprite_sheet(&mut d, t),
                2 => draw_state_machine(&mut d, t),
                3 => draw_blend(&mut d, t),
                4 => draw_events(&mut d, t),
                5 => draw_flip(&mut d, t),
                _ => {}
            }

            d.draw_text(&format!("{} (t={:.1})", names[current], t), 10, 15, 14, ColorRydit::Blanco);
            d.draw_text("1-6 cambiar | ESC salir", 10, 575, 12, ColorRydit::Gris);
        }
    }
    println!("[DEMO] Action Assets completado");
}

fn draw_frame_anim(d: &mut ry_gfx::DrawHandle, t: f64) {
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

fn draw_sprite_sheet(d: &mut ry_gfx::DrawHandle, t: f64) {
    let frame_idx = (t * 2.0) as usize % 16;
    let _result = action_assets::sprite_sheet_parse(256.0, 256.0, 64.0, 64.0, frame_idx, 4);

    for row in 0..4i32 {
        for col in 0..4i32 {
            let x = 100 + col * 50;
            let y = 100 + row * 50;
            let idx = row * 4 + col;
            let c = if idx == frame_idx as i32 { ColorRydit::Amarillo } else { ColorRydit::Gris };
            d.draw_rectangle(x, y, 40, 40, c);
        }
    }
    d.draw_text(&format!("Frame: {} (row: {}, col: {})", frame_idx, frame_idx / 4, frame_idx % 4), 100, 50, 14, ColorRydit::Blanco);
}

fn draw_state_machine(d: &mut ry_gfx::DrawHandle, t: f64) {
    let states = vec!["idle".to_string(), "walk".to_string(), "run".to_string(), "jump".to_string()];
    let durations = vec![2.0, 1.5, 1.0, 0.5];

    let cycle = (t * 0.3) as usize % 4;
    let state_time = t % durations[cycle];
    let result = action_assets::animation_state_machine(&states[cycle], &states, &durations, state_time, "");
    let progress = result.get("progress").and_then(|v| v.as_f64()).unwrap_or(0.0);

    d.draw_rectangle(100, 200, 600, 30, ColorRydit::Gris);
    d.draw_rectangle(100, 200, (600.0 * progress) as i32, 30, ColorRydit::Verde);
    d.draw_text(&format!("Estado: {}", states[cycle]), 100, 150, 16, ColorRydit::Blanco);

    for (i, s) in states.iter().enumerate() {
        let y = 280 + i as i32 * 30;
        let c = if i == cycle { ColorRydit::Amarillo } else { ColorRydit::Gris };
        d.draw_text(&format!("{} ({:.1}s)", s, durations[i]), 100, y, 14, c);
    }
}

fn draw_blend(d: &mut ry_gfx::DrawHandle, t: f64) {
    let state_a_progress = (t * 0.5) % 1.0;
    let state_b_progress = (t * 1.5) % 1.0;
    let result = action_assets::animation_blend(state_a_progress, state_b_progress, 1.0, 2.0, (t * 0.3).min(2.0));
    let blended = result.get("blended_progress").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let blend_f = result.get("blend_factor").and_then(|v| v.as_f64()).unwrap_or(0.0);

    d.draw_text("Idle", 100, 100, 14, ColorRydit::Verde);
    d.draw_rectangle(200, 100, 500, 20, ColorRydit::Gris);
    d.draw_rectangle(200, 100, (500.0 * state_a_progress) as i32, 20, ColorRydit::Verde);

    d.draw_text("Run", 100, 150, 14, ColorRydit::Rojo);
    d.draw_rectangle(200, 150, 500, 20, ColorRydit::Gris);
    d.draw_rectangle(200, 150, (500.0 * state_b_progress) as i32, 20, ColorRydit::Rojo);

    d.draw_text("Blended", 100, 220, 14, ColorRydit::Amarillo);
    d.draw_rectangle(200, 220, 500, 20, ColorRydit::Gris);
    d.draw_rectangle(200, 220, (500.0 * blended) as i32, 20, ColorRydit::Amarillo);

    d.draw_text(&format!("Blend factor: {:.2}", blend_f), 100, 280, 14, ColorRydit::Blanco);
}

fn draw_events(d: &mut ry_gfx::DrawHandle, t: f64) {
    let frame = (t * 4.0) as usize % 8;
    let progress = (t * 4.0) % 8.0 / 8.0;
    let event_types = ["frame_change", "state_change", "loop_complete", "animation_end"];

    for (i, etype) in event_types.iter().enumerate() {
        let result = action_assets::sprite_events(etype, frame, 8, "run", progress);
        let triggered = result.get("triggered").and_then(|v| v.as_bool()).unwrap_or(false);
        let y = 150 + i as i32 * 80;
        let c = if triggered { ColorRydit::Verde } else { ColorRydit::Gris };
        d.draw_rectangle(100, y, 600, 50, c);
        d.draw_text(etype, 120, y + 5, 14, ColorRydit::Negro);
        if triggered {
            d.draw_text("TRIGGERED!", 500, y + 5, 14, ColorRydit::Rojo);
        }
    }
    d.draw_text(&format!("Frame: {}/8", frame + 1), 100, 100, 16, ColorRydit::Blanco);
}

fn draw_flip(d: &mut ry_gfx::DrawHandle, t: f64) {
    let hflip = (t * 0.5).sin() > 0.0;
    let vflip = (t * 0.3).cos() > 0.0;
    let result = action_assets::sprite_flip(hflip, vflip, 0.5, 0.5);
    let scale_x = result.get("scale_x").and_then(|v| v.as_f64()).unwrap_or(1.0);
    let scale_y = result.get("scale_y").and_then(|v| v.as_f64()).unwrap_or(1.0);

    let w = (60.0 * scale_x.abs()) as i32;
    let h = (80.0 * scale_y.abs()) as i32;
    d.draw_rectangle(370, 260, w, h, ColorRydit::Azul);

    let arrow_x = if scale_x > 0.0 { 430 } else { 350 };
    d.draw_text("->", arrow_x, 280, 20, ColorRydit::Blanco);
    d.draw_text(&format!("Flip H: {} | Flip V: {}", hflip, vflip), 100, 100, 16, ColorRydit::Blanco);
    d.draw_text(&format!("Scale X: {:.1} | Scale Y: {:.1}", scale_x, scale_y), 100, 130, 14, ColorRydit::Gris);
}
