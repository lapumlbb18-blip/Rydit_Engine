# 🛡️ Ry-Dit - Tareas Completadas y Pendientes

**Última actualización**: 2026-04-16
**Versión actual**: v0.21.0 ✅ Tipado Fuerte Assets + Higiene Módulos + 5 READMEs
**Próxima versión**: v0.22.0 — Asset Pipeline (Integración) + Editor Visual (Prototipo) + Hot Reload Real
**Análisis estratégico**: Ver `TASKS_2.md`

---

## 📊 RESUMEN RÁPIDO

| Métrica | Valor |
|---------|-------|
| **Crates** | 25 |
| **Errores** | 0 |
| **Tests** | ~260 pasando |
| **Higiene de código** | ✅ Renombrado de módulos de partículas (anim, gpu, script) |

---

## ✅ TAREAS COMPLETADAS (v0.15.0 → v0.22.0)

| Versión | Features Clave | Fecha |
|---------|---------------|-------|
| v0.19.3 | Ciencia Avanzada + Radiación + Genética (DNA) | Abr 15 |
| v0.20.0 | postfx-ry + ry-windows + Asset Pipeline Base | Abr 15 |
| v0.21.0 | Tipado Fuerte Assets + Higiene Partículas | Abr 16 |
| v0.22.0 | ryArt + Unificación Input + Asset Pipeline v2 | Abr 17 |
| v0.23.0 | Consolidación Estructural + API Maestra | Abr 17 |

---

## 🔴 TAREAS PENDIENTES — PRIORIDAD ALTA (v1.0.0 - Hacia la Meta)

### ryArt + Editor

| # | Tarea | Esfuerzo | Detalle |
|---|-------|----------|---------|
| 4 | **ryArt Skins Toolkit** | 8h | Estilos procedimentales para botones y HUDs. |
| 5 | **Hot Reload Real** | 6h | Recarga de texturas en runtime sin reiniciar. |
| 6 | **Prototipo Editor Visual** | 20h | Ventana de inspección de entidades con migui. |


---

## 🟡 TAREAS FUTURO (v0.20.0 → v1.0.0)

### LAZOS + Streaming + Comunidad

| # | Tarea | Esfuerzo | Detalle |
|---|-------|----------|---------|
| 12 | **LAZOS Python bridge** | 20-30h | Scripting Python en Ry-Dit |
| 13 | **LAZOS C++ bridge** | 15-20h | Native extensions C++ |
| 14 | **LAZOS C bridge** | 10-15h | Native extensions C |
| 15 | **Stream multiplayer LAN** | 10-15h | ry-stream multiplayer |

### Motor Completo + Comunidad

| # | Tarea | Esfuerzo | Detalle |
|---|-------|----------|---------|
| 16 | **GitHub Actions CI completo** | 6-8h | Linux + Windows + macOS + Android |
| 17 | **SAZ (Shield Archive Format)** | 10-15h | Paquete de proyecto |
| 18 | **Motor estable** | 20-30h | API estable + sin breaking changes |
| 19 | **Documentación completa** | 15-20h | Docs para todos los crates |
| 20 | **Debugger .rydit** | 10-15h | Breakpoints + step-through |
| 21 | **Profiler CPU/GPU** | 8-12h | Profiling integrado |
| 22 | **Export desktop nativo** | 6-8h | Linux + Windows + macOS builds |
| 23 | **Plugin registry** | 8-12h | crates.io integration |

### NO intentar ahora (imposible en Adreno 610)

| Feature | Por qué NO | Alternativa Ry-Dit |
|---------|-----------|-------------------|
| Vulkan | 30-50h, GLES 2.0 basta | Fallback mantenido |
| WGPU/WebGPU | Demasiado para WASM hoy | OpenGL ES primero |
| PBR 3D / Nanite / Lumen | Hardware AAA imposible | Iluminación 2D simple + GPU instancing |
| LOD system | Premature optimization | Después de iluminación |
| Occlusion culling | Complejidad alta | Frustum culling primero |
| ECS puro | ry-ecs eliminado, no volver | Entity system actual + mejoras |

---

## 🔧 TAREAS ESTRUCTURALES — Conexiones y renombrados (v0.19.2)

### Crates huérfanos (existen pero nadie los usa)

| # | Crate | Archivos | Problema | Solución | Esfuerzo |
|---|-------|----------|----------|----------|----------|
| A1 | **ry-god** | 13 .rs | Security framework aislado, nadie lo importa | Importar en rybot como supervisor de seguridad | 4-6h |
| A2 | **ry-script** | 1 .rs | Loader de scripts `.rydit` sin usuarios | Integrar con ry-loader o ry-vm | 2-3h |
| A3 | **ry-system-ry** | 1 .rs | Fuera del workspace Cargo.toml members | Agregar a members o eliminar | 1h |

### Subsystems vacíos en Rybot (wrappers sin lógica real)

| # | Subsystem | Crate real | Qué debe llamar | Esfuerzo |
|---|-----------|-----------|-----------------|----------|
| B1 | PhysicsSubsystem | ry-physics | Projectile, N-body, gravity update | 3-4h |
| B2 | AnimationSubsystem | ry-anim | Disney principles + action_sprite update | 4-6h |
| B3 | ScienceSubsystem | ry-science | Bezier, stats, geometry update | 2-3h |
| B4 | RenderSubsystem | ry-gfx | GPU instancing + FSR + transitions | 6-8h |
| B5 | NetworkSubsystem | ry-stream | WebSocket update + portal | 4-6h |

---

## 🚀 TAREAS FÍSICA AVANZADA + SIMULACIONES (v0.19.3-v0.20.0)

### Partículas con física Newtoniana (ry-anim + ry-gfx)

| # | Feature | Fórmula | Impacto visual | Esfuerzo |
|---|---------|---------|----------------|----------|
| F1 | **Gravitación newtoniana** | F = G·m₁·m₂/r² | Meteoros, órbitas, lluvia de estrellas | 4-6h |
| F2 | **Colisiones elásticas** | p = m·v, conservación momentum | Choque vehículos, bolas de billar | 6-8h |
| F3 | **Arrastre/fricción aire** | F_d = ½·ρ·v²·C_d·A | Paracaídas, hojas cayendo | 3-4h |
| F4 | **Viento/turbulencia** | Vector fuerza variable | Banderas, humo, polvo | 3-4h |
| F5 | **Flocking (cohesión/separación)** | Reglas de Reynolds | Bandadas de pájaros, cardúmenes | 4-6h |

### Efectos visuales de partículas

| # | Feature | Detalle | Esfuerzo |
|---|---------|---------|----------|
| F6 | **Color por velocidad** | Azul=lento → Rojo=rápido | 2-3h |
| F7 | **Size por energía** | Partícula crece con velocidad | 1-2h |
| F8 | **Alpha por vida** | Fade out natural | 1-2h |
| F9 | **Blend aditivo** | Explosiones brillantes | 2-3h |
| F10 | **Trail de partículas** | Estela luminosa (ya existe en ry-anim) | Conectar |

### Simulaciones avanzadas

| # | Simulación | Detalle | Esfuerzo |
|---|-----------|---------|----------|
| F11 | **Meteor shower** | Gravitación + fricción + explosión impacto | 6-8h |
| F12 | **Choque vehículos** | Colisión elástica + deformación + partículas | 8-12h |
| F13 | **Fuego con convección** | Partículas suben + viento térmico | 4-6h |
| F14 | **Agua SPH** | Smoothed Particle Hydrodynamics básico | 10-15h |
| F15 | **Explosión onda choque** | Onda expansiva + escombros | 4-6h |

### Sonido por física (audio reactivo)

| # | Feature | Fórmula | Esfuerzo |
|---|---------|---------|----------|
| F16 | **Frecuencia por impacto** | f ∝ √(energía cinética) | 2-3h |
| F17 | **Volumen por masa×velocidad** | V ∝ m·v² (energía) | 2-3h |
| F18 | **Doppler pitch shift** | f' = f·(v±v_o)/(v±v_s) | 3-4h |
| F19 | **Frecuencias altas→bajas** | Aturdidor → suave con tiempo | 2-3h |

### Texturas + peso físico

| # | Feature | Detalle | Esfuerzo |
|---|---------|---------|----------|
| F20 | **Texturas aumentan masa** | Sprite con textura = más peso en física | 3-4h |
| F21 | **Deformación por impacto** | Mesh se deforma según fuerza | 6-8h |

### Duplicación de código

| # | Conflicto | Ubicaciones | Solución | Esfuerzo |
|---|-----------|-------------|----------|----------|
| C1 | **particles.rs** (3 copias) | ry-anim, ry-gfx, ry-rs/modules | Renombrar: `anim_particles`, `gpu_particles`, `script_particles` | 2-3h |
| C2 | **camera.rs** (2 copias) | ry-gfx, ry-rs/modules | Renombrar: `camera2d`, `script_camera` | 1-2h |
| C3 | **theme.rs** (2 copias) | ry-gfx/toolkit/, toolkit-ry/ | Eliminar duplicado en ry-gfx, usar toolkit-ry | 3-4h |
| C4 | **backend_sdl2.rs** (2 copias) | migui, ry-gfx | Aceptar (propósitos distintos), agregar comentario | 0.5h |
| C5 | **cli.rs** (2 copias) | ry-rs, rybot | Renombrar: `main_cli`, `project_cli` | 1h |

### Input duplicado (events-ry vs ry-input)

| # | Problema | Solución | Esfuerzo |
|---|----------|----------|----------|
| D1 | events-ry tiene InputManager, Shell, TextInput — ry-input tiene InputMap, InputState | Unificar: que events-ry dependa de ry-input, o fusionar crates | 6-8h |

### Re-exports faltantes

| # | Crate | Qué exportar | Donde |
|---|-------|-------------|-------|
| E1 | ry-rs/lib.rs | Solo re-exporta ry_config + ry_gfx | Agregar pub use de rybot, ry-input, ry-anim, ry3d-gfx, toolkit-ry |
| E2 | ry-gfx/lib.rs | ~20 módulos públicos sin re-export de tipos clave | Agregar pub use de ColorRydit, RyditGfx, AudioSystem, Key |
| E3 | ry-rs/modules/mod.rs | Define módulos locales (camera, particles, physics) en vez de re-exportar crates | Reemplazar con re-exports de crates especializados |

---

## 🟡 TAREAS PARALELAS — SDL2 Avanzado + Raylib + main.rs

### SDL2 Avanzado en Editores (sensación profesional, no toy)

| # | Tarea | Filosofía | Esfuerzo |
|---|-------|-----------|----------|
| 1 | **SDL2 avanzado en editores** | Raylib avanzado pintado — editores con sensación profesional, no toy | 10-15h |
| 2 | **SDL2 ≠ Dear ImGui dependencia** | ImGui es fascinante pero SDL2 nunca dependió de él — aprovechar SDL2 puro al máximo | 6-8h |
| 3 | **Raylib single-thread ligero** | Raylib es tan ligero — librería single-thread en Rust — usarlo sin sobrecargar | 4-6h |
| 4 | **SDL2 potencial máximo** | Buscar el máximo de SDL2 sin sobrecargar — eficiencia sobre features | 8-12h |

### main.rs — El Corazón del Motor

| Aspecto | Estado Actual | Meta | Inspiración |
|---------|--------------|------|------------|
| **main.rs líneas** | ~5K líneas | 50K-500K líneas | Motores grandes tienen main de millones o 500K líneas |
| **Qué hacer** | Expandir masivamente | Sistema completo de desarrollo | Godot engine.cpp, Unity core |
| **Filosofía** | Mínimo viable | Motor completo con todo integrado | Los grandes motores crecen desde main |

**Señales claras**:
- main.rs de 5K es diminuto — los grandes engines tienen main de 500K+ líneas
- Hay que expandir main.rs con TODO el sistema integrado
- Rust permite esto sin overhead — la magia de Rust hace posible lo imposible
- Cada línea de main.rs es una capability del motor

---

## 📋 CRATES SIN README (5 pendientes)

| Crate | README | Tests | Publish ready? |
|-------|--------|-------|----------------|
| ry-lexer | ❌ | ⏳ | 🟡 Con 1h |
| ry-parser | ❌ | ⏳ | 🟡 Con 1h |
| events-ry | ❌ | ⏳ | 🟡 Con 1h |
| ry-loader | ❌ | ⏳ | 🟡 Con 1h |
| blast-core | ❌ | ⏳ | 🟡 Con 1h |

---

## 📋 PRÓXIMA SESIÓN — Lo que traerás

### Crates Nuevos (desarrollo en paralelo)
- [ ] **postfx-ry** ✅ v0.1.0 creada — Post-processing + Materiales + Química + Transformación visual
- [ ] **ry-windows** — Ventana unificada configurable por plataforma
- [ ] **ry3d-gfx expansión** — Iluminación 3D, PBR low-end, carga modelos completa

### Asset Pipeline + Editor
- [ ] **Asset pipeline** — Carga automática + compresión + hot reload
- [ ] **Tilemap editor visual** — GUI de tilemap + tileset
- [ ] **Editor visual + por código** — Ambos modos, fusión físicas+animaciones+ciencia

### Estructural (conexiones)
- [ ] **Conectar rybot subsystems** — physics, anim, science, render, network
- [ ] **Renombrar particles.rs** — anim_particles, gpu_particles, script_particles
- [ ] **Resolver theme.rs duplicado** — ry-gfx/toolkit vs toolkit-ry
- [ ] **Re-exports en ry-rs/lib.rs** — rybot, ry-input, ry-anim, ry3d-gfx
- [ ] **Resolver events-ry vs ry-input** — unificar input
- [ ] **5 READMEs faltantes** — ry-lexer, ry-parser, events-ry, ry-loader, blast-core

### LAZOS + Streaming
- [ ] **LAZOS Python+C+++C**
- [ ] **Stream multiplayer LAN**

---

<div align="center">

**🛡️ Ry-Dit v0.19.2 — Tareas Completadas y Pendientes**

*25 crates · ~260 tests · 12 crates.io · 24+ demos · 0 errores*

*3 Pilares: 🎮 Gaming · 🎬 Animaciones+Ciencia · 📡 Streaming+Comunidad*

**Próximo: v0.20.0 — ryfrac-postFX + ry-windows + ry3d-gfx + Asset Pipeline + Editor**

*Ver `TASKS_2.md` para análisis estratégico completo*

</div>
