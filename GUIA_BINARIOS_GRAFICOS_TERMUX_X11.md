# 🛡️ GUÍA: CREACIÓN DE BINARIOS GRÁFICOS PARA TERMUX-X11

**Fecha**: 2026-04-02  
**Versión RyDit**: v0.11.5 (0 errores | 0 warnings)  
**Próxima versión**: v0.11.6 - Snake reescrito + Platformer SDL2  
**Objetivo**: Guía completa para crear binarios gráficos y archivos .rydit desde cero

---

## 📋 ÍNDICE

1. [Contexto Histórico](#contexto-histórico)
2. [Arquitectura de Binarios](#arquitectura-de-binarios)
3. [Binarios Eliminados y Por Qué](#binarios-eliminados)
4. [Estructura de un Binario Gráfico](#estructura-de-un-binario-gráfico)
5. [Guía Paso a Paso: Crear Binario SDL2](#guía-paso-a-paso-crear-binario-sdl2)
6. [Guía Paso a Paso: Crear Archivo .rydit](#guía-paso-a-paso-crear-archivo-rydit)
7. [Templates Reutilizables](#templates-reutilizables)
8. [Ejemplos Completos](#ejemplos-completos)
9. [Testing en Termux-X11](#testing-en-termux-x11)
10. [Checklist de Validación](#checklist-de-validación)

---

## 📜 CONTEXTO HISTÓRICO

### Situación Previa (v0.11.4)

- **132 problemas totales**: 34 lifetimes + 18 errores + 80 warnings
- **Binarios rotos**: Múltiples binarios con errores de compilación
- **Parser refactorizado**: `lizer` → `rydit-parser` con lifetimes `'a`
- **Regla de oro**: NUNCA usar `sed` después de refactorizar parser

### Solución Aplicada (v0.11.5)

- ✅ **0 errores | 0 warnings**
- ✅ **20+ binarios compilados**
- ✅ **SDL2 backend funcional** con helpers
- ✅ **7 binarios esenciales** en `crates/rydit-rs/src/bin/`
- ✅ **11 binarios Plan B** en `ejemplos-gfx/pendientes-revision/`
- ✅ **21 binarios pendientes** en `ejemplos-gfx/pendientes/`

### Binarios Eliminados (v0.11.1)

| Binario | Razón | Errores |
|---------|-------|---------|
| `test_callback_glfw.rs` | GLFW no declarado, legacy | 19 errores |
| `test_raylib_callback.rs` | API mixta (GLFW + Raylib) | 7 errores |
| `test_solo_audio.rs` | Unsafe code error | 1 error |

**Filosofía**: "Código roto debe eliminarse, no fixearse" - menos código que mantener

---

## 🏗️ ARQUITECTURA DE BINARIOS

### Estructura Actual del Proyecto

```
crates/rydit-rs/src/bin/
├── snake.rs                          ✅ Snake Game (VM bytecode)
├── demo_particles.rs                 ✅ Sistema de partículas (5 efectos)
├── demo_platformer_completo.rs       ✅ Platformer SDL2
├── demo_stream.rs                    ✅ Streaming demo
├── rybot_cli.rs                      ✅ RyBot CLI
├── scene_runner.rs                   ✅ Scene runner genérico
├── nivel3_test_lowend.rs             ✅ Test gráficos low-end
├── nivel3_test_audio_lowend.rs       ✅ Test audio low-end
├── nivel3_test_input_lowend.rs       ✅ Test input low-end
├── debug_*.rs                        ✅ Debug tests (7 archivos)
└── archive/                          📁 Binarios eliminados (vacío)

ejemplos-gfx/
├── pendientes-revision/              📋 11 binarios Plan B (más compatibles)
│   ├── demo_movimiento.rs
│   ├── demo_particulas_sdl2.rs
│   ├── demo_sdl2_puro.rs
│   ├── demo_toolkit_ry.rs
│   └── ...
└── pendientes/                       ⏳ 17 binarios por verificar
    ├── demo_10k_particulas.rs
    ├── demo_big_bang.rs
    ├── ecs_demo_10k.rs
    └── ...

demos/
├── snake.rydit                       ✅ Snake script
├── demo_termux_x11.rydit             ✅ Test Termux-X11
├── demo_particulas.rydit             ✅ Partículas script
├── platformer_demo.rydit             ✅ Platformer script
└── ... (147 archivos .rydit totales)
```

---

## 🗑️ BINARIOS ELIMINADOS

### Lecciones Aprendidas de Eliminaciones

1. **GLFW vs SDL2**: SDL2 es el backend oficial, GLFW fue eliminado
2. **APIs mixtas**: No mezclar raylib + GLFW + SDL2 en mismo binario
3. **Unsafe innecesario**: Evitar unsafe sin justificación clara
4. **Código legacy**: Eliminar, no mantener "por si acaso"

### Qué NO Hacer

```rust
// ❌ MAL: Mezclar APIs
use raylib;
use sdl2;
use glfw;  // ← Dependency no declarado

// ❌ MAL: Unsafe sin justificación
unsafe {
    // Código que podría ser safe
}

// ❌ MAL: Callbacks complejos sin necesidad
fn callback_handler(ptr: *mut c_void) {
    // Demasiado complejo para uso simple
}
```

---

## 🏗️ ESTRUCTURA DE UN BINARIO GRÁFICO

### Template Base SDL2

```rust
//! demo_ejemplo.rs
//! Descripción: Template base para binario SDL2
//! Uso: cargo run --bin demo_ejemplo

use rydit_gfx::sdl2_backend::{
    SDL2Backend,
    ColorRydit,
    InputSDL2,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Inicializar SDL2
    let mut gfx = SDL2Backend::new(800, 600, "Demo Ejemplo")?;
    
    // 2. Configurar colores
    let rojo = ColorRydit::new(255, 0, 0);
    let verde = ColorRydit::new(0, 255, 0);
    let azul = ColorRydit::new(0, 0, 255);
    let blanco = ColorRydit::new(255, 255, 255);
    let negro = ColorRydit::new(0, 0, 0);
    
    // 3. Variables de estado
    let mut x = 400.0;
    let mut y = 300.0;
    let mut corriendo = true;
    
    println!("🎮 Demo Ejemplo iniciada");
    
    // 4. Game loop
    while corriendo {
        // 4.1 Input
        let input = InputSDL2::poll();
        
        if input.key_pressed(sdl2::keyboard::Keycode::Escape) {
            corriendo = false;
        }
        
        if input.key_held(sdl2::keyboard::Keycode::Left) {
            x -= 5.0;
        }
        if input.key_held(sdl2::keyboard::Keycode::Right) {
            x += 5.0;
        }
        if input.key_held(sdl2::keyboard::Keycode::Up) {
            y -= 5.0;
        }
        if input.key_held(sdl2::keyboard::Keycode::Down) {
            y += 5.0;
        }
        
        // 4.2 Update (físicas, lógica)
        // ... tu código aquí ...
        
        // 4.3 Render
        gfx.clear_background(negro)?;
        
        // Dibujar formas
        gfx.draw_circle(x, y, 50.0, rojo)?;
        gfx.draw_rect(x - 60.0, y - 60.0, 120.0, 120.0, verde)?;
        gfx.draw_line(x - 100.0, y, x + 100.0, y, azul)?;
        gfx.draw_text("Demo RyDit v0.11.6", 250, 20, 24, blanco)?;
        
        // 4.4 Presentar frame
        gfx.present()?;
    }
    
    println!("✅ Demo Ejemplo finalizada");
    Ok(())
}
```

### Template con RyditModule (Carga .rydit)

```rust
//! scene_ejemplo.rs
//! Carga y ejecuta archivo .rydit como configuración
//! Uso: cargo run --bin scene_ejemplo

use rydit_rs::RyditCore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Cargando escena: ejemplo.rydit");
    
    // 1. Crear core
    let mut core = RyditCore::new();
    
    // 2. Cargar script .rydit
    core.load_script("demos/ejemplo.rydit")?;
    
    // 3. Ejecutar escena
    core.run()?;
    
    println!("✅ Escena completada");
    Ok(())
}
```

---

## 📝 GUÍA PASO A PASO: CREAR BINARIO SDL2

### Paso 1: Crear Archivo de Binario

```bash
# Navegar al directorio de binarios
cd crates/rydit-rs/src/bin/

# Crear nuevo archivo (ejemplo: demo_formas.rs)
touch demo_formas.rs
```

### Paso 2: Escribir Código Base

Usar el template base de arriba como punto de partida.

### Paso 3: Agregar Imports Necesarios

```rust
// SDL2 Backend (OBLIGATORIO)
use rydit_gfx::sdl2_backend::{
    SDL2Backend,      // Backend principal
    ColorRydit,       // Sistema de colores
    InputSDL2,        // Input de teclado
};

// SDL2 Keycode (para input)
use sdl2::keyboard::Keycode;
```

### Paso 4: Implementar Game Loop

Estructura obligatoria:

```
while corriendo {
    1. Input   → Leer teclado/mouse
    2. Update  → Físicas, lógica, colisiones
    3. Render  → Dibujar formas/sprites
    4. Present → Mostrar frame en pantalla
}
```

### Paso 5: Helpers SDL2 Disponibles

```rust
// Helpers simplificados (v0.11.5+)
gfx.clear_background(negro)?;                    // Limpiar pantalla
gfx.draw_rect_color(x, y, w, h, color)?;         // Dibujar rectángulo
gfx.draw_text_color(texto, x, y, color)?;        // Dibujar texto
```

### Paso 6: Compilar y Probar

```bash
# Compilar binario
cargo build -p rydit-rs --bin demo_formas

# Ejecutar en Termux-X11
cargo run --bin demo_formas

# Build release (optimizado)
cargo build -p rydit-rs --bin demo_formas --release
```

---

## 📝 GUÍA PASO A PASO: CREAR ARCHIVO .RYDIT

### Sintaxis Básica

```rydit
# 1. Inicialización
shield.init

# 2. Variables
dark.slot x = 400
dark.slot y = 300
dark.slot velocidad = 5

# 3. Game loop
ryda frame < 10000 {
    dark.slot frame = frame + 1
    
    # Input
    onif tecla_presionada("arrow_left") {
        dark.slot x = x - velocidad
    }
    
    # Render
    draw.circle(x, y, 50, "rojo")
    draw.text("Frame: " + frame, 10, 10, 20, "blanco")
    
    # Salir
    onif tecla_presionada("escape") {
        romper
    }
}
```

### Palabras Clave

| Palabra | Significado | Ejemplo |
|---------|-------------|---------|
| `shield.init` | Inicializar motor | `shield.init` |
| `dark.slot` | Declarar variable | `dark.slot x = 10` |
| `ryda` | Bucle (while/for) | `ryda x < 100 { }` |
| `onif` | Condicional (if) | `onif x > 5 { }` |
| `romper` | Break/return | `romper` |
| `voz` | Print/return | `voz "Hola"` |
| `rytmo` | Función | `rytmo saludar() { }` |

### Módulos Disponibles

```rydit
# Importar módulos
import math
import arrays
import strings
import random
import time
import io
import files
import json
import colisiones
import regex
```

### Ejemplo Completo: Snake

```rydit
# snake.rydit
shield.init

# Variables del juego
dark.slot serpiente = [[10, 10], [9, 10], [8, 10]]
dark.slot direccion = "derecha"
dark.slot comida = [15, 10]
dark.slot puntuacion = 0
dark.slot velocidad = 10
dark.slot frame = 0
dark.slot corriendo = 1

ryda corriendo == 1 {
    dark.slot frame = frame + 1
    
    # Input de dirección
    onif tecla_presionada("arrow_up") {
        dark.slot direccion = "arriba"
    }
    onif tecla_presionada("arrow_down") {
        dark.slot direccion = "abajo"
    }
    onif tecla_presionada("arrow_left") {
        dark.slot direccion = "izquierda"
    }
    onif tecla_presionada("arrow_right") {
        dark.slot direccion = "derecha"
    }
    
    # Mover serpiente
    dark.slot cabeza = serpiente[0]
    dark.slot nueva_cabeza = []
    
    onif direccion == "derecha" {
        dark.slot nueva_cabeza = [cabeza[0] + 1, cabeza[1]]
    }
    onif direccion == "izquierda" {
        dark.slot nueva_cabeza = [cabeza[0] - 1, cabeza[1]]
    }
    onif direccion == "arriba" {
        dark.slot nueva_cabeza = [cabeza[0], cabeza[1] - 1]
    }
    onif direccion == "abajo" {
        dark.slot nueva_cabeza = [cabeza[0], cabeza[1] + 1]
    }
    
    # Verificar colisión con comida
    onif nueva_cabeza[0] == comida[0] y nueva_cabeza[1] == comida[1] {
        dark.slot puntuacion = puntuacion + 10
        dark.slot comida = [random::int(5, 35), random::int(5, 25)]
    } blelse {
        arrays::pop_atras(serpiente)
    }
    
    arrays::insertar(serpiente, 0, nueva_cabeza)
    
    # Render
    draw.rect(0, 0, 800, 600, "negro")
    
    # Dibujar serpiente
    dark.slot i = 0
    ryda i < arrays::length(serpiente) {
        dark.slot segmento = serpiente[i]
        draw.rect(segmento[0] * 20, segmento[1] * 20, 18, 18, "verde")
        dark.slot i = i + 1
    }
    
    # Dibujar comida
    draw.rect(comida[0] * 20, comida[1] * 20, 18, 18, "rojo")
    
    # Dibujar puntuación
    draw.text("Puntuación: " + puntuacion, 10, 10, 20, "blanco")
    
    # Delay para velocidad
    ryda frame < frame + velocidad {
        # Empty loop for delay
    }
    
    # Salir con ESC
    onif tecla_presionada("escape") {
        dark.slot corriendo = 0
    }
}

voz "Game Over! Puntuación: " + puntuacion
```

---

## 📦 TEMPLATES REUTILIZABLES

### Template 1: Formas Geométricas

```rust
//! demo_formas.rs
use rydit_gfx::sdl2_backend::{SDL2Backend, ColorRydit, InputSDL2};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut gfx = SDL2Backend::new(800, 600, "Demo Formas")?;
    
    let colores = [
        ColorRydit::new(255, 0, 0),    // Rojo
        ColorRydit::new(0, 255, 0),    // Verde
        ColorRydit::new(0, 0, 255),    // Azul
        ColorRydit::new(255, 255, 0),  // Amarillo
        ColorRydit::new(255, 0, 255),  // Magenta
    ];
    
    let blanco = ColorRydit::new(255, 255, 255);
    let negro = ColorRydit::new(0, 0, 0);
    
    let mut frame = 0;
    let mut corriendo = true;
    
    while corriendo {
        frame += 1;
        let input = InputSDL2::poll();
        
        if input.key_pressed(sdl2::keyboard::Keycode::Escape) {
            corriendo = false;
        }
        
        gfx.clear_background(negro)?;
        
        // Círculos
        for i in 0..5 {
            let x = 100 + (i as f32 * 150.0);
            let y = 100 + (frame as f32 * 0.5).sin() * 50.0;
            gfx.draw_circle(x, y, 30.0, colores[i])?;
        }
        
        // Rectángulos
        for i in 0..5 {
            let x = 100 + (i as f32 * 150.0);
            let y = 300 + (frame as f32 * 0.3).cos() * 40.0;
            gfx.draw_rect(x - 25.0, y - 25.0, 50.0, 50.0, colores[i])?;
        }
        
        // Texto
        gfx.draw_text("Demo Formas Geométricas", 250, 20, 24, blanco)?;
        gfx.draw_text(&format!("Frame: {}", frame), 10, 10, 16, blanco)?;
        
        gfx.present()?;
    }
    
    Ok(())
}
```

### Template 2: Sistema de Partículas

```rust
//! demo_particulas_simple.rs
use rydit_gfx::sdl2_backend::{SDL2Backend, ColorRydit, InputSDL2};

struct Particula {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    vida: f32,
    color: ColorRydit,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut gfx = SDL2Backend::new(800, 600, "Demo Partículas")?;
    
    let mut particulas: Vec<Particula> = Vec::new();
    let negro = ColorRydit::new(0, 0, 0);
    let blanco = ColorRydit::new(255, 255, 255);
    
    let mut frame = 0;
    let mut corriendo = true;
    
    while corriendo {
        frame += 1;
        let input = InputSDL2::poll();
        
        if input.key_pressed(sdl2::keyboard::Keycode::Escape) {
            corriendo = false;
        }
        
        // Spawn nuevas partículas
        if frame % 2 == 0 {
            particulas.push(Particula {
                x: 400.0,
                y: 300.0,
                vx: (frame as f32).sin() * 5.0,
                vy: (frame as f32).cos() * 5.0 - 3.0,
                vida: 1.0,
                color: ColorRydit::new(255, (frame * 5) as u8 % 256, 0),
            });
        }
        
        // Update partículas
        particulas.retain(|p| p.vida > 0.0);
        for p in &mut particulas {
            p.x += p.vx;
            p.y += p.vy;
            p.vy += 0.1; // Gravedad
            p.vida -= 0.01;
        }
        
        // Render
        gfx.clear_background(negro)?;
        
        for p in &particulas {
            let alpha = (p.vida * 255.0) as u8;
            gfx.draw_circle(p.x, p.y, 5.0, p.color)?;
        }
        
        gfx.draw_text("Demo Partículas", 300, 20, 24, blanco)?;
        gfx.draw_text(&format!("Partículas: {}", particulas.len()), 10, 10, 16, blanco)?;
        
        gfx.present()?;
    }
    
    Ok(())
}
```

### Template 3: Input + Movimiento

```rust
//! demo_input.rs
use rydit_gfx::sdl2_backend::{SDL2Backend, ColorRydit, InputSDL2};
use sdl2::keyboard::Keycode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut gfx = SDL2Backend::new(800, 600, "Demo Input")?;
    
    let mut jugador = (400.0, 300.0);
    let mut velocidad = 5.0;
    let negro = ColorRydit::new(0, 0, 0);
    let blanco = ColorRydit::new(255, 255, 255);
    let cyan = ColorRydit::new(0, 255, 255);
    
    let mut corriendo = true;
    
    println!("🎮 Controles: ← → ↑ ↓ para mover, ESC para salir");
    
    while corriendo {
        let input = InputSDL2::poll();
        
        // Input
        if input.key_pressed(Keycode::Escape) {
            corriendo = false;
        }
        
        if input.key_held(Keycode::Left) {
            jugador.0 -= velocidad;
        }
        if input.key_held(Keycode::Right) {
            jugador.0 += velocidad;
        }
        if input.key_held(Keycode::Up) {
            jugador.1 -= velocidad;
        }
        if input.key_held(Keycode::Down) {
            jugador.1 += velocidad;
        }
        
        // Límites de pantalla
        jugador.0 = jugador.0.clamp(50.0, 750.0);
        jugador.1 = jugador.1.clamp(50.0, 550.0);
        
        // Render
        gfx.clear_background(negro)?;
        gfx.draw_circle(jugador.0, jugador.1, 30.0, cyan)?;
        gfx.draw_text("Jugador", 350, 20, 24, blanco)?;
        gfx.draw_text(&format!("Posición: ({:.0}, {:.0})", jugador.0, jugador.1), 10, 10, 16, blanco)?;
        gfx.draw_text("Controles: ← → ↑ ↓", 10, 550, 16, blanco)?;
        gfx.draw_text("ESC: Salir", 10, 570, 16, blanco)?;
        gfx.present()?;
    }
    
    Ok(())
}
```

---

## 🎨 EJEMPLOS COMPLETOS

### Ejemplo 1: Demo Formas (.rydit)

**Archivo**: `demos/demo_formas.rydit`

```rydit
# Demo de formas geométricas animadas
shield.init

dark.slot frame = 0
dark.slot x = 400
dark.slot y = 300

ryda frame < 5000 {
    dark.slot frame = frame + 1
    
    # Fondo
    draw.rect(0, 0, 800, 600, "negro")
    
    # Círculos animados
    dark.slot i = 0
    ryda i < 10 {
        dark.slot i = i + 1
        dark.slot cx = 100 + (i * 60)
        dark.slot cy = 200 + (frame + i * 30) % 200
        draw.circle(cx, cy, 30, "rojo")
    }
    
    # Rectángulos
    dark.slot j = 0
    ryda j < 5 {
        dark.slot j = j + 1
        dark.slot rx = 50 + (j * 140)
        dark.slot ry = 450 + (frame + j * 50) % 100
        draw.rect(rx, ry, 80, 80, "verde")
    }
    
    # Texto de info
    draw.text("Demo Formas v0.11.6", 280, 20, 24, "amarillo")
    draw.text("Frame: " + frame, 10, 10, 16, "blanco")
    
    # Salir
    onif tecla_presionada("escape") {
        romper
    }
}

voz "Demo completada! Frames: " + frame
```

**Ejecutar**:
```bash
cargo run -p rydit-rs --bin rydit-rs -- --gfx demos/demo_formas.rydit
```

### Ejemplo 2: Platformer (.rydit)

**Archivo**: `demos/platformer_simple.rydit`

```rydit
# Platformer simple
shield.init

dark.slot jugador_x = 100
dark.slot jugador_y = 400
dark.slot jugador_vy = 0
dark.slot en_suelo = 1
dark.slot gravedad = 0.8
dark.slot salto = -12
dark.slot frame = 0

ryda frame < 10000 {
    dark.slot frame = frame + 1
    
    # Input horizontal
    onif tecla_presionada("arrow_right") {
        dark.slot jugador_x = jugador_x + 5
    }
    onif tecla_presionada("arrow_left") {
        dark.slot jugador_x = jugador_x - 5
    }
    
    # Salto
    onif en_suelo == 1 y tecla_presionada("space") {
        dark.slot jugador_vy = salto
        dark.slot en_suelo = 0
    }
    
    # Gravedad
    dark.slot jugador_vy = jugador_vy + gravedad
    dark.slot jugador_y = jugador_y + jugador_vy
    
    # Suelo
    onif jugador_y > 450 {
        dark.slot jugador_y = 450
        dark.slot jugador_vy = 0
        dark.slot en_suelo = 1
    }
    
    # Límites
    onif jugador_x < 50 {
        dark.slot jugador_x = 50
    }
    onif jugador_x > 750 {
        dark.slot jugador_x = 750
    }
    
    # Render
    draw.rect(0, 0, 800, 600, "azul_oscuro")
    
    # Plataformas
    draw.rect(0, 500, 800, 100, "verde_oscuro")
    draw.rect(200, 400, 150, 20, "marrón")
    draw.rect(450, 350, 150, 20, "marrón")
    
    # Jugador
    draw.rect(jugador_x - 20, jugador_y - 40, 40, 40, "rojo")
    
    # Info
    draw.text("Platformer Simple", 300, 20, 24, "blanco")
    draw.text("← → : Mover | SPACE: Saltar", 250, 50, 16, "amarillo")
    
    # Salir
    onif tecla_presionada("escape") {
        romper
    }
}
```

### Ejemplo 3: Partículas (.rydit)

**Archivo**: `demos/particulas_fuego.rydit`

```rydit
# Sistema de partículas - Efecto fuego
shield.init

dark.slot frame = 0
dark.slot particulas_x = []
dark.slot particulas_y = []
dark.slot particulas_vy = []
dark.slot particulas_vida = []

ryda frame < 5000 {
    dark.slot frame = frame + 1
    
    # Spawn partículas
    dark.slot i = 0
    ryda i < 5 {
        dark.slot i = i + 1
        arrays::push(particulas_x, 400 + (random::int(-20, 20)))
        arrays::push(particulas_y, 500)
        arrays::push(particulas_vy, random::int(-8, -3))
        arrays::push(particulas_vida, 1.0)
    }
    
    # Update partículas
    dark.slot j = 0
    ryda j < arrays::length(particulas_vida) {
        dark.slot particulas_y[j] = particulas_y[j] + particulas_vy[j]
        dark.slot particulas_vida[j] = particulas_vida[j] - 0.02
        dark.slot j = j + 1
    }
    
    # Eliminar muertas
    arrays::filtrar_mayor(particulas_vida, 0.0)
    
    # Render
    draw.rect(0, 0, 800, 600, "negro")
    
    # Dibujar partículas
    dark.slot k = 0
    ryda k < arrays::length(particulas_x) {
        dark.slot color = "rojo"
        onif particulas_vida[k] > 0.7 {
            dark.slot color = "amarillo"
        }
        onif particulas_vida[k] > 0.4 y particulas_vida[k] <= 0.7 {
            dark.slot color = "naranja"
        }
        draw.circle(particulas_x[k], particulas_y[k], 8, color)
        dark.slot k = k + 1
    }
    
    # Base del fuego
    draw.rect(350, 520, 100, 30, "gris")
    
    # Info
    draw.text("Fuego - Partículas: " + arrays::length(particulas_x), 250, 20, 20, "blanco")
    draw.text("ESC: Salir", 10, 570, 16, "blanco")
    
    # Salir
    onif tecla_presionada("escape") {
        romper
    }
}
```

---

## 🧪 TESTING EN TERMUX-X11

### Requisitos Previos

```bash
# 1. Termux-X11 instalado
pkg install termux-x11-nightly

# 2. SDL2 instalado
pkg install sdl2 sdl2_image sdl2_ttf sdl2_mixer

# 3. Rust toolchain
pkg install rust

# 4. Variables de entorno
export DISPLAY=:0
export SDL_VIDEO_DRIVER=x11
```

### Comandos de Test

```bash
# Test 1: Gráficos low-end (formas básicas)
cargo run --bin nivel3_test_lowend

# Test 2: Audio SDL2
cargo run --bin nivel3_test_audio_lowend

# Test 3: Input SDL2
cargo run --bin nivel3_test_input_lowend

# Test 4: Partículas
cargo run --bin demo_particles

# Test 5: Platformer
cargo run --bin demo_platformer_completo

# Test 6: Snake
cargo run --bin snake

# Test 7: Script .rydit
cargo run -p rydit-rs --bin rydit-rs -- --gfx demos/demo_termux_x11.rydit
```

### Checklist de Verificación Manual

- [ ] Ventana se abre correctamente (800x600)
- [ ] Formas se dibujan (círculos, rects, líneas, texto)
- [ ] Colores se muestran correctamente
- [ ] Input de teclado funciona (← → ↑ ↓ ESC)
- [ ] FPS estables (50-60 FPS)
- [ ] Sin crashes en 5 minutos de ejecución
- [ ] Memoria RAM < 200 MB
- [ ] CPU usage < 80%

---

## ✅ CHECKLIST DE VALIDACIÓN

### Antes de Commit

- [ ] Binario compila sin errores: `cargo build --bin <nombre>`
- [ ] Binario compila sin warnings: `cargo clippy --bin <nombre>`
- [ ] Código formateado: `cargo fmt --bin <nombre>`
- [ ] Testeado en Termux-X11 (al menos 1 ejecución exitosa)
- [ ] Documentación en comentarios (qué hace, cómo usarlo)
- [ ] No usa `sed` para modificar código
- [ ] No mezcla APIs (solo SDL2, no GLFW/raylib)
- [ ] Unsafe justificado si existe

### Estructura del Commit

```bash
git add crates/rydit-rs/src/bin/nuevo_binario.rs
git commit -m "✨ Agregar binario nuevo_binario

- Template base SDL2
- Game loop con input/update/render
- Helpers SDL2 (clear_background, draw_*)
- Compatible con Termux-X11

Refs: #v0.11.6"
```

### Agregar al Cargo.toml (si es necesario)

Verificar que el binario esté registrado en `Cargo.toml`:

```toml
[[bin]]
name = "nuevo_binario"
path = "src/bin/nuevo_binario.rs"
```

---

## 🚀 PRÓXIMOS PASOS (v0.11.6)

### Binarios Planificados

1. ✅ Snake reescrito con VM bytecode
2. ✅ Platformer demo SDL2
3. ⏳ Input Map integrado con VM
4. ⏳ Asset loading con SDL2_image
5. ⏳ Audio con SDL2_mixer

### Scripts .rydit Planificados

1. ⏳ `snake_vm.rydit` - Snake con bytecode VM
2. ⏳ `platformer_vm.rydit` - Platformer con bytecode VM
3. ⏳ `nbody_gravity.rydit` - Simulación gravitacional
4. ⏳ `bezier_curves.rydit` - Curvas de Bezier animadas
5. ⏳ `fractal_demo.rydit` - Fractales (Mandelbrot)

---

## 📚 RECURSOS ADICIONALES

### Documentación Relacionada

- [README.md](../../README.md) - Documentación principal
- [ROADMAP.md](../../ROADMAP.md) - Plan de desarrollo
- [QWEN.md](../../QWEN.md) - Bitácora técnica
- [ESTADO_V0.11.5.md](../../ESTADO_V0.11.5.md) - Estado actual
- [GPU_PARTICLES_ANALISIS_V0.9.1.md](../../docs/GPU_PARTICLES_ANALISIS_V0.9.1.md) - Análisis partículas

### Binarios de Referencia

- `snake.rs` - Snake Game (mejor ejemplo de game loop)
- `demo_particles.rs` - Sistema de partículas
- `demo_platformer_completo.rs` - Platformer SDL2
- `scene_runner.rs` - Scene runner genérico

### Scripts .rydit de Referencia

- `demos/snake.rydit` - Snake clásico
- `demos/demo_termux_x11.rydit` - Test completo Termux-X11
- `demos/demo_particulas.rydit` - Sistema de partículas
- `demos/platformer_demo.rydit` - Platformer clásico

---

## ⚠️ REGLAS DE ORO

1. **NUNCA usar `sed`** después de refactorizar parser
2. **SOLO usar SDL2** (no GLFW, no raylib mezclado)
3. **Evitar `unsafe`** sin justificación clara
4. **Commit frecuente** con mensajes descriptivos
5. **Test en Termux-X11** antes de marcar como completado
6. **Documentar** qué hace cada binario en comentarios
7. **Mantener simple** - menos código = menos bugs
8. **Seguir templates** de esta guía

---

<div align="center">

**🛡️ RyDit v0.11.5 - GUÍA BINARIOS GRÁFICOS**

*Código limpio ✅ | 0 warnings ✅ | 20+ binarios compilados ✅*

**Próximo: v0.11.6 - Snake + Platformer con VM**

</div>
