// Demo SDL2 Puro - RyDit v0.10.4
// USA SDL2 para TODO: ventana, render, input
// Ejecutar: cargo run --bin demo_sdl2_puro --release

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() {
    println!("🛡️ RyDit v0.10.4 - Demo SDL2 Puro");
    println!("==================================");
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem
        .window("Demo SDL2 Puro - RyDit", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    // Variables de estado (persistentes!)
    let mut jugador_x: i32 = 400;
    let mut jugador_y: i32 = 300;
    let velocidad: i32 = 5;
    
    let mut frame = 0;
    let mut running = true;
    let mut eventos: Vec<String> = Vec::new();
    
    println!("✅ Ventana abierta - SDL2 puro (ventana + render + input)");
    println!("Presioná W, A, S, D o Flechas para mover");
    println!("ESC para salir\n");
    
    'running: while running {
        // ✅ PROCESAR EVENTOS SDL2
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                    println!("👋 Saliendo...");
                    break 'running;
                }
                
                // Teclas presionadas (evento inicial)
                Event::KeyDown { keycode: Some(key), repeat: false, .. } => {
                    println!("[KEY PRESS] {:?}", key);
                    
                    match key {
                        Keycode::W | Keycode::Up => jugador_y -= velocidad,
                        Keycode::S | Keycode::Down => jugador_y += velocidad,
                        Keycode::A | Keycode::Left => jugador_x -= velocidad,
                        Keycode::D | Keycode::Right => jugador_x += velocidad,
                        _ => {}
                    }
                    
                    eventos.push(format!("Frame {}: {:?}", frame, key));
                    if eventos.len() > 5 { eventos.remove(0); }
                }
                
                // Teclas mantenidas (repeat)
                Event::KeyDown { keycode: Some(key), repeat: true, .. } => {
                    match key {
                        Keycode::W | Keycode::Up => jugador_y -= velocidad,
                        Keycode::S | Keycode::Down => jugador_y += velocidad,
                        Keycode::A | Keycode::Left => jugador_x -= velocidad,
                        Keycode::D | Keycode::Right => jugador_x += velocidad,
                        _ => {}
                    }
                }
                
                _ => {}
            }
        }
        
        frame += 1;
        
        // ✅ RENDER SDL2
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // Dibujar referencia (círculo azul en centro)
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        let centro = Rect::new(395, 295, 10, 10);
        canvas.fill_rect(centro).unwrap();
        
        // Dibujar jugador (cuadrado rojo)
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let jugador = Rect::new(jugador_x - 25, jugador_y - 25, 50, 50);
        canvas.fill_rect(jugador).unwrap();
        
        // Dibujar punto blanco en centro del jugador
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let punto = Rect::new(jugador_x - 3, jugador_y - 3, 6, 6);
        canvas.fill_rect(punto).unwrap();
        
        // Dibujar texto (usando SDL2_ttf o simple)
        // Por ahora solo dibujamos rects
        
        canvas.present();
        
        if frame % 60 == 0 {
            println!("[Frame {}] Pos: ({}, {})", frame, jugador_x, jugador_y);
        }
    }
    
    println!("✅ Demo finalizado - {} frames", frame);
    println!("📍 Posición final: ({}, {})", jugador_x, jugador_y);
}
