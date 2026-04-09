# 🛡️ QWEN.md - Bitácora Técnica Ry-Dit

**Última actualización**: 2026-04-09
**Versión actual**: v0.16.0 ✅ Health Bars + HUD + Cámara 2D + ry3d-gfx + 12 crates publicados
**Versión anterior**: v0.15.0 (GPU Instancing + FSR 1.0)
**Próxima versión**: v0.17.0 - 3D en PC, iluminación, materiales
**Commit**: `42fef11`
**Repositorio**: `https://github.com/lapumlbb18-blip/Ry-dit`
**Crates publicados**: 12 ✅

---

## 🎉 v0.16.0 COMPLETADA — Health Bars + HUD + Cámara 2D + ry3d-gfx + 12 crates publicados

### ✅ **ESTADO: 23 CRATES | 12 PUBLICADOS | 0 ERRORES | 95+ TESTS**

| Feature | Estado | Detalles |
|---------|--------|----------|
| **Health Bars** | ✅ | toolkit-ry/world_hud.rs, EntityHUD, color dinámico (verde→amarillo→rojo) |
| **HUD Debug Overlay** | ✅ | FPS, cámara (x,y,zoom,rot), entidades, tiempo, memoria |
| **Stats HUD** | ✅ | Score, tiempo MM:SS, nivel, TTF cacheado (refresco 30 frames) |
| **Cámara 2D** | ✅ | Zoom 0.2-5x, rotación 0-360°, follow_smooth(), set_bounds() |
| **Minimap avanzado** | ✅ | Entidades coloreadas por tipo, viewport, jugador verde |
| **demo_hud_camera** | ✅ | Demo funcional con todos los HUD + cámara interactiva |
| **ry3d-gfx mejorado** | ✅ | Modelo3D load (GLTF/OBJ/IQM/VOX), draw_text_3d, draw_model |
| **Launchers Zink** | ✅ | launcher_hud_camera.sh con auto-detección DISPLAY |
| **ry-config publicado** | ✅ | v0.1.0 en crates.io |
| **ry-physics publicado** | ✅ | v0.7.34 en crates.io |
| **ry-science publicado** | ✅ | v0.7.34 en crates.io |
| **ry-test eliminado** | ✅ | Código muerto removido (-1 crate) |

### **Crates Publicados en esta sesión**

| Crate | Versión | crates.io |
|-------|---------|-----------|
| ry-config | 0.1.0 | ✅ |
| ry-physics | 0.7.34 | ✅ |
| ry-science | 0.7.34 | ✅ |

### **Total Crates Publicados: 12**

| # | Crate | Versión | Sesión |
|---|-------|---------|--------|
| 1 | ry-god | 0.1.0 | v0.12.0 |
| 2 | ry-stream | 0.2.0 | v0.16.0-alpha |
| 3 | v-shield | 0.2.0 | v0.16.0-alpha |
| 4 | ry-backend | 0.1.0 | v0.16.0-alpha |
| 5 | migui | 0.4.1 | v0.16.0-alpha |
| 6 | ry-gfx | 0.10.8 | v0.16.0-alpha |
| 7 | ry-core | 0.8.2 | v0.16.0-alpha |
| 8 | ry-anim | 0.12.0 | v0.16.0-alpha |
| 9 | toolkit-ry | 0.1.0 | v0.16.0-alpha |
| 10 | **ry-config** | **0.1.0** | **v0.16.0** |
| 11 | **ry-physics** | **0.7.34** | **v0.16.0** |
| 12 | **ry-science** | **0.7.34** | **v0.16.0** |

### **Fixes Aplicados**

| Bug | Problema | Solución |
|-----|----------|----------|
| **ry-rs: feature `migui`** | Dependencia ry-gfx sin feature `migui` habilitada | Agregar `features = ["migui"]` en Cargo.toml |
| **demo_render_queue.rs** | Assets::new() en swap_and_execute | Mover creación fuera del loop |
| **ry-physics Cargo.toml** | URL incorrecta | Fix a repository URL |
| **ry-science Cargo.toml** | URL incorrecta | Fix a repository URL |

### **Comandos demo_hud_camera**

```bash
./launcher_hud_camera.sh

# Controles:
#   ←→↑↓ / WASD: Mover cámara
#   +/-: Zoom (0.2x - 5.0x)
#   Q/E: Rotación (-/+ 15°)
#   R: Reset cámara
#   D: Toggle debug overlay
#   M: Toggle minimap
#   H: Toggle health bars
#   ESC: Salir
```

---

## 🎉 v0.15.0 COMPLETADA — GPU Instancing + FSR 1.0

### ✅ **ESTADO: 2 DEMOS GPU FUNCIONALES | Zink/Adreno 610 | 0 ERRORES**

| Demo | GPU | FPS | Partículas/Resolución | Draw Calls | Notas |
|------|-----|-----|----------------------|------------|-------|
| **demo_gpu_instancing** | zink (Adreno 610) | ~53 FPS | 50K en 1 draw call | 1 | TRIANGLES instanced, shaders embebidos |
| **demo_fsr** | zink (Adreno 610) | ~48 FPS | 960x540 → 1280x720 (FSR Quality) | FBO + upscale | FBO render-to-texture + EASU bilinear + edge-adaptive |

### **Fixes Críticos Aplicados**

| Bug | Problema | Solución |
|-----|----------|----------|
| **gl_PointCoord en fragment shader** | Solo funciona con `gl_POINTS`, no con `gl_QUADS` instanced | Agregar `vLocalPos` varying desde vertex shader, calcular `length(vLocalPos)` |
| **gl::QUADS en Core Profile 3.3** | `QUADS` no existe en OpenGL Core Profile → `GL_INVALID_ENUM` | Cambiar quad de 4 vértices a 2 triángulos (6 vértices) + `gl::TRIANGLES` |
| **Shaders desde path relativo** | Crash al no encontrar archivos (`crates/ry-gfx/shaders/`) | Embeber shaders con `include_str!()` → escribir a `/usr/tmp/` en runtime |
| **FSR: FS como VS** | `fsr_upscale.glsl` (fragment shader) se usaba como vertex shader → `gl_FragCoord undeclared` | Crear `FSR_VS_SRC` genérico fullscreen quad + pasar `vUV` como varying |
| **gl_FragCoord en FSR upscale** | Dependía de `gl_FragCoord / outputSize` — incompatible con VS genérico | Usar `vUV` del vertex shader directamente como coordenada de textura |
| **llvmpipe en vez de Zink** | SDL2 creaba contexto OpenGL nativo → software rendering | Variables: `MESA_LOADER_DRIVER_OVERRIDE=zink GALLIUM_DRIVER=zink` |

### **Nuevos Archivos**

| Archivo | Tipo | Descripción |
|---------|------|-------------|
| `demo_gpu_instancing.rs` | demo bin | 50K-150K partículas instanciadas, cámara interactiva, controles |
| `demo_fsr.rs` | demo bin | Pipeline FBO → FSR upscale → screen, quality toggle, auto-detect |
| `launcher_gpu_instancing.sh` | script | Detección automática DISPLAY + Zink + GPU Adreno |
| `launcher_fsr.sh` | script | Detección automática DISPLAY + Zink + GPU Adreno |
| `fsr.rs` → `FboFrame` | módulo ry-gfx | Framebuffer Object para render-to-texture |

### **Comandos**

```bash
# GPU Instancing 50K partículas (1 draw call)
./launcher_gpu_instancing.sh

# FSR 1.0 Quality (960x540 → 1280x720)
./launcher_fsr.sh

# Controles demo_gpu_instancing:
#   1-6: 10K/25K/50K/75K/100K/150K partículas
#   ←→↑↓ / WASD: Mover cámara
#   +/-: Tamaño
#   P: Pausa, R: Regenerar

# Controles demo_fsr:
#   F: Cycle calidad (Quality → Balanced → Performance)
#   E: Toggle FSR ON/OFF
#   A: Toggle auto-detect (baja resolución si FPS < 30)
```

### **Benchmarks en Termux-X11 / Adreno 610**

| Configuración | GPU Instancing (50K) | FSR Quality (960→1280) |
|---------------|---------------------|------------------------|
| llvmpipe (CPU) | 217 FPS | N/A |
| zink (Adreno 610) | 53 FPS | 48 FPS |
| Sin FSR (1280x720 nativo) | N/A | ~30 FPS (estimado) |

**Ganancia FSR**: ~60% más FPS al renderizar a 960x540 y upscale a 1280x720 vs nativo.

---

## 🎉 v0.14.0 COMPLETADA

### ✅ **ESTADO ACTUAL: 25 CRATES COMPILANDO | 0 ERRORES | 95+ TESTS**

| Sistema | Estado | Cambios | Notas |
|---------|--------|---------|-------|
| **ry-backend** | ✅ v0.1.0 | Dual backend | raylib + SDL2 TTF/input/audio |
| **events-ry** | ✅ v0.1.0 | Input 3 capas | InputEvent, TextInput, Shell + Sdl2InputBackend |
| **ry-config** | ✅ v0.1.0 | Config parser | entities, levels, checkpoints - zero deps |
| **ry-system-ry** | ✅ v0.14.0 | RySystem struct | core + gui |
| **ry-anim** | ✅ v0.12.0 | 41 funciones | 58 tests, 4 demos |
| **ry-stream** | ✅ crates.io | v0.1.0 publicado | LAN streaming |
| **ry-god** | ✅ crates.io | v0.1.0 publicado | Security & Efficiency |
| **Crates** | ✅ 25/25 | 0 errores | Workspace completo |
| **ELFs** | ✅ 9+ compilados | demo_gpu_instancing ~500K, demo_fsr ~480K, demo_torreta_vs_sprites 434K | release |
| **Bins** | ✅ ~33+ | src/bin/ | Demos + tests |

**Total**: Juego completo funcional + 2 crates publicados ✅

---

## 📋 METODOLOGÍA APLICADA v0.14.0

### **Sesión Completa (todo lo hecho)**

| # | Feature | Estado | Detalles |
|---|---------|--------|----------|
| 1 | ry-backend v0.1.0 | ✅ | Dual backend: raylib + SDL2 TTF/input/audio |
| 2 | events-ry v0.1.0 | ✅ | Input unificado 3 capas + Sdl2InputBackend |
| 3 | ry-config v0.1.0 | ✅ | Config parser (entities, levels, checkpoints) - zero deps |
| 4 | ry-system-ry v0.14.0 | ✅ | Sistema unificado: RySystem (core + gui) |
| 5 | migui → ry-backend | ✅ | Conectado a ry-backend (no sdl2 directo) |
| 6 | demo_torreta_vs_sprites | ✅ | JUEGO COMPLETO: menú + 3 niveles + cámara + AI + audio (434K) |
| 7 | demo_menu_bar | ✅ | Menús Dear ImGui + mouse completo + touch (330K) |
| 8 | demo_panel_visual | ✅ | 4 paneles + consola interactiva (339K) |
| 9 | ry-rs bin + lib | ✅ | Antes solo bin |
| 10 | Código muerto eliminado | ✅ | module.rs (230 líneas, RyditModule duplicado) |
| 11 | Tests desactualizados | ✅ | Movidos a docs/tests_referencia/ |
| 12 | lizer AST cache | ✅ | FNV-1a, 256 entradas, LRU |
| 13 | Texto TTF profesional | ✅ | Anti-alias blended |
| 14 | Mouse events completos | ✅ | Click, doble click, derecho, scroll |
| 15 | Touch Android | ✅ | FingerDown/Motion/Up |
| 16 | Features multi-backend | ✅ | raylib-only, sdl2-only, dual-backend, mobile-hybrid |
| 17 | Demos existentes confirmados | ✅ | demo_rigidbody, demo_anime_ry, demo_ttf_sprites, demo_platformer_completo, demo_completo_sdl2, demo_50k_particulas, demo_colisiones |
| 18 | Documentos actualizados | ✅ | README, QWEN, ESTRUCTURA, ROADMAP |

### **demo_torreta_vs_sprites - Features**

| Feature | Estado |
|---------|--------|
| Sprites PNG | ✅ |
| Texto TTF real | ✅ |
| Física + colisiones | ✅ |
| Audio SDL2 | ✅ |
| Cámara 2D follow | ✅ |
| Mapa extenso (1200x800) | ✅ |
| HUD (toolkit-ry ready) | ✅ |
| Menús (migui MenuBar) | ✅ |
| Game states: Menu, Playing, Paused, GameOver, GameWin, LevelComplete | ✅ |
| 3 niveles con dificultad creciente | ✅ |
| Enemigos con patrol AI | ✅ |
| Huecos (caer = -1 vida) | ✅ |
| Pausa + reinicio | ✅ |

### **Controles demo_torreta_vs_sprites**
- **← → ó A/D**: Mover torreta
- **W ó ↑**: Saltar
- **S ó ↓**: Bajar rápido
- **SPACE**: Disparar
- **P**: Pausa
- **R**: Reiniciar nivel
- **ESC**: Salir / Volver menú

### **Pipeline gráfico**
- Zink/DRI3 → OpenGL ES → VirGL fallback
- SDL2_ttf para texto profesional (anti-alias blended)
- SDL2_image para sprites PNG

---

## 📦 Lista de Crates (23)

| Crate | Versión | Estado | Notas |
|-------|---------|--------|-------|
| ry-core | 0.8.2 | ✅ crates.io | Core traits, module system |
| ry-lexer | 0.1.0 | ✅ | Zero-copy lexer |
| ry-parser | 0.1.0 | ✅ | Parser AST + error recovery |
| ry-vm | — | ⚠️ | VM opcodes |
| ry-gfx | 0.10.8 | ✅ crates.io | Graphics (raylib + SDL2 + OpenGL FFI) |
| ry-physics | 0.7.34 | ✅ crates.io | + nbody_simulate |
| ry-anim | 0.12.0 | ✅ crates.io | 41 funciones, 58 tests |
| ry-science | 0.7.34 | ✅ crates.io | Geometry 2D + stats + Bezier |
| ry-script | 0.8.2 | ✅ | Script loading |
| ry-stream | 0.2.0 | ✅ crates.io | LAN streaming |
| ry-god | 0.1.0 | ✅ crates.io | Security & Efficiency |
| ry-loader | — | ⚠️ | Module loader |
| ry-rs | — | Main | Binary + lib |
| ry-system-ry | 0.14.0 | ✅ | Sistema unificado: RySystem (core + gui) |
| ry-backend | 0.1.0 | ✅ crates.io | Dual backend: raylib + SDL2 TTF/input/audio |
| ry-config | 0.1.0 | ✅ crates.io | Config parser (entities, levels, checkpoints) |
| toolkit-ry | 0.1.0 | ✅ crates.io | 5 temas + 20+ widgets + world_hud |
| migui | 0.4.1 | ✅ crates.io | Conectado a ry-backend |
| blast-core | 0.1.0 | ✅ | Minimal value executor |
| lizer | 0.11.2 | ✅ | Legacy + AST cache real (FNV-1a, 256 entradas, LRU) |
| v-shield | 0.2.0 | ✅ crates.io | Platform layer + sync |
| ry3d-gfx | mejorado | ✅ | Modelo3D load (GLTF/OBJ/IQM/VOX), draw_text_3d, draw_model |
| events-ry | 0.1.0 | ✅ | Input unificado 3 capas + Sdl2InputBackend |
| ~~ry-ecs~~ | — | 🗑️ | Eliminado (-1,143 líneas) |
| ~~ry-test~~ | — | 🗑️ | Eliminado (código muerto) |

---

## 🔴 LECCIONES CRÍTICAS

### **Bug Crítico: self.advance() faltante**
**Problema**: 3 refactorizaciones fallidas por 1 línea missing
**Causa**: `parse_statement()` no consumía el ident antes de delegar
**Síntoma**: Mismos errores persistentes sin importar los parches
**Solución**: `self.advance()` antes de `parse_call_or_ident(name)`
**Regla de oro**: SIEMPRE consumir el token actual antes de delegar

### **NUNCA usar sed para código estructural**
- ✅ sed seguro SOLO para imports simples (`rydit_` → `ry_`)
- ❌ sed peligroso para código multilínea o con contexto
- **Siempre usar el tool `edit` para modificar archivos**

### **Investigar la raíz, no parchar síntomas**
- El agente de investigación encontró el bug en 1 análisis profundo
- 3 refactorizaciones vs 1 análisis = diferencia abismal

### **Tests desactualizados son peor que no tener tests**
- 151 errores de tests por nombres de AST viejos
- Solución: mover a docs/tests_referencia/ y crear nuevos

### **No dar vueltas en círculos con demos**
- demo_rigidbody YA funciona con Sdl2Backend de ry-gfx
- No crear demos duplicados (demo_ttf_sprites_real falló)
- Usar los que ya compilan: demo_rigidbody, demo_ttf_sprites, demo_anime_ry

---

## 🚀 PRÓXIMOS PASOS (v0.17.0)

### **Pendientes - Prioridad Alta**

| Tarea | Esfuerzo | Prioridad |
|-------|----------|-----------|
| 3D en PC con iluminación | 12-16h | 🔴 Alta |
| Materiales y texturas 3D | 10-15h | 🟡 Media |
| Bordes suaves (antialiasing) | 8-12h | 🟡 Media |

### **Pendientes - Prioridad Media/Futura**

| Tarea | Esfuerzo | Prioridad |
|-------|----------|-----------|
| Opacidad/transparencia | 6-8h | 🟡 Media |
| Shaders avanzados (bloom, glow) | 10-15h | 🟡 Media |
| GitHub Actions CI mejorado | 6-8h | 🟡 Media |
| ry-stream v0.3.0 (mDNS) | 8-12h | 🟡 Media |
| ry-physics N-cuerpos >2 | 10-15h | 🟡 Media |
| Texturas + sprite animation system | 10-15h | 🔮 Futuro |
| Motor multiplataforma completo | 20-30h | 🔮 Futuro |
| Soporte de emojis en TTF | 4-6h | 🔮 Futuro |
| GIF animation | 8-12h | 🔮 Futuro |
| Features 3D paso a paso | 12-16h | 🔮 Futuro |
| LAZOS Python bridge | 20-30h | 🔮 Futuro |
| Editor visual | 24-32h | 🔮 Futuro |

### **Después de v0.16.0**

1. 3D en PC con iluminación y materiales
2. Más crates publicados en crates.io
3. Documentación completa
4. Videos de demos

---

## ⚠️ ADVERTENCIA CRÍTICA: SED ES PELIGROSO

### **Regla de Oro**

> **"Después de refactorizar parser con lifetimes, NUNCA uses sed. Solo fix manual + debug tests."**

| Herramienta | ¿Usar? | ¿Cuándo? | Riesgo |
|-------------|--------|----------|--------|
| **sed** | ❌ **NUNCA** | Nunca en código refactorizado | 🔴 ALTO |
| **cargo clippy --fix** | ✅ SÍ | Warnings simples | 🟢 Bajo |
| **Fix manual** | ✅ SÍ | Siempre que sea posible | 🟢 Bajo |
| **Debug tests** | ✅ SÍ | Antes de fixear | 🟢 Bajo |
| **cargo expand** | ✅ SÍ | Ver macros expandidas | 🟢 Bajo |

---

## 📝 COMANDOS ÚTILES

### **Build y Test**
```bash
# Build debug
cargo build -p ry-rs --bin rydit-rs

# Build release
cargo build -p ry-rs --bin rydit-rs --release

# Tests workspace
cargo test --workspace

# Check workspace
cargo check --workspace
```

### **Git y Tags**
```bash
# Ver últimos commits
git log --oneline -10

# Ver tags
git tag -l

# Commit + tag
git add -A && git commit -m "mensaje" && git tag -a v0.14.0
```

---

## 🎯 LECCIONES APRENDIDAS

### **✅ LO QUE SÍ FUNCIONÓ**

1. **ry-backend dual**: Raylib drawing + SDL2 TTF profesional
2. **migui conectado a ry-backend**: Texto real, mouse completo
3. **ry-system-ry unificado**: RySystem con core + gui
4. **ry-config zero deps**: Config parser sin dependencias
5. **events-ry 3 capas**: InputEvent → TextInput → Shell
6. **demo_torreta_vs_sprites**: Juego completo funcionando
7. **Debug tests antes de fixear** - Identificar tipos exactos
8. **Agente para inspección** - Análisis profundo de errores
9. **Fix manual (NO sed)** - Control total de cambios
10. **Commits frecuentes** - Puntos de reversión claros
11. **Tags descriptivos** - Cada fix importante tiene tag
12. **Cargo clippy --fix** - Identifica warnings ocultos
13. **GPU Instancing + FSR 1.0**: 50K partículas + upscale funcionales en Adreno 610
14. **Health Bars world-space**: toolkit-ry/world_hud.rs con color dinámico
15. **Cámara 2D completa**: Zoom + rotación + follow suave + límites
16. **12 crates publicados**: ry-config, ry-physics, ry-science en esta sesión
17. **ry3d-gfx mejorado**: Modelo3D load GLTF/OBJ/IQM/VOX

### **❌ LO QUE NO FUNCIONÓ**

1. **sed automático** - Rompió código, tuvo que revertir
2. **Arc<str> option** - Evaluada pero rechazada (muy riesgosa)
3. **Lifetime elision** - Compilador requirió explícitos
4. **Crear demos duplicados** - demo_ttf_sprites_real falló linker
5. **Dar vueltas en círculos** - Analizar lo que ya funciona primero
6. **Features sin feature flags** - ry-rs necesitaba `migui` en ry-gfx

---

<div align="center">

**🛡️ RyDit v0.16.0 - Health Bars + HUD + Cámara 2D + ry3d-gfx + 12 crates publicados**

*23 crates | 12 publicados | 0 errores | Health Bars world-space | Cámara 2D con zoom/rotación | ry3d-gfx mejorado*

**Próximo: v0.17.0 - 3D en PC, iluminación, materiales**

**REGLA DE ORO: NUNCA SED DESPUÉS DE REFACTORIZAR PARSER**

**LECCIÓN v0.16.0: HUD world-space requiere transformación correcta + feature flags en Cargo.toml**

</div>
