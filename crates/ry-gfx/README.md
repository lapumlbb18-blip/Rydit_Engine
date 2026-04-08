# 🎨 ry-gfx — Graphics Layer for Ry-Dit

**SDL2 + OpenGL GPU Instancing + FSR 1.0 — Built for Termux, runs everywhere.**

[![Version](https://img.shields.io/badge/version-v0.10.8-blue.svg)](https://crates.io/crates/ry-gfx)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![SDL2](https://img.shields.io/badge/SDL2-0.37-red.svg)](https://www.libsdl.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE)

---

## 📱 ¿Qué es ry-gfx?

**ry-gfx** es la capa de gráficos del motor Ry-Dit, diseñada desde cero para desarrolladores que programan **desde Termux en sus celulares** o máquinas virtuales de bajos recursos.

No necesitas una GPU de gama alta. No necesitas un IDE de 5GB. Solo Rust, SDL2, y ganas de crear.

> *"Si funciona en un Adreno 610 de 2018, funciona en tu máquina."*

---

## ✨ Features

| Feature | Descripción |
|---------|-------------|
| **GPU Instancing** | 50K partículas en 1 draw call (48 FPS en Adreno 610/Zink) |
| **FSR 1.0 Upscale** | 960×540 → 1280×720 con EASU edge-adaptive (~60% más FPS) |
| **SDL2 Backend** | Input, audio, TTF, imágenes — todo integrado |
| **Render Queue** | Command buffer con 8192+ draw calls + double buffering |
| **Platform Sync** | Sincronización X11/OpenGL (v-shield) para Termux-X11 |
| **MiGUI Integration** | UI toolkit con 12+ widgets (botones, sliders, paneles) |
| **Raylib FFI** | Compatibilidad con raylib para drawing primitives |
| **ECS básico** | Entidades con position, velocity, physics body |

---

## 🚀 Quick Start

### Dependencias (Termux)

```bash
pkg update && pkg upgrade
pkg install rust libllvm libx11 libxrandr libxi libxcursor sdl2 sdl2_image sdl2_ttf sdl2_mixer
```

### Usar como crate

```toml
[dependencies]
ry-gfx = "0.10"
```

### Ejemplo mínimo

```rust
use ry_gfx::RyditGfx;
use ry_gfx::render_queue::{RenderQueue, DrawCommand};

fn main() {
    let mut gfx = RyditGfx::new(1280, 720, "Mi Juego");
    let mut queue = RenderQueue::with_capacity(8192);

    // Acumular comandos
    queue.push(DrawCommand::Circle {
        x: 400, y: 300, radius: 50, color: "rojo",
    });

    // Ejecutar
    queue.execute(&mut gfx, &assets);
}
```

---

## 🎮 Demos incluidos

| Demo | Descripción | FPS (Adreno 610) |
|------|-------------|-------------------|
| `demo_gpu_instancing` | 50K-150K partículas instanciadas | ~53 FPS |
| `demo_fsr` | FSR 1.0 upscale pipeline | ~48 FPS |
| `demo_torreta_vs_sprites` | Juego completo: menú + 3 niveles + AI + audio | 60 FPS |
| `demo_platformer_completo` | Plataforma con física, colisiones, cámara | 60 FPS |
| `demo_rigidbody` | Cuerpos rígidos con gravedad | 60 FPS |

```bash
# Ejecutar demo desde Termux
cargo run --bin demo_gpu_instancing --release

# Con launcher (auto-detecta Zink + DISPLAY)
./launcher_gpu_instancing.sh
```

---

## 🏗️ Arquitectura

```
ry-gfx/
├── src/
│   ├── lib.rs              # RyditGfx + ColorRydit + exports
│   ├── backend_sdl2.rs     # SDL2 render (texturas, TTF, sprites)
│   ├── gpu_instancing.rs   # GPU instancing (VAO, VBO, IBO)
│   ├── fsr.rs              # FSR 1.0 EASU + RCAS shaders
│   ├── render_queue.rs     # Command queue + double buffer + PlatformSync
│   ├── shaders/            # GLSL embebidos (vertex + fragment)
│   └── entity.rs           # Entidades con transform + physics
├── shaders/
│   ├── vertex.glsl         # GPU instancing NDC
│   ├── fragment.glsl       # Quad sólido con círculos
│   ├── fsr_upscale.glsl    # AMD FidelityFX EASU
│   └── fsr_sharpen.glsl    # AMD FidelityFX RCAS
└── examples/
    └── demo_render_queue.rs
```

---

## 📊 Benchmarks

### GPU Instancing (50K partículas)

| Plataforma | GPU | FPS | Draw Calls |
|------------|-----|-----|------------|
| Termux-X11 | Zink / Adreno 610 | ~53 | 1 |
| llvmpipe (CPU) | Software | ~217 | 1 |
| Desktop | NVIDIA RTX 3060 | ~4000+ | 1 |

### FSR 1.0 Quality

| Configuración | FPS nativo | FPS con FSR | Ganancia |
|---------------|-----------|-------------|----------|
| 960×540 → 1280×720 | ~30 | ~48 | **+60%** |
| 720p → 1080p | ~20 | ~35 | **+75%** |

---

## 🔧 Features de Cargo

| Feature | Descripción | Default |
|---------|-------------|---------|
| `sdl2` | SDL2 backend completo | ✅ Sí |
| `gpu_instancing` | VAO/VBO instancing | ✅ Sí |
| `fsr` | FSR 1.0 post-processing | ✅ Sí |

---

## 📱 Filosofía Low-End First

Este crate fue construido **100% en un Redmi Note 8** (Snapdragon 665, Adreno 610, 4GB RAM) usando Termux + Termux-X11.

**Por qué importa**:

- No todos los devs tienen una MacBook Pro o una RTX 4090
- En Latinoamérica, el celular es la única computadora para millones
- Programar en un teléfono te enseña a optimizar cada byte
- Si tu motor corre aquí, corre en cualquier lado

**Nuestro compromiso**: cada feature nueva debe funcionar en hardware de 2018.

---

## 📦 Integración con Ry-Dit

ry-gfx es parte del workspace de Ry-Dit:

```
Ry-Dit/
├── crates/
│   ├── ry-gfx      ← Estás aquí
│   ├── ry-backend  # Raylib drawing + SDL2 input/audio
│   ├── ry-stream   # LAN streaming (WebSocket + JSON-RPC)
│   ├── v-shield    # Platform layer + sync primitives
│   ├── toolkit-ry  # UI widgets para HUD/menús
│   └── ry-rs       # Main binary + scripting language
```

---

## 📝 Changelog

### v0.10.8 (actual)
- ✅ v-shield Platform Sync (migrado de render_queue local)
- ✅ Platform Sync re-export desde v-shield

### v0.10.7
- ✅ SDL2 backend completo (input, TTF, audio, sprites)
- ✅ GPU Instancing funcional (50K partículas)
- ✅ FSR 1.0 con FBO pipeline

### v0.10.0
- ✅ GPU Instancing base (shaders embebidos, VAO, instance VBO)
- ✅ FSR 1.0 shaders (EASU + RCAS)
- ✅ Render queue con double buffering

---

## 🤝 Contribuir

1. Fork el repo: `https://github.com/lapumlbb18-blip/Ry-dit`
2. Crea tu rama: `git checkout -b mi-feature`
3. Commit: `git commit -m "Mi feature nueva"`
4. Push: `git push origin mi-feature`
5. Abre un Pull Request

**Nota**: Si desarrollas desde Termux, eres nuestro público objetivo. ¡Tu feedback vale oro!

---

## 📄 Licencia

MIT License — ver [LICENSE](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE)

---

<div align="center">

**🛡️ ry-gfx — Graphics que funcionan en tu celular**

*50K partículas · FSR 1.0 · 48 FPS en Adreno 610 · 100% Rust*

**"Hecho con ❤️ desde Termux, para devs que crean desde cualquier lugar"**

[⬆️ Ry-Dit GitHub](https://github.com/lapumlbb18-blip/Ry-dit)

</div>
