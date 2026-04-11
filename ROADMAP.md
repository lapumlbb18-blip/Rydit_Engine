# Ry-Dit - ROADMAP v0.18.0 → v1.0.0

**Última actualización**: 2026-04-11
**Versión actual**: v0.18.0 ✅ 3D Primitives + Transiciones + Audio Mixer + UTF-8 Fix + Emojis
**Análisis estratégico**: Ver `TASKS_2.md` — Análisis comparativo con Unreal, Unity, Godot, Bevy

---

## Estado Actual

| Métrica | Valor |
|---------|-------|
| **Crates** | 23 |
| **Líneas Rust** | ~38K+ |
| **Compilación** | 0 errores |
| **Tests** | 147/147 pasando |
| **Crates publicados** | 12 |
| **Demos funcionales** | 20+ |
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

### v0.19.0 — Letras 3D + Escenas (.ryscene) + Input map + Rybot CLI+GUI

**Prioridad**: MEDIA | **Pilar**: Gaming + Ciencia

| Feature | Estado | Tiempo est. | Inspiración | Detalle |
|---------|--------|-------------|-------------|---------|
| Letras 3D en demos | ⏳ | 6-8h | ry3d-gfx | Texto 3D real en demos |
| Panel visual mejorado | ⏳ | 8-12h | — | migui + toolkit-ry |
| migui mejoras | ⏳ | 4-6h | Dear ImGui | Más widgets + temas |
| Rybot CLI completo | ⏳ | 10-15h | — | CLI para crear proyectos |
| Rybot GUI | ⏳ | 12-16h | — | GUI de Rybot CLI |
| **Sistema de escenas** | ⏳ | 8-12h | Godot PackedScene | .ryscene archivos |
| **Scene transitions** | ⏳ | 4-6h | Unity loading | Fade entre niveles |
| **Scene tree visual** | ⏳ | 12-16h | Godot | Editor de escenas |
| **Input map configurable** | ⏳ | 6-8h | Godot Input Map | .rydit-input rebind |

```
Progreso: ░░░░░░░░░░░░░░░░░░░░ 0%
```

### v0.20.0 — Editor + LAZOS Multi-lenguaje + Asset Pipeline

**Prioridad**: MEDIA | **Pilar**: Gaming + Streaming

| Feature | Estado | Tiempo est. | Inspiración | Detalle |
|---------|--------|-------------|-------------|---------|
| Editor separado (o 2-in-1) | ⏳ | 20-30h | Godot/Unity | Editor visual de juegos |
| LAZOS Python bridge | ⏳ | 20-30h | Unity Python | Scripting Python en Ry-Dit |
| LAZOS C++ bridge | ⏳ | 15-20h | Godot GDExtension | Native extensions C++ |
| LAZOS C bridge | ⏳ | 10-15h | — | Native extensions C |
| Tilemap editor visual | ⏳ | 12-16h | Godot TileMap | Editor GUI de tilemap |
| **Asset pipeline** | ⏳ | 8-12h | Bevy asset server | Carga automática + hot reload |
| **Layout Flexbox UI** | ⏳ | 15-20h | Bevy UI / Godot | migui con layout system |
| **Hot reload de assets** | ⏳ | 6-8h | Unity reimport | Recarga sin reiniciar |
| **Stream multiplayer LAN** | ⏳ | 10-15h | — | ry-stream multiplayer |

```
Progreso: ░░░░░░░░░░░░░░░░░░░░ 0%
```

### v1.0.0 — Motor Completo + GitHub Actions + SAZ + Comunidad

**Prioridad**: META | **Pilar**: Todos

| Feature | Estado | Tiempo est. | Detalle |
|---------|--------|-------------|---------|
| GitHub Actions CI completo | ⏳ | 6-8h | Linux + Windows + macOS + Android |
| SAZ (Shield Archive Format) | ⏳ | 10-15h | Paquete de proyecto |
| Motor estable | ⏳ | 20-30h | API estable + sin breaking changes |
| Documentación completa | ⏳ | 15-20h | Docs para todos los crates |
| Videos tutoriales | ⏳ | 10-15h | YouTube + Discord |
| 15+ crates publicados | ⏳ | 5-10h | Todos los crates con README |
| Comunidad | ⏳ | — | Discord + docs + ejemplos |
| **Debugger .rydit** | ⏳ | 10-15h | Breakpoints + step-through |
| **Profiler CPU/GPU** | ⏳ | 8-12h | Profiling integrado |
| **Post-processing** | ⏳ | 6-8h | Bloom, glow, blur, color grade |
| **Export desktop nativo** | ⏳ | 6-8h | Linux + Windows + macOS builds |
| **Plugin registry** | ⏳ | 8-12h | crates.io integration |
| **Render pipelines** | ⏳ | 10-15h | Forward + Deferred como features |

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
v0.19.0   ░░░░░░░░░░░░░░░░░░░░   0%
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

**Ry-Dit v0.18.0 - ROADMAP**

*12 crates publicados ✅ | 147 tests ✅ | 20+ demos ✅ | 0 errores*

*3 Pilares: 🎮 Gaming · 🎬 Animaciones+Ciencia · 📡 Streaming+Comunidad*

*Próximo: v0.19.0 — Letras 3D + Escenas (.ryscene) + Input map + Rybot CLI+GUI*

*Ver `TASKS.md` para tareas completadas y pendientes*

</div>
