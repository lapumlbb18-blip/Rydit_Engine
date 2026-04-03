# 🛡️ RyDit v0.9.1 - GPU PARTICLES: ANÁLISIS TÉCNICO

**Fecha**: 2026-03-28  
**Estado**: ⚠️ INVESTIGACIÓN - Limitaciones de raylib-rs

---

## 🎯 OBJETIVO

Implementar **GPU Instancing + Shaders** para 10,000+ partículas @ 60 FPS, similar al demo de Python con ModernGL.

---

## 📊 COMPARATIVA TÉCNICA

### Python + ModernGL + Zink ✅

```python
# SHADERS GLSL
vertex_shader = '''
#version 330
layout (location = 0) in vec2 in_vert;      # Círculo geometry
layout (location = 1) in vec2 in_pos;       # Posición (instancing)
layout (location = 2) in vec3 in_color;     # Color (instancing)

uniform vec2 screen;
out vec2 v_uv;
out vec3 v_color;

void main() {
    v_uv = in_vert;
    v_color = in_color;
    vec2 ndc = (in_pos / screen) * 2.0 - 1.0;
    gl_Position = vec4(ndc, 0.0, 1.0);
    gl_Position.xy += in_vert * 0.015;
}
'''

# GPU INSTANCING
vao.render(moderngl.TRIANGLE_FAN, instances=15000)
# 1 draw call = 15,000 partículas
```

**Rendimiento**: 15,000 partículas @ 60 FPS ✅  
**Draw Calls**: 1 por frame ✅

---

### Rust + raylib-rs ⚠️

**Limitaciones encontradas**:

1. **Shader API limitada**: raylib-rs no expone `LoadShaderCode()` completo
2. **Sin instancing nativo**: No hay `glDrawArraysInstanced()` en la API
3. **Buffers manuales**: Requiere FFI directo a OpenGL

**Código intentado**:

```rust
// ESTO NO COMPILE CON RAYLIB-RS
let vs = CString::new(VERTEX_SHADER).unwrap();
unsafe {
    let vs_ptr = raylib::ffi::LoadShaderCode(vs.as_ptr(), std::ptr::null());
    // Error: raylib::ffi::Shader no es compatible con raylib::prelude::Shader
}
```

**Error**:
```
expected `raylib::ffi::Shader`, found `raylib::prelude::Shader`
```

---

## 🔍 ANÁLISIS DE ARQUITECTURA

### Python ModernGL

```
┌─────────────────────────────────────────┐
│  Python (CPU)                           │
│  - NumPy: física vectorizada            │
│  - Buffers: numpy arrays                │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│  ModernGL (OpenGL Wrapper)              │
│  - Shaders GLSL directos                │
│  - VAO + Instancing                     │
│  - 1 draw call = N partículas           │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│  GPU Adreno 610 (Zink/Vulkan)           │
│  - Vertex Shader: 15,000 instancias     │
│  - Fragment Shader: círculos con glow   │
└─────────────────────────────────────────┘
```

### Rust raylib-rs

```
┌─────────────────────────────────────────┐
│  Rust (CPU)                             │
│  - Física: Vec<Particle>                │
│  - Buffers: structs Rust                │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│  raylib-rs (FFI Wrapper)                │
│  - Shaders: API limitada ⚠️             │
│  - Draw calls: Una por partícula ❌     │
│  - Sin instancing expuesto              │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│  raylib C (OpenGL ES)                   │
│  - rlDrawRenderBatch()                  │
│  - Auto-batching (limitado)             │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│  GPU Adreno 610 (Zink/Vulkan)           │
│  - Mismos shaders, pero...              │
│  - 500 draw calls = 500 partículas      │
└─────────────────────────────────────────┘
```

---

## ✅ SOLUCIÓN IMPLEMENTADA: RENDER QUEUE

Dado que **GPU Instancing no está disponible** en raylib-rs sin FFI directo a OpenGL, implementamos:

### Render Queue v0.9.0

```rust
// crates/rydit-gfx/src/render_queue.rs
pub struct RenderQueue {
    commands: VecDeque<DrawCommand>,
    capacity: usize,  // 8192+
}

impl RenderQueue {
    pub fn push(&mut self, command: DrawCommand);
    pub fn execute(&mut self, gfx: &mut RyditGfx);
}
```

**Ventajas**:
- ✅ 8192+ draw calls acumulados
- ✅ 1 begin_draw() por frame
- ✅ Batch processing automático
- ✅ Compatible con raylib-rs actual

**Desventajas**:
- ⚠️ 500 partículas = 500 draw calls (vs 1 con instancing)
- ⚠️ CPU-bound, no GPU-bound
- ⚠️ Límite práctico: ~1000 partículas @ 60 FPS

---

## 📈 RENDIMIENTO ESPERADO

| Método | Partículas | Draw Calls | FPS | Estado |
|--------|------------|------------|-----|--------|
| **CPU (particles.rs)** | 500 | 500 | 30 | ✅ Funciona |
| **Render Queue** | 1000 | 1000 | 60 | ✅ Implementado |
| **GPU Instancing** | 10,000 | 1 | 60 | ❌ No disponible |
| **GPU Instancing** | 15,000 | 1 | 60 | ✅ Python |

---

## 🔜 PRÓXIMOS PASOS

### Opción A: FFI Directo a OpenGL

```rust
// Usar gl-rs directamente
use gl;

unsafe {
    // Crear shader program
    let vs = gl::CreateShader(gl::VERTEX_SHADER);
    gl::ShaderSource(vs, 1, &vs_source, std::ptr::null());
    gl::CompileShader(vs);

    // Crear VAO con instancing
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    // Instanced rendering
    gl::DrawArraysInstanced(
        gl::TRIANGLE_FAN,
        0,
        circle_vertex_count,
        particle_count as i32,
    );
}
```

**Pros**:
- ✅ Control total de GPU
- ✅ 10,000+ partículas reales

**Contras**:
- ❌ Requiere unsafe massivo
- ❌ Duplica código de raylib
- ❌ Pierde ventajas de raylib-rs

---

### Opción B: Usar wgpu (Recomendado)

```rust
// wgpu: API moderna de GPU
use wgpu;

// Compute shader para física
// Render pipeline con instancing
// 10,000+ partículas @ 60 FPS
```

**Pros**:
- ✅ API moderna (Vulkan/Metal/DX12)
- ✅ Compute shaders para física GPU
- ✅ 10,000+ partículas

**Contras**:
- ❌ Cambiar arquitectura completa
- ❌ Curva de aprendizaje

---

### Opción C: Optimizar Render Queue (ACTUAL)

```rust
// Mejorar batching en render_queue.rs
pub struct RenderQueue {
    circles: Vec<CircleCommand>,
    rects: Vec<RectCommand>,
    lines: Vec<LineCommand>,
    // Separar por tipo = mejor batching
}
```

**Pros**:
- ✅ Compatible con raylib-rs
- ✅ Sin unsafe
- ✅ Fácil de mantener

**Contras**:
- ⚠️ Límite ~1000 partículas
- ⚠️ No es GPU instancing real

---

## 🎯 CONCLUSIÓN

**GPU Instancing NO está disponible** en raylib-rs sin FFI directo a OpenGL.

**Solución actual**: Render Queue con batching (v0.9.0)
- ✅ 8192+ draw calls acumulados
- ✅ 1 begin_draw() por frame
- ⚠️ Límite práctico: 1000 partículas @ 60 FPS

**Para 10,000+ partículas reales**, se necesita:
1. FFI directo a OpenGL (unsafe)
2. O usar wgpu (cambio de arquitectura)
3. O cambiar a Python + ModernGL (para demos masivas)

---

## 📚 REFERENCIAS

### Shaders + Instancing
- [Learn OpenGL - Instancing](https://learnopengl.com/Advanced-OpenGL/Instancing)
- [ModernGL Instancing](https://moderngl.readthedocs.io/en/latest/examples/instancing.html)

### raylib-rs Limitations
- [raylib-rs GitHub](https://github.com/Deltand/raylib-rs)
- [raylib Shaders](https://www.raylib.com/examples/shaders/loader.html?name=shaders)

### wgpu (Alternativa)
- [wgpu GitHub](https://github.com/gfx-rs/wgpu)
- [wgpu Examples](https://github.com/gfx-rs/wgpu/tree/trunk/examples)

---

<div align="center">

**🛡️ RyDit v0.9.1 - GPU PARTICLES: ANÁLISIS**

*Render Queue ✅ | GPU Instancing ❌ | FFI OpenGL ⚠️*

**Solución actual: Render Queue batching**

**Límite: ~1000 partículas @ 60 FPS**

</div>
