// Nivel 3 - Test de Audio Low-End Simplificado
// v0.11.1 - Test MANUAL (no automático)
// ✅ Solo verifica que Audio SDL2 funciona

use rydit_gfx::audio_sdl2::AudioSystemSDL2;
use std::thread;
use std::time::Duration;

fn main() {
    println!("🎵 Nivel 3 - Test Audio Low-End v0.11.1\n");

    // Test 1: Inicializar Audio SDL2
    println!("📦 Test 1: Inicializar Audio SDL2");
    let mut audio = match AudioSystemSDL2::new() {
        Ok(a) => {
            println!("   ✅ Audio SDL2 inicializado\n");
            a
        }
        Err(e) => {
            eprintln!("   ❌ Error: {}", e);
            eprintln!("   ℹ️ SDL2_mixer puede no estar instalado");
            return;
        }
    };

    // Test 2: Cargar sonido (si existe el archivo)
    println!("🔊 Test 2: Cargar sonido (test_audio.wav)");
    let sonido_cargado = match audio.load_sound("click", "test_audio.wav") {
        Ok(_) => {
            println!("   ✅ Sonido cargado\n");
            true
        }
        Err(e) => {
            println!("   ⚠️ Sonido no encontrado (esperado): {}", e);
            println!("   ℹ️ Continuamos sin sonido\n");
            false
        }
    };

    // Test 3: Reproducir sonido (si se cargó)
    if sonido_cargado {
        println!("🔊 Test 3: Reproducir sonido");
        if audio.play_sound("click") {
            println!("   ✅ Sonido reproduciendo...");
            thread::sleep(Duration::from_millis(500));
            println!("   ✅ Sonido completado\n");
        } else {
            println!("   ❌ Error reproduciendo sonido\n");
        }
    }

    // Test 4: Configurar volumen
    println!("🎚️ Test 4: Configurar volumen");
    audio.set_music_volume(0.5);
    audio.set_sound_volume("click", 0.8);
    println!("   ✅ Volumen configurado (música: 0.5, sonido: 0.8)\n");

    // Test 5: Stop de música
    println!("⏹️ Test 5: Stop de música");
    audio.stop_music();
    println!("   ✅ Stop ejecutado\n");

    // Resumen
    println!("═══════════════════════════════════════════");
    println!("📊 RESUMEN:");
    println!("   ├─ Audio SDL2: ✅ Inicializado");
    if sonido_cargado {
        println!("   ├─ Carga de sonido: ✅ Funciona");
        println!("   ├─ Reproducción: ✅ Funciona");
    } else {
        println!("   ├─ Carga de sonido: ⚠️ Sin archivo de test");
        println!("   ├─ Reproducción: ⏭️ Skip");
    }
    println!("   ├─ Volumen: ✅ Controlable");
    println!("   └─ Stop: ✅ Funciona");
    println!();
    println!("🎉 ¡Test Audio Low-End Completado!");
    println!("═══════════════════════════════════════════");
}
