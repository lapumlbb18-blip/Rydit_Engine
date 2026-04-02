# 🛡️ RyDit - FSR 1.0 + RYDIT-STREAM (Documentación Pre-Existente)

**Fecha**: 2026-04-01  
**Versión**: v0.11.2 ✅ COMPLETADA  
**Documentación Original**: QWEN.md + docs/ANALISIS_estrategico_V0.11.1.md  
**Estado**: ✅ **Lista para implementar**

---

## 🎯 PRIORIDADES ORIGINALES (Pre-v0.11.2)

Según QWEN.md (antes del parser):

| # | Feature | Tiempo | Impacto | Valor |
|---|---------|--------|---------|-------|
| **1** | Parser Fuerte | 2-3 semanas | 🔴 ALTO | $200K |
| **2** | **FSR 1.0** | 1-2 semanas | 🔴 ALTO | $150K |
| **3** | RyBot UI Táctil | 2 semanas | 🔴 ALTO | $200K |
| **4** | **Rydi-Stream** | 3-4 semanas | 🔴 ALTO | $300K |

**Valor Total Potencial**: **$900K+**

---

## 🔮 FSR 1.0 - ARQUITECTURA ORIGINAL

### **Estructura Propuesta**

```
crates/rydit-gfx/
├── shaders/
│   ├── fsr_upscale.glsl    # FSR 1.0 shader
│   ├── fsr_easu.glsl       # Edge Adaptive Spatial Upsampling
│   └── fsrcas.glsl         # Robust Contrast Adaptive Sharpening
├── gpu_instancing.rs       # GPU instancing existente
└── fsr_upscale.rs          # 🆕 FSR integration (Rust)
```

---

### **Shader FSR - Estructura GLSL**

```glsl
// shaders/fsr_upscale.glsl
#version 330 core

// Uniforms
uniform sampler2D inputTexture;
uniform vec2 inputSize;      // 1280x720
uniform vec2 outputSize;     // 1920x1080
uniform float sharpness;     // 0.0 - 1.0

// Outputs
out vec4 fragColor;

// EASU - Edge Adaptive Spatial Upsampling
vec4 easu(vec2 uv) {
    // Implementación de EASU
    // Muestreo adaptativo basado en bordes
    // 1. Detectar bordes
    // 2. Interpolar direccionalmente
    // 3. Preservar bordes nítidos
}

// RCAS - Robust Contrast Adaptive Sharpening
vec4 rcas(vec4 color, vec2 uv) {
    // Implementación de RCAS
    // Sharpening adaptativo basado en contraste
    // 1. Calcular contraste local
    // 2. Aplicar sharpening proporcional
    // 3. Limitar overshoot
}

void main() {
    vec2 uv = gl_FragCoord.xy / outputSize;

    // EASU upscale
    vec4 upscaled = easu(uv);

    // RCAS sharpen
    fragColor = rcas(upscaled, uv);
}
```

---

### **Implementación Rust**

```rust
// crates/rydit-gfx/src/fsr_upscale.rs

pub struct FsrUpscaler {
    program: GLuint,
    vao: GLuint,
    vbo: GLuint,
    input_texture: GLint,
    input_size: GLint,
    output_size: GLint,
    sharpness: GLint,
}

impl FsrUpscaler {
    pub fn new() -> Self {
        // Compilar shaders
        // Crear VAO/VBO para fullscreen quad
        // Inicializar uniforms
    }

    pub fn render(&self, 
                  input_texture: GLuint,
                  input_size: (u32, u32),
                  output_size: (u32, u32)) {
        // Bind framebuffer
        // Usar programa FSR
        // Render fullscreen quad
        // Unbind
    }

    pub fn set_sharpness(&self, value: f32) {
        // Actualizar uniform sharpness
    }

    pub fn set_quality_mode(&self, mode: FsrQuality) {
        match mode {
            FsrQuality::Performance => { /* 0.5x */ }
            FsrQuality::Balanced => { /* 0.66x */ }
            FsrQuality::Quality => { /* 0.75x */ }
        }
    }
}

pub enum FsrQuality {
    Performance,  // 720p → 1080p (+50% FPS)
    Balanced,     // 900p → 1080p (+30% FPS)
    Quality,      // 1080p → 1440p (+20% FPS)
}
```

---

### **Modos de Calidad**

| Modo | Resolución Input | Resolución Output | FPS Gain | Calidad |
|------|------------------|-------------------|----------|---------|
| **Performance** | 1280x720 | 1920x1080 | +50% | ⭐⭐⭐⭐ |
| **Balanced** | 1280x900 | 1920x1080 | +30% | ⭐⭐⭐⭐⭐ |
| **Quality** | 1440x1080 | 1920x1080 | +20% | ⭐⭐⭐⭐⭐ |

---

### **Plan de Implementación FSR 1.0**

**Semana 1: Shaders**
- [ ] Investigar FSR 1.0 GLSL (AMD GitHub)
- [ ] Adaptar a OpenGL 3.3 Core
- [ ] Testear EASU + RCAS separados

**Semana 2: Integración**
- [ ] `rydit-gfx/src/fsr_upscale.rs`
- [ ] Toggle FSR on/off
- [ ] Quality modes (Performance, Balanced, Quality)

**Semana 3: Optimización**
- [ ] Benchmark performance
- [ ] Ajustar sharpness
- [ ] Documentar uso

**Riesgo**: Medio (shaders complejos)  
**Valor**: Alto (+30-50% FPS, $150K)

---

## 🎥 RYDIT-STREAM - ARQUITECTURA ORIGINAL

### **Análisis Competitivo**

| Motor | Streaming Nativo | Estado | Notas |
|-------|------------------|--------|-------|
| **Unity** | ❌ NO nativo | Requiere OBS/plugins | Third-party necesario |
| **Unreal** | ⚠️ Pixel Streaming | ✅ Existe pero complejo | Servidores dedicados ($50-200/mes) |
| **Godot** | ❌ NO nativo | Sin soporte | Comunidad pide desde 2020 |
| **RyDit** | 🔮 **En desarrollo** | 🟢 **NATIVO + Ligero** | **ÚNICO en educación/STEM** |

---

### **Ventaja Competitiva**

**Unreal Pixel Streaming** (lo más cercano):
```
✅ Funciona
❌ Requiere servidor dedicado (~$50-200/mes)
❌ Complejo de configurar (Docker, WebRTC, signaling)
❌ Enfocado en AAA graphics (no educación)
❌ No funciona en Chromebooks/Android barato
```

**RyDit Stream Portal** (propuesta):
```
✅ Nativo (sin servidores externos)
✅ LAN streaming (gratis, sin infraestructura)
✅ WebRTC simple (P2P, sin servidor central)
✅ Enfocado en educación (Chromebooks, Android)
✅ TUI táctil para control (ÚNICO)
✅ Ligero (< 500MB RAM)
```

---

### **Arquitectura Propuesta**

```
crates/rydi-stream/
├── src/
│   ├── lib.rs
│   ├── server.rs        # Servidor de streaming
│   ├── client.rs        # Cliente viewer
│   ├── lan.rs           # Discovery LAN (mDNS)
│   ├── webrtc.rs        # WebRTC para web
│   ├── rtmp.rs          # RTMP para Twitch/YouTube
│   └── websocket.rs     # WebSocket para custom
├── Cargo.toml
└── README.md
```

---

### **Features del Portal**

| Feature | Descripción | Impacto |
|---------|-------------|---------|
| **Stream Local (LAN)** | Ver escenas en otros dispositivos de la red | 🔴 ALTO |
| **Stream Web** | Broadcast a internet (YouTube, Twitch, custom) | 🔴 ALTO |
| **Multi-Viewer** | Múltiples espectadores simultáneos | 🟡 MEDIO |
| **Interactive Stream** | Viewers pueden interactuar (votar, cambiar params) | 🔴 ALTO |
| **Record & Replay** | Grabar sesiones y reproducir después | 🟡 MEDIO |
| **TUI Control** | Control táctil tipo Yazi (Android/Chromebook) | 🔴 ALTO |

---

### **Casos de Uso**

#### **1. Educación STEM** 🎓
```
Profesor crea simulación → Estudiantes ven en Chromebooks
→ Interactúan cambiando parámetros en tiempo real
```

**Valor**: ÚNICO en educación, Chromebooks baratos

#### **2. Game Development** 🎮
```
Dev juega su juego → Stream a Discord/Twitch
→ Viewers votan por power-ups/enemigos
```

**Valor**: Twitch integration nativa

#### **3. Visualización Científica** 🔬
```
Simulación N-Body → Stream a múltiples monitores
→ Control remoto desde tablets
```

**Valor**: Multi-monitor, control remoto

#### **4. Colaboración** 👥
```
2+ devs editan misma escena → Stream sincronizado
→ Cambios se reflejan en tiempo real
```

**Valor**: Google Docs para juegos

---

### **Implementación por Fases**

| Fase | Feature | Tiempo | Estado |
|------|---------|--------|--------|
| **1** | Stream Local (LAN) | 1-2 semanas | 🔮 Pendiente |
| **2** | Stream Web (WebRTC) | 2-3 semanas | 🔮 Pendiente |
| **3** | RTMP (Twitch/YouTube) | 1-2 semanas | 🔮 Pendiente |
| **4** | TUI Táctil | 2 semanas | 🔮 Pendiente |

---

### **Tecnologías**

| Tecnología | Propósito | Crate | Estado |
|------------|-----------|-------|--------|
| **WebRTC** | Stream P2P web | `webrtc-rs` | 🔮 Por integrar |
| **RTMP** | Twitch/YouTube | `rtmp-rs` | 🔮 Por integrar |
| **WebSocket** | Custom streaming | `tungstenite` | ✅ Ya usado |
| **mDNS** | Discovery LAN | `libmdns` | 🔮 Por integrar |
| **H.264/VP8** | Codec de video | `ffmpeg` bindings | 🔮 Por integrar |

---

### **Plan de Implementación Rydit-Stream**

**Semana 1: WebSocket**
- [ ] `crates/rydit-stream/` estructura
- [ ] WebSocket server/client
- [ ] Scene serialization (bytecode)

**Semana 2: LAN Discovery**
- [ ] mDNS discovery
- [ ] Auto-connect LAN
- [ ] Multi-viewer support (5+ concurrentes)

**Semana 3: WebRTC**
- [ ] WebRTC integration
- [ ] Browser viewer
- [ ] Low-latency optimization

**Semana 4: RTMP**
- [ ] RTMP streaming
- [ ] Twitch/YouTube integration
- [ ] Bitrate control

**Riesgo**: Alto (WebRTC complejo)  
**Valor**: Muy Alto ($300K+ potencial)

---

## 🎯 SINERGIA FSR + STREAM

| Feature | FSR 1.0 | Rydit-Stream | Combinado |
|---------|---------|--------------|-----------|
| **Performance** | +30-50% FPS | -10-20% overhead | **+10-30% net** ✅ |
| **Calidad** | Upscale | Compresión | **Aceptable** ✅ |
| **Low-end** | Chromebooks | Web viewers | **Edu STEM** ✅ |
| **Complejidad** | 2 shaders | 5-6 crates | **Medio** ⚠️ |
| **Valor** | $150K | $300K | **$500K+** ✅ |

---

## 📋 RECOMENDACIÓN ORIGINAL (Pre-v0.11.2)

Según QWEN.md y docs/ANALISIS_estrategico_V0.11.1.md:

### **Orden Original Sugerido**:

1. **FSR 1.0** (1-2 semanas) - **ANTES DEL PARSER**
   - ✅ Más crítico (mejora visual inmediata)
   - ✅ Beneficia a Stream (menos bandwidth)
   - ✅ 2 shaders, sin dependencias

2. **Parser Fuerte** (2-3 semanas)
   - ⚠️ Complejo (3,329 líneas monolíticas)
   - ✅ Necesario para producción

3. **Rydit-Stream** (3-4 semanas) - **DESPUÉS DE FSR**
   - ✅ Mayor valor único ($300K+)
   - ⚠️ Requiere infraestructura (WebRTC, LAN)
   - ✅ FSR beneficia a Stream

---

## 🔄 ACTUALIZACIÓN POST-v0.11.2

### **Lo Que Cambió**:

✅ **Parser completado** (v0.11.2) - 4 fases, 65 tests  
✅ **Bytecode VM** - Facilita streaming (serialización)  
✅ **Zero-Copy** - Menos overhead en streaming  

### **Lo Que Persiste**:

🔮 **FSR 1.0** - Pendiente (1-2 semanas)  
🔮 **Rydit-Stream** - Pendiente (3-4 semanas)  

### **Nueva Recomendación**:

**Opción A: FSR 1.0 Primero** (2-3 semanas)
- ✅ Sigue siendo más simple (2 shaders)
- ✅ Beneficia a Stream
- ✅ $150K valor

**Opción B: Rydit-Stream Primero** (3-4 semanas) ⭐
- ✅ Mayor valor único ($300K+)
- ✅ VM + bytecode facilita streaming
- ⚠️ Más complejo

**Recomendación Actual**: **Opción B** (Stream primero)

**Por qué**:
- ✅ VM ya está lista (bytecode serializable)
- ✅ Mayor diferenciación competitiva
- ✅ FSR puede esperar 2-3 semanas

---

## 📊 MÉTRICAS ESPERADAS

| Métrica | v0.11.2 | Con FSR | Con Stream | Ambos |
|---------|---------|---------|------------|-------|
| **FPS (1080p)** | 60 | 80-90 | 50-55 | 70-80 |
| **FPS (720p)** | 60 | 90-100 | 55-60 | 80-90 |
| **Latency** | N/A | N/A | <100ms | <100ms |
| **Viewers** | N/A | N/A | 5+ LAN | 10+ Web |
| **Valor** | $200K | $350K | $500K | $750K+ |

---

## 🔒 PUNTOS DE REVERSIÓN

| Fase | Tag | Reversión |
|------|-----|-----------|
| **Pre-FSR** | `v0.11.3-pre-fsr` | `git checkout v0.11.3-pre-fsr` |
| **Pre-Stream** | `v0.11.3-pre-stream` | `git checkout v0.11.3-pre-stream` |
| **Fase 1** | `v0.11.3-fase-1` | `git revert HEAD~3..HEAD` |
| **Fase 2** | `v0.11.3-fase-2` | `git revert HEAD~3..HEAD` |

---

## 🚀 DECISIÓN FINAL

**¿Qué implementamos?**

1. **Opción A**: FSR 1.0 primero (2-3 semanas)
2. **Opción B**: Rydit-Stream primero (3-4 semanas) ⭐ RECOMENDADO
3. **Opción C**: Ambos paralelo (4-5 semanas)

**Recomendación**: **Opción B** (Rydit-Stream primero)

**Por qué**:
- ✅ Mayor valor único ($300K+)
- ✅ Diferenciación competitiva
- ✅ Mercado STEM educativo
- ✅ FSR puede esperar 2-3 semanas
- ✅ VM + bytecode ya están listos

---

<div align="center">

**🛡️ RyDit - FSR 1.0 + RYDIT-STREAM (Pre-Existente)**

*Documentación original encontrada ✅ | QWEN.md + docs/ANALISIS_estrategico_V0.11.1.md*

**Recomendación: Rydit-Stream primero (3 semanas) | FSR 1.0 después (2 semanas)**

**Próxima decisión**: ¿Aprobamos Opción B?
</div>
