# Ry-Dit - ROADMAP v0.16.1 → v1.0.0

**Última actualización**: 2026-04-09
**Versión actual**: v0.16.1 ✅ Snake + Buscaminas + Action Sprite + Tilemap 2.0 + 12 crates publicados

---

## Estado Actual

| Métrica | Valor |
|---------|-------|
| **Crates** | 23 |
| **Líneas Rust** | ~35K+ |
| **Compilación** | 0 errores |
| **Tests** | 144/144 pasando |
| **Crates publicados** | 12 |
| **Demos funcionales** | 15+ |
| **Launchers** | 8 con auto-detección DISPLAY + Zink |
| **Repositorio** | `github.com/lapumlbb18-blip/Ry-dit` |

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

### v0.17.0 — Sprite Sheets + Texturas + Emojis + GIF

**Prioridad**: ALTA

| Feature | Estado | Tiempo est. |
|---------|--------|-------------|
| Sprite sheets reales | ⏳ | 6-8h |
| Texturas en demos | ⏳ | 4-6h |
| Soporte emojis TTF | ⏳ | 4-6h |
| Carga/edición GIF | ⏳ | 8-12h |
| Audio/Mix más completo | ⏳ | 6-8h |

```
Progreso: ░░░░░░░░░░░░░░░░░░░░ 0%
```

### v0.18.0 — DLSS/NIS + Bordes Suaves + Opacidad

**Prioridad**: ALTA

| Feature | Estado | Tiempo est. |
|---------|--------|-------------|
| NIS (NVIDIA Image Scaling) | ⏳ | 6-8h |
| FSR 2.0 (temporal) | ⏳ | 20-30h |
| Opacidad/transparencia | ⏳ | 4-6h |
| Fade in/out transiciones | ⏳ | 2-4h |
| Texturas con canal alpha | ⏳ | 4-6h |

```
Progreso: ░░░░░░░░░░░░░░░░░░░░ 0%
```

### v0.19.0 — Letras 3D + Panel Visual + Rybot CLI+GUI

**Prioridad**: MEDIA

| Feature | Estado | Tiempo est. |
|---------|--------|-------------|
| Letras 3D en demos | ⏳ | 6-8h |
| Panel visual mejorado | ⏳ | 8-12h |
| migui mejoras | ⏳ | 4-6h |
| Rybot CLI completo | ⏳ | 10-15h |
| Rybot GUI | ⏳ | 12-16h |

```
Progreso: ░░░░░░░░░░░░░░░░░░░░ 0%
```

### v0.20.0 — Editor + LAZOS Multi-lenguaje

**Prioridad**: MEDIA

| Feature | Estado | Tiempo est. |
|---------|--------|-------------|
| Editor separado (o 2-in-1) | ⏳ | 20-30h |
| LAZOS Python bridge | ⏳ | 20-30h |
| LAZOS C++ bridge | ⏳ | 15-20h |
| LAZOS C bridge | ⏳ | 10-15h |
| Tilemap editor visual | ⏳ | 12-16h |

```
Progreso: ░░░░░░░░░░░░░░░░░░░░ 0%
```

### v1.0.0 — Motor Completo + GitHub Actions + SAZ

**Prioridad**: META

| Feature | Estado | Tiempo est. |
|---------|--------|-------------|
| GitHub Actions CI completo | ⏳ | 6-8h |
| SAZ (Shield Archive Format) | ⏳ | 10-15h |
| Motor estable | ⏳ | 20-30h |
| Documentación completa | ⏳ | 15-20h |
| Videos tutoriales | ⏳ | 10-15h |
| 15+ crates publicados | ⏳ | 5-10h |
| Comunidad | ⏳ | — |

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
v0.17.0   ░░░░░░░░░░░░░░░░░░░░   0%
v0.18.0   ░░░░░░░░░░░░░░░░░░░░   0%
v0.19.0   ░░░░░░░░░░░░░░░░░░░░   0%
v0.20.0   ░░░░░░░░░░░░░░░░░░░░   0%
v1.0.0    ░░░░░░░░░░░░░░░░░░░░   0%
```

---

## Objetivos a Largo Plazo

1. **Motor 2D/3D completo** para Termux-X11 y escritorio
2. **Lenguaje de scripting** .rydit en español
3. **Comunidad** de desarrolladores hispanohablantes
4. **Multiplataforma**: Android, Linux, Windows
5. **Editor visual** integrado (separado o 2-in-1)
6. **GPU instancing** para rendimiento masivo
7. **LAZOS** bridge: Python, C++, C
8. **DLSS/NIS/FSR 2.0** para upscaling de calidad
9. **GIF animation** soporte completo
10. **GitHub Actions** CI/CD automático

---

<div align="center">

**Ry-Dit v0.16.1 - ROADMAP**

*12 crates publicados ✅ | 144 tests ✅ | 15+ demos ✅ | 0 errores*

*Próximo: v0.17.0 — Sprite sheets reales + Texturas + Emojis + GIF*

</div>
