# 🛡️ RyDit Engine - ROADMAP COMPLETO v0.10.2 - v0.11.0

**Última actualización**: 2026-03-30
**Versión actual**: v0.10.2 ✅ INVERSIÓN DE CONTROL + AST CACHING
**Commit**: `209069e`

---

## 📊 ESTADO ACTUAL (v0.10.2)

### ✅ Completado en v0.10.2
- [x] **GPU Instancing** - 100K+ partículas (gl-rs, shaders GLSL)
- [x] **ECS** - Entity Component System (bevy-inspired)
- [x] **ECS Renderer** - Integración ECS + rlgl
- [x] **AST Caching** - 10x más rápido (parse_cached)
- [x] **Límites Parser** - Removidos (game loops infinitos)
- [x] **scene_runner** - Inversión de Control (326KB)
- [x] **Config Parser** - .rydit como configuración
- [x] **Demos** - ecs_demo_10k, gpu_demo_100k
- [x] **Git Push** - main actualizado
- [x] **Google Drive Sync** - alucard18:RyDit_Backup

---

## 🔥 TAREAS PENDIENTES (ORDENADAS POR PRIORIDAD)

### **PRIORIDAD 🔴 CRÍTICA** (Esta semana - v0.10.3)

| # | Tarea | Tiempo | Riesgo | Impacto | Estado |
|---|-------|--------|--------|---------|--------|
| **1** | **Fixear rydit-rs binario** | 2-3 horas | 🟡 Medio | Alto | ⏸️ Pendiente |
| **2** | **Reescribir particles.rs** | 4-6 horas | 🟡 Medio | Medio | ⏸️ Pendiente |
| **3** | **Testear demos en Termux-X11** | 2-3 horas | 🟢 Bajo | Alto | ⏸️ Pendiente |

---

### **PRIORIDAD 🟡 ALTA** (Próxima semana - v0.10.4)

| # | Tarea | Tiempo | Riesgo | Impacto | Estado |
|---|-------|--------|--------|---------|--------|
| **4** | **Activar RyditModule trait** | 1-2 días | 🟡 Medio | Alto | ⏸️ Pendiente |
| **5** | **Reorganizar repositorio GitHub** | 3-4 horas | 🟢 Bajo | Medio | ⏸️ Pendiente |
| **6** | **Documentar migración legacy → scene** | 2-3 horas | 🟢 Bajo | Medio | ⏸️ Pendiente |

---

### **PRIORIDAD 🟢 MEDIA** (v0.10.5 - v0.11.0)

| # | Tarea | Tiempo | Riesgo | Impacto | Estado |
|---|-------|--------|--------|---------|--------|
| **7** | **Parser óptimo (zero-copy)** | 1-2 días | 🔴 Alto | Alto | ⏸️ Futuro |
| **8** | **Bytecode compilation** | 2-3 días | 🔴 Alto | Medio | ⏸️ Futuro |
| **9** | **AST Cache con TTL** | 4-6 horas | 🟡 Medio | Bajo | ⏸️ Futuro |
| **10** | **Testear caching en producción** | 1-2 horas | 🟢 Bajo | Medio | ⏸️ Pendiente |

---

## 📋 DETALLE DE TAREAS

### **1. Fixear rydit-rs binario** (2-3 horas) 🔴

**Problema**: 64 errores de compilación

**Causa**: Módulos comentados (level, tilemap, collision, window) todavía se usan en:
- `main.rs` - Funciones de módulos
- `executor.rs` - Inicialización
- `eval/mod.rs` - Registro de funciones

**Solución**:
```bash
# 1. Buscar referencias
grep -rn "modules::level" crates/rydit-rs/src/
grep -rn "modules::tilemap" crates/rydit-rs/src/
grep -rn "modules::collision" crates/rydit-rs/src/
grep -rn "modules::window" crates/rydit-rs/src/

# 2. Comentar cada referencia
# 3. Compilar
cargo build --release --bin rydit-rs
```

**Archivos afectados**: ~10 archivos

---

### **2. Reescribir particles.rs** (4-6 horas) 🔴

**Problema**: particles_module.rs en `disabled/` depende de `eval::evaluar_expr_gfx`

**Solución**:
```rust
// particles_module.rs - Versión nueva
use rydit_gfx::particles::{ParticleEmitter, ParticleSystem};

// NO usar eval::evaluar_expr_gfx
// Usar directamente funciones de rydit_gfx

pub fn create_emitter(name: &str, x: f32, y: f32, rate: f32) {
    // Implementación directa sin evaluator
}
```

**Archivos afectados**: 
- `crates/rydit-rs/src/bin/particles_module.rs` (reescribir)
- `crates/rydit-gfx/src/particles.rs` (quizás actualizar)

---

### **3. Testear demos en Termux-X11** (2-3 horas) 🔴

**Objetivo**: Verificar que todas las demos funcionan en producción

**Demos a testear**:
```bash
# ECS Demo
./target/release/ecs_demo_10k

# GPU Demo
./target/release/gpu_demo_100k

# Scene Runner
./target/release/scene_runner demos/nivel_config.rydit

# Legacy (si se fixea)
./target/release/rydit-rs --gfx demos/test_loop.rydit
```

**Verificar**:
- ✅ 60 FPS estables
- ✅ Sin crashes
- ✅ Input funciona (W,A,S,D, ESC)
- ✅ AST caching activo (debería decir "cached")

---

### **4. Activar RyditModule trait** (1-2 días) 🟡

**Estado actual**: Trait existe en `crates/rydit-core/src/lib.rs` pero NO se usa

**Plan**:
```rust
// crates/rydit-physics/src/lib.rs
use rydit_core::{RyditModule, ModuleResult, ModuleMetadata};

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

**Archivos afectados**:
- `crates/rydit-physics/src/lib.rs` (implementar)
- `crates/rydit-core/src/lib.rs` (quizás actualizar)
- `main.rs` (registrar módulo)

---

### **5. Reorganizar repositorio GitHub** (3-4 horas) 🟡

**Problema actual**:
- Muchos archivos en root
- `docs/` mezclado con código
- Sin estructura clara

**Nueva estructura propuesta**:
```
rydit-engine/
├── .github/              # GitHub Actions, templates
├── crates/               # Todos los crates Rust
│   ├── lizer/           # Parser + AST caching
│   ├── rydit-core/      # RyditModule trait
│   ├── rydit-ecs/       # ECS
│   ├── rydit-gfx/       # GPU + ECS renderer
│   └── rydit-rs/        # Binario principal
├── demos/               # Demos .rydit
├── docs/                # Documentación
│   ├── api/            # API docs
│   ├── guides/         # Guías
│   └── roadmap/        # Roadmaps
├── scripts/            # Scripts de build/test
├── tests/              # Integration tests
└── README.md
```

**Acciones**:
1. Mover archivos a carpetas correctas
2. Actualizar CI/CD
3. Actualizar README con nueva estructura

---

### **6. Documentar migración legacy → scene** (2-3 horas) 🟡

**Objetivo**: Guía para migrar de .rydit legacy a scene_runner

**Contenido**:
```markdown
# Guía de Migración: Legacy → Scene Runner

## Antes (Legacy)
```rydit
ryda frame < 10000 {
    dibujar.circulo(x, y, radio, "rojo")
    # Lógica compleja aquí
}
```

## Después (Scene Runner)
```rust
// mi_juego.rs
use rydit_ecs::EcsWorld;
use rydit_gfx::ecs_render::EcsRenderer;

fn main() {
    let mut world = EcsWorld::new();
    let mut renderer = EcsRenderer::new();
    
    // Game loop nativo
    while !gfx.should_close() {
        world.update(0.016);
        renderer.render(&world);
    }
}
```

## Config .rydit (opcional)
```rydit
entidad "jugador" {
    tipo: "player"
    x: 100
    y: 200
}
```
```

---

### **7. Parser óptimo (zero-copy)** (1-2 días) 🟢

**Problema**: Token enum con Strings (copias)

**Solución**:
```rust
// ANTES:
pub enum Token {
    Ident(String),  // ❌ Copia
}

// DESPUÉS:
pub enum Token<'a> {
    Ident(&'a str),  // ✅ Referencia
}
```

**Impacto**: 2-3x más rápido, menos allocaciones

**Riesgo**: Cambia TODAS las signatures (alto)

---

### **8. Bytecode compilation** (2-3 días) 🟢

**Idea**: Compilar AST → bytecode una vez, ejecutar bytecode muchas veces

```rust
pub enum OpCode {
    PushNum(f64),
    PushText(String),
    Call { name: String, args: u8 },
    JumpIfFalse(usize),
}

// Compilar una vez
let bytecode = compile_to_bytecode(&ast);

// Ejecutar muchas veces (game loop)
execute_bytecode(&bytecode);
```

**Impacto**: 5-10x más rápido para lógica compleja

---

## 📈 TIMELINE ESTIMADO

| Semana | Versión | Tareas | Estado |
|--------|---------|--------|--------|
| **Semana 1** | v0.10.3 | Fix rydit-rs, Reescribir particles, Testear demos | ⏸️ Pendiente |
| **Semana 2** | v0.10.4 | RyditModule, Reorganizar repo, Documentar migración | ⏸️ Futuro |
| **Semana 3-4** | v0.10.5 | Parser zero-copy, Bytecode | ⏸️ Futuro |
| **Semana 5** | v0.11.0 | AST Cache TTL, Testing producción | ⏸️ Futuro |

---

## 🎯 RESUMEN DE TAREAS

| Prioridad | Cantidad | Tiempo Total |
|-----------|----------|--------------|
| 🔴 Crítica | 3 tareas | 8-12 horas |
| 🟡 Alta | 3 tareas | 2-3 días |
| 🟢 Media | 4 tareas | 4-6 días |
| **TOTAL** | **10 tareas** | **~2 semanas** |

---

## 🛡️ MÉTRICAS DE PROGRESO

| Métrica | Valor |
|---------|-------|
| Tareas completadas (v0.10.2) | 10+ |
| Tareas pendientes | 10 |
| Tiempo estimado total | ~2 semanas |
| Riesgo promedio | 🟡 Medio |
| Impacto promedio | 🟡 Alto |

---

<div align="center">

**🛡️ RyDit v0.10.2 - ROADMAP COMPLETO**

*10 tareas pendientes | ~2 semanas | Prioridad: Fix rydit-rs + Testear demos*

</div>
