# 🛡️ Ry-Dit — Sistema de Creación en Rust para Android/Termux

> **Ry-Dit no es solo un motor de juegos.** Es un sistema de creación para desarrolladores, artistas, soñadores y streamers que quieren construir lo que imaginen — sin límites de categoría ni de hardware.
>
> *Hecho desde un Redmi Note 8 (4GB RAM, Adreno 610) en Termux. Sin PC de alto gama.*

<div align="center">

![Ry-Dit Logo](screenshots/logo.png)

**"Construido sin prisa, madurado con paciencia"**

[![Version](https://img.shields.io/badge/version-v0.16.1-blue.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Errors](https://img.shields.io/badge/errors-0-green.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Tests](https://img.shields.io/badge/tests-144%2F144-brightgreen.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Status](https://img.shields.io/badge/estado-v0.16.1--snake--anime--buscaminas-green.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![SDL2](https://img.shields.io/badge/SDL2-0.37-red.svg)](https://www.libsdl.org/)
[![Raylib](https://img.shields.io/badge/raylib-5.0-orange.svg)](https://www.raylib.com/)
[![Platform](https://img.shields.io/badge/platform-Android%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-12%20publicados-purple.svg)](https://crates.io/crates/ry-anim)

[🚀 Inicio Rápido](#-inicio-rápido) • [🖼️ Galería](#-galería) • [🆕 Qué hay de nuevo](#-qué-hay-de-nuevo-en-v0161) • [🏆 Logros](#-logros) • [🎮 Demos](#-demos-funcionales) • [📦 Crates](#-crates-publicados) • [🎯 Roadmap](#-roadmap) • [📚 Archivos](#-archivos-del-proyecto)

</div>

---

## 🚀 Inicio Rápido

### ¿Qué puedes crear con Ry-Dit?

| Categoría | Ejemplo |
|-----------|---------|
| 🎮 Juegos 2D y 3D | Snake, Buscaminas, Torreta, platformers |
| 🛠️ Editores y herramientas | Tu propio editor visual, IDE, dashboard |
| 🎓 Software educativo | Simulaciones, visualizaciones interactivas |
| 🔬 Ciencia | Bezier, ondas, L-System, ilusiones ópticas |
| 📡 Streaming LAN | Servidor WebSocket + portal web |
| 🎬 Animaciones | 12 principios Disney, sprite animation |

### Instalar y compilar

```bash
# Clonar el repositorio
git clone https://github.com/lapumlbb18-blip/Ry-dit.git
cd Ry-dit

# Compilar en modo release
cargo build -p ry-rs --bin rydit-rs --release

# Ejecutar todos los tests
cargo test --workspace
```

### Ejecutar demos

```bash
# 🐍 Snake Anime v2 — Juego completo con cámara follow
./launcher_anime_v2.sh

# 💣 Buscaminas 16×16 con flood fill y banderas
./launcher_buscaminas.sh

# 🎮 Torreta vs Sprites — 3 niveles con AI y audio
./launcher_torreta.sh

# 🎬 Sprite animation con state machine
cargo run --bin demo_action_sprite --release

# 💥 50K partículas GPU instancing
./launcher_gpu_instancing.sh

# 🔍 FSR 1.0 upscaling 960→1280
./launcher_fsr.sh

# 📷 HUD + Cámara 2D con zoom y rotación
./launcher_hud_camera.sh
```

### Tu primer juego — Snake mínimo

```rust
use ry_rs::prelude::*;

fn main() {
    let mut game = Game::new("Mi Snake", 800, 600);
    let mut snake = Snake::new(100, 100);

    while game.running() {
        // Input WASD
        if game.input().key(KeyCode::W) { snake.move_up(); }
        if game.input().key(KeyCode::S) { snake.move_down(); }
        if game.input().key(KeyCode::A) { snake.move_left(); }
        if game.input().key(KeyCode::D) { snake.move_right(); }

        // Render
        game.draw_rect(snake.x(), snake.y(), 16, 16, Color::GREEN);
        game.present();
    }
}
```

### Scripting con .rydit

```rydit
# mi_nivel.rydit
tilemap 2400 1800
  tileset "sprites/tileset.png" 32 32
  layer 0 "mapa.csv"

camera follow player
  zoom 1.0
  rotation 0.0

entity player
  sprite "player.png"
  physics dynamic
  collider rect 32 48
```

### Stack técnico

| Capa | Tecnología |
|------|-----------|
| **Lenguaje** | Rust |
| **Scripting** | `.rydit` (lenguaje propio) |
| **2D** | Raylib + rlgl + SDL2 |
| **3D** | ry3d-gfx + v-shield |
| **GPU** | FSR 1.0 + GPU Instancing (250K partículas) |
| **Audio** | SDL2 Mixer + TTF |
| **Red** | Tokio + Tungstenite + LAZOS JSON/RPC |
| **GUI** | migui (12+ widgets) |
| **CI/CD** | GitHub Actions |

**Plataformas**: `Android` · `Linux` · `Windows` · `macOS` *(WASM en roadmap)*

---

## 🆕 ¿Qué hay de nuevo en v0.16.1?

**Última actualización**: 2026-04-09
**Versión actual**: v0.16.1 ✅ SNAKE ANIME + BUSCAMINAS + ACTION SPRITE + TILEMAP 2.0
**Estado**: 23 crates | 0 errores | 144 tests | 12 crates.io | 15+ demos

### 🐍 Snake Anime v2 — Juego Completo
| Feature | Detalle |
|---------|---------|
| **Snake controlable** | WASD/Flechas, ojos que miran dirección |
| **15 Manzanas 🍎** | Círculos amarillos recolectables |
| **20 Bombas 💣** | Con mecha y ojos, game over al tocar |
| **8 Entidades móviles** | Colores variados, rebotan en límites |
| **Cámara 2D follow** | Suave, límites de mapa 2400x1800 |
| **Minimap completo** | Snake, manzanas, bombas, entidades |
| **HUD + Debug** | Score, tiempo, manzanas, posición |
| **Game Over / Reinicio** | R: reiniciar, ESC: salir |

### 💣 Buscaminas Clásico
| Feature | Detalle |
|---------|---------|
| **Grid 16×16** | 576 casillas, 40 minas |
| **Click izquierdo** | Revelar con flood fill automático |
| **Click derecho** | Bandera 🚩 |
| **Primer click seguro** | Minas se colocan después del primer click |
| **Números con colores** | 1-8 clásicos (azul, verde, rojo...) |
| **Game Over / Victoria** | Revelación completa de minas |

### 🎬 Action Sprite System
| Feature | Detalle |
|---------|---------|
| **SpriteSheet** | Textura + grid de frames |
| **AnimationClip** | Named clips (idle, run, jump) |
| **AnimatedSprite** | Sheet + clips + state machine + flip + blend |
| **SpriteColor** | Color RGBA independiente |
| **RenderCommand** | Listo para backend con source_rect |
| **Reutiliza ry-anim** | 100% action_assets functions |

### 🗺️ Tilemap v2.0
| Feature | Detalle |
|---------|---------|
| **Texturas reales** | Tileset con source_rect por tile ID |
| **Camera culling** | 95% menos tiles procesados |
| **Import CSV** | Niveles desde archivos CSV simples |
| **Export CSV** | Exportar tilemap actual a CSV |
| **Multi-capa** | Layer count configurable |
| **13 funciones .rydit** | Scripting completo |

### ✨ Bordes Suaves + Alpha Blending
| Feature | Detalle |
|---------|---------|
| **smoothstep AA** | Fragment shader con anti-aliasing |
| **Alpha blending** | glEnable(GL_BLEND) + BlendFunc |
| **set_smoothness()** | 0.0=sólido, 0.05=sutil, 0.3=círculos |

### 🎨 ry3d-gfx Mejorado
| Feature | Detalle |
|---------|---------|
| **Texto 3D** | draw_text_3d (billboard) |
| **Modelos 3D** | Load GLTF/OBJ/IQM/VOX/MDL |
| **draw_model** | Posición + escala |
| **draw_model_ex** | Posición + rotación XYZ + escala |

### 📦 3 Crates Más Publicados
| Crate | Versión | Descripción |
|-------|---------|-------------|
| **ry-config** | 0.1.0 | Config parser zero-dependency |
| **ry-physics** | 0.7.34 | Projectile, N-body, gravity |
| **ry-science** | 0.7.34 | Bezier, stats, optical illusions |

---

## 🏆 Logros

| # | Logro | Fecha | Detalle |
|---|-------|-------|---------|
| 1 | **GPU Instancing** | v0.15.0 | 50K partículas a 48 FPS en Adreno 610 |
| 2 | **FSR 1.0** | v0.15.0 | 960x540 → 1280x720 a 48 FPS |
| 3 | **12 Crates publicados** | v0.16.0 | ry-god, ry-stream, v-shield, ry-backend, migui, ry-gfx, ry-core, ry-anim, toolkit-ry, ry-config, ry-physics, ry-science |
| 4 | **144 Tests** | v0.16.1 | ry-anim: 65, toolkit-ry: 14, ry-physics: 6, ry-science: 21, + más |
| 5 | **15+ Demos funcionales** | v0.16.1 | Snake, Buscaminas, Torreta, GPU Instancing, FSR, HUD, etc. |
| 6 | **0 Errores** | v0.16.1 | Workspace completo compilando limpio |
| 7 | **Launcher scripts** | v0.16.1 | 8 launchers con auto-detección DISPLAY + Zink |
| 8 | **action_sprite system** | v0.16.1 | Sprite animation completo con state machine |
| 9 | **Tilemap v2.0** | v0.16.1 | Texturas reales + CSV import/export + culling |

---

## 🖼️ Galería

### 🎬 Videos

#### Varios Demos — Recorrido General (4 partes)
> Múltiples demos funcionando en Termux-X11 (Adreno 610) — 4:29 min total

**Parte 1** — 1:07 | ![Varios Demos Parte 1](ry-galery_contenido/varios_demos_parte_1.gif)

**Parte 2** — 1:07 | ![Varios Demos Parte 2](ry-galery_contenido/varios_demos_parte_2.gif)

**Parte 3** — 1:07 | ![Varios Demos Parte 3](ry-galery_contenido/varios_demos_parte_3.gif)

**Parte 4** — 1:08 | ![Varios Demos Parte 4](ry-galery_contenido/varios_demos_parte_4.gif)

#### Demo Torreta vs Sprites — Juego Completo
> 3 niveles, cámara 2D, AI, audio (disparos), game states — 1:48 min

![Torreta Demo](ry-galery_contenido/demo_torreta_vs_sprites.gif)

#### Demo GPU Instancing — 50K Partículas
> GPU instancing con smoothstep AA a 48 FPS — 5 seg

![Partículas GPU](ry-galery_contenido/demo_particles.gif)

### 📸 Capturas de Pantalla

<details>
<summary><strong>📷 5 capturas de demos funcionales</strong> (click para expandir)</summary>

| | |
|:---:|:---:|
| ![Torreta Nivel 1](ry-galery_contenido/1er_demo_torreta.jpg) | ![Torreta Combate](ry-galery_contenido/1er_demo_torreta2.jpg) |
| 🎮 Demo Torreta — Nivel 1 | 🎮 Demo Torreta — Combate |
| ![Carga Sprites](ry-galery_contenido/carga_sprite.jpg) | ![Directorio Crates](ry-galery_contenido/directorio_crates.jpg) |
| 🎬 Carga de sprites + tilemap | 📦 Directorio de crates |
| ![Texto TTF](ry-galery_contenido/ttf.jpg) | — |
| 🔤 Texto TTF + sprites PNG | — |

</details>

---

## 🎮 Demos Funcionales

| Demo | Descripción | Launcher | Tamaño |
|------|-------------|----------|--------|
| **demo_anime_ry_v2** | 🐍 Snake + manzanas + bombas + entidades + minimap | `launcher_anime_v2.sh` | — |
| **demo_buscaminas** | 💣 Buscaminas 16×16 con mouse | `launcher_buscaminas.sh` | — |
| **demo_hud_camera** | HUD + Cámara 2D rotación/zoom + health bars | `launcher_hud_camera.sh` | — |
| **demo_action_sprite** | 🎬 Sprite animation con sprite sheet | — | — |
| **demo_gpu_instancing** | 50K partículas GPU instancing | `launcher_gpu_instancing.sh` | ~500K |
| **demo_fsr** | FSR 1.0 upscaling 960→1280 | `launcher_fsr.sh` | ~480K |
| **demo_torreta_vs_sprites** | Juego completo: 3 niveles, cámara, AI, audio | `launcher_torreta.sh` | 434K |
| **demo_rigidbody** | Física + colisiones + sprites PNG | — | 446K |
| **demo_panel_visual** | 4 paneles + consola interactiva | — | 339K |
| **demo_menu_bar** | Menús Dear ImGui + mouse + touch | — | 330K |
| **demo_anime_ry** | Showcase ry-anim v0.12.0 | — | 341K |
| **demo_50k_particulas** | 50K partículas | — | 313K |
| **demo_colisiones** | Sistema de colisiones | — | 309K |
| **demo_platformer_completo** | Plataformas + gravedad + salto | — | — |
| **demo_ttf_sprites** | Sprites PNG + texto TTF | — | 436K |

---

## 📦 Crates Publicados

| # | Crate | Versión | Descargas | Tests | Descripción |
|---|-------|---------|-----------|-------|-------------|
| 1 | ry-god | 0.1.0 | — | — | Security & Efficiency |
| 2 | ry-stream | 0.2.0 | 22 | 17 | LAN streaming |
| 3 | v-shield | 0.2.0 | 38 | 26 | Platform layer + sync |
| 4 | ry-backend | 0.1.0 | — | — | Dual backend (raylib + SDL2) |
| 5 | migui | 0.4.1 | — | — | Immediate Mode GUI |
| 6 | ry-gfx | 0.10.8 | — | — | GPU Instancing + FSR |
| 7 | ry-core | 0.8.2 | 🆕 | 9 | Core trait + registry |
| 8 | ry-anim | 0.12.0 | 🆕 | 65 | 12 Disney principles + action_sprite |
| 9 | toolkit-ry | 0.1.0 | 🆕 | 14 | UI toolkit + 5 themes + HUD |
| 10 | ry-config | 0.1.0 | 🆕 | 3 | Config parser zero-deps |
| 11 | ry-physics | 0.7.34 | 🆕 | 6 | Projectile, N-body, gravity |
| 12 | ry-science | 0.7.34 | 🆕 | 21 | Bezier, stats, illusions |

---

## 🎯 Roadmap

**3 Pilares**: 🎮 Gaming · 🎬 Animaciones+Ciencia · 📡 Streaming+Comunidad

| Versión | Features | Pilar | Target |
|---------|----------|-------|--------|
| **v0.17.0** | Sprite sheets reales + Texturas + Emojis TTF + GIF + Audio OGG | Gaming + Animaciones | 2-3 meses |
| **v0.18.0** | Audio/Mix completo + DLSS/NIS + Iluminación 2D + Sombras | Gaming + Render | 3-4 meses |
| **v0.19.0** | Letras 3D + Escenas (.ryscene) + Input map + Rybot CLI+GUI | Gaming + Ciencia | 4-5 meses |
| **v0.20.0** | Editor visual + Asset pipeline + LAZOS (Python+C+++C) + Multiplayer LAN | Gaming + Streaming | 6-8 meses |
| **v1.0.0** | GitHub Actions + SAZ + Motor completo + Debugger + Comunidad | Todos | 12-18 meses |

> 📋 Ver `ROADMAP.md` y `TASKS_2.md` para plan detallado con análisis comparativo de motores.

---

## 📚 Archivos del Proyecto

| Archivo | Descripción |
|---------|-------------|
| `ROADMAP.md` | Plan de versiones v0.16.1 → v1.0.0 con 3 pilares |
| `TASKS.md` | Tareas completadas y pendientes (43 tareas) |
| `TASKS_2.md` | 🆕 Análisis estratégico — Comparativa con Unreal, Unity, Godot, Bevy |
| `QWEN.md` | Bitácora técnica — Lecciones, bugs fixeados, comandos |
| `ESTRUCTURA.md` | Estructura del workspace y crates |
| `docs/ANALISIS.md` | Análisis y seguimiento sesión a sesión |
| `docs/GUIA_USUARIO.md` | Guía de instalación y uso |
| `MANIFIESTO.md` | Filosofía Low-End First |
| `CONTRIBUTING.md` | Guía de contribución |
| `ry-galery_contenido/` | 🆕 Videos + screenshots + assets para demos |

---

<div align="center">

**🛡️ Ry-Dit v0.16.1 — Snake + Buscaminas + Action Sprite + Tilemap 2.0**

*23 crates · 144 tests · 12 crates.io · 15+ demos · 0 errores · Low-End First*

*3 Pilares: 🎮 Gaming · 🎬 Animaciones+Ciencia · 📡 Streaming+Comunidad*

**Próximo: v0.17.0 — Sprite sheets reales + Texturas + Emojis + GIF + Audio**

> 📋 `ROADMAP.md` · `TASKS.md` · `TASKS_2.md` (análisis estratégico) · `QWEN.md` (bitácora)

</div>
