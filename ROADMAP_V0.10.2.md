# 🛡️ RyDit Engine - ROADMAP v0.10.2

**Última actualización**: 2026-03-30
**Versión actual**: v0.10.1 ✅ GPU INSTANCING + ECS
**Próxima versión**: v0.10.2 - INVERSIÓN DE CONTROL
**Versión futura**: v0.11.0 - RYDITMODULE TRAIT + PARSER OPTIMIZADO

---

## 📊 ESTADO ACTUAL (v0.10.1)

### ✅ Completado en v0.10.1
- [x] **GPU Instancing** - 100K+ partículas (gl-rs, shaders GLSL)
- [x] **ECS** - Entity Component System (bevy_ecs inspired)
- [x] **ECS Renderer** - Integración ECS + rlgl
- [x] **Demos** - ecs_demo_10k, gpu_demo_100k
- [x] **Git Push** - main actualizado (a897e3b)
- [x] **Google Drive Sync** - alucard18:RyDit_Backup

### ⚠️ PENDIENTES CRÍTICOS (v0.10.2 - Inversión de Control)

| Tarea | Complejidad | Riesgo | Tiempo | Prioridad |
|-------|-------------|--------|--------|-----------|
| **1. Quitar límites parser** | 🟢 Baja | 🟢 Bajo | 1 hora | 🔴 CRÍTICA |
| **2. Compilar scene_runner** | 🟡 Media | 🟡 Medio | 2 horas | 🔴 CRÍTICA |
| **3. Activar RyditModule** | 🟡 Media | 🟡 Medio | 1-2 días | 🟡 ALTA |
| **4. Optimizar parser** | 🔴 Alta | 🔴 Alto | 3-5 días | 🟢 MEDIA |

---

## 🎯 FASE 1: QUITAR LÍMITES PARSER (1 hora) 🟢

### Problema
```rust
// main.rs línea 173
while iterations < 100 {  // ⚠️ Límite artificial
    // Game loop truncado a 100 frames
}

// main.rs línea 4181
while iterations < 10 {  // ⚠️ MiGui limitado
    // Solo 10 iteraciones
}
```

### Solución
```rust
// CAMBIAR A:
loop {
    // Game loop sin límites
    // Romper con condición real (ESC, muerte, etc.)
}
```

### Archivos a modificar
- `crates/rydit-rs/src/main.rs` (2 líneas)

### Test
```bash
./target/release/rydit-rs --gfx demos/test_loop_infinito.rydit
# Debe correr >100 frames sin truncar
```

---

## 🎯 FASE 2: COMPILAR SCENE_RUNNER (2 horas) 🟡

### Problema de Compilación

**Error actual**:
```
error[E0425]: cannot find function `evaluar_expr_gfx` in the crate root
   --> crates/rydit-rs/src/modules/particles.rs:31:29
    |
31  | let nombre_val = crate::evaluar_expr_gfx(&args[0], ...);
    |                             ^^^^^^^^^^^^^ not found
```

**Causa raíz**:
- `lib.rs` no exporta funciones de `main.rs`
- Módulos legacy dependen del evaluator
- `scene_runner` no necesita módulos legacy

### Solución (2 enfoques)

#### **Enfoque A: Minimalista (Recomendado)**
```rust
// crates/rydit-rs/src/lib.rs
// SOLO exportar config_parser para scene_runner
pub mod config_parser;
pub use rydit_gfx;
pub use rydit_ecs;
// NO exportar módulos legacy
```

```rust
// crates/rydit-rs/src/bin/scene_runner.rs
use rydit_gfx::{RyditGfx, ColorRydit, Key};
use rydit_ecs::EcsWorld;
use rydit_gfx::ecs_render::EcsRenderer;
use rydit_rs::config_parser::ConfigParser;  // ✅ Solo esto
```

#### **Enfoque B: Exportar todo**
```rust
// crates/rydit-rs/src/lib.rs
pub mod config_parser;
pub mod modules;

// Re-exportar desde main
pub use crate::main::evaluar_expr_gfx;
pub use crate::main::InputEstado;
```

**Riesgo**: Puede romper compilación de main.rs

### Test
```bash
cargo build --release --bin scene_runner
./target/release/scene_runner demos/nivel_config.rydit
```

---

## 🎯 FASE 3: ACTIVAR RYDITMODULE TRAIT (1-2 días) 🟡

### Estado Actual
✅ **Trait existe** en `crates/rydit-core/src/lib.rs`:
```rust
pub trait RyditModule: Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn register(&self) -> HashMap<&'static str, &'static str>;
    fn execute(&self, command: &str, params: Value) -> ModuleResult;
    fn metadata(&self) -> ModuleMetadata;
    fn on_reload(&mut self) {}
    fn on_unload(&mut self) {}
}
```

✅ **ModuleRegistry existe**:
```rust
pub struct ModuleRegistry {
    modules: HashMap<String, Box<dyn RyditModule>>,
}
```

### Lo que falta
- [ ] Implementar en 1 crate piloto (rydit-physics)
- [ ] Registrar en main.rs
- [ ] Llamar desde evaluator

### Plan
```rust
// crates/rydit-physics/src/lib.rs
pub struct PhysicsModule;

impl RyditModule for PhysicsModule {
    fn name(&self) -> &'static str { "physics" }
    fn version(&self) -> &'static str { "0.9.3" }
    
    fn register(&self) -> HashMap<&'static str, &'static str> {
        let mut cmds = HashMap::new();
        cmds.insert("apply_gravity", "Aplicar gravedad");
        cmds.insert("resolve_collision", "Resolver colisión");
        cmds
    }
    
    fn execute(&self, cmd: &str, params: Value) -> ModuleResult {
        match cmd {
            "apply_gravity" => { /* ... */ },
            _ => Err(ModuleError { ... }),
        }
    }
}
```

---

## 🎯 FASE 4: OPTIMIZAR PARSER (3-5 días) 🔴

### Problemas Actuales
1. **Copias de String** - Lexer copia cada token
2. **Sin AST caching** - Reparsea cada frame
3. **Evaluator lento** - 5 capas de overhead

### Soluciones Propuestas

#### **4.1: Lexer sin copias (1 día)**
```rust
// ANTES:
pub enum Token {
    Ident(String),  // Copia
    Texto(String),  // Copia
}

// DESPUÉS:
pub enum Token<'a> {
    Ident(&'a str),  // Referencia
    Texto(&'a str),  // Referencia
}
```

#### **4.2: AST Caching (1 día)**
```rust
// Cache de programas parseados
static AST_CACHE: Lazy<Mutex<HashMap<String, Program>>> = ...;

pub fn parse_cached(source: &str) -> Program {
    let hash = hash(source);
    if let Some(prog) = AST_CACHE.lock().get(&hash) {
        return prog.clone();
    }
    // Parsear y cachear
}
```

#### **4.3: Bytecode (opcional, 2-3 días)**
```rust
pub enum OpCode {
    PushNum(f64),
    PushText(String),
    Call { name: String, args: u8 },
    JumpIfFalse(usize),
    // ...
}

// Compilar AST → bytecode una vez
// Ejecutar bytecode es más rápido
```

---

## 📋 ORDEN DE EJECUCIÓN RECOMENDADO

| Fase | Tarea | Tiempo | Riesgo | Decisión |
|------|-------|--------|--------|----------|
| **1** | Quitar límites parser | 1 hora | 🟢 Bajo | ✅ HACER AHORA |
| **2** | Compilar scene_runner | 2 horas | 🟡 Medio | ✅ HACER DESPUÉS |
| **3** | Activar RyditModule | 1-2 días | 🟡 Medio | ⏸️ ESPERAR |
| **4** | Optimizar parser | 3-5 días | 🔴 Alto | ⏸️ FUTURO |

---

## 🔥 PRÓXIMOS PASOS INMEDIATOS

### AHORA (Fase 1):
```bash
# 1. Quitar límites
sed -i 's/while iterations < 100/loop/' crates/rydit-rs/src/main.rs
sed -i 's/while iterations < 10/loop/' crates/rydit-rs/src/main.rs

# 2. Compilar
cargo build --release

# 3. Test
./target/release/rydit-rs --gfx demos/test_loop.rydit
```

### DESPUÉS (Fase 2):
```bash
# 1. Fix lib.rs (solo config_parser)
# 2. Compilar scene_runner
cargo build --release --bin scene_runner

# 3. Test
./target/release/scene_runner demos/nivel_config.rydit
```

---

## 📊 RESUMEN DE COMPLEJIDAD

```
FASE 1: Quitar límites     ████████░░ 80% segura (2 líneas)
FASE 2: scene_runner       ██████░░░░ 60% segura (imports)
FASE 3: RyditModule        ████░░░░░░ 40% segura (implementar)
FASE 4: Parser opt         ██░░░░░░░░ 20% segura (refactor mayor)
```

---

<div align="center">

**🛡️ RyDit v0.10.2 - PLAN DE ACCIÓN**

*Fase 1: 1 hora | Fase 2: 2 horas | Fase 3: 1-2 días | Fase 4: 3-5 días*

**Empezar por Fase 1 (más fácil y segura)**

</div>
