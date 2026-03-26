//! Ejemplo de rydit-gfx
//! Rust = Arquitecto, Raylib = Pincel

use rydit_gfx::{ColorRydit, Key, RyditGfx};

fn main() {
    println!("=== RYDIT-GFX v0.1.0 ===");
    println!("Rust = Arquitecto, Raylib = Pincel\n");

    // Crear ventana (Rust controla)
    let mut gfx = RyditGfx::new("RyDit v0.0.7 - rydit-gfx", 800, 600);
    gfx.set_target_fps(60);

    println!("[RUST] Ventana creada, iniciando game loop...\n");

    // Game loop controlado por Rust
    let mut frame = 0;
    while !gfx.should_close() {
        frame += 1;

        // Rust decide input PRIMERO (antes de dibujar)
        let escape_pressed = gfx.is_key_pressed(Key::Escape);

        // Iniciar dibujo (toma &mut self, no podemos usar gfx después)
        {
            let mut d = gfx.begin_draw();

            // Limpiar pantalla
            d.clear(ColorRydit::Negro);

            // Dibujar círculo rojo en el centro (animado)
            let radius = 50 + (frame % 20) as i32;
            d.draw_circle(400, 300, radius, ColorRydit::Rojo);

            // Dibujar rectángulo verde
            d.draw_rectangle(100, 100, 100, 100, ColorRydit::Verde);

            // Dibujar línea azul
            d.draw_line(0, 0, 800, 600, ColorRydit::Azul);

            // Dibujar texto
            d.draw_text(
                "RyDit v0.0.7 - rydit-gfx",
                250,
                50,
                20,
                ColorRydit::Amarillo,
            );
            d.draw_text(
                "Rust = Arquitecto, Raylib = Pincel",
                220,
                80,
                16,
                ColorRydit::Blanco,
            );
            d.draw_text("Presiona ESC para salir", 280, 550, 16, ColorRydit::Blanco);

            // d se va de scope aquí, finaliza dibujo
        }

        // Rust decide después de dibujar
        if escape_pressed {
            println!("\n[RUST] Decidiendo cerrar en frame {}...", frame);
            break;
        }
    }

    println!("\n[RUST] Juego terminado. ¡Éxito!");
}
