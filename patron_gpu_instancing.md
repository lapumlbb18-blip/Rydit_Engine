# Patron GPU Instancing — RyDit v0.15.0

## Pipeline Funcional

```
SDL2 + OpenGL Core 3.3 (sin Canvas, sin raylib)
  └─ window.gl_create_context()
  └─ gl::load_with(video.gl_get_proc_address)
  └─ GPUInstancer → shaders custom → glDrawArraysInstanced
```

## Clave: NO mezclar con Canvas ni raylib

| Backend | ¿Sirve para GPU Instancing? | Razón |
|---------|----------------------------|-------|
| **SDL2 OpenGL directo** | ✅ SÍ | Contexto OpenGL vivo, `gl::load_with` funciona |
| `Sdl2Backend` (ry-gfx) | ❌ NO | `window.into_canvas()` destruye el contexto GL |
| `RyditGfx` (raylib) | ❌ NO | Contexto GLFW/GLAD diferente al `gl` crate |

## Setup mínimo funcional

```rust
use ry_gfx::gpu_instancing::{GPUInstancer, ParticleData};
use sdl2::video::GLProfile;

// 1. SDL2 OpenGL (NO Canvas)
let sdl = sdl2::init().unwrap();
let video = sdl.video().unwrap();
let ga = video.gl_attr();
ga.set_context_profile(GLProfile::Core);
ga.set_context_version(3, 3);

let win = video.window("GPU Instancing", 1280, 720)
    .position_centered().opengl().build().unwrap();
let ctx = win.gl_create_context().unwrap();
win.gl_make_current(&ctx).unwrap();
gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

// 2. GPUInstancer
let mut inst = GPUInstancer::new();
inst.set_resolution(1280.0, 720.0);
inst.load_shaders("/tmp/vs.glsl", "/tmp/fs.glsl").unwrap();

// 3. Partículas
let mut particles = Vec::new();
for i in 0..50000 {
    let sx = ((i * 137) % 1280) as f32;
    let sy = ((i * 251) % 720) as f32;
    particles.push(ParticleData::new(sx, sy, 5.0, 1.0, 1.0, 1.0, 1.0));
}
inst.set_particles(&particles);

// 4. Render loop
unsafe {
    gl::ClearColor(0.05, 0.05, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
    gl::Disable(gl::DEPTH_TEST);
    gl::Disable(gl::SCISSOR_TEST);
    gl::Viewport(0, 0, 1280, 720);
    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
}
inst.draw();
win.gl_swap_window();
```

## Shaders

### vertex.glsl
```glsl
#version 330 core
layout(location = 0) in vec2 aPosition;  // Quad local (-0.5 a 0.5)
layout(location = 1) in vec2 aOffset;    // Pos partícula (píxeles)
layout(location = 2) in float aSize;
layout(location = 3) in vec4 aColor;

out vec4 vColor;
out vec2 vLocalPos;
uniform vec2 uResolution;

void main() {
    vec2 screenPos = aPosition * aSize + aOffset;
    gl_Position = vec4(
        (screenPos.x / uResolution.x) * 2.0 - 1.0,
       -((screenPos.y / uResolution.y) * 2.0 - 1.0),
        0.0, 1.0);
    vColor = aColor;
    vLocalPos = aPosition;
}
```

### fragment.glsl (quad sólido, estilo fill_rect)
```glsl
#version 330 core
in vec4 vColor;
in vec2 vLocalPos;
out vec4 fragColor;
void main() {
    fragColor = vColor;
}
```

## Bugs fixeados v0.15.0

| # | Bug | Fix |
|---|-----|-----|
| 1 | `instance_vbo` no bindeado | `glBindBuffer(instance_vbo)` antes de atributos |
| 2 | Stride location 0 = 8 (mal) | Stride = 16 (4 floats × 4 bytes) |
| 3 | QUADS en Core Profile | 6 vértices (2 triángulos) + `gl::TRIANGLES` |
| 4 | Fragment shader: `length(vLocalPos) > 1.0` nunca descarta | Simplificado a `fragColor = vColor` (quad sólido) |
| 5 | `uResolution` no seteado | `inst.set_resolution()` + shader uniform |
| 6 | `glViewport` no configurado | `gl::Viewport(0, 0, 1280, 720)` cada frame |
| 7 | `glScissorTest` cortando bordes | `gl::Disable(gl::SCISSOR_TEST)` |
| 8 | Shaders desde path relativo | `include_str!()` → escribir a `/usr/tmp/` |

## Patrón de partículas "estrellas" (como demo_torreta menú)

```rust
// Distribución pseudo-aleatoria que NO colapsa en barras
for i in 0..n {
    let sx = ((i.wrapping_mul(137)) % 1280) as f32;
    let sy = ((i.wrapping_mul(251) + i / 10) % 720) as f32;
    // 4 tipos de color: blanco, azul, amarillo, morado
}
```

## Benchmarks Adreno 610 / Zink

| Partículas | FPS | Draw calls | Notas |
|-----------|-----|------------|-------|
| 50K | ~48 | 1 | Estrellas animadas |
| 100K | ~40 | 1 | Estrellas animadas |
| 150K | ~35 | 1 | Estrellas animadas |
| 250K | ~28 | 1 | Estrellas animadas |

## Demo binario

- **Crates/ry-rs/src/bin/demo_gpu_instancing.rs** — Demo completo funcional
- Compilar: `cargo build --bin demo_gpu_instancing --release`
- Lanzar: `DISPLAY=:0 MESA_LOADER_DRIVER_OVERRIDE=zink GALLIUM_DRIVER=zink ./target/release/demo_gpu_instancing`
- Controles: 1-5 = 50K-250K partículas, ESC = salir

---
*Última actualización: 2026-04-06 | v0.15.0*
