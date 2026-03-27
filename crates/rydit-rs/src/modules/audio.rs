// crates/rydit-rs/src/modules/audio.rs
// Audio Module - Sonidos, música y efectos de audio
//
// Diseño para extensión comunitaria:
// - Reproductores de música
// - Mezcladores DJ
// - Visualizadores de audio
// - Efectos de sonido

use blast_core::{Executor, Valor};
use lizer::{Expr, Stmt};
use std::collections::HashMap;

use crate::eval::evaluar_expr;

// ============================================================================
// ESTADO GLOBAL DE AUDIO
// ============================================================================

/// Estado global de audio (compartido entre módulos)
pub struct AudioState {
    sounds: HashMap<String, String>,  // ID -> path
    music_path: Option<String>,
    is_playing: bool,
    volume: f32,
}

impl AudioState {
    pub fn new() -> Self {
        Self {
            sounds: HashMap::new(),
            music_path: None,
            is_playing: false,
            volume: 1.0,
        }
    }
}

impl Default for AudioState {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    static AUDIO_STATE: std::cell::RefCell<AudioState> = std::cell::RefCell::new(AudioState::new());
}

// ============================================================================
// FUNCIONES BÁSICAS DE AUDIO
// ============================================================================

/// audio::beep(frecuencia, duracion) - Generar beep tipo consola
pub fn audio_beep(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("audio::beep() requiere 2 argumentos: frecuencia, duracion".to_string());
    }

    let freq_val = evaluar_expr(&args[0], executor, _funcs);
    let dur_val = evaluar_expr(&args[1], executor, _funcs);

    if let (Valor::Num(frecuencia), Valor::Num(duracion)) = (freq_val, dur_val) {
        // NOTA: Implementación real requiere acceso a rydit-gfx AudioSystem
        // Por ahora, retornamos éxito simulado
        println!("[AUDIO] Beep: {} Hz, {} ms", frecuencia, duracion);
        return Valor::Texto(format!("audio::beep() - {} Hz, {} ms (pendiente implementación)", frecuencia, duracion));
    }

    Valor::Error("audio::beep() requiere números (frecuencia, duracion)".to_string())
}

/// audio::click() - Sonido de click UI
pub fn audio_click(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    // NOTA: Implementación real requiere acceso a rydit-gfx AudioSystem
    println!("[AUDIO] Click UI");
    Valor::Texto("audio::click() - Click UI (pendiente implementación)".to_string())
}

// ============================================================================
// GESTIÓN DE SONIDOS
// ============================================================================

/// audio::load(id, path) - Cargar sonido desde archivo
pub fn audio_load_sound(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("audio::load() requiere 2 argumentos: id, path".to_string());
    }

    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let path_val = evaluar_expr(&args[1], executor, _funcs);

    let id = match id_val {
        Valor::Texto(s) => s,
        Valor::Num(n) => n.to_string(),
        _ => return Valor::Error("audio::load() el ID debe ser texto".to_string()),
    };

    let path = match path_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("audio::load() el path debe ser texto".to_string()),
    };

    // Registrar en estado global
    AUDIO_STATE.with(|state| {
        let mut state_ref = state.borrow_mut();
        state_ref.sounds.insert(id.clone(), path.clone());
    });

    println!("[AUDIO] Sonido '{}' registrado: {}", id, path);
    Valor::Texto(format!("audio::load() - '{}' cargado (listo para reproducir)", id))
}

/// audio::play(id) - Reproducir sonido
pub fn audio_play(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("audio::play() requiere 1 argumento: id".to_string());
    }

    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("audio::play() el ID debe ser texto".to_string()),
    };

    // Verificar si existe
    let existe = AUDIO_STATE.with(|state| {
        let state_ref = state.borrow();
        state_ref.sounds.contains_key(&id)
    });

    if existe {
        println!("[AUDIO] Reproduciendo sonido: {}", id);
        Valor::Texto(format!("audio::play() - Reproduciendo '{}'", id))
    } else {
        Valor::Error(format!("audio::play() El sonido '{}' no está cargado", id))
    }
}

/// audio::stop(id) - Detener sonido
pub fn audio_stop(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("audio::stop() requiere 1 argumento: id".to_string());
    }

    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("audio::stop() el ID debe ser texto".to_string()),
    };

    println!("[AUDIO] Deteniendo sonido: {}", id);
    Valor::Texto(format!("audio::stop() - Detenido '{}'", id))
}

/// audio::volume(id, level) - Configurar volumen de sonido (0.0 - 1.0)
pub fn audio_volume(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("audio::volume() requiere 2 argumentos: id, volumen".to_string());
    }

    let id_val = evaluar_expr(&args[0], executor, _funcs);
    let vol_val = evaluar_expr(&args[1], executor, _funcs);

    let _id = match id_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("audio::volume() el ID debe ser texto".to_string()),
    };

    let volume = match vol_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("audio::volume() el volumen debe ser número".to_string()),
    };

    if volume < 0.0 || volume > 1.0 {
        return Valor::Error("audio::volume() el volumen debe estar entre 0.0 y 1.0".to_string());
    }

    // Actualizar volumen global
    AUDIO_STATE.with(|state| {
        let mut state_ref = state.borrow_mut();
        state_ref.volume = volume;
    });

    println!("[AUDIO] Volumen: {:.1}%", volume * 100.0);
    Valor::Texto(format!("audio::volume() - Volumen al {:.0}%", volume * 100.0))
}

// ============================================================================
// GESTIÓN DE MÚSICA
// ============================================================================

/// audio::load_music(path) - Cargar música desde archivo
pub fn audio_load_music(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("audio::load_music() requiere 1 argumento: path".to_string());
    }

    let path_val = evaluar_expr(&args[0], executor, _funcs);
    let path = match path_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("audio::load_music() el path debe ser texto".to_string()),
    };

    // Registrar en estado global
    AUDIO_STATE.with(|state| {
        let mut state_ref = state.borrow_mut();
        state_ref.music_path = Some(path.clone());
    });

    println!("[AUDIO] Música registrada: {}", path);
    Valor::Texto(format!("audio::load_music() - '{}' cargada", path))
}

/// audio::play_music() - Reproducir música
pub fn audio_play_music(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let music_path = AUDIO_STATE.with(|state| {
        let state_ref = state.borrow();
        state_ref.music_path.clone()
    });

    match music_path {
        Some(path) => {
            AUDIO_STATE.with(|state| {
                let mut state_ref = state.borrow_mut();
                state_ref.is_playing = true;
            });
            println!("[AUDIO] Reproduciendo música: {}", path);
            Valor::Texto(format!("audio::play_music() - Reproduciendo '{}'", path))
        }
        None => Valor::Error("audio::play_music() No hay música cargada".to_string())
    }
}

/// audio::stop_music() - Detener música
pub fn audio_stop_music(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    AUDIO_STATE.with(|state| {
        let mut state_ref = state.borrow_mut();
        state_ref.is_playing = false;
    });

    println!("[AUDIO] Música detenida");
    Valor::Texto("audio::stop_music() - Música detenida".to_string())
}

/// audio::is_playing() - Verificar si hay música reproduciendo
pub fn audio_is_playing(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let is_playing = AUDIO_STATE.with(|state| {
        let state_ref = state.borrow();
        state_ref.is_playing
    });

    Valor::Bool(is_playing)
}

/// audio::music_volume(level) - Configurar volumen de música (0.0 - 1.0)
pub fn audio_music_volume(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("audio::music_volume() requiere 1 argumento: volumen".to_string());
    }

    let vol_val = evaluar_expr(&args[0], executor, _funcs);
    let volume = match vol_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("audio::music_volume() el volumen debe ser número".to_string()),
    };

    if volume < 0.0 || volume > 1.0 {
        return Valor::Error("audio::music_volume() el volumen debe estar entre 0.0 y 1.0".to_string());
    }

    AUDIO_STATE.with(|state| {
        let mut state_ref = state.borrow_mut();
        state_ref.volume = volume;
    });

    println!("[AUDIO] Volumen de música: {:.1}%", volume * 100.0);
    Valor::Texto(format!("audio::music_volume() - Volumen al {:.0}%", volume * 100.0))
}

// ============================================================================
// UTILIDADES
// ============================================================================

/// audio::count() - Cantidad de sonidos cargados
pub fn audio_count(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let count = AUDIO_STATE.with(|state| {
        let state_ref = state.borrow();
        state_ref.sounds.len()
    });

    Valor::Num(count as f64)
}

/// audio::list() - Listar sonidos cargados
pub fn audio_list(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let sounds = AUDIO_STATE.with(|state| {
        let state_ref = state.borrow();
        state_ref.sounds.keys().cloned().collect::<Vec<String>>()
    });

    let array = sounds.iter()
        .map(|s| Valor::Texto(s.clone()))
        .collect::<Vec<Valor>>();

    Valor::Array(array)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_module_functions() {
        // Verificar que las funciones existen
        let _ = audio_beep;
        let _ = audio_click;
        let _ = audio_load_sound;
        let _ = audio_play;
        let _ = audio_stop;
        let _ = audio_volume;
        let _ = audio_load_music;
        let _ = audio_play_music;
        let _ = audio_stop_music;
        let _ = audio_is_playing;
        let _ = audio_music_volume;
        let _ = audio_count;
        let _ = audio_list;
    }
}
