# 🛡️ RyDit v0.11.6 - Guía de Usuario

**Fecha**: 2026-04-03
**Versión**: v0.11.6 - Input SDL2 + TTF + Sprites + Rigid Body + Audio
**Estado**: ✅ Stack completo funcional: Input + TTF + Sprites + Físicas + Audio + Videos

---

## 🆕 ¿Qué hay de nuevo en v0.11.6?

La versión v0.11.6 es la primera versión completamente funcional con el stack gráfico SDL2:

| Feature | Descripción | Estado |
|---------|-------------|--------|
| **Input SDL2** | Teclado funcional con patrón `repeat: false` (cada pulsación = acción individual). ← → ↑ ↓ WASD SPACE | ✅ 10/10 |
| **Texto TTF** | Texto real con fuente del sistema (`DroidSans.ttf`), sin parpadeo, texturas cacheadas cada 30 frames | ✅ 8/10 |
| **Sprites PNG** | 4 sprites cargados con SDL2_image: tank, helicopter, crate, platform | ✅ 8/10 |
| **Rigid Body** | 4 cuerpos con gravedad independiente + colisiones AABB + empuje del jugador | ✅ 9/10 |
| **Audio SDL2** | Sonidos generados dinámicamente (salto 600Hz, colisión 300Hz) con SDL2_mixer | ✅ 7/10 |
| **Videos Demo** | 3 videos MP4 embebidos mostrando todas las features | ✅ |

**Puntaje global del stack**: 5.1/10 → Potencial: 9.2/10 (con audio.rs migrado + rydit-anim maduro)

### Comparativa con otros motores

| Característica | PICO-8 | Defold | Godot | **RyDit v0.11.6** |
|---------------|--------|--------|-------|-------------------|
| **Binario** | ~5 MB | ~15 MB | ~50 MB | **~550 KB** ✅ |
| **RAM mínimo** | — | ~100 MB | ~200 MB | **~45 MB** ✅ |
| **Sprites** | 128 | Ilimitados | Ilimitados | **Ilimitados (PNG)** ✅ |
| **Sonido** | 4 canales | Ilimitado | Ilimitado | **SDL2_mixer ilimitado** ✅ |
| **Android nativo** | ❌ | ✅ Export | ❌ | ✅ **Nativo Termux** |
| **Lenguaje** | Lua | Lua | GDScript | **RyDit (español)** ✅ |
| **Construido en móvil** | ❌ | ❌ | ❌ | ✅ **Redmi Note 8** |

---

## 🎮 Controles

Todos los demos usan el mismo patrón de input. Las teclas funcionan en el teclado virtual de Android (Termux-X11) y en teclado físico.

| Tecla | Acción |
|-------|--------|
| **← / A** | Mover izquierda |
| **→ / D** | Mover derecha |
| **↑ / W** | Mover arriba / Saltar |
| **↓ / S** | Mover abajo |
| **SPACE** | Saltar (en platformer) |
| **R** | Reset (reiniciar posiciones) |
| **G** | Toggle (activar/desactivar feature) |
| **ESC** | Salir del demo |

> **Nota importante**: El teclado virtual de Android solo envía pulsaciones individuales (`repeat: false`). Cada toque = una acción. Mantener presionado NO produce movimiento continuo.

### Por que funciona este patrón

1. **Event loop directo** → Sin intermediarios (`backend.is_key_pressed()` NO funciona)
2. **`repeat: false`** → Cada pulsación es una acción (teclado virtual Android NO envía `repeat: true`)
3. **`Keycode::Left | Keycode::A`** → Múltiples teclas para la misma acción
4. **Movimiento instantáneo** → No depende de velocidad × dt

### Template para crear tus propios demos

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

## 📦 Demos disponibles

### 1. demo_colisiones — Input básico + colisiones

**Qué verás**: Un cuadro rojo que controlas sobre 6 plataformas grises. Gravedad + salto.

| Elemento | Color | Función |
|----------|-------|---------|
| Jugador | 🔴 Rojo | Controlable con ← → SPACE |
| Plataformas | ⬜ Gris | Colisiones AABB |
| Fondo | ⬛ Negro | Sin decoración |

**Cómo ejecutar**:
```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink

cargo build -p rydit-rs --bin demo_colisiones --release
DISPLAY=:0 ./target/release/demo_colisiones
```

**Qué probar**:
- Toca ← → para mover el cuadro
- Toca SPACE para saltar
- Observa cómo el cuadro cae por gravedad y aterriza en las plataformas

---

### 2. demo_rigidbody — Demo completo (⭐ recomendado)

**Qué verás**: Un cuadro rojo (jugador) + 4 sprites con físicas independientes + texto TTF en tiempo real mostrando FPS, posiciones y estado.

| Elemento | Color/Sprite | Función |
|----------|-------------|---------|
| Jugador | 🔴 Rojo | Controlable con ← → ↑ ↓ WASD SPACE |
| Tank | 🟢 Verde | Sprite PNG con gravedad |
| Helicopter | 🔵 Cyan | Sprite PNG con gravedad |
| Crate | 🟤 Marrón | Sprite PNG con gravedad |
| Platform | ⚫ Gris | Sprite PNG con gravedad |
| Plataformas | ⬜ Gris | 7 plataformas estáticas |
| Texto TTF | ⬜ Blanco | Info en tiempo real (FPS, posiciones) |

**Cómo ejecutar**:
```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink

cargo build -p rydit-rs --bin demo_rigidbody --release
DISPLAY=:0 ./target/release/demo_rigidbody
```

**Qué probar**:
- Mueve el jugador con ← → ↑ ↓
- Salta con SPACE
- Empuja los sprites: el jugador transfiere velocidad a los rigid bodies
- Observa cómo cada sprite cae, rebota y aterriza independientemente
- Lee el texto TTF en la esquina superior con info en tiempo real

---

### 3. test_audio_minimal — Sonidos SDL2_mixer

**Qué verás**: Una pantalla que genera archivos WAV con tonos puros y los reproduce.

| Sonido | Frecuencia | Cuándo suena |
|--------|-----------|--------------|
| Salto | 600 Hz | Al presionar SPACE |
| Colisión | 300 Hz | Al chocar con plataforma |

**Cómo ejecutar**:
```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink

cargo build -p rydit-rs --bin test_audio_minimal --release
DISPLAY=:0 ./target/release/test_audio_minimal
```

**Qué probar**:
- Presiona SPACE para escuchar el tono de salto (600Hz)
- Observa la pantalla que confirma la reproducción del sonido

---

### 4. demo_50k_particulas — Sistema de partículas masivo

**Qué verás**: Miles de partículas animadas con diferentes efectos (fuego, humo, explosión, lluvia, chispas).

| Tecla | Efecto |
|-------|--------|
| **F** | Fuego |
| **H** | Humo |
| **E** | Explosión |
| **S** | Chispas |
| **L** | Lluvia |

**Cómo ejecutar**:
```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink

cargo build -p rydit-rs --bin demo_50k_particulas --release
DISPLAY=:0 ./target/release/demo_50k_particulas
```

**Qué probar**:
- Presiona F para fuego, H para humo, E para explosión
- Observa el rendimiento con miles de partículas simultáneas

---

### 5. demo_sprites_v2 — Verificación de sprites PNG

**Qué verás**: 4 sprites verificados con indicador visual de existencia + animación senoidal + input completo con 69 teclas mapeadas.

| Elemento | Color/Sprite | Función |
|----------|-------------|---------|
| Tank | 🟢 Verde | Sprite verificado (16x16) |
| Helicopter | 🔵 Cyan | Sprite verificado (16x16) |
| Crate | 🟤 Marrón | Sprite verificado (8x8) |
| Platform | ⚫ Gris | Sprite verificado (16x16) |
| Indicador | ⬜ Barra blanca | Archivo PNG existe |

**Cómo ejecutar**:
```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink

cargo build -p rydit-rs --bin demo_sprites_v2 --release
DISPLAY=:0 ./target/release/demo_sprites_v2
```

**Controles**:
| Tecla | Acción |
|-------|--------|
| **← → ↑ ↓** | Mover sprite seleccionado |
| **1-4** | Seleccionar sprite |
| **A** | Toggle animación |
| **R** | Reset posiciones |
| **ESC** | Salir |

**Qué probar**:
- Presiona 1-4 para seleccionar cada sprite
- Mueve el sprite seleccionado con las flechas
- Observa la barra blanca que confirma que el archivo PNG existe
- Toggle animación A para ver movimiento senoidal

---

## 🖼️ Sprites disponibles

Todos los sprites están en `logo_icon_asst/sprites/`:

| # | Nombre | Archivo | Tamaño | Color representativo |
|---|--------|---------|--------|---------------------|
| 1 | **Tank** | `tank_16x16.png` | 16×16 | 🟢 Verde |
| 2 | **Helicopter** | `helicopter_16x16.png` | 16×16 | 🔵 Cyan |
| 3 | **Crate** | `crate_8x8.png` | 8×8 | 🟤 Marrón |
| 4 | **Platform** | `platform_16x16.png` | 16×16 | ⚫ Gris |
| 5 | **Cube** | `cube_8x8.png` | 8×8 | — |

Los sprites se cargan con `SDL2_image` y se dibujan como texturas SDL2. En `demo_rigidbody` los 4 primeros se ven como objetos con gravedad que rebotan y aterrizan.

---

## 📝 Texto TTF

El texto TTF se renderiza usando `FontFFI` (FFI interno de `rydit-gfx`), **no** usando `sdl2::ttf` directamente.

**Qué se ve en pantalla** (en `demo_rigidbody`):
```
═══════════════════════════════════════
  RyDit v0.11.6 - Demo Rigid Body
═══════════════════════════════════════
  FPS: 60
  Jugador: x=400 y=300
  Rigid Bodies: 4
  Plataformas: 7
  ← → ↑ ↓ WASD SPACE = Mover
  R = Reset | ESC = Salir
═══════════════════════════════════════
```

**Características del texto TTF**:
- ✅ Fuente real del sistema (`/system/fonts/DroidSans.ttf` en Android)
- ✅ Sin parpadeo (texturas cacheadas cada 30 frames)
- ✅ Múltiples tamaños (14px, 18px, 20px)
- ✅ Colores RGB (blanco, verde, amarillo)

**Cómo se carga** (patrón para tus propios demos):
```rust
let mut backend = Sdl2Backend::new("Mi Demo", 800, 600)?;

// Probar rutas de fuente
for path in &["/system/fonts/DroidSans.ttf", "/usr/share/fonts/noto-sans/NotoSans-Regular.ttf"] {
    if std::path::Path::new(path).exists() {
        let _ = backend.load_font(path, 18);
        break;
    }
}

// Dibujar texto
backend.draw_text("Título", 15, 15, 18, 255, 255, 255);
backend.draw_text(&format!("FPS: {}", fps), 15, 45, 14, 0, 255, 0);
```

---

## 🔊 Audio

El audio funciona con `SDL2_mixer` a través de `Sdl2Backend`. Los sonidos se generan dinámicamente como archivos WAV con tonos puros.

| Sonido | Frecuencia | Duración | Uso |
|--------|-----------|----------|-----|
| Salto | 600 Hz | 0.2s | Cuando el jugador salta |
| Colisión | 300 Hz | 0.15s | Cuando hay impacto con plataforma |

**Cómo se genera un sonido**:
```rust
// Generar WAV con tono puro
fn generar_wav(ruta: &str, frecuencia: f32, duracion: f32) {
    let sample_rate = 44100u32;
    let num_samples = (sample_rate as f32 * duracion) as usize;
    let mut datos = Vec::with_capacity(num_samples * 2);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let valor = (2.0 * std::f32::consts::PI * frecuencia * t).sin();
        let sample = (valor * 16000.0) as i16;
        datos.extend_from_slice(&sample.to_le_bytes());
    }

    // Escribir header WAV + datos...
}

// Reproducir con SDL2_mixer
let chunk = Mix_Chunk::from_file("sonido_salto.wav")?;
Mix_PlayChannel(-1, &chunk, 0)?;
```

---

## 💥 Colisiones

Las colisiones usan el algoritmo **AABB** (Axis-Aligned Bounding Box) con `Rect::has_intersection()`.

### Cómo funcionan

1. **Cada frame** se calcula la nueva posición del jugador/rigid body
2. **Se verifica** si el rect del objeto intersecta con alguna plataforma
3. **Si hay intersección** y el objeto cae (`vy > 0`):
   - Se reposiciona el objeto encima de la plataforma
   - Se setea `vy = 0` y `en_suelo = true`
4. **Jugador ↔ Rigid Body**: Si intersectan, se transfiere velocidad (empuje)

```rust
// Colisión con plataforma
for plat in plataformas {
    if self.rect().has_intersection(*plat) {
        if self.rect().bottom() as i32 <= plat.y + 10 && self.vy > 0.0 {
            self.y = plat.y as f32 - self.h as f32;
            self.vy = 0.0;
            self.en_suelo = true;
        }
    }
}

// Colisión jugador ↔ rigid body (empuje)
if self.rect().has_intersection(jugador_rect) {
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

### En demo_rigidbody

- **4 rigid bodies** con gravedad independiente
- **7 plataformas** estáticas
- **Jugador** empuja rigid bodies al colisionar
- **Cada rigid body** también colisiona con plataformas

### Patrón Rigid Body (para tus propios demos)

```rust
struct RigidBody {
    x: f32, y: f32,      // Posición
    vx: f32, vy: f32,    // Velocidad
    w: u32, h: u32,      // Tamaño
    en_suelo: bool,       // ¿Está en el suelo?
}

impl RigidBody {
    fn aplicar_gravedad(&mut self, dt: f32, gravedad: f32, plataformas: &[Rect]) {
        self.vy += gravedad * dt;
        self.y += self.vy * dt;

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

// Colisión jugador ↔ rigid body (empuje)
if self.rect().has_intersection(jugador_rect) {
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

## 📋 Requisitos

### Software necesario

| Dependencia | Versión | Cómo verificar |
|-------------|---------|----------------|
| **SDL2** | 0.37 | `pkg list-installed \| grep sdl2` |
| **SDL2_image** | — | `ls /data/data/com.termux/files/usr/lib/libSDL2_image.so` |
| **SDL2_ttf** | — | `ls /data/data/com.termux/files/usr/lib/libSDL2_ttf.so` |
| **SDL2_mixer** | — | `ls /data/data/com.termux/files/usr/lib/libSDL2_mixer.so` |
| **Rust** | 1.70+ | `rustc --version` |
| **Termux-X11** | — | Ejecutar `xinit` |

### Instalar dependencias

```bash
pkg install sdl2 sdl2_image sdl2_ttf sdl2_mixer
```

### Variables de entorno

```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1
```

> **Importante**: Termux-X11 debe estar corriendo antes de ejecutar cualquier demo. En una sesión aparte ejecuta `xinit`.

---

## 🎬 Videos Demo

Los 3 videos están en `screenshots/` y son reproducibles directamente en GitHub:

### Video 1: Rigid Body + TTF + Sprites
- **Archivo**: `screenshots/Rydit_demo1.mp4`
- **Qué muestra**: Input SDL2 + texto TTF + 4 sprites PNG + gravedad + colisiones + audio
- **Duración**: ~30 segundos

### Video 2: Test Audio SDL2_mixer
- **Archivo**: `screenshots/test_demo.mp4`
- **Qué muestra**: Generación de tonos WAV + reproducción con SDL2_mixer
- **Duración**: ~15 segundos

### Video 3: Sistema de Partículas
- **Archivo**: `screenshots/particulas.mp4`
- **Qué muestra**: 5 efectos (fuego, humo, explosión, lluvia, chispas) a 60 FPS
- **Duración**: ~30 segundos

---

## 🚀 Inicio rápido

```bash
# 1. Iniciar Termux-X11 (en otra sesión)
xinit

# 2. Configurar entorno
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink

# 3. Compilar y ejecutar el demo recomendado
cargo build -p rydit-rs --bin demo_rigidbody --release
DISPLAY=:0 ./target/release/demo_rigidbody
```

**Controles en pantalla**:
- ← → para mover el jugador
- ↑ o SPACE para saltar
- R para reset
- ESC para salir

---

## ⚠️ Solución de problemas

| Problema | Causa | Solución |
|----------|-------|----------|
| "no SDL2 video device" | Termux-X11 no iniciado | Ejecutar `xinit` primero |
| "failed to create window" | DISPLAY no configurado | `export DISPLAY=:0` |
| "undefined symbol: TTF_Init" | Usando `sdl2::ttf` directo | Usar `backend.draw_text()` |
| Input no responde | Usando `repeat: true` o wrapper | Usar SOLO `repeat: false` + `event_pump.poll_iter()` directo |
| Sprites no aparecen | Ruta incorrecta | Verificar archivo existe en `logo_icon_asst/sprites/` |
| Audio no suena | SDL2_mixer no linkado | Usar `Sdl2Backend` que ya tiene mixer init |
| Texturas no se guardan | Lifetime de TextureCreator | Usar verificación de archivos + fallback rects |
| Parpadeo en texto | Texturas no cacheadas | Cacheadas cada 30 frames (ya implementado) |
| Ventana se cierra al instante | Zink no configurado | `export MESA_LOADER_DRIVER_OVERRIDE=zink` |
| Linker falla con SDL2 | SDL2 no instalado | `pkg install sdl2 sdl2_image sdl2_ttf sdl2_mixer` |

### Errores comunes de código

**1. Input no responde**
```rust
// ❌ NO USAR wrapper para input
if backend.is_key_pressed("arrow_left") { ... }

// ✅ USAR event loop directo
for event in backend.event_pump.poll_iter() {
    match event {
        Event::KeyDown { keycode: Some(key), repeat: false, .. } => {
            match key {
                Keycode::Left | Keycode::A => jugador.x -= 30.0,
                // ...
            }
        }
        _ => {}
    }
}
```

**2. Linker falla con SDL2_ttf**
```rust
// ❌ NO usar sdl2::ttf directo
use sdl2::ttf::Font; // undefined symbol: TTF_Init

// ✅ Usar Sdl2Backend (ya tiene TTF linkado)
backend.load_font("/system/fonts/DroidSans.ttf", 18);
backend.draw_text("Hola", 10, 10, 18, 255, 255, 255);
```

**3. Sprites no aparecen**
```rust
// ❌ NO asumir que la ruta es correcta
let surface = Surface::from_file("tank.png")?; // Falla si no existe

// ✅ Verificar antes de cargar
let path = "logo_icon_asst/sprites/tank_16x16.png";
if std::path::Path::new(path).exists() {
    let surface = Surface::from_file(path)?;
    // cargar textura...
} else {
    // Fallback: dibujar rectángulo de color
    backend.draw_rect(x, y, w, h, 0, 255, 0);
}
```

---

## 🎯 Próximo juego - Plan basado en demos

Usando lo que ya funciona en v0.11.6, puedes construir un primer juego con:

| Feature | Demo fuente | Código listo |
|---------|-------------|-------------|
| Input ← → SPACE | `demo_colisiones` | ✅ Copiar |
| Gravedad + colisiones | `demo_rigidbody` | ✅ Copiar |
| Sprites PNG | `demo_rigidbody` | ✅ Copiar |
| Texto TTF para UI | `demo_rigidbody` | ✅ Copiar |
| Audio (salto/colisión) | `test_audio_minimal` | ✅ Copiar |
| Múltiples enemigos | `demo_rigidbody` | ✅ Copiar |
| Game over + restart | Snake existente | ✅ Adaptar |

---

<div align="center">

**🛡️ RyDit v0.11.6 - Guía de Usuario**

*Input SDL2 ✅ 10/10 | TTF ✅ 8/10 | Sprites PNG ✅ 8/10 | Rigid Body ✅ 9/10 | Audio ✅ 7/10*

**Puntaje global: 5.1/10 → Potencial: 9.2/10**

**Próximo: v0.11.7 — Audio.rs migrado a SDL2_mixer + Demo .rydit con audio**

*"Construido sin prisa, madurado con paciencia"*

</div>
