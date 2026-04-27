use ry_gfx::{RyditGfx, Key};
use ry_editor::EditorState;
use migui::{Rect, WidgetId};

fn main() {
    // 1. Inicializar RyditGfx con SDL2 como líder (Recomendado para Editor)
    // Esto restaura el soporte nativo de SDL2, TTF y eventos precisos.
    let mut gfx = RyditGfx::new_with_sdl2("🛡️ Ry-Dit Editor v0.23.0 (Fusional)", 1200, 800)
        .expect("Error al inicializar el backend SDL2");
    
    // 2. Inicializar Estado del Editor
    let mut editor = EditorState::new();

    // 3. Inicializar Viewports
    editor.viewports.add_viewport("viewport_3d_main", 800, 560);
    editor.engine.start();

    // 4. Bucle Principal
    while !gfx.should_close() {
        // --- LÓGICA DE INPUT (Sincronización Automática en SDL2 Mode) ---
        gfx.procesar_eventos_sdl2();
        
        // Sincronizar coordenadas con migui directamente desde el estado unificado
        use migui::Event as MiguiEvent;
        editor.gui.handle_event(MiguiEvent::MouseMove { 
            x: gfx.input_sdl2.mouse_x as f32, 
            y: gfx.input_sdl2.mouse_y as f32 
        });
        
        if gfx.input_sdl2.mouse_left_pressed {
            editor.gui.handle_event(MiguiEvent::MouseDown { 
                button: migui::MouseButton::Left, 
                x: gfx.input_sdl2.mouse_x as f32, 
                y: gfx.input_sdl2.mouse_y as f32 
            });
        }
        if gfx.input_sdl2.mouse_left_released {
            editor.gui.handle_event(MiguiEvent::MouseUp { 
                button: migui::MouseButton::Left, 
                x: gfx.input_sdl2.mouse_x as f32, 
                y: gfx.input_sdl2.mouse_y as f32 
            });
        }

        // Navegación del Viewport
        let dx = gfx.input_sdl2.mouse_x - gfx.input_sdl2.prev_mouse_x;
        let dy = gfx.input_sdl2.mouse_y - gfx.input_sdl2.prev_mouse_y;
        let is_interacting = gfx.input_sdl2.mouse_left || gfx.input_sdl2.mouse_right;
        
        // El wheel se obtiene del backend si está disponible
        let wheel = if let Some(ref b) = gfx.backend_sdl2 {
            // Nota: Podríamos inyectar esto en input_sdl2 para mayor consistencia
            0.0 // Placeholder por ahora
        } else {
            gfx.get_mouse_wheel()
        };

        editor.viewport_ctrl.handle_input(wheel, is_interacting, dx, dy);

        // Atajos de teclado (usando el nuevo sistema unificado)
        if gfx.is_key_pressed_sdl2("1") { editor.rybot_gui.show_animation = !editor.rybot_gui.show_animation; }
        if gfx.is_key_pressed_sdl2("2") { editor.rybot_gui.show_tilemap = !editor.rybot_gui.show_tilemap; }
        if gfx.is_key_pressed_sdl2("3") { editor.rybot_gui.show_inspector = !editor.rybot_gui.show_inspector; }
        if gfx.is_key_pressed_sdl2("4") { editor.rybot_gui.show_scene_tree = !editor.rybot_gui.show_scene_tree; }

        // 5. Sincronizar controlador con el viewport real
        if let Some(vp) = editor.viewports.viewports.get_mut("viewport_3d_main") {
            vp.pan_x = editor.viewport_ctrl.pan_x;
            vp.pan_y = editor.viewport_ctrl.pan_y;
            vp.zoom = editor.viewport_ctrl.zoom;
            vp.grid_enabled = editor.viewport_ctrl.show_grid;
        }

        // 6. Update del editor
        editor.update(&mut gfx);
        
        // 7. Definir layout del editor con migui
        editor.gui.begin_frame();
        
        // Toolbar lateral
        let stats = editor.engine.get_stats(
            editor.viewport_ctrl.pan_x, 
            editor.viewport_ctrl.pan_y, 
            editor.viewport_ctrl.zoom
        );
        let scene = editor.engine.scene();
        editor.rybot_gui.draw(&mut editor.gui, &stats, scene);
        
        // Viewport 3D principal (aquí ocurrirá la magia híbrida)
        let viewport_rect = Rect::new(50.0, 30.0, 800.0, 560.0);
        editor.gui.viewport_3d(WidgetId::new("viewport_3d_main"), viewport_rect);
        
        editor.viewport_ctrl.draw_controls(&mut editor.gui, viewport_rect);
        
        editor.gui.end_frame();

        // 8. Renderizado (Automáticamente usa SDL2 o Raylib según el backend activo)
        gfx.render_migui_frame(editor.gui.draw_commands(), &mut editor.viewports);
    }
}
