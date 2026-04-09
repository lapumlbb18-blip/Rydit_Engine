# 🛡️ Ry-Dit - Guía del Usuario

**Versión**: v0.16.0
**Última actualización**: 2026-04-09

---

## 📋 Índice

1. [¿Qué es Ry-Dit?](#qué-es-ry-dit)
2. [Requisitos](#requisitos)
3. [Instalación](#instalación)
4. [Ejecutar Demos](#ejecutar-demos)
5. [Controles de los Demos](#controles-de-los-demos)
6. [Scripting .rydit](#scripting-rydit)
7. [Crear Niveles con .rydit](#crear-niveles-con-rydit)
8. [Troubleshooting](#troubleshooting)

---

## ¿Qué es Ry-Dit?

Ry-Dit es un **motor de juegos 2D + lenguaje de scripting en Rust**, diseñado para funcionar en dispositivos de gama baja como el Redmi Note 8 (Adreno 610) corriendo Android/Termux.

### Características principales

- **Motor 2D completo**: Sprites PNG, texto TTF, física, colisiones, audio
- **Lenguaje de scripting .rydit**: Scripting en español con matemáticas, arrays, Vec2
- **GPU Instancing**: Hasta 150K partículas en un solo draw call
- **FSR 1.0**: Upscaling AMD para mejorar rendimiento
- **Health Bars + HUD**: Sistema de HUD world-space con color dinámico
- **Cámara 2D avanzada**: Zoom, rotación, follow suave, límites de mapa
- **12 crates publicados** en crates.io
- **Multi-plataforma**: Android/Termux, Linux, Windows

---

## Requisitos

### Mínimos
- **Android 8+** con Termux + Termux-X11
- **RAM**: 2GB mínimo
- **GPU**: Adreno 610 o equivalente (soporte OpenGL ES 3.2)
- **Almacenamiento**: 500MB libre

### Recomendados
- **Linux** (Ubuntu/Debian) o **Windows** con WSL2
- **Rust 1.70+** instalado
- **SDL2, SDL2_ttf, SDL2_image, SDL2_mixer** development libraries
- **raylib** (opcional, para features 3D)

---

## Instalación

### 1. Clonar el repositorio

```bash
git clone https://github.com/lapumlbb18-blip/Ry-dit.git
cd Ry-dit
```

### 2. Instalar dependencias (Termux)

```bash
pkg update && pkg upgrade
pkg install rust pkg-config sdl2 sdl2_ttf sdl2_image sdl2_mixer
pkg install zink mesa virglrenderer
```

### 3. Instalar dependencias (Linux Ubuntu/Debian)

```bash
sudo apt install build-essential pkg-config libssl-dev
sudo apt install libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev libsdl2-mixer-dev
```

### 4. Compilar

```bash
# Debug (rápido, para desarrollo)
cargo build -p ry-rs --bin rydit-rs

# Release (optimizado, para jugar)
cargo build -p ry-rs --bin rydit-rs --release
```

### 5. Ejecutar

```bash
# Motor principal
cargo run -p ry-rs --bin rydit-rs --release

# O directamente el ELF compilado
./target/release/rydit-rs
```

---

## Ejecutar Demos

### Demos principales

```bash
# Juego completo: torreta vs sprites (3 niveles)
cargo run -p ry-rs --bin demo_torreta_vs_sprites --release

# GPU Instancing: 50K partículas
cargo run -p ry-rs --bin demo_gpu_instancing --release

# FSR 1.0: Upscaling 960x540 → 1280x720
cargo run -p ry-rs --bin demo_fsr --release

# Health Bars + Cámara 2D + HUD + Minimap
cargo run -p ry-rs --bin demo_hud_camera --release

# Física + colisiones + audio + TTF
cargo run -p ry-rs --bin demo_rigidbody --release

# Showcase ry-anim (12 principios Disney)
cargo run -p ry-rs --bin demo_anime_ry --release
```

### Con Launchers Zink (Termux-X11)

```bash
# Auto-detección DISPLAY + Zink + GPU Adreno
./launcher_hud_camera.sh
./launcher_gpu_instancing.sh
./launcher_fsr.sh
./launcher_sdl2.sh
./launcher_torreta.sh
```

### Lista completa de demos

| Demo | Descripción |
|------|-------------|
| `demo_hud_camera` | Health bars + Cámara 2D + Debug overlay + Minimap |
| `demo_gpu_instancing` | 50K partículas GPU instancing |
| `demo_fsr` | FSR 1.0 upscaling |
| `demo_torreta_vs_sprites` | Juego completo: 3 niveles, boss fights |
| `demo_rigidbody` | Física + colisiones + audio |
| `demo_anime_ry` | Showcase ry-anim |
| `demo_panel_visual` | 4 paneles + consola interactiva |
| `demo_menu_bar` | Menús Dear ImGui |
| `demo_ttf_sprites` | Texto TTF + sprites PNG |
| `demo_platformer_completo` | Plataformas + gravedad + salto |
| `demo_50k_particulas` | 50K partículas simples |
| `demo_colisiones` | Sistema de colisiones |

---

## Controles de los Demos

### demo_torreta_vs_sprites

| Tecla | Acción |
|-------|--------|
| ← → ó A/D | Mover torreta |
| W ó ↑ | Saltar |
| S ó ↓ | Bajar rápido |
| SPACE | Disparar |
| P | Pausa |
| R | Reiniciar nivel |
| ESC | Salir / Volver menú |

### demo_hud_camera

| Tecla | Acción |
|-------|--------|
| ← → ↑ ↓ ó WASD | Mover cámara |
| + / - | Zoom in/out (0.2x - 5.0x) |
| Q / E | Rotación (-/+ 15°) |
| R | Reset cámara |
| D | Toggle debug overlay |
| M | Toggle minimap |
| H | Toggle health bars |
| ESC | Salir |

### demo_gpu_instancing

| Tecla | Acción |
|-------|--------|
| 1-6 | 10K/25K/50K/75K/100K/150K partículas |
| ← → ↑ ↓ ó WASD | Mover cámara |
| + / - | Tamaño de partículas |
| P | Pausar animación |
| R | Regenerar partículas |
| ESC | Salir |

### demo_fsr

| Tecla | Acción |
|-------|--------|
| F | Cycle calidad (Quality → Balanced → Performance) |
| E | Toggle FSR ON/OFF |
| A | Toggle auto-detect |
| ESC | Salir |

### Controles generales (panel visual)

| Tecla | Acción |
|-------|--------|
| 1-4 | Cambiar panel (Screen, Console, Input, Controls) |
| ESC | Salir |

---

## Scripting .rydit

Ry-Dit incluye un **lenguaje de scripting en español** que permite crear lógica de juego sin compilar.

### Sintaxis básica

```rydit
# Variables
mi_variable = 10
nombre = "Hola Mundo"
posicion = vec2(100, 200)

# Matemáticas
resultado = sin(PI / 2)
distancia = sqrt(pow(x, 2) + pow(y, 2))

# Condicionales
si vida > 0 entonces
    imprimir("Jugador vivo")
sino
    imprimir("Game Over")
fin

# Bucles
repetir 10 veces
    imprimir("Iteración")
fin

# Funciones
funcion saludar(nombre)
    imprimir("Hola " + nombre)
fin

saludar("Mundo")
```

### Funciones disponibles

#### Matemáticas
| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `sin(x)` | Seno | `sin(PI / 2)` → 1.0 |
| `cos(x)` | Coseno | `cos(0)` → 1.0 |
| `tan(x)` | Tangente | `tan(PI / 4)` → 1.0 |
| `sqrt(x)` | Raíz cuadrada | `sqrt(16)` → 4.0 |
| `pow(x, y)` | Potencia | `pow(2, 3)` → 8.0 |
| `log(x)` | Logaritmo natural | `log(E)` → 1.0 |
| `abs(x)` | Valor absoluto | `abs(-5)` → 5.0 |
| `floor(x)` | Redondear abajo | `floor(3.7)` → 3.0 |
| `ceil(x)` | Redondear arriba | `ceil(3.2)` → 4.0 |
| `lerp(a, b, t)` | Interpolación lineal | `lerp(0, 10, 0.5)` → 5.0 |

#### Arrays
| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `push(arr, elem)` | Agregar elemento | `push(lista, 5)` |
| `pop(arr)` | Remover último | `pop(lista)` |
| `len(arr)` | Longitud | `len(lista)` |
| `contains(arr, elem)` | Contiene elemento | `contains(lista, 3)` |
| `join(arr, sep)` | Unir con separador | `join(lista, ", ")` |

#### Vec2
| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `vec2(x, y)` | Crear vector | `vec2(100, 200)` |
| `add(a, b)` | Sumar vectores | `add(v1, v2)` |
| `normalize(v)` | Normalizar | `normalize(vec2(3, 4))` → (0.6, 0.8) |
| `dist(a, b)` | Distancia | `dist(jugador, enemigo)` |
| `lerp(a, b, t)` | Interpolar | `lerp(pos1, pos2, 0.5)` |

### Ejecutar scripts

```bash
# Desde el motor
./target/release/rydit-rs mi_script.rydit

# Modo REPL interactivo
./target/release/rydit-rs
```

---

## Crear Niveles con .rydit

### Estructura de un nivel

Los niveles se definen con el módulo `ry-config` que parsea archivos de configuración:

```rydit
# nivel1.rydit

# Entidades
entidad "jugador" {
    x = 100
    y = 300
    vida = 100
    sprite = "sprites/jugador.png"
}

entidad "enemigo" {
    x = 500
    y = 300
    vida = 50
    sprite = "sprites/enemigo.png"
    ai = "patrol"
}

# Plataformas
plataforma {
    x = 0
    y = 400
    ancho = 800
    alto = 20
}

plataforma {
    x = 200
    y = 300
    ancho = 100
    alto = 10
}

# Cámara
camara {
    follow = "jugador"
    zoom = 1.0
    limites = { x = 0, y = 0, ancho = 1200, alto = 800 }
}
```

### Checkpoints

```rydit
checkpoint {
    x = 400
    y = 200
    nombre = "Mitad del nivel"
}
```

### HUD Configuration

```rydit
hud {
    health_bars = true
    debug_overlay = false
    minimap = true
    stats = {
        score = true
        tiempo = true
        nivel = "Nivel 1"
    }
}
```

---

## Troubleshooting

### Error: SDL2 no encontrado

**Síntoma**: `error: could not find native static library`

**Solución**:
```bash
# Termux
pkg install sdl2 sdl2_ttf sdl2_image sdl2_mixer

# Ubuntu/Debian
sudo apt install libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev libsdl2-mixer-dev
```

### Error: DISPLAY no configurado (Termux-X11)

**Síntoma**: `Cannot initialize SDL video`

**Solución**:
```bash
# Usar launchers con auto-detección
./launcher_hud_camera.sh

# O configurar manualmente
export DISPLAY=:0
```

### Rendimiento bajo en GPU Instancing

**Síntoma**: Menos de 30 FPS con 50K partículas

**Solución**:
```bash
# Forzar Zink (GPU) en vez de llvmpipe (CPU)
export MESA_LOADER_DRIVER_OVERRIDE=zink
export GALLIUM_DRIVER=zink

# Verificar GPU activa
glxinfo | grep "OpenGL renderer"
```

### Error: Shaders no encontrados

**Síntoma**: Crash al iniciar demo con shaders

**Causa**: Shaders desde path relativo no se encuentran

**Solución**: Ya fixeado en v0.15.0 - shaders embebidos con `include_str!()`

### Error: Texto no se muestra

**Síntoma**: Pantalla sin texto TTF

**Solución**:
```bash
# Verificar SDL2_ttf instalado
pkg install sdl2_ttf  # Termux
sudo apt install libsdl2-ttf-dev  # Linux

# Verificar fuentes del sistema
ls /usr/share/fonts/
```

### Error: Audio no funciona

**Síntoma**: Sin sonido al jugar

**Solución**:
```bash
# Verificar SDL2_mixer
pkg install sdl2_mixer  # Termux
sudo apt install libsdl2-mixer-dev  # Linux
```

### Crash en demo con muchos sprites

**Síntoma**: Panic con "out of memory"

**Solución**:
- Reducir número de entidades en el nivel
- Usar GPU Instancing para partículas (1 draw call vs N draw calls)
- Activar FSR 1.0 para reducir resolución interna

### Problemas de compilación

```bash
# Limpiar y recompilar
cargo clean
cargo build -p ry-rs --release

# Verificar workspace
cargo check --workspace

# Ejecutar tests
cargo test --workspace
```

### Problemas con crates.io

```bash
# Actualizar índice de crates
cargo update

# Verificar crates publicados
cargo search ry-anim
cargo search ry-god
cargo search ry-stream
cargo search v-shield
```

---

## Recursos Adicionales

| Recurso | URL |
|---------|-----|
| **Repositorio** | `https://github.com/lapumlbb18-blip/Ry-dit` |
| **Documentación técnica** | `QWEN.md` |
| **Estructura del proyecto** | `ESTRUCTURA.md` |
| **Roadmap** | `ROADMAP.md` |
| **Tareas pendientes** | `TASKS.md` |
| **Manifiesto** | `MANIFIESTO.md` |
| **crates.io** | `https://crates.io/crates/ry-anim` |

---

<div align="center">

**🛡️ Ry-Dit v0.16.0 - Guía del Usuario**

*Construido sin prisa, madurado con paciencia*

*23 crates · 12 publicados · 0 errores · Low-End First*

</div>
