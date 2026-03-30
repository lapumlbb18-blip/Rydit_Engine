// crates/rydit-rs/src/bin/scene_runner.rs
// Scene Runner - RyDit v0.10.2 Inversión de Control
// Core Rust con game loop nativo, .rydit es solo configuración
// NO usa módulos legacy, solo ECS + config_parser

use rydit_ecs::EcsWorld;
use rydit_gfx::{RyditGfx, ColorRydit, Key};
use rydit_gfx::ecs_render::EcsRenderer;
use rydit_rs::config_parser::ConfigParser;

fn main() {
    println!("🛡️ RyDit v0.10.2 - Scene Runner (Inversión de Control)");
    println!("======================================================");
    
    // Obtener nombre de escena desde argumentos
    let args: Vec<String> = std::env::args().collect();
    let escena_nombre = if args.len() > 1 {
        &args[1]
    } else {
        "demos/nivel_config.rydit"
    };
    
    println!("Cargando escena: {}", escena_nombre);
    
    // Parsear configuración (SOLO datos, sin evaluator)
    let config = match ConfigParser::parse(escena_nombre) {
        Ok(c) => {
            println!("✅ Configuración cargada: {} entidades", c.entidades.len());
            c
        },
        Err(e) => {
            eprintln!("❌ Error cargando configuración: {}", e);
            return;
        }
    };
    
    // Crear ventana
    let mut gfx = RyditGfx::new(&format!("RyDit - {}", config.nombre), 800, 600);
    gfx.set_target_fps(60);
    
    // Crear ECS World
    let mut ecs_world = EcsWorld::new();
    ecs_world.set_gravity(0.0, config.gravedad);
    
    // Spawnear entidades desde configuración
    println!("Spawneando {} entidades...", config.entidades.len());
    for ent in &config.entidades {
        match ent.tipo.as_str() {
            "player" => {
                ecs_world.create_player(ent.x, ent.y);
                println!("  ✅ Jugador creado en ({}, {})", ent.x, ent.y);
            },
            _ => {
                ecs_world.create_sprite_entity(
                    ent.x, ent.y, &ent.sprite, ent.ancho, ent.alto
                );
            }
        }
    }
    
    // Crear ECS Renderer
    let mut renderer = EcsRenderer::new();
    renderer.set_camera(0.0, 0.0, 1.0);
    
    // Variables de juego
    let mut frame = 0;
    let mut cam_x = 0.0;
    let mut cam_y = 0.0;
    
    println!("\n🎮 Game Loop Nativo Iniciado");
    println!("Controles: W,A,S,D = Mover cámara, ESC = Salir\n");
    
    // GAME LOOP NATIVO (Rust puro, sin evaluator)
    while !gfx.should_close() {
        gfx.begin_draw();
        gfx.clear_background(ColorRydit::Negro);
        
        // INPUT (directo, sin parser)
        if gfx.is_key_pressed(Key::Escape) {
            break;
        }
        
        let speed = 10.0;
        if gfx.is_key_down(Key::D) { cam_x += speed; }
        if gfx.is_key_down(Key::A) { cam_x -= speed; }
        if gfx.is_key_down(Key::S) { cam_y += speed; }
        if gfx.is_key_down(Key::W) { cam_y -= speed; }
        
        // UPDATE (ECS nativo, sin evaluator)
        ecs_world.update(0.016);
        renderer.set_camera(cam_x, cam_y, 1.0);
        
        // RENDER (ECS + rlgl, sin FFI desde script)
        renderer.render_colored(&ecs_world);
        
        // UI (debug info)
        let fps = gfx.get_fps();
        gfx.draw_text(&format!("🛡️ {}", config.nombre), 10, 20, 18, ColorRydit::Blanco);
        gfx.draw_text(&format!("Entidades: {} | FPS: {}", ecs_world.entity_count(), fps), 10, 45, 16, ColorRydit::Verde);
        gfx.draw_text("W,A,S,D = Mover | ESC = Salir", 10, 580, 12, ColorRydit::Gris);
        
        gfx.end_draw();
        frame += 1;
    }
    
    println!("\n✅ Scene completada: {} frames", frame);
}
