# 🛡️ QWEN.md - Bitácora Técnica Ry-Dit

**Última actualización**: 2026-04-11
**Versión actual**: v0.18.0 ✅ 3D Primitives + Transiciones + Audio Mixer + UTF-8 Fix + Emojis
**Versión anterior**: v0.17.0 (Demo Militar + Emoji Atlas + Audio Mixer)
**Próxima versión**: v0.19.0 — Mesh Generation + Materiales + Iluminación 3D
**Commit**: `v0.18.0`
**Repositorio**: `https://github.com/lapumlbb18-blip/Ry-dit`
**Crates publicados**: 12 ✅

---

## 🎉 SESIÓN v0.18.0 — ÉPICA: 3D + TRANSICIONES + AUDIO MIXER

### ✅ ESTADO: 23 CRATES | 0 ERRORES | 147 TESTS | 20+ DEMOS | 12 CRATES.IO

| Demo | GPU | FPS | Features | Notas |
|------|-----|-----|----------|-------|
| **demo_3d_primitives** | raylib (Adreno 610) | ~60 FPS | Cubos, esferas, cilindros, grid, ejes 3D | Cámara orbital |
| **demo_transitions** | zink (Adreno 610) | ~60 FPS | 19 transiciones: fade, slide, wipe, zoom, spiral | Auto-advance |
| **demo_militar** | zink (Adreno 610) | ~30 FPS | Soldado procedural + partículas + granadas arco + salto | WASD, física, gravedad |
| **demo_emoji_utf8** | zink (Adreno 610) | ~28 FPS | 25+ emojis como sprites procedurales PNG | UTF-8 fix, atlas de texturas |
| **demo_audio_mixer** | zink (Adreno 610) | ~30 FPS | 4 buses + spatial 2D + fade in/out | Master, SFX, Music, Ambient |
| **demo_anime_ry_v2** | zink (Adreno 610) | ~32 FPS | Snake + manzanas + bombas + entidades + minimap | WASD, flood fill, camera follow |
| **demo_buscaminas** | zink (Adreno 610) | ~29 FPS | 16×16, 40 minas, flood fill, banderas | Click izq/der, R: reiniciar |
| **demo_action_sprite** | zink (Adreno 610) | ~30 FPS | Sprite sheet + frame animation + state machine | Procedural sprites |
| **demo_hud_camera** | zink (Adreno 610) | ~30 FPS | Health bars + debug overlay + stats + minimap | Camera2D zoom+rotación |
| **demo_gpu_instancing** | zink (Adreno 610) | ~53 FPS | 50K partículas instanced | smoothstep AA ahora |
| **demo_fsr** | zink (Adreno 610) | ~48 FPS | FSR 1.0 upscale 960→1280 | FBO render-to-texture |

### Features Implementadas

| # | Feature | Detalle |
|---|---------|---------|
| 1 | **demo_militar** | Soldado procedural con primitivas (cabeza, cuerpo, arma) |
| 2 | **Granadas en arco** | Trayectoria parabólica con gravedad |
| 3 | **Sistema de partículas** | Efectos de disparo y explosión |
| 4 | **Salto del soldado** | Física de salto con input de teclado |
| 5 | **Emoji Atlas UTF-8** | TTF_RenderUTF8_Blended fix |
| 6 | **25+ emojis procedurales** | Sprites PNG generados en runtime |
| 7 | **Atlas de texturas** | Textura única con grid de emojis |
| 8 | **Audio Mixer** | 4 buses: Master, SFX, Music, Ambient |
| 9 | **Spatial 2D** | Volumen/panning según posición del oyente |
| 10 | **Fade in/out** | Transiciones suaves de volumen |
| 11 | **docs/ ignorado** | docs/ y launchers/ en .gitignore |
| 12 | **Organización git** | Archivos de desarrollo fuera del tracking |

### Bugs Fixeados

| Bug | Fix |
|-----|-----|
| TTF_RenderText no soportaba UTF-8 | Cambiar a TTF_RenderUTF8_Blended |
| Emojis no se renderizaban | Atlas procedural como sprites PNG |
| Audio sin buses | Implementar 4 buses con mixer |
| Sin spatial audio | Implementar volumen/panning 2D |

---

## 🎉 SESIÓN v0.16.1 — ÉPICA: SNAKE + BUSCAMINAS + ACTION SPRITE + TILEMAP

### ✅ ESTADO: 23 CRATES | 0 ERRORES | 144 TESTS | 15+ DEMOS | 12 CRATES.IO

| Demo | GPU | FPS | Features | Notas |
|------|-----|-----|----------|-------|
| **demo_anime_ry_v2** | zink (Adreno 610) | ~32 FPS | Snake + manzanas + bombas + entidades + minimap | WASD, flood fill, camera follow |
| **demo_buscaminas** | zink (Adreno 610) | ~29 FPS | 16×16, 40 minas, flood fill, banderas | Click izq/der, R: reiniciar |
| **demo_action_sprite** | zink (Adreno 610) | ~30 FPS | Sprite sheet + frame animation + state machine | Procedural sprites |
| **demo_hud_camera** | zink (Adreno 610) | ~30 FPS | Health bars + debug overlay + stats + minimap | Camera2D zoom+rotación |
| **demo_gpu_instancing** | zink (Adreno 610) | ~53 FPS | 50K partículas instanced | smoothstep AA ahora |
| **demo_fsr** | zink (Adreno 610) | ~48 FPS | FSR 1.0 upscale 960→1280 | FBO render-to-texture |

### Features Implementadas

| # | Feature | Detalle |
|---|---------|---------|
| 1 | **ry3d-gfx v0.1.0** | 15 primitivas 3D + draw_model/ex + DrawHandle3D RAII |
| 2 | **demo_3d_primitives** | Escena 3D interactiva con cámara orbital |
| 3 | **19 transiciones** | Fade, Slide, Wipe, Zoom, Circle, Blinds, Dissolve, Spiral, etc. |
| 4 | **TransitionManager** | Cola de transiciones + auto-advance + easing smoothstep |
| 5 | **Audio Mixer** | 4 buses + spatial 2D + fade in/out + volúmenes independientes |
| 6 | **UTF-8 Fix** | TTF_RenderUTF8_Blended → acentos (áéíóú ñ ü) correctos |
| 7 | **Emoji Atlas** | 25+ emojis como sprites procedurales PNG |
| 8 | **FontSystem** | Múltiples fuentes + fallback automático |
| 9 | **docs/ + launchers/** | Organización del repo (gitignored) |
| 10 | **INFORME_RY3D_GFX.md** | Estado completo + roadmap del 3D |
| 11 | **sync_drive_bg.sh** | Sync a Google Drive con rclone |
| 12 | **ry3d-gfx como dep** | Agregado a ry-rs/Cargo.toml |

### Bugs Fixeados

| Bug | Fix |
|-----|-----|
| draw_model/draw_model_ex stubs vacíos | Implementar FFI DrawModel/DrawModelEx real |
| draw_text_3d no-op | Stub (GetWorldToScreen no en FFI) |
| TTF_RenderText_Blended = Latin-1 | Cambiar a TTF_RenderUTF8_Blended |
| SDL2 Keycode nombres | Number1→KEY_ONE, CameraMode→FFI enum |
| DrawHandle3D sin camera ref | Agregar lifetime `'a` + guardar `&'a Camera3D` |
| DrawModelEx scale = f32 | Cambiar a Vector3 (x,y,z) |

### Patrón Correcto para 3D con ry3d-gfx

**El patrón correcto:**
```rust
// imports
use raylib::prelude::*;
use ry3d_gfx::DrawHandle3D;
use ry3d_gfx::touch_controls::TouchControls;
use ry_gfx::ColorRydit;

fn main() -> Result<(), String> {
    // 1. Iniciar raylib (ventana + contexto GL)
    let (mut rl, thread) = raylib::init()
        .size(900, 600)
        .title("Mi Demo 3D")
        .build();

    // 2. Crear cámara con Camera3D::perspective()
    let camera = Camera3D::perspective(
        Vector3::new(cam_x, cam_y, cam_z),
        target,
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    // 3. Game loop
    while !rl.window_should_close() {
        // Input (mouse, touch, teclado)
        let touching = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        let touch_pos = rl.get_mouse_position();

        // 4. Render
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // 5. Modo 3D con DrawHandle3D
        let mut h3d = DrawHandle3D::new(&camera);
        h3d.clear_3d(ColorRydit::Negro);
        h3d.draw_grid_3d(20, 1.0);
        h3d.draw_cube_3d((0.0, 1.0, 0.0), (2.0, 2.0, 2.0), ColorRydit::Rojo);
        // ... más primitivas 3D
        drop(h3d); // Fin modo 3D

        // 6. Touch controls overlay (2D encima de 3D)
        // touch.draw();

        // 7. HUD 2D
        d.draw_text("HUD", 10, 10, 20, Color::WHITE);
    }
    Ok(())
}
```

**Lecciones críticas:**
- NO usar `gl::Clear()` directamente — usar `h3d.clear_3d()`
- NO usar `d.begin_mode_3d()` — usar `DrawHandle3D::new(&camera)`
- NO importar `raylib::prelude::*` en ry3d-gfx (causa conflicto con re-exports)
- TouchControls se dibuja DESPUÉS de `drop(h3d)` (2D sobre 3D)
- Mouse button es `MOUSE_BUTTON_LEFT` no `MOUSE_LEFT_BUTTON`

**Errores comunes y soluciones:**
| Error | Causa | Solución |
|-------|-------|----------|
| `gl function was not loaded` | `gl::Clear()` sin contexto | Usar `h3d.clear_3d()` |
| `no method named end_mode_3d` | API incorrecta | `DrawHandle3D::new()` hace Begin/End automático |
| `Camera3D reimported` | Wildcard imports en ry3d-gfx | Importar tipos específicos |
| `MOUSE_LEFT_BUTTON not found` | Nombre incorrecto | Usar `MOUSE_BUTTON_LEFT` |

---

## 📦 Lista de Crates (23)

| Crate | Versión | Estado | Publicado | Notas |
|-------|---------|--------|-----------|-------|
| ry-core | 0.8.2 | ✅ | ✅ crates.io | Core trait + registry |
| ry-lexer | 0.1.0 | ✅ | ❌ | Zero-copy lexer |
| ry-parser | 0.1.0 | ✅ | ❌ | AST parser |
| ry-vm | — | ⚠️ | ❌ | VM opcodes |
| ry-gfx | 0.10.8 | ✅ | ✅ crates.io | GPU Instancing + FSR |
| ry-physics | 0.7.34 | ✅ | ✅ crates.io | Projectile + N-body |
| ry-anim | 0.12.0 | ✅ | ✅ crates.io | 12 Disney + action_sprite |
| ry-science | 0.7.34 | ✅ | ✅ crates.io | Bezier + stats + illusions |
| ry-script | 0.8.2 | ✅ | ❌ | Script loading |
| ry-stream | 0.2.0 | ✅ | ✅ crates.io | LAN streaming |
| ry-god | 0.1.0 | ✅ | ✅ crates.io | Security |
| ry-loader | — | ⚠️ | ❌ | Module loader |
| ry-rs | — | Main | N/A | Binary + lib |
| ry-test | — | 🗑️ | — | ELIMINADO |
| ry-backend | 0.1.0 | ✅ | ✅ crates.io | Dual backend |
| ry-config | 0.1.0 | ✅ | ✅ crates.io | Config parser |
| toolkit-ry | 0.1.0 | ✅ | ✅ crates.io | UI toolkit + 5 themes |
| migui | 0.4.1 | ✅ | ✅ crates.io | Immediate mode GUI |
| blast-core | 0.1.0 | ✅ | ❌ | Minimal executor |
| lizer | 0.11.2 | ✅ | ❌ | Legacy + AST cache |
| v-shield | 0.2.0 | ✅ | ✅ crates.io | Platform layer + sync |
| ry3d-gfx | 0.1.0 | ✅ | ❌ | 3D graphics |
| events-ry | 0.1.0 | ✅ | ❌ | Input unificado |

---

## 🔴 LECCIONES CRÍTICAS

### 1. ry-rs lib vs bin — NO REFACTORIZAR AHORA
**Problema**: main.rs ~5000 líneas con ciclo de dependencia circular
**Análisis**: El costo (10-14h) supera el beneficio (nadie usa ry_rs como lib)
**Decisión**: Dejar como está hasta que haya un consumidor real

### 2. Model3D necesita contexto raylib activo
**Problema**: `LoadModel` FFI requiere ventana inicializada
**Solución**: Cargar modelos dentro del game loop, no fuera
**Lección**: FFI de raylib necesita contexto de ventana

### 3. Texture no implementa Debug
**Problema**: `#[derive(Debug)]` falla en sdl2::render::Texture
**Solución**: Implementar `Debug` manualmente sin el campo texture

### 4. Borrow checker en snake game
**Problema**: `snake.head()` inmutable + mutación de `snake.alive`
**Solución**: Copiar posición primero (`let hp = snake.head_pos()`)

### 5. NUNCA usar sed para código estructural
- ✅ sed seguro SOLO para imports simples
- ❌ sed peligroso para código multilínea
- **Siempre usar el tool `edit`**

---

## 🚀 PRÓXIMOS PASOS (v0.19.0)

### **Lo que traerás en la próxima sesión:**
- [ ] Letras 3D en demos
- [ ] Sistema de escenas (.ryscene)
- [ ] Scene transitions con fade
- [ ] Input map configurable (.rydit-input)
- [ ] Panel visual mejorado con migui
- [ ] Rybot CLI + GUI
- [ ] Editor separado o 2-in-1
- [ ] LAZOS: Python + C++ + C
- [ ] GitHub Actions completo
- [ ] SAZ formato
- [ ] v1.0 de Ry-Dit

### **Prioridades técnicas:**
1. Letras 3D → texto real en ry3d-gfx
2. Escenas → .ryscene archivos legibles
3. Input map → rebind de teclas
4. migui → más widgets + temas
5. Rybot CLI → crear proyectos desde terminal

---

## 📝 COMANDOS ÚTILES

### Build y Test
```bash
# Build release
cargo build -p ry-rs --bin rydit-rs --release

# Tests workspace
cargo test --workspace

# Check workspace
cargo check --workspace
```

### Launchers
```bash
./launcher_militar.sh        # 🎖️ Militar
./launcher_emoji_utf8.sh     # 😀 Emoji Atlas
./launcher_audio_mixer.sh    # 🎵 Audio Mixer
./launcher_anime_v2.sh      # 🐍 Snake
./launcher_buscaminas.sh     # 💣 Buscaminas
./launcher_hud_camera.sh     # HUD + Cámara
./launcher_gpu_instancing.sh # 50K partículas
./launcher_fsr.sh            # FSR 1.0
./launcher_torreta.sh        # Torreta vs Sprites
cargo run --bin demo_3d_primitives --release   # 🧊 3D Primitives
cargo run --bin demo_transitions --release      # 🎬 19 Transiciones
```

---

<div align="center">

**🛡️ RyDit v0.18.0 — 3D + Transiciones + Audio Mixer + UTF-8 Fix**

*23 crates · 147 tests · 12 crates.io · 20+ demos · 0 errores*

**Próximo: v0.19.0 — Letras 3D + Escenas (.ryscene) + Input map + Rybot CLI+GUI**

**LECCIÓN v0.18.0: NO usar gl::Clear() directo | DrawHandle3D RAII | TouchControls después de drop(h3d) | MOUSE_BUTTON_LEFT no MOUSE_LEFT_BUTTON**

</div>
