# 🛡️ CLAVE INPUT SDL2 - Demos que FUNCIONARON

**Fecha**: 2026-04-03  
**Versión**: v0.11.6  
**Estado**: ✅ VERIFICADO Y FUNCIONAL  

---

## 🎯 LOS DOS DEMOS CLAVE

### 1. demo_colisiones.rs - Input Básico

**Propósito**: Verificar que input SDL2 funciona con teclado virtual Android  
**Ubicación**: `crates/rydit-rs/src/bin/demo_colisiones.rs`  
**Características**:
- ✅ Cuadro rojo controlable
- ✅ 6 plataformas con colisiones AABB
- ✅ Gravedad + salto
- ✅ Input repeat: false (una pulsación = acción)
- ✅ 800x600

### 2. demo_rigidbody.rs - Input Completo + Rigid Bodies

**Propósito**: Juego completo con personaje principal + objetos con físicas  
**Ubicación**: `crates/rydit-rs/src/bin/demo_rigidbody.rs`  
**Características**:
- ✅ Jugador controlable (cuadro rojo)
- ✅ 4 rigid bodies (sprites PNG) con gravedad independiente
- ✅ Colisiones jugador ↔ rigid bodies (empuje)
- ✅ Colisiones con 7 plataformas
- ✅ Texto TTF real con info en tiempo real
- ✅ Input con ← → ↑ ↓ + WASD
- ✅ 800x600

---

## 🔑 LA CLAVE: PATRÓN DE INPUT DIRECTO

### ❌ LO QUE NO FUNCIONA

```rust
// NO USAR wrapper Sdl2Backend para input
if backend.is_key_pressed("arrow_left") { ... }
if backend.is_key_pressed("space") { ... }
```

**Por qué falla**: El wrapper `Sdl2Backend` procesa eventos internamente pero el mapeo de teclas puede no coincidir con el teclado virtual de Android.

### ✅ LO QUE SÍ FUNCIONA

```rust
// ✅ PATRÓN DIRECTO - como demo_colisiones.rs y demo_rigidbody.rs
for event in backend.event_pump.poll_iter() {
    match event {
        Event::Quit { .. } |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            break;
        }

        // ✅ repeat: false (cada pulsación = acción individual)
        Event::KeyDown { keycode: Some(key), repeat: false, .. } => {
            match key {
                Keycode::Left | Keycode::A => j_x -= 30.0,
                Keycode::Right | Keycode::D => j_x += 30.0,
                Keycode::Up | Keycode::W => j_y -= 30.0,
                Keycode::Down | Keycode::S => j_y += 30.0,
                Keycode::Space => { /* saltar */ }
                Keycode::R => { /* reset */ }
                Keycode::G => { /* toggle */ }
                _ => {}
            }
        }
        _ => {}
    }
}
```

### Por Qué Funciona

1. **Event loop directo** → Sin intermediarios
2. **`repeat: false`** → Cada pulsación es una acción (teclado virtual Android no envía `repeat: true`)
3. **`Keycode::Left | Keycode::A`** → Múltiples teclas para la misma acción
4. **Movimiento instantáneo** → No depende de velocidad × dt

---

## 📋 PATRÓN COMPLETO DE INPUT

### Teclas que Funcionan en Termux-X11

| Keycode | Tecla Física | Alternativa |
|---------|-------------|-------------|
| `Keycode::Left` | ← | `Keycode::A` |
| `Keycode::Right` | → | `Keycode::D` |
| `Keycode::Up` | ↑ | `Keycode::W` |
| `Keycode::Down` | ↓ | `Keycode::S` |
| `Keycode::Space` | Barra espaciadora | - |
| `Keycode::R` | R | - |
| `Keycode::G` | G | - |
| `Keycode::Escape` | ESC | - |

### Template para Futuros Demos

```rust
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// En el game loop:
for event in backend.event_pump.poll_iter() {
    match event {
        Event::Quit { .. } |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            running = false;
            break;
        }

        Event::KeyDown { keycode: Some(key), repeat: false, .. } => {
            match key {
                // Movimiento
                Keycode::Left | Keycode::A => jugador.x -= 30.0,
                Keycode::Right | Keycode::D => jugador.x += 30.0,
                Keycode::Up | Keycode::W => jugador.y -= 30.0,
                Keycode::Down | Keycode::S => jugador.y += 30.0,
                
                // Acción
                Keycode::Space => jugador.saltar(),
                
                // Utilidades
                Keycode::R => reset(),
                Keycode::P => pausa(),
                
                _ => {}
            }
        }
        _ => {}
    }
}
```

---

## 🏃 PATRÓN RIGID BODY

### Estructura

```rust
struct RigidBody {
    x: f32, y: f32,      // Posición
    vx: f32, vy: f32,    // Velocidad
    w: u32, h: u32,      // Tamaño
    en_suelo: bool,       // ¿Está en el suelo?
    // ... texturas, colores, etc.
}

impl RigidBody {
    fn aplicar_gravedad(&mut self, dt: f32, gravedad: f32, plataformas: &[Rect]) {
        self.vy += gravedad * dt;
        self.y += self.vy * dt;
        
        // Colisiones con plataformas
        for plat in plataformas {
            if self.rect().has_intersection(*plat) {
                if self.rect().bottom() as i32 <= plat.y + 10 && self.vy > 0.0 {
                    self.y = plat.y as f32 - self.h as f32;
                    self.vy = 0.0;
                    self.en_suelo = true;
                }
            }
        }
    }
}
```

### Colisión Jugador ↔ Rigid Body

```rust
// En el update de cada rigid body:
if self.rect().has_intersection(jugador_rect) {
    // Empujar según posición relativa
    if jugador_rect.y < self.rect().y as i32 {
        self.vy = -200.0;  // Empujar arriba
    }
    if jugador_rect.x < self.rect().x as i32 {
        self.vx -= 100.0;  // Empujar izquierda
    } else {
        self.vx += 100.0;  // Empujar derecha
    }
}
```

---

## 📝 PATRÓN TTF + SPRITES

### Carga de Fuente

```rust
// Sdl2Backend ya tiene SDL2_ttf linkado
for path in &["/system/fonts/DroidSans.ttf", "/usr/share/fonts/noto-sans/NotoSans-Regular.ttf"] {
    if std::path::Path::new(path).exists() {
        let _ = backend.load_font(path, 18);
        break;
    }
}
```

### Dibujar Texto

```rust
backend.draw_text("Título", 15, 15, 18, 255, 255, 255);
backend.draw_text(&format!("Info: {}", valor), 15, 45, 14, 0, 255, 0);
```

### Carga de Sprites PNG

```rust
use sdl2::surface::Surface;
use sdl2::image::LoadSurface;

let path = "sprites/tank.png";
let surface = Surface::from_file(path)?;
let texture = backend.canvas.texture_creator()
    .create_texture_from_surface(&surface)?;
let tex_static: sdl2::render::Texture<'static> = unsafe { 
    std::mem::transmute(texture) 
};
```

### Dibujar Sprite

```rust
if let Some(ref tex) = sprite.textura {
    backend.canvas.copy(tex, None, Rect::new(x, y, w, h));
}
```

---

## 🚀 CÓMO COMPILAR Y EJECUTAR

### Compilar

```bash
# Demo colisiones (básico)
cargo build -p rydit-rs --bin demo_colisiones --release

# Demo rigid body (completo)
cargo build -p rydit-rs --bin demo_rigidbody --release
```

### Ejecutar

```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

# Demo colisiones
DISPLAY=:0 ./target/release/demo_colisiones

# Demo rigid body
DISPLAY=:0 ./target/release/demo_rigidbody
```

---

## 📊 COMPARACIÓN DE DEMOS

| Característica | demo_colisiones | demo_rigidbody |
|---------------|-----------------|----------------|
| **Input** | ✅ ← → SPACE | ✅ ← → ↑ ↓ SPACE WASD |
| **Gravedad** | ✅ Jugador | ✅ Jugador + 4 rigid bodies |
| **Colisiones** | ✅ Jugador ↔ plataformas | ✅ Todo ↔ todo |
| **Sprites PNG** | ❌ | ✅ 4 sprites cargados |
| **Texto TTF** | ❌ (fallback rects) | ✅ Texto real con fuente |
| **Empuje** | ❌ | ✅ Jugador empuja rigid bodies |
| **Líneas** | ~180 | ~360 |
| **Uso** | Test input rápido | Base para primer juego |

---

## 🎮 PRIMER JUEGO - Plan

### Estructura Basada en Estos Demos

```rust
// 1. Input (de demo_colisiones.rs)
for event in backend.event_pump.poll_iter() {
    match event {
        Event::KeyDown { keycode: Some(key), repeat: false, .. } => {
            // ← → SPACE para mover/saltar
        }
    }
}

// 2. Físicas (de demo_rigidbody.rs)
jugador.aplicar_gravedad(dt, gravedad, plataformas);
enemigos.aplicar_gravedad(dt, gravedad, plataformas);

// 3. Colisiones
if jugador.rect().has_intersection(plataforma) {
    // Aterrizar
}
if jugador.rect().has_intersection(enemigo) {
    // Game over o daño
}

// 4. Render (de ambos demos)
backend.draw_text("Mi Juego", 15, 15, 18, 255, 255, 255);
// Dibujar sprites, plataformas, etc.
```

### Features del Primer Juego

- [ ] Input ← → SPACE (demo_colisiones)
- [ ] Gravedad + colisiones (demo_rigidbody)
- [ ] Sprites PNG cargados (demo_rigidbody)
- [ ] Texto TTF para UI (demo_rigidbody)
- [ ] Múltiples enemigos rigid body
- [ ] Puntuación en texto TTF
- [ ] Game over + restart

---

## ⚠️ ERRORES COMUNES Y SOLUCIONES

### 1. Input No Responde

**Problema**: `backend.is_key_pressed()` no funciona  
**Solución**: Usar `backend.event_pump.poll_iter()` directo

### 2. Linker Falla con SDL2_ttf

**Problema**: `undefined symbol: TTF_Init`  
**Solución**: Usar `Sdl2Backend` (ya tiene TTF linkado), NO `sdl2::ttf` directo

### 3. Sprites No Aparecen

**Problema**: `Surface::from_file()` falla  
**Solución**: Verificar ruta existe + SDL2_image init en backend

### 4. Texto TTF No Aparece

**Problema**: Fuente no encontrada  
**Solución**: Probar múltiples paths (`/system/fonts/DroidSans.ttf`, etc.)

---

<div align="center">

**🛡️ CLAVE INPUT SDL2 - RyDit v0.11.6**

*demo_colisiones.rs ✅ | demo_rigidbody.rs ✅*

**Patrón: `event_pump.poll_iter()` + `repeat: false`**

</div>
