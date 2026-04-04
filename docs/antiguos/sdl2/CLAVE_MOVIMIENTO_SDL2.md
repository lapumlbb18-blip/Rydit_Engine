# 🛡️ CLAVE DEL MOVIMIENTO SDL2 - DOCUMENTACIÓN

**Fecha**: 2026-03-31  
**Versión**: v0.11.0  
**Estado**: ✅ **MOVIMIENTO SDL2 RESUELTO**

---

## 🔑 **LA CLAVE: DOS EVENTOS**

El **secreto** para que el movimiento funcione en SDL2 (Termux-X11) es usar **DOS eventos separados**:

```rust
// ✅ EVENTO 1: Primera pulsación (sin repeat)
Event::KeyDown { keycode: Some(keycode), repeat: false, .. } => {
    // Se ejecuta SOLO la primera vez que presionás la tecla
    match keycode {
        Keycode::D => jugador_x += velocidad,
        Keycode::A => jugador_x -= velocidad,
        _ => {}
    }
}

// ✅ EVENTO 2: Tecla mantenida (con repeat = true)
Event::KeyDown { keycode: Some(keycode), repeat: true, .. } => {
    // Se ejecuta MIENTRAS tenés presionada la tecla
    match keycode {
        Keycode::D => jugador_x += velocidad,
        Keycode::A => jugador_x -= velocidad,
        _ => {}
    }
}
```

---

## ❌ **ERROR COMÚN (NO FUNCIONA)**

```rust
// ❌ MAL: Solo usa un evento
Event::KeyDown { keycode: Some(keycode), .. } => {
    // Esto SOLO detecta la primera pulsación
    // NO hay movimiento continuo
    match keycode {
        Keycode::D => jugador_x += velocidad,
        _ => {}
    }
}
```

**Resultado**: El personaje salta pero **NO se mueve** hacia los lados.

---

## ✅ **SOLUCIÓN COMPLETA (FUNCIONA)**

```rust
'reading: while running {
    for event in event_pump.poll_iter() {
        match event {
            // Salir
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                running = false;
                break 'running;
            }

            // ✅ PRIMERA PULSACIÓN
            Event::KeyDown { keycode: Some(keycode), repeat: false, .. } => {
                match keycode {
                    Keycode::W | Keycode::Up | Keycode::Space => {
                        if en_suelo {
                            velocidad_y = fuerza_salto;
                            en_suelo = false;
                        }
                    }
                    _ => {}
                }
            }

            // ✅ TECLA MANTENIDA (movimiento continuo)
            Event::KeyDown { keycode: Some(keycode), repeat: true, .. } => {
                match keycode {
                    Keycode::D | Keycode::Right => {
                        jugador_x += velocidad_movimiento * dt;
                    }
                    Keycode::A | Keycode::Left => {
                        jugador_x -= velocidad_movimiento * dt;
                    }
                    _ => {}
                }
            }

            _ => {}
        }
    }
}
```

---

## 📊 **DEMO COMPLETO - CARACTERÍSTICAS**

### **Físicas**
| Parámetro | Valor | Descripción |
|-----------|-------|-------------|
| **Gravedad** | 800.0 | Aceleración hacia abajo |
| **Fuerza de salto** | -500.0 | Velocidad inicial del salto |
| **Velocidad movimiento** | 250.0 | Velocidad lateral |
| **DT** | 0.016 | Delta time (60 FPS) |

---

### **Colisiones**
| Tipo | Detección | Respuesta |
|------|-----------|-----------|
| **Suelo** | `bottom() <= plataforma.y` | `velocidad_y = 0`, `en_suelo = true` |
| **Techo** | `top() >= plataforma.bottom()` | `velocidad_y = 0` |
| **Pared izquierda** | `right() <= plataforma.x` | `jugador_x = plataforma.x - ancho` |
| **Pared derecha** | `left() >= plataforma.right()` | `jugador_x = plataforma.right()` |

---

### **Controles**
| Tecla | Acción |
|-------|--------|
| **A / ←** | Mover izquierda (mantener) |
| **D / →** | Mover derecha (mantener) |
| **W / ↑ / SPACE** | Saltar |
| **ESC** | Salir |

---

## 🎮 **DEMO FILES**

| Archivo | Descripción | Estado |
|---------|-------------|--------|
| `test_callback_sdl2.rs` | Test original (funciona) | ✅ |
| `demo_movimiento.rs` | Demo movimiento básico | ✅ |
| `demo_platformer_completo.rs` | Demo completo con plataformas | ✅ |

---

## 🛡️ **POR QUÉ FUNCIONA EN TERMUX-X11**

SDL2 en Android/Termux-X11 usa **eventos** en vez de **polling**:

| Método | Funciona en Termux-X11 | Notas |
|--------|------------------------|-------|
| **Event Loop** (`poll_iter()`) | ✅ SÍ | Eventos de teclado reales |
| **Polling** (`glfwGetKey()`) | ❌ NO | No hay estado de teclado |

**Raylib/GLFW** usa polling → **NO funciona**  
**SDL2** usa event loop → **SÍ funciona**

---

## 📝 **EJEMPLO MÍNIMO**

```rust
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Test", 800, 600).build().unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut jugador_x: i32 = 400;
    let velocidad: i32 = 5;
    let mut running = true;

    'running: while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => running = false,
                
                // ✅ CLAVE: Ambos eventos
                Event::KeyDown { keycode: Some(keycode), repeat: false, .. } |
                Event::KeyDown { keycode: Some(keycode), repeat: true, .. } => {
                    match keycode {
                        Keycode::D => jugador_x += velocidad,
                        Keycode::A => jugador_x -= velocidad,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        canvas.fill_rect(sdl2::rect::Rect::new(jugador_x, 300, 50, 50)).unwrap();
        canvas.present();
    }
}
```

---

## 🎯 **PRÓXIMOS PASOS**

1. ✅ **Movimiento SDL2** - Resuelto
2. ✅ **Plataformas** - Implementadas
3. ✅ **Colisiones** - Funcionando
4. ⚠️ **Sprites** - Pendiente (assets::load_texture_sdl2)
5. ⚠️ **Texto** - Pendiente (SDL2_ttf o ab_glyph)
6. ⚠️ **Parser** - Pendiente (zero-copy + bytecode)

---

<div align="center">

**🛡️ Clave del Movimiento SDL2 - DOCUMENTADA**

*repeat: false ✅ | repeat: true ✅ | Movimiento continuo ✅*

**Próximo: Sprites + Texto + Parser**

</div>
