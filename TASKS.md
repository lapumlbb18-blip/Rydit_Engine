# 🛡️ Ry-Dit - Tareas v0.15.0 → v1.0.0

**Última actualización**: 2026-04-07
**Versión actual**: v0.16.0-alpha ✅ v-shield v0.2.0 Platform Layer + Sync Primitives
**Próxima versión**: v0.16.0 — Bordes suaves + Opacidad + Shaders avanzados

---

## 📊 RESUMEN RÁPIDO

| Métrica | Valor |
|---------|-------|
| **Crates** | 25 |
| **Errores** | 0 |
| **Tests** | 70/70+ pasando (workspace completo) |
| **Crates publicados** | 4 (ry-god + ry-stream v0.2.0 + v-shield v0.2.0) |
| **Demos funcionales** | 10+ (Termux-X11) |
| **GPU Instancing** | 50K partículas, 48 FPS, Adreno 610 |
| **FSR 1.0** | 960x540 → 1280x720, 48 FPS |
| **Commit** | ver `git log` |
| **Tag** | `v0.16.0-alpha` (pendiente) |

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

### 5. Health Bars + Identificadores de Entidades
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🟡 MEDIA |
| **Esfuerzo** | 4-6h |
| **Versión** | v0.16.0 |
| **Estado** | ⏳ Pendiente |

**Detalle**:
- Barras de vida que **siguen a las entidades** en pantalla
- Posición: `entity.x, entity.y - entity.height - 10`
- Color dinámico: verde (100%) → amarillo (50%) → rojo (25%)
- Fondo oscuro + barra de color (estilo RPG)
- Nombre/ID encima de la barra
- Opcional: nivel, estado (vivo/muerto)

**Referencia**: demo_torreta_vs_sprites tiene barras de vida de enemigos
(`crates/ry-rs/src/bin/demo_torreta_vs_sprites.rs` líneas ~644)

**Archivos a crear**:
- `crates/ry-gfx/src/health_bar.rs` (nuevo módulo)
- Integrar en `ry-gfx/src/lib.rs`

---

### 6. HUD de Información + Debug Overlay
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🟡 MEDIA |
| **Esfuerzo** | 4-6h |
| **Versión** | v0.16.0 |
| **Estado** | ⏳ Pendiente |

**Detalle**:
- **FPS counter** en esquina superior
- **Partículas activas** (count)
- **Posición de cámara** / entidad seleccionada
- **Memoria usada** (heap, GPU)
- **Estado del motor** (playing, paused, menu)
- **Debug toggle** (F1): hitboxes, colliders, velocidades, raycasts
- **Mini-map** opcional

**Referencia**: toolkit-ry tiene 18+ widgets HUD
(`crates/toolkit-ry/src/`)

---

### 7. 3D Viewport + Objetos Genéricos
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🟡 MEDIA |
| **Esfuerzo** | 10-15h |
| **Versión** | v0.17.0 |
| **Estado** | ⏳ Pendiente |

**Detalle**:
- Viewport 3D embebible en panel visual
- **Primitivas**: cube, sphere, cylinder, cone, torus, plane
- **Cámara 3D**: orbit (mouse drag), zoom (scroll), pan (middle-click)
- **Iluminación**: directional + ambient
- **Grid de referencia** en el suelo
- **Transform gizmo** (ejes XYZ)
- **Material básico**: color sólido, wireframe, texturado

**Estado actual de 3D**:
- `crates/ry3d-gfx/` existe con 15 funciones
- `raylib::DrawHandle` ya tiene funciones 3D básicas
- ry-backend tiene raylib_draw para 3D

**Archivos a crear**:
- `crates/ry-rs/src/bin/demo_3d_viewport.rs` (demo viewport)
- `crates/ry3d-gfx/src/viewport.rs` (viewport module)

---

## 🟡 TAREAS PARALELAS

### 8. Publicar 5+ crates en crates.io
| Campo | Valor |
|-------|-------|
| **Esfuerzo** | 4-6h |
| **Versión** | v0.16.0 |
| **Estado** | ⏳ Pendiente |

**Crates listos para publicar**:
- ry-backend v0.1.0
- events-ry v0.1.0
- ry-anim v0.12.0
- ry-gfx v0.10.7
- toolkit-ry v0.1.0
- lizer v0.11.2

**Publicados ya**: ry-god v0.1.0, ry-stream v0.1.0

---

### 9. Tareas en Paralelo (pueden ir simultáneas)

| Tarea | Puede ir con | Dependencia |
|-------|-------------|-------------|
| GitHub Actions CI | v-shield platform | Ninguna |
| Bordes suaves + opacidad | Shaders avanzados | GPU instancing ✅ |
| Health bars | HUD info | Demo torreta ✅ |
| 3D viewport | ry3d-gfx primitives | ry3d-gfx ✅ |
| Publicar crates | CI/CD | Crates estables ✅ |

**Combinación recomendada v0.16.0**:
```
Semana 1: CI/CD + Bordes suaves (paralelo)
Semana 2: Health bars + HUD info (paralelo)
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
| **v0.16.0** | Bordes suaves + Opacidad + Shaders + Health bars + HUD | 20-30h | 2-3 meses |
| **v0.17.0** | 3D Viewport + Objetos genéricos + Cámara orbit | 15-20h | 3-4 meses |
| **v0.18.0** | v-shield completo + GitHub Actions + CI multi-plataforma | 15-20h | 4-5 meses |
| **v0.19.0** | Texturas + Sprite animation system + Tilemap editor | 20-25h | 5-6 meses |
| **v0.20.0** | Motor multiplataforma completo (Linux/Win/Mac/Android/WASM) | 25-30h | 6-8 meses |
| **v1.0.0** | Motor estable: editor visual, scripting, docs, comunidad | 50-80h | 12-18 meses |

---

## 📊 DEPENDENCIAS ENTRE TAREAS

```
v0.15.0 ✅ (GPU Instancing + FSR)
    │
    ├──→ v0.16.0: Bordes suaves + Opacidad (usa GPU instancer ✅)
    │       │
    │       └──→ Shaders avanzados (bloom, glow, outline)
    │
    ├──→ v0.16.0: Health bars + HUD (usa demo_torreta ✅)
    │       │
    │       └──→ Debug overlay
    │
    ├──→ v0.16.0: GitHub Actions CI (independiente)
    │
    ├──→ v0.17.0: 3D Viewport (usa ry3d-gfx ✅)
    │       │
    │       └──→ Iluminación + materiales
    │
    └──→ v0.18.0: v-shield platform (base para todo)
            │
            └──→ Multiplataforma real
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

**🛡️ Ry-Dit v0.15.0 — GPU Instancing + FSR 1.0 + Manifiesto**

*25 crates · 0 errores · 48 FPS Adreno 610 · Low-End First*

**Próximo: v0.16.0 — Bordes suaves + Opacidad + Shaders + Health bars + CI/CD**

</div>
