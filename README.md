# 🛡️ Ry-Dit - Motor Simulador de Escenas 2D + Lenguaje de Scripting en Rust para Android/Termux

<div align="center">

![Ry-Dit Logo](screenshots/logo.png)

**"Construido sin prisa, madurado con paciencia"**

[![Version](https://img.shields.io/badge/version-v0.14.0-blue.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Errors](https://img.shields.io/badge/errors-0-green.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Tests](https://img.shields.io/badge/tests-95%2F95-brightgreen.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Status](https://img.shields.io/badge/estado-v0.14.0--dual--backend-green.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![SDL2](https://img.shields.io/badge/SDL2-0.37-red.svg)](https://www.libsdl.org/)
[![Raylib](https://img.shields.io/badge/raylib-5.0-orange.svg)](https://www.raylib.com/)
[![Platform](https://img.shields.io/badge/platform-Android%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-2%20publicados-purple.svg)](https://crates.io/crates/ry-god)

[📖 Documentación](#-documentación) • [🆕 Qué hay de nuevo en v0.14.0](#-qué-hay-de-nuevo-en-v0140) • [🔧 Estado Actual](#-estado-actual) • [🔌 events-ry v0.1.0](#-events-ry-v010--input-unificado--text-input--shell) • [🎮 Panel Visual](#-panel-visual-demo_panel_visual) • [🏆 Logros](#-logros) • [🎯 Roadmap](#-roadmap)

</div>

---

## 🆕 ¿Qué hay de nuevo en v0.14.0?

**Última actualización**: 2026-04-06
**Versión actual**: v0.14.0 ✅ FUNCIONAL
**Commit**: `df4ec17`
**Estado**: 25 crates compilando | 0 errores | 95+ tests pasando

### 🎨 ry-backend v0.1.0 - Dual Backend (raylib + SDL2 TTF) ✅
| Feature | Descripción | Estado |
|---------|-------------|--------|
| **raylib-backend** | Render + input vía raylib | ✅ |
| **SDL2 TTF** | Texto profesional anti-alias blended | ✅ |
| **dual-backend** | Ambos backends simultáneos | ✅ |
| **mobile-hybrid** | SDL2 input + raylib render (Termux-X11) | ✅ |

### 🖱️ Mouse Events Completos ✅
| Evento | Descripción |
|--------|-------------|
| Click | Click izquierdo |
| Doble Click | Doble click detectado |
| Click Derecho | Right button event |
| Scroll | Wheel up/down |

### 📱 Touch Android Completo ✅
| Evento | Descripción |
|--------|-------------|
| FingerDown | Touch inicio detectado |
| FingerMotion | Movimiento en pantalla |
| FingerUp | Touch fin detectado |

### 🎮 demo_torreta_vs_sprites - JUEGO COMPLETO ✅
| Feature | Estado |
|---------|--------|
| Sprites PNG | ✅ |
| Texto TTF real | ✅ |
| Física + colisiones | ✅ |
| Audio SDL2 | ✅ |
| Cámara 2D follow | ✅ |
| Mapa extenso (1200x800) | ✅ |
| HUD (toolkit-ry ready) | ✅ |
| Menús (migui MenuBar) | ✅ |
| Game states: Menu, Playing, Paused, GameOver, GameWin, LevelComplete | ✅ |
| 3 niveles con dificultad creciente | ✅ |
| Enemigos con patrol AI | ✅ |
| Huecos (caer = -1 vida) | ✅ |
| Pausa + reinicio | ✅ |
| Tamaño release | 434K |

### 🎮 Controles demo_torreta_vs_sprites
- **← → ó A/D**: Mover torreta
- **W ó ↑**: Saltar
- **S ó ↓**: Bajar rápido
- **SPACE**: Disparar
- **P**: Pausa
- **R**: Reiniciar nivel
- **ESC**: Salir / Volver menú

### 🏗️ ry-system-ry v0.14.0 - Sistema Unificado ✅
| Componente | Descripción |
|------------|-------------|
| **RySystem struct** | Sistema unificado (core + gui) |
| **Init/Shutdown** | Lifecycle completo |
| **Event Loop** | Loop unificado multi-backend |

### 📦 ry-config v0.1.0 - Config Parser ✅
| Feature | Descripción |
|---------|-------------|
| **Entities** | Parseo de configuraciones |
| **Levels** | Gestión de niveles |
| **Checkpoints** | Puntos de control |
| **Zero deps** | Sin dependencias externas |

### 🎨 migui con ry-backend ✅
| Cambio | Antes | Ahora |
|--------|-------|-------|
| **Backend** | sdl2 directo | ry-backend (abstracción) |
| **Texto TTF** | Básico | Profesional anti-alias blended |
| **Features** | sdl2-only | raylib-only, sdl2-only, dual-backend, mobile-hybrid |

### Sesión Completa v0.14.0 (todo lo hecho)
1. ✅ **ry-backend v0.1.0**: Dual backend raylib + SDL2 TTF/input/audio
2. ✅ **migui migra a ry-backend**: Abstracción de render
3. ✅ **ry-system-ry v0.14.0**: Sistema unificado con RySystem struct (core + gui)
4. ✅ **ry-config v0.1.0**: Config parser (entities, levels, checkpoints) - zero deps
5. ✅ **events-ry v0.1.0**: Input unificado 3 capas (InputEvent, TextInput, Shell) + Sdl2InputBackend
6. ✅ **Texto TTF profesional**: Anti-alias blended rendering
7. ✅ **Mouse events completos**: Click, doble click, derecho, scroll
8. ✅ **Touch Android completo**: FingerDown/Motion/Up
9. ✅ **Features multi-backend**: raylib-only, sdl2-only, dual-backend, mobile-hybrid
10. ✅ **demo_torreta_vs_sprites**: JUEGO COMPLETO - Menú + 3 niveles + cámara + enemigos + audio (434K)
11. ✅ **demo_menu_bar**: Menús Dear ImGui + mouse completo + touch
12. ✅ **demo_panel_visual**: 4 paneles (Screen, Console, Input, Controls) + consola interactiva
13. ✅ **25 crates compilando**: 0 errores en workspace completo
14. ✅ **Código muerto eliminado**: module.rs (230 líneas, RyditModule duplicado)
15. ✅ **Tests desactualizados**: Movidos a docs/tests_referencia/
16. ✅ **ry-rs ahora es bin + lib**: Antes solo bin
17. ✅ **lizer AST cache real**: FNV-1a, 256 entradas, LRU

---

## 🆕 ¿Qué hay de nuevo en v0.13.0?

**Última actualización**: 2026-04-05
**Versión actual**: v0.13.0 ✅ FUNCIONAL
**Commit**: `df4ec17`
**Estado**: 23 crates compilando | 0 errores | 95+ tests pasando

### 🔌 events-ry v0.1.0 Completo ✅
| Capa | Funciones | Tests | Descripción |
|------|-----------|-------|-------------|
| **CAPA 1: InputEvent** | 41 variantes + 90+ teclas | 37 | Input unificado (teclado, mouse, touch, gamepad) |
| **CAPA 2: TextInput** | Composición IME completa | — | Backspace, cursor, commit/cancel, max_length |
| **CAPA 3: Shell** | 9 handlers integrados | — | help, load, exec, debug, echo, clear, version |
| **MANAGER** | InputManager unificado | — | Conecta las 3 capas + MockBackend |
| **SDL2 Backend** | Sdl2InputBackend (feature) | — | EventPump → InputEvent real |

### 🎮 Panel Visual (inspirado en bgfx_libs) ✅
| Panel | Contenido | Activación |
|-------|-----------|------------|
| **Screen** | 6 escenas ry-anim animadas | Tecla 1 |
| **Console** | Shell interactivo + comandos | Tecla 2 |
| **Input State** | Mouse, keys, FPS en vivo | Tecla 3 |
| **Controls** | Key bindings del panel | Tecla 4 |

### Sesión Completa v0.13.0 (todo lo hecho)
1. ✅ **events-ry v0.1.0**: Input unificado + TextInput + Shell (95 tests totales)
2. ✅ **Sdl2InputBackend**: Backend real con SDL2 event_pump (feature-gated)
3. ✅ **demo_panel**: Panel consola puro con shell interactivo
4. ✅ **demo_panel_visual**: Panel visual SDL2 completo (inspirado en bgfx_libs/console_emulator_prototype.cpp)
5. ✅ **6 escenas animadas**: Disney Follow Through, Arcs, Rotating Snakes, Neon Glow, Morphing, Tusi Couple
6. ✅ **Pipeline gráfico**: Zink/DRI3 → OpenGL ES → VirGL fallback
7. ✅ **Limpieza warnings**: 146→37 warnings (0 errores)
8. ✅ **events-ry integrado** como dependencia de ry-rs
9. ✅ **TextInputAction exportado** públicamente
10. ✅ **Tag v0.13.0** creado y publicado
| Demo | Tamaño | Modo |
|------|--------|------|
| demo_anime_ry | 341K | release |
| demo_rigidbody | 446K | release |

---

## 🔧 ESTADO ACTUAL - v0.14.0 DUAL BACKEND + JUEGO COMPLETO + 25 CRATES

### ✅ **ESTADO REAL: v0.14.0 - 25 CRATES COMPILANDO | 0 ERRORES | 95+ TESTS**

**Última actualización**: 2026-04-06
**Versión actual**: v0.14.0 ✅ FUNCIONAL
**Commit**: `df4ec17`
**Próxima versión**: v0.15.0 - Demos funcionales en Termux-X11 + v-shield platform layer
**Estado**: ry-backend ✅ | migui ry-backend ✅ | ry-system-ry ✅ | events-ry ✅ | ry-config ✅ | demo_torreta_vs_sprites ✅
**Crates**: 25 en workspace | 0 errores | 95+ tests pasando
**Crates publicados**: 2 (ry-god + ry-stream)
**ELF más nuevo**: demo_torreta_vs_sprites 434K release

---

### ✅ **LO QUE SÍ FUNCIONA (v0.14.0)**

| Sistema | Estado | Notas |
|---------|--------|-------|
| **ry-backend** | ✅ v0.1.0 | Dual backend: raylib + SDL2 TTF/input/audio |
| **migui** | ✅ ry-backend | Usa ry-backend (no sdl2 directo) |
| **ry-system-ry** | ✅ v0.14.0 | Sistema unificado: RySystem (core + gui) |
| **ry-config** | ✅ v0.1.0 | Config parser (entities, levels, checkpoints) |
| **events-ry** | ✅ v0.1.0 | Input unificado 3 capas + Sdl2InputBackend |
| **demo_torreta_vs_sprites** | ✅ 434K | JUEGO COMPLETO: menú + 3 niveles + cámara + AI + audio |
| **demo_menu_bar** | ✅ 330K | Menús Dear ImGui + mouse + touch |
| **demo_panel_visual** | ✅ 339K | 4 paneles + consola interactiva |
| **demo_rigidbody** | ✅ 446K | Sprites + física + colisiones + audio + TTF |
| **demo_anime_ry** | ✅ 341K | 6 escenas ry-anim |
| **demo_ttf_sprites** | ✅ 436K | TTF + sprites |
| **demo_platformer_completo** | ✅ | Plataformas + gravedad + salto |
| **demo_completo_sdl2** | ✅ | SDL2 completo |
| **demo_50k_particulas** | ✅ 313K | Partículas |
| **demo_colisiones** | ✅ 309K | Sistema colisiones |
| **Texto TTF** | ✅ Profesional | Anti-alias blended |
| **Mouse Events** | ✅ Completos | Click, doble click, derecho, scroll |
| **Touch Android** | ✅ Completo | FingerDown/Motion/Up |
| **Features** | ✅ 4 modos | raylib-only, sdl2-only, dual-backend, mobile-hybrid |
| **ry-anim** | ✅ v0.12.0 | 41 funciones, 58 tests |
| **ry-stream** | ✅ v0.1.0 crates.io | LAN streaming |
| **ry-god** | ✅ v0.1.0 crates.io | Security & Efficiency |
| **Math avanzado** | ✅ 33 funciones | pow, log, exp, PI, derivada, integral |
| **Arrays** | ✅ 16 funciones | push, pop, slice, contains, join... |
| **Vec2** | ✅ 22 operaciones | add, sub, normalize, dot, rotate... |
| **toolkit-ry** | ✅ v0.1.0 | 5 temas + 20+ widgets |
| **ry3d-gfx** | ✅ v0.1.0 | 15 funciones 3D |
| **Quest System** | ✅ 10 funciones | create, objectives, rewards... |
| **Save/Load** | ✅ 10 funciones | create, set_var, load, list... |
| **FSR 1.0** | ✅ Integrado | Shaders embebidos |
| **Parser** | ✅ Infalible | 6 bugs raíz resueltos |
| **SDL2_ttf** | ✅ Texto real | Texturas cacheadas |
| **Sprites PNG** | ✅ Cargados | SDL2_image |
| **Audio SDL2** | ✅ Tonos WAV | SDL2_mixer |
| **lizer** | ✅ 0.11.2 | AST cache real (FNV-1a, 256 entradas, LRU) |
| **ry-rs** | ✅ bin + lib | Antes solo bin |
| **Demos binarios** | ✅ ~33+ | src/bin/ |

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

## 📊 Lista de Crates (25)

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
| ry-rs | — | Main binary + lib |
| ry-system-ry | 0.14.0 | ✅ RySystem (core + gui) |
| ry-test | — | ⚠️ |
| toolkit-ry | 0.1.0 | ✅ |
| migui | — | ✅ ry-backend |
| blast-core | 0.1.0 | ✅ |
| lizer | 0.11.2 | ✅ AST cache real |
| v-shield | — | ⚠️ |
| ry3d-gfx | 0.1.0 | ✅ |
| events-ry | 0.1.0 | ✅ Input 3 capas |
| ry-backend | 0.1.0 | ✅ Dual backend |
| ry-config | 0.1.0 | ✅ Config parser |
| ~~ry-ecs~~ | — | 🗑️ Eliminado |

---

## 🎯 Roadmap

<div align="center">

| Versión | Estado | Features | Fecha |
|---------|--------|----------|-------|
| **v0.11.5** | ✅ | 0 Errores + lifetimes fixeados | 2026-04-02 |
| **v0.12.0** | ✅ | ry-anim v0.12.0 + Quest + Save/Load + ry-stream crates.io | 2026-04-05 |
| **v0.13.0** | ✅ | events-ry + Panel Visual + Demo Panel + Warnings fix | 2026-04-05 |
| **v0.14.0** | ✅ | ry-backend dual + migui ry-backend + ry-system-ry + ry-config + demo_torreta_vs_sprites | 2026-04-06 |
| **v0.15.0** | ⏳ | Demos Termux-X11 + v-shield platform layer + ry-stream v0.2.0 | Próxima versión |
| **v1.0.0** | ⏳ | Motor Completo + Editor Visual | Futuro |

</div>

### Features pendientes (v0.15.0+)

| Feature | Prioridad | Notas |
|---------|-----------|-------|
| Platform crate (abstracción multiplataforma) | 🔮 Futuro | |
| Soporte de emojis en TTF | 🔮 Futuro | |
| GIF animation | 🔮 Futuro | |
| GPU instancing (gpu_instancing.rs de ry-gfx) | 🔮 Futuro | |
| Features 3D paso a paso | 🔮 Futuro | |
| ry-stream v0.2.0 mDNS | 🟡 Media | |
| Editor visual | 🔮 Futuro | |
| LAZOS Python bridge async | 🔮 Futuro | |

---

## 📋 Tareas Pendientes

| Tarea | Esfuerzo | Prioridad |
|-------|----------|-----------|
| Demos funcionales en Termux-X11 | 8-12h | 🔴 Alta |
| v-shield platform layer | 15-20h | 🔴 Alta |
| ry-stream v0.2.0 (mDNS) | 8-12h | 🟡 Media |
| ry-physics N-cuerpos >2 | 10-15h | 🟡 Media |
| ry-backend v0.2.0 (optimizaciones) | 6-8h | 🟡 Media |
| Consola visual en ry-gfx | 3-4h | 🟡 Media |
| Platform crate (abstracción multiplataforma) | 15-20h | 🔮 Futuro |
| Soporte de emojis en TTF | 4-6h | 🔮 Futuro |
| GIF animation | 8-12h | 🔮 Futuro |
| GPU instancing (revisar gpu_instancing.rs) | 10-15h | 🔮 Futuro |
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

## Ry-Dit v0.14.0 - Dual Backend + Juego Completo + 25 Crates

*0 errores | 25 crates compilando | 95+ tests | 2 crates publicados*

**Proxima version: v0.15.0 - Demos Termux-X11 + v-shield platform layer**

</div>
