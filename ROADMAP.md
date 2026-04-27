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

## 🧭 ROADMAP (v0.22.0+)

| Versión | Foco Principal | Estado |
|---------|----------------|--------|
| **v0.22.0** | Consolidación del Editor (`ry-editor`) | ✅ Completado |
| **v0.23.0** | Inyección de Escena 3D en Viewport | ⏳ Pendiente |
| **v0.24.0** | Integración estética con `ry-art` | ⏳ Pendiente |
| **v0.25.0** | Inspector de Entidades y Hot Reload | ⏳ Pendiente |
| **v1.0.0**  | Motor Completo + GitHub Actions CI/CD | ⏳ Pendiente |

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
