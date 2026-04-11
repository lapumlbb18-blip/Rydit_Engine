# 🔄 INFORME: Doble Backend — SDL2 + Raylib para Ry-Dit

**Fecha**: 2026-04-11
**Tipo**: Evaluación de arquitectura
**Estado**: Propuesta exploratoria

---

## 💡 Idea Central

Crear un **doble sistema de backend** para Ry-Dit donde el motor pueda funcionar con:

1. **SDL2** → Backend actual (ventanas, input, dibujo 2D, audio)
2. **Raylib** → Backend alternativo (ventanas, input, dibujo 2D, dibujo 3D, audio)

Ambos controlados desde **Rust puro**, sin dependencia exclusiva de ninguno.

---

## 🔍 Cómo funciona cada backend internamente

### SDL2 (actual)

```
SDL2 Architecture:
├── SDL2 Video    → Ventanas, contextos OpenGL/EGL
├── SDL2 Events   → Input (teclado, mouse, touch, gamepad)
├── SDL2 Renderer → Dibujo 2D (rects, líneas, texturas)
├── SDL2_ttf      → Texto (Freetype, soporte UTF-8 parcial)
├── SDL2_image    → Carga de imágenes (PNG, JPG)
└── SDL2_mixer    → Audio (WAV, OGG, MP3)

Dependencias externas: Freetype, libpng, libjpeg, libogg, libmp3lame
```

### Raylib (propuesto)

```
Raylib Architecture:
├── GLFW (embebido) → Ventanas + Input (keyboard, mouse, gamepad)
├── rlgl            → Capa de abstracción OpenGL (dibujo 2D y 3D)
├── stb_truetype    → Texto TrueType (integrado, sin dependencias)
├── stb_image       → Carga de imágenes (integrado)
└── miniaudio       → Audio (integrado, sin dependencias externas)

Dependencias externas: NINGUNA (todo está embebido)
```

### Comparación directa

| Aspecto | SDL2 | Raylib |
|---------|------|--------|
| **Ventanas** | SDL_video | GLFW (embebido) |
| **Input** | SDL_events | GLFW (embebido) |
| **Dibujo 2D** | SDL_Renderer | rlgl → OpenGL |
| **Dibujo 3D** | OpenGL directo | rlgl → OpenGL (ya integrado) |
| **Texto** | SDL2_ttf (Freetype) | stb_truetype (integrado) |
| **Audio** | SDL2_mixer | miniaudio (integrado) |
| **Imágenes** | SDL2_image (libpng, libjpg) | stb_image (integrado) |
| **Dependencias** | 6+ bibliotecas externas | 0 (todo self-contained) |
| **Peso binario** | ~500KB-1MB | ~2MB (incluye todo) |
| **Texto visual** | ⚠️ Variable (depende de Freetype) | ✅ Excelente (stb_truetype optimizado) |
| **Android** | ✅ Funciona con Termux | ✅ Soporte nativo oficial |
| **iOS** | ✅ Soporte oficial | ✅ Soporte oficial |

---

## 🏗️ Arquitectura Propuesta

### Opción A: Feature Toggle (recomendada)

```
Cargo.toml de ry-gfx:

[features]
default = ["sdl2"]
sdl2 = ["dep:sdl2"]
raylib = ["dep:raylib"]

Uso:
  cargo build --features sdl2     → Backend SDL2 (actual)
  cargo build --features raylib   → Backend Raylib (nuevo)
```

### Opción B: Runtime Selection

```rust
// El usuario elige en runtime
let renderer: Box<dyn Renderer> = if use_raylib {
    Box::new(RaylibRenderer::new())
} else {
    Box::new(Sdl2Renderer::new())
};
```

### Opción C: Híbrida (lo mejor de ambos)

```
ry-gfx:
├── Input → SDL2 o Raylib (feature toggle)
├── 2D → SDL2 o Raylib (feature toggle)
├── 3D → Siempre Raylib (ry3d-gfx)
└── Audio → SDL2_mixer o miniaudio (feature toggle)
```

---

## 📐 Diseño del Trait Renderer

```rust
// ry-core/src/renderer.rs
pub trait Renderer {
    // Primitivas 2D
    fn clear(&mut self, color: ColorRydit);
    fn draw_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: ColorRydit);
    fn draw_circle(&mut self, cx: i32, cy: i32, r: i32, color: ColorRydit);
    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: ColorRydit);
    fn draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: ColorRydit);

    // Texto
    fn draw_text(&mut self, text: &str, x: i32, y: i32, size: i32, color: ColorRydit);
    fn load_font(&mut self, path: &str, size: i32) -> Result<(), String>;

    // Texturas
    fn draw_texture(&mut self, tex: &Texture, x: i32, y: i32, tint: ColorRydit);

    // Ventana
    fn should_close(&self) -> bool;
    fn swap_buffers(&mut self);
}

// Implementación SDL2 (actual ry-gfx)
impl Renderer for Sdl2Backend { ... }

// Implementación Raylib (nuevo)
impl Renderer for RaylibBackend {
    fn clear(&mut self, color: ColorRydit) {
        unsafe { raylib::ffi::ClearBackground(color.to_ffi()) }
    }
    fn draw_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: ColorRydit) {
        unsafe { raylib::ffi::DrawRectangle(x, y, w as i32, h as i32, color.to_ffi()) }
    }
    fn draw_circle(&mut self, cx: i32, cy: i32, r: i32, color: ColorRydit) {
        unsafe { raylib::ffi::DrawCircle(cx, cy, r, color.to_ffi()) }
    }
    fn draw_text(&mut self, text: &str, x: i32, y: i32, size: i32, color: ColorRydit) {
        // stb_truetype integrado, texto se ve excelente
        use std::ffi::CString;
        let c = CString::new(text).unwrap();
        unsafe { raylib::ffi::DrawText(c.as_ptr(), x, y, size, color.to_ffi()) }
    }
    // ... etc
}
```

---

## 🎮 Ventajas del Doble Backend

### ¿Por qué SDL2 se queda corto?

| Problema SDL2 | Solución Raylib |
|---------------|----------------|
| SDL2_ttf necesita Freetype externo | stb_truetype integrado, sin deps |
| Texto UTF-8 requiere TTF_RenderUTF8_Blended | DrawText nativo con UTF-8 |
| Texto 3D no soportado en FFI | DrawText3D disponible (aunque no en FFI Rust aún) |
| Audio necesita SDL2_mixer | miniaudio integrado |
| Imágenes necesitan SDL2_image + libpng | stb_image integrado |
| 3D requiere OpenGL directo | rlgl ya abstracto |
| Input events puede perder eventos | GLFW polling más estable |

### ¿Por qué Raylib solo NO basta?

| Problema Raylib | Ventaja SDL2 |
|----------------|-------------|
| No tiene eventos push (solo polling) | SDL2 tiene event queue con push |
| Touch support limitado | SDL2 touch más completo |
| Menor control fino de renderer | SDL2_Renderer más configurable |
| Binario más grande (~2MB extra) | SDL2 más ligero |
| No tiene mixer de buses | SDL2_mixer con canales |

---

## ⚡ Implementación Paso a Paso

### Fase 1: Prototipo 2D con Raylib (2-4h)
- [ ] Crear `ry-gfx/src/backend_raylib.rs`
- [ ] Implementar: `draw_rect`, `draw_circle`, `draw_line`, `draw_text`
- [ ] Crear `demo_raylib_2d.rs`
- [ ] Comparar visualmente texto SDL2 vs texto Raylib

### Fase 2: Input Raylib (4-6h)
- [ ] `IsKeyDown`, `GetMousePosition`, `GetMouseButton`
- [ ] Soporte touch básico
- [ ] Comparar latencia input SDL2 vs Raylib

### Fase 3: Feature Toggle (6-8h)
- [ ] `Cargo.toml` con features `sdl2` / `raylib`
- [ ] Trait `Renderer` genérico en `ry-core`
- [ ] Conditional compilation en `ry-gfx`
- [ ] Demo que compile con ambos backends

### Fase 4: Audio (4-6h)
- [ ] Integrar miniaudio de raylib como alternativa
- [ ] Comparar con SDL2_mixer

### Fase 5: Unificación (8-12h)
- [ ] ry-gfx unificado con feature toggle
- [ ] ry3d-gfx funciona con ambos
- [ ] Todos los demos compilables con ambos backends

---

## 🧪 Prueba de Concepto: Texto Raylib vs SDL2

```rust
// Con SDL2 (actual):
// TTF_RenderUTF8_Blended → Surface → Texture → Canvas
// Problema: necesita Freetype, UTF-8 fix manual, rendimiento variable

// Con Raylib (propuesto):
// DrawText(text, x, y, size, color) → rlgl → OpenGL
// Ventaja: stb_truetype integrado, UTF-8 nativo, se ve limpio
```

**Diferencia visual esperada:**
- SDL2_ttf: Bueno pero depende de la fuente cargada
- Raylib stb_truetype: Consistente, limpio, optimizado para games

---

## 🔮 Compatibilidad con Plataformas

| Plataforma | SDL2 | Raylib | Doble Backend |
|-----------|------|--------|--------------|
| Android (Termux) | ✅ Funciona | ✅ Nativo | ✅ Ambos |
| Linux desktop | ✅ Nativo | ✅ Nativo | ✅ Ambos |
| Windows | ✅ Nativo | ✅ Nativo | ✅ Ambos |
| macOS | ✅ Nativo | ✅ Nativo | ✅ Ambos |
| iOS | ⚠️ Requiere SDL2 compilado | ✅ Nativo | ⚠️ Solo Raylib |
| Web (WASM) | ⚠️ Emscripten | ✅ Nativo | ⚠️ Solo Raylib |
| Raspberry Pi | ✅ Nativo | ✅ Nativo | ✅ Ambos |

---

## ⚠️ Riesgos y Mitigación

| Riesgo | Impacto | Mitigación |
|--------|---------|------------|
| Mantener 2 backends | Medio | Tests automáticos + CI en ambos |
| Duplicación de código | Bajo | Trait genérico, impls separados |
| Confusión de usuarios | Bajo | Documentación clara, default = SDL2 |
| Conflictos de dependencias | Bajo | Features mutuellement exclusivas |
| Binarios más grandes | Bajo | Feature toggle = solo uno a la vez |

---

## 📊 Conclusión

### ¿Es posible?
**SÍ, totalmente.** Rust soporta features toggle nativamente y ambos backends pueden coexistir sin conflictos.

### ¿Vale la pena?
**SÍ, por estas razones:**

1. **Textos más bonitos** → stb_truetype integrado, sin dependencias externas
2. **3D unificado** → Mismo backend para 2D y 3D (raylib)
3. **Menos dependencias** → Raylib no necesita Freetype, libpng, SDL2_mixer, etc.
4. **Independencia real** → Si SDL2 falla, Raylib funciona y viceversa
5. **Android nativo** → Raylib tiene mejor soporte Android oficial
6. **Binario self-contained** → Con Raylib, un solo crate, cero deps externas

### ¿Cuánto trabajo sería?
- **Prototipo**: 2-4h (solo 2D básico con raylib)
- **Feature toggle**: 6-8h (Cargo.toml + trait genérico)
- **Completo**: 24-36h (input, audio, texturas, todos los demos)

### Recomendación
**Empezar con Fase 1 (prototipo 2D)** para validar que:
1. El texto se ve mejor que con SDL2_ttf
2. El input responde bien en Termux
3. La arquitectura es viable

Si funciona → continuar con las fases siguientes.
Si no → se pierde poco tiempo y se mantiene SDL2 como único backend.

---

<div align="center">

**🔄 Doble Backend SDL2 + Raylib — Evaluación**

*Posible: ✅ | Recomendado: ✅ | Esfuerzo prototipo: 2-4h*

*Rust no depende de ninguno — puede elegir el mejor para cada situación*

</div>
