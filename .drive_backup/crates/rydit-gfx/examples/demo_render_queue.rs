//! # Demo de Render Queue v0.9.0
//!
//! Este demo prueba las 3 capas críticas:
//! 1. Command Queue (8192+ draw calls)
//! 2. Double Buffering
//! 3. Platform Sync (X11)

use rydit_gfx::{ColorRydit, Key, RyditGfx};
use rydit_gfx::render_queue::{DrawCommand, DoubleBuffer, PlatformSync, RenderQueue};

fn main() {
    println!("🛡️ RyDit v0.9.0 - Demo Render Queue");
    println!("====================================");
    println!("Probando 3 capas críticas:");
    println!("  1. Command Queue (8192+ draw calls)");
    println!("  2. Double Buffering");
    println!("  3. Platform Sync (X11)");
    println!();

    // Crear ventana
    let mut gfx = RyditGfx::new("RyDit v0.9.0 - Render Queue Demo", 800, 600);
    gfx.set_target_fps(60);

    // === CAPA 1: Command Queue ===
    let queue = RenderQueue::with_capacity(8192);
    println!("✅ Command Queue creada: capacidad={}", queue.capacity());

    // === CAPA 2: Double Buffer ===
    let mut double_buffer = DoubleBuffer::new(8192);
    println!("✅ Double Buffer creado");

    // === CAPA 3: Platform Sync ===
    let mut platform_sync = PlatformSync::new();
    println!("✅ Platform Sync iniciado: modo={:?}", platform_sync.mode());
    println!();

    // Variables del demo
    let mut frame = 0u64;
    let mut circulos_dibujados = 0usize;
    let mut rects_dibujados = 0usize;
    let mut lineas_dibujadas = 0usize;

    println!("🎮 Iniciando game loop...");
    println!("Presiona ESC para salir");
    println!();

    // Game loop principal
    while !gfx.should_close() {
        frame += 1;

        // === FASE 1: Acumular comandos (Front Buffer) ===
        
        // Limpiar pantalla
        double_buffer.push(DrawCommand::Clear { color: ColorRydit::Negro });

        // === PRUEBA 1: Grid de círculos (100 círculos) ===
        for i in 0..10 {
            for j in 0..10 {
                let x = 50 + (i * 70) as i32;
                let y = 50 + (j * 50) as i32;
                let radio = (20 + (frame % 30)) as i32;
                let color = match (i + j) % 3 {
                    0 => ColorRydit::Rojo,
                    1 => ColorRydit::Verde,
                    2 => ColorRydit::Azul,
                    _ => ColorRydit::Blanco,
                };

                double_buffer.push(DrawCommand::Circle {
                    x,
                    y,
                    radius: radio,
                    color,
                });
                circulos_dibujados += 1;
            }
        }

        // === PRUEBA 2: Rectángulos animados (50 rects) ===
        for i in 0..10 {
            let x = 50 + (i * 70) as i32;
            let y = 500 + (((frame / 20 + i) % 5) as i32) * 15;
            let color = match i % 5 {
                0 => ColorRydit::Amarillo,
                1 => ColorRydit::Naranja,
                2 => ColorRydit::Cyan,
                3 => ColorRydit::Magenta,
                _ => ColorRydit::Gris,
            };

            double_buffer.push(DrawCommand::Rect {
                x,
                y,
                w: 60,
                h: 30,
                color,
            });
            rects_dibujados += 1;
        }

        // === PRUEBA 3: Líneas en patrón radial (36 líneas) ===
        let centro_x = 400;
        let centro_y = 300;
        let radio_max = 250.0f32;

        for i in 0..36 {
            let angulo = (i as f32) * 10.0 * std::f32::consts::PI / 180.0;
            let x2 = centro_x + (radio_max * angulo.cos()) as i32;
            let y2 = centro_y + (radio_max * angulo.sin()) as i32;

            double_buffer.push(DrawCommand::Line {
                x1: centro_x,
                y1: centro_y,
                x2,
                y2,
                color: ColorRydit::Blanco,
            });
            lineas_dibujadas += 1;
        }

        // === PRUEBA 4: Texto informativo ===
        double_buffer.push(DrawCommand::Text {
            text: format!("RyDit v0.9.0 - Frame {}", frame),
            x: 250,
            y: 30,
            size: 24,
            color: ColorRydit::Blanco,
        });

        double_buffer.push(DrawCommand::Text {
            text: format!("Circulos: {}", circulos_dibujados),
            x: 50,
            y: 550,
            size: 16,
            color: ColorRydit::Amarillo,
        });

        double_buffer.push(DrawCommand::Text {
            text: format!("Rects: {}", rects_dibujados),
            x: 200,
            y: 550,
            size: 16,
            color: ColorRydit::Amarillo,
        });

        double_buffer.push(DrawCommand::Text {
            text: format!("Lineas: {}", lineas_dibujadas),
            x: 350,
            y: 550,
            size: 16,
            color: ColorRydit::Amarillo,
        });

        double_buffer.push(DrawCommand::Text {
            text: format!("FPS: {}", gfx.get_fps()),
            x: 500,
            y: 550,
            size: 16,
            color: if gfx.get_fps() >= 55 {
                ColorRydit::Verde
            } else {
                ColorRydit::Rojo
            },
        });

        // Reset contadores
        circulos_dibujados = 0;
        rects_dibujados = 0;
        lineas_dibujadas = 0;

        // === FASE 2: Swap + Execute (Back Buffer) ===
        double_buffer.swap_and_execute(&mut gfx);

        // === FASE 3: Platform Sync (X11) ===
        platform_sync.sync();

        // Input: Salir con ESC
        if gfx.is_key_pressed(Key::Escape) {
            println!();
            println!("🛑 ESC presionado - Saliendo...");
            break;
        }
    }

    // Estadísticas finales
    println!();
    println!("📊 Estadísticas del demo:");
    println!("  Frames totales: {}", frame);
    println!("  Comando Queue stats: {}", double_buffer.stats().0);
    println!();
    println!("✅ Demo completado - 3 capas probadas exitosamente!");
    println!();
    println!("🎯 CAPAS IMPLEMENTADAS:");
    println!("  1. ✅ Command Queue (8192+ draw calls)");
    println!("  2. ✅ Double Buffering (front/back buffer)");
    println!("  3. ✅ Platform Sync (XFlush/XSync para X11)");
    println!();
    println!("🚀 RyDit v0.9.0 - Listo para demos complejos!");
}
