//! # Ry3D Graphics Layer (ry3d-gfx)
//!
//! **Mundo 3D para Ry-Dit** — Crate independiente de ry-gfx (2D)
//!
//! ## Filosofía
//!
//! ry-gfx = 2D (círculos, rects, líneas, sprites)
//! ry3d-gfx = 3D (cubos, esferas, cilindros, modelos)
//!
//! Comparten `ColorRydit` y la base de raylib, pero son crates separados.

#![allow(clippy::too_many_arguments)]

// Controles táctiles en pantalla (como RayGunz)
pub mod touch_controls;

// Necesario para tipos raylib internos usados en re-exports
#[allow(unused_imports)]
use raylib::prelude::*;
use ry_gfx::ColorRydit;

/// Convertir ColorRydit a ffi::Color (para funciones FFI 3D)
fn to_ffi_color(c: ColorRydit) -> raylib::ffi::Color {
    let rc = c.to_color();
    unsafe { std::mem::transmute::<raylib::prelude::Color, raylib::ffi::Color>(rc) }
}

// ============================================================================
// RE-EXPORTS
// ============================================================================

pub use raylib::core::camera::Camera3D;
pub use raylib::prelude::Vector3;

/// Alias: Camera = Camera3D
pub type Camera = Camera3D;

// ============================================================================
// MODELOS 3D — Carga de assets .gltf, .obj, .iqm, .vox
// ============================================================================

/// Modelo 3D cargado desde archivo (.gltf, .obj, .iqm, .vox, .mdl)
///
/// Los formatos soportados son:
/// - **GLTF/GLB** — estándar abierto (recomendado)
/// - **OBJ** — Wavefront OBJ
/// - **IQM** — Inter-Quake Model (con animaciones)
/// - **VOX** — MagicaVoxel (voxel models)
/// - **MDL** — Quake model
pub struct Model3D {
    inner: raylib::ffi::Model,
    loaded: bool,
}

impl Model3D {
    /// Cargar modelo desde archivo (usa FFI directo de raylib)
    ///
    /// # Nota
    /// Esta función debe llamarse DENTRO de un contexto raylib activo
    /// (después de init_window y antes de close_window).
    pub fn load(path: &str) -> Result<Self, String> {
        use std::ffi::CString;
        let c_path = CString::new(path).map_err(|e| e.to_string())?;
        let model = unsafe { raylib::ffi::LoadModel(c_path.as_ptr()) };
        if model.meshCount > 0 {
            Ok(Self { inner: model, loaded: true })
        } else {
            Err(format!("No se pudo cargar modelo '{}'", path))
        }
    }

    /// Verificar si el modelo está cargado
    pub fn is_loaded(&self) -> bool { self.loaded }
}

impl Drop for Model3D {
    fn drop(&mut self) {
        if self.loaded {
            unsafe { raylib::ffi::UnloadModel(self.inner) };
        }
    }
}

/// Builder conveniente para Camera3D
pub struct Camera3DBuilder {
    position: Vector3,
    target: Vector3,
    up: Vector3,
    fovy: f32,
}

impl Camera3DBuilder {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(10.0, 10.0, 10.0),
            target: Vector3::zero(),
            up: Vector3::new(0.0, 1.0, 0.0),
            fovy: 45.0,
        }
    }

    pub fn position(mut self, pos: Vector3) -> Self { self.position = pos; self }
    pub fn target(mut self, target: Vector3) -> Self { self.target = target; self }
    pub fn up(mut self, up: Vector3) -> Self { self.up = up; self }
    pub fn fovy(mut self, fovy: f32) -> Self { self.fovy = fovy; self }

    pub fn perspective(self) -> Camera3D {
        Camera3D::perspective(self.position, self.target, self.up, self.fovy)
    }

    pub fn orthographic(self) -> Camera3D {
        Camera3D::orthographic(self.position, self.target, self.up, self.fovy)
    }
}

impl Default for Camera3DBuilder {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// DRAW HANDLE 3D
// ============================================================================

/// Handle de dibujo 3D.
///
/// Se crea con `DrawHandle3D::new(&camera)`.
/// Al salir de scope, llama `EndMode3D()` automáticamente.
///
/// # Ejemplo
///
/// ```rust,ignore
/// // Dentro de tu game loop con raylib:
/// let mut d = gfx.begin_draw();
/// d.clear(ColorRydit::Negro);
///
/// let mut h3d = DrawHandle3D::new(&camera);
/// h3d.draw_cube_3d((0, 0, 0), (2, 2, 2), ColorRydit::Rojo);
/// drop(h3d); // sale de modo 3D
///
/// d.draw_text("HUD", 10, 10, 20, ColorRydit::Blanco);
/// ```
pub struct DrawHandle3D<'a> {
    _camera: &'a Camera3D,
}

impl<'a> DrawHandle3D<'a> {
    /// Entrar modo 3D
    pub fn new(camera: &'a Camera3D) -> Self {
        unsafe { raylib::ffi::BeginMode3D((*camera).into()) };
        Self { _camera: camera }
    }

    /// Dibujar cubo 3D (sólido)
    pub fn draw_cube_3d(&mut self, pos: (f32,f32,f32), size: (f32,f32,f32), color: ColorRydit) {
        unsafe {
            raylib::ffi::DrawCube(
                raylib::ffi::Vector3 { x: pos.0, y: pos.1, z: pos.2 },
                size.0, size.1, size.2, to_ffi_color(color));
        }
    }

    /// Cubo wireframe
    pub fn draw_cube_wires_3d(&mut self, pos: (f32,f32,f32), size: (f32,f32,f32), color: ColorRydit) {
        unsafe {
            raylib::ffi::DrawCubeWires(
                raylib::ffi::Vector3 { x: pos.0, y: pos.1, z: pos.2 },
                size.0, size.1, size.2, to_ffi_color(color));
        }
    }

    /// Esfera sólida
    pub fn draw_sphere_3d(&mut self, center: (f32,f32,f32), radius: f32, color: ColorRydit) {
        unsafe {
            raylib::ffi::DrawSphere(
                raylib::ffi::Vector3 { x: center.0, y: center.1, z: center.2 },
                radius, to_ffi_color(color));
        }
    }

    /// Esfera wireframe
    pub fn draw_sphere_wires_3d(&mut self, center: (f32,f32,f32), radius: f32, color: ColorRydit) {
        unsafe {
            raylib::ffi::DrawSphereWires(
                raylib::ffi::Vector3 { x: center.0, y: center.1, z: center.2 },
                radius, 16, 16, to_ffi_color(color));
        }
    }

    /// Cilindro
    pub fn draw_cylinder_3d(&mut self, pos: (f32,f32,f32), rt: f32, rb: f32, h: f32, color: ColorRydit) {
        unsafe {
            raylib::ffi::DrawCylinder(
                raylib::ffi::Vector3 { x: pos.0, y: pos.1, z: pos.2 },
                rt, rb, h, 16, to_ffi_color(color));
        }
    }

    /// Cilindro wireframe
    pub fn draw_cylinder_wires_3d(&mut self, pos: (f32,f32,f32), rt: f32, rb: f32, h: f32, color: ColorRydit) {
        unsafe {
            raylib::ffi::DrawCylinderWires(
                raylib::ffi::Vector3 { x: pos.0, y: pos.1, z: pos.2 },
                rt, rb, h, 16, to_ffi_color(color));
        }
    }

    /// Plano 3D (en plano XZ)
    pub fn draw_plane_3d(&mut self, center: (f32,f32,f32), size: f32, color: ColorRydit) {
        unsafe {
            raylib::ffi::DrawPlane(
                raylib::ffi::Vector3 { x: center.0, y: center.1, z: center.2 },
                raylib::ffi::Vector2 { x: size, y: size },
                to_ffi_color(color));
        }
    }

    /// Grid 3D (suelo de referencia)
    pub fn draw_grid_3d(&mut self, slices: i32, spacing: f32) {
        unsafe { raylib::ffi::DrawGrid(slices, spacing) };
    }

    /// Punto 3D
    pub fn draw_point_3d(&mut self, pos: (f32,f32,f32), color: ColorRydit) {
        unsafe {
            raylib::ffi::DrawPoint3D(
                raylib::ffi::Vector3 { x: pos.0, y: pos.1, z: pos.2 },
                to_ffi_color(color));
        }
    }

    /// Línea 3D
    pub fn draw_line_3d(&mut self, start: (f32,f32,f32), end: (f32,f32,f32), color: ColorRydit) {
        unsafe {
            raylib::ffi::DrawLine3D(
                raylib::ffi::Vector3 { x: start.0, y: start.1, z: start.2 },
                raylib::ffi::Vector3 { x: end.0, y: end.1, z: end.2 },
                to_ffi_color(color));
        }
    }

    /// Triángulo 3D
    pub fn draw_triangle_3d(&mut self, p1: (f32,f32,f32), p2: (f32,f32,f32), p3: (f32,f32,f32), color: ColorRydit) {
        unsafe {
            raylib::ffi::DrawTriangle3D(
                raylib::ffi::Vector3 { x: p1.0, y: p1.1, z: p1.2 },
                raylib::ffi::Vector3 { x: p2.0, y: p2.1, z: p2.2 },
                raylib::ffi::Vector3 { x: p3.0, y: p3.1, z: p3.2 },
                to_ffi_color(color));
        }
    }

    /// Bounding box
    pub fn draw_bounding_box_3d(&mut self, min: (f32,f32,f32), max: (f32,f32,f32), color: ColorRydit) {
        let bbox = raylib::ffi::BoundingBox {
            min: raylib::ffi::Vector3 { x: min.0, y: min.1, z: min.2 },
            max: raylib::ffi::Vector3 { x: max.0, y: max.1, z: max.2 },
        };
        unsafe { raylib::ffi::DrawBoundingBox(bbox, to_ffi_color(color)) };
    }

    /// Ejes XYZ debug (rojo=X, verde=Y, azul=Z)
    pub fn draw_axes_gizmo(&mut self, length: f32) {
        self.draw_line_3d((0.0, 0.0, 0.0), (length, 0.0, 0.0), ColorRydit::Rojo);
        self.draw_line_3d((0.0, 0.0, 0.0), (0.0, length, 0.0), ColorRydit::Verde);
        self.draw_line_3d((0.0, 0.0, 0.0), (0.0, 0.0, length), ColorRydit::Azul);
    }

    /// Limpiar fondo
    pub fn clear_3d(&mut self, color: ColorRydit) {
        unsafe { raylib::ffi::ClearBackground(to_ffi_color(color)) };
    }

    // ========================================================================
    // TEXTO 3D — Letras en coordenadas del mundo (billboard text)
    // ========================================================================

    /// Dibujar texto en coordenadas 3D (proyectando 3D→2D)
    ///
    /// ⚠️ PENDIENTE: GetWorldToScreen no está expuesto en raylib FFI Rust.
    /// Se necesita usar raylib prelude directamente o implementar la proyección manual.
    /// Por ahora usa DrawText 2D en el HUD para texto.
    pub fn draw_text_3d(&mut self, _pos: (f32,f32,f32), _text: &str, _size: f32, _color: ColorRydit) {
        // TODO: Implementar con GetWorldToScreen cuando esté disponible en FFI
        // Alternativa: usar raylib prelude Camera3D.get_world_to_screen()
    }

    // ========================================================================
    // FUNCIONES PARA MODELOS 3D
    // ========================================================================

    /// Dibujar modelo 3D en posición con escala
    pub fn draw_model(&mut self, model: &Model3D, pos: (f32,f32,f32), scale: f32, tint: ColorRydit) {
        let pos3 = raylib::ffi::Vector3 { x: pos.0, y: pos.1, z: pos.2 };
        let scale3 = raylib::ffi::Vector3 { x: scale, y: scale, z: scale };
        let tint_color = to_ffi_color(tint);
        unsafe {
            raylib::ffi::DrawModel(model.inner, pos3, scale, tint_color);
        }
    }

    /// Dibujar modelo 3D con rotación y escala por eje
    pub fn draw_model_ex(&mut self, model: &Model3D, pos: (f32,f32,f32), rot_axis: (f32,f32,f32), rot_angle: f32, scale: (f32,f32,f32), tint: ColorRydit) {
        let pos3 = raylib::ffi::Vector3 { x: pos.0, y: pos.1, z: pos.2 };
        let rot_axis3 = raylib::ffi::Vector3 { x: rot_axis.0, y: rot_axis.1, z: rot_axis.2 };
        let scale3 = raylib::ffi::Vector3 { x: scale.0, y: scale.1, z: scale.2 };
        let tint_color = to_ffi_color(tint);
        unsafe {
            raylib::ffi::DrawModelEx(model.inner, pos3, rot_axis3, rot_angle, scale3, tint_color);
        }
    }

    /// Dibujar modelo 3D con rotación (escala uniforme)
    pub fn draw_model_ex_uniform(&mut self, model: &Model3D, pos: (f32,f32,f32), rot_axis: (f32,f32,f32), rot_angle: f32, scale: f32, tint: ColorRydit) {
        self.draw_model_ex(model, pos, rot_axis, rot_angle, (scale, scale, scale), tint);
    }
}

impl<'a> Drop for DrawHandle3D<'a> {
    fn drop(&mut self) {
        unsafe { raylib::ffi::EndMode3D() };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera3d_builder_perspective() {
        let c = Camera3DBuilder::new()
            .position(Vector3::new(5.0, 5.0, 5.0))
            .target(Vector3::zero())
            .fovy(60.0)
            .perspective();
        assert_eq!(c.position.x, 5.0);
        assert_eq!(c.fovy, 60.0);
    }

    #[test]
    fn test_camera3d_builder_orthographic() {
        let c = Camera3DBuilder::default().orthographic();
        assert_eq!(c.position.x, 10.0);
    }

    #[test]
    fn test_model3d_not_loaded() {
        // Modelo sin cargar (solo verifica compilación)
        assert!(!Model3D { inner: unsafe { std::mem::zeroed() }, loaded: false }.is_loaded());
    }
}
