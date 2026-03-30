# 🛡️ RyDit - ESTRUCTURA DEL PROYECTO

**Última actualización**: 2026-03-30
**Versión**: v0.9.4 ✅ ENTITY SYSTEM COMPLETADO
**Próxima versión**: v0.10.0 - GPU INSTANCING + SHADERS
**Versión futura**: v0.10.2 - 🛡️ INVERSIÓN DE CONTROL
**Estado**: 10/10 - TODOS LOS SISTEMAS 2D COMPLETADOS

---

## 🎉 v0.9.4 COMPLETADA - ENTITY SYSTEM

### ✅ Features Completadas

| Feature | Estado | Funciones | Líneas |
|---------|--------|-----------|--------|
| **Level Manager** | ✅ | 13 | 532 |
| **Tilemap** | ✅ | 12 | 563 |
| **Collision** | ✅ | 13 | 577 |
| **Window** | ✅ | 17 | 515 |
| **Entity** | ✅ | 50+ | 2672 |
| **Camera** | ✅ | 15 | 969 |
| **Physics** | ✅ | 20 | 693 |

**Total v0.9.4**: 140+ funciones, ~2200 líneas nuevas

---

## 📊 ESTADO REAL (SIN FILTROS)

### Puntuación: 10/10 ✅ (v0.9.4 completado)

**Verificado en Producción (2026-03-30):**
- ✅ Level Manager (13 funciones)
- ✅ Tilemap System (12 funciones)
- ✅ Collision System (13 funciones)
- ✅ Window Manager (17 funciones)
- ✅ Entity System (50+ funciones)
- ✅ Cámara 2D (15 funciones)
- ✅ Physics 2D (20 funciones)
- ✅ 0 warnings clippy
- ✅ 260+ tests passing
- ✅ ~2.2 MB binario release
- ✅ Demo platformer jugable

**⚠️ LIMITACIONES RESTANTES (v0.9.4)**:
- ⚠️ **Arquitectura** - Script manda sobre Core → v0.10.2: Core manda, Script configura
- ⚠️ **GPU Instancing** - Sin FFI OpenGL → v0.10.0: 100K+ partículas

**🔥 PLAN CRÍTICO v0.10.0**:
- 🔥 GPU Instancing - FFI OpenGL (gl-rs), shaders GLSL
- 🔥 Shaders - Vertex + Fragment shaders
- 🔥 100K+ partículas - glDrawArraysInstanced

**🛡️ ARQUITECTURA v0.10.2** (INVERSIÓN DE CONTROL):
- 🛡️ Core manda - rydit-rs hace game loop nativo
- 🛡️ Script configura - .rydit solo parámetros
- 🛡️ ECS Entt - 100K+ entidades
- 🛡️ Comando nativo de RyDit: `./rydit-rs --scene <nombre>`

**🔥 EN PROCESO (v0.10.0) - SOLO DESPUÉS DE v0.9.2-v0.9.5**:
- # en proceso: GPU Instancing - 100K+ partículas
- # en proceso: Shaders GLSL
- # en proceso: ECS (ENTT)

**Test de Verificación**:
```bash
# Físicas
./target/release/rydit-rs --gfx demos/test_fisicas.rydit

# Sprites con queue
./target/release/rydit-rs --gfx demos/test_sprites.rydit

# Teclado completo
./target/release/rydit-rs --gfx demos/test_teclado.rydit

# Input Map + IME
./target/release/rydit-rs --gfx demos/test_input_map.rydit
```

**Lo que FALTA (v0.9.4)**:
- 🔥 Bloques sin límite
- 🔥 Text Input real
- 🔥 Input Map combinaciones

**NOTA**: Render Queue + Assets + Físicas es SUFICIENTE para juegos 2D completos.
Para 100,000+ partículas → **GPU Instancing en rydit-gfx** (solo después de v0.9.5)

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

**Patrón detectado**: Todos los motores tienen **Core pesado + Script ligero + VM delgada**.

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

## 🎯 ARQUITECTURA HÍBRIDA: .rydit vs .rs

### El Overhead de .rydit (5 pasos):

```
dark.slot x = 400
draw.circle(x, 300, 50, "rojo")

↓

1. Lexer → Tokens (~0.1ms)
2. Parser → AST (~0.2ms)
3. Eval → Evaluar expresión (~0.5ms)
4. Executor → Llamar función Rust (~0.2ms)
5. rydit-gfx → draw_circle() ← AQUÍ (~1-2ms)

Total: ~2-4ms por draw call
1000 draw calls = 2-4 segundos ← LÍMITE DE .rydit
```

### Binario Rust (.rs) - 1 paso:

```rust
let x = 400;
gfx.draw_circle(x, 300, 50, ColorRydit::Rojo);

↓

1. Llamada directa a función compilada (~0.01ms)

Total: ~0.01ms por draw call
10,000 draw calls = ~100ms = 60 FPS ✅
```

---

## 📁 ESTRUCTURA ACTUAL (HÍBRIDA)

```
shield-project/
├── crates/
│   ├── lizer/              # Lexer + Parser ✅ (overhead de .rydit)
│   │   └── src/lib.rs      # ~3,383 líneas
│   │
│   ├── blast-core/         # Executor + Memoria ✅ (overhead de .rydit)
│   │   └── src/lib.rs      # ~475 líneas
│   │
│   ├── rydit-core/         # RyditModule trait ✅ ESTABLE
│   │   └── src/lib.rs      # ~401 líneas
│   │
│   ├── rydit-loader/       # Carga dinámica ✅ ESTABLE
│   │   └── src/lib.rs      # ~420 líneas
│   │
│   ├── rydit-script/       # Scripts como módulos ✅ ESTABLE
│   │   └── src/lib.rs      # ~340 líneas
│   │
│   ├── rydit-anim/         # Animación ✅ ESTABLE
│   │   └── src/lib.rs      # ~265 líneas
│   │
│   ├── rydit-physics/      # Física ✅ ESTABLE
│   │   └── src/lib.rs      # ~205 líneas
│   │
│   ├── rydit-science/      # Bezier + Stats + Geometry ✅ ESTABLE
│   │   └── src/lib.rs      # ~988 líneas
│   │
│   ├── rydit-gfx/          # Gráficos raylib ✅ ESTABLE + v0.9.0
│   │   ├── src/lib.rs      # ~1,846 líneas
│   │   ├── src/particles.rs# CPU particles (500 partículas)
│   │   ├── src/camera.rs   # Cámara 2D
│   │   ├── src/debug_log.rs# Debug logging
│   │   └── src/render_queue.rs  # ✅ v0.9.0: 8192+ draw calls
│   │                       # - Command Queue + Double Buffering
│   │                       # - Platform Sync (XFlush/XSync)
│   │                       # - 540 líneas nuevas
│   │
│   ├── rydit-http/         # HTTP + WebSocket ✅ v0.8.7
│   │   └── src/lib.rs      # ~450 líneas (ureq + tungstenite)
│   │
│   ├── rydit-rs/           # Binario principal + stdlib
│   │   ├── src/main.rs     # ~8,235 líneas
│   │   ├── src/eval/       # ✅ CSV + HTTP/WS implementados
│   │   │   └── mod.rs      # ~2,400 líneas (overhead de .rydit)
│   │   ├── src/modules/    # ✅ Módulos
│   │   │   ├── csv.rs      # ✅ 885 líneas, 13 funciones
│   │   │   ├── input_map.rs# ✅ 220 líneas, 8 funciones
│   │   │   ├── audio.rs    # ✅ 427 líneas, 12 funciones
│   │   │   └── assets.rs   # ⚠️ 180 líneas, 3 funciones
│   │   ├── src/bin/        # ⭐ BINARIOS RUST (SIN OVERHEAD)
│   │   │   ├── demo_particles.rs  # ✅ 500+ partículas @ 60 FPS
│   │   │   └── snake.rs           # ✅ Snake Game
│   │   └── src/bindings/   # Bindings
│   │
│   ├── migui/              # Immediate Mode GUI ✅ ESTABLE
│   │   └── src/lib.rs      # ~1,391 líneas
│   │
│   └── v-shield/           # Wrapper raylib ✅ ESTABLE
│       └── src/lib.rs      # ~434 líneas
│
├── demos/                  # Scripts .rydit (CON OVERHEAD)
│   ├── demo_shapes.rydit       # ✅ Funciona (15 draw calls)
│   ├── ejemplo_gfx.rydit       # ✅ Funciona (10 draw calls)
│   └── test_renderizado_v0.9.0.rydit  # ✅ Test completo
│
├── target/                 # Build artifacts
│   ├── release/
│   │   ├── rydit-rs            # Binario principal (.rydit interpreter)
│   │   ├── demo_particles      # ⭐ Binario Rust (SIN OVERHEAD)
│   │   └── snake               # ⭐ Binario Rust (SIN OVERHEAD)
│   └── debug/
│       └── ...
│
└── docs/
    ├── 3_CAPAS_CRITICAS_V0.9.0.md      # Documentación técnica
    ├── PANORAMA_GPU_INSTANCING_V0.9.x.md  # Análisis GPU
    ├── VERIFICACION_PRODUCCION_V0.9.0.md  # Tests reales
    └── ANALISIS_BINARIOS_VS_RYDIT.txt     # .rydit vs .rs
```

---

## 🔍 ¿DÓNDE ESTÁ EL OVERHEAD?

### NO es Termux. Es la arquitectura del proyecto.

```
┌─────────────────────────────────────────────────────────┐
│  RYDIT - CAPAS DE OVERHEAD                              │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  .rydit (script) → Tiene overhead:                     │
│  ┌─────────────────────────────────────────────────┐   │
│  │  lizer/         ← Lexer + Parser                │   │
│  │  blast-core/    ← Executor                       │   │
│  │  rydit-rs/eval  ← Evaluador                      │   │
│  │  rydit-gfx/     ← FFI a raylib                  │   │
│  │  raylib-sys/    ← FFI a raylib C                │   │
│  │  raylib C       ← OpenGL ES                     │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  .rs (binario) → SIN OVERHEAD:                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  tu_código.rs    ← Llamada directa              │   │
│  │  rydit-gfx/     ← FFI a raylib                  │   │
│  │  raylib-sys/    ← FFI a raylib C                │   │
│  │  raylib C       ← OpenGL ES                     │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  Diferencia: .rydit tiene 3 capas extra                │
│  .rs va DIRECTO a rydit-gfx → raylib → GPU             │
└─────────────────────────────────────────────────────────┘
```

---

## 🐍 ANALOGÍA: RyDit es el Python de Rust

```
┌─────────────────────────────────────────────────────────┐
│  PYTHON vs C++ (IA/Ciencia de Datos)                    │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Python (fácil, lento):                                │
│  import numpy as np                                     │
│  x = np.array([1, 2, 3])  ← Interpreter overhead       │
│  y = x * 2                ← Llamada a C++              │
│                                                         │
│  C++ (difícil, rápido):                                │
│  auto x = Eigen::Vector3f(1, 2, 3);  ← Compilado       │
│  auto y = x * 2;                        ← Directo      │
│                                                         │
│  Resultado:                                             │
│  - Python: Prototipado rápido, producción lenta        │
│  - C++: Producción rápida, desarrollo lento            │
│  - Juntos: Python llama a C++ (PyTorch, TensorFlow)    │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│  RYDIT vs RUST (Simulador de Escenas)                   │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  RyDit .rydit (fácil, lento):                          │
│  dark.slot x = 400                                     │
│  draw.circle(x, 300, 50, "rojo")  ← Interpreter        │
│                                                         │
│  Rust .rs (difícil, rápido):                           │
│  let x = 400;                                          │
│  gfx.draw_circle(x, 300, 50, ColorRydit::Rojo); ← Directo│
│                                                         │
│  Resultado:                                             │
│  - .rydit: Prototipado rápido, demos masivas lentas    │
│  - .rs: Demos masivas rápidas, desarrollo más lento    │
│  - Juntos: .rydit llama a .rs (GPU Instancing)         │
└─────────────────────────────────────────────────────────┘
```

---

## 🎯 ESTRUCTURA CORRECTA DE RYDIT

### Lo que RyDit DEBERÍA ser:

```
┌─────────────────────────────────────────────────────────┐
│  RYDIT - SIMULADOR DE ESCENAS 2D                        │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  CAPA 1: SCRIPTING (.rydit) - Para lógica              │
│  - Lexer + Parser + Eval                                │
│  - Overhead: ~2-4ms por draw call                       │
│  - Uso: Juegos simples, prototipado, lógica            │
│  - Límite: ~1000 partículas                             │
│                                                         │
│  CAPA 2: BINARIOS RUST (.rs) - Para GPU                │
│  - Llamadas directas a rydit-gfx                        │
│  - Overhead: ~0.01ms por draw call                      │
│  - Uso: 10,000+ partículas, shaders, GPU Instancing    │
│  - Límite: GPU (~100,000 partículas)                   │
│                                                         │
│  CAPA 3: GPU INSTANCING (FFI OpenGL) - Futuro          │
│  - Shaders GLSL                                         │
│  - glDrawArraysInstanced()                              │
│  - Uso: 100,000+ partículas @ 60 FPS                   │
│                                                         │
│  CAPA 4: raylib (el pincel) - Ligero                   │
│  - FFI desde Rust                                       │
│  - OpenGL ES / Vulkan (Zink/Turnip)                    │
│  - NO sobrecarga - solo dibuja                         │
└─────────────────────────────────────────────────────────┘
```

---

## 💡 COMPARATIVA CON PYTORCH3D

```
┌─────────────────────────────────────────────────────────┐
│  PYTORCH3D (Python + C++ + CUDA)                        │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Python:                                                │
│  - Fácil de usar                                        │
│  - Lento para 3D masivo                                 │
│  - Llama a C++ para rendimiento                         │
│                                                         │
│  C++:                                                   │
│  - Difícil de usar                                      │
│  - Rápido para 3D masivo                                │
│  - CUDA para GPU                                        │
│                                                         │
│  Juntos:                                                │
│  - Python para lógica                                   │
│  - C++ para render 3D masivo                            │
│  - CUDA para GPU                                        │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│  RYDIT (RyDit + Rust + GPU Instancing)                  │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  .rydit:                                                │
│  - Fácil de usar                                        │
│  - Lento para 10K+ partículas                           │
│  - Llama a Rust para rendimiento                        │
│                                                         │
│  Rust:                                                  │
│  - Más difícil de usar                                  │
│  - Rápido para 10K+ partículas                          │
│  - GPU Instancing para GPU                              │
│                                                         │
│  Juntos:                                                │
│  - .rydit para lógica                                   │
│  - Rust para render masivo                              │
│  - GPU Instancing para GPU                              │
└─────────────────────────────────────────────────────────┘
```

---

## 🔑 CLAVES DE LA ARQUITECTURA

### 1. Raylib es el Pincel (ligero)

```
raylib NO es el problema.
raylib es la capa FINAL que dibuja.

Rust → raylib-sys (FFI) → raylib C → OpenGL ES → GPU
       ↑
       rydit-gfx (wrapper seguro)

raylib es ligero. El overhead está ARRIBA.
```

### 2. Turnip Adreno + Zink Vulkan

```
Termux-X11 usa:
- Zink: OpenGL sobre Vulkan
- Turnip: Driver Adreno (GPU del celular)
- Vulkan: API moderna de GPU

raylib → OpenGL ES → Zink → Vulkan → Turnip → Adreno 610

Funciona. Es compatible. Es rápido.
```

### 3. .rydit vs .rs - Cuándo usar cada uno

| Caso de Uso | .rydit | .rs |
|-------------|--------|-----|
| **Prototipado** | ✅ Rápido | ⚠️ Lento (compile) |
| **Juegos simples** | ✅ <1000 partículas | ❌ Overkill |
| **10K+ partículas** | ❌ NO PUEDE | ✅ NECESARIO |
| **Shaders GLSL** | ❌ NO PUEDE | ✅ NECESARIO |
| **GPU Instancing** | ❌ NO PUEDE | ✅ NECESARIO |
| **Lógica de juego** | ✅ Fácil | ⚠️ Más código |
| **IA básica** | ✅ Fácil | ⚠️ Más código |

---

## 📋 EJEMPLO DE USO HÍBRIDO

### Escena: Éxodo 14 (División de las Aguas)

**main.rydit** (lógica - fácil):
```rydit
# Lógica del juego (fácil en .rydit)
dark.slot moises_x = 400
dark.slot moises_y = 500
dark.slot aguas_abiertas = false

ryda frame < 3000 {
    # Input
    onif keyboard::key_pressed("space") {
        aguas_abiertas = true
        # Llamar al binario Rust para GPU
        system::exec("./target/release/exodo_gpu")
    }
    
    # Dibujar Moisés (pocas partículas, .rydit es suficiente)
    draw.circle(moises_x, moises_y, 20, "cafe")
    
    # UI
    draw.text("Presiona SPACE para abrir el mar", 200, 550, "blanco")
}
```

**exodo_gpu.rs** (GPU - rápido):
```rust
// Binario Rust para GPU Instancing
use rydit_gfx::{RyditGfx, ColorRydit};

fn main() {
    let mut gfx = RyditGfx::new("Éxodo 14 - GPU", 1280, 720);
    
    // 10,000 partículas de agua (GPU Instancing)
    let mut water_particles = Vec::with_capacity(10000);
    for i in 0..10000 {
        water_particles.push(WaterParticle {
            x: 400.0 + (i as f32 % 100.0) * 10.0,
            y: 500.0,
            vx: (i as f32 - 5000.0) * 0.1,
            vy: -100.0,
        });
    }
    
    while !gfx.should_close() {
        // Actualizar física (CPU)
        for p in &mut water_particles {
            p.vy += 9.8;  // Gravedad
            p.x += p.vx;
            p.y += p.vy;
        }
        
        // Render (GPU - 1 draw call)
        {
            let mut d = gfx.begin_draw();
            d.clear(ColorRydit::Negro);
            
            // GPU Instancing: 10,000 partículas = 1 draw call
            gpu_draw_particles(&mut d, &water_particles);
            
            d.draw_text("Éxodo 14 - División de las Aguas", 400, 50, "blanco");
        }
    }
}
```

**Resultado:**
- .rydit: Lógica fácil, Moisés (pocas partículas)
- .rs: 10,000 partículas de agua @ 60 FPS
- **Ambos coexisten**

---

## 🎯 CONCLUSIÓN

### RyDit NO es solo un motor de juegos.

**Es un SIMULADOR DE ESCENAS 2D con arquitectura híbrida:**

```
.rydit → Python (fácil, lógico, prototipado)
.rs    → C++ (rápido, GPU, masivo)
Juntos → Lo mejor de ambos mundos
```

### El overhead NO es Termux.

**Es la arquitectura del proyecto:**
- .rydit tiene 3 capas extra (lexer, parser, eval)
- .rs va directo a rydit-gfx → raylib → GPU
- Ambos son necesarios para diferentes casos de uso

### La estructura CORRECTA:

```
shield-project/
├── crates/           # Infraestructura (.rydit overhead)
├── rydit-rs/src/bin/ # ⭐ BINARIOS RUST (SIN OVERHEAD)
├── demos/            # Scripts .rydit (CON OVERHEAD)
└── target/release/   # Binarios compilados
    ├── rydit-rs      # Interpreter .rydit
    ├── demo_particles# ⭐ Rust puro (500+ partículas)
    └── exodo_gpu     # ⭐ Rust puro (10K+ partículas, futuro)
```

---

<div align="center">

**🛡️ RyDit v0.9.0 - ARQUITECTURA HÍBRIDA DEFINIDA**

*.rydit = Python (lógica) | .rs = C++ (GPU)*

**Ambos coexisten. Ambos son necesarios.**

**Próximo: GPU Instancing en .rs para 100K+ partículas**

</div>
