// crates/ry-rs/src/bin/demo_3d_player_touch.rs
// Demo: ry3d-gfx — Jugador 3D + Seguimiento de Cámara + Controles Táctiles
//
// Pilar 1: Validación de UX 3D en Android (Termux/X11)
//
// Controles:
// Joystick izq: Mover Jugador (plano XZ)
// Joystick der: Orbitar cámara alrededor del jugador
// Botón A: Toggle controles visuales
// Botón B: Reset posición jugador
// Mouse: Click+arrastrar orbita | Rueda zoom

use raylib::prelude::*;
use ry3d_gfx::{DrawHandle3D, OrbitCamera3D};
use ry3d_gfx::touch_controls::TouchControls;
use ry_gfx::ColorRydit;

struct Player {
    pub position: Vector3,
    pub speed: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vector3::new(x, y, z),
            speed: 5.0,
        }
    }

    pub fn update(&mut self, dx: f32, dz: f32, dt: f32) {
        self.position.x += dx * self.speed * dt;
        self.position.z += dz * self.speed * dt;
    }
}

fn main() -> Result<(), String> {
    println!("🎮 RyDit - Demo 3D Player Follow (100% raylib)");

    let screen_w = 900;
    let screen_h = 600;

    let (mut rl, thread) = raylib::init()
        .size(screen_w, screen_h)
        .title("Demo 3D Player Follow - ry3d-gfx + TouchControls")
        .build();

    // Jugador
    let mut player = Player::new(0.0, 1.0, 0.0);

    // Cámara orbital que seguirá al jugador
    let mut orbit_cam = OrbitCamera3D::new(player.position, 10.0, 0.8, 0.5);

    // Controles táctiles
    let mut touch = TouchControls::new(screen_w as f32, screen_h as f32);

    rl.set_target_fps(60);

    println!("✅ Escena 3D configurada");
    println!("   Joy Izq: Mover Jugador (XZ)");
    println!("   Joy Der: Rotar Cámara");
    println!("   Botón A: Toggle UI | Botón B: Reset");

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        // ---- 1. INPUT ----
        let touching = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        let touch_pos = rl.get_mouse_position();
        touch.update(touching, touch_pos.x, touch_pos.y);

        // Joystick izq → Mover Jugador
        let (jx, jz) = touch.joy_left.axis();
        // El movimiento debe ser relativo a la rotación de la cámara para que sea intuitivo
        let yaw = orbit_cam.yaw;
        let forward = Vector3::new(-yaw.sin(), 0.0, -yaw.cos());
        let right = Vector3::new(yaw.cos(), 0.0, -yaw.sin());
        
        let move_dir = (forward * -jz) + (right * jx);
        player.update(move_dir.x, move_dir.z, dt);

        // Joystick der → Rotar Cámara
        let (rx, ry) = touch.joy_right.axis();
        orbit_cam.yaw += rx * 2.0 * dt;
        orbit_cam.pitch = (orbit_cam.pitch - ry * 1.5 * dt).max(0.1).min(1.5);

        // Zoom mouse
        let wheel = rl.get_mouse_wheel_move();
        orbit_cam.distance = (orbit_cam.distance - wheel * 1.0).max(3.0).min(30.0);

        // Botones
        if touch.btn_a.just_pressed { touch.toggle(); }
        if touch.btn_b.just_pressed { 
            player.position = Vector3::new(0.0, 1.0, 0.0); 
        }

        // ---- 2. CAMERA FOLLOW (Smooth) ----
        // Interpolación lineal suave del target de la cámara hacia la posición del jugador
        let lerp_factor = 0.1; // Ajustar para más/menos suavidad
        orbit_cam.target.x += (player.position.x - orbit_cam.target.x) * lerp_factor;
        orbit_cam.target.y += (player.position.y - orbit_cam.target.y) * lerp_factor;
        orbit_cam.target.z += (player.position.z - orbit_cam.target.z) * lerp_factor;

        let camera = orbit_cam.to_camera();

        // ---- 3. RENDER ----
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // MODO 3D
        let mut h3d = DrawHandle3D::new(&camera);
        h3d.draw_grid_3d(20, 1.0);
        
        // Dibujar Suelo
        h3d.draw_plane_3d((0.0, 0.0, 0.0), 50.0, ColorRydit::Gris);

        // Dibujar Jugador
        h3d.draw_sphere_3d((player.position.x, player.position.y, player.position.z), 1.0, ColorRydit::Rojo);
        h3d.draw_sphere_wires_3d((player.position.x, player.position.y, player.position.z), 1.05, ColorRydit::Blanco);

        // Dibujar algunos obstáculos para referencia
        for i in -2..=2 {
            for j in -2..=2 {
                if i == 0 && j == 0 { continue; }
                let x = i as f32 * 8.0;
                let z = j as f32 * 8.0;
                h3d.draw_cube_3d((x, 1.0, z), (2.0, 2.0, 2.0), ColorRydit::Azul);
                h3d.draw_cube_wires_3d((x, 1.0, z), (2.0, 2.0, 2.0), ColorRydit::Cyan);
            }
        }

        drop(h3d);

        // UI 2D
        touch.draw();

        // HUD
        d.draw_text("🛡️ RyDit 3D Player Touch Validation", 10, 10, 20, Color::WHITE);
        d.draw_text("Joy Izq: Mover (Relativo Cámara) | Joy Der: Rotar Cámara", 10, screen_h - 40, 15, Color::LIGHTGRAY);
        d.draw_fps(screen_w - 80, 10);
    }

    Ok(())
}
