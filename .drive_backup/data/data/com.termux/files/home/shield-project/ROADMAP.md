# 🛡️ RyDit Engine - ROADMAP v0.10.0

**Última actualización**: 2026-03-30
**Versión actual**: v0.9.4 ✅ ENTITY SYSTEM COMPLETADO
**Próxima versión**: v0.10.0 - GPU INSTANCING + SHADERS
**Versión futura**: v0.10.2 - 🛡️ INVERSIÓN DE CONTROL (Core manda, Script configura)

---

## 📊 ESTADO ACTUAL (v0.9.4)

### ✅ Completado en v0.9.4
- [x] **Level Manager** - 13 funciones (load, transition, checkpoints)
- [x] **Tilemap System** - 12 funciones (create, fill_rect, draw)
- [x] **Collision System** - 13 funciones (AABB, Area2D, resolve)
- [x] **Window Manager** - 17 funciones (title, fullscreen, vsync, fps)
- [x] **Entity System** - 50+ funciones (player, enemy, boss, trap, coin)
- [x] **Cámara 2D** - 15 funciones (follow, zoom, scroll)
- [x] **Physics 2D** - 20 funciones (gravedad, fricción, colisión)
- [x] **0 warnings** clippy
- [x] **260+ tests** passing
- [x] **Demo Platformer** - platformer_v094.rydit

### 🛡️ ARQUITECTURA v0.10.2: INVERSIÓN DE CONTROL

**Problema detectado**:
- ⚠️ Script manda sobre Core (arquitectura incorrecta)
- ⚠️ Parser sobrecargado (3K líneas haciendo trabajo de core)
- ⚠️ `main.rs` solo 4K líneas (muy poco para ser core)
- ⚠️ 2000 partículas colapsan el evaluator

**Comparativa con motores**:

| Motor | Core | Scripting | Ratio |
|-------|------|-----------|-------|
| **Godot** | ~500K | GDScript | 10:1 |
| **Unity** | ~1M+ | C# | 5:1 |
| **Unreal** | ~5M+ | C++/BP | 50:1 |
| **RyDit** | ~4K | .rydit | **1.3:1** ⚠️ |

**Solución v0.10.0**:
- 🛡️ Core manda (rydit-rs hace game loop nativo)
- 🛡️ Script configura (.rydit solo parámetros)
- 🛡️ GPU Instancing (100K+ partículas)
- 🛡️ ECS Entt (100K+ entidades)
- 🛡️ Shaders GLSL nativos
- 🛡️ Comando nativo de RyDit: `./rydit-rs --scene <nombre>`

### 🔥 En Proceso (v0.10.0)
| Ítem | Estado | Prioridad | Impacto | Ubicación |
|------|--------|-----------|---------|-----------|
| **Inversión de Control** | 🛡️ Planeado | 🔴 CRÍTICA | Arquitectura correcta | `executor.rs`, `main.rs` |
| **GPU Instancing** | # en proceso | 🔴 CRÍTICA | 100K+ partículas | `rydit-gfx/src/gpu_instancing.rs` |
| **Shaders GLSL** | # en proceso | 🔴 CRÍTICA | GPU rendering | `rydit-gfx/shaders/` |
| **FFI OpenGL** | # en proceso | 🔴 CRÍTICA | gl-rs crate | `rydit-gfx/Cargo.toml` |
| **ECS (ENTT)** | # en proceso | 🟡 ALTA | 100K+ entities | `crates/rydit-ecs/` (nuevo) |

### ⚠️ Futuro (post v0.10.0)
| Ítem | Estado | Prioridad | Impacto |
|------|--------|-----------|---------|
| **N-Body Gravity** | ⚠️ Pendiente | Media | Simulaciones cósmicas |
| **Fluid Dynamics** | ⚠️ Pendiente | Media | Éxodo 14, Jesús aguas |
| **Parser Maduro** | ⚠️ Pendiente | Baja | Expresiones complejas |

---

## 🎯 ROADMAP EVOLUTIVO

### v0.9.0 - 3 Capas Críticas ✅ COMPLETADO

**Fecha**: 2026-03-28

**Features**:
- ✅ Command Queue (8192+ draw calls)
- ✅ Double Buffering (front/back)
- ✅ Platform Sync (XFlush/XSync)
- ✅ 0 warnings clippy
- ✅ 500+ frames verificados

**Archivos clave**:
- `crates/rydit-gfx/src/render_queue.rs` (540 líneas)
- `crates/rydit-gfx/examples/demo_render_queue.rs` (200 líneas)
- `docs/3_CAPAS_CRITICAS_V0.9.0.md`

**Rendimiento**:
- 1000 partículas @ 60 FPS (límite práctico)
- 8192+ draw calls acumulados
- 1 begin_draw() por frame

---

### v0.10.0 - GPU Instancing + Shaders GLSL # en proceso

**Fecha**: 2026-03-29 (en progreso)

**Features**:
- [ ] # en proceso: FFI OpenGL (`gl-rs` crate) en `rydit-gfx/Cargo.toml`
- [ ] # en proceso: Shaders GLSL vertex + fragment en `rydit-gfx/shaders/`
- [ ] # en proceso: `glDrawArraysInstanced()` básico
- [ ] # en proceso: Demo: 10,000+ partículas @ 60 FPS
- [ ] # en proceso: FFI OpenGL seguro

**Ubicación**: `crates/rydit-gfx/src/gpu_instancing.rs`

**Riesgos**:
- ⚠️ Requiere `unsafe` massivo
- ⚠️ Duplica código de raylib
- ⚠️ Gestión manual de memoria GPU

**Beneficios**:
- ✅ 100x más partículas (100,000 vs 1000)
- ✅ Shaders GLSL custom
- ✅ Aprendizaje de GPU programming

---

### v0.10.1 - ECS (Entity Component System) # en proceso

**Fecha**: Después de v0.10.0 (3-4 semanas)

**Features**:
- [ ] # en proceso: Crate nuevo: `crates/rydit-ecs/`
- [ ] # en proceso: ENTT o bevy_ecs
- [ ] # en proceso: Components: Position, Velocity, Sprite
- [ ] # en proceso: Systems: Movement, Render, Physics
- [ ] # en proceso: Integración en executor.rs

**Ubicación**: `crates/rydit-ecs/` (crate nuevo)

**Riesgos**:
- ⚠️ Crate nuevo que mantener
- ⚠️ Curva de aprendizaje ECS

**Beneficios**:
- ✅ 100,000+ entities estables
- ✅ Reutilizable
- ✅ Testing independiente

---

### v0.10.2 - Integración GPU + ECS # en proceso

**Fecha**: Después de v0.10.1 (2-3 semanas)

**Features**:
- [ ] # en proceso: executor.rs usa rydit-ecs
- [ ] # en proceso: executor.rs usa GPU Instancing
- [ ] # en proceso: Crear exodo_gpu.rs (100K+ partículas)
- [ ] # en proceso: .rydit llama a exodo_gpu

**Ubicación**: `crates/rydit-rs/src/executor.rs`

**Beneficios**:
- ✅ 100K+ partículas @ 60 FPS
- ✅ .rydit + .rs trabajando juntos
- ✅ Visión hecha realidad

---

### v0.10.3 - N-Body Gravity ⚠️ Pendiente

**Fecha**: Futuro (3-4 semanas)

**Features**:
- [ ] N-body gravity simulation
- [ ] 100,000+ entities estables
- [ ] Integración con ECS + GPU

**Beneficios**:
- ✅ Escenas cósmicas (Génesis 1, Sistema Solar)

---

### v0.10.4 - Fluid Dynamics ⚠️ Pendiente

**Fecha**: Futuro (4-6 semanas)

**Features**:
- [ ] SPH (Smoothed Particle Hydrodynamics)
- [ ] Fluid surface simulation
- [ ] Wave dynamics

**Beneficios**:
- ✅ Éxodo 14 (división de aguas)
- ✅ Jesús caminando sobre aguas

---

### v1.0.0 - Simulador de Escenas Completo 🔮

**Fecha**: Futuro (5-6 semanas)

**Features**:
- [ ] GPU Instancing maduro
- [ ] ECS completo
- [ ] N-Body + Fluid Dynamics
- [ ] Primera escena bíblica: Éxodo 14
- [ ] Multi-plataforma (Linux, Windows, macOS, Android)

**Features**:
- [ ] 10,000+ partículas reales
- [ ] 1 draw call por frame
- [ ] Shaders GLSL custom
- [ ] API unificada
- [ ] Documentación completa

**Riesgos**:
- ⚠️ Requiere FFI OpenGL estable
- ⚠️ Testing en múltiples plataformas

**Beneficios**:
- ✅ 100,000+ partículas posibles
- ✅ GPU-bound (no CPU-bound)
- ✅ Comparable a Python ModernGL

---

### v1.1.0 - ECS (Entity Component System) 🔮

**Fecha**: Futuro (8-10 semanas)

**Ubicación**: `crates/rydit-ecs/` (crate nuevo)

**Features**:
- [ ] Crate separado: `crates/rydit-ecs/`
- [ ] ENTT o bevy_ecs
- [ ] Components: Position, Velocity, Sprite
- [ ] Systems: Movement, Render, Physics
- [ ] Integración en executor.rs

**Riesgos**:
- ⚠️ Crate nuevo que mantener
- ⚠️ Curva de aprendizaje ECS

**Beneficios**:
- ✅ 100,000+ entities estables
- ✅ Reutilizable
- ✅ Testing independiente

---

### v1.1.0 - Parser Maduro 🔮

**Fecha**: Futuro (8-10 semanas)

**Features**:
- [ ] Refactorizar `lizer/src/lib.rs` completo
- [ ] Paréntesis que funcionen SIEMPRE
- [ ] Expresiones complejas sin dolor
- [ ] Arrays multidimensionales reales
- [ ] Comentarios en cualquier posición

**Riesgos**:
- 🔴 Alto (puede romper código existente)

**Beneficios**:
- ✅ Lenguaje más robusto
- ✅ Mejor experiencia de desarrollo
- ✅ Soporte para código complejo

---

## 📈 COMPARATIVA DE RENDIMIENTO

| Versión | Partículas | Draw Calls | FPS | Complejidad |
|---------|------------|------------|-----|-------------|
| **v0.8.x** | 500 | 500 | 30 | Baja |
| **v0.9.0** | 1000 | 1000 | 60 | Media |
| **v0.9.1** | 5000 | 100 | 60 | Alta |
| **v0.9.5** | 5000 | 10 | 60 | Alta |
| **v1.0.0** | 10,000+ | 1 | 60 | Muy Alta |

---

## 🎯 DECISIONES CRÍTICAS

### ¿GPU Instancing ahora o después?

**AHORA (v0.9.1-v1.0.0)**:
- ✅ Para demos masivos de partículas
- ✅ Aprendizaje de GPU programming
- ⚠️ Requiere 4-6 semanas adicionales

**DESPUÉS (post v1.0.0)**:
- ✅ Render Queue es SUFICIENTE para 90% de casos
- ✅ Enfocarse en otras features
- ⚠️ Límite de 1000 partículas

### ¿FFI OpenGL o wgpu?

**FFI OpenGL (recomendado)**:
- ✅ Mantiene compatibilidad con raylib
- ⚠️ Requiere unsafe
- ✅ Más control directo

**wgpu (alternativa)**:
- ✅ API moderna (Vulkan/Metal/DX12)
- ❌ Cambia arquitectura completa
- ❌ Pierde ventajas de raylib

---

## 📊 MÉTRICAS DE ÉXITO

### v0.9.0 ✅
- [x] 8192+ draw calls
- [x] 0 warnings clippy
- [x] 500+ frames verificados
- [x] 60 FPS estables

### v0.9.1 (Objetivo)
- [ ] 5000 partículas @ 60 FPS
- [ ] Shaders GLSL funcionando
- [ ] Demo de partículas masivas

### v1.0.0 (Objetivo)
- [ ] 10,000+ partículas @ 60 FPS
- [ ] 1 draw call por frame
- [ ] API unificada

---

## 🔗 REFERENCIAS

### Documentos
- [3_CAPAS_CRITICAS_V0.9.0.md](docs/3_CAPAS_CRITICAS_V0.9.0.md)
- [PANORAMA_GPU_INSTANCING_V0.9.x.md](docs/PANORAMA_GPU_INSTANCING_V0.9.x.md)
- [VERIFICACION_PRODUCCION_V0.9.0.md](docs/VERIFICACION_PRODUCCION_V0.9.0.md)

### Código
- `crates/rydit-gfx/src/render_queue.rs`
- `crates/rydit-gfx/examples/demo_render_queue.rs`

### Externas
- [Learn OpenGL - Instancing](https://learnopengl.com/Advanced-OpenGL/Instancing)
- [gl-rs crate](https://github.com/bjz/gl-rs)
- [wgpu](https://github.com/gfx-rs/wgpu)

---

<div align="center">

**🛡️ RyDit Engine - ROADMAP v0.9.0**

*v0.9.0 ✅ | v0.9.1 🔜 | v1.0.0 🔮*

**Próxima sesión: v0.9.1 - GPU Particles (FFI) o Optimización**

</div>
