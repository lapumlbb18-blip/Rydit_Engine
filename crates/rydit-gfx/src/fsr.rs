// crates/rydit-gfx/src/fsr.rs
// 🆕 FSR 1.0 - FidelityFX Super Resolution (2D Simplificado)
// v0.11.4 - Upscale + Sharpening para mejor performance

use gl;
use gl::types::GLuint;
use std::ffi::CString;
use std::fs;
use std::path::Path;

/// Calidad FSR
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FsrQuality {
    Performance, // 0.5x (720p → 1080p) +50% FPS
    Balanced,    // 0.66x (900p → 1080p) +30% FPS
    Quality,     // 0.75x (1080p → 1440p) +20% FPS
}

impl FsrQuality {
    /// Obtener scale factor
    pub fn scale(&self) -> f32 {
        match self {
            FsrQuality::Performance => 0.5,
            FsrQuality::Balanced => 0.66,
            FsrQuality::Quality => 0.75,
        }
    }

    /// Obtener sharpness recomendado
    pub fn sharpness(&self) -> f32 {
        match self {
            FsrQuality::Performance => 0.7, // Más sharpen para compensar upscale
            FsrQuality::Balanced => 0.5,
            FsrQuality::Quality => 0.3,
        }
    }
}

/// FSR Upscaler para 2D
pub struct FsrUpscaler {
    program: GLuint,
    vao: GLuint,
    vbo: GLuint,
    input_size_loc: i32,
    output_size_loc: i32,
    texel_size_loc: i32,
    sharpness_loc: i32,
    quality: FsrQuality,
    enabled: bool,
}

impl FsrUpscaler {
    /// Crear FSR upscaler
    pub fn new() -> Result<Self, String> {
        // Crear fullscreen quad
        let (vao, vbo) = Self::create_quad();

        // Cargar shaders
        let program = Self::load_shaders("shaders/fsr_upscale.glsl", "shaders/fsr_sharpen.glsl")?;

        // Obtener uniform locations
        unsafe {
            gl::UseProgram(program);
            let input_size_loc =
                gl::GetUniformLocation(program, b"inputSize\0".as_ptr() as *const _);
            let output_size_loc =
                gl::GetUniformLocation(program, b"outputSize\0".as_ptr() as *const _);
            let texel_size_loc =
                gl::GetUniformLocation(program, b"texelSize\0".as_ptr() as *const _);
            let sharpness_loc =
                gl::GetUniformLocation(program, b"sharpness\0".as_ptr() as *const _);
            gl::UseProgram(0);

            Ok(Self {
                program,
                vao,
                vbo,
                input_size_loc,
                output_size_loc,
                texel_size_loc,
                sharpness_loc,
                quality: FsrQuality::Performance,
                enabled: true,
            })
        }
    }

    /// Crear fullscreen quad
    fn create_quad() -> (GLuint, GLuint) {
        unsafe {
            let (mut vao, mut vbo) = (0, 0);
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Fullscreen quad vertices
            let vertices: [f32; 18] = [
                -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, 0.0,
                -1.0, 1.0, 0.0,
            ];

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * 4) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindVertexArray(0);
            (vao, vbo)
        }
    }

    /// Cargar shaders desde archivos
    fn load_shaders(vertex_path: &str, fragment_path: &str) -> Result<GLuint, String> {
        unsafe {
            let vertex_source = fs::read_to_string(vertex_path)
                .map_err(|e| format!("Error leyendo vertex shader: {}", e))?;
            let fragment_source = fs::read_to_string(fragment_path)
                .map_err(|e| format!("Error leyendo fragment shader: {}", e))?;

            let vertex_shader =
                Self::compile_shader(gl::VERTEX_SHADER, &vertex_source, "Vertex shader")?;
            let fragment_shader =
                Self::compile_shader(gl::FRAGMENT_SHADER, &fragment_source, "Fragment shader")?;

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

            Ok(program)
        }
    }

    /// Compilar shader
    fn compile_shader(shader_type: GLuint, source: &str, name: &str) -> Result<GLuint, String> {
        unsafe {
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
    }

    /// Render FSR upscale + sharpen
    pub fn render(&self, input_texture: GLuint, input_size: (u32, u32), output_size: (u32, u32)) {
        if !self.enabled {
            return;
        }

        unsafe {
            // Usar programa FSR
            gl::UseProgram(self.program);

            // Set uniforms
            gl::Uniform2f(
                self.input_size_loc,
                input_size.0 as f32,
                input_size.1 as f32,
            );
            gl::Uniform2f(
                self.output_size_loc,
                output_size.0 as f32,
                output_size.1 as f32,
            );
            gl::Uniform2f(
                self.texel_size_loc,
                1.0 / output_size.0 as f32,
                1.0 / output_size.1 as f32,
            );
            gl::Uniform1f(self.sharpness_loc, self.quality.sharpness());

            // Bind input texture
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, input_texture);

            // Render fullscreen quad
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);

            gl::UseProgram(0);
        }
    }

    /// Set calidad
    pub fn set_quality(&mut self, quality: FsrQuality) {
        self.quality = quality;
    }

    /// Obtener calidad actual
    pub fn quality(&self) -> FsrQuality {
        self.quality
    }

    /// Toggle on/off
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Verificar si está activo
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Cycle quality modes
    pub fn cycle_quality(&mut self) {
        self.quality = match self.quality {
            FsrQuality::Performance => FsrQuality::Balanced,
            FsrQuality::Balanced => FsrQuality::Quality,
            FsrQuality::Quality => FsrQuality::Performance,
        };
    }
}

impl Drop for FsrUpscaler {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_scale() {
        assert_eq!(FsrQuality::Performance.scale(), 0.5);
        assert_eq!(FsrQuality::Balanced.scale(), 0.66);
        assert_eq!(FsrQuality::Quality.scale(), 0.75);
    }

    #[test]
    fn test_quality_sharpness() {
        assert!(FsrQuality::Performance.sharpness() > FsrQuality::Quality.sharpness());
    }
}
