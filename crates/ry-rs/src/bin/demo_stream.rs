// crates/rydit-rs/src/bin/demo_stream.rs
// 🆕 Demo: Streaming de bytecode con rydit-stream

use ry_stream::{stream, StreamServer, WebPortal};
use ry_vm::{compile_source, VM};
use std::thread;
use std::time::Duration;

fn main() {
    println!("🚀 RyDit Stream Demo v0.11.3");
    println!("============================\n");

    // 1. Iniciar servidor de streaming
    let server = StreamServer::new("ws://0.0.0.0:8765");
    server.start();
    println!("✅ Stream server: ws://0.0.0.0:8765");

    // 2. Iniciar portal web
    let mut portal = WebPortal::new(8080);
    portal.start();
    println!("✅ Web portal: http://localhost:8080");
    println!("\n📱 Abre el portal en tu browser para ver el stream\n");

    // 3. Compilar bytecode de ejemplo
    let source = r#"
        dark.slot x = 100
        dark.slot y = 100
        dark.slot dx = 2.0
        dark.slot dy = 2.0
        dark.slot width = 50
        dark.slot height = 50
        dark.slot color = "red"
    "#;

    let program = match compile_source(source) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ Compile error: {}", e);
            return;
        }
    };

    println!("✅ Bytecode compiled: {} instructions", program.len());

    // 4. Iniciar VM
    let mut vm = VM::new();
    vm.load(program);

    // 5. Game loop → Stream
    let mut frame = 0;
    let mut x = 100.0;
    let mut y = 100.0;
    let mut dx = 2.0;
    let mut dy = 2.0;

    println!("\n🎮 Streaming entities...\n");

    loop {
        // Actualizar posición (rebote en bordes)
        x += dx;
        y += dy;

        if x <= 0.0 || x >= 750.0 {
            dx = -dx;
        }
        if y <= 0.0 || y >= 550.0 {
            dy = -dy;
        }

        // Crear entidad para stream
        let entities = vec![stream::EntityData {
            id: 1,
            x,
            y,
            sprite: None,
            color: Some("cyan".to_string()),
            width: Some(50.0),
            height: Some(50.0),
        }];

        // Enviar update
        let msg = stream::update(entities, 0.016);
        if let Ok(json) = msg.to_json() {
            if let Err(e) = server.broadcast(&json) {
                eprintln!("❌ Broadcast error: {}", e);
            }
        }

        frame += 1;
        if frame % 60 == 0 {
            println!(
                "📊 Frame {} | Clients: {} | Pos: ({}, {})",
                frame,
                server.client_count(),
                x as i32,
                y as i32
            );
        }

        // 60 FPS
        thread::sleep(Duration::from_millis(16));
    }
}
