## 🔴 TAREAS PENDIENTES — PRIORIDAD ALTA (v0.19.1)

### Render + Iluminación (Análisis de motores completado)

| # | Tarea | Esfuerzo | Dependencia | Detalle |
|---|-------|----------|-------------|---------|
| 1 | **Iluminación 2D** (luces puntuales + sombras) | 15-20h | ry-gfx | Separa "toy" de "usable" para juegos reales. GLES 2.0 |
| 2 | **Layout Flexbox en migui** | 15-20h | migui | Sin layout auto no hay editor visual ni UI responsive |
| 3 | **Render pipeline abstraction (forward)** | 10-15h | ry-gfx | Flexibilidad tipo Unity SRP adaptada a low-end |
| 4 | **Sistema de materiales GLSL configurables** | 8-12h | #3 Render pipeline | Sin esto cada efecto requiere hardcodear shader |
| 5 | **Post-processing en crate separado (ry-postfx)** | 8-10h | ry-gfx, ry-anim | ry-anim no debería tener bloom/glow/blur |

---

## 🟡 TAREAS PENDIENTES — PRIORIDAD MEDIA (v0.20.0)

### Editor + UI Avanzada + Herramientas

| # | Tarea | Esfuerzo | Dependencia | Detalle |
|---|-------|----------|-------------|---------|
| 6 | **Frustum culling generalizado** | 6-8h | ry-gfx, ry3d-gfx | Solo existe para tilemap ahora |
| 7 | **Hot-reload de shaders** | 6-8h | #4 Materiales | Recarga sin reiniciar |
| 8 | **ry-window unificado (SDL2 + raylib)** | 12-16h | ry-backend, ry-input | API común de ventanas/input |
| 9 | **Theme editor GUI para migui** | 6-8h | migui, toolkit-ry | Editor visual de temas |
| 10 | **Gamepad haptic/rumble** | 3-4h | ry-input | Feedback táctil |

---

## 🟢 TAREAS FUTURO (v0.20.0-v1.0.0)

### Editor + LAZOS + Streaming

| # | Tarea | Esfuerzo | Detalle |
|---|-------|----------|---------|
| 11 | Editor visual separado (o 2-in-1) | 20-30h | Editor visual de juegos |
| 12 | Tilemap editor visual | 12-16h | Editor GUI de tilemap |
| 13 | LAZOS Python bridge | 20-30h | Scripting Python en Ry-Dit |
| 14 | LAZOS C++ bridge | 15-20h | Native extensions C++ |
| 15 | LAZOS C bridge | 10-15h | Native extensions C |
| 16 | Asset pipeline + hot reload | 8-12h | Carga automática |
| 17 | Stream multiplayer LAN | 10-15h | ry-stream multiplayer |

### Motor Completo + Comunidad

| # | Tarea | Esfuerzo | Detalle |
|---|-------|----------|---------|
| 18 | GitHub Actions CI completo | 6-8h | Linux + Windows + macOS + Android |
| 19 | SAZ (Shield Archive Format) | 10-15h | Paquete de proyecto |
| 20 | Motor estable | 20-30h | API estable + sin breaking changes |
| 21 | Documentación completa | 15-20h | Docs para todos los crates |
| 22 | Debugger .rydit | 10-15h | Breakpoints + step-through |
| 23 | Profiler CPU/GPU | 8-12h | Profiling integrado |
| 24 | Export desktop nativo | 6-8h | Linux + Windows + macOS builds |
| 25 | Plugin registry | 8-12h | crates.io integration |

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

- [ ] **Iluminación 2D** — luces puntuales + sombras básicas (GLES 2.0)
- [ ] **Layout Flexbox en migui** — layout automático tipo Bevy UI
- [ ] **Render pipeline abstraction** — forward pass configurable
- [ ] **Materiales GLSL** — shaders configurables sin hardcodear
- [ ] **Post-processing crate** — mover bloom/glow/blur de ry-anim
- [ ] **Frustum culling generalizado** |
- [ ] **Hot-reload shaders** |
- [ ] **ry-window unificado** |
- [ ] **Theme editor GUI** |
- [ ] **Editor visual** |
- [ ] **LAZOS Python+C+++C** |
- [ ] **v1.0 de Ry-Dit** |

---

## 📐 ANÁLISIS COMPARATIVO — Inspiración de motores grandes

> Ver `TASKS_2.md` para el análisis completo comparando Unreal, Unity, Godot 4 y Bevy.

### Lo que adaptaremos (resumen):

| Feature | Inspiración | Versión Ry-Dit |
|---------|-------------|----------------|
| Input Map | Godot | `.rydit-input` archivo simple ✅ |
| Escenas | Godot PackedScene | `.ryscene` texto legible |
| Render Pipeline | Unity SRP | `ry-gfx` features en Cargo.toml |
| Iluminación | Godot Light2D | Luces 2D low-end, no AAA |
| UI Layout | Bevy UI / Godot | migui con Flexbox |
| Audio Mixer | Godot Audio buses | Mixer ligero con spatial 2D ✅ |
| Scene Tree | Godot | Rybot SceneTree ✅ |
| Hot Reload | Unity | Recarga sin reiniciar |
| Plugin System | crates.io | Registry de crates Ry-Dit |

### Lo que NO adaptaremos:

| Feature | Por qué NO | Alternativa Ry-Dit |
|---------|-----------|-------------------|
| ECS puro | ry-ecs eliminado, no volver | Entity system actual + mejoras |
| Nanite/Lumen | Requiere hardware AAA | GPU instancing + iluminación 2D simple |
| Ray Tracing | Hardware imposible en low-end | Sombras 2D con raycasting |
| Vulkan ahora | 30-50h, OpenGL ES 2.0 basta | Fallback mantenido |
| Asset Store | No hay comunidad aún | crates.io es suficiente |

---

<div align="center">

**🛡️ Ry-Dit v0.19.0 — Tareas Completadas y Pendientes**

*25 crates · ~203 tests · 12 crates.io · 21+ demos · 0 errores*

*3 Pilares: 🎮 Gaming · 🎬 Animaciones+Ciencia · 📡 Streaming+Comunidad*

**Próximo: v0.19.1 — Iluminación 2D + Flexbox migui + Render pipeline**

*Ver `TASKS_2.md` para análisis estratégico completo*

</div>
