# 📚 DEUDA TÉCNICA RYDIT

**Última actualización**: v0.6.0 (2026-03-23)

Este documento lista la deuda técnica acumulada durante el desarrollo acelerado de RyDit. **NO tocar durante features activos** (v0.6.x). Planificado para v0.7.0.

---

## 🔴 CRÍTICO - Refactor Urgente

### 1. Split de `main.rs` (3,662 líneas)

**Problema**: `crates/rydit-rs/src/main.rs` tiene 3,662 líneas en un solo archivo.

**Riesgo**: 
- Difícil de navegar
- Merge conflicts frecuentes
- Testing complicado
- Onboarding lento para nuevos contribuidores

**Solución Planificada (v0.7.0)**:

```
crates/rydit-rs/src/
├── main.rs              # Solo entry point + CLI parsing (~50 líneas)
├── repl.rs              # REPL interactivo (~400 líneas)
├── bindings/
│   ├── mod.rs           # Exportaciones
│   ├── audio.rs         # audio::load_sound, audio::play, etc. (~150 líneas)
│   ├── particles.rs     # particles::fire, particles::smoke, etc. (~100 líneas)
│   ├── migui.rs         # migui::button, migui::slider, etc. (~200 líneas)
│   ├── assets.rs        # assets::load_texture, assets::draw (~100 líneas)
│   └── stdlib.rs        # math, arrays, strings, io, random, time, json (~300 líneas)
├── game_loop.rs         # Game loop con raylib (~400 líneas)
└── eval/
    ├── mod.rs           # Evaluador de expresiones
    ├── expr.rs          # Expresiones RyDit
    └── gfx.rs           # Expresiones gráficas (draw.*) (~500 líneas)
```

**Orden de ejecución**:
1. Crear estructura de directorios
2. Extraer REPL → `repl.rs`
3. Extraer bindings → `bindings/*.rs`
4. Extraer game loop → `game_loop.rs`
5. Extraer eval → `eval/*.rs`
6. Actualizar imports en `main.rs`
7. Tests de regresión

**Tiempo estimado**: 4-6 horas
**Riesgo**: Alto (puede romper demos existentes)
**Mitigación**: Tests exhaustivos antes de commit

---

### 2. Refactor de Bindings (Patrón `__` vs namespace)

**Problema**: Bindings actuales usan patrón inconsistente:

```rust
// Actual (inconsistente):
__audio_play(id)      // Función interna
audio::play(id)       // Función RyDit (wrapper)

__particle_fire(x, y)
particles::fire(x, y)
```

**Solución**:

```rust
// Propuesto (consistente):
// crates/rydit-rs/src/bindings/audio.rs
pub fn load_sound(id: &str, path: &str) -> bool { ... }
pub fn play(id: &str) -> bool { ... }
pub fn stop(id: &str) -> bool { ... }

// crates/rydit-rs/src/main.rs (registro)
scope.insert("audio::load_sound", Func::new(audio::load_sound));
scope.insert("audio::play", Func::new(audio::play));
```

**Ventajas**:
- Sin prefijos `__` confusos
- Namespace claro en RyDit: `audio::play()`
- Más fácil de testear
- Documentación automática

**Tiempo estimado**: 2-3 horas
**Riesgo**: Medio (cambia API interna, no rompe demos RyDit)

---

## 🟡 IMPORTANTE - Mejoras de Calidad

### 3. Tests de Gráficos (rydit-gfx)

**Problema**: `rydit-gfx` no tiene tests automáticos (requiere X11).

**Solución**:
- Mock de raylib para tests
- Tests de lógica (sin renderizado real)
- Tests de conversión de colores
- Tests de partículas (física sin render)

**Tiempo estimado**: 2 horas
**Riesgo**: Bajo

---

### 4. Documentación de API Pública

**Problema**: Funciones sin documentación `///`.

**Ejemplo actual**:
```rust
pub fn fire(x: f32, y: f32) -> Self { ... }
```

**Propuesto**:
```rust
/// Crea un emisor de fuego en la posición especificada.
/// 
/// # Ejemplo
/// ```rydit
/// emitter = particles::fire(400, 300)
/// ```
/// 
/// # Argumentos
/// - `x`: Posición X en pantalla
/// - `y`: Posición Y en pantalla
/// 
/// # Retorna
/// ParticleEmitter configurado para efecto de fuego
pub fn fire(x: f32, y: f32) -> Self { ... }
```

**Tiempo estimado**: 3-4 horas
**Riesgo**: Bajo (solo documentación)

---

### 5. Error Messages en RyDit

**Problema**: Errores del REPL podrían ser más útiles.

**Actual**:
```
Error: función no existe
```

**Propuesto**:
```
Error en línea 15, columna 8:
  Función 'particlas::fire' no existe

  14 |   emitter = particlas::fire(400, 300)
     |                        ^^^^
     |
  ¿Quisiste decir: 'particles::fire'?
  
  Funciones disponibles en 'particles':
    - fire(x, y)
    - smoke(x, y)
    - explosion(x, y)
```

**Tiempo estimado**: 4-5 horas
**Riesgo**: Medio (cambia parser)

---

## 🟢 DESEABLE - Optimizaciones

### 6. Cargo Bloat Analysis

**Acción**: Instalar y ejecutar `cargo-bloat`

```bash
cargo install cargo-bloat
cargo bloat --release --crates
```

**Objetivo**: Identificar dependencias que más aumentan binario.

**Meta v1.0.0**: Binario release <600 KB (actual: ~550 KB)

---

### 7. Benchmarking Continuo

**Actual**: 16 benchmarks en `crates/lizer/benches/bench_lizer.rs`

**Propuesto**:
- Benchmarks para rydit-gfx (renderizado)
- Benchmarks para rydit-rs (ejecución RyDit)
- CI/CD que compare benchmarks (evitar regresiones)

**Tiempo estimado**: 2 horas
**Riesgo**: Bajo

---

### 8. Módulo `files` (Lectura/Escritura)

**Propuesto**:
```rydit
files::read("archivo.txt")      # Lee archivo completo
files::write("archivo.txt", "contenido")  # Escribe archivo
files::append("archivo.txt", "más")       # Añade al final
files::exists("archivo.txt")    # Verifica existencia
files::delete("archivo.txt")    # Elimina archivo
```

**Implementación**: ~100 líneas Rust
**Impacto binario**: +10 KB
**Prioridad**: Media (útil para guardar partidas, configs)

---

### 9. Módulo `regex`

**Propuesto**:
```rydit
regex::match("hola\\d+", "hola123")        # true/false
regex::replace("hola\\d+", "adiós", "hola123")  # "adiós"
regex::split(",", "a,b,c")                 # ["a", "b", "c"]
regex::find_all("\\d+", "a1b2c3")          # ["1", "2", "3"]
```

**Implementación**: Depende de `regex` crate (~50 líneas wrapper)
**Impacto binario**: +30-40 KB
**Prioridad**: Media (útil para validación input, parsing)

---

### 10. Animaciones 2D (Spritesheets)

**Propuesto**:
```rydit
# Cargar spritesheet
anim::load("tanque", "sprites/tank.png", 16, 16, 4)  # 4 frames

# Reproducir animación
anim::play("tanque", "walk")
anim::set_fps("tanque", 12)
anim::is_playing("tanque")  # true/false

# Easing functions
t = 0.5  # Progreso (0.0 a 1.0)
y = ease::in_quad(t)   # 0.25
y = ease::out_quad(t)  # 0.75
y = ease::bounce(t)    # 0.8+
```

**Implementación**: ~500-700 líneas Rust
**Impacto binario**: +50-80 KB
**Prioridad**: Alta (feature estrella v0.7.0)

---

## 📋 PLAN DE EJECUCIÓN v0.7.0

### Semana 1: Refactor Crítico
- [ ] Split de main.rs
- [ ] Refactor bindings
- [ ] Tests de regresión

### Semana 2: Calidad
- [ ] Tests rydit-gfx (mocks)
- [ ] Documentación API
- [ ] Error messages mejorados

### Semana 3: Features
- [ ] Módulo files
- [ ] Módulo regex
- [ ] Animaciones 2D (spritesheets + easing)

### Semana 4: Optimización
- [ ] cargo-bloat analysis
- [ ] Benchmarks continuos
- [ ] Demo tanque animado

---

## 🎯 PRIORIDADES

| Prioridad | Issue | Impacto | Esfuerzo |
|-----------|-------|---------|----------|
| 🔴 | Split main.rs | Alto | Alto |
| 🔴 | Refactor bindings | Medio | Medio |
| 🟡 | Tests rydit-gfx | Medio | Bajo |
| 🟡 | Documentación API | Bajo | Medio |
| 🟡 | Error messages | Alto | Medio |
| 🟢 | cargo-bloat | Bajo | Bajo |
| 🟢 | Benchmarks | Bajo | Bajo |
| 🟢 | Módulo files | Medio | Bajo |
| 🟢 | Módulo regex | Medio | Bajo |
| 🟢 | Animaciones 2D | Alto | Alto |

---

## 📝 NOTAS

- **NO tocar refactor durante features activos** (v0.6.x)
- **Siempre tests de regresión** después de refactor
- **Commits pequeños** y frecuentes durante refactor
- **Backup antes de empezar** (git tag v0.6.1-refactor-start)

---

<div align="center">

**🛡️ RyDit v0.7.0 - Refactor + Animaciones 2D**

[MANIFIESTO.md](MANIFIESTO.md) • [Roadmap](MANIFIESTO.md#-roadmap-público)

</div>
