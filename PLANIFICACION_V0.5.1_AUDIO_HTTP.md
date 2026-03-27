# 📋 PLANIFICACIÓN v0.5.1 - AUDIO + HTTP

**Fecha**: 2026-03-26 (Próxima sesión)
**Versión actual**: v0.5.0 ✅ ESTABLE
**Versión objetivo**: v0.5.1

---

## 🎯 OBJETIVOS

### 1. Audio - Sonidos Básicos ⭐⭐⭐
**Prioridad**: ALTA

#### Features a Implementar
- `audio::beep(frecuencia, duracion)` - Sonido tipo beep
- `audio::click()` - Sonido de click UI
- `audio::play_sound("path")` - Reproducir archivo WAV/MP3

#### Implementación
```rust
// crates/rydit-gfx/src/lib.rs o crates/rydit-audio/
pub struct AudioPlayer {
    // raylib audio context
}

impl AudioPlayer {
    pub fn beep(&mut self, frequency: f32, duration: f32);
    pub fn click(&mut self);
    pub fn load_sound(&mut self, path: &str) -> Result<Sound, String>;
    pub fn play_sound(&mut self, sound: &Sound);
}
```

#### Uso en RyDit
```rydit
# Beep simple
audio::beep(440, 0.5)  # 440Hz, 0.5 segundos

# Click UI
audio::click()

# Cargar y reproducir
dark.slot sonido = audio::load_sound("sounds/explosion.wav")
audio::play(sonido)
```

---

### 2. HTTP Request - GET Sencillo ⭐⭐
**Prioridad**: MEDIA

#### Features a Implementar
- `http::get(url)` - GET request sencillo
- `http::post(url, data)` - POST request (opcional)

#### Implementación
```rust
// crates/rydit-http/ o en rydit-rs
use ureq; // o reqwest

pub fn http_get(url: &str) -> Result<String, String> {
    match ureq::get(url).call() {
        Ok(response) => Ok(response.into_string()?),
        Err(e) => Err(format!("HTTP Error: {}", e)),
    }
}
```

#### Uso en RyDit
```rydit
# GET request
dark.slot respuesta = http::get("https://api.example.com/data")
voz respuesta

# Parsear JSON (si es JSON)
dark.slot datos = json::parse(respuesta)
voz datos["key"]
```

---

### 3. Documentación ⭐
**Prioridad**: BAJA

#### Tasks
- [ ] Actualizar README con ejemplos de audio
- [ ] Actualizar README con ejemplos de HTTP
- [ ] Crear demo de audio (beep + sonidos)
- [ ] Crear demo de HTTP (API call simple)

---

## 📦 CRATES INVOLUCRADOS

### Nuevos (a crear)
- `rydit-audio/` - Audio (beep, sonidos, música)
- `rydit-http/` - HTTP requests (GET, POST)

### Existentes (a modificar)
- `rydit-rs/src/main.rs` - Exponer funciones `audio::`, `http::`
- `rydit-gfx/` - Posible integración con audio

---

## 🔧 IMPLEMENTACIÓN PASO A PASO

### Sesión 1: Audio Básico
1. Crear `crates/rydit-audio/Cargo.toml`
2. Implementar `beep()` y `click()` con raylib
3. Exponer en `main.rs` como `audio::beep()`, `audio::click()`
4. Crear demo `demo_audio.rydit`
5. Tests

### Sesión 2: HTTP GET
1. Agregar `ureq` dependency a `rydit-rs/Cargo.toml`
2. Implementar `http_get()` function
3. Exponer en `main.rs` como `http::get()`
4. Crear demo `demo_http.rydit`
5. Tests

### Sesión 3: Integración + Docs
1. Demo combinado (audio + HTTP)
2. Actualizar README
3. Actualizar QWEN.md
4. Release v0.5.1

---

## 📊 METAS

| Feature | Líneas | Tests | Demo |
|---------|--------|-------|------|
| Audio beep/click | ~100 | 5+ | ✅ |
| HTTP GET | ~50 | 3+ | ✅ |
| Documentación | - | - | ✅ |

**Total estimado**: ~150 líneas nuevas, 8+ tests, 2 demos

---

## ⚠️ RIESGOS

### Audio
- raylib audio puede no estar disponible en Termux
- Solución: Usar `miniaudio` o `rodio` como fallback

### HTTP
- Requiere TLS/SSL para HTTPS
- Solución: `ureq` con `native-tls` o `rustls`

---

## ✅ CRITERIOS DE ACEPTACIÓN

- [ ] `audio::beep()` funciona en Termux-X11
- [ ] `audio::click()` suena al hacer click en UI
- [ ] `http::get()` retorna datos de API pública
- [ ] 8+ tests passing
- [ ] 2 demos funcionales
- [ ] README actualizado

---

<div align="center">

**🛡️ RyDit v0.5.1 - Audio + HTTP**

*~150 líneas | 8+ tests | 2 demos | 0 dependencias críticas*

</div>
