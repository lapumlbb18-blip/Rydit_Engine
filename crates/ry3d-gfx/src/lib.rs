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

// Necesario para tipos raylib internos
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

    /// Dibujar texto en coordenadas 3D (proyección perspectiva 3D→2D).
    /// Equivalente a raylib DrawText3D.
    pub fn draw_text_3d(&mut self, pos: (f32,f32,f32), text: &str, size: f32, color: ColorRydit) {
        let world = raylib::ffi::Vector3 { x: pos.0, y: pos.1, z: pos.2 };
        let cam_ffi = self._camera_to_ffi();
        let screen = unsafe { raylib::ffi::GetWorldToScreen(world, cam_ffi) };

        if screen.x > -500.0 && screen.x < 2500.0 && screen.y > -500.0 && screen.y < 2500.0 {
            let c = to_ffi_color(color);
            let ct = std::ffi::CString::new(text).unwrap_or_default();
            unsafe {
                raylib::ffi::DrawTextEx(
                    raylib::ffi::GetFontDefault(),
                    ct.as_ptr(),
                    screen,
                    size, 1.0, c);
            }
        }
    }

    /// Texto 3D con fondo (mejor legibilidad)
    pub fn draw_text_3d_with_bg(&mut self, pos: (f32,f32,f32), text: &str, size: f32, fg: ColorRydit, bg: ColorRydit) {
        let world = raylib::ffi::Vector3 { x: pos.0, y: pos.1, z: pos.2 };
        let cam_ffi = self._camera_to_ffi();
        let screen = unsafe { raylib::ffi::GetWorldToScreen(world, cam_ffi) };

        if screen.x > -500.0 && screen.x < 2500.0 && screen.y > -500.0 && screen.y < 2500.0 {
            let tw = text.len() as f32 * size * 0.6;
            unsafe {
                raylib::ffi::DrawRectangle(
                    screen.x as i32 - 4, (screen.y - 4.0) as i32,
                    (tw + 8.0) as i32, (size + 8.0) as i32, to_ffi_color(bg));
            }
            let ct = std::ffi::CString::new(text).unwrap_or_default();
            unsafe {
                raylib::ffi::DrawTextEx(
                    raylib::ffi::GetFontDefault(),
                    ct.as_ptr(),
                    screen,
                    size, 1.0, to_ffi_color(fg));
            }
        }
    }

    /// Convertir Camera3D a ffi::Camera para GetWorldToScreen
    fn _camera_to_ffi(&self) -> raylib::ffi::Camera {
        raylib::ffi::Camera {
            position: raylib::ffi::Vector3 {
                x: self._camera.position.x,
                y: self._camera.position.y,
                z: self._camera.position.z,
            },
            target: raylib::ffi::Vector3 {
                x: self._camera.target.x,
                y: self._camera.target.y,
                z: self._camera.target.z,
            },
            up: raylib::ffi::Vector3 {
                x: self._camera.up.x,
                y: self._camera.up.y,
                z: self._camera.up.z,
            },
            fovy: self._camera.fovy,
            projection: raylib::ffi::CameraProjection::CAMERA_PERSPECTIVE as i32,
        }
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

// ============================================================================
// MESH — Geometría procedural (alta/media/baja poligonal)
// ============================================================================

/// Mesh procedural envuelto
pub struct Mesh3D {
    inner: raylib::ffi::Mesh,
}

impl Mesh3D {
    /// Generar cubo con raylib (nivel de detalle: 1-10)
    pub fn cube(detail: u32) -> Self {
        let d = detail.clamp(1, 10) as i32;
        let mesh = unsafe { raylib::ffi::GenMeshCube(1.0, 1.0, 1.0) };
        let _ = d; // reserve for future subdivision
        Self { inner: mesh }
    }

    /// Generar esfera
    pub fn sphere(radius: f32, rings: u32, slices: u32) -> Self {
        let mesh = unsafe { raylib::ffi::GenMeshSphere(radius, rings as i32, slices as i32) };
        Self { inner: mesh }
    }

    /// Generar cilindro
    pub fn cylinder(radius: f32, h: f32, slices: u32) -> Self {
        let mesh = unsafe { raylib::ffi::GenMeshCylinder(radius, h, slices as i32) };
        Self { inner: mesh }
    }

    /// Generar plano
    pub fn plane(w: f32, h: f32, res_w: u32, res_h: u32) -> Self {
        let mesh = unsafe { raylib::ffi::GenMeshPlane(w, h, res_w as i32, res_h as i32) };
        Self { inner: mesh }
    }

    /// Upload mesh to GPU
    pub fn upload_gpu(&mut self) {
        unsafe { raylib::ffi::UploadMesh(&mut self.inner, false) };
    }

    /// Dibujar mesh con posición, escala y color
    pub fn draw(&self, pos: (f32,f32,f32), scale: f32, tint: ColorRydit) {
        let transform = raylib::ffi::Matrix {
            m0: scale, m4: 0.0, m8: 0.0, m12: pos.0,
            m1: 0.0, m5: scale, m9: 0.0, m13: pos.1,
            m2: 0.0, m6: 0.0, m10: scale, m14: pos.2,
            m3: 0.0, m7: 0.0, m11: 0.0, m15: 1.0,
        };
        let c = to_ffi_color(tint);
        unsafe {
            let mut mat = raylib::ffi::LoadMaterialDefault();
            // material.maps is *mut MaterialMap — set color via pointer
            if !mat.maps.is_null() {
                (*mat.maps).color = c;
            }
            raylib::ffi::DrawMesh(self.inner, mat, transform);
            raylib::ffi::UnloadMaterial(mat);
        }
    }
}

impl Drop for Mesh3D {
    fn drop(&mut self) {
        unsafe { raylib::ffi::UnloadMesh(self.inner) };
    }
}

// ========================================================================
// SKELETON / BONES — Sistema de huesos para animación
// ========================================================================

/// Hueso individual en un esqueleto
#[derive(Debug, Clone)]
pub struct Bone3D {
    pub name: String,
    pub parent: Option<usize>,
    pub position: (f32,f32,f32),
    pub rotation: (f32,f32,f32),
    pub scale: (f32,f32,f32),
    pub length: f32,
}

impl Bone3D {
    pub fn new(name: &str, pos: (f32,f32,f32), len: f32) -> Self {
        Self {
            name: name.into(),
            parent: None,
            position: pos,
            rotation: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
            length: len,
        }
    }
    pub fn with_parent(mut self, parent: usize) -> Self {
        self.parent = Some(parent); self
    }
}

/// Esqueleto 3D completo
#[derive(Debug, Clone)]
pub struct Skeleton3D {
    pub bones: Vec<Bone3D>,
    pub root: usize,
}

impl Skeleton3D {
    pub fn new(bones: Vec<Bone3D>, root: usize) -> Self {
        Self { bones, root }
    }

    /// Esqueleto humanoide básico (22 bones)
    pub fn humanoid() -> Self {
        let mut bones = Vec::new();
        bones.push(Bone3D::new("Hips", (0.0, 1.0, 0.0), 0.2));
        let hips = bones.len()-1;
        bones.push(Bone3D::new("Spine", (0.0, 1.3, 0.0), 0.3).with_parent(hips));
        let spine = bones.len()-1;
        bones.push(Bone3D::new("Chest", (0.0, 1.6, 0.0), 0.3).with_parent(spine));
        let chest = bones.len()-1;
        bones.push(Bone3D::new("UpperChest", (0.0, 1.9, 0.0), 0.3).with_parent(chest));
        let uc = bones.len()-1;
        bones.push(Bone3D::new("Neck", (0.0, 2.2, 0.0), 0.15).with_parent(uc));
        let neck = bones.len()-1;
        bones.push(Bone3D::new("Head", (0.0, 2.4, 0.0), 0.3).with_parent(neck));
        // Left arm
        bones.push(Bone3D::new("LeftShoulder", (-0.2, 1.9, 0.0), 0.1).with_parent(uc));
        bones.push(Bone3D::new("LeftUpperArm", (-0.4, 1.8, 0.0), 0.3).with_parent(bones.len()-1));
        bones.push(Bone3D::new("LeftLowerArm", (-0.6, 1.5, 0.0), 0.3).with_parent(bones.len()-1));
        bones.push(Bone3D::new("LeftHand", (-0.7, 1.2, 0.0), 0.15).with_parent(bones.len()-1));
        // Right arm
        bones.push(Bone3D::new("RightShoulder", (0.2, 1.9, 0.0), 0.1).with_parent(uc));
        bones.push(Bone3D::new("RightUpperArm", (0.4, 1.8, 0.0), 0.3).with_parent(bones.len()-1));
        bones.push(Bone3D::new("RightLowerArm", (0.6, 1.5, 0.0), 0.3).with_parent(bones.len()-1));
        bones.push(Bone3D::new("RightHand", (0.7, 1.2, 0.0), 0.15).with_parent(bones.len()-1));
        // Left leg
        bones.push(Bone3D::new("LeftUpperLeg", (-0.15, 0.9, 0.0), 0.4).with_parent(hips));
        bones.push(Bone3D::new("LeftLowerLeg", (-0.15, 0.5, 0.0), 0.4).with_parent(bones.len()-1));
        bones.push(Bone3D::new("LeftFoot", (-0.15, 0.1, 0.05), 0.2).with_parent(bones.len()-1));
        // Right leg
        bones.push(Bone3D::new("RightUpperLeg", (0.15, 0.9, 0.0), 0.4).with_parent(hips));
        bones.push(Bone3D::new("RightLowerLeg", (0.15, 0.5, 0.0), 0.4).with_parent(bones.len()-1));
        bones.push(Bone3D::new("RightFoot", (0.15, 0.1, 0.05), 0.2).with_parent(bones.len()-1));
        Self::new(bones, 0)
    }

    /// Dibujar esqueleto como líneas + esferas
    pub fn draw(&self, h3d: &mut DrawHandle3D, bone_c: ColorRydit, joint_c: ColorRydit) {
        for bone in &self.bones {
            if let Some(pi) = bone.parent {
                let parent = &self.bones[pi];
                h3d.draw_line_3d(parent.position, bone.position, bone_c);
            }
            h3d.draw_sphere_3d(bone.position, bone.length * 0.5, joint_c);
        }
    }
}

impl<'a> Drop for DrawHandle3D<'a> {
    fn drop(&mut self) {
        unsafe { raylib::ffi::EndMode3D() };
    }
}

// ============================================================================

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
