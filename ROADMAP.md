# Ry-Dit - ROADMAP v0.19.0 → v1.0.0

**Última actualización**: 2026-04-17
**Versión actual**: v0.23.0 ✅ Consolidación Estructural + Puerta Principal ry-rs + Prelude
**Próxima versión**: v1.0.0 — Motor Completo + Editor Visual + LAZOS
**Análisis estratégico**: Ver `TASKS_2.md` — Análisis comparativo con Unreal, Unity, Godot, Bevy

---

## Estado Actual

| Métrica | Valor |
|---------|-------|
| **Crates** | 25 |
| **Líneas Rust** | ~42K+ |
| **Compilación** | 0 errores |
| **Tests** | ~260/260 pasando |
| **Crates publicados** | 12 |
| **Demos funcionales** | 24+ |
| **Launchers** | 11+ con auto-detección DISPLAY + Zink |
| **Repositorio** | `github.com/lapumlbb18-blip/Ry-dit` |

---

## 🧭 VISIÓN ESTRATÉGICA — 3 Pilares de Ry-Dit

Ry-Dit no es solo un motor de juegos. Son **3 capacidades simultáneas**:

### 🎮 Pilar 1: Gaming 2D/3D
- Juegos 2D completos (Snake, Buscaminas, Torreta)
- GPU Instancing (50K partículas a 48 FPS)
- FSR 1.0 nativo, sprite animation, tilemap con texturas
- Camino: sprites reales → iluminación 2D → escenas → editor visual

### 🎬 Pilar 2: Animaciones + Ciencia
- 12 principios Disney (ry-anim)
- 8 simulaciones científicas (ry-science: Bezier, ondas, L-System)
- 6 ilusiones ópticas animadas
- Efectos post-proceso (bloom, glow, blur, morph)
- Camino: GIF → más efectos → simulaciones interactivas

### 📡 Pilar 3: Streaming + Comunidad en Tiempo Real
- ry-stream: LAN streaming con WebSocket
- v-shield: Platform layer + sync primitives
- Servidor WebSocket + portal web
- Camino: multiplayer LAN → streaming en vivo → comunidad

---

## Versiones Completadas

### v0.16.1 — Snake + Buscaminas + Action Sprite + Tilemap 2.0 ✅

**Fecha**: 2026-04-09

| Feature | Estado | Detalle |
|---------|--------|---------|
| Snake Anime v2 | ✅ | WASD, manzanas, bombas, entidades, minimap, cámara follow |
| Buscaminas | ✅ | 16×16, 40 minas, flood fill, banderas, game over/victoria |
| action_sprite module | ✅ | SpriteSheet, AnimationClip, AnimatedSprite, RenderCommand |
| demo_action_sprite | ✅ | Sprite sheet loading + frame animation + state machine |
| Tilemap v2.0 | ✅ | Texturas reales, CSV import/export, camera culling (95% menos) |
| Bordes suaves | ✅ | smoothstep + alpha blending en GPU instancing |
| ry3d-gfx mejorado | ✅ | Texto 3D, modelos GLTF/OBJ/IQM/VOX/MDL |
| ry-config publicado | ✅ | README profesional, 3/3 tests |
| ry-physics publicado | ✅ | README profesional, 6/6 tests |
| ry-science publicado | ✅ | README profesional, 21/21 tests |
| ry-test eliminado | ✅ | Código muerto removido |
| GUIA_USUARIO.md | ✅ | Guía completa de instalación y uso |
| 8 Launchers | ✅ | Auto-detección DISPLAY + Zink |

```
Progreso: ████████████████████ 100%
```

### v0.16.0-alpha — CI 3 plataformas + 6 crates ✅

| Feature | Estado |
|---------|--------|
| v-shield v0.2.0 | ✅ Platform layer + sync primitives |
| ry-gfx v0.10.8 | ✅ GPU Instancing + FSR |
| ry-stream v0.2.0 | ✅ v-shield sync integrado |
| GitHub Actions CI | ✅ Linux + Windows + macOS |
| 65 tests fixeados | ✅ ry-rs lifetimes |

```
Progreso: ████████████████████ 100%
```

### v0.15.0 — GPU Instancing + FSR 1.0 ✅

| Feature | Estado |
|---------|--------|
| 50K partículas GPU | ✅ 48 FPS en Adreno 610 |
| FSR 1.0 | ✅ 960x540 → 1280x720 a 48 FPS |
| 8 demos Termux-X11 | ✅ Funcionales |

```
Progreso: ████████████████████ 100%
```

---

## Versiones Planificadas

### v0.17.0 — Demo Militar + Emoji Atlas + Audio Mixer + Organización ✅

**Fecha**: 2026-04-11

| Feature | Estado | Detalle |
|---------|--------|---------|
| Demo Militar | ✅ | Soldado procedural + partículas + granadas en arco + salto |
| Granadas con trayectoria parabólica | ✅ | Física de gravedad en arco |
| Partículas de disparo/explosión | ✅ | Efectos visuales |
| Salto del soldado | ✅ | Física de salto con input |
| Emoji Atlas UTF-8 | ✅ | TTF_RenderUTF8_Blended fix |
| 25+ emojis procedurales | ✅ | Sprites PNG generados en runtime |
| Atlas de texturas | ✅ | Textura única con grid de emojis |
| Audio Mixer 4 buses | ✅ | Master, SFX, Music, Ambient |
| Spatial 2D audio | ✅ | Volumen/panning por posición |
| Fade in/out | ✅ | Transiciones suaves de volumen |
| docs/ ignorado en git | ✅ | docs/ y launchers/ en .gitignore |

```
Progreso: ████████████████████ 100%
```

### v0.18.0 — 3D Primitives + Transiciones + Audio Mixer + UTF-8 Fix + Emojis ✅

**Fecha**: 2026-04-11

| Feature | Estado | Detalle |
|---------|--------|---------|
| ry3d-gfx v0.1.0 | ✅ | 15 primitivas 3D: cubos, esferas, cilindros, líneas, grid, ejes, bbox |
| demo_3d_primitives | ✅ | Escena 3D interactiva con cámara orbital |
| draw_model / draw_model_ex | ✅ | Stubs arreglados con FFI real |
| DrawHandle3D con lifetime | ✅ | Guarda referencia `&'a Camera3D`, RAII pattern |
| TouchControls | ✅ | Joysticks virtuales + botones táctiles (como RayGunz) |
| 19 transiciones | ✅ | Fade, Slide, Wipe, Zoom, Circle, Blinds, Dissolve, Spiral, etc. |
| TransitionManager | ✅ | Cola de transiciones + auto-advance + easing smoothstep |
| Audio Mixer avanzado | ✅ | 4 buses + spatial 2D + fade in/out |
| UTF-8 Fix | ✅ | TTF_RenderUTF8_Blended → acentos (áéíóú ñ ü) correctos |
| Emoji Atlas | ✅ | 25+ emojis como sprites procedurales PNG |
| FontSystem | ✅ | Múltiples fuentes + fallback automático |
| demo_militar | ✅ | Soldado procedural + partículas + granadas en arco + salto |
| demo_transitions | ✅ | Galería de 19 transiciones tipo editor de video |
| demo_emoji_utf8 | ✅ | Texto UTF-8 + emojis sprites |
| demo_audio_mixer | ✅ | Mixer interactivo con 4 buses |

```
Progreso: ████████████████████ 100%
```

### v0.19.0 — Letras 3D + Escenas (.ryscene) + Input map + Rybot CLI+GUI ✅

**Fecha**: 2026-04-12

| Feature | Estado | Detalle |
|---------|--------|---------|
| `ry-input` crate | ✅ | Input map configurable (.rydit-input parser), InputSource (Key, Mouse, Gamepad, Touch), InputState (pressed/just_pressed/just_released) |
| Macros K!() M!() P!() PA!() | ✅ | Macros ergonómicos para input |
| 16 acciones game + 17 editor defaults | ✅ | Pre-configuradas con rebind |
| 18 tests en ry-input | ✅ | Tests de parsing, input state, macros |
| `rybot` crate | ✅ | Motor central que orquesta todos los crates |
| SceneTree con parse .ryscene | ✅ | Archivos de escena legibles |
| 6 subsistemas | ✅ | Input, Physics, Animation, Science, Render, Network |
| CLI (new, templates, info) | ✅ | Crear proyectos desde terminal |
| GUI con migui (4 paneles) | ✅ | New Project, Inspector, Scene Tree, Engine Stats |
| 33 tests en rybot | ✅ | Tests de orquestación, scene tree, CLI, GUI |
| Letras 3D (draw_text_3d) | ✅ | GetWorldToScreen FFI real en ry3d-gfx |
| draw_text_3d_with_bg | ✅ | Fondo gris para visibilidad |
| demo_text_3d | ✅ | Demo exclusivo para letras 3D con fondo |
| Mesh3D | ✅ | Cubo, esfera, cilindro, plano con GenMesh raylib + UploadMesh + DrawMesh |
| Skeleton3D | ✅ | 22 bones humanoides con Bone3D jerárquico + draw con líneas y esferas |
| Panel visual mejorado | ✅ | migui + toolkit-ry |
| migui mejoras | ✅ | Más widgets + temas |

```
Progreso: ████████████████████ 100%
```

### v0.19.1 — Iluminación 2D + Flexbox + Física Newtoniana ✅

**Fecha**: 2026-04-13

| Feature | Estado | Detalle |
|---------|--------|---------|
| Iluminación 2D | ✅ | Luces direccionales + puntuales + sombras simples |
| Flexbox UI | ✅ | Layout system para migui (auto-sizing, wrapping) |
| Física Newtoniana en game loop | ✅ | Gravedad F=G·m₁·m₂/r² entre cuerpos cada frame |
| rybot subsystems conectados | ✅ | Input, Physics, Animation, Science, Render, Network orquestados |

```
Progreso: ████████████████████ 100%
```

### v0.19.2 — SDL2 Helpers + Rybot Subsystems + Demo War Spacio ✅

**Fecha**: 2026-04-13

| Feature | Estado | Detalle |
|---------|--------|---------|
| SDL2 Helpers | ✅ | `ry_gfx::sdl2_helpers` — velocity_color, blend_additive, newtonian_gravity, audio_procedural |
| Color por velocidad | ✅ | Ramp azul → amarillo → naranja → rojo → blanco |
| Blend aditivo | ✅ | Colores se suman al superponerse (explosiones) |
| Audio reactivo por impacto | ✅ | Procedural: shoot (tono descendente), explosion (noise), powerup (sweep) |
| Demo War Spacio | ✅ | Galaga completo con SDL2 + gravitación + partículas + color velocidad |
| Rybot subsystems conectados | ✅ | 6 subsistemas funcionando en RybotEngine game loop |
| 25 crates | ✅ | ry-input + rybot agregados al workspace |
| ~260 tests | ✅ | 18 ry-input + 33 rybot + existentes |
| 24+ demos | ✅ | demo_war_spacio + existentes |

```
Progreso: ████████████████████ 100%
```

### v0.19.3 — Ciencia Avanzada + Radiación + Genética ✅

**Fecha**: 2026-04-15 (Gemini CLI)

| Feature | Estado | Detalle |
|---------|--------|---------|
| L-System Avanzado | ✅ | 8 presets (binary_tree, fern, coral, dragon, etc.) + Interpretación turtle |
| Sistema Solar (Kepler) | ✅ | Órbitas reales (Leyes de Kepler) + Ecuación de Kepler solver |
| Radiación Atómica | ✅ | Fisión, Fusión, Decaimiento, Geiger rate, Exposición |
| Fuego Convección | ✅ | Física de partículas térmicas unificada en ry-physics |
| Mutación Genética (DNA) | ✅ | **NUEVO**: Secuencias DNA, mutación por radiación, crossover y expresión |
| 49 tests en ry-science | ✅ | 100% pasando (incluye genética) |
| 34 tests en ry-physics | ✅ | 100% pasando (incluye radiación/Kepler) |

```
Progreso: ████████████████████ 100%
```

### v0.20.0 — ryfrac-postFX + ry-windows + ry3d-gfx + Asset Pipeline + Editor ✅

**Prioridad**: ALTA | **Pilar**: Gaming + Animaciones + Ciencia

| Feature | Estado | Tiempo est. | Inspiración | Detalle |
|---------|--------|-------------|-------------|---------|
| **postfx-ry** | ✅ | — | Post-processing | 6 efectos GPU, FBO chain, presets cyberpunk. |
| **ry-windows** | ✅ | — | Ventana unificada | 7 plataformas, 2 backends, WindowEvent tipos. |
| **ry3d-gfx** | ✅ | — | Expansión v0.2.0 | OrbitCamera3D, FpsCamera3D, Skybox procedural. |
| **Asset pipeline** | ✅ | — | Bevy asset server | Carga automática + tipos fuertes + Sdl2Provider. |
| **ryArt Foundation** | ✅ | — | Generative IA | Pinceles físicos con inercia y trazo persistente. |
| **Unified Input** | ✅ | — | Input Maestro | Fusión ry-input + events-ry en InputManager. |

```
Progreso: ████████████████████ 100%
```

### v1.0.0 — Motor Completo + GitHub Actions + SAZ + Comunidad + LAZOS

**Prioridad**: META | **Pilar**: Todos

| Feature | Estado | Tiempo est. | Detalle |
|---------|--------|-------------|---------|
| **LAZOS Python bridge** | ⏳ | 20-30h | Scripting Python en Ry-Dit |
| **LAZOS C++ bridge** | ⏳ | 15-20h | Native extensions C++ |
| **LAZOS C bridge** | ⏳ | 10-15h | Native extensions C |
| GitHub Actions CI completo | ⏳ | 6-8h | Linux + Windows + macOS + Android |
| SAZ (Shield Archive Format) | ⏳ | 10-15h | Paquete de proyecto |
| Motor estable | ⏳ | 20-30h | API estable + sin breaking changes |
| Documentación completa | ⏳ | 15-20h | Docs para todos los crates |
| Videos tutoriales | ⏳ | 10-15h | YouTube + Discord |
| 15+ crates publicadas | ⏳ | 5-10h | Todos los crates con README |
| Comunidad | ⏳ | — | Discord + docs + ejemplos |
| **Debugger .rydit** | ⏳ | 10-15h | Breakpoints + step-through |
| **Profiler CPU/GPU** | ⏳ | 8-12h | Profiling integrado |
| **Export desktop nativo** | ⏳ | 6-8h | Linux + Windows + macOS builds |
| **Plugin registry** | ⏳ | 8-12h | crates.io integration |
| **Stream multiplayer LAN** | ⏳ | 10-15h | ry-stream multiplayer |

```
Progreso: ░░░░░░░░░░░░░░░░░░░░ 0%
```

---

## Progreso General

```
v0.15.0   ████████████████████ 100%
v0.16.0-a ████████████████████ 100%
v0.16.0   ████████████████████ 100%
v0.16.1   ████████████████████ 100%
v0.17.0   ████████████████████ 100%
v0.18.0   ████████████████████ 100%
v0.19.0   ████████████████████ 100%
v0.19.1   ████████████████████ 100%
v0.19.2   ████████████████████ 100%
v0.20.0   ░░░░░░░░░░░░░░░░░░░░   0%
v1.0.0    ░░░░░░░░░░░░░░░░░░░░   0%
```

---

## Objetivos a Largo Plazo

### 🎮 Gaming
1. **Motor 2D/3D completo** para Termux-X11 y escritorio
2. **GPU instancing** para rendimiento masivo (250K+ partículas)
3. **Iluminación 2D + sombras** dinámicas
4. **Editor visual** integrado (separado o 2-in-1)
5. **DLSS/NIS/FSR 2.0** para upscaling de calidad
6. **GIF animation** soporte completo
7. **Sprite sheets reales** con texturas
8. **Sistema de escenas** con transiciones

### 🎬 Animaciones + Ciencia
9. **12 principios Disney** mejorados con más efectos
10. **Simulaciones científicas** interactivas (ondas, cristalización, L-System)
11. **Ilusiones ópticas** animadas en tiempo real
12. **Post-processing** (bloom, glow, blur, morph, color grading)
13. **Emojis TTF** para UI expresiva
14. **Efectos de partículas** configurables (emisores, 50K-250K GPU)

### 📡 Streaming + Comunidad
15. **Streaming LAN** en vivo con ry-stream
16. **Multiplayer LAN** para juegos en red local
17. **Comunidad** de desarrolladores hispanohablantes
18. **Multiplataforma**: Android, Linux, Windows, macOS
19. **LAZOS** bridge: Python, C++, C
20. **GitHub Actions** CI/CD automático
21. **SAZ formato** de archivo (paquete de proyecto)
22. **Lenguaje de scripting** .rydit en español
23. **Debugger + Profiler** integrados

---

## 📐 Filosofía de Adaptación — No copiar, adaptar al estilo Ry-Dit

### Reglas (detalle en `TASKS_2.md`):

| Si el motor X hace... | Ry-Dit adapta así... |
|----------------------|---------------------|
| Godot Input Map simple | → `.rydit-input` archivo simple |
| Godot PackedScene | → `.ryscene` texto legible |
| Bevy asset server | → `AssetServer::load()` idiomático |
| Unity SRP configurable | → `ry-gfx` features en Cargo.toml |
| Unreal Lumen AAA | → Iluminación 2D simple low-end |
| Unity Asset Store | → Plugin registry con crates.io |

**Regla de oro**: Si funciona en Adreno 610, funciona en todo. Binario <1MB (o <5MB con features). RAM <128MB. GPU mínima: OpenGL ES 2.0.

---

<div align="center">

**Ry-Dit v0.19.2 - ROADMAP**

*12 crates publicados ✅ | ~260 tests ✅ | 24+ demos ✅ | 0 errores*

*3 Pilares: 🎮 Gaming · 🎬 Animaciones+Ciencia · 📡 Streaming+Comunidad*

*Próximo: v0.20.0 — postfx-ry ✅ + ry-windows ✅ + ry3d-gfx ✅ + Asset Pipeline + Editor*

*Ver `TASKS.md` para tareas completadas y pendientes*

</div>
