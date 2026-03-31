# 🛡️ RyDit - ESTRUCTURA DEL PROYECTO

**Última actualización**: 2026-03-31
**Versión**: v0.10.5 ✅ SDL2 BACKEND DESCUBIERTO
**Commit**: `209069e`
**Estado**: Input funciona en Termux-X11 con SDL2

---

## 🎯 ARQUITECTURA ACTUALIZADA v0.10.5

```
rydit-engine/
├── crates/
│   ├── lizer/              # ✅ Parser + AST Caching (v0.10.2)
│   │   └── src/lib.rs      # parse_cached() con LazyLock
│   │
│   ├── rydit-core/         # ✅ RyditModule trait (existe, sin usar)
│   │   └── src/lib.rs      # Trait + ModuleRegistry
│   │
│   ├── rydit-ecs/          # ✅ ECS (v0.10.0)
│   │   ├── src/
│   │   │   ├── lib.rs      # EcsWorld
│   │   │   ├── components.rs
│   │   │   └── systems.rs
│   │   └── Cargo.toml      # bevy_ecs = "0.15"
│   │
│   ├── rydit-gfx/          # ✅ Backend Dual (v0.10.5)
│   │   ├── src/
│   │   │   ├── lib.rs      # RyditGfx + input_sdl2
│   │   │   ├── input_sdl2.rs      # ✅ Input con eventos SDL2
│   │   │   ├── gpu_instancing.rs  # ✅ 100K partículas
│   │   │   ├── ecs_render.rs      # ✅ ECS + rlgl
│   │   │   ├── render_queue.rs    # ✅ 8192 draw calls
│   │   │   └── shaders/           # ✅ vertex.glsl, fragment.glsl
│   │   └── Cargo.toml      # sdl2 = "0.37" + raylib
│   │
│   ├── rydit-rs/           # ⚠️ Binario principal
│   │   ├── src/
│   │   │   ├── bin/
│   │   │   │   ├── demo_sdl2_puro.rs      # ✅ SDL2 puro (funciona)
│   │   │   │   ├── test_callback_sdl2.rs  # ✅ Test SDL2 (funciona)
│   │   │   │   ├── scene_runner.rs        # ✅ Inversión de Control
│   │   │   │   ├── ecs_demo_10k.rs        # ✅ ECS test
│   │   │   │   └── gpu_demo_100k.rs       # ✅ GPU test
│   │   │   ├── config_parser.rs     # ✅ Parser de configs
│   │   │   ├── lib.rs               # ✅ Mínimo (solo config_parser)
│   │   │   ├── cli.rs               # ✅ Usa parse_cached
│   │   │   ├── executor.rs          # ⚠️ Con módulos comentados
│   │   │   ├── main.rs              # ⚠️ Límites removidos
│   │   │   ├── modules/             # ✅ input_map.rs (657 líneas)
│   │   │   └── disabled/            # 📁 particles_module.rs
│   │   └── Cargo.toml      # sdl2 = "0.37"
│   │
│   └── ... (otros crates: anim, physics, science, etc.)
│
├── demos/                  # ✅ Demos .rydit
│   ├── nivel_config.rydit  # ✅ Formato config (v0.10.2)
│   ├── platformer_v094.rydit
│   └── ...
│
├── docs/                   # ✅ Documentación
│   ├── DIAGNOSTICO_INPUT_TERMUX_X11.md  # ✅ Diagnóstico completo
│   ├── CAMBIOS_V0.10.2_FASE1-2.md
│   ├── TAREAS_PENDIENTES_V0.10.2.md
│   ├── RYDIT_V0.9.4_COMPLETADA.md
│   └── ...
│
├── ROADMAP.md              # ✅ Roadmap completo
├── RESUMEN_V0.10.2.md      # ✅ Resumen ejecutivo
├── QWEN.md                 # ✅ Bitácora técnica (v0.10.5)
└── README.md               # ✅ Estado actual (v0.10.5)
```

---

## 📊 ESTADO POR CRATE

| Crate | Estado | Líneas | Funciones | Notas |
|-------|--------|--------|-----------|-------|
| **lizer** | ✅ 100% | ~3300 | Parser + AST caching | v0.10.2 |
| **rydit-core** | ✅ 100% | ~400 | RyditModule trait | Sin usar |
| **rydit-ecs** | ✅ 100% | ~640 | ECS completo | v0.10.0 |
| **rydit-gfx** | ✅ 100% | ~1700 | Backend Dual + Input SDL2 | v0.10.5 |
| **rydit-rs (scene_runner)** | ✅ 100% | ~326KB | Inversión de Control | v0.10.2 |
| **rydit-rs (legacy)** | ❌ 64 errores | ~4500 | Legacy | Needs fix |
| **rydit-physics** | ✅ 100% | ~690 | Físicas 2D | v0.9.3 |
| **rydit-anim** | ✅ 100% | ~200 | Animaciones | v0.8.5 |
| **rydit-science** | ✅ 100% | ~500 | Ciencia/math | v0.8.7 |

---

## 🔥 TAREAS PENDIENTES POR CRATE

### **lizer/**
- [ ] Parser zero-copy (lifetimes) - 1-2 días
- [ ] Bytecode compilation - 2-3 días
- [ ] AST Cache con TTL - 4-6 horas

### **rydit-core/**
- [ ] Documentar RyditModule trait - 2-3 horas
- [ ] Ejemplo de implementación - 1-2 horas

### **rydit-ecs/**
- [ ] Testear con 100K entidades - 1-2 horas
- [ ] Integrar con GPU instancing - 2-3 horas

### **rydit-gfx/**
- [ ] Optimizar shaders GLSL - 4-6 horas
- [ ] Soporte para texturas reales - 1 día

### **rydit-rs/**
- [ ] **Fixear binario legacy (64 errores)** - 2-3 horas 🔴
- [ ] **Reescribir particles_module** - 4-6 horas 🔴
- [ ] **Testear demos en Termux-X11** - 2-3 horas 🔴
- [ ] Activar RyditModule en crates - 1-2 días 🟡
- [ ] Documentar migración legacy → scene - 2-3 horas 🟡

---

## 📋 FLUJO DE DATOS ACTUAL

### **Inversión de Control (v0.10.2)**:

```
┌─────────────────────────────────────┐
│  .rydit (configuración)             │
│    entidad "jugador" { x: 100 }     │
└─────────────────────────────────────┘
            ↓ parse_cached()
┌─────────────────────────────────────┐
│  scene_runner (Rust Core)           │
│    - Parsea config (1 vez)          │
│    - Spawnea entidades (ECS)        │
│    - Game loop nativo (sin eval)    │
└─────────────────────────────────────┘
            ↓ ECS update()
┌─────────────────────────────────────┐
│  rydit-ecs                          │
│    - Components (Position, etc.)    │
│    - Systems (movement, gravity)    │
└─────────────────────────────────────┘
            ↓ ECS render()
┌─────────────────────────────────────┐
│  rydit-gfx (ecs_render.rs)          │
│    - rlgl para draw calls           │
│    - GPU instancing (100K+)         │
└─────────────────────────────────────┘
            ↓ OpenGL
┌─────────────────────────────────────┐
│  Termux-X11 / Raylib                │
│    - Render final en pantalla       │
└─────────────────────────────────────┘
```

### **Legacy (v0.9.x - ROTO)**:

```
┌─────────────────────────────────────┐
│  .rydit (script pesado)             │
│    ryda frame < 10000 {             │
│      dibujar.circulo(...)           │
│      # Lógica compleja aquí         │
│    }                                │
└─────────────────────────────────────┘
            ↓ parse() CADA FRAME
┌─────────────────────────────────────┐
│  main.rs (evaluator)                │
│    - evaluar_expr() 60 veces/seg    │
│    - 5 capas de overhead            │
│    - Límite 100 iteraciones         │
└─────────────────────────────────────┘
            ↓
┌─────────────────────────────────────┐
│  modules/ (particles, entity, etc.) │
│    - Dependen de eval::             │
│    - 64 errores actualmente         │
└─────────────────────────────────────┘
```

---

## 🎯 COMPARATIVA: LEGACY vs SCENE RUNNER

| Aspecto | Legacy (.rydit) | Scene Runner (Rust) |
|---------|-----------------|---------------------|
| **Parseo** | Cada frame (lento) | 1 vez + cache (rápido) |
| **Overhead** | 5 capas | 1 capa (ECS) |
| **Límites** | 100 iteraciones | Infinitas |
| **Partículas** | 500 @ 15 FPS | 100K @ 60 FPS |
| **Entidades** | ~100 | ~100K |
| **FPS** | 15-30 | 60 estables |
| **Uso CPU** | Alto | Bajo |
| **Recomendado** | ❌ No (legacy) | ✅ Sí (futuro) |

---

## 📈 MÉTRICAS TOTALES

| Métrica | Valor |
|---------|-------|
| **Total líneas Rust** | ~25,000+ |
| **Crates activos** | 13 |
| **Binarios compilados** | 4 (scene, ecs, gpu, lizer) |
| **Binarios rotos** | 1 (rydit-rs legacy) |
| **Funciones totales** | 170+ |
| **Tests passing** | 260+ |
| **Warnings** | 1 (lib.rs) |
| **Errores** | 64 (legacy, no crítico) |

---

## 🛡️ PRÓXIMOS PASOS POR SEMANA

### **Semana 1 (v0.10.3)**:
```bash
# 1. Fixear rydit-rs binario (2-3 horas)
grep -rn "modules::level" crates/rydit-rs/src/
# Comentar referencias

# 2. Reescribir particles.rs (4-6 horas)
# Usar rydit_gfx::particles directo

# 3. Testear demos (2-3 horas)
./target/release/scene_runner demos/nivel_config.rydit
```

### **Semana 2 (v0.10.4)**:
```bash
# 4. Activar RyditModule (1-2 días)
# Implementar en rydit-physics

# 5. Reorganizar repo (3-4 horas)
# Mover archivos a carpetas correctas

# 6. Documentar migración (2-3 horas)
# Crear guía legacy → scene
```

---

## 📚 DOCUMENTACIÓN DISPONIBLE

| Documento | Ubicación | Descripción |
|-----------|-----------|-------------|
| **ROADMAP** | `ROADMAP.md` | 10 tareas pendientes |
| **RESUMEN** | `RESUMEN_V0.10.2.md` | Resumen ejecutivo |
| **CAMBIOS** | `docs/CAMBIOS_V0.10.2_FASE1-2.md` | Cambios técnicos |
| **TAREAS** | `docs/TAREAS_PENDIENTES_V0.10.2.md` | Tareas detalladas |
| **BITÁCORA** | `QWEN.md` | Bitácora técnica |

---

<div align="center">

**🛡️ RyDit v0.10.2 - ESTRUCTURA COMPLETA**

*10 tareas pendientes | ~2 semanas | Prioridad: Fix rydit-rs + Testear demos*

**Inversión de Control ✅ | AST Caching ✅ | GPU Instancing ✅ | ECS ✅**

</div>
