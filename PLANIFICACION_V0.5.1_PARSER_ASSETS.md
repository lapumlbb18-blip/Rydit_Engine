# 🛡️ PLANIFICACIÓN v0.5.1 - PARSER + ASSETS + PARTICLES + CSV + AUDIO + HTTP

**Fecha**: 2026-03-27
**Versión actual**: v0.5.0 ✅ ESTABLE (7 demos, 157 tests)
**Versión objetivo**: v0.5.1

---

## 🎯 OBJETIVOS PRINCIPALES

### 1️⃣ PARSER (PRIORIDAD CRÍTICA) ⚠️⚠️⚠️
**Problema**: Paréntesis fallan, expresiones complejas se rompen, arrays limitados

**Features**:
- ✅ Paréntesis que funcionen SIEMPRE: `(a + b) * c`
- ✅ Expresiones complejas sin dolor: `"texto" + (x * 2) + "más"`
- ✅ Arrays multidimensionales reales: `[[1,2,3], [4,5,6]]`
- ✅ Concatenación sin fixes manuales

**Archivo**: `crates/lizer/src/lib.rs`
**Tiempo estimado**: 2-3 días
**Impacto**: 10/10 - RyDit usable para código real

---

### 2️⃣ ASSETS MANAGER (PRIORIDAD ALTA) ⭐⭐⭐
**Estilo Godot**: Sprite2D anidado a escena 2D

**Arquitectura**:
```rust
// crates/rydit-rs/src/modules/assets.rs
assets::sprite(id, path)      // Crear sprite 2D
assets::draw(id, x, y, scale) // Dibujar sprite
assets::load(id, path)        // Cargar textura
```

**Uso en RyDit**:
```rydit
# Estilo Godot - Sprite2D anidado
dark.slot tank = assets::sprite("tank", "sprites/tank.png")
assets::draw(tank, 400, 300, 2.0)

# O directo:
assets::load("tank", "sprites/tank.png")
assets::draw_scaled("tank", 400, 300, 4.0)
```

**Features**:
- Carga de texturas (PNG, JPG)
- Draw con escala
- Draw centrado
- Sprites reutilizables

**Tiempo estimado**: 1-2 días
**Impacto**: 9/10 - Sprites estilo Godot

---

### 3️⃣ PARTÍCULAS EN rydit-anim (PRIORIDAD ALTA) ⭐⭐⭐
**Arquitectura**:
```rust
// crates/rydit-anim/src/particles.rs
particles::emit(x, y, effect)  // Emitir partículas
particles::update()            // Actualizar sistema
particles::draw()              // Dibujar partículas
```

**Efectos**:
- 🔥 `particles::emit(x, y, "fire")` - Fuego
- 💨 `particles::emit(x, y, "smoke")` - Humo
- ✨ `particles::emit(x, y, "spark")` - Chispas
- 💥 `particles::emit(x, y, "explosion")` - Explosión
- 🌧️ `particles::emit(x, y, "rain")` - Lluvia

**Uso en game loop**:
```rydit
shield.init
ryda frame < 1000 {
    particles::emit(400, 300, "fire")
    particles::update()
    particles::draw()
}
```

**Tiempo estimado**: 1-2 días
**Impacto**: 8/10 - Efectos visuales

---

### 4️⃣ CSV + DATA SCIENCE (PRIORIDAD MEDIA) ⭐⭐
**En rydit-science**:
```rydit
csv::read("file.csv")           // Leer CSV
csv::write(data, "file.csv")    // Escribir CSV
stats::std_dev([1,2,3,4,5])     // Desviación estándar
stats::variance([1,2,3,4,5])    // Varianza
plot::ascii(data)               // Gráfico ASCII en consola
```

**Dependencias**:
- `csv` crate de Rust (ligero, ~500 KB)

**Tiempo estimado**: 1 día
**Impacto**: 7/10 - Data science

---

### 5️⃣ AUDIO (PRIORIDAD MEDIA) ⭐⭐
```rydit
audio::beep(frecuencia, duracion)  // Beep tipo consola
audio::click()                      // Click UI
audio::play_sound("path")          // WAV/MP3
```

**Dependencias**:
- `miniaudio` o `rodio` (audio)

**Tiempo estimado**: 1 día
**Impacto**: 7/10 - Sonidos

---

### 6️⃣ HTTP (PRIORIDAD MEDIA) ⭐⭐
```rydit
dark.slot response = http::get("https://api.example.com/data")
voz response
```

**Dependencias**:
- `ureq` (HTTP client ligero, ~1 MB)

**Tiempo estimado**: 1 día
**Impacto**: 6/10 - APIs externas

---

## 📋 ORDEN DE IMPLEMENTACIÓN

| Orden | Feature | Tiempo Est. | Impacto |
|-------|---------|-------------|---------|
| 1 | **Parser** | 2-3 días | 10/10 - RyDit usable |
| 2 | **Assets Manager** | 1-2 días | 9/10 - Sprites estilo Godot |
| 3 | **Partículas** | 1-2 días | 8/10 - Efectos visuales |
| 4 | **CSV + Stats** | 1 día | 7/10 - Data science |
| 5 | **Audio** | 1 día | 7/10 - Sonidos |
| 6 | **HTTP** | 1 día | 6/10 - APIs externas |

**Total estimado**: 7-9 días de trabajo

---

## 📊 METAS

| Feature | Líneas | Tests | Demo |
|---------|--------|-------|------|
| Parser fix | ~500 | 20+ | ✅ Expresiones complejas |
| Assets Manager | ~200 | 8+ | ✅ Tanque + Helicóptero |
| Partículas | ~250 | 10+ | ✅ Fuego + Humo + Explosión |
| CSV + Stats | ~200 | 9+ | ✅ Leer/escribir CSV |
| Audio | ~100 | 5+ | ✅ Beep + sonidos |
| HTTP | ~50 | 3+ | ✅ API call |

**Total**: ~1,300 líneas, 55+ tests, 6 demos

---

## 📦 CRATES INVOLUCRADOS

### A modificar
- `crates/lizer/src/lib.rs` - Parser completo
- `crates/rydit-rs/src/modules/assets.rs` - Assets Manager
- `crates/rydit-anim/src/particles.rs` - Particle System
- `crates/rydit-science/src/csv.rs` - CSV Reader/Writer
- `crates/rydit-science/src/stats.rs` - std_dev, variance
- `crates/rydit-rs/Cargo.toml` - Dependencias (ureq, csv, audio)

### Nuevos módulos
- `crates/rydit-rs/src/modules/audio.rs` - Audio Module
- `crates/rydit-rs/src/modules/http.rs` - HTTP Module

---

## ⚠️ RIESGOS

### Parser
- Refactorizar puede romper demos existentes
- Mitigación: Tests exhaustivos antes y después

### Assets
- El struct `Assets` YA existe en `rydit-gfx` ✅
- Solo falta exponer como módulo ✅

### Partículas
- Sistema puede ser complejo
- Mitigación: Implementar versión simple primero (círculos)

### Audio
- raylib audio puede no estar disponible en Termux
- Mitigación: Usar `miniaudio` o `rodio` como fallback

### HTTP
- Requiere TLS/SSL para HTTPS
- Mitigación: `ureq` con `native-tls` o `rustls`

### CSV
- Parsing de CSV con comas, quotes, escapes
- Mitigación: Usar crate `csv` de Rust

---

## ✅ CRITERIOS DE ACEPTACIÓN

- [ ] Parser: `(a + b) * c` funciona SIEMPRE
- [ ] Parser: `[[1,2,3], [4,5,6]]` arrays multidimensionales
- [ ] Parser: `"texto" + variable` sin fixes
- [ ] `assets::sprite()` crea sprites estilo Godot
- [ ] `assets::draw()` dibuja sprites cargados
- [ ] `particles::emit()` emite partículas (fuego, humo, etc.)
- [ ] `particles::update()` actualiza sistema
- [ ] `particles::draw()` dibuja partículas
- [ ] `csv::read()` lee archivos CSV correctamente
- [ ] `csv::write()` escribe archivos CSV
- [ ] `stats::std_dev()` calcula desviación estándar
- [ ] `stats::variance()` calcula varianza
- [ ] `audio::beep()` funciona en Termux-X11
- [ ] `audio::click()` suena al hacer click en UI
- [ ] `http::get()` retorna datos de API pública
- [ ] 55+ tests passing
- [ ] 6 demos funcionales
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
│  │  crates/lizer/src/                               │   │
│  │  └── lib.rs            ← Parser refactorizado    │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  crates/rydit-rs/src/modules/                    │   │
│  │  ├── assets.rs         ← Assets Manager          │   │
│  │  ├── audio.rs          ← Audio Module            │   │
│  │  └── http.rs           ← HTTP Module             │   │
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
│  │  └── stats.rs          ← std_dev, variance       │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## 📝 PASOS INMEDIATOS

### Paso 1: Parser (COMIENZO)
1. Leer `lizer/src/lib.rs` completo
2. Identificar bugs en `parse_primary()`
3. Identificar bugs en `parse_expression()`
4. Agregar tests de estrés
5. Refactorizar
6. Validar con demos existentes

### Paso 2: Assets Manager
1. Leer `rydit-gfx/src/lib.rs` (struct Assets)
2. Crear `rydit-rs/src/modules/assets.rs`
3. Implementar `sprite()`, `draw()`, `load()`
4. Registrar como `RyditModule`
5. Crear demo `demo_assets.rydit`
6. Tests

### Paso 3: Partículas
1. Crear `rydit-anim/src/particles.rs`
2. Implementar `ParticleSystem`, `Particle` structs
3. Implementar `emit()`, `update()`, `draw()`
4. Agregar a `rydit-anim` module
5. Crear demo `demo_particulas.rydit`
6. Tests

### Paso 4: CSV + Stats
1. Agregar `csv` dependency a `rydit-science/Cargo.toml`
2. Crear `rydit-science/src/csv.rs`
3. Implementar `csv::read()`, `csv::write()`
4. Agregar `stats::std_dev()`, `stats::variance()`
5. Crear demo `demo_csv.rydit`
6. Tests

### Paso 5: Audio
1. Agregar `miniaudio` o `rodio` dependency
2. Crear `rydit-rs/src/modules/audio.rs`
3. Implementar `beep()`, `click()`, `play_sound()`
4. Crear demo `demo_audio.rydit`
5. Tests

### Paso 6: HTTP
1. Agregar `ureq` dependency a `rydit-rs/Cargo.toml`
2. Crear `rydit-rs/src/modules/http.rs`
3. Implementar `http::get()`, `http::post()`
4. Crear demo `demo_http.rydit`
5. Tests

### Paso 7: Integración + Docs
1. Demo combinado (todos los features)
2. Actualizar README.md
3. Actualizar QWEN.md
4. Release v0.5.1

---

<div align="center">

**🛡️ RyDit v0.5.1 - Parser Maduro + Módulos**

*~1,300 líneas | 55+ tests | 6 demos | Arquitectura Modular*

**Orden: Parser → Assets → Partículas → CSV → Audio → HTTP**

</div>
