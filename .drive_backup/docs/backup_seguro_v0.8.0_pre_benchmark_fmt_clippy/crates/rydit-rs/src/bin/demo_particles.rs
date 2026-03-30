// Demo de Partículas v0.5.3
// Ejecutar: cargo run --bin demo_particles

use rydit_gfx::particles::{ParticleEmitter, ParticleSystem};
use rydit_gfx::{ColorRydit, Key, RyditGfx};

fn main() {
    println!("=== DEMO PARTÍCULAS v0.5.3 ===");

    let mut gfx = RyditGfx::new("RyDit Partículas v0.5.3", 800, 600);
    gfx.set_target_fps(60);

    let mut particles = ParticleSystem::new();

    // Crear emisores de ejemplo
    particles.create_emitter("fuego", 400.0, 500.0, 30.0);
    if let Some(emitter) = particles.get_emitter_mut("fuego") {
        *emitter = ParticleEmitter::fire(400.0, 500.0);
    }

    particles.create_emitter("humo", 400.0, 480.0, 10.0);
    if let Some(emitter) = particles.get_emitter_mut("humo") {
        *emitter = ParticleEmitter::smoke(400.0, 480.0);
    }

    particles.create_emitter("chispas", 200.0, 300.0, 50.0);
    if let Some(emitter) = particles.get_emitter_mut("chispas") {
        *emitter = ParticleEmitter::sparks(200.0, 300.0);
    }

    // Configurar gravedad
    particles.set_gravity(-50.0); // Hacia arriba para fuego

    let mut last_time = std::time::Instant::now();

    println!("Controles:");
    println!("  F - Toggle fuego");
    println!("  S - Toggle chispas");
    println!("  H - Toggle humo");
    println!("  E - Explosión en posición del mouse");
    println!("  ESC - Salir");
    println!("============================");

    while !gfx.should_close() {
        // Calcular delta time
        let now = std::time::Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        // Input
        let escape = gfx.is_key_pressed(Key::Escape);
        let fps = gfx.get_fps();

        // Toggle fuego con F
        if gfx.is_key_pressed(Key::F) {
            if particles.get_emitter_mut("fuego").is_some() {
                particles.remove_emitter("fuego");
                println!("[INFO] Fuego DESACTIVADO");
            } else {
                particles.create_emitter("fuego", 400.0, 500.0, 30.0);
                if let Some(emitter) = particles.get_emitter_mut("fuego") {
                    *emitter = ParticleEmitter::fire(400.0, 500.0);
                }
                println!("[INFO] Fuego ACTIVADO");
            }
        }

        // Toggle chispas con S
        if gfx.is_key_pressed(Key::S) {
            if particles.get_emitter_mut("chispas").is_some() {
                particles.remove_emitter("chispas");
                println!("[INFO] Chispas DESACTIVADAS");
            } else {
                particles.create_emitter("chispas", 200.0, 300.0, 50.0);
                if let Some(emitter) = particles.get_emitter_mut("chispas") {
                    *emitter = ParticleEmitter::sparks(200.0, 300.0);
                }
                println!("[INFO] Chispas ACTIVADAS");
            }
        }

        // Toggle humo con H
        if gfx.is_key_pressed(Key::H) {
            if particles.get_emitter_mut("humo").is_some() {
                particles.remove_emitter("humo");
                println!("[INFO] Humo DESACTIVADO");
            } else {
                particles.create_emitter("humo", 400.0, 480.0, 10.0);
                if let Some(emitter) = particles.get_emitter_mut("humo") {
                    *emitter = ParticleEmitter::smoke(400.0, 480.0);
                }
                println!("[INFO] Humo ACTIVADO");
            }
        }

        // Explosión con E
        if gfx.is_key_pressed(Key::E) {
            let (mx, my) = gfx.get_mouse_position();
            particles.create_emitter("explosion", mx as f32, my as f32, 500.0);
            if let Some(emitter) = particles.get_emitter_mut("explosion") {
                *emitter = ParticleEmitter::explosion(mx as f32, my as f32);
            }
            println!("[INFO] Explosión en ({}, {})", mx, my);
        }

        // Actualizar partículas
        particles.update(dt);

        // Dibujar
        {
            let mut d = gfx.begin_draw();
            d.clear(ColorRydit::Negro);

            // Dibujar partículas
            particles.draw(&mut d.draw);

            // UI
            d.draw_text(
                "=== RyDit Partículas v0.5.3 ===",
                10,
                10,
                20,
                ColorRydit::Blanco,
            );
            d.draw_text(&format!("FPS: {}", fps), 10, 35, 16, ColorRydit::Verde);
            d.draw_text(
                &format!("Partículas: {}", particles.particle_count()),
                10,
                55,
                16,
                ColorRydit::Amarillo,
            );
            d.draw_text(
                "F: Fuego | S: Chispas | H: Humo | E: Explosión | ESC: Salir",
                10,
                580,
                16,
                ColorRydit::Gris,
            );
        }

        if escape {
            break;
        }
    }

    println!("Demo finalizada. ¡Hasta luego!");
}
