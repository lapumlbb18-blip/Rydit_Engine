# 🛡️ Ry-Dit - Tareas v0.16.0 → v1.0.0

**Última actualización**: 2026-04-09
**Versión actual**: v0.16.0 ✅ Health Bars + HUD + Cámara 2D + ry3d-gfx + 12 crates publicados
**Próxima versión**: v0.17.0 — 3D en PC, iluminación, materiales

---

## 📊 RESUMEN RÁPIDO

| Métrica | Valor |
|---------|-------|
| **Crates** | 23 |
| **Errores** | 0 |
| **Tests** | 95+ pasando |
| **Crates publicados** | 12 (ry-god, ry-stream, v-shield, ry-backend, migui, ry-gfx, ry-core, ry-anim, toolkit-ry, ry-config, ry-physics, ry-science) |
| **CI/CD** | ✅ Linux + Windows + macOS |
| **Demos funcionales** | 11+ (Termux-X11) |
| **GPU Instancing** | 50K partículas, 48 FPS, Adreno 610 |
| **FSR 1.0** | 960x540 → 1280x720, 48 FPS |
| **Health Bars** | ✅ world-space, color dinámico |
| **Cámara 2D** | ✅ Zoom + rotación + follow suave |
| **Commits** | `42fef11` |

---

## ✅ v0.15.0 COMPLETADA

| # | Feature | Estado | Notas |
|---|---------|--------|-------|
| 1 | GPU Instancing funcional | ✅ | 50K partículas, 48 FPS, Adreno 610/Zink |
| 2 | FSR 1.0 con FBO | ✅ | Render-to-texture, EASU upscale |
| 3 | Shaders VAO fixeados | ✅ | instance_vbo, stride 16B, TRIANGLES |
| 4 | Shaders fragment fixeados | ✅ | vLocalPos, quad sólido |
| 5 | demo_gpu_instancing | ✅ | SDL2+OpenGL puro, estrellas animadas |
| 6 | demo_fsr | ✅ | Pipeline FBO → upscale → screen |
| 7 | patron_gpu_instancing.md | ✅ | Documento completo del patrón |
| 8 | Launchers Zink | ✅ | Detección automática DISPLAY |
| 9 | Manifiesto Low-End First | ✅ | Filosofía, propósito, visión |
| 10 | Docs redes sociales | ✅ | YouTube, X, Reddit, Discord |
| 11 | gpu_debug, gpu_solid, gpu_triangle, gpu_circle_test | ✅ | Diagnósticos |

### Bugs fixeados en v0.15.0

| # | Bug | Fix |
|---|-----|-----|
| 1 | `instance_vbo` no bindeado | `glBindBuffer` antes de atributos |
| 2 | Stride location 0 = 8 bytes | Stride = 16 (4 floats × 4 bytes) |
| 3 | `QUADS` en Core Profile | 6 vértices (2 triángulos) + `TRIANGLES` |
| 4 | `vLocalPos` sin escala en FS | `length(vLocalPos * 2.0)` |
| 5 | `uResolution` no seteado | `inst.set_resolution()` + uniform |
| 6 | `glViewport` no configurado | `gl::Viewport(0, 0, 1280, 720)` cada frame |
| 7 | `glScissorTest` cortando | `gl::Disable(gl::SCISSOR_TEST)` |
| 8 | Shaders desde path relativo | `include_str!()` → `/usr/tmp/` |

---

## ✅ v0.16.0-alpha COMPLETADA — v-shield Platform Layer + Sync

| # | Feature | Estado | Notas |
|---|---------|--------|-------|
| 1 | v-shield v0.2.0 | ✅ | Platform detection + sync primitives + PlatformSync |
| 2 | ry-gfx v0.10.8 | ✅ | Usa v-shield PlatformSync (migrado de render_queue.rs) |
| 3 | ry-stream v0.2.0 | ✅ | Usa v-shield sync::Mutex |
| 4 | Tests | ✅ | 39/39 pasando (26 v-shield + 17 ry-stream) |
| 5 | README v-shield | ✅ | Documentación completa con features |
| 6 | Cargo.toml features | ✅ | native, wasm, graphics, async-tokio |
| 7 | ry-rs tests fix | ✅ | 65 errores fixeados → 31 tests pasando |

---

## 🔴 TAREAS PRINCIPALES (v0.16.0)

### 1. ~~Corregir tests de ry-rs~~ → ✅ COMPLETADO
| Campo | Valor |
|-------|-------|
| **Prioridad** | ✅ HECHO |
| **Esfuerzo** | ~1h real |
| **Versión** | v0.16.0-alpha |
| **Estado** | ✅ Completado |

**Lo que se hizo**:
- ✅ `Vec<Stmt>` → `Vec<Stmt<'static>>` en 3 setup_test()
- ✅ `Expr::Texto("x".to_string())` → `Expr::Texto("x")` en 54 ocurrencias
- ✅ `Expr::Texto(id.clone())` → `Expr::Texto(&id)` en 6 ocurrencias
- ✅ `collides_with(body)` → `collides_with(&body)` en 2 líneas
- ✅ 31 tests pasando en ry-rs --bin rydit-rs (antes 65 errores)
- ✅ Causa raíz documentada: lifetimes en AST de ry-parser

---

### 2. ~~v-shield Platform Layer + Sync~~ → ✅ COMPLETADA en v0.16.0-alpha
| Campo | Valor |
|-------|-------|
| **Prioridad** | ✅ HECHO |
| **Esfuerzo** | ~4h reales |
| **Versión** | v0.16.0-alpha |
| **Estado** | ✅ Completado |

**Lo que se hizo**:
- ✅ Platform detection (Linux, Windows, macOS, Android, iOS, WASM)
- ✅ Sync primitives (Mutex, RwLock, Barrier, Condvar)
- ✅ Platform Sync migrado de ry-gfx a v-shield
- ✅ ry-gfx usa v-shield PlatformSync (re-export)
- ✅ ry-stream usa v-shield sync::Mutex
- ✅ 26 tests v-shield + 17 tests ry-stream = 43/43 pasando
- ✅ README documentado

---

### 3. Publicar crates en crates.io
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🔴 ALTA |
| **Esfuerzo** | 1-2h |
| **Versión** | v0.16.0 |
| **Estado** | ⏳ Pendiente |

**Crates a publicar**:
- v-shield v0.2.0 (nuevo)
- ry-stream v0.2.0 (actualización)

**Comandos**:
```bash
cargo publish -p v-shield
cargo publish -p ry-stream
```

---

### 2. GitHub Actions CI/CD
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🔴 ALTA |
| **Esfuerzo** | 4-6h |
| **Versión** | v0.16.0 |
| **Estado** | ⏳ Pendiente |

**Detalle**:
- Runner `ubuntu-latest`: build + tests de todo el workspace
- Runner `windows-latest`: build de crates compatibles
- Runner `macos-latest`: build de crates compatibles
- Android cross-compile: `cargo build --target aarch64-linux-android`
- Artifact: ELF release + crates publish automático
- Previene regressions

**Archivos a crear**:
- `.github/workflows/ci.yml`
- `.github/workflows/release.yml`

---

### 3. Bordes Suaves + Opacidad
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🔴 ALTA |
| **Esfuerzo** | 6-8h |
| **Versión** | v0.16.0 |
| **Estado** | ⏳ Pendiente |

**Detalle**:
- Anti-aliasing en GPU Instancing: fragment shader con `smoothstep` + `discard`
- Alpha blending por partícula (ya soportado por `ParticleData.color.a`)
- Opacidad global de entidades (HUD, menús, transiciones)
- Fade in/out para transiciones de escenas
- Texturas con canal alpha (PNG con transparencia)

**Archivos a modificar**:
- `crates/ry-gfx/shaders/fragment.glsl` (smoothstep + discard)
- `crates/ry-gfx/src/gpu_instancing.rs` (soporte alpha)
- `crates/ry-gfx/src/render_queue.rs` (DrawCommand con opacidad)

---

### 4. Shaders Avanzados
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🟡 MEDIA |
| **Esfuerzo** | 8-12h |
| **Versión** | v0.16.0-v0.17.0 |
| **Estado** | ⏳ Pendiente |

**Detalle**:
- **Bloom**: post-proceso con blur + aditivo
- **Glow**: outline luminoso alrededor de entidades
- **Outline**: borde de color alrededor de sprites
- **Blur**: desenfoque gaussiano para fondos/UI
- **Color grading**: LUT de colores para atmósfera
- **Chromatic aberration**: efecto retro/distorsión
- **Pixel art shader**: downscale + nearest neighbor

**Referencia shaders actuales**:
- `crates/ry-gfx/shaders/fsr_upscale.glsl` (EASU bilinear + edge-adaptive)
- `crates/ry-gfx/shaders/fsr_sharpen.glsl` (RCAS contrast-adaptive)
- `crates/ry-gfx/shaders/vertex.glsl` (GPU instancing NDC)
- `crates/ry-gfx/shaders/fragment.glsl` (quad sólido)

---

### 5. ~~Health Bars + Identificadores de Entidades~~ → ✅ COMPLETADA en v0.16.0
| Campo | Valor |
|-------|-------|
| **Prioridad** | ✅ HECHO |
| **Esfuerzo** | ~4h reales |
| **Versión** | v0.16.0 |
| **Estado** | ✅ Completado |

**Lo que se hizo**:
- ✅ EntityHUD struct en toolkit-ry/world_hud.rs
- ✅ draw_entity_health_bar_world() con world-space → screen-space
- ✅ Color dinámico: verde (>50%) → amarillo (25-50%) → rojo (<25%)
- ✅ Nombres/IDs con TTF cacheado encima de la barra
- ✅ Integrado en demo_hud_camera

---

### 6. ~~HUD de Información + Debug Overlay~~ → ✅ COMPLETADA en v0.16.0
| Campo | Valor |
|-------|-------|
| **Prioridad** | ✅ HECHO |
| **Esfuerzo** | ~4h reales |
| **Versión** | v0.16.0 |
| **Estado** | ✅ Completado |

**Lo que se hizo**:
- ✅ Debug overlay: FPS, cámara (x,y,zoom,rot), entidades, tiempo, memoria
- ✅ Stats HUD: Score, tiempo MM:SS, nivel (esquina superior derecha)
- ✅ Texturas TTF cacheadas con refresco cada 30 frames
- ✅ Minimap avanzado con entidades coloreadas por tipo
- ✅ Viewport visible en minimap

---

### 7. ~~3D Viewport + Objetos Genéricos~~ → ✅ ry3d-gfx mejorado en v0.16.0
| Campo | Valor |
|-------|-------|
| **Prioridad** | ✅ HECHO |
| **Esfuerzo** | ~3h reales |
| **Versión** | v0.16.0 |
| **Estado** | ✅ Completado (parcial - ry3d-gfx mejorado) |

**Lo que se hizo**:
- ✅ Modelo3D load: GLTF/OBJ/IQM/VOX support
- ✅ draw_text_3d: Texto en espacio 3D
- ✅ draw_model: Renderizado de modelos 3D con transform
- ⏳ Viewport 3D embebible (pendiente para v0.17.0)

---

## 🟡 TAREAS PARALELAS

### 8. ~~Publicar 5+ crates en crates.io~~ → ✅ COMPLETADA (12 total)
| Campo | Valor |
|-------|-------|
| **Esfuerzo** | 4-6h |
| **Versión** | v0.16.0 |
| **Estado** | ✅ Completado |

**Crates publicados**:
- ry-config v0.1.0 ✅
- ry-physics v0.7.34 ✅
- ry-science v0.7.34 ✅
- Total: 12 crates publicados

---

### 9. Tareas en Paralelo (pueden ir simultáneas)

| Tarea | Puede ir con | Dependencia |
|-------|-------------|-------------|
| 3D en PC | Iluminación + materiales | ry3d-gfx ✅ |
| Bordes suaves + opacidad | Shaders avanzados | GPU instancing ✅ |
| GitHub Actions CI mejorado | Independiente | CI existente ✅ |
| Publicar más crates | CI/CD | Crates estables ✅ |

**Combinación recomendada v0.17.0**:
```
Semana 1: 3D en PC + iluminación (paralelo)
Semana 2: Bordes suaves + opacidad (paralelo)
Semana 3: Shaders avanzados (bloom, glow)
Semana 4: Testing + documentación
```

---

### 10. ry-rs: Desacoplar y Completar
| Campo | Valor |
|-------|-------|
| **Esfuerzo** | 8-12h |
| **Estado** | ⏳ Pendiente |

**Problemas**:
- main.rs: ~5000 líneas, acoplamiento alto
- Dos RyditModule traits incompatibles
- Solo binario, falta `[lib]`

**Plan**:
1. Unificar traits
2. Agregar `[lib]`
3. Extraer eval/ si es posible

---

## 🔮 ROADMAP v0.16.0 → v1.0.0

| Versión | Feature | Esfuerzo | Target |
|---------|---------|----------|--------|
| **v0.16.0** | Health Bars + HUD + Cámara 2D + ry3d-gfx + 12 crates publicados | ✅ HECHO | ✅ |
| **v0.17.0** | 3D en PC + Iluminación + Materiales + Bordes suaves + Opacidad | 30-40h | 2-3 meses |
| **v0.18.0** | GitHub Actions CI mejorado + Shaders avanzados | 15-20h | 3-4 meses |
| **v0.19.0** | Texturas + Sprite animation system + Tilemap editor | 20-25h | 5-6 meses |
| **v0.20.0** | Motor multiplataforma completo (Linux/Win/Mac/Android/WASM) | 25-30h | 6-8 meses |
| **v1.0.0** | Motor estable: editor visual, scripting, docs, comunidad | 50-80h | 12-18 meses |

---

## 📊 DEPENDENCIAS ENTRE TAREAS

```
v0.15.0 ✅ (GPU Instancing + FSR)
    │
    ├──→ v0.16.0-alpha: CI + 6 crates publicados ✅
    │       │
    │       └──→ v0.16.0: Health bars + HUD + Cámara 2D + ry3d-gfx ✅
    │
    ├──→ v0.17.0: 3D en PC + iluminación (usa ry3d-gfx ✅)
    │       │
    │       ├──→ Bordes suaves + opacidad (usa GPU instancer ✅)
    │       │
    │       └──→ Shaders avanzados (bloom, glow, outline)
    │
    ├──→ v0.18.0: GitHub Actions CI mejorado + shaders
    │
    └──→ v0.19.0: Texturas + sprite animation
            │
            └──→ v0.20.0: Motor multiplataforma
                    │
                    └──→ v1.0.0: Motor completo
```

---

## 📋 ARCHIVOS CLAVE PARA PRÓXIMAS VERSIONES

### Shaders existentes (base para avanzados)
```
crates/ry-gfx/shaders/
├── vertex.glsl          # GPU instancing NDC
├── fragment.glsl        # Quad sólido
├── fsr_upscale.glsl     # EASU bilinear + edge-adaptive
├── fsr_sharpen.glsl     # RCAS contrast-adaptive
├── fragment_test.glsl   # Test sólido
└── [nuevos: bloom.glsl, glow.glsl, outline.glsl, blur.glsl]
```

### Demos diagnósticos (no borrar, útiles para debug)
```
crates/ry-rs/src/bin/
├── gpu_debug.rs         # 9 partículas grandes para debug
├── gpu_solid.rs         # Quads sólidos sin círculo
├── gpu_triangle.rs      # Triángulo NDC mínimo
├── gpu_circle_test.rs   # 9 círculos con raylib (confirmado funciona)
├── demo_gpu_instancing.rs  # Demo principal 50K estrellas
└── demo_fsr.rs          # FSR con pipeline FBO
```

### Documentos de referencia
```
patron_gpu_instancing.md  # Pipeline funcional SDL2+OpenGL
MANIFIESTO.md             # Filosofía Low-End First
DESCRIPCION_YOUTUBE.md    # Texto para canal YouTube
DESCRIPCION_TWITTER.md    # Bio + 6 tweets listos
POST_REDDIT.md            # Post para Reddit
DISCORD_SERVER.md         # Config servidor Discord
```

---

## 🌐 PUBLICACIÓN EN REDES

- **GitHub**: ✅ Publicado (`c409f98`, tag `v0.15.0`)
- **YouTube**: Bio + descripción lista (`DESCRIPCION_YOUTUBE.md`)
- **X/Twitter**: Bio + 6 tweets listos (`DESCRIPCION_TWITTER.md`)
- **Reddit**: Post listo para r/rust, r/gamedev (`POST_REDDIT.md`)
- **Discord**: Estructura de servidor lista (`DISCORD_SERVER.md`)
- **Manifiesto**: `MANIFIESTO.md` — filosofía completa

---

<div align="center">

**🛡️ Ry-Dit v0.16.0 — Health Bars + HUD + Cámara 2D + ry3d-gfx + 12 crates publicados**

*23 crates · 0 errores · 12 crates.io · 95+ tests · Low-End First*

**Próximo: v0.17.0 — 3D en PC + Iluminación + Materiales + Bordes suaves**

</div>
