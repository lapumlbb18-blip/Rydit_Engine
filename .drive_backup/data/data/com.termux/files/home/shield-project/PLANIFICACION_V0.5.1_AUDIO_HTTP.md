# 📋 PLANIFICACIÓN v0.5.1 - AUDIO + HTTP + CSV + ASSETS + PARTICLES

**Fecha**: 2026-03-26 (Próxima sesión)
**Versión actual**: v0.5.0 ✅ ESTABLE
**Versión objetivo**: v0.5.1

---

## 🎯 OBJETIVOS

### 1. Audio - Sonidos Básicos ⭐⭐⭐
**Prioridad**: ALTA

#### Features
- `audio::beep(frecuencia, duracion)` - Sonido tipo beep
- `audio::click()` - Sonido de click UI
- `audio::play_sound("path")` - Reproducir archivo WAV/MP3

---

### 2. HTTP Request - GET Sencillo ⭐⭐
**Prioridad**: MEDIA

#### Features
- `http::get(url)` - GET request sencillo
- `http::post(url, data)` - POST request (opcional)

---

### 3. Data Science - CSV ⭐⭐
**Prioridad**: MEDIA

#### Features
- `csv::read("file.csv")` - Leer archivos CSV
- `csv::write(data, "file.csv")` - Escribir CSV
- `stats::std_dev([1,2,3,4,5])` - Desviación estándar
- `stats::variance([1,2,3,4,5])` - Varianza
- `plot::ascii(data)` - Gráficos ASCII en consola

---

### 4. Assets Manager ⭐⭐⭐
**Prioridad**: ALTA

#### Arquitectura Modular
**NO es crate nuevo** - Usa `RyditModule` existente:
- `crates/rydit-rs/src/modules/assets.rs` - Assets Module

#### Features
- `assets::sprite(id, path)` - Crear sprite 2D
- `assets::draw(id, x, y, scale)` - Dibujar sprite
- `assets::draw_scaled(id, x, y, scale)` - Dibujar escalado
- `assets::load(id, path)` - Cargar textura

#### Uso
```rydit
# Crear y dibujar sprite
dark.slot tank = assets::sprite("tank", "sprites/tank.png")
assets::draw(tank, 400, 300, 2.0)

# O directo:
assets::load("tank", "sprites/tank.png")
assets::draw_scaled("tank", 400, 300, 2.0)
```

---

### 5. Partículas en rydit-anim ⭐⭐⭐
**Prioridad**: ALTA

#### Arquitectura
**NO es crate nuevo** - En `rydit-anim` existente:
- `crates/rydit-anim/src/particles.rs` - Particle System

#### Features
- `particles::emit(x, y, effect)` - Emitir partículas
- `particles::update()` - Actualizar sistema
- `particles::draw()` - Dibujar partículas

#### Efectos
- 🔥 `particles::emit(x, y, "fire")` - Fuego
- 💨 `particles::emit(x, y, "smoke")` - Humo
- ✨ `particles::emit(x, y, "spark")` - Chispas
- 💥 `particles::emit(x, y, "explosion")` - Explosión

#### Uso
```rydit
# En game loop
particles::emit(400, 300, "fire")
particles::update()
particles::draw()
```

---

### 6. Documentación ⭐
**Prioridad**: BAJA

#### Tasks
- [ ] Actualizar README con ejemplos de audio
- [ ] Actualizar README con ejemplos de HTTP
- [ ] Actualizar README con ejemplos de CSV
- [ ] Actualizar README con ejemplos de Assets
- [ ] Actualizar README con ejemplos de Partículas
- [ ] Crear demo de audio (beep + sonidos)
- [ ] Crear demo de HTTP (API call simple)
- [ ] Crear demo de CSV (leer/escribir datos)
- [ ] Crear demo de Assets (tanque + helicóptero)
- [ ] Crear demo de Partículas (fuego + humo + explosión)

---

## 📦 CRATES INVOLUCRADOS

### Nuevos (a crear)
- `crates/rydit-audio/` - Audio (beep, sonidos, música)
- `crates/rydit-http/` - HTTP requests (GET, POST)

### Existentes (a modificar)
- `crates/rydit-rs/src/` - Agregar `modules/assets.rs`
- `crates/rydit-rs/src/main.rs` - Exponer funciones `audio::`, `http::`, `csv::`, `assets::`
- `crates/rydit-anim/src/` - Agregar `particles.rs`
- `crates/rydit-science/src/` - Agregar `csv.rs`, `stats_advanced.rs`

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

### Sesión 3: CSV + Stats Avanzados
1. Crear `crates/rydit-science/src/csv.rs`
2. Implementar `csv::read()`, `csv::write()`
3. Agregar `stats::std_dev()`, `stats::variance()` a `rydit-science`
4. Implementar `plot::ascii()`
5. Crear demo `demo_csv.rydit`
6. Tests

### Sesión 4: Assets Manager (Estilo Godot)
1. Crear `crates/rydit-rs/src/modules/assets.rs`
2. Implementar `assets::sprite()`, `assets::draw()`, `assets::load()`
3. Usar `Assets` struct que YA existe en `rydit-gfx`
4. Registrar módulo en `RyditModule`
5. Crear demo `demo_assets.rydit` (tanque + helicóptero)
6. Tests

### Sesión 5: Partículas en rydit-anim
1. Crear `crates/rydit-anim/src/particles.rs`
2. Implementar `ParticleSystem`, `Particle` structs
3. Implementar `emit()`, `update()`, `draw()`
4. Agregar a `rydit-anim` module
5. Crear demo `demo_particulas.rydit` (fuego + humo + explosión)
6. Tests

### Sesión 6: Integración + Docs
1. Demo combinado (audio + HTTP + CSV + Assets + Partículas)
2. Actualizar README
3. Actualizar QWEN.md
4. Release v0.5.1

---

## 📊 METAS

| Feature | Líneas | Tests | Demo |
|---------|--------|-------|------|
| Audio beep/click | ~100 | 5+ | ✅ |
| HTTP GET | ~50 | 3+ | ✅ |
| CSV read/write | ~150 | 5+ | ✅ |
| Stats avanzados | ~50 | 4+ | ✅ |
| Plot ASCII | ~80 | 2+ | ✅ |
| Assets Manager | ~200 | 8+ | ✅ |
| Partículas | ~250 | 10+ | ✅ |

**Total estimado**: ~880 líneas nuevas, 37+ tests, 6-7 demos

---

## ⚠️ RIESGOS

### Audio
- raylib audio puede no estar disponible en Termux
- Solución: Usar `miniaudio` o `rodio` como fallback

### HTTP
- Requiere TLS/SSL para HTTPS
- Solución: `ureq` con `native-tls` o `rustls`

### CSV
- Parsing de CSV con comas, quotes, escapes
- Solución: Usar crate `csv` de Rust

### Assets
- El struct `Assets` YA existe en `rydit-gfx` ✅
- Solo falta exponer como módulo ✅

### Partículas
- Sistema de partículas puede ser complejo
- Solución: Implementar versión simple primero (círculos)

---

## ✅ CRITERIOS DE ACEPTACIÓN

- [ ] `audio::beep()` funciona en Termux-X11
- [ ] `audio::click()` suena al hacer click en UI
- [ ] `http::get()` retorna datos de API pública
- [ ] `csv::read()` lee archivos CSV correctamente
- [ ] `csv::write()` escribe archivos CSV
- [ ] `stats::std_dev()` calcula desviación estándar
- [ ] `stats::variance()` calcula varianza
- [ ] `plot::ascii()` imprime gráfico en consola
- [ ] `assets::sprite()` crea sprites estilo Godot
- [ ] `assets::draw()` dibuja sprites cargados
- [ ] `particles::emit()` emite partículas (fuego, humo, etc.)
- [ ] `particles::update()` actualiza sistema
- [ ] `particles::draw()` dibuja partículas
- [ ] 37+ tests passing
- [ ] 6-7 demos funcionales
- [ ] README actualizado

---

## 🎯 ARQUITECTURA MODULAR

### Sistema Universal Ry (v0.8.2+)
```
┌─────────────────────────────────────────────────────────┐
│  RyDit Core (RyditModule trait)                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  rydit-rs/src/modules/                           │   │
│  │  ├── assets.rs         ← Assets Manager          │   │
│  │  └── mod.rs            ← Registro                │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  crates/rydit-anim/src/                          │   │
│  │  └── particles.rs      ← Particle System         │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  crates/rydit-science/src/                       │   │
│  │  ├── csv.rs            ← CSV Reader/Writer       │   │
│  │  └── stats_advanced.rs ← std_dev, variance       │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

<div align="center">

**🛡️ RyDit v0.5.1 - Audio + HTTP + CSV + Assets + Partículas**

*~880 líneas | 37+ tests | 6-7 demos | Arquitectura Modular*

</div>
