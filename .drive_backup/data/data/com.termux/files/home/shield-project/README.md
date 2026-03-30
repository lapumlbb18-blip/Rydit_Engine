# 🛡️ RyDit - Motor Simulador de Escenas 2D + Lenguaje de Scripting en Rust para Android/Termux

<div align="center">

![RyDit Engine Logo](screenshots/logo.png)

**"Construido sin prisa, madurado con paciencia"**

[![Version](https://img.shields.io/badge/version-v0.9.0-blue.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine)
[![Tests](https://img.shields.io/badge/tests-260%2B%20passing-green.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine)
[![Status](https://img.shields.io/badge/estado-v0.9.0--ready-orange.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![raylib](https://img.shields.io/badge/raylib-5.5-purple.svg)](https://www.raylib.com/)
[![Platform](https://img.shields.io/badge/platform-Android%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine/blob/main/LICENSE)

[📖 Documentación](#-documentación) • [🔧 Estado Actual](#-estado-actual) • [🎯 Roadmap](#-roadmap) • [📱 Construido en Android](#-construido-en-androidtermux)

</div>

---

## 🎯 ¿Qué es RyDit?

**RyDit** es un **motor simulador de escenas 2D con lenguaje de scripting** escrito en **Rust** con **raylib**, diseñado para ejecutarse nativamente en **Android/Termux**.

**Casos de Uso:**
- 🎮 Desarrollo de juegos 2D
- 🔬 Visualización científica
- 📚 Educación STEM
- 🎨 Animaciones interactivas
- 🧪 Simulaciones físicas
- 📊 Prototipado rápido

**Filosofía Actual:**
- 🐌 **Sin prisa** - Madurar cada feature antes de lanzar
- 🔧 **Bien hecho** - Calidad sobre cantidad
- 🏠 **Para nosotros** - No para la comunidad ni releases
- 📦 **Modular** - Usar RyditModule, no tantos crates

---

## ⚠️ ESTADO ACTUAL

### Puntuación: 10/10 ✅ (v0.9.5 - INPUT AVANZADO COMPLETADO)

**Última actualización**: 2026-03-30
**Versión actual**: v0.9.5 ✅ INPUT AVANZADO
**Próxima versión**: v0.10.0 - GPU INSTANCING + SHADERS
**Versión futura**: v0.10.1 - ECS INSPIRADO EN BEVY

**✅ COMPLETADO EN v0.9.5**:
- ✅ **Input Map** - 8 funciones + combinaciones (Ctrl+S, Alt+Enter, etc.)
- ✅ **Input IME** - 9 funciones (teclado virtual Android)
- ✅ **Gamepad** - 5 funciones (A, B, X, Y, LB, RB, Start, Back, sticks)
- ✅ **100+ teclas mapeadas** - Todas las teclas estándar

**✅ COMPLETADO EN v0.9.4**:
- ✅ **Level Manager** - 13 funciones (load, unload, transition, checkpoints)
- ✅ **Tilemap System** - 12 funciones (create, fill_rect, draw, get_tile)
- ✅ **Collision System** - 13 funciones (AABB, Area2D, resolve)
- ✅ **Window Manager** - 17 funciones (title, size, fullscreen, vsync, fps)
- ✅ **Entity System** - 50+ funciones (player, enemy, boss, trap, coin)
- ✅ **Cámara 2D** - 15 funciones (follow, zoom, scroll, bounds)
- ✅ **Physics 2D** - 20 funciones (gravedad, fricción, colisión, respuesta)

**⚠️ LIMITACIONES RESTANTES (v0.9.5)**:
- ⚠️ **Arquitectura** - Script manda sobre Core → v0.10.1: Core manda, Script configura
- ⚠️ **GPU Instancing** - Sin FFI OpenGL → v0.10.0: 100K+ partículas

**🔥 PLAN CRÍTICO v0.10.0**:
- 🔥 **GPU Instancing** - FFI OpenGL (gl-rs), shaders GLSL
- 🔥 **Shaders** - Vertex + Fragment shaders
- 🔥 **100K+ partículas** - glDrawArraysInstanced

**🛡️ ARQUITECTURA v0.10.1** (INVERSIÓN DE CONTROL):
- 🛡️ Core manda - rydit-rs hace game loop nativo
- 🛡️ Script configura - .rydit solo parámetros
- 🛡️ ECS Entt - 100K+ entidades
- 🛡️ Comando nativo de RyDit: `./rydit-rs --scene <nombre>`

**🔥 EN PROCESO EN v0.10.0** (SOLO DESPUÉS DE v0.9.2-v0.9.5):
- # en proceso: **GPU Instancing** - 100K+ partículas @ 60 FPS
- # en proceso: **Shaders GLSL** - Vertex + Fragment shaders
- # en proceso: **FFI OpenGL** - gl-rs crate
- # en proceso: **ECS (ENTT)** - Entity Component System

**✅ COMPLETADO EN v0.8.7**:
- ✅ **HTTP + WebSocket** - 10 funciones (ureq + tungstenite compilados)
- ✅ **CSV Data Science** - 13 funciones (read, write, filter, join, aggregate)
- ✅ **Input Map** - 8 funciones (press, release, is_pressed, get_active)

**✅ COMPLETADO EN v0.8.5**:
- ✅ Audio Module - 12 funciones
- ✅ Particles Module - 5 efectos
- ✅ Config Termux-X11

**📊 MÉTRICAS**:
- ✅ 260+ tests passing
- ✅ 0 warnings clippy
- ✅ ~2.2 MB binario release
- ✅ 60 FPS estables (con Render Queue + Entity System)
- ✅ 170+ funciones para juegos 2D

**🔍 DIAGNÓSTICO TÉCNICO**:
- ✅ Level Manager (13 funciones)
- ✅ Tilemap System (12 funciones)
- ✅ Collision System (13 funciones)
- ✅ Window Manager (17 funciones)
- ✅ Entity System (50+ funciones)
- ✅ Cámara 2D (15 funciones)
- ✅ Physics 2D (20 funciones)
- ✅ Render Queue integrada
- ✅ Assets con queue (2000 sprites @ 60 FPS)
- ✅ Teclado completo (100+ teclas)
- ✅ Input Map acciones (nativo de RyDit)
- ✅ IME integrado (teclado virtual Android)
- ⚠️ Arquitectura: Script manda (v0.10.2: Core manda)

**📋 DOCUMENTACIÓN CRÍTICA**:
- [docs/RYDIT_V0.9.4_COMPLETADA.md](docs/RYDIT_V0.9.4_COMPLETADA.md) - v0.9.4 completa
- [docs/LEVEL_MANAGER_V0.9.4.md](docs/LEVEL_MANAGER_V0.9.4.md) - Level Manager
- [docs/ANALISIS_HONESTO_Y_PLAN_MAESTRO_V0.9.x.md](docs/ANALISIS_HONESTO_Y_PLAN_MAESTRO_V0.9.x.md) - Limitaciones + plan

---

## 🛡️ ARQUITECTURA v0.10.0: INVERSIÓN DE CONTROL

### 🔍 Problema Detectado

**Arquitectura actual (INCORRECTA)**:
```
Usuario → demo.rydit → Parser (secretaria) → Intenta cargar todo
                    ↓
            "No tengo fuerza para cosas pesadas"
```

**Síntomas**:
- ⚠️ Parser `lizer` sobrecargado (3K líneas haciendo trabajo de core)
- ⚠️ `main.rs` solo 4K líneas (muy poco para ser core)
- ⚠️ Scripts `.rydit` intentan dibujar directo (FFI desde script)
- ⚠️ Inestabilidad con lógica compleja
- ⚠️ 2000 partículas colapsan el evaluator

**Comparativa con motores profesionales**:

| Motor | Core (C++) | Scripting | VM/Interpreter | Ratio Core/Script |
|-------|------------|-----------|----------------|-------------------|
| **Godot** | ~500K | GDScript | ~50K | 10:1 |
| **Unity** | ~1M+ | C# | ~200K | 5:1 |
| **Unreal** | ~5M+ | C++/BP | ~100K | 50:1 |
| **RyDit** | ~4K | .rydit | ~3K | **1.3:1** ⚠️ |

### ✅ Solución: Inversión de Control

**Nueva arquitectura (CORRECTA)**:
```
┌──────────────────────────────────────┐
│  rydit-rs (CORE en Rust)             │ ← MANDA EL CORE
│    → Game loop nativo                │
│    → Render Queue nativa             │
│    → ECS nativo (Entt)               │
│    → GPU Instancing nativo           │
│    → Físicas nativas                 │
└──────────────────────────────────────┘
           ↓ carga configuración
┌──────────────────────────────────────┐
│  tank_demo.rydit (CONFIG)            │ ← Solo parámetros
│    → Entidades a spawnear            │
│    → Valores de física               │
│    → Scripts de comportamiento       │
└──────────────────────────────────────┘
```

**Comando actual (roto)**:
```bash
./rydit-rs --gfx demos/test.rydit
# ↑ El .rydit tiene que hacer TODO
```

**Comando correcto (nativo/Python)**:
```bash
./rydit-rs --run tank_demo
# ↑ El core carga la demo, .rydit es solo config
```

**Ejemplo de .rydit como configuración**:
```rydit
# tank_demo.rydit - Solo configuración
# NO dibuja, NO hace render
# Solo dice QUÉ cargar

entidad "jugador" {
    sprite: "tank.png"
    x: 400
    y: 300
    fisica: true
    script: "tank_control.rydit"
}

mundo {
    gravedad: 9.8
    fondo: "cielo.png"
}
```

### 📋 Plan de Implementación v0.10.0

| Tarea | Descripción | Impacto |
|-------|-------------|---------|
| **1. Mover game loop a Rust** | `executor.rs` hace loop nativo | Core estable 60 FPS |
| **2. .rydit como configuración** | Solo datos, no lógica pesada | Parser no se satura |
| **3. GPU Instancing en rydit-gfx** | FFI OpenGL nativo | 100K partículas |
| **4. ECS Entt en Rust** | Componentes nativos | 100K entidades |
| **5. Shaders GLSL nativos** | Vertex + Fragment en Rust | Render masivo |
| **6. Comando nativo de RyDit** | `./rydit-rs --scene <nombre>` | Arquitectura correcta |

### 🎯 Comparativa: Antes vs Después

| Aspecto | Antes (v0.9.x) | Después (v0.10.0) |
|---------|---------------|-------------------|
| **Core** | 4K líneas | 20K+ líneas |
| **Script** | Hace todo | Solo configura |
| **Parser** | 3K líneas sobrecargadas | 3K líneas ligeras |
| **Render** | FFI desde script | Nativo en Rust |
| **Partículas** | 500 @ 15 FPS | 100K @ 60 FPS |
| **Entidades** | Limitadas por script | ECS nativo (100K+) |
| **Estabilidad** | Inestable con complejidad | Estable siempre |
| **Comando** | `--gfx demo.rydit` | `--run demo_name` |

---

## 🔧 PRÓXIMAS FASES DE MADURACIÓN

### Fase 1: GPU Instancing + Shaders (v0.10.0) # en proceso
- [ ] # en proceso: FFI OpenGL (`gl-rs` crate) en rydit-gfx
- [ ] # en proceso: Shaders GLSL vertex + fragment en `rydit-gfx/shaders/`
- [ ] # en proceso: `glDrawArraysInstanced()` básico
- [ ] # en proceso: Demo: 100,000+ partículas @ 60 FPS
- [ ] # en proceso: Ubicación: `crates/rydit-gfx/src/gpu_instancing.rs`

### Fase 2: ECS - Entity Component System (v0.10.1) # en proceso
- [ ] # en proceso: Crate nuevo: `crates/rydit-ecs/`
- [ ] # en proceso: ENTT o bevy_ecs
- [ ] # en proceso: Components: Position, Velocity, Sprite
- [ ] # en proceso: Systems: Movement, Render, Physics
- [ ] # en proceso: Integración en executor.rs

### Fase 3: Integración GPU + ECS (v0.10.2) # en proceso
- [ ] # en proceso: executor.rs usa ECS + GPU Instancing
- [ ] # en proceso: Crear exodo_gpu.rs (100K+ partículas)
- [ ] # en proceso: .rydit llama a binarios .rs

### Fase 4: Optimización Render Queue (v0.9.2) ⚠️ Pendiente
- [ ] Separar por tipo (círculos, rects, líneas)
- [ ] Mejor batching interno
- [ ] Posible: 2000 partículas @ 60 FPS

### Fase 5: N-Body Gravity (v0.10.3) ⚠️ Pendiente
- [ ] N-body gravity simulation
- [ ] 100,000+ entities estables
- [ ] Integración con ECS + GPU

### Fase 6: Fluid Dynamics (v0.10.4) ⚠️ Pendiente
- [ ] SPH (Smoothed Particle Hydrodynamics)
- [ ] Fluid surface simulation
- [ ] Wave dynamics

### Fase 7: Parser Maduro (v1.0.0) ⚠️ Pendiente
- [ ] Refactorizar `lizer/src/lib.rs` completo
- [ ] Paréntesis que funcionen SIEMPRE
- [ ] Expresiones complejas sin dolor
- [ ] Arrays multidimensionales reales

### Fase 4: CSV + Data Science ✅ COMPLETADO
- [x] `crates/rydit-rs/src/modules/csv.rs` - 13 funciones
- [x] `csv::read()`, `csv::write()` - File I/O
- [x] `csv::to_json()`, `csv::from_json()` - Conversión
- [x] `csv::filter()`, `csv::columns()`, `csv::row_count()`, `csv::col_count()`
- [x] `csv::join()`, `csv::group_by()`, `csv::aggregate()` - Operaciones avanzadas

### Fase 5: Audio + HTTP + Entity System ✅ COMPLETADO
- [x] Módulos (NO crates nuevos)
- [x] `audio::beep()`, `audio::play()` - 12 funciones
- [x] `http::get()`, `http::post()`, `http::put()`, `http::delete()` - 4 funciones
- [x] `ws::connect()`, `ws::send()`, `ws::recv()`, `ws::disconnect()` - 6 funciones
- [x] Crate `rydit-http` compilado exitosamente
- [x] **Entity System** - 63 funciones (player, enemy, boss, trap, coin)
- [x] **Cámara 2D** - 15 funciones
- [x] **Collision System** - 5 funciones
- [x] **Area2D System** - 6 funciones (propio de RyDit)

### Fase 6: LAZOS Maduro (1 semana)
- [ ] Unificar `evaluar_expr()`
- [ ] Protocolo universal funcionando

---

## 📊 POTENCIAL

| Estado | Score |
|--------|-------|
| Actual | 9.5/10 ✅ |
| Fase 1 (Parser) | 7/10 |
| Fase 2 (Assets) | 8/10 |
| Fase 3 (Particles) | 8.5/10 |
| Fase 4 (CSV) | 9/10 ✅ COMPLETADO |
| Fase 5 (Audio+HTTP) | 9.5/10 ✅ COMPLETADO |
| Fase 6 (LAZOS) | **9.5/10** ✅ |

---

## 🚫 LO QUE NO HAREMOS

- ❌ NO git push hasta que esté maduro
- ❌ NO publicar en crates.io
- ❌ NO release público
- ❌ NO prisa por terminar

---

## ✅ LO QUE SÍ HAREMOS

- ✅ Trabajar sin presión
- ✅ Cada feature bien hecha
- ✅ Tests reales (no solo que pasen)
- ✅ Demos complejos (no simplificados)
- ✅ Código del que estemos orgullosos

---

## 📖 Documentación

| Documento | Descripción |
|-----------|-------------|
| [ESTRUCTURA.md](ESTRUCTURA.md) | ⚠️ Estado real sin filtros |
| [QWEN.md](QWEN.md) | 📓 Bitácora de sesión |
| [PLANIFICACION_V0.5.1_AUDIO_HTTP.md](PLANIFICACION_V0.5.1_AUDIO_HTTP.md) | 📋 Plan detallado |
| [ESTADO_DEL_CODIGO_V0.8.4.md](ESTADO_DEL_CODIGO_V0.8.4.md) | 📊 Revisión completa |

---

## 🚀 Roadmap

<div align="center">

| Versión | Estado | Features | Fecha |
|---------|--------|----------|-------|
| **v0.9.0** | ✅ | 3 Capas Críticas (Command Queue, Double Buffer, Platform Sync X11) | 2026-03-28 |
| **v0.9.1** | ✅ | Render Queue Integrada con Evaluator | 2026-03-29 |
| **v0.9.2** | ✅ | Assets Queue + Teclado + Input Map + IME | 2026-03-29 |
| **v0.9.3** | ✅ | Físicas Respuesta + Camera Apply | 2026-03-29 |
| **v0.9.4** | 🔥 | Bloques sin límite + Text Input real | 1 semana |
| **v0.9.5** | 🔮 | Platform Sync Multi-Plataforma | 2-3 semanas |
| **v0.10.0** | # en proceso | GPU Instancing + Shaders GLSL (100K+ partículas) | **Después de v0.9.5** |
| **v0.10.1** | # en proceso | ECS (Entity Component System - ENTT) | **Después de v0.10.0** |
| **v0.10.2** | 🔮 | Integración GPU + ECS | Futuro |
| **v0.10.3** | 🔮 | N-Body Gravity | Futuro |
| **v0.10.4** | 🔮 | Fluid Dynamics (Éxodo 14) | Futuro |
| **v1.0.0** | 🔮 | Simulador de Escenas Completo (Multi-plataforma) | Futuro |

</div>

**NOTA**: GPU Instancing (v0.10.0) solo después de completar v0.9.2-v0.9.5 (bases sólidas).

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

## 🛡️ RyDit v0.5.0 - En Maduración

*Sin prisa | Sin releases | Solo código bien hecho*

**Potencial: 9.5/10**

</div>
- 🎮 Game loop integrado
- 🎨 Renderizado gráfico (círculos, rectángulos, líneas, texto, sprites)
- 🎹 Input de teclado en tiempo real
- 🎲 Sistema de módulos (math, arrays, strings, io, random, time, json, colisiones, regex, files)
- 🧪 **163 tests automáticos**
- 🎨 **12 widgets UI** (migui - immediate mode GUI)
- 🎵 **Audio** (sonidos + música)
- ✨ **Sistema de partículas** (fuego, humo, explosión, lluvia, chispas)
- 💻 **REPL interactivo** con historial y auto-completado
- 📦 **Stdlib embebido** (sin archivos externos necesarios)
- 🎨 **Animación 2D** (10 funciones, 12 principios de Disney - 3 implementados)
- 📐 **Curvas de Bezier** (6 funciones, algoritmo de De Casteljau)
- ⚛️ **Física 2D** (proyectiles, gravedad, ondas, péndulo)
- 📊 **Ciencia de Datos** (CSV, estadísticas, gráficos ASCII/SVG)
- 🔌 **Arquitectura modular** (v0.8.2+: Sistema Universal Ry)
- 🔗 **Protocolo LAZOS** (comunicación universal con Python, web, etc.)
- 🔥 **Carga dinámica de módulos** (.so/.dll + scripts .rydit)
- 🔥 **Hot reload** de módulos en runtime
- 🐍 Snake Game completo como demo

```rydit
# Tu primer juego en RyDit (3 líneas)
shield.init
ryda frame < 1000 {
    draw.circle(400, 300, 50, "rojo")
}
```

| Característica | RyDit | Godot | Love2D | PICO-8 |
|---------------|-------|-------|--------|--------|
| **Android Native** | ✅ Sí (Termux) | ❌ No | ❌ No | ❌ No |
| **Lenguaje** | RyDit (Español) | GDScript | Lua | Lua |
| **Backend** | Rust | C++ | C | C |
| **Binario** | ~550 KB | ~50 MB | ~10 MB | ~5 MB |
| **Sin IDE** | ✅ Sí | ❌ Requiere editor | ⚠️ VS Code | ⚠️ Editor propio |
| **Game Loop** | ✅ Integrado | ✅ Integrado | ✅ Integrado | ✅ Integrado |
| **Partículas** | ✅ 5 efectos | ✅ Sí | ⚠️ Librerías | ❌ Limitado |
| **Audio** | ✅ Sonidos + Música | ✅ Sí | ✅ Sí | ✅ Sí |
| **UI Widgets** | ✅ 12 (migui) | ✅ Sí | ⚠️ Librerías | ❌ No |

---

## 🛡️ MANIFIESTO

> **"David vs Goliat - Un motor de videojuegos en Rust, construido 100% en un Redmi Note 8"**

### Filosofía RyDit

1. **Mobile-First Real** - No "también funciona en Android". **Nació en Android**.
2. **Ligero y Portable** - Binario de ~550 KB (no 50 MB propio de RyDit)
3. **Educativo** - Código 100% abierto, lenguaje en español, sin magia
4. **David vs Goliat** - 12,000 líneas de Rust bien escritas > 500,000 líneas de C++
5. **Rendimiento Estable** - Sin calentamiento, RAM <100 MB, 60 FPS estables
6. **Portabilidad** - Próximamente Linux + Windows + WebAssembly

### ¿Por qué existe RyDit?

**El Problema:**
- Barrera de entrada alta: Godot/Unity requieren PC potente
- Android es ciudadano de segunda: "Funciona en Android" ≠ "Nació en Android"
- Herramientas sobredimensionadas: 90% de features que no usas
- Inglés como requisito: Documentación y sintaxis solo en inglés

**La Solución RyDit:**
✅ **Termux como plataforma primera** - No emulación, nativo
✅ **Binario <1 MB** - Cabe en cualquier lado
✅ **12 widgets, 8 módulos stdlib** - Lo justo y necesario
✅ **Español nativo** - `draw.circulo()`, `si`, `mientras`
✅ **Sin IDE** - `cargo run` y listo

### Valores

- **Código > Burocracia** - Preferimos código funcionando a documentación perfecta
- **Mobile-First** - Si no funciona en Android, no es RyDit
- **Ligero** - Cada KB cuenta. Sin dependencias innecesarias
- **Abierto** - MIT license. 100% transparente
- **Educativo** - Enseñamos, no solo damos herramientas
- **Comunidad** - Hispanohablantes, mobile developers, hobbyists

---

## 🔌 Arquitectura Modular (v0.7.3.x+)

**Filosofía:** Núcleo estable + módulos extensibles (Manim + Bevy style).

**Arquitectura de Referencia:**
- 🎬 **Manim** (3Blue1Brown) - Escenas matemáticas, animaciones científicas
- 🎮 **Bevy** (Rust) - ECS moderno, components, systems

```
┌─────────────────────────────────────────────────────────┐
│  RYDIT - ARQUITECTURA MANIM + BEVY                      │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  CAPA 1: NÚCLEO ESTABLE (NO TOCAR)               │   │
│  │  - main.rs (game loop, rydit-gfx FFI)            │   │
│  │  - eval/mod.rs (evaluar_expr)                    │   │
│  │  - repl.rs (REPL interactivo)                    │   │
│  │  - lazos.rs (protocolo LAZOS)                    │   │
│  │  Tamaño: ~5,000 líneas (ESTABLE)                 │   │
│  └─────────────────────────────────────────────────┘   │
│                         │                               │
│  ┌──────────────────────▼──────────────────────────┐   │
│  │  CAPA 2: CRATES INDEPENDIENTES (PUBLICABLES)     │   │
│  │                                                   │   │
│  │  v0.7.3.0 🔷 crates/rydit-core/                  │   │
│  │     ├── RyditModule (trait)                      │   │
│  │     ├── ModuleRegistry                           │   │
│  │     └── ModuleError/Result                       │   │
│  │     Tests: 4 passing ✅                          │   │
│  │                                                   │   │
│  │  v0.7.3.1 🔬 crates/rydit-science/               │   │
│  │     ├── Bezier (linear, quadratic, cubic)        │   │
│  │     └── Stats (mean, median, min, max)           │   │
│  │     Tests: 9 passing ✅                          │   │
│  │                                                   │   │
│  │  v0.7.3.2 ⚛️  crates/rydit-physics/               │   │
│  │     ├── Projectile (trayectoria, altura)         │   │
│  │     └── NBody (gravedad 2 cuerpos)               │   │
│  │     Tests: 6 passing ✅                          │   │
│  │                                                   │   │
│  │  v0.7.3.3 🎨 crates/rydit-anim/                  │   │
│  │     ├── Easing (ease_in, ease_out, ease_in_out)  │   │
│  │     ├── Squash & Stretch                         │   │
│  │     └── Anticipation                             │   │
│  │     Tests: 9 passing ✅                          │   │
│  │                                                   │   │
│  │  v0.7.3.x 👁️  crates/rydit-geometry/ (PENDIENTE)   │   │
│  │     ├── Penrose Triangle                         │   │
│  │     ├── Impossible Cube                          │   │
│  │     └── Optical Spirals                          │   │
│  │                                                   │   │
│  │  Cada crate: ~150-330 líneas                     │   │
│  │  Independiente, testeable, publicable crates.io  │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  CAPA 3: APLICACIÓN PRINCIPAL                    │   │
│  │  - crates/rydit-rs/ (binario)                    │   │
│  │  - Protocolo LAZOS (stdin/stdout JSON-RPC)       │   │
│  │  - Tests: 53 passing ✅                          │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

**Ventajas:**
- ✅ Núcleo estable (no se rompe)
- ✅ Crates independientes (testing fácil)
- ✅ Comunidad puede crear módulos
- ✅ Publicables a crates.io
- ✅ Manim + Bevy = Arquitectura probada

**Versionamiento Granular:**
```
v0.MAJOR.MINOR.PATCH
v0.7.3.0 → rydit-core (trait + registry)
v0.7.3.1 → rydit-science (Bezier + Stats)
v0.7.3.2 → rydit-physics (Projectile + NBody)
v0.7.3.3 → rydit-anim (Easing + Squash/Stretch)
v0.7.3.x → rydit-geometry (pendiente)
v0.8.0.0 → Publicación crates.io + Linux/Windows
```

---

## 🎮 Snake Game - Demo Funcional

<div align="center">

![Snake Game](screenshots/04_snake_gameplay.jpg)

*Snake Game completo con game loop, colisiones, puntuación y game over screen*

</div>

### Características del Snake
- ✅ **Cuerpo de serpiente** con arrays dinámicos
- ✅ **Comida aleatoria** con `random::int()`
- ✅ **Colisiones** con paredes y propio cuerpo
- ✅ **Puntuación** + high score
- ✅ **Velocidad progresiva**
- ✅ **Game Over** + restart con SPACE
- ✅ **Pausa** con P
- ✅ **Salir** con ESC

### Ejecutar Snake
```bash
# En Termux (Android)
cargo run --bin rydit-rs -- --gfx snake.rydit

# O con binario directo
./target/release/rydit-rs --gfx snake.rydit
```

### Controles
| Tecla | Acción |
|-------|--------|
| `↑` `→` `↓` `←` | Mover serpiente |
| `P` | Pausa |
| `SPACE` | Reiniciar |
| `ESC` | Salir |

---

## 🎨 Demo Visual - Formas y Colores

<div align="center">

![Demo Shapes](screenshots/03_demo_shapes_circulos.jpg)

*Demo de formas geométricas con draw.circle(), draw.rect(), draw.line(), draw.text()*

</div>

---

## 📸 Galería de Capturas

<div align="center">

### 🎮 Tank Combat Demo v0.3.0

| Tank Combat | Tanque con Torreta |
|--------------|--------------|
| ![Tank Combat](screenshots/demotank.jpg) | ![Tank Torreta](screenshots/demotank2.jpg) |
| Tanque verde con seguimiento de mouse | Torreta rotando hacia el objetivo |

| Campo de Batalla |
|--------------|
| ![Tank Battlefield](screenshots/demotank3.jpg) |
| Grid táctico con balas y colisiones |

---

### 🖥️ Migui GUI v0.4.1 - Immediate Mode GUI

| Migui Backend |
|--------------|
| ![Migui GUI](screenshots/demomigui.jpg) |
| **Immediate Mode GUI con backend raylib** - Botones, slider, checkbox, textbox, ventana arrastrable |

---

### 🤖 Rybot Asistente

| Rybot Interface |
|--------------|
| ![Rybot](screenshots/rybot.jpg) |
| **Asistente de código RyDit** - Menú de comandos y ayuda interactiva |

---

### 🐍 Snake Game

| Snake Gameplay |
|--------------|
| ![Snake](screenshots/04_snake_gameplay.jpg) |
| Snake en movimiento, grid retro, comida roja |

---

### ✨ Sistema de Partículas v0.5.3

| Demo Partículas |
|--------------|
| ![Partículas Demo](screenshots/particulas.jpg) |
| **5 efectos: fuego, humo, explosión, lluvia, chispas** - 60 FPS con 500+ partículas |

**Video completo:** [Ver video de partículas](screenshots/particulas.mp4)

**Controles del Demo:**
- `F` - Fuego
- `S` - Chispas
- `H` - Humo
- `E` - Explosión
- `ESC` - Salir

---

### 🎨 Carga de Sprites v0.5.1

| Tanque + Helicóptero |
|--------------|
| ![Carga Sprite](logo_icon_asst/carga_sprite.jpg) |
| **Assets Manager funcionando** - Tanque y helicóptero con sprites 16x16 escalados |

**Funciones de Assets:**
```rydit
assets::load_texture("tank", "sprites/tank.png")
assets::draw("tank", x, y)
assets::draw_scaled("tank", x, y, 4.0)
```

</div>

**Todas las capturas fueron tomadas en un Redmi Note 8 con Termux-X11 + raylib** 📱🎮

```rydit
shield.init

ryda frame < 500 {
    draw.circle(400, 200, 80, "rojo")
    draw.rect(200, 350, 60, 60, "naranja")
    draw.line(100, 500, 300, 500, "blanco")
    draw.text("Demo RyDit", 250, 50, "amarillo")
}
```

---

## 📖 Sintaxis del Lenguaje

### Funciones Básicas
```rydit
rytmo saludar {
    voz "Hola Mundo"
    return 1
}

saludar()
```

### Funciones con Parámetros
```rydit
rytmo saludar(nombre) {
    voz "Hola " + nombre
}

saludar("Mundo")
```

### Condicionales
```rydit
dark.slot x = 10
onif x > 5 voz "Mayor" blelse voz "Menor"
```

### Ciclos
```rydit
dark.slot x = 3
ryda x {
    voz x
    dark.slot x = x - 1
}
```

### Arrays
```rydit
# Array básico
dark.slot lista = [1, 2, 3]

# Multidimensional (tablero)
dark.slot tablero = [[0, 0, 0], [0, 0, 0], [0, 0, 0]]

# Asignación por índice
dark.slot lista[0] = 5
```

### Sistema de Módulos
```rydit
# Importar módulos
import math
import arrays
import strings

# Usar funciones con namespace
dark.slot suma = math::sumar(10, 3)
dark.slot len = arrays::length([1, 2, 3])
dark.slot upper = strings::upper("hola")
```

### Gráficos (Modo Ventana)
```rydit
shield.init

# Dibujar formas
draw.circle(400, 300, 50, "rojo")
draw.rect(100, 100, 100, 100, "verde")
draw.line(0, 0, 800, 600, "azul")
draw.text("RyDit v0.1.9", 300, 50, "blanco")
```

---

## 🏗️ Arquitectura

```
┌─────────────────────────────────────────────────────────┐
│  RyDit Core (Rust)                                      │
│  ├── lizer       → Lexer + Parser + AST (~2,452 líneas) │
│  ├── blast-core  → Executor + Memoria (~465 líneas)     │
│  ├── rydit-gfx   → Gráficos raylib (~481 líneas)        │
│  ├── rydit-rs    → Binario + stdlib (~2,491 líneas)     │
│  └── v-shield    → Wrapper raylib (~120 líneas)         │
└─────────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────┐
│  Scripts RyDit (.rydit)                                 │
│  ├── Snake Game                                         │
│  ├── Demos visuales                                     │
│  ├── Módulos stdlib (math, arrays, strings, io, etc.)   │
│  └── Juegos de la comunidad                             │
└─────────────────────────────────────────────────────────┘
```

### Métricas del Proyecto (v0.6.0)
```
Líneas totales:     ~12,000 líneas
├── Rust:           ~9,500 líneas
└── RyDit:          ~2,500 líneas (demos + módulos + tests)

Tests automáticos:  126 pasando ✅
Demos funcionales:  19 principales ✅
Warnings activos:   0 ✅

Binarios:
├── rydit-rs:       ~550 KB (release, strip = true)
├── rydit-rs:       ~920 KB (debug)
└── demo_particles: ~560 KB

Crates:
├── lizer:          Lexer + Parser + AST (~2,452 líneas)
├── blast-core:     Executor + Memoria (~465 líneas)
├── rydit-gfx:      Gráficos + Partículas + Audio (~680 líneas)
├── rydit-rs:       Binario + stdlib + REPL (~3,662 líneas)
├── v-shield:       Wrapper raylib (~120 líneas)
├── migui:          Immediate Mode GUI (~600 líneas)
└── modules:        Stdlib embebido (8 módulos ~800 líneas)
```

---

## 📱 Construido en Android/Termux

<div align="center">

| Setup | Especificaciones |
|-------|-----------------|
| **Dispositivo** | Redmi Note 8 |
| **OS** | Android 11 |
| **Terminal** | Termux |
| **RAM** | 4 GB |
| **Almacenamiento** | 64 GB |
| **IDE** | Ninguno (vim/nano) |
| **Teclado** | Pantalla táctil + Bluetooth |

</div>

**Este proyecto fue construido 100% en un teléfono Android**, sin laptop, sin desktop, sin IDE. Solo:
- 📱 Teléfono Redmi Note 8
- ⌨️ Terminal Termux
- 🦀 Rust + Cargo
- 🎨 Raylib (nativo)

**Filosofía:** Demostrar que el desarrollo serio es posible en dispositivos móviles cuando tienes arquitectura clara, tests automatizados, buena documentación y determinación.

---

## 🚀 Roadmap

<div align="center">

| Versión | Estado | Features Principales | Fecha |
|---------|--------|---------------------|-------|
| **v0.0.1-v0.0.14** | ✅ | CLI → Snake Game | 2026-03-14 a 2026-03-16 |
| **v0.1.0** | ✅ | Snake Game Completo | 2026-03-17 |
| **v0.1.1** | ✅ | Sistema de Módulos | 2026-03-17 |
| **v0.1.4** | ✅ | Strings + IO + Arrays | 2026-03-18 |
| **v0.1.6** | ✅ | Random + Time | 2026-03-18 |
| **v0.1.8** | ✅ | Maduración + Gráficos | 2026-03-20 |
| **v0.1.9** | ✅ | **110 Tests Checkpoint** | 2026-03-20 |
| **v0.2.0** | ✅ | Module System Avanzado + CI/CD | 2026-03-21 |
| **v0.3.0** | ✅ | Tank Combat + Colisiones + Math | 2026-03-21 |
| **v0.4.0** | ✅ | **migui** (Immediate Mode GUI ~600 líneas) | 2026-03-22 |
| **v0.4.1** | ✅ | **migui backend raylib** (renderizado real) | 2026-03-22 |
| **v0.5.0** | ✅ | **Ecosistema Maduro** (dropdown, progress bar, assets manager) | 2026-03-23 |
| **v0.5.1** | ✅ | **Funciones Assets + Renderizado X11** (sprites en RyDit, fix zink) | 2026-03-23 |
| **v0.5.2** | ✅ | **Audio + ListBox + Layout** (sonidos, música, UI mejorada) | 2026-03-23 |
| **v0.5.3** | ✅ | **REPL Interactivo + Partículas** (historial, sistema partículas) | 2026-03-23 |
| **v0.6.0** | ✅ | **Fix Termux-X11 + Stdlib Embebido** (auto-config, 8 módulos) | 2026-03-23 |
| **v0.6.1** | ✅ | **Limpieza repositorio + Video partículas** (README, galería) | 2026-03-24 |
| **v0.6.2** | ✅ | **Módulo REGEX** (match, replace, split, find_all, capture) | 2026-03-24 |
| **v0.6.3** | ✅ | **Módulo FILES** (read, write, append, exists, delete) | 2026-03-24 |
| **v0.6.4** | ✅ | **cargo fmt + Evaluación Split** (código consistente) | 2026-03-24 |
| **v0.7.0** | ✅ | **Split PARCIAL** (REPL + eval extraídos, -17% main.rs) | 2026-03-24 |
| **v0.7.0.bis** | ✅ | **Clippy + RyditModule diseño** (55→6 warnings, documentación) | 2026-03-24 |
| **v0.7.1.1** | ✅ | **ANIMACIÓN 2D** (10 funciones, 12 principios Disney, 4 ilusiones) | 2026-03-24 |
| **v0.7.1.2** | 🔜 | **Módulo RED** (HTTP, WebSocket, TCP/UDP) | Próxima sesión |
| **v0.7.1.3** | 🔮 | **Módulo DATOS** (CSV, HDF5, plots, statistics) | 2-3 semanas |
| **v0.7.2.0** | ✅ | **Protocolo LAZOS** (stdin/stdout JSON-RPC, Python bridge) | 2026-03-25 |
| **v0.7.3.0** | ✅ | **SPLIT PROGRESIVO** (RyditModule trait + registry) | 2026-03-26 |
| **v0.7.3.1** | ✅ | **rydit-science** (Bezier + Stats extraídos) | 2026-03-26 |
| **v0.7.3.2** | ✅ | **rydit-physics** (Projectile + NBody extraídos) | 2026-03-26 |
| **v0.7.3.3** | ✅ | **rydit-anim** (Easing + Squash/Stretch extraídos) | 2026-03-26 |
| **v0.8.0** | ✅ | **Sistema Universal Ry** (módulos dinámicos, hot reload) | 2026-03-26 |
| **v0.8.2** | ✅ | **Sistema Universal Ry** (Fases 1-4 completadas) | 2026-03-26 |
| **v0.8.3** | ✅ | **Fix Warnings** (50→~15 warnings, -70%) | 2026-03-26 |
| **v0.8.4** | ✅ | **FIX GRÁFICOS** (draw.text con expresiones, game loop estable) | 2026-03-26 |
| **v0.5.0** | ✅ | **RELEASE ESTABLE** (7 demos funcionales, 157 tests) | 2026-03-26 |
| **v0.5.1** | 🔜 | **Audio + HTTP + CSV + Assets + Partículas** | Próxima sesión |
| **v0.7.3.x** | ⏳ | **rydit-geometry** (Ilusiones ópticas, pendiente) | Próxima sesión |
| **v0.8.0.0** | 🔮 | **Publicación crates.io + Linux/Windows** | 4-6 semanas |
| **v1.0.0** | 🔮 | Production Ready | 6-8 meses |

</div>

---

## 🎯 Estado del Proyecto

### ✅ Completado (v0.6.0)
- [x] Lexer + Parser con AST
- [x] Executor con memoria y scopes
- [x] Sistema de módulos (import)
- [x] 45+ tests automáticos (core sin gráficos)
- [x] 16 benchmarks
- [x] Snake Game completo
- [x] Gráficos con raylib
- [x] Strings, IO, Arrays maduros
- [x] Soporte JSON (`json::parse()`, `json::stringify()`)
- [x] Random + Time ligeros
- [x] UTF-8 completo
- [x] Escapes en strings
- [x] Símbolos en identificadores
- [x] Tank Combat + colisiones
- [x] **migui** (Immediate Mode GUI ~600 líneas)
- [x] **migui backend raylib** (renderizado real 60 FPS)
- [x] **Funciones Assets** - `assets::load_texture()`, `assets::draw()`, `assets::draw_scaled()`
- [x] **Demo Assets** - Tanque + Helicóptero con sprites
- [x] **Fix Renderizado Termux-X11** - Variables zink, frame variable, evaluar_expr_gfx
- [x] **Audio System** - `audio::load_sound()`, `audio::play()`, `audio::load_music()`, `audio::play_music()`
- [x] **ListBox Widget** - Lista seleccionable con hover y scroll
- [x] **Layout Automático** - `begin_vertical()`, `next_y()`, `begin_horizontal()`, `next_x()`
- [x] **12 widgets migui** - button, label, checkbox, slider, panel, textbox, window, message_box, dropdown, progress_bar, listbox, layout
- [x] **REPL Interactivo** - `:help`, `:load`, `:save`, `:vars`, `:history`, `:clear`, `:exit`
- [x] **Sistema de Partículas** - `particles::emit()`, efectos: fuego, humo, explosión, lluvia, chispas
- [x] **Fix Termux-X11 Automático** - Detección y configuración automática de DISPLAY, zink, DRI3
- [x] **Stdlib Embebido** - 8 módulos en binario (math, arrays, strings, io, random, time, json, colisiones)
- [x] **Optimización** - `strip = true`, binario release ~550 KB (-100 KB)

### 🔜 Próximamente (v0.6.0 - v1.0.0)
- [ ] **Animaciones 2D** - Sprite sheets, 12 principios de animación, blending
- [ ] **Motor de Escenas** - Cambiar entre menús, niveles, nodos
- [ ] **Prefabs** - Objetos reutilizables
- [ ] **Temas Personalizables** - dark, light, custom
- [ ] **Más widgets** - treeview, table, toolbar
- [ ] **Layout grid** - Distribución en cuadrícula
- [ ] **Ecosistema maduro** - Integración con otras herramientas
- [ ] **Editor visual de escenas** - Inspector de propiedades
- [ ] **Ecosistema de frameworks** - RPG, platformer, shooter
- [ ] **Asset store comunitario**

---

## 🧪 Evaluación de la Comunidad

Este proyecto está siendo revisado por la comunidad de desarrolladores. Las evaluaciones detalladas de asistentes de IA se incluirán en la próxima actualización cuando el repositorio sea público.

> **"Espero tu evaluación de este proyecto con buena intención. Es mostrar lo que se hace en un celular gama baja, y que a futuras versiones con su apoyo, osea la comunidad, crezca en ecosistema con la capacidad enorme de la comunidad, para que creen sus escenas en hardware modesto sin depender de IA que hace un video rápido y sin experiencia propia. Esa es una de las claves."**

---

## 📦 Instalación y Dependencias

### Crates en crates.io (NUEVO ✅)

**RyDit ahora está disponible en crates.io**. Puedes usar los módulos individuales en tus proyectos Rust:

```toml
[dependencies]
rydit-core = "0.7.34"      # Trait RyditModule + Registry
rydit-science = "0.7.34"   # Bezier, Stats, Geometry, Optical illusions
rydit-physics = "0.7.34"   # Projectile, Gravity, N-body
rydit-anim = "0.7.34"      # Easing, Squash & Stretch
```

```bash
# Instalar crates individuales
cargo add rydit-core
cargo add rydit-science
cargo add rydit-physics
cargo add rydit-anim

# Ver en crates.io
# https://crates.io/crates/rydit-core
# https://crates.io/crates/rydit-science
# https://crates.io/crates/rydit-physics
# https://crates.io/crates/rydit-anim
```

### Ejemplo de Uso

```rust
use rydit_science::ScienceModule;
use rydit_core::RyditModule;
use serde_json::json;

// Curva Bezier cúbica
let science = ScienceModule;
let point = science.execute("bezier::cubic", 
    json!([0.0, 0.0, 30.0, 100.0, 70.0, 100.0, 100.0, 0.0, 0.5])
).unwrap();
println!("Punto en t=0.5: {:?}", point); // [50.0, 75.0]

// Estadísticas
let mean = science.execute("stats::mean", json!([1.0, 2.0, 3.0, 4.0, 5.0])).unwrap();
println!("Media: {:?}", mean); // 3.0

// Geometría - Triángulo de Penrose
let penrose = science.execute("geometry::penrose", json!([400.0, 300.0, 100.0])).unwrap();
println!("Líneas de Penrose: {:?}", penrose);
```

---

### Android/Termux (Plataforma Primaria)

```bash
# 1. Instalar Termux (desde F-Droid, NO Play Store)
# https://f-droid.org/en/packages/com.termux/

# 2. Actualizar paquetes
pkg update && pkg upgrade

# 3. Instalar Rust
pkg install rust

# 4. Instalar dependencias de sistema (para raylib)
pkg install xorg-xrandr libx11

# 5. Clonar repositorio
git clone https://github.com/lapumlbb18-blip/Rydit_Engine
cd Rydit_Engine

# 6. Compilar
cargo build --release

# 7. Ejecutar REPL
./target/release/rydit-rs --repl

# 8. Ejecutar demo Snake
./target/release/rydit-rs --gfx snake.rydit
```

### Linux (Ubuntu/Debian)

```bash
# 1. Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Instalar dependencias de sistema (raylib)
sudo apt-get update
sudo apt-get install libasound2-dev libx11-dev libxi-dev libxrandr-dev

# 3. Clonar y compilar
git clone https://github.com/lapumlbb18-blip/Rydit_Engine
cd Rydit_Engine
cargo build --release

# 4. Ejecutar
./target/release/rydit-rs --repl
```

### Windows

```powershell
# 1. Instalar Rust desde https://rustup.rs/

# 2. Instalar dependencias (vcpkg o winget)
winget install raylib.raylib

# 3. Clonar y compilar
git clone https://github.com/lapumlbb18-blip/Rydit_Engine
cd Rydit_Engine
cargo build --release

# 4. Ejecutar
.\target\release\rydit-rs.exe --repl
```

### Dependencias Principales

| Dependencia | Versión | License | Uso |
|-------------|---------|---------|-----|
| **raylib** | 5.5 | Zlib | Gráficos, audio, input |
| **serde** | 1.0 | MIT | Serialización JSON |
| **serde_json** | 1.0 | MIT | Parsing JSON |
| **sccache** | 0.4+ | Apache-2.0 | Cache de compilación (opcional) |

### Dependencias de Sistema

#### Linux (para compilación)
```bash
sudo apt-get install libasound2-dev libx11-dev libxi-dev libxrandr-dev libgl1-mesa-dev
```

#### Termux (Android)
```bash
pkg install xorg-xrandr libx11
```

---

## 💬 Comunidad

### 🌐 Únete y Evalúa este Proyecto

| Plataforma | Enlace |
|-----------|--------|
| **Discord Mouredev** | https://discord.gg/mouredev |
| **Reddit r/rust** | https://reddit.com/r/rust |
| **Reddit r/gamedev** | https://reddit.com/r/gamedev |
| **Reddit r/AndroidGaming** | https://reddit.com/r/AndroidGaming |
| **X (Twitter)** | #RyDit #RustLang #AndroidDev |

### 💌 Tu Opinión Importa

¿Qué piensas de este proyecto? ¿Crees que es posible crear un motor de videojuegos completo en un celular gama baja?

**Tu evaluación ayuda a:**
- 📱 Demostrar que el desarrollo en Android es posible
- 🤝 Construir una comunidad que comparte conocimiento real
- 🎮 Crear un ecosistema donde todos pueden hacer sus juegos
- 🚀 Inspirar a otros con hardware modesto

### 🔜 Próximamente

- **Servidor Discord propio** - Espacio dedicado para RyDit
- **Evaluaciones públicas de IA** - Análisis detallado del código
- **Asset store comunitario** - Frameworks, escenas, herramientas

### 🤝 Contribuciones

¡Las contribuciones son bienvenidas! Lee [CONTRIBUTING.md](CONTRIBUTING.md) para más detalles.

```bash
# Clonar repositorio
git clone https://github.com/alucard18/shield-project.git

# Build
cd shield-project
cargo build

# Tests
cargo test

# Ejecutar demo
cargo run --bin rydit-rs -- --gfx demo_shapes.rydit
```

---

## 📚 Documentación

| Documento | Descripción |
|-----------|-------------|
| **[README.md](README.md)** | Documentación técnica interna |
| **[LIBRO_RYDIT.md](LIBRO_RYDIT.md)** | Guía completa del lenguaje (~400 líneas) |
| **[ROADMAP.md](ROADMAP.md)** | Planificación detallada |
| **[CONTRIBUTING.md](CONTRIBUTING.md)** | Guía de contribuciones |
| **[CHANGELOG_v0.1.8.md](CHANGELOG_v0.1.8.md)** | Cambios de versión |
| **[diagnostico/](diagnostico/)** | Logs de desarrollo (25 sesiones) |

---

## 🎮 Ejemplos de la Comunidad

### RPG Simple
```rydit
import rpg::engine

rytmo juego {
    rpg.iniciar("mi_rpg.rydit")
    rpg.crear_personaje("heroe", ["espada", "escudo"])
    rpg.iniciar_dialogo("npc", ["Hola", "Adiós"])
}
```

### Platformer
```rydit
import platformer::physics

platformer.fisica.gravedad(9.8)
dark.slot jugador = platformer.crear_jugador(100, 200)

ryda frame < 10000 {
    platformer.mover(jugador, "derecha")
    onif platformer.colision(jugador, "suelo") {
        platformer.saltar(jugador)
    }
}
```

### Visual Novel
```rydit
shield.init

dark.slot nombre = input("¿Cómo te llamas?")

ryda frame < 500 {
    draw.text("Hola " + nombre, 200, 100, "blanco")
    draw.text("¿Qué haces hoy?", 200, 200, "blanco")
    
    onif gui.button("Estudiar", 100, 300, 200, 50) {
        draw.text("¡Buena decisión!", 200, 400, "verde")
    }
}
```

---

## 🏆 Logros

### Sesión v0.5.3 - REPL Interactivo + Partículas
- ✅ **REPL Interactivo** - `:help`, `:load`, `:save`, `:vars`, `:history`, `:clear`, `:exit` (+150 líneas)
- ✅ **Historial de comandos** - Navegación con ↑ ↓ (listo para implementación con crossterm)
- ✅ **Auto-completado** - Función `auto_complete()` con keywords RyDit
- ✅ **Colores en output** - Verde (éxito), Rojo (error), Cyan (ayuda)
- ✅ **Sistema de Partículas** - ~400 líneas Rust (`particles.rs`)
- ✅ **5 efectos preset** - fuego, humo, explosión, lluvia, chispas
- ✅ **Demo Partículas** - Binary independiente (`demo_particles`)
- ✅ **45+ tests pasando** (core sin gráficos, sin regresiones)
- ✅ **0 errors, 1 warning menor**
- ✅ **~1,200 líneas Rust** agregadas (REPL ~150, partículas ~400, demo ~130, docs ~576)

### Sesión v0.5.2 - Audio + ListBox + Layout
- ✅ **Audio System** - `audio::load_sound()`, `audio::play()`, `audio::load_music()`, `audio::play_music()` (10 funciones)
- ✅ **ListBox Widget** - Lista seleccionable con hover y scroll automático
- ✅ **Layout Automático** - Vertical y horizontal con spacing configurable
- ✅ **45+ tests pasando** (core sin gráficos, sin regresiones)
- ✅ **0 warnings, 0 errors**
- ✅ **~500 líneas Rust** agregadas (audio ~200, migui ~160, main ~130)

### Sesión v0.5.1 - Funciones Assets + Renderizado X11
- ✅ **Funciones Assets en RyDit** - `assets::load_texture()`, `assets::draw()`, `assets::draw_scaled()`
- ✅ **Demo Assets Funcional** - Tanque + Helicóptero con sprites (60 FPS)
- ✅ **Fix Renderizado Termux-X11** - Variables zink, frame variable, evaluar_expr_gfx
- ✅ **124 tests pasando** (sin regresiones)
- ✅ **0 warnings, 0 errors**
- ✅ **~230 líneas Rust** agregadas

### General
- ✅ **38+ sesiones en 13 días** (v0.0.1 → v0.5.3)
- ✅ **6 crates funcionales**
- ✅ **~11,700 líneas de código**
- ✅ **Documentación completa** (20+ archivos .md)
- ✅ **GitHub público** (Rydit_Engine)
- ✅ **README en inglés** (README_EN.md)

---

## 📄 Licencia

MIT License - Ver [LICENSE](LICENSE) para más detalles.

---

## 🙏 Agradecimientos

- **Comunidad Mouredev** - Discord: https://discord.gg/mouredev - Por el apoyo y espacio para compartir proyectos
- **raylib** (https://www.raylib.com/) - Por la biblioteca gráfica más ligera y fácil de usar
- **Rust** (https://www.rust-lang.org/) - Por el lenguaje más amado por 8 años consecutivos
- **Termux** - Por hacer posible el desarrollo en Android sin root

---

<div align="center">

## 🚀 "Construido con ❤️ en Android/Termux"

**"No necesitas una laptop cara para crear software impresionante. Solo necesitas un teléfono, determinación y mucha café."** ☕

**"Este proyecto es una invitación a la comunidad: miren lo que se puede hacer en un celular gama baja. Mi sueño es que a futuras versiones, con su apoyo, crezcamos en ecosistema. Que todos puedan crear sus escenas y juegos en hardware modesto, sin depender de herramientas que hacen todo rápido pero sin experiencia propia. Esa es la clave: aprender creando, no solo consumiendo."**

---

*¿Quieres evaluar este proyecto?* Únete al **Discord Mouredev**: https://discord.gg/mouredev y comparte tu opinión en #mostrar-proyecto

*Última actualización:* 2026-03-26 (v0.7.3.3 - 4 crates extraídos: core, science, physics, anim)
*Próxima versión:* v0.7.3.x (rydit-geometry) → v0.8.0.0 (crates.io + Linux/Windows)
*Estado:* ✅ **81 TESTS - 4 CRATES INDEPENDIENTES - LAZOS FUNCIONAL - 730 KB**

---

## 🔗 Protocolo LAZOS (v0.7.2.0)

**Sistema universal de comunicación entre RyDit y otros lenguajes.**

### **Características:**

- ✅ **Universal** - Funciona con Python, Node.js, C, Bash, cualquier lenguaje
- ✅ **Simple** - JSON-RPC sobre stdin/stdout
- ✅ **Nativo** - Parte del binario de rydit-rs
- ✅ **Seguro** - Sin red, solo local
- ✅ **Rápido** - ~200 líneas de código Rust

### **Ejemplo desde Python:**

```python
from ry_lazo import RyLazo

with RyLazo() as ry:
    # Bezier cúbica
    punto = ry.call("science::bezier::cubic", 
                   [0, 0, 30, 100, 70, 100, 100, 0, 0.5])
    print(punto)  # [50.0, 75.0]
    
    # Física: proyectil
    trayectoria = ry.call("physics::projectile", [0, 0, 50, 45])
    print(f"Alcance: {trayectoria[4]:.2f} m")
    
    # Estadísticas
    media = ry.call("stats::mean", [[1, 2, 3, 4, 5]])
    print(f"Media: {media}")
```

### **Ejemplo desde Shell:**

```bash
# Ping
echo '{"method":"system::ping"}' | rydit-rs --lazos
# {"result":"pong"}

# Bezier
echo '{"method":"science::bezier::cubic","params":[0,0,30,100,70,100,100,0,0.5]}' | rydit-rs --lazos
# {"result":[50.0,75.0]}
```

### **Comandos Disponibles:**

**System:**
- `system::version` - Versión de RyDit
- `system::ping` - Verificar conexión
- `system::info` - Información completa

**Bezier:**
- `science::bezier::linear` - Bezier lineal (2 puntos)
- `science::bezier::quadratic` - Bezier cuadrática (1 control)
- `science::bezier::cubic` - Bezier cúbica (2 controles)

**Física:**
- `physics::projectile` - Trayectoria de proyectil
- `physics::nbody_2` - Gravedad (2 cuerpos)

**Estadísticas:**
- `stats::mean` - Media aritmética
- `stats::median` - Mediana
- `stats::min` / `stats::max` - Mínimo/Máximo

### **Documentación Completa:**

Ver [ROADMAP_LAZOS.md](ROADMAP_LAZOS.md) para arquitectura completa.

---

[⬆️ Volver arriba](#-rydit---rust-gaming--scripting-engine-for-androidtermux)

</div>
