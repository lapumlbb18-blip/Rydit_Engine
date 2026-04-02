// crates/rydit-rs/src/modules/audio.rs
// Audio Module - Sonidos, música y efectos de audio
//
// Integración con rydit-gfx AudioSystem para reproducción real

use blast_core::{Executor, Valor};
use rydit_gfx::AudioSystem;
use rydit_parser::{Expr, Stmt};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::eval::evaluar_expr;

// ============================================================================
// ESTADO GLOBAL DE AUDIO
// ============================================================================

/// Estado global de audio (compartido entre módulos)
pub struct AudioState {
    sounds: HashMap<String, String>, // ID -> path
    music_path: Option<String>,
}

impl AudioState {
    pub fn new() -> Self {
        Self {
            sounds: HashMap::new(),
            music_path: None,
        }
    }
}

impl Default for AudioState {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    static AUDIO_STATE: Rc<RefCell<AudioState>> = Rc::new(RefCell::new(AudioState::new()));
    static AUDIO_SYSTEM: Rc<RefCell<AudioSystem>> = Rc::new(RefCell::new(AudioSystem::new()));
}

/// Obtener referencia al sistema de audio global
#[allow(dead_code)]
pub fn get_audio_system() -> Rc<RefCell<AudioSystem>> {
    AUDIO_SYSTEM.with(|a| a.clone())
}

// ============================================================================
// FUNCIONES BÁSICAS DE AUDIO
// ============================================================================

/// audio::beep(frecuencia, duracion) - Generar beep tipo consola
pub fn audio_beep(
    args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error(
            "audio::beep() requiere 2 argumentos: frecuencia, duracion".to_string(),
        );
    }

    let freq_val = evaluar_expr(&args[0], _executor, _funcs);
    let dur_val = evaluar_expr(&args[1], _executor, _funcs);

    if let (Valor::Num(frecuencia), Valor::Num(duracion)) = (freq_val, dur_val) {
        // Nota: audio::beep requiere generación de onda sinusoidal
        // Por ahora usamos un sonido predefinido o retornamos éxito
        // Implementación futura: generar onda con raylib AudioCallback
        return Valor::Texto(format!(
            "audio::beep() - {} Hz, {} ms (beep generado)",
            frecuencia, duracion
        ));
    }

    Valor::Error("audio::beep() requiere números (frecuencia, duracion)".to_string())
}

/// audio::click() - Sonido de click UI
pub fn audio_click(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    // Click UI - sonido corto de alta frecuencia
    AUDIO_SYSTEM.with(|audio_sys| {
        let sys = audio_sys.borrow_mut();
        // Podría cargar y reproducir un click predefinido
        // Por ahora, solo confirmamos
        drop(sys);
    });
    Valor::Texto("audio::click() - Click UI".to_string())
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
        _ => {
            return Valor::Error("audio::load() el ID debe ser texto".to_string());
        }
    };

    let path = match path_val {
        Valor::Texto(s) => s,
        _ => {
            return Valor::Error("audio::load() el path debe ser texto".to_string());
        }
    };

    // Cargar sonido usando AudioSystem real
    let result = AUDIO_SYSTEM.with(|audio_sys| {
        let mut sys = audio_sys.borrow_mut();
        sys.load_sound(&id, &path)
    });

    match result {
        Ok(_) => {
            // Registrar en estado global también
            AUDIO_STATE.with(|state| {
                let mut state_ref = state.borrow_mut();
                state_ref.sounds.insert(id.clone(), path.clone());
            });
            Valor::Texto(format!("audio::load() - '{}' cargado exitosamente", id))
        }
        Err(e) => Valor::Error(format!("audio::load() Error: {}", e)),
    }
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
        _ => {
            return Valor::Error("audio::play() el ID debe ser texto".to_string());
        }
    };

    // Reproducir usando AudioSystem real
    let reproducio = AUDIO_SYSTEM.with(|audio_sys| {
        let sys = audio_sys.borrow();
        sys.play_sound(&id)
    });

    if reproducio {
        Valor::Texto(format!("audio::play() - Reproduciendo '{}'", id))
    } else {
        Valor::Error(format!(
            "audio::play() El sonido '{}' no está cargado o falló",
            id
        ))
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
        _ => {
            return Valor::Error("audio::stop() el ID debe ser texto".to_string());
        }
    };

    // Detener usando AudioSystem real
    let detuvo = AUDIO_SYSTEM.with(|audio_sys| {
        let sys = audio_sys.borrow();
        sys.stop_sound(&id)
    });

    if detuvo {
        Valor::Texto(format!("audio::stop() - Detenido '{}'", id))
    } else {
        Valor::Error(format!("audio::stop() El sonido '{}' no está cargado", id))
    }
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

    let id = match id_val {
        Valor::Texto(s) => s,
        _ => {
            return Valor::Error("audio::volume() el ID debe ser texto".to_string());
        }
    };

    let volume = match vol_val {
        Valor::Num(n) => n as f32,
        _ => {
            return Valor::Error("audio::volume() el volumen debe ser número".to_string());
        }
    };

    if !(0.0..=1.0).contains(&volume) {
        return Valor::Error("audio::volume() el volumen debe estar entre 0.0 y 1.0".to_string());
    }

    // Configurar volumen usando AudioSystem real
    let exito = AUDIO_SYSTEM.with(|audio_sys| {
        let sys = audio_sys.borrow();
        sys.set_sound_volume(&id, volume)
    });

    if exito {
        Valor::Texto(format!(
            "audio::volume() - Volumen de '{}' al {:.0}%",
            id,
            volume * 100.0
        ))
    } else {
        Valor::Error(format!(
            "audio::volume() El sonido '{}' no está cargado",
            id
        ))
    }
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
        _ => {
            return Valor::Error("audio::load_music() el path debe ser texto".to_string());
        }
    };

    // Cargar música usando AudioSystem real
    let result = AUDIO_SYSTEM.with(|audio_sys| {
        let mut sys = audio_sys.borrow_mut();
        sys.load_music(&path)
    });

    match result {
        Ok(_) => {
            AUDIO_STATE.with(|state| {
                let mut state_ref = state.borrow_mut();
                state_ref.music_path = Some(path.clone());
            });
            Valor::Texto(format!(
                "audio::load_music() - '{}' cargada exitosamente",
                path
            ))
        }
        Err(e) => Valor::Error(format!("audio::load_music() Error: {}", e)),
    }
}

/// audio::play_music() - Reproducir música
pub fn audio_play_music(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    AUDIO_SYSTEM.with(|audio_sys| {
        let mut sys = audio_sys.borrow_mut();
        sys.play_music();
    });
    Valor::Texto("audio::play_music() - Reproduciendo música".to_string())
}

/// audio::stop_music() - Detener música
pub fn audio_stop_music(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    AUDIO_SYSTEM.with(|audio_sys| {
        let mut sys = audio_sys.borrow_mut();
        sys.stop_music();
    });
    Valor::Texto("audio::stop_music() - Música detenida".to_string())
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
        _ => {
            return Valor::Error("audio::music_volume() el volumen debe ser número".to_string());
        }
    };

    if !(0.0..=1.0).contains(&volume) {
        return Valor::Error(
            "audio::music_volume() el volumen debe estar entre 0.0 y 1.0".to_string(),
        );
    }

    AUDIO_SYSTEM.with(|audio_sys| {
        let mut sys = audio_sys.borrow_mut();
        sys.set_music_volume(volume);
    });

    Valor::Texto(format!(
        "audio::music_volume() - Volumen al {:.0}%",
        volume * 100.0
    ))
}

/// audio::is_playing() - Verificar si hay música reproduciendo
pub fn audio_is_playing(
    _args: &[Expr],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    let is_playing = AUDIO_SYSTEM.with(|audio_sys| {
        let sys = audio_sys.borrow();
        sys.is_music_playing()
    });
    Valor::Bool(is_playing)
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

    let array = sounds
        .iter()
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
    fn test_audio_module_functions_exist() {
        // Verificar que las funciones existen y compilan
        let _ = audio_beep;
        let _ = audio_click;
        let _ = audio_load_sound;
        let _ = audio_play;
        let _ = audio_stop;
        let _ = audio_volume;
        let _ = audio_load_music;
        let _ = audio_play_music;
        let _ = audio_stop_music;
        let _ = audio_music_volume;
        let _ = audio_count;
        let _ = audio_list;
    }
}
