# Ry-Dit - ROADMAP v0.14.0 -> v1.0.0

**Última actualización**: 2026-04-06
**Versión actual**: v0.14.0 ✅ demo_torreta_vs_sprites + 25 crates + juego completo
**Commit**: `df4ec17`

---

## Estado Actual

| Métrica | Valor |
|---------|-------|
| **Crates** | 25 |
| **Líneas Rust** | ~30K+ |
| **Compilación** | 0 errores |
| **Tests** | 95+ pasando |
| **Crates publicados** | 2 (ry-god + ry-stream) |
| **ELFs compilados** | demo_torreta_vs_sprites 434K, demo_rigidbody 446K, demo_ttf_sprites 436K, demo_panel_visual 339K, demo_menu_bar 330K, demo_anime_ry 341K, demo_50k_particulas 313K, demo_colisiones 309K |
| **Bins en src/bin/** | ~33+ |
| **Repositorio** | `github.com/lapumlbb18-blip/Ry-dit` |

---

## Versiones Planificadas

### v0.14.0 - ry-backend dual + juego completo + 25 crates (COMPLETADA ✅)

**Fecha**: 2026-04-06
**Commits**: `df4ec17`

| Feature | Estado |
|---------|--------|
| ry-backend v0.1.0: Dual backend (raylib + SDL2 TTF/input/audio) | ✅ |
| Features: raylib-only, sdl2-only, dual-backend, mobile-hybrid | ✅ |
| migui conectado a ry-backend (antes sdl2 directo) | ✅ |
| ry-system-ry v0.14.0: Sistema unificado (RySystem: core + gui) | ✅ |
| ry-config v0.1.0: Config parser (entities, levels, checkpoints) - zero deps | ✅ |
| events-ry v0.1.0: Input unificado 3 capas + Sdl2InputBackend | ✅ |
| demo_torreta_vs_sprites: JUEGO COMPLETO (menú + 3 niveles + cámara + AI + audio) | ✅ 434K |
| demo_menu_bar: Menús Dear ImGui + mouse + touch | ✅ 330K |
| demo_panel_visual: 4 paneles + consola interactiva | ✅ 339K |
| Texto TTF profesional: Anti-alias blended | ✅ |
| Mouse events completos: Click, doble click, derecho, scroll | ✅ |
| Touch Android: FingerDown/Motion/Up | ✅ |
| ry-rs ahora es bin + lib (antes solo bin) | ✅ |
| Código muerto eliminado: module.rs (230 líneas) | ✅ |
| Tests desactualizados movidos a docs/tests_referencia/ | ✅ |
| lizer AST cache real: FNV-1a, 256 entradas, LRU | ✅ |
| Demos existentes confirmados funcionando | ✅ |
| 25 crates compilando | ✅ 0 errores |

```
Progreso: ████████████████████ 100%
```

### v0.15.0 - Demos Termux-X11 + v-shield platform layer (PLANIFICADA)

**Prioridad**: ALTA

| Feature | Estado | Tiempo est. |
|---------|--------|-------------|
| Demos funcionales en Termux-X11 con RySystem | ⏳ | 6-8h |
| v-shield platform layer | ⏳ | 15-20h |
| ry-stream v0.2.0 (mDNS) | ⏳ | 8-12h |
| ry-physics N-cuerpos >2 | ⏳ | 10-15h |
| ry-backend v0.2.0 (optimizaciones) | ⏳ | 6-8h |
| Consola visual en ry-gfx | ⏳ | 3-4h |

```
Progreso: ░░░░░░░░░░░░░░░░░░░░ 0%
```

### v0.16.0 - Features Avanzadas (PLANIFICADA)

**Prioridad**: MEDIA

| Feature | Estado | Tiempo est. |
|---------|--------|-------------|
| Platform crate (abstracción multiplataforma) | ⏳ | 15-20h |
| Soporte de emojis en TTF | ⏳ | 4-6h |
| GIF animation | ⏳ | 8-12h |
| GPU instancing (revisar gpu_instancing.rs de ry-gfx) | ⏳ | 10-15h |
| Features 3D paso a paso | ⏳ | 12-16h |

```
Progreso: ░░░░░░░░░░░░░░░░░░░░ 0%
```

### v1.0.0 - Lanzamiento Público

**Prioridad**: META

| Feature | Estado | Notas |
|---------|--------|-------|
| Parser 100% funcional | ✅ | Sin errores conocidos |
| 7+ demos funcionales | ✅ | 8 ELFs compilados |
| Crates publicados | ✅ | 2 publicados (ry-god + ry-stream) |
| Documentación completa | ⏳ | Guía usuario + dev |
| Videos tutoriales | ⏳ | YouTube |
| README completo | ⏳ | Con galería |
| GitHub Actions CI | ⏳ | Tests automáticos + builds |
| Publicar 10+ crates crates.io | ⏳ | Visibilidad |
| Editor visual | ⏳ | 24-32h |
| LAZOS Python bridge completo | ⏳ | 20-30h |

---

## Progreso General

```
v0.12.0 ████████████████████ 100%
v0.13.0 ████████████████████ 100%
v0.14.0 ████████████████████ 100%
v1.0.0  ██████░░░░░░░░░░░░░░  30%
```

---

## Tabla de Versiones

| Versión | Fecha | Commits | Crates | Errores | Tests | Features Clave |
|---------|-------|---------|--------|---------|-------|----------------|
| v0.11.4 | 2026-04-02 | 20+ | 18 | 0 | — | Lifetimes fix |
| v0.11.5 | 2026-04-02 | 10+ | 18 | 0 | — | 0 errores final |
| v0.12.0 | 2026-04-05 | 19+ | 22 | 0 | 58 | ry-anim + Quest + Save/Load + ry-stream |
| **v0.13.0** | **2026-04-05** | **10+** | **23** | **0** | **95+** | **events-ry + Panel Visual + RyBot IPC + LAZOS** |
| **v0.14.0** | **2026-04-06** | **df4ec17** | **25** | **0** | **95+** | **ry-backend dual + juego completo + 25 crates** |
| v0.15.0 | 2026-04-xx | — | 25+ | — | — | Demos Termux-X11 + v-shield + ry-stream v0.2.0 |
| v1.0.0 | Futuro | — | 25+ | — | — | Motor completo + Editor Visual + Comunidad |

---

## Objetivos a Largo Plazo

1. **Motor 2D/3D completo** para Termux-X11 y escritorio
2. **Lenguaje de scripting** en español
3. **Comunidad** de desarrolladores hispanohablantes
4. **Multiplataforma**: Android, Linux, Windows
5. **Editor visual** integrado
6. **GPU instancing** para rendimiento masivo
7. **LAZOS Python bridge** para IA/ML

---

<div align="center">

**Ry-Dit v0.14.0 - ROADMAP**

*demo_torreta_vs_sprites 434K | 25 crates | 95+ tests | 2 crates publicados*

*Próxima versión: v0.15.0 - Demos Termux-X11 + v-shield platform layer*

</div>
