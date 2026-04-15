# 🛡️ Ry-Dit - Tareas Completadas y Pendientes

**Última actualización**: 2026-04-14
**Versión actual**: v0.19.2 ✅ SDL2 Helpers + Rybot Subsystems + War Spacio + Iluminación 2D + Flexbox
**Próxima versión**: v0.20.0 — ryfrac-postFX + ry-windows + ry3d-gfx + Asset Pipeline + Editor
**Análisis estratégico**: Ver `TASKS_2.md` — Comparativa con Unreal, Unity, Godot, Bevy

---

## 📊 RESUMEN RÁPIDO

| Métrica | Valor |
|---------|-------|
| **Crates** | 25 |
| **Errores** | 0 |
| **Tests** | ~260 pasando |
| **Crates publicados** | 12 |
| **Demos funcionales** | 24+ |
| **Launchers** | 11+ con auto-detección DISPLAY + Zink |

---

## ✅ TAREAS COMPLETADAS (v0.15.0 → v0.19.2)

| Versión | Features Clave | Fecha |
|---------|---------------|-------|
| v0.15.0 | GPU Instancing (50K) + FSR 1.0 | — |
| v0.16.0-alpha | CI 3 plataformas + 6 crates | — |
| v0.16.1 | Snake + Buscaminas + Action Sprite + Tilemap 2.0 | Abr 9 |
| v0.17.0 | Demo Militar + Emoji Atlas + Audio Mixer | Abr 11 |
| v0.18.0 | 3D Primitives + 19 Transiciones + UTF-8 Fix | Abr 11 |
| v0.19.0 | ry-input + rybot + Mesh3D + Skeleton3D + Letras 3D | Abr 12 |
| v0.19.1 | Iluminación 2D + Flexbox + Física Newtoniana | Abr 13 |
| v0.19.2 | SDL2 Helpers + War Spacio + Rybot Subsystems | Abr 13 |
| v0.19.3 | Ciencia Avanzada + Radiación + Genética (DNA) | Abr 15 |

---

## 🔴 TAREAS PENDIENTES — PRIORIDAD ALTA (v0.20.0)

### 3 Crates Nuevos — Desarrollo en Paralelo

| # | Crate | Esfuerzo | Dependencia | Detalle |
|---|-------|----------|-------------|---------|
| 1 | **postfx-ry** (NUEVO) ✅ | 15-20h | ry-gfx, ry-anim, ry-physics | Post-processing (bloom, blur, sharpen, color grade) + Materiales (goma, lava, vidrio, metal) + Química (mezcla, reacción, fusión) + Transformación visual (cortar, mojar, endurecer, reventar, estirar, fusionar). Puente entre física, química y animación visual. **v0.1.0 creada**: 6 efectos GPU, FBO chain, 3 presets (cyberpunk/natural/retro), 2 tests. |
| 2 | **ry-windows** (NUEVO) ✅ | 12-16h | ry-backend, ry-input, migui | Ventana unificada configurable por plataforma (Termux/openbox, Windows, Linux, Mac, Android, iOS). Menús/submenus/controles en pantalla preconfigurables en el buildeo. Fusión SDL2 + Raylib a nivel de ventana/binario. **v0.1.0 creada**: WindowConfig, WindowBuilder, Platform enum (7 plataformas), Backend enum, WindowEvent (15 tipos), InputState, WindowTrait, SDL2 + Raylib implementations, presets (game_2d, editor, demo), 9 tests. |
| 3 | **ry3d-gfx** (EXPANSIÓN) ✅ | 20-30h | ry-gfx, postfx-ry | Desarrollo paralelo continuo: **v0.1.0 → v0.2.0**: OrbitCamera3D (mouse drag + scroll zoom), FpsCamera3D (WASD + mouse look), Skybox procedural (day/sunset/night presets), Primitivas extra (cono, toroide, cápsula, pirámide, heightmap), Iluminación 3D básica (reservado para shaders), 21 tests (de 3 a 21). |

### 🚀 CIENCIA Y FÍSICA FASE 2 COMPLETADAS ✅

| # | Feature | Estado | Crate | Detalle |
|---|---------|--------|-------|---------|
| F1 | L-System Avanzado | ✅ | ry-science | 8 presets + interpretación turtle graphics |
| F2 | Sistema Solar | ✅ | ry-physics | Leyes de Kepler + simulación orbital real |
| F3 | Radiación Atómica | ✅ | ry-physics | Fisión, Fusión, Decaimiento, Geiger rate |
| F4 | Fuego Convección | ✅ | ry-physics | Física térmica unificada en partículas |
| F5 | Mutación Genética | ✅ | ry-science | **DNA system**: mutación por radiación, crossover y fenotipo |

### Asset Pipeline + Editor

| # | Tarea | Esfuerzo | Dependencia | Detalle |
|---|-------|----------|-------------|---------|
| 4 | **Asset pipeline** | 10-15h | ry-windows, ryfrac-postFX | Carga automática de assets + compresión (basis-universal, ktx2) + hot reload. `AssetServer::load()` idiomático tipo Bevy. **Base arquitectónica completada (v0.20.0-alpha)**: Trait AssetProvider, AssetServer con caché, Sdl2/Raylib adaptadores base. |
| 5 | **Tilemap editor visual** | 12-16h | migui, toolkit-ry, asset pipeline | Editor GUI de tilemap + tileset con preview, paint, export CSV. |
| 6 | **Editor visual + por código** | 20-30h | ry-windows, migui, asset pipeline | Editor visual de juegos (separado o 2-in-1). Ambos modos: visual (drag & drop) y por código. Fusión de físicas + animaciones Disney + procesos científicos/biológicos. Sencillo pero potente. |

### Conexiones Estructurales

| # | Tarea | Esfuerzo | Detalle |
|---|-------|----------|---------|
| 7 | **Crates huérfanos** | 4-6h | ry-god → rybot seguridad, ry-script → ry-vm, ry-system-ry → workspace |
| 8 | **Subsystems rybot** | 10-15h | Physics, Animation, Science, Render, Network — conectar lógica real |
| 9 | **Duplicados** | 6-8h | particles.rs (3), camera.rs (2), theme.rs (2), backend_sdl2.rs (2), cli.rs (2) |
| 10 | **Re-exports** | 3-4h | ry-rs/lib.rs, ry-gfx/lib.rs — exponer tipos clave |
| 11 | **5 READMEs faltantes** | 5h | ry-lexer, ry-parser, events-ry, ry-loader, blast-core |

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
