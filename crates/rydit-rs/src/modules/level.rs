// crates/rydit-rs/src/modules/level.rs
// Level Manager - Gestión de Niveles para RyDit
//
// Funciones:
// - level::load("nivel.rydit") - Cargar nivel
// - level::unload() - Descargar nivel actual
// - level::transition("nivel2.rydit") - Transición a otro nivel
// - level::get_current() - Obtener nivel actual
// - level::reload() - Recargar nivel actual
// - level::get_name() - Obtener nombre del nivel
// - level::set_checkpoint(name, x, y) - Establecer checkpoint
// - level::load_checkpoint(name) - Cargar checkpoint
// - level::get_checkpoint(name) - Obtener posición de checkpoint
// - level::list_checkpoints() - Listar checkpoints
// - level::transition_fade(duration) - Transición fade
// - level::transition_slide(direction) - Transición slide

use blast_core::{Executor, Valor};
use lizer::{Expr, Stmt};
use std::collections::HashMap;
use std::fs;

use crate::eval::evaluar_expr;

// ============================================================================
// LEVEL MANAGER STRUCT
// ============================================================================

/// Gestor de niveles
#[derive(Debug)]
pub struct LevelManager {
    /// Nivel actual cargado
    pub current_level: Option<String>,
    /// Ruta del nivel actual
    pub current_path: Option<String>,
    /// Checkpoints registrados
    pub checkpoints: HashMap<String, (f32, f32)>,
    /// Datos del nivel (configuración)
    pub level_data: HashMap<String, Valor>,
    /// Historial de niveles (para transiciones)
    pub level_history: Vec<String>,
    /// Estado de transición
    pub is_transitioning: bool,
    /// Duración de transición (ms)
    pub transition_duration: f32,
    /// Próximo nivel (para transición)
    pub next_level: Option<String>,
}

impl LevelManager {
    /// Crear nuevo Level Manager
    pub fn new() -> Self {
        Self {
            current_level: None,
            current_path: None,
            checkpoints: HashMap::new(),
            level_data: HashMap::new(),
            level_history: Vec::new(),
            is_transitioning: false,
            transition_duration: 1000.0, // 1 segundo por defecto
            next_level: None,
        }
    }

    /// Cargar nivel desde archivo
    pub fn load(&mut self, level_name: &str, level_path: &str) -> Result<String, String> {
        // Verificar que el archivo existe
        if !std::path::Path::new(level_path).exists() {
            return Err(format!("El archivo de nivel '{}' no existe", level_path));
        }

        // Leer contenido del nivel
        let content = match fs::read_to_string(level_path) {
            Ok(c) => c,
            Err(e) => return Err(format!("Error leyendo nivel '{}': {}", level_path, e)),
        };

        // Guardar en historial si hay nivel actual
        if let Some(ref current) = self.current_level {
            self.level_history.push(current.clone());
        }

        // Establecer como nivel actual
        self.current_level = Some(level_name.to_string());
        self.current_path = Some(level_path.to_string());
        
        // Limpiar checkpoints del nivel anterior
        self.checkpoints.clear();
        self.level_data.clear();

        // Parsear datos básicos del nivel (comentarios, metadata)
        self.parse_level_metadata(&content);

        Ok(format!("Nivel '{}' cargado exitosamente", level_name))
    }

    /// Parsear metadata del nivel (comentarios con datos)
    fn parse_level_metadata(&mut self, content: &str) {
        // Buscar líneas con metadata especial
        // Ejemplo: # @nombre "Nivel 1"
        // Ejemplo: # @musica "tema1.mp3"
        for line in content.lines() {
            if line.starts_with("# @") {
                let parts: Vec<&str> = line[3..].splitn(2, ' ').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim().to_string();
                    let value = parts[1].trim().trim_matches('"').to_string();
                    self.level_data.insert(key, Valor::Texto(value));
                }
            }
        }
    }

    /// Descargar nivel actual
    pub fn unload(&mut self) -> String {
        if self.current_level.is_some() {
            let name = self.current_level.take().unwrap();
            self.current_path = None;
            self.checkpoints.clear();
            self.level_data.clear();
            format!("Nivel '{}' descargado", name)
        } else {
            "No hay nivel cargado".to_string()
        }
    }

    /// Transición a otro nivel
    pub fn transition(&mut self, next_level: &str) -> String {
        self.next_level = Some(next_level.to_string());
        self.is_transitioning = true;
        format!("Iniciando transición a '{}'", next_level)
    }

    /// Obtener nivel actual
    pub fn get_current(&self) -> Option<String> {
        self.current_level.clone()
    }

    /// Recargar nivel actual
    #[allow(dead_code)] // Para futura funcionalidad de reload
    pub fn reload(&mut self) -> Result<String, String> {
        // Guardar path y nombre antes de unload
        let path = self.current_path.clone();
        let name = self.current_level.clone();
        
        if let Some(path_str) = path {
            let name_str = name.unwrap_or_default();
            self.unload();
            self.load(&name_str, &path_str)
        } else {
            Err("No hay nivel cargado para recargar".to_string())
        }
    }

    /// Obtener nombre del nivel
    pub fn get_name(&self) -> Option<String> {
        self.current_level.clone()
    }

    // ========================================================================
    // CHECKPOINTS
    // ========================================================================

    /// Establecer checkpoint
    pub fn set_checkpoint(&mut self, name: &str, x: f32, y: f32) {
        self.checkpoints.insert(name.to_string(), (x, y));
    }

    /// Cargar checkpoint (mover cámara/player al checkpoint)
    pub fn load_checkpoint(&self, name: &str) -> Option<(f32, f32)> {
        self.checkpoints.get(name).copied()
    }

    /// Obtener posición de checkpoint
    pub fn get_checkpoint(&self, name: &str) -> Option<(f32, f32)> {
        self.checkpoints.get(name).copied()
    }

    /// Listar todos los checkpoints
    pub fn list_checkpoints(&self) -> Vec<String> {
        self.checkpoints.keys().cloned().collect()
    }

    // ========================================================================
    // TRANSICIONES
    // ========================================================================

    /// Iniciar transición fade
    pub fn transition_fade(&mut self, duration_ms: f32) {
        self.transition_duration = duration_ms;
        self.is_transitioning = true;
    }

    /// Iniciar transición slide
    pub fn transition_slide(&mut self, direction: &str, duration_ms: f32) {
        self.transition_duration = duration_ms;
        self.is_transitioning = true;
        // Guardar dirección para el render
        self.level_data.insert(
            "transition_direction".to_string(),
            Valor::Texto(direction.to_string()),
        );
    }

    /// Completar transición
    #[allow(dead_code)] // Para futura integración con game loop
    pub fn complete_transition(&mut self) -> Option<String> {
        if self.is_transitioning {
            self.is_transitioning = false;
            self.next_level.take()
        } else {
            None
        }
    }

    /// Verificar si está en transición
    #[allow(dead_code)] // Para futura integración con game loop
    pub fn is_transitioning(&self) -> bool {
        self.is_transitioning
    }

    /// Obtener duración de transición
    #[allow(dead_code)] // Para futura integración con game loop
    pub fn get_transition_duration(&self) -> f32 {
        self.transition_duration
    }
}

impl Default for LevelManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ESTADO GLOBAL
// ============================================================================

use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static LEVEL_MANAGER: Rc<RefCell<LevelManager>> = Rc::new(RefCell::new(LevelManager::new()));
}

/// Obtener referencia al Level Manager
pub fn get_level_manager() -> Rc<RefCell<LevelManager>> {
    LEVEL_MANAGER.with(|lm| lm.clone())
}

// ============================================================================
// FUNCIONES PARA .rydit
// ============================================================================

/// level::load(ruta) - Cargar nivel
pub fn level_load(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("level::load() requiere 1 argumento: ruta".to_string());
    }

    let ruta_val = evaluar_expr(&args[0], executor, funcs);

    let ruta = match ruta_val {
        Valor::Texto(r) => r,
        _ => return Valor::Error("level::load() ruta debe ser texto".to_string()),
    };

    // Extraer nombre del archivo (sin extensión)
    let name = std::path::Path::new(&ruta)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(&ruta);

    let lm = get_level_manager();
    let mut lm_ref = lm.borrow_mut();

    match lm_ref.load(name, &ruta) {
        Ok(msg) => Valor::Texto(msg),
        Err(e) => Valor::Error(e),
    }
}

/// level::unload() - Descargar nivel actual
pub fn level_unload(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let lm = get_level_manager();
    let mut lm_ref = lm.borrow_mut();
    Valor::Texto(lm_ref.unload())
}

/// level::transition(ruta) - Transición a otro nivel
pub fn level_transition(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("level::transition() requiere 1 argumento: ruta".to_string());
    }

    let ruta_val = evaluar_expr(&args[0], executor, funcs);

    let ruta = match ruta_val {
        Valor::Texto(r) => r,
        _ => return Valor::Error("level::transition() ruta debe ser texto".to_string()),
    };

    let lm = get_level_manager();
    let mut lm_ref = lm.borrow_mut();
    Valor::Texto(lm_ref.transition(&ruta))
}

/// level::get_current() - Obtener nivel actual
pub fn level_get_current(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let lm = get_level_manager();
    let lm_ref = lm.borrow();

    match lm_ref.get_current() {
        Some(name) => Valor::Texto(name),
        None => Valor::Texto("ninguno".to_string()),
    }
}

/// level::reload() - Recargar nivel actual
pub fn level_reload(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let lm = get_level_manager();
    let mut lm_ref = lm.borrow_mut();

    // Guardar path antes de unload
    let path = lm_ref.current_path.clone();
    let name = lm_ref.current_level.clone();
    
    if path.is_some() {
        lm_ref.unload();
        let path_str = path.unwrap();
        let name_str = name.unwrap_or_default();
        match lm_ref.load(&name_str, &path_str) {
            Ok(msg) => Valor::Texto(msg),
            Err(e) => Valor::Error(e),
        }
    } else {
        Valor::Error("No hay nivel cargado para recargar".to_string())
    }
}

/// level::get_name() - Obtener nombre del nivel
pub fn level_get_name(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let lm = get_level_manager();
    let lm_ref = lm.borrow();

    match lm_ref.get_name() {
        Some(name) => Valor::Texto(name),
        None => Valor::Texto("ninguno".to_string()),
    }
}

/// level::set_checkpoint(nombre, x, y) - Establecer checkpoint
pub fn level_set_checkpoint(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 3 {
        return Valor::Error("level::set_checkpoint() requiere 3 argumentos: nombre, x, y".to_string());
    }

    let nombre_val = evaluar_expr(&args[0], executor, funcs);
    let x_val = evaluar_expr(&args[1], executor, funcs);
    let y_val = evaluar_expr(&args[2], executor, funcs);

    let nombre = match nombre_val {
        Valor::Texto(n) => n,
        _ => return Valor::Error("level::set_checkpoint() nombre debe ser texto".to_string()),
    };

    let x = match x_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("level::set_checkpoint() x debe ser número".to_string()),
    };

    let y = match y_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("level::set_checkpoint() y debe ser número".to_string()),
    };

    let lm = get_level_manager();
    let mut lm_ref = lm.borrow_mut();
    lm_ref.set_checkpoint(&nombre, x, y);

    Valor::Texto(format!("Checkpoint '{}' establecido en ({}, {})", nombre, x, y))
}

/// level::load_checkpoint(nombre) - Cargar checkpoint
pub fn level_load_checkpoint(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("level::load_checkpoint() requiere 1 argumento: nombre".to_string());
    }

    let nombre_val = evaluar_expr(&args[0], executor, funcs);

    let nombre = match nombre_val {
        Valor::Texto(n) => n,
        _ => return Valor::Error("level::load_checkpoint() nombre debe ser texto".to_string()),
    };

    let lm = get_level_manager();
    let lm_ref = lm.borrow();

    match lm_ref.load_checkpoint(&nombre) {
        Some((x, y)) => Valor::Array(vec![Valor::Num(x as f64), Valor::Num(y as f64)]),
        None => Valor::Error(format!("Checkpoint '{}' no encontrado", nombre)),
    }
}

/// level::get_checkpoint(nombre) - Obtener posición de checkpoint
pub fn level_get_checkpoint(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("level::get_checkpoint() requiere 1 argumento: nombre".to_string());
    }

    let nombre_val = evaluar_expr(&args[0], executor, funcs);

    let nombre = match nombre_val {
        Valor::Texto(n) => n,
        _ => return Valor::Error("level::get_checkpoint() nombre debe ser texto".to_string()),
    };

    let lm = get_level_manager();
    let lm_ref = lm.borrow();

    match lm_ref.get_checkpoint(&nombre) {
        Some((x, y)) => Valor::Array(vec![Valor::Num(x as f64), Valor::Num(y as f64)]),
        None => Valor::Error(format!("Checkpoint '{}' no encontrado", nombre)),
    }
}

/// level::list_checkpoints() - Listar checkpoints
pub fn level_list_checkpoints(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let lm = get_level_manager();
    let lm_ref = lm.borrow();

    let checkpoints = lm_ref.list_checkpoints();
    let valores: Vec<Valor> = checkpoints.into_iter().map(Valor::Texto).collect();

    Valor::Array(valores)
}

/// level::transition_fade(duracion) - Transición fade
pub fn level_transition_fade(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("level::transition_fade() requiere 1 argumento: duracion".to_string());
    }

    let duracion_val = evaluar_expr(&args[0], executor, funcs);

    let duracion = match duracion_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("level::transition_fade() duracion debe ser número".to_string()),
    };

    let lm = get_level_manager();
    let mut lm_ref = lm.borrow_mut();
    lm_ref.transition_fade(duracion);

    Valor::Texto(format!("Transición fade iniciada ({}ms)", duracion))
}

/// level::transition_slide(direccion, duracion) - Transición slide
pub fn level_transition_slide(
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("level::transition_slide() requiere 2 argumentos: direccion, duracion".to_string());
    }

    let direccion_val = evaluar_expr(&args[0], executor, funcs);
    let duracion_val = evaluar_expr(&args[1], executor, funcs);

    let direccion = match direccion_val {
        Valor::Texto(d) => d,
        _ => return Valor::Error("level::transition_slide() direccion debe ser texto".to_string()),
    };

    let duracion = match duracion_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("level::transition_slide() duracion debe ser número".to_string()),
    };

    let lm = get_level_manager();
    let mut lm_ref = lm.borrow_mut();
    lm_ref.transition_slide(&direccion, duracion);

    Valor::Texto(format!("Transición slide '{}' iniciada ({}ms)", direccion, duracion))
}
