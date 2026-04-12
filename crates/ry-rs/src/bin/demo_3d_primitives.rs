// demo_3d_primitives.rs
// Demo: ry3d-gfx — Primitivas 3D

use raylib::prelude::*;
use ry3d_gfx::*;
use ry_gfx::ColorRydit;

fn main() -> Result<(), String> {
    println!("🧊 RyDit - Demo 3D Primitives (ry3d-gfx)");

    let (mut rl, thread) = raylib::init()
        .size(900, 600)
        .title("Demo 3D Primitives - ry3d-gfx")
        .build();

    let mut camera = Camera3D::perspective(
        Vector3::new(8.0, 6.0, 8.0),
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    rl.set_target_fps(60);
    println!("✅ Ventana 3D | Mouse: Orbitar | ESC: Salir");

    while !rl.window_should_close() {
        // Usar update_camera con CAMERA_ORBITAL del FFI
        rl.update_camera(&mut camera, raylib::ffi::CameraMode::CAMERA_ORBITAL);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // ---- MODO 3D ----
        let mut h3d = DrawHandle3D::new(&camera);

        h3d.clear_3d(ColorRydit::Negro);
        h3d.draw_grid_3d(20, 1.0);
        h3d.draw_axes_gizmo(3.0);

        // CUBOS
        h3d.draw_cube_3d((0.0, 1.0, 0.0), (2.0, 2.0, 2.0), ColorRydit::Rojo);
        h3d.draw_cube_wires_3d((-3.0, 1.0, 0.0), (2.0, 2.0, 2.0), ColorRydit::Verde);
        h3d.draw_cube_3d((3.0, 0.5, 0.0), (1.0, 1.0, 1.0), ColorRydit::Azul);

        // ESFERAS
        h3d.draw_sphere_3d((0.0, 1.0, -4.0), 1.0, ColorRydit::Amarillo);
        h3d.draw_sphere_wires_3d((-3.0, 1.0, -4.0), 1.0, ColorRydit::Magenta);
        h3d.draw_sphere_3d((3.0, 0.5, -4.0), 0.5, ColorRydit::Cyan);

        // CILINDROS
        h3d.draw_cylinder_3d((0.0, 1.0, -8.0), 0.5, 0.8, 2.0, ColorRydit::Naranja);
        h3d.draw_cylinder_wires_3d((-3.0, 1.0, -8.0), 0.5, 0.8, 2.0, ColorRydit::Blanco);
        h3d.draw_cylinder_3d((3.0, 1.0, -8.0), 0.0, 0.8, 2.0, ColorRydit::Morado);

        // PLANO
        h3d.draw_plane_3d((0.0, 0.01, -12.0), 4.0, ColorRydit::Gris);

        // LÍNEAS 3D (pirámide)
        let base = [
            (-5.0, 0.0, -12.0), (-4.0, 0.0, -12.0),
            (-4.0, 0.0, -13.0), (-5.0, 0.0, -13.0),
        ];
        let apex = (-4.5, 2.0, -12.5);
        for i in 0..4 {
            h3d.draw_line_3d(base[i], apex, ColorRydit::Rojo);
            h3d.draw_line_3d(base[i], base[(i + 1) % 4], ColorRydit::Verde);
        }

        // TRIÁNGULO 3D
        h3d.draw_triangle_3d(
            (5.0, 0.0, -12.0), (7.0, 0.0, -12.0), (6.0, 2.0, -12.0),
            ColorRydit::Naranja,
        );

        // PUNTOS 3D
        for i in 0..10 {
            let x = -5.0 + i as f32;
            h3d.draw_point_3d((x, 3.0 + i as f32 * 0.3, -14.0), ColorRydit::Blanco);
        }

        // BOUNDING BOX
        h3d.draw_bounding_box_3d((5.0, 0.0, -4.0), (7.0, 2.0, -2.0), ColorRydit::Verde);

        // LETRAS 3D
        h3d.draw_text_3d((0.0, 3.5, 0.0), "CUBO", 20.0, ColorRydit::Rojo);
        h3d.draw_text_3d((0.0, 3.5, -4.0), "ESFERA", 20.0, ColorRydit::Amarillo);
        h3d.draw_text_3d((0.0, 3.5, -8.0), "CILINDRO", 18.0, ColorRydit::Naranja);
        h3d.draw_text_3d((-4.5, 3.0, -12.5), "PIRAMIDE", 16.0, ColorRydit::Verde);
        h3d.draw_text_3d((6.0, 3.0, -12.0), "TRIANGULO", 16.0, ColorRydit::Naranja);

        // Texto con fondo
        h3d.draw_text_3d_with_bg((0.0, 5.0, -2.0), "Ry3D-GFX + Letras 3D", 16.0,
            ColorRydit::Blanco, ColorRydit::Negro);

        // ---- HUD 2D ----
        drop(h3d);

        d.draw_text("🧊 Demo 3D Primitives — ry3d-gfx v0.1.0", 10, 10, 20, Color::WHITE);
        d.draw_text("Mouse: Orbitar camara | ESC: Salir", 10, 35, 16, Color::LIGHTGRAY);

        let cam_pos = format!(
            "Camera: ({:.1}, {:.1}, {:.1})",
            camera.position.x, camera.position.y, camera.position.z
        );
        d.draw_text(&cam_pos, 10, 55, 14, Color::GRAY);
    }

    println!("\n✅ Demo 3D cerrado");
    Ok(())
}
