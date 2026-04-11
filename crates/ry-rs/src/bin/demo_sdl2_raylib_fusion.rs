// demo_sdl2_raylib_fusion.rs
// FUSIÓN: SDL2 (ventana + input + lógica) + Raylib (dibujo 3D)
//
// Hipótesis: SDL2 crea el contexto OpenGL, raylib dibuja en él
// SDL2 maneja input, raylib maneja dibujo 3D

use gl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::GLProfile;

// Raylib FFI para dibujo 3D (sin crear ventana)
use raylib::ffi::{
    Vector3, Camera3D,
    DrawCube, DrawCubeWires,
    DrawSphere, DrawSphereWires,
    DrawCylinder, DrawCylinderWires,
    DrawGrid, DrawLine3D, DrawTriangle3D, DrawPoint3D,
    DrawPlane, DrawBoundingBox, DrawText,
    BeginMode3D, EndMode3D,
    ClearBackground,
    Color as RLColor,
};

fn to_rl_color(r: u8, g: u8, b: u8) -> RLColor {
    RLColor { r, g, b, a: 255 }
}

// Colores estilo ry-gfx
const ROJO: RLColor = RLColor { r: 230, g: 41, b: 55, a: 255 };
const VERDE: RLColor = RLColor { r: 0, g: 228, b: 48, a: 255 };
const AZUL: RLColor = RLColor { r: 0, g: 121, b: 241, a: 255 };
const AMARILLO: RLColor = RLColor { r: 253, g: 249, b: 0, a: 255 };
const MAGENTA: RLColor = RLColor { r: 200, g: 111, b: 234, a: 255 };
const CYAN: RLColor = RLColor { r: 0, g: 218, b: 214, a: 255 };
const NARANJA: RLColor = RLColor { r: 255, g: 161, b: 0, a: 255 };
const BLANCO: RLColor = RLColor { r: 255, g: 255, b: 255, a: 255 };
const NEGRO: RLColor = RLColor { r: 0, g: 0, b: 0, a: 255 };
const GRIS: RLColor = RLColor { r: 130, g: 130, b: 130, a: 255 };
const MORADO: RLColor = RLColor { r: 135, g: 59, b: 237, a: 255 };

// Cámara orbital simple
struct OrbitalCamera {
    distance: f32,
    theta: f32,  // ángulo horizontal
    phi: f32,    // ángulo vertical
    target: Vector3,
}

impl OrbitalCamera {
    fn new() -> Self {
        Self {
            distance: 12.0,
            theta: 0.8,
            phi: 0.5,
            target: Vector3 { x: 0.0, y: 1.0, z: 0.0 },
        }
    }

    fn to_camera3d(&self) -> Camera3D {
        let x = self.target.x + self.distance * self.phi.sin() * self.theta.cos();
        let y = self.target.y + self.distance * self.phi.cos();
        let z = self.target.z + self.distance * self.phi.sin() * self.theta.sin();

        Camera3D {
            position: Vector3 { x, y, z },
            target: self.target,
            up: Vector3 { x: 0.0, y: 1.0, z: 0.0 },
            fovy: 45.0,
            projection: 0, // CAMERA_PERSPECTIVE
        }
    }

    fn rotate(&mut self, dtheta: f32, dphi: f32) {
        self.theta += dtheta;
        self.phi = (self.phi + dphi).max(0.1).min(3.0);
    }

    fn zoom(&mut self, delta: f32) {
        self.distance = (self.distance + delta).max(3.0).min(30.0);
    }
}

fn main() -> Result<(), String> {
    println!("🔄 RyDit - FUSIÓN SDL2 + Raylib");
    println!("   SDL2 → Ventana + Input + Lógica");
    println!("   Raylib → Dibujo 3D");
    println!("   ESC: Salir | Mouse: Orbitar | Rueda: Zoom\n");

    // ========================================================================
    // SDL2: Ventana + Contexto OpenGL + Input
    // ========================================================================
    let sdl_ctx = sdl2::init().map_err(|e| e.to_string())?;
    let video = sdl_ctx.video().map_err(|e| e.to_string())?;

    // Configurar OpenGL
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_double_buffer(true);

    let window = video
        .window("🔄 FUSIÓN SDL2+Raylib — Dibujo 3D con Raylib, Input con SDL2", 900, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let gl_context = window.gl_create_context().map_err(|e| e.to_string())?;
    window.gl_make_current(&gl_context).map_err(|e| e.to_string())?;

    // Cargar OpenGL functions
    gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

    // Event pump
    let mut event_pump = sdl_ctx.event_pump().map_err(|e| e.to_string())?;

    println!("✅ SDL2: Ventana + OpenGL contexto creados");
    println!("✅ Raylib: Funciones de dibujo 3D listas (FFI directo)");
    println!("📝 Controles: Mouse orbitar | Rueda zoom | Teclas: 1/2 orbit preset | ESC salir");

    // ========================================================================
    // Cámara orbital (lógica nuestra, sin GLFW)
    // ========================================================================
    let mut camera = OrbitalCamera::new();
    let mut running = true;

    // ========================================================================
    // GAME LOOP — SDL2 maneja input, Raylib dibuja
    // ========================================================================
    let mut cur_mouse_x = 450i32;
    let mut cur_mouse_y = 300i32;

    while running {
        let mut mouse_dx = 0;
        let mut mouse_dy = 0;

        // ---- SDL2 INPUT ----
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                }
                Event::MouseMotion { x, y, .. } => {
                    mouse_dx = x - cur_mouse_x;
                    mouse_dy = y - cur_mouse_y;
                    cur_mouse_x = x;
                    cur_mouse_y = y;
                }
                Event::MouseWheel { y, .. } => {
                    camera.zoom(y as f32 * -0.5);
                }
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                    camera.theta = 0.8; camera.phi = 0.5; camera.distance = 12.0;
                }
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                    camera.theta = 1.5; camera.phi = 0.3; camera.distance = 20.0;
                }
                _ => {}
            }
        }

        // Actualizar cámara con mouse
        camera.rotate(mouse_dx as f32 * 0.005, mouse_dy as f32 * 0.005);

        // ====================================================================
        // RAYLIB: Dibujo 3D en el contexto OpenGL de SDL2
        // ====================================================================

        // Limpiar con OpenGL directo
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
        }

        // Camera3D de raylib
        let cam = camera.to_camera3d();

        // ---- MODO 3D (raylib) ----
        unsafe {
            BeginMode3D(cam);

            // Grid (suelo)
            DrawGrid(20, 1.0);

            // Ejes
            DrawLine3D(
                Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                Vector3 { x: 3.0, y: 0.0, z: 0.0 },
                ROJO,
            );
            DrawLine3D(
                Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                Vector3 { x: 0.0, y: 3.0, z: 0.0 },
                VERDE,
            );
            DrawLine3D(
                Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                Vector3 { x: 0.0, y: 0.0, z: 3.0 },
                AZUL,
            );

            // CUBOS
            DrawCube(Vector3 { x: 0.0, y: 1.0, z: 0.0 }, 2.0, 2.0, 2.0, ROJO);
            DrawCubeWires(Vector3 { x: -3.5, y: 1.0, z: 0.0 }, 2.0, 2.0, 2.0, VERDE);
            DrawCube(Vector3 { x: 3.5, y: 0.5, z: 0.0 }, 1.0, 1.0, 1.0, AZUL);

            // ESFERAS
            DrawSphere(Vector3 { x: 0.0, y: 1.0, z: -4.0 }, 1.0, AMARILLO);
            DrawSphereWires(Vector3 { x: -3.5, y: 1.0, z: -4.0 }, 1.0, 16, 16, MAGENTA);
            DrawSphere(Vector3 { x: 3.5, y: 0.5, z: -4.0 }, 0.5, CYAN);

            // CILINDROS
            DrawCylinder(Vector3 { x: 0.0, y: 1.0, z: -8.0 }, 0.5, 0.8, 2.0, 16, NARANJA);
            DrawCylinderWires(Vector3 { x: -3.5, y: 1.0, z: -8.0 }, 0.5, 0.8, 2.0, 16, BLANCO);
            // Cono (radio superior = 0)
            DrawCylinder(Vector3 { x: 3.5, y: 1.0, z: -8.0 }, 0.0, 0.8, 2.0, 16, MORADO);

            // PLANO
            DrawPlane(
                Vector3 { x: 0.0, y: 0.01, z: -12.0 },
                raylib::ffi::Vector2 { x: 4.0, y: 4.0 },
                GRIS,
            );

            // PIRÁMIDE con líneas
            let base = [
                Vector3 { x: -5.0, y: 0.0, z: -12.0 },
                Vector3 { x: -4.0, y: 0.0, z: -12.0 },
                Vector3 { x: -4.0, y: 0.0, z: -13.0 },
                Vector3 { x: -5.0, y: 0.0, z: -13.0 },
            ];
            let apex = Vector3 { x: -4.5, y: 2.0, z: -12.5 };
            for i in 0..4 {
                DrawLine3D(base[i], apex, ROJO);
                DrawLine3D(base[i], base[(i + 1) % 4], VERDE);
            }

            // TRIÁNGULO 3D
            DrawTriangle3D(
                Vector3 { x: 5.0, y: 0.0, z: -12.0 },
                Vector3 { x: 7.0, y: 0.0, z: -12.0 },
                Vector3 { x: 6.0, y: 2.0, z: -12.0 },
                NARANJA,
            );

            // PUNTOS 3D
            for i in 0..10 {
                let x = -5.0 + i as f32;
                DrawPoint3D(
                    Vector3 { x, y: 3.0 + i as f32 * 0.3, z: -14.0 },
                    BLANCO,
                );
            }

            // BOUNDING BOX
            let bbox = raylib::ffi::BoundingBox {
                min: Vector3 { x: 5.0, y: 0.0, z: -4.0 },
                max: Vector3 { x: 7.0, y: 2.0, z: -2.0 },
            };
            DrawBoundingBox(bbox, VERDE);

            EndMode3D();
        }

        // ---- SDL2: HUD con texto simple ----
        // (El texto HUD se podría hacer con SDL2_ttf, pero por simplicidad
        //  lo dejamos para la terminal. Lo importante es que raylib dibuja 3D)

        // Swap buffers vía SDL2
        window.gl_swap_window();

        // Cap a 60 FPS
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    println!("\n✅ FUSIÓN SDL2+Raylib cerrada");
    println!("   SDL2 manejó: ventana, input, eventos, swap");
    println!("   Raylib dibujó: cubos, esferas, cilindros, grid, líneas, bbox");
    Ok(())
}
