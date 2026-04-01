# 🛡️ RyDit - ESTRUCTURA DEL PROYECTO

**Última actualización**: 2026-04-01
**Versión**: v0.11.0 ✅ RYBOT + SDL2 + TOOLKIT
**Commit**: Ver `git log -n 1`
**Estado**: Compilación 100% + Features funcionales

---

## 🎯 ARQUITECTURA v0.11.0

```
rydit-engine/
├── crates/
│   ├── lizer/                  # ⚠️ Parser (70% funcional)
│   │   └── src/
│   │       ├── lib.rs          # 3327 líneas (monolítico)
│   │       ├── lexer.rs        # Tokenización
│   │       ├── parser.rs       # Parsing
│   │       └── ast.rs          # AST sin tipos
│   │
│   ├── rydit-core/             # ✅ RyditModule trait
│   │   └── src/lib.rs          # Trait + ModuleRegistry
│   │
│   ├── rydit-ecs/              # ✅ ECS (bevy_ecs)
│   │   ├── src/
│   │   │   ├── lib.rs          # EcsWorld
│   │   │   ├── components.rs   # Position, Velocity, Sprite
│   │   │   └── systems.rs      # Movement, Render
│   │   └── Cargo.toml          # bevy_ecs = "0.15"
│   │
│   ├── rydit-gfx/              # ✅ Backend + Toolkit UI
│   │   ├── src/
│   │   │   ├── lib.rs          # RyditGfx + exports
│   │   │   ├── backend_sdl2.rs # ✅ SDL2 ventana + input + render
│   │   │   ├── input_sdl2.rs   # ✅ Event Loop (69 teclas)
│   │   │   ├── sdl2_ffi.rs     # ✅ FFI nativo (TTF, Image, Mixer)
│   │   │   ├── toolkit/        # 🆕 UI Toolkit v0.11.0
│   │   │   │   ├── mod.rs
│   │   │   │   ├── theme.rs    # Dark/Light themes
│   │   │   │   └── widgets/
│   │   │   │       ├── button.rs   # Botones clickeables
│   │   │   │       ├── label.rs    # Texto SDL2_ttf
│   │   │   │       └── panel.rs    # Contenedores
│   │   │   ├── gpu_instancing.rs   # ✅ 100K partículas
│   │   │   ├── ecs_render.rs       # ✅ ECS + rlgl
│   │   │   ├── render_queue.rs     # ✅ 8192 draw calls
│   │   │   └── shaders/            # vertex.glsl, fragment.glsl
│   │   └── Cargo.toml              # sdl2 = "0.37" + raylib
│   │
│   ├── rydit-rs/                   # ✅ Core + RyBot
│   │   ├── src/
│   │   │   ├── bin/                # 🆕 Binarios de prueba
│   │   │   │   ├── demo_toolkit_ry.rs      # 🆕 UI Toolkit demo
│   │   │   │   ├── rybot_cli.rs            # 🆕 RyBot CLI
│   │   │   │   ├── test_sdl2_basico.rs     # 🆕 SDL2 test simple
│   │   │   │   ├── test_sdl2_sprite_debug.rs # 🆕 Sprite debug
│   │   │   │   ├── demo_particles.rs       # Partículas
│   │   │   │   ├── demo_big_bang.rs        # Explosión cósmica
│   │   │   │   └── snake.rs                # Juego Snake
│   │   │   ├── rybot/            # 🆕 RyBot Inspector v0.11.0
│   │   │   │   ├── mod.rs        # RyBot struct
│   │   │   │   └── registry.rs   # Registry + Alertas (530 líneas)
│   │   │   ├── modules/          # Sistema Ry (180K líneas)
│   │   │   │   ├── camera.rs     # ✅ 16.9K líneas
│   │   │   │   ├── entity.rs     # ✅ 88.8K líneas
│   │   │   │   ├── level.rs      # ✅ 17.2K líneas
│   │   │   │   ├── assets.rs     # ✅ 15.6K líneas
│   │   │   │   ├── physics.rs    # ✅ 22.8K líneas
│   │   │   │   ├── input_map.rs  # ✅ 21.1K líneas
│   │   │   │   └── particles.rs  # ✅ 7K líneas
│   │   │   ├── executor.rs       # ✅ Game loop con RyBot
│   │   │   ├── main.rs           # Entry point
│   │   │   └── lib.rs            # Config parser
│   │   └── Cargo.toml
│   │
│   ├── rydit-physics/            # ✅ Físicas 2D
│   │   └── src/lib.rs            # 20 funciones
│   │
│   ├── rydit-anim/               # ✅ Animaciones
│   │   └── src/lib.rs            # 8.8K líneas
│   │
│   ├── rydit-science/            # ✅ Funciones científicas
│   │   └── src/lib.rs            # 18.1K líneas
│   │
│   ├── rydit-loader/             # ✅ Dynamic module loader
│   │   └── src/lib.rs
│   │
│   └── migui/                    # ✅ Separado (sin usar en RyDit)
│       └── src/
│           ├── lib.rs
│           └── backend_sdl2.rs
│
├── demos/                        # Scripts .rydit
│   ├── demo_particles.rydit
│   ├── demo_big_bang.rydit
│   └── snake.rydit
│
├── logo_icon_asst/               # Assets de prueba
│   └── sprites/
│       ├── cube_8x8.png
│       ├── crate_8x8.png
│       ├── tank_16x16.png
│       └── helicopter_16x16.png
│
├── target/                       # ⚠️ EXCLUIDO DE DRIVE
│   ├── release/                  # Binarios compilados
│   └── debug/
│
├── .sync_drive.log.binaries      # 🆕 Log de sincronización
├── .sync_exclude                 # 🆕 Exclusiones de Drive
├── sync_drive.sh                 # 🆕 Script de sincronización
│
├── README.md                     # Documentación principal
├── ESTRUCTURA.md                 # Este archivo
├── QWEN.md                       # Bitácora técnica
├── ESTADO_COMPLETO_V0.11.0.md    # 🆕 Estado completo v0.11.0
└── Cargo.toml                    # Workspace config
```

---

## 📊 CRATES PRINCIPALES

### **rydit-gfx** ✅ 100%

| Módulo | Líneas | Funciones | Estado |
|--------|--------|-----------|--------|
| `backend_sdl2.rs` | 360 | Ventana, Input, Render | ✅ |
| `input_sdl2.rs` | 210 | Event Loop, 69 teclas | ✅ |
| `sdl2_ffi.rs` | 370 | TTF, Image, Mixer FFI | ✅ |
| `toolkit/` | 200+ | Button, Label, Panel | ✅ 90% |
| `gpu_instancing.rs` | - | 100K partículas | ✅ |
| `render_queue.rs` | 600+ | 8192 draw calls | ✅ |

**Total**: ~2K líneas + shaders GLSL

---

### **rydit-rs** ✅ 90%

| Módulo | Líneas | Funciones | Estado |
|--------|--------|-----------|--------|
| `rybot/` | 530 | Registry, Alertas, CLI | ✅ 80% |
| `modules/camera.rs` | 16.9K | `apply_sdl2()` | ✅ |
| `modules/entity.rs` | 88.8K | `render_sdl2()` | ✅ 95% |
| `modules/level.rs` | 17.2K | `render_sdl2()` | ✅ 90% |
| `modules/assets.rs` | 15.6K | `load_texture_sdl2()` | ✅ 90% |
| `modules/physics.rs` | 22.8K | 20 funciones | ✅ |
| `modules/input_map.rs` | 21.1K | 69 teclas + gamepad | ✅ |
| `executor.rs` | - | Game loop con RyBot | ✅ |

**Total**: ~180K líneas Sistema Ry

---

### **Crates Externos** ✅ 100%

| Crate | Líneas | Funciones | Estado |
|-------|--------|-----------|--------|
| `rydit-physics` | 22.8K | Físicas 2D | ✅ |
| `rydit-anim` | 8.8K | Animaciones | ✅ |
| `rydit-science` | 18.1K | Matemáticas + Geometría | ✅ |
| `rydit-loader` | - | Dynamic module loader | ✅ |
| `rydit-core` | - | RyditModule trait | ✅ |

---

## 🆕 BINARIOS DE PRUEBA v0.11.0

### **SDL2 + Toolkit**

| Binario | Función | Estado |
|---------|---------|--------|
| `demo_toolkit_ry` | UI Toolkit demo | ✅ Funciona |
| `test_sdl2_basico` | SDL2 básico | ✅ 60 FPS |
| `test_sdl2_sprite_debug` | Debug sprites | ✅ 470 frames |
| `rybot_cli` | RyBot CLI | ✅ status/inspect/logs |

### **Demos Clásicas**

| Binario | Función | Estado |
|---------|---------|--------|
| `demo_particles` | Partículas | ✅ Funciona |
| `demo_big_bang` | Explosión cósmica | ✅ Funciona |
| `demo_10k_particulas` | 10K partículas | ✅ 30-50 FPS |
| `snake` | Juego Snake | ✅ Funciona |

---

## 📁 ARCHIVOS CLAVE

### **Documentación**

| Archivo | Líneas | Propósito |
|---------|--------|-----------|
| `ESTADO_COMPLETO_V0.11.0.md` | 500+ | Estado completo del proyecto |
| `QWEN.md` | 990+ | Bitácora técnica |
| `README.md` | 1334+ | Documentación principal |
| `ESTRUCTURA.md` | Este archivo | Arquitectura del proyecto |

### **Configuración**

| Archivo | Propósito |
|---------|-----------|
| `Cargo.toml` | Workspace definition |
| `.sync_exclude` | Exclusiones de Drive |
| `sync_drive.sh` | Script de sincronización |

---

## 🔄 SINCRONIZACIÓN CON GOOGLE DRIVE

### **Archivos Sincronizados** ✅

- ✅ Todo el código fuente (`.rs`, `.toml`, `.md`)
- ✅ Documentación completa
- ✅ Assets (`.png`, `.jpg`)
- ✅ Scripts (`.sh`)

### **Archivos NO Sincronizados** ❌

- ❌ `target/` completo (binarios compilados)
- ❌ `.git/` (repositorio)
- ❌ Archivos temporales

### **Excepciones (Binarios Clave)** ⚠️

Solo se sincronizan los binarios de prueba:
- ✅ `target/release/demo_toolkit_ry`
- ✅ `target/release/test_sdl2_*`
- ✅ `target/release/rybot_cli`
- ✅ `target/release/*.d` (debug info)

**Configuración**: Ver `.sync_exclude`

---

## 🎯 FLUJO DE TRABAJO

### **Desarrollo**

```bash
# 1. Compilar binario de prueba
cargo build --bin demo_toolkit_ry --release

# 2. Ejecutar con GPU activada
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1
./target/release/demo_toolkit_ry

# 3. Ver estado con RyBot CLI
./target/release/rybot_cli status
```

### **Sincronización**

```bash
# Sincronizar con Drive (segundo plano)
./sync_drive.sh

# Ver log de sincronización
cat .sync_drive.log.binaries
```

### **Git**

```bash
# Commit + Push
git add -A
git commit -m "feat: Descripción"
git push origin main
```

---

## 📊 MÉTRICAS v0.11.0

| Métrica | Valor |
|---------|-------|
| **Líneas Rust Total** | ~250K |
| **Crates** | 13 activos |
| **Binarios Compilados** | 15+ |
| **Tests Passing** | 260+ |
| **Warnings Críticos** | 0 |
| **Compilación** | ✅ 100% |

---

<div align="center">

**🛡️ RyDit v0.11.0 - ESTRUCTURA COMPLETA**

*SDL2 ✅ | Toolkit ✅ | RyBot ✅ | GPU ✅ | Docs ✅*

**Próximo: FSR 1.0 + Parser Modular**

</div>
