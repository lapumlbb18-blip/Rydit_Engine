# 🛡️ Ry-Dit - Motor Simulador de Escenas 2D + Lenguaje de Scripting en Rust para Android/Termux

<div align="center">

![Ry-Dit Logo](screenshots/logo.png)

**"Construido sin prisa, madurado con paciencia"**

[![Version](https://img.shields.io/badge/version-v0.12.0-blue.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Errors](https://img.shields.io/badge/errors-0-green.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Tests](https://img.shields.io/badge/tests-58%2F58-brightgreen.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Status](https://img.shields.io/badge/estado-v0.12.0--anim--action--assets-green.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![SDL2](https://img.shields.io/badge/SDL2-0.37-red.svg)](https://www.libsdl.org/)
[![Platform](https://img.shields.io/badge/platform-Android%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-2%20publicados-purple.svg)](https://crates.io/crates/ry-god)

[📖 Documentación](#-documentación) • [🆕 Qué hay de nuevo en v0.12.0](#-qué-hay-de-nuevo-en-v0120) • [🔧 Estado Actual](#-estado-actual) • [🎬 ry-anim v0.12.0](#-ry-anim-v0120--41-funciones--58-tests) • [🏆 Logros](#-logros) • [🎯 Roadmap](#-roadmap)

</div>

---

## 🆕 ¿Qué hay de nuevo en v0.12.0?

**Última actualización**: 2026-04-05
**Versión actual**: v0.12.0 ✅ FUNCIONAL
**Commit**: `405a945`
**Estado**: 22 crates compilando | 0 errores | 58 tests pasando

### ry-anim v0.12.0 Completo ✅
| Versión | Features | Tests | Demos |
|---------|----------|-------|-------|
| v0.8.0 | 15 Disney completo | 28 | — |
| v0.9.0 | 21 + 6 ilusiones | 35 | demo_illusions |
| v0.10.0 | 27 + 6 efectos | 42 | demo_effects |
| v0.11.0 | 35 + 8 ciencia | 50 | demo_science |
| v0.12.0 | 41 + 6 action_assets | 58 | demo_action_assets |

### Sesión Completa (todo lo hecho)
1. ✅ Math avanzado: 23 funciones nuevas (pow, log, derivadas, integrales, PI, E...)
2. ✅ Arrays completos: 16 funciones (push, pop, len, insert, remove, contains...)
3. ✅ Vec2 tipo nativo: 22 operaciones (add, sub, scale, normalize, dot, cross...)
4. ✅ toolkit-ry v0.1.0: 5 temas + 20+ widgets UI
5. ✅ ry3d-gfx v0.1.0: 15 funciones 3D (cube, sphere, cylinder, grid...)
6. ✅ Fix input Android: SDL_TEXTINPUT + 7 hints SDL2
7. ✅ FSR 1.0 integrado con shaders embebidos
8. ✅ Quest System: 10 funciones
9. ✅ Save/Load System: 10 funciones
10. ✅ One-way platforms: 2 funciones
11. ✅ ry-stream v0.1.0 publicado en crates.io
12. ✅ ry-ecs eliminado (-1,143 líneas)
13. ✅ nbody_simulate movido a ry-physics
14. ✅ ry-anim v0.8.0 → v0.12.0 completo (41 funciones, 58 tests, 4 demos)
15. ✅ Fix linking raylib en build.rs
16. ✅ demo_anime_ry ELF compilado (341K release)
17. ✅ 9 documentos nuevos creados
18. ✅ 17 archivos antiguos organizados en docs/

### ELFs Compilados
| Demo | Tamaño | Modo |
|------|--------|------|
| demo_anime_ry | 341K | release |
| demo_rigidbody | 446K | release |

---

## 🔧 ESTADO ACTUAL - v0.12.0 ANIM + ACTION ASSETS

### ✅ **ESTADO REAL: v0.12.0 - 22 CRATES COMPILANDO | 0 ERRORES | 58 TESTS**

**Última actualización**: 2026-04-05
**Versión actual**: v0.12.0 ✅ FUNCIONAL
**Próxima versión**: v0.13.0 - ry-input + Demos completos
**Estado**: ry-anim ✅ | ry-stream ✅ | ry-god ✅ | Quest ✅ | Save/Load ✅ | FSR ✅
**Commit**: `405a945` (HEAD)
**Crates**: 22 en workspace | 0 errores | 58 tests pasando
**Crates publicados**: 2 (ry-god + ry-stream)

---

### ✅ **LO QUE SÍ FUNCIONA (v0.12.0)**

| Sistema | Estado | Notas |
|---------|--------|-------|
| **ry-anim** | ✅ v0.12.0 | 41 funciones, 58 tests, 4 demos |
| **ry-stream** | ✅ v0.1.0 crates.io | LAN streaming |
| **ry-god** | ✅ v0.1.0 crates.io | Security & Efficiency |
| **Math avanzado** | ✅ 23 funciones | pow, log, exp, PI, derivada, integral |
| **Arrays** | ✅ 16 funciones | push, pop, slice, contains, join... |
| **Vec2** | ✅ 22 operaciones | add, sub, normalize, dot, rotate... |
| **toolkit-ry** | ✅ v0.1.0 | 5 temas + 20+ widgets |
| **ry3d-gfx** | ✅ v0.1.0 | 15 funciones 3D |
| **Quest System** | ✅ 10 funciones | create, objectives, rewards... |
| **Save/Load** | ✅ 10 funciones | create, set_var, load, list... |
| **One-way platforms** | ✅ 2 funciones | check_one_way, resolve_one_way |
| **FSR 1.0** | ✅ Integrado | Shaders embebidos |
| **Parser** | ✅ Infalible | 6 bugs raíz resueltos |
| **Input SDL2** | ✅ Fix Android | SDL_TEXTINPUT + 7 hints |
| **SDL2_ttf** | ✅ Texto real | Texturas cacheadas |
| **Sprites PNG** | ✅ 4 cargados | SDL2_image |
| **Audio SDL2** | ✅ Tonos WAV | SDL2_mixer |
| **Demos binarios** | ✅ ~20+ | src/bin/ |

**Total**: Stack completo funcional + 2 crates publicados ✅

---

## 🎬 ry-anim v0.12.0: 41 Funciones + 58 Tests

### Evolución Completa

| Versión | Features | Tests | Demos | Descripción |
|---------|----------|-------|-------|-------------|
| **v0.8.0** | 15 | 28 | — | 9 principios Disney completos |
| **v0.9.0** | 21 + 6 ilusiones | 35 | demo_illusions | 6 ilusiones ópticas animadas |
| **v0.10.0** | 27 + 6 efectos | 42 | demo_effects | 6 efectos especiales |
| **v0.11.0** | 35 + 8 ciencia | 50 | demo_science | 8 animaciones científicas |
| **v0.12.0** | 41 + 6 action | 58 | demo_action_assets | 6 action assets (sprite animation) |

### Demos Binarios
| Demo | Descripción | Tamaño Release |
|------|-------------|----------------|
| demo_illusions | Ilusiones ópticas | — |
| demo_effects | Efectos especiales | — |
| demo_science | Animaciones científicas | — |
| demo_action_assets | Action assets + sprite anim | — |
| demo_anime_ry | Showcase completo ry-anim | 341K |
| demo_rigidbody | Física + colisiones | 446K |

### ~31 Bins en src/bin/
demo_50k_particulas, demo_action_assets, demo_anime_ry, demo_carga_sprites, demo_colisiones, demo_completo_final, demo_completo_sdl2, demo_effects, demo_fsr_audio, demo_illusions, demo_particles, demo_platformer_completo, demo_rigidbody, demo_science, demo_sprites_final, demo_sprites_v2, demo_stream, demo_ttf_sprites, demo_ttf_sprites_audio, nivel3_test_audio_lowend, nivel3_test_input_lowend, nivel3_test_lowend, rybot_cli, snake, test_audio_minimal, test_audio_sdl2, test_bloques_anidados, test_callback_sdl2, test_parser, test_rydit_simple

---

## 📊 Lista de Crates (22)

| Crate | Versión | Estado |
|-------|---------|--------|
| ry-core | 0.8.2 | ✅ |
| ry-lexer | 0.1.0 | ✅ |
| ry-parser | 0.1.0 | ✅ |
| ry-vm | — | ⚠️ |
| ry-gfx | 0.10.7 | ✅ |
| ry-physics | 0.7.34 | ✅ + nbody_simulate |
| ry-anim | 0.12.0 | ✅ 41 funciones |
| ry-science | — | ⚠️ |
| ry-script | 0.8.2 | ✅ |
| ry-stream | 0.1.0 | ✅ crates.io |
| ry-god | 0.1.0 | ✅ crates.io |
| ry-loader | — | ⚠️ |
| ry-rs | — | Main binary |
| ry-system-ry | 0.11.0 | ⚠️ |
| ry-test | — | ⚠️ |
| toolkit-ry | 0.1.0 | ✅ |
| migui | — | ✅ 12 widgets |
| blast-core | 0.1.0 | ✅ |
| lizer | 0.11.2 | ✅ |
| v-shield | — | ⚠️ |
| ry3d-gfx | 0.1.0 | ✅ |
| ~~ry-ecs~~ | — | 🗑️ Eliminado |

---

## 🎯 Roadmap

<div align="center">

| Versión | Estado | Features | Fecha |
|---------|--------|----------|-------|
| **v0.11.5** | ✅ | 0 Errores + lifetimes fixeados | 2026-04-02 |
| **v0.12.0** | ✅ | ry-anim v0.12.0 + Quest + Save/Load + ry-stream crates.io | 2026-04-05 |
| **v0.13.0** | ⏳ | ry-input + Demos completos | Próxima versión |
| **v1.0.0** | ⏳ | Motor Completo + Editor Visual | Futuro |

</div>

---

## 📋 Tareas Pendientes

| Tarea | Esfuerzo | Prioridad |
|-------|----------|-----------|
| ry-input crate (SDL2 input + raylib render) | 10-15h | 🔴 Alta |
| Sprite animation en juegos reales | 15-20h | 🟡 Media |
| v-shield platform layer | 15-20h | 🟡 Media |
| ry-stream v0.2.0 (mDNS) | 8-12h | 🟡 Media |
| ry-physics N-cuerpos >2 | 10-15h | 🟡 Media |
| LAZOS Python bridge | 20-30h | 🔮 Futuro |
| Editor visual | 24-32h | 🔮 Futuro |

---

## 📖 Documentación

| Documento | Descripción |
|-----------|-------------|
| [ESTRUCTURA.md](ESTRUCTURA.md) | 📂 Estructura completa del proyecto |
| [QWEN.md](QWEN.md) | 📓 Bitácora de sesión |
| [ROADMAP.md](ROADMAP.md) | 🗺️ Planificación v0.12→v1.0 |
| [docs/panorama_v0.13.0.md](docs/panorama_v0.13.0.md) | 🔭 Panorama completo |
| [docs/vision_estrategica.md](docs/vision_estrategica.md) | 🎯 Visión estratégica |
| [docs/guia_compilacion_termux.md](docs/guia_compilacion_termux.md) | 📋 Guía compilación Termux |
| [docs/arquitectura_demos.md](docs/arquitectura_demos.md) | 🏗️ Arquitectura de demos |

---

## 🛡️ MANIFIESTO

> **"David vs Goliat - Un motor de videojuegos en Rust, construido 100% en un Redmi Note 8"**

### Filosofía Ry-Dit

1. **Mobile-First Real** - No "también funciona en Android". **Nació en Android**.
2. **Ligero y Portable** - Binario de ~341-446 KB (demos release)
3. **Educativo** - Código 100% abierto, lenguaje en español, sin magia
4. **David vs Goliat** - 25K+ líneas de Rust bien escritas
5. **Rendimiento Estable** - Sin calentamiento, RAM <100 MB, 60 FPS estables
6. **Portabilidad** - Próximamente Linux + Windows + WebAssembly

### Valores

- **Código > Burocracia** - Preferimos código funcionando a documentación perfecta
- **Mobile-First** - Si no funciona en Android, no es Ry-Dit
- **Ligero** - Cada KB cuenta. Sin dependencias innecesarias
- **Abierto** - MIT license. 100% transparente
- **Educativo** - Enseñamos, no solo damos herramientas
- **Comunidad** - Hispanohablantes, mobile developers, hobbyists

---

## 📱 Construido en Android/Termux

<div align="center">

| Setup | Especificaciones |
|-------|-----------------|
| **Dispositivo** | Redmi Note 8 |
| **OS** | Android 11 |
| **Terminal** | Termux |
| **RAM** | 4 GB |

</div>

**Filosofía:** Construido 100% en teléfono Android, sin prisa, madurado con paciencia.

---

<div align="center">

## Ry-Dit v0.12.0 - ry-anim Action Assets + 58 Tests

*0 errores | 22 crates compilando | 58 tests | 2 crates publicados*

**Proxima version: v0.13.0 - ry-input + Demos completos**

</div>
