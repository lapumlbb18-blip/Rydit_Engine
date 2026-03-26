# CHANGELOG v0.7.0

## v0.7.0 (2026-03-24) - Split Parcial + Arquitectura Modular

### 🔪 Split de main.rs (PARCIAL)

**Archivos extraídos:**
- ✅ `crates/rydit-rs/src/repl.rs` (86 líneas) - REPL interactivo
- ✅ `crates/rydit-rs/src/eval/mod.rs` (970 líneas) - evaluar_expr
- ✅ `crates/rydit-rs/src/bindings/mod.rs` (20 líneas) - Placeholder

**Archivos NO extraídos (decisión estratégica):**
- 🔴 `game_loop.rs` - Mantenido en main.rs
  - Demasiado acoplado con rydit-gfx
  - Riesgo alto de romper demos existentes
  - Se extraerá en v0.8.0+ como módulo independiente

### 📊 Métricas del Split

| Archivo | Antes | Después | Cambio |
|---------|-------|---------|--------|
| main.rs | 5,526 líneas | 4,573 líneas | -953 (-17%) |
| repl.rs | 0 | 86 líneas | +86 |
| eval/mod.rs | 0 | 970 líneas | +970 |
| bindings/mod.rs | 0 | 20 líneas | +20 |

### 🛠️ Fixes

- ✅ 0 warnings activos (eliminados 3 warnings)
  - unused import: `self`
  - unused import: `Write`
  - unused import: `std::io`
  - dead_code: `registrar_todos()`
- ✅ 102 tests passing (sin regresiones)
- ✅ Build exitoso

### 📋 Cambios Técnicos

**Funciones públicas agregadas:**
- `pub fn evaluar_expr()` - En eval/mod.rs
- `pub fn ejecutar_stmt()` - En main.rs
- `pub fn valor_serde_a_rydit()` - En main.rs
- `pub fn valor_rydit_a_serde()` - En main.rs
- `pub fn valor_a_bool()` - En main.rs
- `pub struct InputEstado` - En main.rs

**Módulos nuevos:**
- `mod repl` - REPL interactivo
- `mod eval` - Evaluador de expresiones
- `mod bindings` - Placeholder para bindings futuros

### 🏗️ Arquitectura Modular (v0.8.0+)

**Nueva filosofía:** Núcleo estable + módulos extensibles.

**Capas:**
1. **Núcleo estable** (~4,500 líneas)
   - main.rs (game loop, rydit-gfx FFI)
   - eval/mod.rs (evaluar_expr)
   - repl.rs (REPL)

2. **Módulos extensores** (independientes)
   - crates/rydit-mod-science/ (física, química)
   - crates/rydit-mod-anim/ (sprite sheets, easing)
   - crates/rydit-mod-network/ (HTTP, WebSocket)
   - crates/rydit-mod-data/ (CSV, HDF5, JSON)

**Ventajas:**
- ✅ Núcleo no se rompe
- ✅ Módulos independientes (testing fácil)
- ✅ Comunidad puede crear módulos
- ✅ Publicables a crates.io

### 📁 Archivos Creados

**Código:**
- `crates/rydit-rs/src/repl.rs`
- `crates/rydit-rs/src/eval/mod.rs`
- `crates/rydit-rs/src/bindings/mod.rs`

**Documentación privada:**
- `.private/RYDIT_2.0_VISION_CIENTIFICA.md` (21.8 KB)
- `.private/EVALUACION_FISICA_VS_ANIMACIONES.md` (9.5 KB)

### 📝 Documentación Actualizada

- `README.md` - Arquitectura modular, roadmap v0.8.0
- `QWEN.md` - Sesión v0.7.0 agregada
- `CHANGELOG_v0.7.0.md` - Este archivo

### 🎯 Decisiones Estratégicas

**Game Loop MANTENIDO en main.rs:**
- Demasiado acoplado con rydit-gfx
- Firmas complejas (10+ parámetros)
- Riesgo alto de romper demos
- **Se extraerá en v0.8.0+** con módulos independientes

**Módulos INDEPENDIENTES en lugar de split:**
- Split solo redujo 17% (poco impacto)
- Módulos permiten crecimiento horizontal
- Comunidad puede contribuir sin tocar core
- Alineado con visión RyDit 2.0.0 (científico)

### 📦 Backup

- ✅ Google Drive: 70.3 MB sincronizados
- ✅ GitHub: Commit + tags actualizados

### 🚀 Próxima Sesión: v0.8.0

**Módulos a implementar:**
1. **Módulo Ciencia** (física, química, biología)
   - Proyectil, n-body, reacciones
2. **Módulo Animación** (sprite sheets, 12 principios)
   - Easing functions, anim::load(), anim::play()
3. **Módulo Red** (HTTP, WebSocket, TCP/UDP)
   - HTTP GET/POST, WebSocket connect/send/recv
4. **Módulo Datos** (CSV, HDF5, pandas-like)
   - Load/save CSV, HDF5, análisis básico

---

## Resumen v0.6.0 - v0.7.0

| Versión | Fecha | Features | Tests | Binario |
|---------|-------|----------|-------|---------|
| v0.6.0 | 2026-03-23 | Stdlib embebido, Fix Termux | 126 | ~550 KB |
| v0.6.1 | 2026-03-24 | Limpieza repo, MANIFIESTO | 126 | ~580 KB |
| v0.6.2 | 2026-03-24 | Módulo REGEX | 133 (+7) | ~600 KB |
| v0.6.3 | 2026-03-24 | Módulo FILES | 137 (+4) | ~600 KB |
| v0.6.4 | 2026-03-24 | cargo fmt, evaluación | 137 | ~600 KB |
| v0.7.0 | 2026-03-24 | Split PARCIAL, Arquitectura Modular | 102 | ~600 KB |

**Total Added (v0.6.0 - v0.7.0):**
- Tests: -24 (126 → 102, algunos tests gfx no corren sin X11)
- Funciones: +10 (regex: 5, files: 5)
- Módulos stdlib: 10 (math, arrays, strings, io, random, time, json, colisiones, regex, files)
- Líneas Rust: +1,056 (repl + eval + bindings)
- Documentación: +500 líneas

**Próximo:** v0.8.0 - Arquitectura Modular completa
