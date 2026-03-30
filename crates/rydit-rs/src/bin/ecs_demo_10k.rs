// crates/rydit-rs/src/bin/ecs_demo_10k.rs
// Demo: 10,000 entidades con ECS + rlgl
// v0.10.0: ECS + rlgl integration test

use rydit_ecs::EcsWorld;
use rydit_gfx::ecs_render::{EcsRenderer, create_demo_world, create_nbody_demo};
use rydit_gfx::{RyditGfx, ColorRydit, Key};

fn main() {
    println!("🛡️ RyDit v0.10.0 - ECS Demo 10K Entidades");
    println!("==========================================");
    
    // Crear ventana
    let mut gfx = RyditGfx::new("RyDit v0.10.0 - ECS 10K Demo", 800, 600);
    gfx.set_target_fps(60);
    
    // Crear ECS Renderer
    let mut renderer = EcsRenderer::new();
    renderer.set_camera(0.0, 0.0, 1.0);
    
    // Crear mundo con 10K entidades
    println!("Creando 10,000 entidades...");
    let mut world = create_demo_world(10000);
    println!("✅ {} entidades creadas", world.entity_count());
    
    // Variables de control
    let mut frame = 0;
    let mut last_fps = 0;
    let mut demo_mode = 0; // 0 = 10K sprites, 1 = N-Body
    
    println!("\nControles:");
    println!("  A = 10K Sprites");
    println!("  B = N-Body Gravity");
    println!("  Space = Reiniciar");
    println!("  ESC = Salir\n");
    
    // Game loop
    while !gfx.should_close() {
        gfx.begin_draw();
        gfx.clear_background(ColorRydit::Negro);
        
        // Input
        if gfx.is_key_pressed(Key::A) {
            demo_mode = 0;
            world = create_demo_world(10000);
            println!("Mode: 10K Sprites");
        }
        
        if gfx.is_key_pressed(Key::B) {
            demo_mode = 1;
            world = create_nbody_demo(100);
            println!("Mode: N-Body (100 cuerpos)");
        }
        
        if gfx.is_key_pressed(Key::Space) {
            if demo_mode == 0 {
                world = create_demo_world(10000);
            } else {
                world = create_nbody_demo(100);
            }
            frame = 0;
            println!("Reiniciado");
        }
        
        if gfx.is_key_pressed(Key::Escape) {
            break;
        }
        
        // Actualizar ECS
        world.update(0.016);
        
        // Renderizar
        if demo_mode == 0 {
            renderer.render_colored(&world);
        } else {
            renderer.render_nbody(&world);
        }
        
        // Debug UI
        let current_fps = gfx.get_fps();
        if current_fps != last_fps {
            last_fps = current_fps;
        }
        
        gfx.draw_text(
            &format!("RyDit v0.10.0 - ECS Demo"),
            10, 20, 20, ColorRydit::Blanco,
        );
        
        gfx.draw_text(
            &format!("Entidades: {}", world.entity_count()),
            10, 45, 16, ColorRydit::Verde,
        );
        
        gfx.draw_text(
            &format!("FPS: {}", current_fps),
            10, 70, 16, ColorRydit::Cyan,
        );
        
        gfx.draw_text(
            &format!("Frame: {}", frame),
            10, 95, 14, ColorRydit::Gris,
        );
        
        gfx.draw_text(
            &format!("Mode: {}", if demo_mode == 0 { "10K Sprites" } else { "N-Body Gravity" }),
            10, 120, 14, ColorRydit::Amarillo,
        );
        
        gfx.draw_text(
            "Controles: A=10K, B=N-Body, Space=Reiniciar, ESC=Salir",
            10, 580, 12, ColorRydit::Gris,
        );
        
        gfx.end_draw();
        
        frame += 1;
        
        // Log cada 60 frames
        if frame % 60 == 0 {
            println!("Frame {}: {} entidades, {} FPS", frame, world.entity_count(), current_fps);
        }
    }
    
    println!("\n✅ Demo completado: {} frames totales", frame);
}
