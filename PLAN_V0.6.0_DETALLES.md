# 🎯 PLAN v0.6.0 - ANIMACIONES + TERMUX-X11 + MANIFIESTO

**Fecha:** 2026-03-23
**Estado:** ✅ APROBADO - En discusión de detalles

---

## 📋 FASES PRINCIPALES

### FASE 1: FIX PANTALLA NEGRA TERMUX-X11 (Prioridad 1)
- [ ] Script `ejecutar_rydit.sh` con auto-detección
- [ ] Detección automática en Rust (main.rs)
- [ ] Demo `gpu_check` para verificación

### FASE 2: MANIFIESTO RYDIT (Prioridad 2)
- [ ] Crear `MANIFIESTO.md`
- [ ] Actualizar README con enlace al manifiesto
- [ ] Agregar sección "Nuestra Misión" en README

### FASE 3: ANIMACIONES 2D BÁSICAS (Prioridad 3)
- [ ] `animation.rs` (struct Animation)
- [ ] Funciones RyDit: `anim::load()`, `anim::play()`
- [ ] Demo tanque animado

### FASE 4: ORGANIZACIÓN DE ARCHIVOS (Prioridad 4)
- [ ] Mover demos antiguos a `historial/`
- [ ] Actualizar estructura de directorios
- [ ] Limpiar root (dejar solo 10 archivos .md)

### FASE 5: REPOSITORIO GITHUB
- [ ] Actualizar descripción en GitHub
- [ ] Agregar `MANIFIESTO.md` al repo
- [ ] Actualizar screenshots

---

## 🔍 DETALLES PENDIENTES DE DISCUSIÓN

### 1. BINDINGS INDICADOS (Evaluación)

**Problema actual:**
- Funciones Rust se exponen como `__audio_play`, `__assets_draw`
- Usuario final usa `audio::play()`, `assets::draw()`
- ¿Hay una mejor forma de hacer el mapeo?

**Opciones:**

#### Opción A: Mantener bindings actuales (doble nombre)
```rust
// En main.rs
if (name == "__audio_play" || name == "audio::play") && args.len() == 1 {
    // ...
}
```
**Pros:**
- ✅ Compatible con código existente
- ✅ Fácil de entender
- ✅ No requiere cambios

**Contras:**
- ❌ Código repetitivo
- ❌ Dos nombres para misma función

#### Opción B: Sistema de aliases automático
```rust
// En main.rs, al inicio
let aliases = HashMap::from([
    ("audio::play", "__audio_play"),
    ("audio::stop", "__audio_stop"),
    ("assets::draw", "__assets_draw"),
]);

// En evaluación, buscar alias automáticamente
let real_name = aliases.get(name).unwrap_or(&name);
```
**Pros:**
- ✅ Menos código repetitivo
- ✅ Centralizado
- ✅ Fácil de extender

**Contras:**
- ❌ Requiere refactor (~50 líneas)
- ❌ Posible overhead mínimo

#### Opción C: Macro de Rust para bindings
```rust
// En crates/lizer/src/lib.rs
macro_rules! bind_function {
    ($rydit_name:expr, $rust_fn:expr) => {
        ($rydit_name, $rust_fn)
    };
}

// Registro centralizado
let bindings = vec![
    bind_function!("audio::play", audio_play),
    bind_function!("audio::stop", audio_stop),
];
```
**Pros:**
- ✅ Muy limpio
- ✅ Type-safe
- ✅ Escalable

**Contras:**
- ❌ Requiere refactor mayor (~200 líneas)
- ❌ Curva de aprendizaje para contribuidores

**Mi recomendación:** **Opción A (mantener)** por ahora.
- Estamos en "recta final"
- Funciona bien
- Refactor puede esperar a v0.7.0 o v1.0.0

---

### 2. STDLIB BINARIO - EMBEBER/EMBEDER

**Problema actual:**
- Módulos stdlib están en `crates/modules/*.rydit`
- Usuario debe tener archivos `.rydit` en sistema
- ¿Podemos embeber stdlib en el binario?

**Opciones:**

#### Opción A: Mantener archivos externos (actual)
```rydit
# Usuario debe tener:
crates/modules/math.rydit
crates/modules/arrays.rydit
crates/modules/audio.rydit

# En script:
import math
import arrays
```
**Pros:**
- ✅ Usuario puede modificar módulos
- ✅ Fácil debugging
- ✅ Sin aumento de binario

**Contras:**
- ❌ Usuario debe gestionar archivos
- ❌ Posibles errores de ruta
- ❌ No es "plug-and-play"

#### Opción B: Embed con `include_str!` de Rust
```rust
// En crates/rydit-rs/src/main.rs
const MATH_MODULE: &str = include_str!("../../modules/math.rydit");
const ARRAYS_MODULE: &str = include_str!("../../modules/arrays.rydit");

// Al hacer import, cargar desde string embebido
fn cargar_modulo_embebido(nombre: &str) -> Option<&str> {
    match nombre {
        "math" => Some(MATH_MODULE),
        "arrays" => Some(ARRAYS_MODULE),
        _ => None,
    }
}
```
**Pros:**
- ✅ Binario auto-contenido
- ✅ Usuario no gestiona archivos
- ✅ Sin errores de ruta

**Contras:**
- ❌ Binario más grande (~10-20 KB)
- ❌ Usuario no puede modificar fácilmente
- ❌ Requiere recompilar para cambiar módulos

#### Opción C: Sistema híbrido (recomendado)
```rust
// Intentar cargar desde archivo primero
if let Ok(content) = fs::read_to_string(ruta_modulo) {
    return Ok(content);
}

// Fallback a módulo embebido
if let Some(embebido) = cargar_modulo_embebido(nombre) {
    return Ok(embebido.to_string());
}

// Error: módulo no encontrado
Err(format!("Módulo '{}' no encontrado", nombre))
```
**Pros:**
- ✅ Mejor de ambos mundos
- ✅ Usuario puede override con archivo local
- ✅ Fallback seguro
- ✅ Binario funcional sin archivos externos

**Contras:**
- ❌ Complejidad adicional (~30 líneas)
- ❌ Binario ligeramente más grande

**Mi recomendación:** **Opción C (híbrido)**
- Usuario avanzado puede modificar módulos
- Usuario básico tiene todo embebido
- Sin breaking changes

**Implementación estimada:** ~1 hora

---

### 3. EVALUACIÓN DE DEPENDENCIAS

**Estado actual:**
```toml
[dependencies]
raylib = "5.5"        # ~500 KB
serde_json = "1.0"    # ~80 KB
migui = "0.4.0"       # ~600 líneas (interno)
```

**¿Podemos reducir?**

#### Dependencias críticas (NO tocar):
- ✅ `raylib` - Gráficos y audio (esencial)
- ✅ `serde_json` - JSON (necesario para `:save` en REPL)

#### Dependencias opcionales (evaluar):
- ⚠️ `serde` - Derivado de serde_json, ¿se puede eliminar?
- ⚠️ `raylib-sys` - FFI, ya incluido en raylib

**Análisis de `serde`:**
```rust
// Uso actual en main.rs
use serde_json::json;

// En :save comando del REPL
let json = json!({ "variables": vars });
```

**¿Podemos eliminar serde?**
- **Opción A:** JSON manual (~50 líneas)
  ```rust
  fn valor_to_json_string(valor: &Valor) -> String {
      match valor {
          Valor::Num(n) => format!("{}", n),
          Valor::Texto(s) => format!("\"{}\"", s.replace('"', "\\\"")),
          Valor::Bool(b) => format!("{}", b),
          // ...
      }
  }
  ```
  **Ahorro:** ~80 KB
  **Costo:** 50 líneas de código manual

- **Opción B:** Mantener serde_json
  **Costo:** ~80 KB
  **Beneficio:** Robusto, mantenido, estándar

**Mi recomendación:** **Mantener serde_json**
- 80 KB es aceptable
- JSON manual es propenso a bugs
- serde es estándar de industria

---

### 4. BINARIO FINAL - TAMAÑO OBJETIVO

**Estado actual:**
```
rydit-rs: ~920 KB (debug)
rydit-rs: ~650 KB (release con LTO)
```

**Objetivo v1.0.0:**
```
rydit-rs: <500 KB (release)
```

**Estrategias de reducción:**

#### 1. Optimizaciones de Cargo.toml
```toml
[profile.release]
opt-level = "z"      # Optimizar tamaño (ya está)
lto = true           # Link Time Optimization (ya está)
panic = "abort"      # Sin unwind (ya está)
codegen-units = 1    # Mejor optimización (ya está)
strip = true         # NUEVO: Eliminar símbolos de debug
```

**Ahorro estimado:** ~50-100 KB

#### 2. Eliminar código muerto
```bash
# Analizar con cargo-bloat
cargo install cargo-bloat
cargo bloat --release -n 20
```

**Posibles candidatos:**
- Funciones de debug no usadas
- Errores no alcanzables
- Código legacy de versiones antiguas

**Ahorro estimado:** ~20-30 KB

#### 3. Usar `raylib-sys` directamente
En lugar de `raylib` wrapper, usar FFI directo:
```rust
// En lugar de:
use raylib::prelude::*;

// Usar:
use raylib_sys::*;
```

**Ahorro estimado:** ~100-150 KB
**Costo:** ~200 líneas de código FFI manual
**Riesgo:** Más propenso a bugs

**Mi recomendación:** **NO hacer esto**
- Ahorro no justifica riesgo
- raylib wrapper es seguro y cómodo

#### 4. Features opcionales de raylib
```toml
[dependencies.raylib]
version = "5.5"
default-features = false
features = ["graphics", "audio"]  # Solo lo necesario
```

**Ahorro estimado:** ~50 KB
**Riesgo:** Funcionalidad limitada

**Mi recomendación:** **Evaluar después de v0.6.0**

---

### 5. ESTABILIDAD - CHECKLIST v1.0.0

**Para considerar "production ready":**

#### Crítico (bloqueante):
- [ ] **Sin panics en producción** (solo en tests)
  - Estado: ✅ 4 unwrap() fixeados en v0.5.2
  - Pendiente: Revisar resto del código

- [ ] **Manejo de errores gráfico**
  - Estado: ✅ 20 tipos de error en lizer
  - Pendiente: Errores específicos de assets

- [ ] **Tests automáticos**
  - Estado: ✅ 45+ tests pasando
  - Pendiente: Tests de integración gráfica

#### Importante (recomendado):
- [ ] **Documentación completa**
  - Estado: ✅ README + README_EN + CHANGELOGs
  - Pendiente: Tutoriales paso a paso

- [ ] **Ejemplos funcionales**
  - Estado: ✅ 5+ demos (snake, tank, particles)
  - Pendiente: Demo que muestre todas las features

- [ ] **Binarios estables**
  - Estado: ✅ Build limpio sin warnings
  - Pendiente: Reducir tamaño <500 KB

#### Deseable (nice to have):
- [ ] **CI/CD** (GitHub Actions)
  - Estado: ❌ No implementado
  - Pendiente: Build automático en push

- [ ] **Benchmarks públicos**
  - Estado: ✅ 16 benchmarks creados
  - Pendiente: Script de benchmarking automático

- [ ] **Asset store**
  - Estado: ❌ No implementado
  - Pendiente: Repositorio de sprites/sonidos

---

### 6. ROADMAP ACTUALIZADO CON EVALUACIONES

| Versión | Features | Bindings | Stdlib | Tamaño | Estabilidad |
|---------|----------|----------|--------|--------|-------------|
| **v0.6.0** | Animaciones + Fix X11 | Mantener | Embeber (híbrido) | ~900 KB | ✅ Tests |
| **v0.7.0** | Motor escenas | Evaluar refactor | Mantener híbrido | ~850 KB | ✅ +CI/CD |
| **v0.8.0** | Editor visual | Evaluar macros | Opcional embeber | ~800 KB | ✅ Benchmarks |
| **v0.9.0** | Optimizaciones | Decidir | Decidir | ~700 KB | ✅ Docs completas |
| **v1.0.0** | Production ready | Estable | Estable | **<500 KB** | ✅ **100% stable** |

---

## 💬 PUNTOS DE DISCUSIÓN

### Para decidir AHORA (v0.6.0):

1. **Bindings:** ¿Mantener doble nombre o refactorizar?
   - Mi voto: **Mantener** (ahorrar tiempo para animaciones)

2. **Stdlib embebido:** ¿Sí o no?
   - Mi voto: **Sí (híbrido)** - mejor UX sin perder flexibilidad

3. **Tamaño binario:** ¿Optimizar ahora o después?
   - Mi voto: **Después** - primero features, luego optimización

### Para decidir DESPUÉS (v0.7.0+):

4. **CI/CD:** ¿GitHub Actions o otro?
5. **Asset store:** ¿Repositorio separado o integrado?
6. **Editor visual:** ¿TUI (terminal) o GUI (raylib)?

---

## 📊 RESUMEN DE DECISIONES

| Feature | Decisión | Justificación |
|---------|----------|---------------|
| Bindings | Mantener | Tiempo > perfección ahora |
| Stdlib | Embeber (híbrido) | Mejor UX + flexibilidad |
| serde_json | Mantener | 80 KB vale la pena |
| Tamaño | Optimizar después | Features primero |
| CI/CD | Después de v0.6.0 | Prioridad actual: animaciones |

---

## 🎯 PRÓXIMOS PASOS INMEDIATOS

1. **Completar backup** (en progreso)
2. **Discutir y decidir** estos puntos (ahora)
3. **Empezar v0.6.0** con decisiones tomadas
4. **Implementar Fase 1** (Fix Termux-X11)
5. **Implementar Fase 2** (Manifiesto)
6. **Implementar Fase 3** (Animaciones)

---

**¿Qué opinas de estos puntos? ¿Algo que agregar o cambiar?** 🤔
