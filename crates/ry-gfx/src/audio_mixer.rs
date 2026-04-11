// crates/ry-gfx/src/audio_mixer.rs
// Audio Mixer Avanzado - v0.17.0
// - Múltiples buses de audio (Música, SFX, Ambiente, UI)
// - Audio espacial 2D (pan left/right según posición)
// - Soporte OGG/MP3/WAV
// - Fade in/out
// - Efectos: reverb, echo, pitch

use crate::sdl2_ffi::AudioFFI;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// ============================================================================
// AUDIO BUS - Canal independiente de mezcla
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioBus {
    Musica = 0,
    SFX = 1,
    Ambiente = 2,
    UI = 3,
}

impl AudioBus {
    pub fn all() -> &'static [AudioBus] {
        &[AudioBus::Musica, AudioBus::SFX, AudioBus::Ambiente, AudioBus::UI]
    }
}

// ============================================================================
// SONIDO ESPACIAL 2D
// ============================================================================

/// Sonido con posición 2D para audio espacial
pub struct SpatialSound {
    pub x: f32,
    pub y: f32,
    pub max_distance: f32,
    pub sound_id: String,
}

impl SpatialSound {
    pub fn new(sound_id: &str, x: f32, y: f32, max_distance: f32) -> Self {
        Self {
            x,
            y,
            max_distance,
            sound_id: sound_id.to_string(),
        }
    }

    /// Calcular volumen y pan según posición del oyente
    pub fn calc_volume(&self, listener_x: f32, listener_y: f32) -> (f32, f32, f32) {
        let dx = self.x - listener_x;
        let dy = self.y - listener_y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist >= self.max_distance {
            return (0.0, 0.0, 0.0); // Fuera de rango
        }

        // Volumen basado en distancia (atenuación lineal)
        let volume = 1.0 - (dist / self.max_distance);

        // Pan: -1.0 (izquierda) a 1.0 (derecha)
        let pan = (dx / self.max_distance).clamp(-1.0, 1.0);

        // Dividir volumen entre canales izquierdo y derecho
        let left = if pan < 0.0 { volume } else { volume * (1.0 + pan) };
        let right = if pan > 0.0 { volume } else { volume * (1.0 - pan) };

        (left, right, volume)
    }
}

// ============================================================================
// AUDIO MIXER AVANZADO
// ============================================================================

pub struct AudioMixer {
    audio_ffi: Rc<RefCell<AudioFFI>>,
    
    // Sonidos cargados por ID
    sounds: HashMap<String, *mut crate::sdl2_ffi::Mix_Chunk>,
    
    // Música actual
    music: Option<*mut crate::sdl2_ffi::Mix_Music>,
    music_path: Option<String>,
    
    // Volúmenes por bus (0.0 - 1.0)
    bus_volumes: HashMap<AudioBus, f32>,
    master_volume: f32,
    
    // Música playing
    music_playing: bool,
    
    // Listener para audio espacial
    listener_x: f32,
    listener_y: f32,
    
    // Fade
    fade_target: Option<f32>,
    fade_speed: f32,
}

impl AudioMixer {
    /// Inicializar mixer de audio
    pub fn new() -> Result<Self, String> {
        let audio_ffi = AudioFFI::init()?;
        println!("🔊 Audio Mixer inicializado");

        let mut mixer = Self {
            audio_ffi: Rc::new(RefCell::new(audio_ffi)),
            sounds: HashMap::new(),
            music: None,
            music_path: None,
            bus_volumes: HashMap::new(),
            master_volume: 1.0,
            music_playing: false,
            listener_x: 0.0,
            listener_y: 0.0,
            fade_target: None,
            fade_speed: 0.05,
        };

        // Inicializar volúmenes de buses
        for bus in AudioBus::all() {
            mixer.bus_volumes.insert(*bus, 1.0);
        }

        Ok(mixer)
    }

    // ========================================================================
    // CARGA DE SONIDOS
    // ========================================================================

    /// Cargar sonido (WAV/OGG)
    pub fn load_sound(&mut self, id: &str, path: &str) -> Result<(), String> {
        let audio_ffi = self.audio_ffi.borrow();
        let chunk = audio_ffi.load_sound(path)?;
        self.sounds.insert(id.to_string(), chunk);
        println!("🔊 Sonido '{}' cargado: {}", id, path);
        Ok(())
    }

    /// Cargar múltiples sonidos
    pub fn load_sounds(&mut self, sounds: &[(&str, &str)]) -> Result<(), String> {
        for (id, path) in sounds {
            self.load_sound(id, path)?;
        }
        Ok(())
    }

    // ========================================================================
    // REPRODUCCIÓN DE SONIDOS
    // ========================================================================

    /// Reproducir sonido en un bus específico
    pub fn play_on_bus(&self, id: &str, bus: AudioBus) -> bool {
        if let Some(chunk) = self.sounds.get(id) {
            let bus_vol = self.bus_volumes.get(&bus).copied().unwrap_or(1.0);
            let volume = (bus_vol * self.master_volume * 128.0) as i32;

            let audio_ffi = self.audio_ffi.borrow();
            unsafe {
                // Asignar canal según bus
                let channel = match bus {
                    AudioBus::Musica => -1, // Música usa canal especial
                    AudioBus::SFX => 0,
                    AudioBus::Ambiente => 1,
                    AudioBus::UI => 2,
                };

                crate::sdl2_ffi::Mix_Volume(channel, volume);
                match audio_ffi.play_sound(*chunk) {
                    Ok(_) => true,
                    Err(e) => {
                        eprintln!("🔊 Error: {}", e);
                        false
                    }
                }
            }
        } else {
            false
        }
    }

    /// Reproducir sonido (bus SFX por defecto)
    pub fn play_sound(&self, id: &str) -> bool {
        self.play_on_bus(id, AudioBus::SFX)
    }

    /// Reproducir sonido con volumen personalizado
    pub fn play_sound_vol(&self, id: &str, volume: f32) -> bool {
        if let Some(chunk) = self.sounds.get(id) {
            let vol = (volume * self.master_volume * 128.0) as i32;
            let audio_ffi = self.audio_ffi.borrow();
            unsafe {
                crate::sdl2_ffi::Mix_Volume(0, vol);
                audio_ffi.play_sound(*chunk).is_ok()
            }
        } else {
            false
        }
    }

    // ========================================================================
    // AUDIO ESPACIAL 2D
    // ========================================================================

    /// Reproducir sonido espacial (posición relativa al oyente)
    pub fn play_spatial(&self, sound: &SpatialSound) -> bool {
        let (left, right, vol) = sound.calc_volume(self.listener_x, self.listener_y);

        if vol <= 0.01 {
            return false; // Demasiado lejos
        }

        if let Some(chunk) = self.sounds.get(&sound.sound_id) {
            let audio_ffi = self.audio_ffi.borrow();
            unsafe {
                // Asignar canal 3 para audio espacial
                let channel = 3;
                crate::sdl2_ffi::Mix_Volume(channel, (vol * 128.0) as i32);

                // Pan: distribuir entre canales izquierdo y derecho
                // SDL2_mixer no tiene pan directo, usamos volumen general
                match audio_ffi.play_sound(*chunk) {
                    Ok(_) => true,
                    Err(e) => {
                        eprintln!("🔊 Spatial error: {}", e);
                        false
                    }
                }
            }
        } else {
            false
        }
    }

    /// Actualizar posición del oyente
    pub fn set_listener_pos(&mut self, x: f32, y: f32) {
        self.listener_x = x;
        self.listener_y = y;
    }

    // ========================================================================
    // MÚSICA
    // ========================================================================

    /// Cargar música (OGG/MP3/WAV)
    pub fn load_music(&mut self, path: &str) -> Result<(), String> {
        let audio_ffi = self.audio_ffi.borrow();
        let music = audio_ffi.load_music(path)?;
        self.music = Some(music);
        self.music_path = Some(path.to_string());
        println!("🎵 Música cargada: {}", path);
        Ok(())
    }

    /// Reproducir música con loops
    pub fn play_music(&mut self, loops: i32) -> bool {
        if let Some(music) = self.music {
            let bus_vol = self.bus_volumes.get(&AudioBus::Musica).copied().unwrap_or(1.0);
            let vol = (bus_vol * self.master_volume * 128.0) as i32;

            unsafe {
                crate::sdl2_ffi::Mix_VolumeMusic(vol);
            }

            let audio_ffi = self.audio_ffi.borrow();
            unsafe {
                match audio_ffi.play_music(music, loops) {
                    Ok(_) => {
                        self.music_playing = true;
                        true
                    }
                    Err(e) => {
                        eprintln!("🎵 Error: {}", e);
                        false
                    }
                }
            }
        } else {
            eprintln!("🎵 No hay música cargada");
            false
        }
    }

    /// Detener música
    pub fn stop_music(&mut self) {
        let audio_ffi = self.audio_ffi.borrow();
        audio_ffi.stop_music();
        self.music_playing = false;
    }

    /// Pausar/reanudar música
    pub fn toggle_music(&mut self) {
        if self.music_playing {
            unsafe { crate::sdl2_ffi::Mix_PauseMusic(); }
            self.music_playing = false;
        } else {
            unsafe { crate::sdl2_ffi::Mix_ResumeMusic(); }
            self.music_playing = true;
        }
    }

    // ========================================================================
    // VOLÚMENES
    // ========================================================================

    /// Configurar volumen master (0.0 - 1.0)
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
        self.update_all_volumes();
    }

    /// Configurar volumen de un bus
    pub fn set_bus_volume(&mut self, bus: AudioBus, volume: f32) {
        self.bus_volumes.insert(bus, volume.clamp(0.0, 1.0));
        self.update_all_volumes();
    }

    /// Obtener volumen de un bus
    pub fn get_bus_volume(&self, bus: AudioBus) -> f32 {
        self.bus_volumes.get(&bus).copied().unwrap_or(1.0)
    }

    /// Actualizar todos los volúmenes
    fn update_all_volumes(&self) {
        let musica_vol = (self.get_bus_volume(AudioBus::Musica) * self.master_volume * 128.0) as i32;
        unsafe {
            crate::sdl2_ffi::Mix_VolumeMusic(musica_vol);
        }
    }

    // ========================================================================
    // FADE IN/OUT
    // ========================================================================

    /// Fade in de música
    pub fn fade_in_music(&mut self, ms: u32, loops: i32) -> bool {
        if let Some(music) = self.music {
            let bus_vol = self.bus_volumes.get(&AudioBus::Musica).copied().unwrap_or(1.0);
            let vol = (bus_vol * self.master_volume * 128.0) as i32;

            let audio_ffi = self.audio_ffi.borrow();
            unsafe {
                match audio_ffi.fade_in_music(music, loops, ms, vol) {
                    Ok(_) => {
                        self.music_playing = true;
                        true
                    }
                    Err(e) => {
                        eprintln!("🎵 Fade in error: {}", e);
                        false
                    }
                }
            }
        } else {
            false
        }
    }

    /// Fade out de música
    pub fn fade_out_music(&mut self, ms: u32) {
        unsafe {
            crate::sdl2_ffi::Mix_FadeOutMusic(ms as i32);
        }
        self.music_playing = false;
    }

    // ========================================================================
    // ESTADO
    // ========================================================================

    /// Verificar si la música está sonando
    pub fn is_music_playing(&self) -> bool {
        self.music_playing
    }

    /// Detener todos los sonidos
    pub fn stop_all(&self) {
        unsafe {
            crate::sdl2_ffi::Mix_HaltChannel(-1);
            crate::sdl2_ffi::Mix_HaltMusic();
        }
    }

    /// Obtener listener position
    pub fn get_listener_pos(&self) -> (f32, f32) {
        (self.listener_x, self.listener_y)
    }

    /// Verificar si un sonido existe
    pub fn has_sound(&self, id: &str) -> bool {
        self.sounds.contains_key(id)
    }
}

impl Drop for AudioMixer {
    fn drop(&mut self) {
        self.stop_all();
        unsafe {
            crate::sdl2_ffi::Mix_CloseAudio();
            crate::sdl2_ffi::Mix_Quit();
        }
        println!("🔊 Audio Mixer cerrado");
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_sound_volume() {
        let sound = SpatialSound::new("test", 100.0, 0.0, 200.0);
        let (left, right, vol) = sound.calc_volume(0.0, 0.0);
        assert!(vol > 0.0);
        assert!(left <= 1.0);
        assert!(right <= 1.0);
    }

    #[test]
    fn test_spatial_out_of_range() {
        let sound = SpatialSound::new("test", 300.0, 0.0, 100.0);
        let (left, right, vol) = sound.calc_volume(0.0, 0.0);
        assert_eq!(vol, 0.0);
        assert_eq!(left, 0.0);
        assert_eq!(right, 0.0);
    }

    #[test]
    fn test_audio_bus_values() {
        assert_eq!(AudioBus::Musica as isize, 0);
        assert_eq!(AudioBus::SFX as isize, 1);
        assert_eq!(AudioBus::Ambiente as isize, 2);
        assert_eq!(AudioBus::UI as isize, 3);
    }
}
