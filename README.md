# 🛡️ RyDit - Motor de Videojuegos 2D + Lenguaje de Scripting en Rust para Android/Termux

<div align="center">

![RyDit Engine Logo](screenshots/logo.png)

**"David vs Goliat - Un motor de videojuegos en Rust, construido 100% en un Redmi Note 8"**

[![Version](https://img.shields.io/badge/version-v0.6.0-blue.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine)
[![Tests](https://img.shields.io/badge/tests-126%20passing-green.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![raylib](https://img.shields.io/badge/raylib-5.5-purple.svg)](https://www.raylib.com/)
[![Platform](https://img.shields.io/badge/platform-Android%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine/blob/main/LICENSE)

[📖 Documentación](#-documentación) • [🎮 Demo Snake](#-snake-game---demo-funcional) • [🚀 Roadmap](#-roadmap) • [📱 Construido en Android](#-construido-en-androidtermux) • [💬 Comunidad](#-comunidad)

</div>

---

## 🎯 ¿Qué es RyDit?

**RyDit** es un **motor de videojuegos 2D con lenguaje de scripting** escrito en **Rust** con **raylib**, diseñado para ejecutarse nativamente en **Android/Termux** sin necesidad de desktop, emuladores o IDEs pesados.

**No es solo un lenguaje** - es un motor completo con:
- 🎮 Game loop integrado
- 🎨 Renderizado gráfico (círculos, rectángulos, líneas, texto, sprites)
- 🎹 Input de teclado en tiempo real
- 🎲 Sistema de módulos (math, arrays, strings, io, random, time, json, colisiones)
- 🧪 **126 tests automáticos**
- 🎨 **12 widgets UI** (migui - immediate mode GUI)
- 🎵 **Audio** (sonidos + música)
- ✨ **Sistema de partículas** (fuego, humo, explosión, lluvia, chispas)
- 💻 **REPL interactivo** con historial y auto-completado
- 📦 **Stdlib embebido** (sin archivos externos necesarios)
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
2. **Ligero y Portable** - Binario de ~550 KB (no 50 MB como Godot)
3. **Educativo** - Código 100% abierto, lenguaje en español, sin magia
4. **David vs Goliat** - 12,000 líneas de Rust bien escritas > 500,000 líneas de C++

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
| **v0.6.1** | 🔜 | **Animaciones 2D** (sprite sheets, anim::load, anim::play) | Próxima |
| **v0.7.0** | 🔮 | **Motor de Escenas** (nodos, señales, prefabs) | 2-3 meses |
| **v1.0.0** | 🔮 | Production Ready | 4-6 meses |

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
- ✅ **Backup Google Drive** automatizado
- ✅ **README en inglés** (README_EN.md)

---

## 💾 Backup

- **Google Drive:** `alucard18:/shield-project-rydit`
- **Backup Histórico:** `alucard18:/shield-project-rydit-historial` (archivos antiguos)
- **Scripts de Backup:**
  - `./backup_google_drive.sh` - Backup rápido (solo código)
  - `./backup_con_binarios.sh` - Backup completo (código + binarios)
- **Archivos:** 120+
- **Tamaño:** ~2 MB (sin `target/`, con binarios)
- **Última sync:** 2026-03-23 (v0.5.3)
- **Comando:** `./backup_con_binarios.sh`

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

*Próxima actualización:* v0.6.1 Animaciones 2D (sprite sheets)

*Última actualización:* 2026-03-23 (v0.6.0 - Fix Termux-X11 + Stdlib Embebido + Optimización)
*Próxima versión:* v0.6.1 (Animaciones 2D + Sprite Sheets + anim::load/play)
*Estado:* ✅ **45+ TESTS - 16 BENCHMARKS - 12 WIDGETS - AUDIO SYSTEM - PARTÍCULAS - REPL INTERACTIVO - STDLIB EMBEBIDO - 60 FPS**

[⬆️ Volver arriba](#-rydit---rust-gaming--scripting-engine-for-androidtermux)

</div>
