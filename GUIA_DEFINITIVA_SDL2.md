# 🛡️ GUÍA DEFINITIVA SDL2 - Código que FUNCIONÓ

**Fecha**: 2026-04-03  
**Versión**: v0.11.6  
**Basado en**: Sesiones anteriores que FUNCIONARON (v0.10.4 - v0.11.0)  

---

## 🔑 DESCUBRIMIENTO CLAVE: Input SDL2 funciona en Termux-X11

### El Problema

Android/Termux-X11 **NO soporta polling de teclado** (GLFW/Raylib).  
**SÍ soporta event loop** (SDL2/X11 callbacks).

| Método | Funciona | Por qué |
|--------|----------|---------|
| **Raylib `IsKeyDown()`** | ❌ NO | Usa GLFW polling |
| **GLFW `glfwGetKey()`** | ❌ NO | Polling puro |
| **SDL2 `poll_iter()`** | ✅ SÍ | Usa X11 event queue |
| **X11 `XNextEvent()`** | ✅ SÍ | Callback directo |

### La Solución: PATRÓN DOS EVENTOS

```rust
// ✅ EVENTO 1: Primera pulsación (repeat: false)
Event::KeyDown {
    keycode: Some(keycode),
    repeat: false,
    ..
} => {
    // Se ejecuta SOLO la primera vez que presionás la tecla
}

// ✅ EVENTO 2: Tecla mantenida (repeat: true) - LA CLAVE DEL MOVIMIENTO
Event::KeyDown {
    keycode: Some(keycode),
    repeat: true,
    ..
} => {
    // Se ejecuta MIENTRAS tenés presionada la tecla
    // ESTO es lo que da movimiento continuo
}
```

**Error común**: Usar solo un evento sin verificar `repeat` → el personaje salta pero NO se mueve.

---

## 📁 ARCHIVOS QUE FUNCIONARON

### 1. test_callback_sdl2.rs (ORIGINAL)
**Estado**: Eliminado pero documentado en `docs/RESUMEN_SESION_SDL2_2026-03-31.md`  
**Copia exacta**: `ejemplos-gfx/pendientes-revision/demo_movimiento.rs`

### 2. demo_movimiento.rs ✅
**Ubicación**: `ejemplos-gfx/pendientes-revision/demo_movimiento.rs`  
**Estado**: FUNCIONAL - 100% copiado del original

```rust
// ESTRUCTURA CLAVE:
let sdl_context = sdl2::init().unwrap();
let video_subsystem = sdl_context.video().unwrap();
let window = video_subsystem.window("Demo", 800, 600)
    .position_centered().opengl().build().unwrap();
let mut canvas = window.into_canvas().present_vsync().build().unwrap();
let mut event_pump = sdl_context.event_pump().unwrap();

'reading: while running {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                running = false;
                break 'running;
            }
            
            // ✅ DOS EVENTOS:
            Event::KeyDown { keycode: Some(keycode), repeat: false, .. } |
            Event::KeyDown { keycode: Some(keycode), repeat: true, .. } => {
                match keycode {
                    Keycode::W | Keycode::Up => jugador_y -= velocidad,
                    Keycode::S | Keycode::Down => jugador_y += velocidad,
                    Keycode::A | Keycode::Left => jugador_x -= velocidad,
                    Keycode::D | Keycode::Right => jugador_x += velocidad,
                    _ => {}
                }
            }
            _ => {}
        }
    }
    
    // Render
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.fill_rect(Rect::new(jugador_x - 25, jugador_y - 25, 50, 50)).unwrap();
    canvas.present();
}
```

### 3. demo_sdl2_puro.rs ✅
**Ubicación**: `ejemplos-gfx/pendientes-revision/demo_sdl2_puro.rs`  
**Estado**: FUNCIONAL - Demo más completo con logging de eventos

### 4. demo_platformer_completo.rs ✅
**Ubicación**: `crates/rydit-rs/src/bin/demo_platformer_completo.rs`  
**Estado**: FUNCIONAL - Platformer con gravedad, salto y colisiones

**Características**:
- Gravedad: 800.0
- Fuerza salto: -500.0
- Velocidad movimiento: 250.0
- 6 plataformas (suelo + 5 en aire)
- Colisiones: suelo, techo, paredes
- Respawn si cae al vacío

### 5. demo_sprites_final.rs ✅ (NUEVO)
**Ubicación**: `crates/rydit-rs/src/bin/demo_sprites_final.rs`  
**Estado**: FUNCIONAL - Combina input correcto + verificación sprites

---

## 🎮 DEMOS QUE FUNCIONARON (Histórico)

| Demo | Versión | Estado | Notas |
|------|---------|--------|-------|
| **test_callback_sdl2** | v0.10.4 | ✅ Funcionó | Descubrió input SDL2 |
| **demo_sdl2_puro** | v0.10.4 | ✅ Funcionó | SDL2 puro completo |
| **demo_movimiento** | v0.11.0 | ✅ Funcionó | Copia del original |
| **demo_platformer_completo** | v0.11.0 | ✅ Funcionó | Gravedad + colisiones |
| **test_sdl2_sprite_debug** | v0.11.0 | ✅ Funcionó | 470 frames con textura |
| **demo_particles** | v0.10.3 | ✅ Funcionó | 60 FPS, 5 efectos |
| **demo_10k_particulas** | v0.10.3 | ✅ Funcionó | 30-50 FPS, 10K partículas |
| **ecs_demo_10k** | v0.10.3 | ✅ Funcionó | 10K entidades |

---

## 🛠️ ESTRUCTURA BASE PARA BINARIOS SDL2

### Template Mínimo Funcional

```rust
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() {
    // 1. Inicializar SDL2 DIRECTO
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem
        .window("Mi Demo", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    // 2. Estado del juego
    let mut x = 400i32;
    let mut y = 300i32;
    let vel = 5i32;
    let mut running = true;
    
    // 3. Game loop
    'running: while running {
        // 3.1 INPUT (PATRÓN DOS EVENTOS)
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                
                // Evento 1: Primera pulsación
                Event::KeyDown { keycode: Some(key), repeat: false, .. } |
                // Evento 2: Tecla mantenida (MOVIMIENTO CONTINUO)
                Event::KeyDown { keycode: Some(key), repeat: true, .. } => {
                    match key {
                        Keycode::Left => x -= vel,
                        Keycode::Right => x += vel,
                        Keycode::Up => y -= vel,
                        Keycode::Down => y += vel,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        
        // 3.2 UPDATE
        // (físicas, lógica, etc.)
        
        // 3.3 RENDER
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // Dibujar jugador
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(Rect::new(x - 25, y - 25, 50, 50)).unwrap();
        
        canvas.present();
    }
}
```

---

## 📊 FUNCIONES CLAVE

### Input SDL2

| Función | Uso | Notas |
|---------|-----|-------|
| `event_pump.poll_iter()` | Iterar eventos | ✅ Funciona en Termux-X11 |
| `repeat: false` | Primera pulsación | Salto, disparo, etc. |
| `repeat: true` | Tecla mantenida | Movimiento continuo |
| `Keycode::W/A/S/D` | WASD | Alternativa a flechas |
| `Keycode::Up/Down/Left/Right` | Flechas | Input direccional |

### Render SDL2

| Función | Uso | Ejemplo |
|---------|-----|---------|
| `canvas.set_draw_color()` | Establecer color | `Color::RGB(255, 0, 0)` |
| `canvas.clear()` | Limpiar pantalla | - |
| `canvas.fill_rect()` | Dibujar rect relleno | `Rect::new(x, y, w, h)` |
| `canvas.draw_rect()` | Dibujar borde rect | `Rect::new(x, y, w, h)` |
| `canvas.draw_line()` | Dibujar línea | `Point::new(x1, y1), Point::new(x2, y2)` |
| `canvas.present()` | Mostrar frame | **Llamar al final** |

### Colisiones (como demo_platformer_completo.rs)

```rust
// Jugador rect
let jugador_rect = Rect::new(x as i32, y as i32, ancho, alto);

// Colisión con plataforma
for plataforma in &plataformas {
    if jugador_rect.has_intersection(*plataforma) {
        // Colisión desde arriba (aterrizar)
        if jugador_rect.bottom() <= plataforma.y + 10 && velocidad_y > 0.0 {
            y = plataforma.y as f32 - alto as f32;
            velocidad_y = 0.0;
            en_suelo = true;
        }
        // Colisión desde abajo (golpear cabeza)
        else if jugador_rect.top() >= plataforma.bottom() - 10 && velocidad_y < 0.0 {
            y = plataforma.bottom() as f32;
            velocidad_y = 0.0;
        }
    }
}
```

---

## ⚠️ LO QUE NO FUNCIONA

### ❌ Sdl2Backend (rydit_gfx)

**Problema**: Lifetimes complejos con `TextureCreator`  
**Solución**: Usar SDL2 directo como `demo_movimiento.rs`

### ❌ Carga de texturas con lifetime

**Problema**: `Texture<'a>` ligado a `TextureCreator<'a>`  
**Solución actual**: Verificar archivos + rects de colores  
**Solución futura**: Gestor de assets dedicado con `unsafe transmute`

### ❌ Raylib polling

**Problema**: No funciona en Android/Termux-X11  
**Solución**: Usar SDL2 event loop

---

## 🚀 COMANDO DE EJECUCIÓN

```bash
# Con Zink + DRI3 (recomendado)
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

# Ejecutar demo
DISPLAY=:0 ./target/release/demo_sprites_final

# O con cargo
cargo run --bin demo_sprites_final --release
```

---

## 📚 DOCUMENTOS DE REFERENCIA

| Documento | Ubicación | Contenido |
|-----------|-----------|-----------|
| **CLAVE_MOVIMIENTO_SDL2.md** | `CLAVE_MOVIMIENTO_SDL2.md` | Patrón DOS EVENTOS |
| **RESUMEN_SESION_SDL2** | `docs/RESUMEN_SESION_SDL2_2026-03-31.md` | Descubrimiento input SDL2 |
| **ASSETS_SDL2_COMPLETADA** | `docs/ASSETS_SDL2_COMPLETADA.md` | load_texture_sdl2() |
| **ESTADO_COMPLETO_V0.11.0** | `docs/ESTADO_COMPLETO_V0.11.0.md` | Estado completo v0.11.0 |

---

## ✅ CHECKLIST PARA NUEVO BINARIO SDL2

- [ ] Usar `sdl2::init()` directo (NO `Sdl2Backend`)
- [ ] Implementar PATRÓN DOS EVENTOS (`repeat: false` + `repeat: true`)
- [ ] Usar `event_pump.poll_iter()` para input
- [ ] Verificar archivos antes de cargar texturas
- [ ] Usar rects de colores como fallback si no hay texturas
- [ ] Probar con `DISPLAY=:0 ./target/release/demo`
- [ ] Compilar con `--release` para mejor rendimiento

---

<div align="center">

**🛡️ GUÍA DEFINITIVA SDL2 - RyDit v0.11.6**

*Basado en código que FUNCIONÓ (v0.10.4 - v0.11.0)*

**PATRÓN CLAVE: DOS EVENTOS (`repeat: false` + `repeat: true`)**

</div>
