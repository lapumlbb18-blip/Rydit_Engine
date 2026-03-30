// crates/rydit-gfx/src/gpu_instancing.rs
// GPU Instancing - 100K+ partículas en un solo draw call
// v0.10.1: FFI OpenGL + Shaders GLSL

use gl::types::{GLuint, GLsizei, GLint};
use std::ffi::CString;
use std::fs;
use std::path::Path;

// ============================================================================
// DATOS DE PARTÍCULA PARA GPU
// ============================================================================

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ParticleData {
    pub offset: [f32; 2],    // Posición (x, y)
    pub size: f32,            // Tamaño
    pub color: [f32; 4],      // Color (r, g, b, a)
}

impl ParticleData {
    pub fn new(x: f32, y: f32, size: f32, r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            offset: [x, y],
            size,
            color: [r, g, b, a],
        }
    }
}

// ============================================================================
// GPU INSTANCER
// ============================================================================

pub struct GPUInstancer {
    program: GLuint,
    vao: GLuint,
    vbo: GLuint,
    instance_vbo: GLuint,
    particle_count: usize,
    projection: [f32; 16],
    camera: [f32; 2],
}

impl GPUInstancer {
    /// Crear nuevo GPU Instancer
    pub fn new() -> Self {
        unsafe {
            // Crear VAO
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            
            // Crear VBO para quad geometry
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            
            // Quad de 4 vértices (2 triángulos) - 16 floats
            let quad_vertices: [f32; 16] = [
                // Vértice 1
                -0.5_f32, -0.5_f32, 0.0_f32, 1.0_f32,
                // Vértice 2
                 0.5_f32, -0.5_f32, 0.0_f32, 1.0_f32,
                // Vértice 3
                 0.5_f32,  0.5_f32, 0.0_f32, 1.0_f32,
                // Vértice 4
                -0.5_f32,  0.5_f32, 0.0_f32, 1.0_f32,
            ];
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (quad_vertices.len() * std::mem::size_of::<f32>()) as isize,
                quad_vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            
            // Crear instance VBO para datos de partículas
            let mut instance_vbo = 0;
            gl::GenBuffers(1, &mut instance_vbo);
            
            // Configurar atributo de posición (location = 0)
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (2 * std::mem::size_of::<f32>()) as GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            
            // Configurar atributos de instancia (location = 1, 2, 3)
            // Offset (location = 1)
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<ParticleData>() as GLsizei,
                std::ptr::null(),
            );
            gl::VertexAttribDivisor(1, 1);
            gl::EnableVertexAttribArray(1);
            
            // Size (location = 2)
            gl::VertexAttribPointer(
                2,
                1,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<ParticleData>() as GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::VertexAttribDivisor(2, 1);
            gl::EnableVertexAttribArray(2);
            
            // Color (location = 3)
            gl::VertexAttribPointer(
                3,
                4,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<ParticleData>() as GLsizei,
                (3 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::VertexAttribDivisor(3, 1);
            gl::EnableVertexAttribArray(3);
            
            gl::BindVertexArray(0);
            
            Self {
                program: 0,
                vao,
                vbo,
                instance_vbo,
                particle_count: 0,
                projection: [0.0; 16],
                camera: [0.0; 2],
            }
        }
    }
    
    /// Cargar shaders desde archivos
    pub fn load_shaders<P: AsRef<Path>>(&mut self, vertex_path: P, fragment_path: P) -> Result<(), String> {
        unsafe {
            let vertex_source = fs::read_to_string(vertex_path)
                .map_err(|e| format!("Error leyendo vertex shader: {}", e))?;
            let fragment_source = fs::read_to_string(fragment_path)
                .map_err(|e| format!("Error leyendo fragment shader: {}", e))?;
            
            let vertex_shader = self.compile_shader(
                gl::VERTEX_SHADER,
                &vertex_source,
                "Vertex shader"
            )?;
            
            let fragment_shader = self.compile_shader(
                gl::FRAGMENT_SHADER,
                &fragment_source,
                "Fragment shader"
            )?;
            
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            
            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = vec![0u8; len as usize];
                gl::GetProgramInfoLog(program, len, &mut len, buf.as_mut_ptr() as *mut _);
                let msg = String::from_utf8_lossy(&buf[..len as usize]);
                gl::DeleteProgram(program);
                return Err(format!("Error linkeando programa: {}", msg));
            }
            
            gl::DetachShader(program, vertex_shader);
            gl::DetachShader(program, fragment_shader);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            
            self.program = program;
            Ok(())
        }
    }
    
    unsafe fn compile_shader(&self, shader_type: GLuint, source: &str, name: &str) -> Result<GLuint, String> {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(source).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
        
        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0u8; len as usize];
            gl::GetShaderInfoLog(shader, len, &mut len, buf.as_mut_ptr() as *mut _);
            let msg = String::from_utf8_lossy(&buf[..len as usize]);
            gl::DeleteShader(shader);
            return Err(format!("Error compilando {}: {}", name, msg));
        }
        
        Ok(shader)
    }
    
    pub fn set_particles(&mut self, particles: &[ParticleData]) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.instance_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (particles.len() * std::mem::size_of::<ParticleData>()) as isize,
                particles.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );
            self.particle_count = particles.len();
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
    
    pub fn set_projection(&mut self, width: f32, height: f32) {
        self.projection = [
            2.0 / width, 0.0, 0.0, 0.0,
            0.0, -2.0 / height, 0.0, 0.0,
            0.0, 0.0, -1.0, 0.0,
            -1.0, 1.0, 0.0, 1.0,
        ];
    }
    
    pub fn set_camera(&mut self, x: f32, y: f32) {
        self.camera = [x, y];
    }
    
    pub fn draw(&self) {
        if self.particle_count == 0 {
            return;
        }
        
        unsafe {
            gl::UseProgram(self.program);
            
            let proj_loc = gl::GetUniformLocation(self.program, b"uProjection\0".as_ptr() as *const _);
            let cam_loc = gl::GetUniformLocation(self.program, b"uCamera\0".as_ptr() as *const _);
            
            gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, self.projection.as_ptr());
            gl::Uniform2f(cam_loc, self.camera[0], self.camera[1]);
            
            gl::BindVertexArray(self.vao);
            gl::DrawArraysInstanced(gl::QUADS, 0, 4, self.particle_count as GLsizei);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }
    
    pub fn cleanup(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.instance_vbo);
            gl::DeleteVertexArrays(1, &self.vao);
            if self.program != 0 {
                gl::DeleteProgram(self.program);
            }
        }
    }
}

impl Default for GPUInstancer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for GPUInstancer {
    fn drop(&mut self) {
        self.cleanup();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_data_creation() {
        let p = ParticleData::new(100.0, 200.0, 10.0, 1.0, 0.0, 0.0, 1.0);
        assert_eq!(p.offset, [100.0, 200.0]);
        assert_eq!(p.size, 10.0);
        assert_eq!(p.color, [1.0, 0.0, 0.0, 1.0]);
    }
}
