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

// Importar migui para el backend
#[cfg(feature = "migui")]
use migui::{Color as MiguiColor, MiguiBackend, Rect as MiguiRect};

// ... (en la implementación de Sdl2Backend)

#[cfg(feature = "migui")]
impl MiguiBackend for Sdl2Backend {
    fn clear(&mut self, color: MiguiColor) {
        self.canvas.set_draw_color(Color::RGBA(color.r, color.g, color.b, color.a));
        self.canvas.clear();
    }

    fn draw_rect(&mut self, rect: MiguiRect, color: MiguiColor) {
        self.canvas.set_draw_color(Color::RGBA(color.r, color.g, color.b, color.a));
        let sdl_rect = Rect::new(rect.x as i32, rect.y as i32, rect.w as u32, rect.h as u32);
        let _ = self.canvas.fill_rect(sdl_rect);
    }

    fn draw_text(&mut self, text: &str, x: f32, y: f32, size: f32, color: MiguiColor) {
        self.draw_text(text, x as i32, y as i32, size as u16, color.r, color.g, color.b);
    }

    fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: MiguiColor, thickness: f32) {
        self.canvas.set_draw_color(Color::RGBA(color.r, color.g, color.b, color.a));
        // SDL2 nativo no tiene line_thickness directo en canvas básico, dibujamos línea simple
        let _ = self.canvas.draw_line(
            sdl2::rect::Point::new(x1 as i32, y1 as i32),
            sdl2::rect::Point::new(x2 as i32, y2 as i32)
        );
    }
}

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
                    self.width = w;
                    self.height = h;
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

    /// Renderizar comandos de migui usando el backend de SDL2
    pub fn render_migui_commands(
        &mut self, 
        commands: &[migui::DrawCommand], 
        viewports: &mut crate::viewports::ViewportManager
    ) {
        for cmd in commands {
            match cmd {
                migui::DrawCommand::Clear { color } => {
                    self.canvas.set_draw_color(Color::RGBA(color.r, color.g, color.b, color.a));
                    self.canvas.clear();
                }
                migui::DrawCommand::DrawRect { rect, color } => {
                    self.canvas.set_draw_color(Color::RGBA(color.r, color.g, color.b, color.a));
                    let r = Rect::new(rect.x as i32, rect.y as i32, rect.w as u32, rect.h as u32);
                    let _ = self.canvas.fill_rect(r);
                }
                migui::DrawCommand::DrawText { text, x, y, size, color } => {
                    self.draw_text(text, *x as i32, *y as i32, *size as u16, color.r, color.g, color.b);
                }
                migui::DrawCommand::DrawLine { x1, y1, x2, y2, color, thickness: _ } => {
                    self.canvas.set_draw_color(Color::RGBA(color.r, color.g, color.b, color.a));
                    let _ = self.canvas.draw_line(
                        sdl2::rect::Point::new(*x1 as i32, *y1 as i32),
                        sdl2::rect::Point::new(*x2 as i32, *y2 as i32)
                    );
                }
                migui::DrawCommand::DrawViewport3D { id, rect } => {
                    if let Some(vp) = viewports.viewports.get_mut(id) {
                        // 1. Iniciar modo textura en Raylib (vía FFI para evitar necesidad de RaylibHandle)
                        // Esto asume que el contexto OpenGL es el mismo.
                        unsafe {
                            raylib::ffi::BeginTextureMode(vp.target);
                            raylib::ffi::ClearBackground(raylib::ffi::Color { r: 30, g: 30, b: 35, a: 255 });
                            
                            // --- DIBUJO RAYLIB (Fusión) ---
                            // Aquí se inyectan comandos de raylib::ffi
                            if vp.grid_enabled {
                                // Dibujar grid minimalista vía FFI
                                let grid_color = raylib::ffi::Color { r: 60, g: 60, b: 70, a: 100 };
                                raylib::ffi::DrawLine(0, (vp.height/2) as i32, vp.width as i32, (vp.height/2) as i32, grid_color);
                            }
                            
                            raylib::ffi::EndTextureMode();
                        }

                        // 2. Puente: Dibujar el buffer de Raylib en SDL2
                        // En un sistema con contexto compartido, podemos usar la ID de textura de Raylib directamente.
                        // Para esta implementación, usaremos un rectángulo de color como "zona de influencia"
                        // hasta que el sistema de FFI esté 100% linkeado con la 6.0.
                        let r = Rect::new(rect.x as i32, rect.y as i32, rect.w as u32, rect.h as u32);
                        self.canvas.set_draw_color(Color::RGB(40, 40, 50));
                        let _ = self.canvas.fill_rect(r);
                        
                        // Borde del viewport
                        self.canvas.set_draw_color(Color::RGB(100, 100, 255));
                        let _ = self.canvas.draw_rect(r);
                        
                        // Etiqueta del viewport
                        self.draw_text(&format!("Viewport: {}", id), rect.x as i32 + 5, rect.y as i32 + 5, 12, 100, 100, 255);
                    }
                }
            }
        }
    }

    /// Sincroniza eventos de SDL2 directamente con el InputManager de events-ry
    pub fn actualizar_input_unificado(&mut self, manager: &mut events_ry::InputManager) {
        manager.begin_frame();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    manager.inject_event(events_ry::InputEvent::WindowCloseRequested);
                }
                Event::KeyDown { keycode: Some(k), .. } => {
                    // Mapeo simple de Keycode -> Key (mejorar con tabla completa)
                    if let Some(ry_key) = map_sdl_keycode_to_ry(k) {
                        manager.inject_event(events_ry::InputEvent::KeyPressed { key: ry_key });
                    }
                }
                Event::KeyUp { keycode: Some(k), .. } => {
                    if let Some(ry_key) = map_sdl_keycode_to_ry(k) {
                        manager.inject_event(events_ry::InputEvent::KeyReleased { key: ry_key });
                    }
                }
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    let btn = match mouse_btn {
                        sdl2::mouse::MouseButton::Left => events_ry::MouseButton::Left,
                        sdl2::mouse::MouseButton::Right => events_ry::MouseButton::Right,
                        _ => events_ry::MouseButton::Middle,
                    };
                    manager.inject_event(events_ry::InputEvent::MousePressed { x, y, button: btn });
                }
                Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                    let btn = match mouse_btn {
                        sdl2::mouse::MouseButton::Left => events_ry::MouseButton::Left,
                        sdl2::mouse::MouseButton::Right => events_ry::MouseButton::Right,
                        _ => events_ry::MouseButton::Middle,
                    };
                    manager.inject_event(events_ry::InputEvent::MouseReleased { x, y, button: btn });
                }
                Event::MouseMotion { x, y, .. } => {
                    manager.inject_event(events_ry::InputEvent::MouseMoved { x, y });
                }
                _ => {}
            }
        }
    }
}

/// Helper interno para mapear Keycodes de SDL2 a los unificados de RyDit
fn map_sdl_keycode_to_ry(k: sdl2::keyboard::Keycode) -> Option<events_ry::Key> {
    use sdl2::keyboard::Keycode as S;
    use events_ry::Key as R;
    match k {
        S::A => Some(R::A), S::B => Some(R::B), S::C => Some(R::C), S::D => Some(R::D),
        S::E => Some(R::E), S::F => Some(R::F), S::G => Some(R::G), S::H => Some(R::H),
        S::I => Some(R::I), S::J => Some(R::J), S::K => Some(R::K), S::L => Some(R::L),
        S::M => Some(R::M), S::N => Some(R::N), S::O => Some(R::O), S::P => Some(R::P),
        S::Q => Some(R::Q), S::R => Some(R::R), S::S => Some(R::S), S::T => Some(R::T),
        S::U => Some(R::U), S::V => Some(R::V), S::W => Some(R::W), S::X => Some(R::X),
        S::Y => Some(R::Y), S::Z => Some(R::Z),
        S::Escape => Some(R::Escape),
        S::Space => Some(R::Space),
        S::Return => Some(R::Enter),
        _ => None,
    }
}

// ============================================================================
// ASSET PROVIDER (SDL2)
// ============================================================================

use ry_loader::AssetProvider;

/// Adaptador para usar SDL2 como proveedor de assets
pub struct Sdl2AssetProvider;

impl AssetProvider for Sdl2AssetProvider {
    fn load_texture(&self, path: &str) -> Result<Vec<u8>, String> {
        // Carga de bytes crudos desde el sistema de archivos
        std::fs::read(path).map_err(|e| format!("Error leyendo textura {}: {}", path, e))
    }

    fn load_audio(&self, path: &str) -> Result<Vec<u8>, String> {
        // Carga de bytes crudos para audio (wav, ogg, mp3)
        std::fs::read(path).map_err(|e| format!("Error leyendo audio {}: {}", path, e))
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
