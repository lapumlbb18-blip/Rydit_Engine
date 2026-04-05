# 🎮 Arquitectura de Demos y Crates — v0.12.0

**Fecha**: 2026-04-05
**Versión**: v0.12.0

---

## 📊 Estado de los Demos

| Demo | Backend | Input | Texto | Sprites | Físicas | Audio |
|------|---------|-------|-------|---------|---------|-------|
| `demo_rigidbody` | SDL2 puro | ✅ SDL2 directo | ✅ SDL2_ttf | ✅ SDL2_image | ✅ Rigid body | ❌ |
| `demo_50k_particulas` | SDL2 puro | ✅ | ❌ | ❌ | ✅ Partículas | ❌ |
| `demo_colisiones` | SDL2 puro | ✅ | ❌ | ❌ | ✅ | ❌ |
| `demo_carga_sprites` | SDL2 puro | ✅ | ✅ | ✅ | ❌ | ❌ |
| `demo_anime_ry` | raylib (RyditGfx) | ⚠️ Necesita capa input | ✅ raylib | ✅ raylib | ❌ | ❌ |

### El Patrón que Funciona (SDL2)

```
demo_rigidbody.rs (483 líneas, 446K release)
    ↓
sdl2::init() → sdl2::video() → sdl2::render::Canvas
    ↓
EventPump: event_pump.poll_iter()
    ↓
Event::KeyDown { keycode, repeat: false, .. }
    ↓
Game loop nativo: update(dt) → render → present
    ↓
ELF ARM aarch64 446K funcional
```

### El Problema con raylib (RyditGfx)

```
demo_anime_ry.rs (297 líneas, 341K release)
    ↓
RyditGfx::new() → raylib::init()
    ↓
raylib maneja input INTERNO (no compatible con Termux-X11)
    ↓
Solución futura: crear capa de input externa para raylib
    o usar SDL2 para input + raylib para render
```

---

## 🏗️ Arquitectura Propuesta

### Capas Actuales

```
┌─────────────────────────────────────────────────┐
│              Binario .rs (demo)                 │
├─────────────────────┬───────────────────────────┤
│      ry-gfx         │      SDL2 directo         │
│  (texturas, render) │   (input, audio, eventos) │
├─────────────────────┼───────────────────────────┤
│     raylib C lib    │     SDL2 libs (.so)       │
│  (solo dibujado)    │  (input, audio, video)    │
└─────────────────────┴───────────────────────────┘
```

### Capa Futura: Input/Touch para raylib

```
┌─────────────────────────────────────────────────┐
│              Binario .rs (demo)                 │
├─────────────────────┬───────────────────────────┤
│      ry-gfx         │    ry-input (futuro)      │
│  (texturas, render) │   (SDL2 input → raylib)   │
├─────────────────────┼───────────────────────────┤
│     raylib C lib    │     SDL2 libs (.so)       │
│  (solo dibujado)    │  (solo input/touch)       │
└─────────────────────┴───────────────────────────┘
```

### ry-input (Crate Propuesto)

| Función | Descripción | Backend |
|---------|-------------|---------|
| `poll_events()` | Obtener eventos SDL2 | SDL2 |
| `get_key_state(key)` | Estado de tecla | SDL2 |
| `get_touch_pos()` | Posición de touch | SDL2 touch |
| `get_mouse_pos()` | Posición de mouse | SDL2 mouse |
| `is_key_pressed(key)` | Primera pulsación | SDL2 KeyDown repeat:false |
| `is_key_held(key)` | Tecla mantenida | SDL2 KeyDown repeat:true |
| `get_touch_count()` | Número de touches | SDL2 Multi-touch |

**Uso con raylib:**
```rust
use ry_gfx::RyditGfx;  // Solo para dibujado
use ry_input::InputManager; // Solo para input

let mut gfx = RyditGfx::new("Mi Juego", 800, 600);
let mut input = InputManager::new();

while !gfx.should_close() {
    input.poll_events();
    
    if input.is_key_pressed(Keycode::Left) {
        // Mover jugador
    }
    
    if let Some((tx, ty)) = input.get_touch_pos() {
        // Touch en Termux-X11
    }
    
    // Dibujar con raylib
    gfx.draw_circle(x, y, 20, ColorRydit::Rojo);
}
```

---

## 🎯 Por Qué SDL2 Funciona Mejor para Input

| Aspecto | SDL2 | raylib |
|---------|------|--------|
| **Event loop** | `poll_iter()` con eventos claros | `IsKeyPressed()` polling |
| **repeat:false** | ✅ `repeat: false` en KeyDown | ❌ No expone repeat |
| **Termux-X11** | ✅ Probado y funcional | ⚠️ Requiere capa extra |
| **Touch** | ✅ SDL_TouchDevice | ⚠️ No soportado |
| **Multi-touch** | ✅ SDL_Finger | ❌ No soportado |
| **Lorie** | ✅ Compatible | ❌ No probado |

---

## 📋 Demos Funcionales (Referencia para Crear Juegos)

### demo_rigidbody.rs — El Template

**Estructura:**
```rust
// 1. Inicializar SDL2
let sdl = sdl2::init()?;
let video = sdl.video()?;
let window = video.window("Juego", 800, 600).build()?;
let mut canvas = window.into_canvas().build()?;

// 2. Cargar recursos
let font = load_font("font.ttf")?;
let texture = load_texture("sprite.png")?;

// 3. Game loop
let mut event_pump = sdl.event_pump()?;
'running: loop {
    for event in event_pump.poll_iter() {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
            Event::KeyDown { keycode: Some(code), repeat: false, .. } => {
                // Input de una pulsación
            }
            _ => {}
        }
    }
    
    // Update
    update(dt);
    
    // Render
    canvas.clear();
    draw_sprites(&mut canvas);
    canvas.present();
}
```

### Binarios que SÍ Compilan (Patrón SDL2)

| Binario | Líneas | Tamaño release | Usa |
|---------|--------|----------------|-----|
| `demo_rigidbody` | 483 | 446K | SDL2 directo |
| `demo_50k_particulas` | ~500 | 313K | SDL2 directo |
| `demo_colisiones` | ~400 | ~400K | SDL2 directo |
| `demo_carga_sprites` | ~300 | 438K | SDL2 directo |
| `demo_stream` | ~200 | ~300K | SDL2 directo |

---

## 🚀 Hoja de Ruta

### Sesión Actual (Completada)
- ✅ ry-anim v0.12.0 completo (41 funciones, 58 tests)
- ✅ demo_anime_ry ELF compilado (341K release)
- ✅ Fix linking raylib en build.rs
- ✅ Guía de compilación documentada

### Próxima Sesión
1. Crear `crates/ry-input/` — SDL2 input para usar con raylib
2. Implementar `InputManager` con `poll_events()`, `is_key_pressed()`, `get_touch_pos()`
3. Crear demo que use `ry-gfx` (render) + `ry-input` (input)
4. Probar con Termux-X11 + Lorie

### Futuro (v0.13.0+)
- ry-input + ry-gfx = combinación perfecta para juegos
- Soporte touch nativo para Termux-X11
- Multi-touch para gestos
- Gamepad support

---

<div align="center">

**🎮 Arquitectura de Demos — Ry-Dit v0.12.0**

*SDL2 para input + raylib para render = combinación ideal*

*demo_rigidbody.rs es el template para crear nuevos juegos*

*ry-input crate propuesto para la próxima sesión*

</div>
