//! gpu_circle_test.rs — 9 círculos con raylib para confirmar que draw_circle funciona
//! Si ves 9 círculos = raylib OK.
//! Si ves pantalla negra = problema de viewport/raylib.

use ry_gfx::{ColorRydit, RyditGfx, Key};

fn main() {
    println!("[CIRCLE TEST] 9 círculos con raylib\n");

    let mut gfx = RyditGfx::new("Circle Test", 1280, 720);
    gfx.set_target_fps(60);

    let w = 1280;
    let h = 720;

    // 9 posiciones en grid 3x3
    let positions = [
        (0, 0), (w/2, 0), (w-1, 0),
        (0, h/2), (w/2, h/2), (w-1, h/2),
        (0, h-1), (w/2, h-1), (w-1, h-1),
    ];
    let colors = [
        ColorRydit::Rojo, ColorRydit::Verde, ColorRydit::Azul,
        ColorRydit::Amarillo, ColorRydit::Blanco, ColorRydit::Magenta,
        ColorRydit::Cyan, ColorRydit::Naranja, ColorRydit::Morado,
    ];

    println!("9 círculos de radio 40px:");
    for i in 0..9 {
        let (x, y) = positions[i];
        println!("[{}] ({}, {})", i, x, y);
    }

    let mut frame = 0u64;

    while !gfx.should_close() {
        {
            let mut d = gfx.begin_draw();
            d.clear(ColorRydit::Negro);

            for i in 0..9 {
                let (x, y) = positions[i];
                d.draw_circle(x, y, 40, colors[i]);
            }

            d.draw_text("CIRCLE TEST", 10, 10, 20, ColorRydit::Blanco);
            // drop automático = end_draw
        }

        frame += 1;
        if frame == 60 {
            println!("[FRAME 60] 60 frames renderizados");
        }

        if gfx.is_key_pressed(Key::Escape) { break; }
    }
    println!("\n✅ {} frames", frame);
}
