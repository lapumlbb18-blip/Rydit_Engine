//! demo_ryart_cyberpunk.rs
//! Demo de Combate Cyberpunk + Arte Digital (STABLE MODE)
//!
//! ✅ Input rastreado correctamente (mouse_position fix)
//! ✅ Renderizado Directo Ultra-Visible
//! ✅ Fondo Gris Claro para depuración de patrones

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Instant;

use ry_gfx::backend_sdl2::Sdl2Backend;
use ry_gfx::ColorRydit;
use ry_art::{ArtCanvas, PhysicsBrush, Brush};
use events_ry::{InputManager, InputEvent, MouseButton};

fn main() -> Result<(), String> {
    println!("🛡️ Ry-Dit v0.22.0 - ryArt STABLE MODE");
    println!("=====================================");

    let mut backend = Sdl2Backend::new("Ry-Dit | ryArt Stable", 1280, 720)?;
    let mut canvas_art = ArtCanvas::new(1280, 720);
    let mut input = InputManager::new();
    
    // Brochas con inercia muy marcada
    let mut brush_red = PhysicsBrush::new(ColorRydit::Rojo, 20.0);
    let mut brush_blue = PhysicsBrush::new(ColorRydit::Azul, 12.0);

    let mut last_time = Instant::now();
    let mut running = true;

    println!("\n✅ SISTEMA LISTO:");
    println!("   - Fondo: GRIS CLARO (220, 220, 220)");
    println!("   - Click Izquierdo: SANGRE (ROJO)");
    println!("   - Click Derecho: GLITCH (AZUL)");
    println!("   - R: Limpiar lienzo");

    while running {
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        // 1. Sincronizar Input
        backend.actualizar_input_unificado(&mut input);
        
        for event in input.poll_events() {
            match event {
                InputEvent::WindowCloseRequested => running = false,
                InputEvent::KeyPressed { key: events_ry::Key::Escape } => running = false,
                InputEvent::KeyPressed { key: events_ry::Key::R } => {
                    canvas_art = ArtCanvas::new(1280, 720);
                }
                _ => {}
            }
        }

        let (mx, my) = input.mouse_position();
        let left_click = input.is_mouse_button_down(MouseButton::Left);
        let right_click = input.is_mouse_button_down(MouseButton::Right);

        // 2. Pintar (Update ryArt)
        brush_red.update(dt);
        brush_blue.update(dt);

        if left_click {
            // Pintar círculos de píxeles densos
            brush_red.stroke(&mut canvas_art, mx as f32, my as f32, 2.5);
        }
        if right_click {
            brush_blue.stroke(&mut canvas_art, mx as f32, my as f32, 1.8);
        }

        // 3. Renderizado
        backend.canvas.set_draw_color(Color::RGB(220, 220, 220)); 
        backend.canvas.clear();

        // Dibujar el arte persistente (Optimizamos para ver TODO)
        // Recorremos el canvas completo con step 2 para visibilidad garantizada
        for y in (0..canvas_art.height).step_by(2) {
            for x in (0..canvas_art.width).step_by(2) {
                let idx = (y * canvas_art.width + x) as usize;
                let color = canvas_art.pixels[idx];
                
                if color != ColorRydit::Negro {
                    let c = color.to_color();
                    backend.canvas.set_draw_color(Color::RGB(c.r, c.g, c.b));
                    // Dibujamos rects más grandes para asegurar que se vean
                    let _ = backend.canvas.fill_rect(Rect::new(x as i32, y as i32, 2, 2));
                }
            }
        }

        // Barra de Herramientas (Estilo Buscaminas/Cyberpunk)
        backend.canvas.set_draw_color(Color::RGB(40, 40, 40));
        let _ = backend.canvas.fill_rect(Rect::new(0, 0, 1280, 50));
        
        // Indicador de Click (Debug)
        let indicator_color = if left_click { Color::RGB(255, 0, 0) } 
                             else if right_click { Color::RGB(0, 0, 255) } 
                             else { Color::RGB(80, 80, 80) };
        backend.canvas.set_draw_color(indicator_color);
        let _ = backend.canvas.fill_rect(Rect::new(10, 10, 30, 30));

        // Cursor (Para validar posición mx, my)
        backend.canvas.set_draw_color(Color::RGB(0, 0, 0));
        let _ = backend.canvas.draw_rect(Rect::new(mx - 5, my - 5, 10, 10));

        backend.canvas.present();
    }

    Ok(())
}
