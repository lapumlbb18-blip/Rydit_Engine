// crates/rydit-rs/src/modules/window.rs
// Window Manager - Gestión de Ventana para RyDit
//
// Funciones:
// - window::set_title(titulo) - Establecer título de ventana
// - window::get_title() - Obtener título actual
// - window::set_size(width, height) - Cambiar tamaño de ventana
// - window::get_size() - Obtener tamaño actual
// - window::get_width() - Obtener ancho
// - window::get_height() - Obtener alto
// - window::set_fullscreen(enabled) - Activar/desactivar fullscreen
// - window::is_fullscreen() - Verificar si está en fullscreen
// - window::toggle_fullscreen() - Alternar fullscreen
// - window::set_windowed() - Forzar modo ventana
// - window::set_vsync(enabled) - Activar/desactivar VSync
// - window::is_vsync_enabled() - Verificar VSync
// - window::set_resizable(enabled) - Hacer ventana redimensionable
// - window::minimize() - Minimizar ventana
// - window::maximize() - Maximizar ventana
// - window::restore() - Restaurar ventana
// - window::set_fps_limit(fps) - Establecer límite de FPS
// - window::get_fps() - Obtener FPS actuales
// - window::get_delta_time() - Obtener delta time

use blast_core::{Executor, Valor};
use lizer::{Expr, Stmt};
use rydit_gfx::RyditGfx;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::eval::evaluar_expr;

// ============================================================================
// WINDOW MANAGER STRUCT
// ============================================================================

/// Gestor de ventana
pub struct WindowManager {
    /// Título de la ventana
    pub title: String,
    /// Ancho de ventana
    pub width: u32,
    /// Alto de ventana
    pub height: u32,
    /// Si está en fullscreen
    pub fullscreen: bool,
    /// Si VSync está activado
    pub vsync: bool,
    /// Si es redimensionable
    pub resizable: bool,
    /// Límite de FPS
    pub fps_limit: u32,
    /// Referencia a RyditGfx (para operaciones reales)
    gfx: Option<Rc<RefCell<RyditGfx>>>,
}

impl WindowManager {
    /// Crear nuevo Window Manager
    pub fn new() -> Self {
        Self {
            title: "RyDit Engine".to_string(),
            width: 800,
            height: 600,
            fullscreen: false,
            vsync: true,
            resizable: true,
            fps_limit: 60,
            gfx: None,
        }
    }

    /// Establecer referencia a RyditGfx
    pub fn set_gfx(&mut self, gfx: Rc<RefCell<RyditGfx>>) {
        self.gfx = Some(gfx);
    }

    /// Establecer título de ventana
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        // En el futuro: gfx.set_window_title(title)
    }

    /// Obtener título actual
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    /// Establecer tamaño de ventana
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        // En el futuro: gfx.set_window_size(width, height)
    }

    /// Obtener tamaño actual
    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Establecer fullscreen
    pub fn set_fullscreen(&mut self, enabled: bool) {
        self.fullscreen = enabled;
        // En el futuro: gfx.set_fullscreen(enabled)
    }

    /// Verificar si está en fullscreen
    pub fn is_fullscreen(&self) -> bool {
        self.fullscreen
    }

    /// Alternar fullscreen
    pub fn toggle_fullscreen(&mut self) {
        self.set_fullscreen(!self.fullscreen);
    }

    /// Forzar modo ventana
    pub fn set_windowed(&mut self) {
        self.set_fullscreen(false);
    }

    /// Establecer VSync
    pub fn set_vsync(&mut self, enabled: bool) {
        self.vsync = enabled;
        // En el futuro: gfx.set_vsync(enabled)
    }

    /// Verificar si VSync está activado
    pub fn is_vsync_enabled(&self) -> bool {
        self.vsync
    }

    /// Establecer si es redimensionable
    pub fn set_resizable(&mut self, enabled: bool) {
        self.resizable = enabled;
    }

    /// Minimizar ventana
    pub fn minimize(&self) {
        // En el futuro: implementar minimize real
    }

    /// Maximizar ventana
    pub fn maximize(&self) {
        // En el futuro: implementar maximize real
    }

    /// Restaurar ventana
    pub fn restore(&self) {
        // En el futuro: implementar restore real
    }

    /// Establecer límite de FPS
    pub fn set_fps_limit(&mut self, fps: u32) {
        self.fps_limit = fps;
        if let Some(ref gfx) = self.gfx {
            let mut gfx_ref = gfx.borrow_mut();
            gfx_ref.set_target_fps(fps as i32);
        }
    }

    /// Obtener FPS actuales
    pub fn get_fps(&self) -> u32 {
        if let Some(ref gfx) = self.gfx {
            let gfx_ref = gfx.borrow();
            gfx_ref.get_fps() as u32
        } else {
            0
        }
    }

    /// Obtener delta time
    #[allow(dead_code)] // Para futura implementación de delta time
    pub fn get_delta_time(&self) -> f32 {
        // En el futuro: implementar delta time real
        0.016  // 60 FPS por defecto (1/60 = 0.016)
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ESTADO GLOBAL
// ============================================================================

thread_local! {
    static WINDOW: Rc<RefCell<WindowManager>> = Rc::new(RefCell::new(WindowManager::new()));
}

/// Obtener referencia al Window Manager
pub fn get_window() -> Rc<RefCell<WindowManager>> {
    WINDOW.with(|w| w.clone())
}

/// Inicializar Window Manager con referencia a RyditGfx
#[allow(dead_code)] // Para futura integración con el game loop
pub fn init_window_manager(gfx: Rc<RefCell<RyditGfx>>) {
    WINDOW.with(|w| {
        let mut w_ref = w.borrow_mut();
        w_ref.set_gfx(gfx);
    });
}

// ============================================================================
// FUNCIONES PARA .rydit
// ============================================================================

/// window::set_title(titulo) - Establecer título
pub fn window_set_title(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("window::set_title() requiere 1 argumento: titulo".to_string());
    }

    let titulo_val = evaluar_expr(&args[0], executor, funcs);

    let titulo = match titulo_val {
        Valor::Texto(t) => t,
        _ => return Valor::Error("window::set_title() titulo debe ser texto".to_string()),
    };

    let win = get_window();
    let mut win_ref = win.borrow_mut();
    win_ref.set_title(&titulo);

    Valor::Texto(format!("window::set_title(\"{}\")", titulo))
}

/// window::get_title() - Obtener título
pub fn window_get_title(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let win = get_window();
    let win_ref = win.borrow();
    Valor::Texto(win_ref.get_title())
}

/// window::set_size(width, height) - Establecer tamaño
pub fn window_set_size(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("window::set_size() requiere 2 argumentos: width, height".to_string());
    }

    let width_val = evaluar_expr(&args[0], executor, funcs);
    let height_val = evaluar_expr(&args[1], executor, funcs);

    let width = match width_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("window::set_size() width debe ser número".to_string()),
    };

    let height = match height_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("window::set_size() height debe ser número".to_string()),
    };

    let win = get_window();
    let mut win_ref = win.borrow_mut();
    win_ref.set_size(width, height);

    Valor::Texto(format!("window::set_size({}, {})", width, height))
}

/// window::get_size() - Obtener tamaño
pub fn window_get_size(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let win = get_window();
    let win_ref = win.borrow();
    let (w, h) = win_ref.get_size();
    Valor::Array(vec![Valor::Num(w as f64), Valor::Num(h as f64)])
}

/// window::get_width() - Obtener ancho
pub fn window_get_width(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let win = get_window();
    let win_ref = win.borrow();
    let (w, _) = win_ref.get_size();
    Valor::Num(w as f64)
}

/// window::get_height() - Obtener alto
pub fn window_get_height(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let win = get_window();
    let win_ref = win.borrow();
    let (_, h) = win_ref.get_size();
    Valor::Num(h as f64)
}

/// window::set_fullscreen(enabled) - Establecer fullscreen
pub fn window_set_fullscreen(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("window::set_fullscreen() requiere 1 argumento: enabled".to_string());
    }

    let enabled_val = evaluar_expr(&args[0], executor, funcs);

    let enabled = match enabled_val {
        Valor::Bool(e) => e,
        Valor::Num(n) => n != 0.0,
        _ => return Valor::Error("window::set_fullscreen() enabled debe ser booleano o número".to_string()),
    };

    let win = get_window();
    let mut win_ref = win.borrow_mut();
    win_ref.set_fullscreen(enabled);

    Valor::Texto(format!("window::set_fullscreen({})", enabled))
}

/// window::is_fullscreen() - Verificar fullscreen
pub fn window_is_fullscreen(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let win = get_window();
    let win_ref = win.borrow();
    Valor::Bool(win_ref.is_fullscreen())
}

/// window::toggle_fullscreen() - Alternar fullscreen
pub fn window_toggle_fullscreen(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let win = get_window();
    let mut win_ref = win.borrow_mut();
    win_ref.toggle_fullscreen();
    Valor::Texto("window::toggle_fullscreen()".to_string())
}

/// window::set_windowed() - Forzar modo ventana
pub fn window_set_windowed(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let win = get_window();
    let mut win_ref = win.borrow_mut();
    win_ref.set_windowed();
    Valor::Texto("window::set_windowed()".to_string())
}

/// window::set_vsync(enabled) - Establecer VSync
pub fn window_set_vsync(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("window::set_vsync() requiere 1 argumento: enabled".to_string());
    }

    let enabled_val = evaluar_expr(&args[0], executor, funcs);

    let enabled = match enabled_val {
        Valor::Bool(e) => e,
        Valor::Num(n) => n != 0.0,
        _ => return Valor::Error("window::set_vsync() enabled debe ser booleano o número".to_string()),
    };

    let win = get_window();
    let mut win_ref = win.borrow_mut();
    win_ref.set_vsync(enabled);

    Valor::Texto(format!("window::set_vsync({})", enabled))
}

/// window::is_vsync_enabled() - Verificar VSync
pub fn window_is_vsync_enabled(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let win = get_window();
    let win_ref = win.borrow();
    Valor::Bool(win_ref.is_vsync_enabled())
}

/// window::set_resizable(enabled) - Establecer redimensionable
pub fn window_set_resizable(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("window::set_resizable() requiere 1 argumento: enabled".to_string());
    }

    let enabled_val = evaluar_expr(&args[0], executor, funcs);

    let enabled = match enabled_val {
        Valor::Bool(e) => e,
        Valor::Num(n) => n != 0.0,
        _ => return Valor::Error("window::set_resizable() enabled debe ser booleano o número".to_string()),
    };

    let win = get_window();
    let mut win_ref = win.borrow_mut();
    win_ref.set_resizable(enabled);

    Valor::Texto(format!("window::set_resizable({})", enabled))
}

/// window::minimize() - Minimizar ventana
pub fn window_minimize(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let win = get_window();
    let win_ref = win.borrow();
    win_ref.minimize();
    Valor::Texto("window::minimize()".to_string())
}

/// window::maximize() - Maximizar ventana
pub fn window_maximize(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let win = get_window();
    let win_ref = win.borrow();
    win_ref.maximize();
    Valor::Texto("window::maximize()".to_string())
}

/// window::restore() - Restaurar ventana
pub fn window_restore(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let win = get_window();
    let win_ref = win.borrow();
    win_ref.restore();
    Valor::Texto("window::restore()".to_string())
}

/// window::set_fps_limit(fps) - Establecer límite de FPS
pub fn window_set_fps_limit(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("window::set_fps_limit() requiere 1 argumento: fps".to_string());
    }

    let fps_val = evaluar_expr(&args[0], executor, funcs);

    let fps = match fps_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("window::set_fps_limit() fps debe ser número".to_string()),
    };

    let win = get_window();
    let mut win_ref = win.borrow_mut();
    win_ref.set_fps_limit(fps);

    Valor::Texto(format!("window::set_fps_limit({})", fps))
}

/// window::get_fps() - Obtener FPS actuales
pub fn window_get_fps(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let win = get_window();
    let win_ref = win.borrow();
    Valor::Num(win_ref.get_fps() as f64)
}

/// window::get_delta_time() - Obtener delta time
pub fn window_get_delta_time(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let win = get_window();
    let win_ref = win.borrow();
    Valor::Num(win_ref.get_delta_time() as f64)
}
