// crates/rydit-gfx/src/backend_sdl2.rs
// SDL2 Backend para RyDit - Ventana + OpenGL Context + Input
// ✅ v0.10.6: Backend completo para Android/Termux-X11

#![allow(clippy::too_many_arguments)]

use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::GLProfile;

use crate::input_sdl2::InputState;
use crate::sdl2_ffi::FontFFI;
use crate::ColorRydit;

// ============================================================================
// SDL2 BACKEND
// ============================================================================

/// Backend SDL2 completo para RyDit
pub struct Sdl2Backend {
    /// Contexto SDL2
    pub context: sdl2::Sdl,
    /// Subsistema de video
    pub video_subsystem: sdl2::VideoSubsystem,
    /// Canvas para render 2D (incluye la ventana)
    pub canvas: Canvas<sdl2::video::Window>,
    /// Event pump para input
    pub event_pump: sdl2::EventPump,
    /// Estado del input
    pub input: InputState,
    /// Dimensiones de la ventana
    pub width: i32,
    pub height: i32,
    /// Título de la ventana
    pub title: String,
    /// Contexto OpenGL (para GPU Instancing)
    pub gl_context: Option<sdl2::video::GLContext>,
    /// TextureCreator para texturas 2D
    pub texture_creator: TextureCreator<sdl2::video::WindowContext>,
    /// Fuente SDL2_ttf (opcional)
    pub font: Option<FontFFI>,
}

impl Sdl2Backend {
    /// Crear nuevo backend SDL2
    pub fn new(title: &str, width: i32, height: i32) -> Result<Self, String> {
        // Inicializar SDL2
        let context = sdl2::init().map_err(|e| e.to_string())?;
        let video_subsystem = context.video().map_err(|e| e.to_string())?;

        // Configurar OpenGL (versión 3.3 Core)
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        gl_attr.set_double_buffer(true);
        gl_attr.set_multisample_samples(4); // Anti-aliasing

        // Crear ventana OpenGL
        let window = video_subsystem
            .window(title, width as u32, height as u32)
            .position_centered()
            .opengl()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        // Crear contexto OpenGL
        let gl_context = window.gl_create_context().map_err(|e| e.to_string())?;

        window
            .gl_make_current(&gl_context)
            .map_err(|e| e.to_string())?;

        // Cargar extensiones OpenGL (usamos video_subsystem)
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        // Inicializar SDL2_image (PNG, JPG) - GIF no disponible en esta versión
        let _image_context =
            sdl2::image::init(InitFlag::PNG | InitFlag::JPG).map_err(|e| e.to_string())?;

        // Crear canvas para render 2D
        let canvas = window
            .into_canvas()
            .present_vsync() // VSync activado
            .build()
            .map_err(|e| e.to_string())?;

        // Obtener event pump
        let event_pump = context.event_pump().map_err(|e| e.to_string())?;

        // Crear TextureCreator para texturas 2D
        let texture_creator = canvas.texture_creator();

        // Inicializar SDL2_ttf (opcional, sin font por defecto)
        let font = match FontFFI::init() {
            Ok(_) => {
                println!("[SDL2-BACKEND]: SDL2_ttf inicializado");
                None // Font se carga bajo demanda
            }
            Err(e) => {
                println!("[SDL2-BACKEND]: SDL2_ttf no disponible: {}", e);
                None
            }
        };

        println!("[SDL2-BACKEND]: Ventana creada {}x{}", width, height);
        println!("[SDL2-BACKEND]: OpenGL contexto 3.3 Core");
        println!("[SDL2-BACKEND]: VSync activado");
        println!("[SDL2-BACKEND]: SDL2_image inicializado (PNG, JPG)");

        Ok(Self {
            context,
            video_subsystem,
            canvas,
            event_pump,
            input: InputState::new(),
            width,
            height,
            title: title.to_string(),
            gl_context: Some(gl_context),
            texture_creator,
            font,
        })
    }

    /// Procesar eventos SDL2 (debe llamarse en cada frame)
    pub fn procesar_eventos(&mut self) -> bool {
        // Limpiar eventos del frame anterior
        self.input.limpiar_frame();

        let mut should_close = false;

        // Procesar eventos
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    should_close = true;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => {
                    self.input.teclas.insert(keycode, true);
                    self.input.teclas_pressionadas_frame.push(keycode);

                    // ESC cierra la ventana
                    if keycode == Keycode::Escape {
                        should_close = true;
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    self.input.teclas.insert(keycode, false);
                }
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    self.width = w as i32;
                    self.height = h as i32;
                    let viewport = Rect::new(0, 0, w as u32, h as u32);
                    self.canvas.set_viewport(viewport);
                }
                _ => {}
            }
        }

        should_close
    }

    /// Iniciar frame de renderizado
    pub fn begin_draw(&mut self) {
        // Limpiar pantalla (negro por defecto)
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present(); // Presentar inmediatamente después de clear
    }

    /// Limpiar fondo con color
    pub fn clear_background(&mut self, color: ColorRydit) {
        let (r, g, b) = color.to_rgb();
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        self.canvas.clear();
    }

    /// Finalizar frame de renderizado
    pub fn end_draw(&mut self) {
        self.canvas.present();
    }

    /// Verificar si la ventana debe cerrarse
    pub fn should_close(&self) -> bool {
        // SDL2 no tiene un método directo, usamos una bandera
        false // El cierre se maneja en procesar_eventos()
    }

    /// Obtener estado de una tecla (por nombre RyDit)
    pub fn is_key_pressed(&self, nombre: &str) -> bool {
        self.input.is_key_pressed(nombre)
    }

    /// Obtener estado de una tecla (este frame)
    pub fn is_key_just_pressed(&self, nombre: &str) -> bool {
        self.input.is_key_just_pressed(nombre)
    }

    /// Dibujar rectángulo
    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        let rect = sdl2::rect::Rect::new(x, y, w as u32, h as u32);
        self.canvas.fill_rect(rect).unwrap();
    }

    /// Dibujar rectángulo con ColorRydit
    pub fn draw_rect_color(&mut self, x: i32, y: i32, w: i32, h: i32, color: ColorRydit) {
        let (r, g, b) = color.to_rgb();
        self.draw_rect(x, y, w, h, r, g, b);
    }

    /// Dibujar círculo (aproximación con rectángulos)
    pub fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, r: u8, g: u8, b: u8) {
        // Círculo simple usando fill_rect
        let diameter = radius * 2;
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        let rect = Rect::new(cx - radius, cy - radius, diameter as u32, diameter as u32);
        let _ = self.canvas.fill_rect(rect); // Ignorar error
    }

    /// Cargar fuente SDL2_ttf
    pub fn load_font(&mut self, path: &str, size: i32) -> Result<(), String> {
        let font = FontFFI::load(path, size)?;
        self.font = Some(font);
        println!("[SDL2-BACKEND]: Fuente cargada: {} ({}px)", path, size);
        Ok(())
    }

    /// Dibujar texto con SDL2_ttf
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, _size: u16, r: u8, g: u8, b: u8) {
        // Verificar que tenemos una fuente cargada
        let font = match &self.font {
            Some(f) => f,
            None => {
                // Fallback: dibujar rectángulo placeholder
                let width = text.len() as i32 * 10;
                self.draw_rect(x, y, width, 16, r, g, b);
                return;
            }
        };

        // Renderizar texto a superficie (usar Blended para alpha correcto)
        let surface_ptr = match font.render_text_blended(text, r, g, b) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[SDL2-BACKEND]: Error renderizando texto: {}", e);
                return;
            }
        };

        unsafe {
            // Crear wrapper Surface alrededor del raw pointer
            // La superficie será liberada cuando el wrapper se dropee
            let sdl_surface = Surface::from_ll(surface_ptr as *mut sdl2::sys::SDL_Surface);


            // Crear textura desde superficie
            let texture = match self.texture_creator.create_texture_from_surface(&sdl_surface) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("[SDL2-BACKEND]: Error creando textura: {}", e);
                    return;
                }
            };

            // Obtener dimensiones de la textura (usar query())
            let query = texture.query();
            let dst_rect = Rect::new(x, y, query.width, query.height);
            let _ = self.canvas.copy(&texture, None, dst_rect);
        }
    }

    /// Dibujar texto con ColorRydit
    pub fn draw_text_color(&mut self, text: &str, x: i32, y: i32, size: u16, color: ColorRydit) {
        let (r, g, b) = color.to_rgb();
        self.draw_text(text, x, y, size, r, g, b);
    }

    /// Obtener FPS objetivo (vsync = 60)
    pub fn get_target_fps(&self) -> i32 {
        60
    }

    /// Obtener FPS reales (aproximado)
    pub fn get_fps(&self) -> i32 {
        // SDL2 no tiene get_fps(), usamos 60 por vsync
        60
    }
}

// ============================================================================
// GESTOR DE TEXTURAS SDL2 (v0.10.7) - PENDIENTE
// ============================================================================

/// Gestor de texturas con SDL2_image
/// ⚠️ PENDIENTE: El linking de SDL2_image es complejo, se implementará en v0.10.8
pub struct TextureManager;

impl TextureManager {
    pub fn new() -> Self {
        Self
    }

    /// Cargar superficie desde archivo (SDL2_image)
    /// ⚠️ No implementado aún - linking pendiente
    pub fn load_surface(_path: &str) -> Result<sdl2::surface::Surface<'static>, String> {
        Err("SDL2_image linking pendiente - v0.10.8".to_string())
    }

    /// Dibujar textura desde superficie
    pub fn draw_texture_from_surface(
        _surface: &sdl2::surface::Surface,
        _canvas: &mut Canvas<sdl2::video::Window>,
        _x: i32,
        _y: i32,
        _width: u32,
        _height: u32,
    ) -> Result<(), String> {
        Err("SDL2_image linking pendiente - v0.10.8".to_string())
    }

    /// Cargar y dibujar textura inmediatamente
    pub fn draw_texture(
        _path: &str,
        _canvas: &mut Canvas<sdl2::video::Window>,
        _x: i32,
        _y: i32,
        _width: u32,
        _height: u32,
    ) -> Result<(), String> {
        Err("SDL2_image linking pendiente - v0.10.8".to_string())
    }

    /// Contar texturas (siempre 0, no almacenamos)
    pub fn count(&self) -> usize {
        0
    }
}

impl Default for TextureManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_manager() {
        let manager = TextureManager::new();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_input_state() {
        let input = InputState::new();
        assert!(!input.alguna_tecla_presionada());
    }
}
