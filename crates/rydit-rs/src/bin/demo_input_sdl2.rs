// Demo Input SDL2 - RyDit v0.10.4
// Usa el backend SDL2 para input en Termux-X11
// Ejecutar: cargo run --bin demo_input_sdl2 --release

use rydit_gfx::{RyditGfx, ColorRydit};

fn main() {
    println!("🛡️ RyDit v0.10.4 - Demo Input SDL2");
    println!("==================================");
    
    let mut gfx = RyditGfx::new("Demo Input SDL2", 800, 600);
    gfx.set_target_fps(60);
    
    // Variables de estado del jugador (persistentes!)
    let mut jugador_x: f32 = 400.0;
    let mut jugador_y: f32 = 300.0;
    let velocidad: f32 = 5.0;
    
    let mut frame = 0;
    let mut eventos: Vec<String> = Vec::new();
    
    println!("✅ Ventana abierta - Usando SDL2 para input");
    println!("Presioná W, A, S, D o Flechas para mover");
    println!("ESC para salir\n");
    
    while !gfx.should_close() {
        // ✅ PROCESAR EVENTOS SDL2
        gfx.procesar_eventos_sdl2();
        
        gfx.begin_draw();
        gfx.clear_background(ColorRydit::Negro);
        
        frame += 1;
        
        // ✅ MOVER JUGADOR CON SDL2
        if gfx.is_key_pressed_sdl2("w") || gfx.is_key_pressed_sdl2("arrow_up") {
            jugador_y -= velocidad;
        }
        if gfx.is_key_pressed_sdl2("s") || gfx.is_key_pressed_sdl2("arrow_down") {
            jugador_y += velocidad;
        }
        if gfx.is_key_pressed_sdl2("a") || gfx.is_key_pressed_sdl2("arrow_left") {
            jugador_x -= velocidad;
        }
        if gfx.is_key_pressed_sdl2("d") || gfx.is_key_pressed_sdl2("arrow_right") {
            jugador_x += velocidad;
        }
        
        // Registrar eventos
        if gfx.is_key_just_pressed_sdl2("w") {
            eventos.push(format!("Frame {}: W presionada", frame));
            if eventos.len() > 5 { eventos.remove(0); }
        }
        if gfx.is_key_just_pressed_sdl2("a") {
            eventos.push(format!("Frame {}: A presionada", frame));
            if eventos.len() > 5 { eventos.remove(0); }
        }
        if gfx.is_key_just_pressed_sdl2("s") {
            eventos.push(format!("Frame {}: S presionada", frame));
            if eventos.len() > 5 { eventos.remove(0); }
        }
        if gfx.is_key_just_pressed_sdl2("d") {
            eventos.push(format!("Frame {}: D presionada", frame));
            if eventos.len() > 5 { eventos.remove(0); }
        }
        
        // Dibujar UI
        gfx.draw_text("DEMO INPUT SDL2 - W,A,S,D o Flechas", 20, 30, 20, ColorRydit::Blanco);
        gfx.draw_text(&format!("FPS: {} | Pos: ({}, {})", gfx.get_fps(), jugador_x as i32, jugador_y as i32), 
                      20, 60, 16, ColorRydit::Verde);
        
        // Dibujar eventos recientes
        for (i, evento) in eventos.iter().enumerate() {
            let y = 100 + (i as i32 * 25);
            gfx.draw_text(&evento, 20, y, 16, ColorRydit::Gris);
        }
        
        // Dibujar jugador (cuadrado rojo con punto blanco)
        gfx.draw_rect(jugador_x as i32 - 25, jugador_y as i32 - 25, 50, 50, ColorRydit::Rojo);
        gfx.draw_circle(jugador_x as i32, jugador_y as i32, 5, ColorRydit::Blanco);
        
        // Dibujar referencia (círculo azul en el centro)
        gfx.draw_circle(400, 300, 10, ColorRydit::Azul);
        
        gfx.draw_text("← Cuadrado ROJO se mueve | Centro AZUL es referencia →", 20, 450, 14, ColorRydit::Gris);
        
        // Salir con ESC
        if gfx.is_key_just_pressed_sdl2("escape") {
            println!("👋 Saliendo...");
            break;
        }
        
        gfx.end_draw();
    }
    
    println!("✅ Demo finalizado - {} frames", frame);
    println!("📍 Posición final: ({}, {})", jugador_x as i32, jugador_y as i32);
}
