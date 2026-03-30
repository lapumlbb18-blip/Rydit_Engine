# 📋 EVALUACIÓN: SPLIT DE main.rs (v0.7.0)

**Fecha de evaluación**: v0.6.3 (2026-03-24)
**Estado actual**: `cargo fmt` completado ✅
**Próxima sesión**: Split de main.rs

---

## 📊 ESTADO ACTUAL (Post-cargo fmt)

| Métrica | Valor |
|---------|-------|
| **main.rs líneas** | ~4,200 líneas |
| **Formato** | ✅ Consistente (cargo fmt) |
| **Tests** | ✅ 137 passing |
| **Warnings** | ✅ 0 en build principal |
| **Binario** | ~600 KB |

---

## 🎯 OBJETIVO DEL SPLIT

Dividir `crates/rydit-rs/src/main.rs` (4,200 líneas) en módulos manejables:

```
crates/rydit-rs/src/
├── main.rs              # Entry point + CLI parsing (~80 líneas)
├── repl.rs              # REPL interactivo (~450 líneas)
├── bindings/
│   ├── mod.rs           # Exportaciones
│   ├── audio.rs         # audio::load_sound, audio::play (~180 líneas)
│   ├── particles.rs     # particles::fire, smoke, etc. (~120 líneas)
│   ├── migui.rs         # migui::button, slider, etc. (~250 líneas)
│   ├── assets.rs        # assets::load_texture, draw (~100 líneas)
│   └── stdlib.rs        # math, arrays, strings, io, random, time, json, regex, files (~400 líneas)
├── game_loop.rs         # Game loop con raylib (~450 líneas)
└── eval/
    ├── mod.rs           # Evaluador de expresiones
    ├── expr.rs          # Expresiones RyDit
    └── gfx.rs           # Expresiones gráficas (draw.*) (~550 líneas)
```

---

## 🔍 ANÁLISIS DE RIESGO

### ✅ **FACTORES POSITIVOS**

| Factor | Estado | Impacto |
|--------|--------|---------|
| `cargo fmt` completado | ✅ | Código consistente |
| Tests sólidos | ✅ 137 tests | Red de seguridad |
| 0 warnings | ✅ | Código limpio |
| Documentación | ✅ | Funciones claras |
| Git limpio | ✅ | Sin conflictos pendientes |

### ⚠️ **RIESGOS IDENTIFICADOS**

| Riesgo | Probabilidad | Impacto | Mitigación |
|--------|--------------|---------|------------|
| Romper demos existentes | Media | Alto | Tests exhaustivos post-split |
| Errores de borrow checker | Alta | Medio | Iteraciones pequeñas |
| Conflictos de imports | Media | Bajo | `mod.rs` bien definido |
| Regresiones de rendimiento | Baja | Medio | Benchmark pre/post |
| Pérdida de contexto | Baja | Bajo | Comentarios de trazabilidad |

---

## 📋 PLAN DE EJECUCIÓN (POR FASES)

### **FASE 1: PREPARACIÓN (30 min)**

```bash
# 1.1 Tag de respaldo
git tag v0.6.3-pre-split

# 1.2 Crear estructura de directorios
mkdir -p crates/rydit-rs/src/bindings
mkdir -p crates/rydit-rs/src/eval

# 1.3 Verificar tests pre-split
cargo test --quiet
# Esperado: 137 tests passing
```

---

### **FASE 2: EXTRACCIÓN DE REPL (45 min)**

**Archivos:**
- `main.rs` → `repl.rs` (~450 líneas)

**Funciones a mover:**
- `repl_mode()`
- `auto_complete()`
- `guardar_sesion()`
- `comandos_repl()`

**Pasos:**
1. Copiar funciones a `repl.rs`
2. Agregar `pub mod repl;` en `main.rs`
3. Ajustar imports
4. `cargo build` → fix errors
5. `cargo test` → verificar

**Criterio de éxito:**
```bash
cargo test -p rydit-rs repl  # 2-3 tests passing
```

---

### **FASE 3: EXTRACCIÓN DE BINDINGS (1.5 horas)**

**Archivos:**
- `main.rs` → `bindings/audio.rs` (~180 líneas)
- `main.rs` → `bindings/particles.rs` (~120 líneas)
- `main.rs` → `bindings/migui.rs` (~250 líneas)
- `main.rs` → `bindings/assets.rs` (~100 líneas)
- `main.rs` → `bindings/stdlib.rs` (~400 líneas)

**Funciones por módulo:**

**audio.rs:**
```rust
pub fn load_sound(id: &str, path: &str) -> bool
pub fn play(id: &str) -> bool
pub fn stop(id: &str) -> bool
// ... 10 funciones totales
```

**particles.rs:**
```rust
pub fn fire(x: f32, y: f32) -> ParticleEmitter
pub fn smoke(x: f32, y: f32) -> ParticleEmitter
// ... 5 efectos preset
```

**migui.rs:**
```rust
pub fn button(id: &str, label: &str, x, y, w, h) -> bool
pub fn slider(id: &str, value, min, max, x, y, w, h) -> f32
// ... 12 widgets
```

**assets.rs:**
```rust
pub fn load_texture(id: &str, path: &str) -> bool
pub fn draw(id: &str, x, y) -> bool
// ... 5 funciones
```

**stdlib.rs:**
```rust
// Wrapper para módulos embebidos
// math, arrays, strings, io, random, time, json, regex, files
```

**Pasos:**
1. Crear cada archivo
2. Mover funciones
3. Agregar `pub mod bindings;` en `main.rs`
4. `cargo build` → fix errors (iterativo)
5. `cargo test` → verificar por módulo

**Criterio de éxito:**
```bash
cargo test -p rydit-rs  # 23 tests passing
```

---

### **FASE 4: EXTRACCIÓN DE GAME LOOP (45 min)**

**Archivos:**
- `main.rs` → `game_loop.rs` (~450 líneas)

**Funciones a mover:**
- `run_game_loop()`
- `process_input()`
- `update()`
- `render()`

**Pasos:**
1. Copiar a `game_loop.rs`
2. Agregar `pub mod game_loop;` en `main.rs`
3. Ajustar imports de raylib
4. `cargo build` → fix errors
5. Probar demo_particles

**Criterio de éxito:**
```bash
cargo run --bin demo_particles  # 60 FPS, sin errors
```

---

### **FASE 5: EXTRACCIÓN DE EVAL (1 hora)**

**Archivos:**
- `main.rs` → `eval/mod.rs`
- `main.rs` → `eval/expr.rs`
- `main.rs` → `eval/gfx.rs`

**Funciones:**
- `evaluar_expr()` → expr.rs
- `evaluar_expr_gfx()` → gfx.rs
- Helpers → mod.rs

**Pasos:**
1. Crear estructura eval/
2. Mover funciones por archivo
3. Agregar `pub mod eval;` en `main.rs`
4. `cargo build` → fix errors
5. `cargo test` → verificar TODOS los tests

**Criterio de éxito:**
```bash
cargo test -p rydit-rs  # 23 tests passing
cargo test              # 137 tests passing (total)
```

---

### **FASE 6: VERIFICACIÓN FINAL (30 min)**

```bash
# 6.1 Build release
cargo build --release -p rydit-rs
# Esperado: 0 errors, 0 warnings, ~600 KB

# 6.2 Todos los tests
cargo test --quiet
# Esperado: 137 tests passing

# 6.3 Probar demos
cargo run --bin snake
cargo run --bin demo_particles
cargo run -- --gfx demos/demo_files.rydit

# 6.4 Verificar binario
ls -lh target/release/rydit-rs
# Esperado: ~600 KB
```

---

## 📊 MÉTRICAS ESPERADAS POST-SPLIT

| Métrica | Antes | Después | Cambio |
|---------|-------|---------|--------|
| **main.rs** | 4,200 líneas | ~80 líneas | -98% |
| **Módulos nuevos** | 0 | 10 archivos | +10 |
| **Líneas totales** | 4,200 | ~4,400 | +5% (imports) |
| **Tests** | 137 | 137+ | = o + |
| **Binario** | ~600 KB | ~600-610 KB | +0-2% |
| **Build time** | ~90s | ~85-95s | = (incremental) |
| **Mantenibilidad** | Difícil | Fácil | ✅✅✅ |

---

## 🛡️ RED DE SEGURIDAD

### **Tests Existentes (137)**
- lizer: 74 tests
- blast-core: 20 tests
- v-shield: 7 tests
- migui: 8 tests
- rydit-rs: 23 tests
- docs: 5 tests

### **Demos Funcionales**
- snake.rydit
- demo_particles.rs
- demo_files.rydit
- demo_regex.rydit

### **Backup Strategy**
```bash
# Antes de empezar
git tag v0.6.3-pre-split

# Si algo sale mal
git reset --hard v0.6.3-pre-split
```

---

## ⏱️ TIEMPO ESTIMADO

| Fase | Tiempo |
|------|--------|
| Preparación | 30 min |
| REPL | 45 min |
| Bindings | 1.5 horas |
| Game Loop | 45 min |
| Eval | 1 hora |
| Verificación | 30 min |
| **TOTAL** | **~5 horas** |

---

## ✅ CRITERIOS DE ACEPTACIÓN

- [ ] 0 errors de compilación
- [ ] 0 warnings
- [ ] 137+ tests passing (sin regresiones)
- [ ] main.rs <100 líneas
- [ ] Todos los demos funcionales
- [ ] Binario <650 KB
- [ ] Documentación actualizada

---

## 🚀 PRÓXIMOS PASOS (DESPUÉS DEL SPLIT)

**v0.7.1 - Clippy por módulo**
- Ejecutar `cargo clippy` en cada módulo
- Fix warnings específicos
- Código 100% limpio

**v0.7.2 - Animaciones 2D**
- Sprite sheets
- Easing functions
- Demo tanque animado

**v0.8.0 - Editor Visual**
- migui-based editor
- Inspector de propiedades

---

## 📝 NOTAS FINALES

1. **NO hacer en producción**: Este split es de desarrollo interno
2. **Commit frecuente**: Cada fase exitosa = commit separado
3. **Tests primero**: Si un test falla, fixear ANTES de continuar
4. **Documentar cambios**: Actualizar README si cambia API pública

---

<div align="center">

**🛡️ RyDit v0.7.0 - Split de main.rs**

*Evaluado: v0.6.3 | Próxima sesión: Split completo*

</div>
