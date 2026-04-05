# Ry-Dit - ROADMAP v0.12.0 -> v1.0.0

**Última actualización**: 2026-04-05
**Versión actual**: v0.12.0 ✅ ry-anim v0.12.0 + Action Assets

---

## Estado Actual

| Métrica | Valor |
|---------|-------|
| **Crates** | 22 (21 ry-* + 1 eliminado) |
| **Líneas Rust** | ~25K+ |
| **Compilación** | 0 errores |
| **Warnings** | ~15 (dead_code en demos) |
| **Tests** | 58/58 pasando (ry-anim) |
| **Crates publicados** | 2 (ry-god + ry-stream) |
| **ELFs compilados** | demo_anime_ry 341K, demo_rigidbody 446K |
| **Bins en src/bin/** | ~31 |
| **Repositorio** | `github.com/lapumlbb18-blip/Ry-dit` |

---

## Versiones Planificadas

### v0.12.0 - ry-anim + Action Assets + Quest + Save/Load + ry-stream (COMPLETADA ✅)

**Fecha**: 2026-04-05
**Commits**: `2f210b7` -> `405a945` (HEAD)

| Feature | Estado |
|---------|--------|
| Math avanzado: 23 funciones (pow, log, exp, PI, E, TAU...) | ✅ |
| Cálculo numérico: derivada, derivada2, integral, integral_trapezio | ✅ |
| Arrays completos: 16 funciones (push, pop, slice, contains...) | ✅ |
| Vec2 tipo nativo: 22 operaciones (add, sub, normalize, dot...) | ✅ |
| toolkit-ry v0.1.0: 5 temas + 20+ widgets | ✅ |
| ry3d-gfx v0.1.0: 15 funciones 3D | ✅ |
| Fix input Android: SDL_TEXTINPUT + 7 hints | ✅ |
| FSR 1.0 integrado con shaders embebidos | ✅ |
| Quest System: 10 funciones | ✅ |
| Save/Load System: 10 funciones | ✅ |
| One-way platforms: 2 funciones | ✅ |
| ry-stream v0.1.0 publicado en crates.io | ✅ |
| ry-god v0.1.0 publicado en crates.io | ✅ |
| ry-ecs eliminado: -1,143 líneas | ✅ |
| nbody_simulate movido a ry-physics | ✅ |
| ry-anim v0.8.0 → v0.12.0 completo | ✅ |
| Fix linking raylib en build.rs | ✅ |
| demo_anime_ry ELF compilado (341K release) | ✅ |
| 9 documentos nuevos creados | ✅ |
| 17 archivos antiguos organizados en docs/ | ✅ |
| 22 crates compilando | ✅ 0 errores |

```
Progreso: ████████████████████ 100%
```

### v0.13.0 - ry-input + Demos completos (EN PROGRESO)

**Prioridad**: ALTA

| Feature | Estado | Tiempo est. |
|---------|--------|-------------|
| ry-input crate (SDL2 input + raylib render) | ⏳ | 10-15h |
| Sprite animation en juegos reales | ⏳ | 15-20h |
| v-shield platform layer | ⏳ | 15-20h |
| ry-stream v0.2.0 (mDNS) | ⏳ | 8-12h |
| ry-physics N-cuerpos >2 | ⏳ | 10-15h |
| Demos funcionales en Termux-X11 | ⏳ | 15-20h |
| Galería actualizada en README | ⏳ | 2-4h |

```
Progreso: ░░░░░░░░░░░░░░░░░░░░ 0%
```

### v0.14.0 - Integraciones Avanzadas

**Prioridad**: MEDIA

| Feature | Estado | Tiempo est. |
|---------|--------|-------------|
| LAZOS Python bridge | ⏳ | 20-30h |
| Camera3D + DrawCube | ⏳ | 12-16h |
| ry-science FFT, fractales | ⏳ | 15-20h |
| ry-geometry Vec3/Mat4 | ⏳ | 12-16h |

```
Progreso: ░░░░░░░░░░░░░░░░░░░░ 0%
```

### v1.0.0 - Lanzamiento Público

**Prioridad**: META

| Feature | Estado | Notas |
|---------|--------|-------|
| Parser 100% funcional | ✅ | Sin errores conocidos |
| 5+ demos funcionales | ⏳ | Termux-X11 |
| Crates publicados | ✅ | 2 publicados (ry-god + ry-stream) |
| Documentación completa | ⏳ | Guía usuario + dev |
| Videos tutoriales | ⏳ | YouTube |
| README completo | ⏳ | Con galería |
| GitHub Actions CI | ⏳ | Build automático |
| Editor visual | ⏳ | Integrado |

```
Progreso: ██░░░░░░░░░░░░░░░░░░ 10%
```

---

## Progreso General

```
v0.12.0 ████████████████████ 100%
v0.13.0 ░░░░░░░░░░░░░░░░░░░░   0%
v0.14.0 ░░░░░░░░░░░░░░░░░░░░   0%
v1.0.0  ██░░░░░░░░░░░░░░░░░░  10%
```

---

## Tabla de Versiones

| Versión | Fecha | Commits | Crates | Errores | Tests | Features Clave |
|---------|-------|---------|--------|---------|-------|----------------|
| v0.11.4 | 2026-04-02 | 20+ | 18 | 0 | — | Lifetimes fix |
| v0.11.5 | 2026-04-02 | 10+ | 18 | 0 | — | 0 errores final |
| v0.12.0 | 2026-04-04 | 7 | 22 | 0 | 15 | Rebrand + Parser infalible |
| **v0.12.0** | **2026-04-05** | **18+** | **22** | **0** | **58** | **ry-anim + Quest + Save/Load + ry-stream** |
| v0.13.0 | 2026-04-xx | — | 22 | — | — | ry-input + Demos completos |
| v0.14.0 | 2026-04-xx | — | 22 | — | — | LAZOS + Camera3D |
| v1.0.0 | Futuro | — | 25+ | — | — | Motor completo |

---

## Objetivos a Largo Plazo

1. **Motor 2D completo** para Termux-X11
2. **Lenguaje de scripting** en español
3. **Comunidad** de desarrolladores
4. **Multiplataforma**: Android, Linux, Windows
5. **Editor visual** integrado

---

<div align="center">

**Ry-Dit v0.12.0 - ROADMAP**

*ry-anim v0.12.0 | 41 funciones | 58 tests | 22 crates | 2 publicados*

</div>
