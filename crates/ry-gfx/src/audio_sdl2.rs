// crates/rydit-gfx/src/audio_sdl2.rs
// Audio SDL2 para RyDit - v0.11.1
// ✅ Implementación con SDL2_mixer FFI

use crate::sdl2_ffi::AudioFFI;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Sistema de audio con SDL2_mixer para Android/Termux-X11
pub struct AudioSystemSDL2 {
    audio_ffi: Rc<RefCell<AudioFFI>>,
    sounds: HashMap<String, *mut crate::sdl2_ffi::Mix_Chunk>,
    music: Option<*mut crate::sdl2_ffi::Mix_Music>,
    music_playing: RefCell<bool>,
}

impl AudioSystemSDL2 {
    /// Inicializar sistema de audio SDL2
    pub fn new() -> Result<Self, String> {
        let audio_ffi = AudioFFI::init()?;
        println!("[AUDIO SDL2] Dispositivo de audio inicializado");

        Ok(Self {
            audio_ffi: Rc::new(RefCell::new(audio_ffi)),
            sounds: HashMap::new(),
            music: None,
            music_playing: RefCell::new(false),
        })
    }

    /// Cargar sonido desde archivo
    pub fn load_sound(&mut self, id: &str, path: &str) -> Result<(), String> {
        let audio_ffi = self.audio_ffi.borrow();
        let chunk = audio_ffi.load_sound(path)?;
        self.sounds.insert(id.to_string(), chunk);
        println!("[AUDIO SDL2] Sonido '{}' cargado: {}", id, path);
        Ok(())
    }

    /// Reproducir sonido
    pub fn play_sound(&self, id: &str) -> bool {
        if let Some(chunk) = self.sounds.get(id) {
            let audio_ffi = self.audio_ffi.borrow();
            unsafe {
                match audio_ffi.play_sound(*chunk) {
                    Ok(_) => true,
                    Err(e) => {
                        eprintln!("[AUDIO SDL2] Error: {}", e);
                        false
                    }
                }
            }
        } else {
            eprintln!("[AUDIO SDL2] Sonido '{}' no encontrado", id);
            false
        }
    }

    /// Detener todos los sonidos
    pub fn stop_all_sounds(&self) {
        unsafe {
            crate::sdl2_ffi::Mix_HaltChannel(-1);
        }
    }

    /// Cargar música desde archivo
    pub fn load_music(&mut self, path: &str) -> Result<(), String> {
        let audio_ffi = self.audio_ffi.borrow();
        let music = audio_ffi.load_music(path)?;
        self.music = Some(music);
        println!("[AUDIO SDL2] Música cargada: {}", path);
        Ok(())
    }

    /// Reproducir música
    pub fn play_music(&self, loops: i32) -> bool {
        if let Some(music) = self.music {
            let audio_ffi = self.audio_ffi.borrow();
            unsafe {
                match audio_ffi.play_music(music, loops) {
                    Ok(_) => {
                        *self.music_playing.borrow_mut() = true;
                        true
                    }
                    Err(e) => {
                        eprintln!("[AUDIO SDL2] Error: {}", e);
                        false
                    }
                }
            }
        } else {
            eprintln!("[AUDIO SDL2] No hay música cargada");
            false
        }
    }

    /// Detener música
    pub fn stop_music(&self) {
        let audio_ffi = self.audio_ffi.borrow();
        audio_ffi.stop_music();
        *self.music_playing.borrow_mut() = false;
    }

    /// Verificar si la música está sonando
    pub fn is_music_playing(&self) -> bool {
        *self.music_playing.borrow()
    }

    /// Configurar volumen de música (0.0 - 1.0)
    pub fn set_music_volume(&self, volume: f32) {
        let vol = (volume * 128.0) as i32;
        unsafe {
            crate::sdl2_ffi::Mix_VolumeMusic(vol);
        }
    }

    /// Configurar volumen de sonido (0.0 - 1.0)
    pub fn set_sound_volume(&self, _id: &str, volume: f32) -> bool {
        let vol = (volume * 128.0) as i32;
        unsafe {
            crate::sdl2_ffi::Mix_Volume(-1, vol);
        }
        true
    }
}

impl Drop for AudioSystemSDL2 {
    fn drop(&mut self) {
        unsafe {
            crate::sdl2_ffi::Mix_CloseAudio();
            crate::sdl2_ffi::Mix_Quit();
        }
        println!("[AUDIO SDL2] Sistema de audio cerrado");
    }
}
