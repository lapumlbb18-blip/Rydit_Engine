# 🛡️ RyDit - 2D Game Engine + Rust Scripting Language for Android/Termux

<div align="center">

![RyDit Engine Logo](screenshots/logo.png)

**"David vs Goliath - A game engine in Rust, built 100% on a Redmi Note 8"**

[![Version](https://img.shields.io/badge/version-v0.10.6-blue.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine)
[![Tests](https://img.shields.io/badge/tests-260%20passing-green.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![raylib](https://img.shields.io/badge/raylib-5.5-purple.svg)](https://www.raylib.com/)
[![SDL2](https://img.shields.io/badge/SDL2-0.37-red.svg)](https://www.libsdl.org/)
[![Platform](https://img.shields.io/badge/platform-Android%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine/blob/main/LICENSE)

[📖 Documentation](#-documentation) • [🎮 Snake Demo](#-snake-game---functional-demo) • [🚀 Roadmap](#-roadmap) • [📱 Built on Android](#-built-on-androidtermux) • [💬 Community](#-community)

</div>

---

## 🔥 v0.10.6 - SDL2 BACKEND COMPLETE (2026-03-31)

### ✅ **SDL2 Backend Fully Functional!**

After 10 days stuck with input issues, we implemented a complete SDL2 backend:

| System | Method | Works on Termux-X11? |
|--------|--------|---------------------|
| **Raylib/GLFW** | Polling (`glfwGetKey()`) | ❌ **NO** |
| **SDL2 Backend** | Event Loop (`poll_iter()`) | ✅ **YES!** |
| **glxgears** | X11 Events | ✅ YES |

**Working Demos**:
- ✅ `test_callback_sdl2.rs` - Pure SDL2, perfect input
- ✅ `demo_sdl2_puro.rs` - Pure SDL2, smooth movement
- ✅ `demo_particulas_sdl2.rs` - 100+ particles @ 60 FPS

**Solution**: SDL2 backend for Android/Termux-X11, Raylib for Desktop.

### v0.10.6 Features
- ✅ **Sdl2Backend** - Complete window + OpenGL 3.3 Core context
- ✅ **InputState** - 69 keys mapped with event loop
- ✅ **GPU Instancing Ready** - OpenGL context for GPU rendering
- ✅ **Primitives** - Rect, Circle, Text (basic)
- ✅ **VSync** - 60 FPS stable
- ⚠️ **SDL2_image** - Pending linking fix
- ⚠️ **SDL2_ttf** - Pending
- ⚠️ **SDL2_mixer** - Pending

---

## 🎯 What is RyDit?

**RyDit** is a **2D game engine with scripting language** written in **Rust** with **raylib**, designed to run natively on **Android/Termux** without needing a desktop, emulators, or heavy IDEs.

**Not just a language** - it's a complete engine with:
- 🎮 Integrated game loop
- 🎨 Graphics rendering (circles, rectangles, lines, text)
- 🎹 Real-time keyboard input
- 🎲 Module system (math, arrays, strings, io, random, time, json)
- 🧪 45+ automatic tests
- 📦 Complete Snake Game as demo
- 🎵 Audio system (sounds + music)
- 🖼️ Immediate Mode GUI (12 widgets)

```rydit
# Your first game in RyDit (3 lines)
shield.init
ryda frame < 1000 {
    draw.circle(400, 300, 50, "red")
}
```

| Feature | RyDit | Godot | Love2D | PICO-8 |
|---------------|-------|-------|--------|--------|
| **Android Native** | ✅ Yes (Termux) | ❌ No | ❌ No | ❌ No |
| **Language** | RyDit (Spanish) | GDScript | Lua | Lua |
| **Backend** | Rust | C++ | C | C |
| **Binary Size** | ~890 KB | ~50 MB | ~10 MB | ~5 MB |
| **No IDE Required** | ✅ Yes | ❌ Requires editor | ⚠️ VS Code | ⚠️ Own editor |
| **Game Loop** | ✅ Built-in | ✅ Built-in | ✅ Built-in | ✅ Built-in |

---

## 🎮 Snake Game - Functional Demo

<div align="center">

![Snake Game](screenshots/04_snake_gameplay.jpg)

*Complete Snake Game with game loop, collisions, scoring, and game over screen*

</div>

### Snake Features
- ✅ **Snake body** with dynamic arrays
- ✅ **Random food** with `random::int()`
- ✅ **Collisions** with walls and self
- ✅ **Score** + high score
- ✅ **Progressive speed**
- ✅ **Game Over** + restart with SPACE
- ✅ **Pause** with P
- ✅ **Exit** with ESC

### Run Snake
```bash
# On Termux (Android)
cargo run --bin rydit-rs -- --gfx snake.rydit

# Or with direct binary
./target/release/rydit-rs --gfx snake.rydit
```

### Controls
| Key | Action |
|-------|--------|
| `↑` `→` `↓` `←` | Move snake |
| `P` | Pause |
| `SPACE` | Restart |
| `ESC` | Exit |

---

## 🎨 Visual Demo - Shapes and Colors

<div align="center">

![Demo Shapes](screenshots/03_demo_shapes_circulos.jpg)

*Demo with geometric shapes using draw.circle(), draw.rect(), draw.line(), draw.text()*

</div>

### Execute Demo
```bash
./target/debug/rydit-rs --gfx demos/demo_shapes.rydit
```

---

## 🚀 Quick Start

### 1. Install Dependencies (Termux)
```bash
# Update packages
pkg update && pkg upgrade

# Install Rust
pkg install rust

# Install raylib dependencies
pkg install libllvm libx11 libxrandr libxi libxcursor libxinerama

# Clone repository
git clone https://github.com/lapumlbb18-blip/Rydit_Engine.git
cd Rydit_Engine
```

### 2. Build
```bash
# Debug build (fast)
cargo build

# Release build (optimized)
cargo build --release
```

### 3. Run Your First Script
```bash
# Create my_game.rydit
echo 'shield.init
ryda frame < 500 {
    draw.circle(400, 300, 50, "red")
    draw.text("Hello RyDit!", 350, 280, 24, "white")
}' > my_game.rydit

# Execute
./target/debug/rydit-rs --gfx my_game.rydit
```

---

## 📖 RyDit Language Documentation

### Basic Syntax

```rydit
# Comments with #
# Variables
player_name = "Hero"
health = 100
speed = 5.5
is_alive = true

# Arrays
items = ["sword", "shield", "potion"]

# Conditionals
ryda health > 0 {
    draw.text("Alive!", 100, 100, 20, "green")
} blelse {
    draw.text("Game Over", 100, 100, 20, "red")
}

# Game Loop (while)
ryda frame < 1000 {
    draw.circle(x, y, 50, "blue")
    x = x + 1
}

# For Each Loop
cada item en items {
    voz(item)  # Print
}

# Functions
ryda suma(a, b) {
    return a + b
}

result = suma(5, 3)
voz(result)  # Prints: 8
```

### Graphics Commands

```rydit
shield.init  # Initialize graphics

# Draw shapes
draw.circle(x, y, radius, "color")
draw.rect(x, y, width, height, "color")
draw.line(x1, y1, x2, y2, "color")
draw.text("message", x, y, size, "color")

# Colors: "red", "green", "blue", "yellow", "white", "black", etc.
```

### Audio Commands (v0.5.2+)

```rydit
# Load and play sounds
audio::load_sound("click", "sounds/click.wav")
audio::play("click")
audio::set_volume("click", 0.8)

# Load and play music
audio::load_music("music/ost.ogg")
audio::play_music()
audio::set_music_volume(0.5)
audio::stop_music()
```

### GUI Commands - migui (v0.4.0+)

```rydit
# Button - returns true when clicked
ryda migui::button("btn", "Click Me", 100, 100, 200, 40) {
    voz("Button clicked!")
}

# Checkbox - returns new state
checkbox_state = migui::checkbox("chk", checkbox_state, 100, 150, 200, 30)

# Slider - returns new value
slider_value = migui::slider("sld", slider_value, 0.0, 100.0, 100, 200, 30)

# ListBox - returns selected index
selected = migui::listbox("lst", ["Option 1", "Option 2"], 100, 250, 200, 100)

# Layout - automatic positioning
migui::begin_vertical("layout", 100, 100, 200, 300, 10)
y = migui::next_y("layout", 40)
migui::button("btn1", "Button 1", 100, y, 200, 40)
migui::end_vertical("layout")
```

### Modules

```rydit
# Import standard modules
import math
import arrays
import strings
import io
import random
import time
import json

# Use with alias
import math as m
x = m::sqrt(16)

# Import custom modules
import my_module
```

### Standard Library

```rydit
# Math
math::sqrt(16)      # 4.0
math::sin(1.57)     # 1.0
math::cos(0)        # 1.0
math::abs(-5)       # 5
math::pow(2, 3)     # 8.0
math::min(5, 10)    # 5
math::max(5, 10)    # 10

# Arrays
arrays::push(my_array, element)
arrays::pop(my_array)
arrays::len(my_array)
arrays::slice(my_array, 0, 5)
arrays::reverse(my_array)

# Strings
strings::length("hello")    # 5
strings::upper("hello")     # "HELLO"
strings::lower("HELLO")     # "hello"

# IO
io::print("Hello")
io::write("file.txt", "content")
io::read("file.txt")
io::copy("src.txt", "dst.txt")

# Random
random::int(1, 10)    # Random int between 1-10
random::float()       # Random float 0.0-1.0

# Time
time::now()           # Current timestamp

# JSON
json::parse('{"key": "value"}')
json::stringify(my_data)
```

---

## 🎯 Project Status

### ✅ Completed (v0.5.2)
- [x] Lexer + Parser with AST
- [x] Executor with memory and scopes
- [x] Module system (import)
- [x] 45+ automatic tests (core without graphics)
- [x] 16 benchmarks
- [x] Complete Snake Game
- [x] Graphics with raylib
- [x] Mature Strings, IO, Arrays
- [x] JSON support (`json::parse()`, `json::stringify()`)
- [x] Lightweight Random + Time
- [x] Full UTF-8 support
- [x] String escapes (\n, \t, \\, \")
- [x] Symbols in identifiers (@, $, %, etc.)
- [x] Tank Combat + collisions
- [x] **migui** (Immediate Mode GUI ~600 lines)
- [x] **migui raylib backend** (real 60 FPS rendering)
- [x] **Assets Functions** - `assets::load_texture()`, `assets::draw()`, `assets::draw_scaled()`
- [x] **Audio System** - `audio::load_sound()`, `audio::play()`, `audio::load_music()`, `audio::play_music()`
- [x] **ListBox Widget** - Selectable list with hover and scroll
- [x] **Automatic Layout** - `begin_vertical()`, `next_y()`, `begin_horizontal()`, `next_x()`
- [x] **12 migui widgets** - button, label, checkbox, slider, panel, textbox, window, message_box, dropdown, progress_bar, listbox, layout

### 🔜 Coming Soon (v0.5.3 - v1.0.0)
- [ ] **Interactive REPL** - History, auto-completion, syntax highlighting
- [ ] **Particle System** - Emitter, forces (gravity, wind), dynamic colors
- [ ] **Sprite Animations** - Sprite sheets, frames per second
- [ ] **Scene Manager** - Switch between menus, levels, nodes
- [ ] **Prefabs** - Reusable objects
- [ ] **Custom Themes** - dark, light, custom
- [ ] **More widgets** - treeview, table, toolbar
- [ ] **Grid Layout** - Grid distribution
- [ ] **Mature ecosystem** - Integration with other tools
- [ ] **Visual scene editor** - Property inspector
- [ ] **Framework ecosystem** - RPG, platformer, shooter
- [ ] **Community asset store**

---

## 🏆 Achievements

### Session v0.5.2 - Audio + ListBox + Layout
- ✅ **Audio System** - `audio::load_sound()`, `audio::play()`, `audio::load_music()`, `audio::play_music()` (10 functions)
- ✅ **ListBox Widget** - Selectable list with hover and auto-scroll
- ✅ **Automatic Layout** - Vertical and horizontal with configurable spacing
- ✅ **45+ tests passing** (core without graphics, no regressions)
- ✅ **0 warnings, 0 errors**
- ✅ **~500 Rust lines** added (audio ~200, migui ~160, main ~130)

### General
- ✅ **36+ sessions in 12 days** (v0.0.1 → v0.5.2)
- ✅ **6 functional crates**
- ✅ **~10,500 lines of code**
- ✅ **Complete documentation** (16+ .md files)
- ✅ **Public GitHub** (Rydit_Engine)
- ✅ **Automated Google Drive Backup**

---

## 💾 Backup

- **Google Drive:** `alucard18:/shield-project-rydit`
- **Historical Backup:** `alucard18:/shield-project-rydit-historial` (old files)
- **Backup Scripts:**
  - `./backup_google_drive.sh` - Quick backup (code only)
  - `./backup_con_binarios.sh` - Complete backup (code + binaries)
- **Files:** 120+
- **Size:** ~2 MB (without `target/`, with binaries)
- **Last sync:** 2026-03-23 (v0.5.2)
- **Command:** `./backup_con_binarios.sh`

---

## 📄 License

MIT License - See [LICENSE](LICENSE) for details.

---

## 🚀 "Built with ❤️ on Android/Termux"

**"You don't need an expensive laptop to create amazing software. You just need a phone, determination, and lots of coffee."** ☕

**"This project is an invitation to the community: look what can be done on a low-end phone. My dream is that in future versions, with your support, we grow into an ecosystem. That everyone can create their scenes and games on modest hardware, without depending on tools that do everything fast but without their own experience. That's the key: learning by creating, not just consuming."**

---

*Want to evaluate this project?* Join the **Mouredev Discord**: https://discord.gg/mouredev and share your opinion in #mostrar-proyecto

*Next update:* v0.5.3 Interactive REPL + Particles

*Last updated:* 2026-03-23 (v0.5.2 - Audio + ListBox + Automatic Layout)
*Next version:* v0.5.3 (Interactive REPL + Particle System + Sprite Animations)
*Status:* ✅ **45+ TESTS - 16 BENCHMARKS - 12 WIDGETS - AUDIO SYSTEM - AUTOMATIC LAYOUT - 60 FPS**

[⬆️ Back to top](#-rydit---rust-gaming--scripting-engine-for-androidtermux)

</div>
