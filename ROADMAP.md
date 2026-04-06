# Ry-Dit - ROADMAP v0.13.0 -> v1.0.0

**Última actualización**: 2026-04-05
**Versión actual**: v0.13.0 ✅ events-ry + Panel Visual + RyBot IPC + LAZOS

---

## Estado Actual

| Métrica | Valor |
|---------|-------|
| **Crates** | 23 |
| **Líneas Rust** | ~27K+ |
| **Compilación** | 0 errores |
| **Warnings** | ~37 (100% dead_code API futura) |
| **Tests** | 95/95 pasando |
| **Crates publicados** | 2 (ry-god + ry-stream) |
| **ELFs compilados** | demo_anime_ry 341K, demo_rigidbody 446K, demo_panel_visual |
| **Bins en src/bin/** | ~33 |
| **Repositorio** | `github.com/lapumlbb18-blip/Ry-dit` |

---

## Versiones Planificadas

### v0.13.0 - events-ry + Panel Visual + RyBot IPC + LAZOS (COMPLETADA ✅)

**Fecha**: 2026-04-05
**Commits**: `405a945` -> `118ee6a` (HEAD)

| Feature | Estado |
|---------|--------|
| events-ry v0.1.0: Input unificado + TextInput + Shell | ✅ |
| Sdl2InputBackend feature-gated (41 variantes, 90+ teclas) | ✅ |
| demo_panel_visual: 4 paneles (Screen, Console, Input, Controls) | ✅ |
| demo_panel: Panel consola puro con shell interactivo | ✅ |
| 6 escenas animadas en panel (Disney, Illusions, Effects, Science) | ✅ |
| Pipeline gráfico: Zink/DRI3 → OpenGL ES → VirGL fallback | ✅ |
| Limpieza warnings: 146→37 (0 errores) | ✅ |
| events-ry integrado como dependencia de ry-rs | ✅ |
| TextInputAction exportado públicamente | ✅ |
| lizer: AST cache real (FNV-1a, 256 entradas, LRU) | ✅ |
| RyBot CLI IPC: save_status cada 30 frames | ✅ |
| LAZOS JSON-RPC + Python bridge verificado | ✅ |
| README actualizado a v0.13.0 | ✅ |
| Tag v0.13.0 creado y publicado | ✅ |
| snake movido a ejemplos_pendientes/ | ✅ |
| 23 crates compilando | ✅ 0 errores |

```
Progreso: ████████████████████ 100%
```

### v0.14.0 - migui texto real + v-shield platform layer (EN PLANIFICACIÓN)

**Prioridad**: ALTA

| Feature | Estado | Tiempo est. |
|---------|--------|-------------|
| migui texto real (ab_glyph) | ⏳ | 4-6h |
| Sdl2InputBackend en demo real | ⏳ | 2-3h |
| v-shield platform layer | ⏳ | 15-20h |
| Consola visual en ry-gfx | ⏳ | 3-4h |
| ry-stream v0.2.0 (mDNS) | ⏳ | 8-12h |
| ry-physics N-cuerpos >2 | ⏳ | 10-15h |
| HybridBackend (SDL2+raylib) | ⏳ | 6-8h |

```
Progreso: ░░░░░░░░░░░░░░░░░░░░ 0%
```

### v0.15.0 - Integraciones Avanzadas

**Prioridad**: MEDIA

| Feature | Estado | Tiempo est. |
|---------|--------|-------------|
| LAZOS Python bridge completo | ⏳ | 20-30h |
| Editor visual básico | ⏳ | 24-32h |
| ry-geometry Vec3/Mat4 | ⏳ | 12-16h |
| bgfx_libs referencias (imgui, sokol, SDL3) | ⏳ | 10-15h |

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
| ry-input (SDL2 input + raylib render) | ⏳ | 10-15h |
| Publicar 10+ crates crates.io | ⏳ | Visibilidad |

### v0.13.0 - FASE FINAL + LANZAMIENTO (EN PROGRESO — Tareas en paralelo)

**Prioridad**: MÁXIMA — Lanzamiento en días

| Tarea | Estado | Notas |
|-------|--------|-------|
| **Raylib + Lorie + Plataformas** | ⏳ | Cerrar soporte multiplataforma |
| **UI/Editor estilizado** | ⏳ | migui + SDL2: ventanas y botones listos, solo estilizar |
| **bgfx_libs integración** | ⏳ | Ver ~/bgfx_libs (raylib + lorie) |
| **GitHub Actions CI/CD** | ⏳ | Tests automáticos + builds multiplataforma |
| ry-input (SDL2 input + raylib render) | ⏳ | Input directo Termux-X11/Lorie |
| Lanzamiento público | 🔮 | En unos días |

---

## Progreso General

```
v0.12.0 ████████████████████ 100%
v0.13.0 ████████████░░░░░░░░  60% ← Fase final (tareas en paralelo)
v1.0.0  ████░░░░░░░░░░░░░░░░  20%
```

---

## Tabla de Versiones

| Versión | Fecha | Commits | Crates | Errores | Tests | Features Clave |
|---------|-------|---------|--------|---------|-------|----------------|
| v0.11.4 | 2026-04-02 | 20+ | 18 | 0 | — | Lifetimes fix |
| v0.11.5 | 2026-04-02 | 10+ | 18 | 0 | — | 0 errores final |
| v0.12.0 | 2026-04-04 | 7 | 22 | 0 | 15 | Rebrand + Parser infalible |
| **v0.12.0** | **2026-04-05** | **19+** | **22** | **0** | **58** | **ry-anim + Quest + Save/Load + ry-stream + demos** |
| **v0.13.0** | **2026-04-xx** | **—** | **22** | **—** | **—** | **Fase Final: raylib+lorie+UI+bgfx+CI/CD → LANZAMIENTO** |
| v1.0.0 | Futuro | — | 25+ | — | — | Motor completo + comunidad |

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
