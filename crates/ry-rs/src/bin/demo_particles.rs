// Demo de Partículas v0.5.3
// Ejecutar: cargo run --bin demo_particles

use ry_gfx::gpu_particles::{ParticleEmitter, ParticleSystem};
use ry_gfx::{ColorRydit, Key, RyditGfx};

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

        // Crear Render Queue para este frame
        let mut queue = ry_gfx::render_queue::RenderQueue::with_capacity(8192);

        // Clear screen
        queue.push(ry_gfx::render_queue::DrawCommand::Clear {
            color: ColorRydit::Negro,
        });

        // Dibujar partículas (usando el draw directo por ahora)
        // TODO: Integrar particles.draw con RenderQueue
        {
            let mut d = gfx.begin_draw();
            if let Some(ref mut rd) = d.draw {
                particles.draw(rd);
            }
            drop(d);
        }

        // UI con RenderQueue
        queue.push(ry_gfx::render_queue::DrawCommand::Text {
            text: "=== RyDit Partículas v0.5.3 ===".to_string(),
            x: 10,
            y: 10,
            size: 20,
            color: ColorRydit::Blanco,
        });
        queue.push(ry_gfx::render_queue::DrawCommand::Text {
            text: format!("FPS: {}", fps),
            x: 10,
            y: 35,
            size: 16,
            color: ColorRydit::Verde,
        });
        queue.push(ry_gfx::render_queue::DrawCommand::Text {
            text: format!("Partículas: {}", particles.particle_count()),
            x: 10,
            y: 55,
            size: 16,
            color: ColorRydit::Amarillo,
        });
        queue.push(ry_gfx::render_queue::DrawCommand::Text {
            text: "F: Fuego | S: Chispas | H: Humo | E: Explosión | ESC: Salir".to_string(),
            x: 10,
            y: 580,
            size: 16,
            color: ColorRydit::Gris,
        });

        // Ejecutar queue (sin assets - particles no usa sprites)
        // Las partículas se dibujan directamente con particles.draw()
        let mut d = gfx.begin_draw();
        if let Some(ref mut rd) = d.draw {
            particles.draw(rd);
        }
        drop(d);

        if escape {
            break;
        }
    }

    println!("Demo finalizada. ¡Hasta luego!");
}
