# Ry-Dit - ESTRUCTURA DEL PROYECTO v0.13.0

**Última actualización**: 2026-04-05
**Versión**: v0.13.0 events-ry + Panel Visual + RyBot IPC + LAZOS
**Commit**: `118ee6a` (HEAD)
**Estado**: `cargo check --workspace`: 0 errores | 23 crates compilando | 95 tests pasando

---

## ARQUITECTURA ACTUAL

```
shield-project/
├── Cargo.toml                  # Workspace (23 crates)
├── README.md                   # Documentación principal (v0.13.0)
├── ROADMAP.md                  # Planificación v0.13→v1.0
├── QWEN.md                     # Bitácora técnica
├── ESTRUCTURA.md               # Este archivo
├── TASKS.md                    # 🆕 Tareas principales y paralelas
├── CONTRIBUTING.md             # Guía contribuidores
├── MANIFIESTO.md               # Filosofía del proyecto
├── LICENSE                     # MIT
├── .gitignore
│
├── crates/
│   ├── ry-core/                # 0.8.2  Core traits, module system, Valor
│   ├── ry-lexer/               # 0.1.0  Zero-copy lexer
│   ├── ry-parser/              # 0.1.0  Parser AST + error recovery
│   ├── ry-vm/                  #        VM opcodes + compiler
│   ├── ry-gfx/                 # 0.10.7 Graphics (raylib + SDL2 + OpenGL FFI)
│   ├── ry-physics/             # 0.7.34 2D projectile + N-body + nbody_simulate
│   ├── ry-anim/                # 0.12.0  Easing + Disney + ilusiones + ciencia + action
│   ├── ry-science/             #        Geometry 2D + stats + Bezier
│   ├── ry-script/              # 0.8.2  Script loading
│   ├── ry-stream/              # 0.1.0  LAN streaming (WebSocket) ✅ crates.io
│   ├── ry-god/                 # 0.1.0  Security & efficiency ✅ crates.io
│   ├── ry-loader/              #        Module loader
│   ├── ry-rs/                  #        Main binary + demos + eval + modules
│   ├── ry-system-ry/           # 0.11.0 Universal system (SDL2)
│   ├── ry-test/                #        Test utilities
│   ├── ry3d-gfx/               # 0.1.0  Graphics 3D (cube, sphere, cylinder...)
│   ├── toolkit-ry/             # 0.1.0  UI toolkit (5 temas + 20+ widgets)
│   ├── migui/                  #        Immediate mode GUI (12 widgets)
│   ├── blast-core/             # 0.1.0  Minimal value executor
│   ├── lizer/                  # 0.11.2 Legacy lexer wrapper (con AST cache)
│   ├── events-ry/              # 🆕 0.1.0  Input unificado + TextInput + Shell
│   └── v-shield/               #        Platform layer (pendiente)
│   └── ~~ry-ecs~~/             #        🗑️ Eliminado (-1,143 líneas)
│
├── crates/ry-rs/src/
│   ├── main.rs                 # Entry point + eval modo gráfico + modules
│   ├── eval/
│   │   └── mod.rs              # Evaluar expresiones (~4100 líneas)
│   │                           #   - Math: sin, cos, tan, sqrt, pow, log, exp...
│   │                           #   - Arrays: push, pop, len, slice, insert...
│   │                           #   - Cálculo: derivada, integral (Simpson)
│   │                           #   - Strings, regex, CSV, JSON, random...
│   ├── module.rs               # Registro de módulos (math, arrays, strings...)
│   ├── json_helpers.rs         # Conversión Serde <-> Valor
│   ├── rybot/                  # Asistente de código
│   └── modules/                # Módulos del lenguaje
│       ├── assets.rs           # Carga/dibujo de sprites PNG
│       ├── audio.rs            # SDL2_mixer (tonos, WAV)
│       ├── camera.rs           # Camera2D (posición, zoom, rotación)
│       ├── collision.rs        # Colisiones 2D (AABB, raycast)
│       ├── csv.rs              # CSV parser + queries
│       ├── entity.rs           # Sistema de entidades
│       ├── input_map.rs        # Input SDL2 mapeado
│       ├── level.rs            # Gestión de niveles/checkpoints
│       ├── physics.rs          # Física 2D (gravedad, proyectiles)
│       ├── tilemap.rs          # Tilemap system
│       └── window.rs           # Creación de ventana SDL2
│
├── crates/ry-rs/src/bin/       # ~31 bins
│   ├── demo_50k_particulas.rs
│   ├── demo_action_assets.rs
│   ├── demo_anime_ry           # ← ELF 341K release
│   ├── demo_carga_sprites.rs
│   ├── demo_carga_sprites_v1.rs
│   ├── demo_colisiones.rs
│   ├── demo_completo_final.rs
│   ├── demo_completo_sdl2.rs
│   ├── demo_effects.rs
│   ├── demo_fsr_audio.rs
│   ├── demo_illusions.rs
│   ├── demo_particles.rs
│   ├── demo_platformer_completo.rs
│   ├── demo_rigidbody          # ← ELF 446K release
│   ├── demo_science.rs
│   ├── demo_sprites_final.rs
│   ├── demo_sprites_v2.rs
│   ├── demo_stream.rs
│   ├── demo_ttf_sprites.rs
│   ├── demo_ttf_sprites_audio.rs
│   ├── nivel3_test_audio_lowend.rs
│   ├── nivel3_test_input_lowend.rs
│   ├── nivel3_test_lowend.rs
│   ├── rybot_cli.rs
│   ├── snake.rs
│   ├── test_audio_minimal.rs
│   ├── test_audio_sdl2.rs
│   ├── test_bloques_anidados.rs
│   ├── test_callback_sdl2.rs
│   ├── test_parser.rs
│   └── test_rydit_simple.rs
│
├── crates/ry-parser/src/
│   ├── lib.rs                  # API pública
│   ├── ast.rs                  # Expr<'a>, Stmt<'a>, BinaryOp, UnaryOp
│   ├── parser.rs               # Parser completo (~1500 líneas)
│   └── token.rs                # TokenKind (60+ tipos)
│
├── crates/ry-lexer/src/
│   ├── lib.rs                  # API pública
│   ├── lexer.rs                # Zero-copy Lexer
│   └── token.rs                # Token<'a> zero-copy
│
├── crates/ry-gfx/src/
│   ├── lib.rs                  # Graphics layer (~1700 líneas)
│   ├── camera.rs               # Camera2D
│   ├── gpu_instancing.rs       # OpenGL FFI + instancing
│   ├── render_queue.rs         # Cola de renderizado
│   ├── ecs_render.rs           # ECS renderer
│   ├── fsr.rs                  # FSR 1.0 upscaling
│   └── shaders/                # GLSL shaders embebidos
│
├── crates/ry-anim/src/         # v0.12.0 - 41 funciones
│   ├── lib.rs                  # API pública
│   ├── easing.rs               # ease_in, ease_out, ease_in_out
│   ├── disney.rs               # 9 principios Disney
│   ├── illusions.rs            # 6 ilusiones ópticas
│   ├── effects.rs              # 6 efectos especiales
│   ├── science.rs              # 8 animaciones científicas
│   └── action_assets.rs       # 6 action assets (sprite animation)
│
├── crates/toolkit-ry/src/
│   ├── lib.rs                  # UI Toolkit API pública
│   ├── themes/
│   │   ├── dark.rs
│   │   ├── light.rs
│   │   ├── retro.rs
│   │   ├── neon.rs
│   │   └── minimal.rs
│   └── widgets/
│       ├── health_bar.rs
│       ├── mana_bar.rs
│       ├── xp_bar.rs
│       ├── score.rs
│       ├── menu.rs
│       ├── inventario.rs
│       ├── dialogo.rs
│       ├── minimap.rs
│       ├── loading.rs
│       └── notificaciones.rs
│
├── crates/ry3d-gfx/src/
│   ├── lib.rs                  # 3D Graphics API
│   ├── primitives/
│   │   ├── cube.rs
│   │   ├── sphere.rs
│   │   ├── cylinder.rs
│   │   └── plane.rs
│   ├── debug/
│   │   ├── grid.rs
│   │   ├── axes_gizmo.rs
│   │   └── bounding_box.rs
│   └── shapes/
│       ├── point3D.rs
│       ├── line3D.rs
│       └── triangle3D.rs
│
├── crates/ry-physics/src/
│   ├── lib.rs                  # Physics API
│   ├── projectile.rs           # 2D projectile
│   ├── nbody.rs               # N-body (2 cuerpos)
│   └── nbody_simulate.rs       # Movido desde ry-ecs
│
├── crates/ry-stream/src/       # ✅ crates.io
│   ├── lib.rs                  # Streaming API
│   └── websocket.rs            # WebSocket LAN streaming
│
├── crates/ry-god/src/          # ✅ crates.io
│   ├── lib.rs                  # Security & Efficiency
│   └── framework.rs            # Security framework
│
├── docs/
│   ├── panorama_v0.13.0.md
│   ├── plan_limpieza_v0.13.0.md
│   ├── vision_estrategica.md
│   ├── vision_ciencia_ry_science_physics.md
│   ├── vision_ry_stream_comunidad.md
│   ├── vision_ry_anim.md
│   ├── analisis_sistema_universal_ry.md
│   ├── analisis_raylib_2d_3d.md
│   ├── analisis_display_input_render.md
│   ├── analisis_ry_ecs.md
│   ├── sesion_control_total_v0.13.0.md
│   ├── guia_compilacion_termux.md
│   └── arquitectura_demos.md
│
├── screenshots/                # Capturas y videos MP4
├── tests/                      # Tests automáticos
└── tests_rydit/                # Tests del lenguaje
```

---

## FUNCIONES POR MÓDULO

### math:: / matematica::
| Función | Args | Retorna |
|---------|------|---------|
| `sin, cos, tan` | 1 | f64 |
| `sqrt` | 1 | f64 |
| `pow` | 2 | f64 |
| `log, log10` | 1 | f64 |
| `exp` | 1 | f64 |
| `abs` | 1 | f64 |
| `floor, ceil, round, trunc, fract` | 1 | f64 |
| `min, max` | 2 | f64 |
| `clamp` | 3 | f64 |
| `lerp` | 3 | f64 |
| `sign` | 1 | f64 |
| `mod` | 2 | f64 |
| `hypot` | 2 | f64 |
| `cbrt` | 1 | f64 |
| `atan2` | 2 | f64 |
| `deg2rad, rad2deg` | 1 | f64 |
| **Constantes**: `PI`, `E`, `TAU`, `INF` | 0 | f64 |

### calc::
| Función | Args | Retorna |
|---------|------|---------|
| `derivada(f, x, h)` | 2-3 | f64 |
| `derivada2(f, x, h)` | 2-3 | f64 |
| `integral(f, a, b, n)` | 4 | f64 |
| `integral_trapezio(f, a, b, n)` | 4 | f64 |

### arrays::
| Función | Args | Retorna |
|---------|------|---------|
| `push(arr, elem)` | 2 | array |
| `pop(arr)` | 1 | elem |
| `shift(arr)` | 1 | elem |
| `unshift(arr, elem)` | 2 | array |
| `slice(arr, start, end)` | 3 | array |
| `reverse(arr)` | 1 | array |
| `len(arr)` | 1 | num |
| `insert(arr, idx, elem)` | 3 | array |
| `remove(arr, idx)` | 2 | elem |
| `contains(arr, elem)` | 2 | bool |
| `find(arr, elem)` | 2 | num |
| `join(arr, sep)` | 2 | texto |
| `clear(arr)` | 1 | array |
| `first(arr)` | 1 | elem |
| `last(arr)` | 1 | elem |

### vec2::
| Función | Args | Retorna |
|---------|------|---------|
| `new(x, y)` | 2 | Vec2 |
| `add(a, b)` | 2 | Vec2 |
| `sub(a, b)` | 2 | Vec2 |
| `scale(v, s)` | 2 | Vec2 |
| `magnitude(v)` | 1 | f64 |
| `normalize(v)` | 1 | Vec2 |
| `dot(a, b)` | 2 | f64 |
| `cross(a, b)` | 2 | f64 |
| `angle(v)` | 1 | f64 |
| `rotate(v, angle)` | 2 | Vec2 |
| `lerp(a, b, t)` | 3 | Vec2 |
| `dist(a, b)` | 2 | f64 |
| `negate(v)` | 1 | Vec2 |
| `midpoint(a, b)` | 2 | Vec2 |
| `from_angle(angle)` | 1 | Vec2 |
| **Constantes**: `zero`, `one`, `up`, `down`, `left`, `right` | 0 | Vec2 |

### quest::
| Función | Args | Retorna |
|---------|------|---------|
| `create(id, name, desc)` | 3 | Quest |
| `add_objective(quest, desc)` | 2 | Quest |
| `complete_objective(quest, idx)` | 2 | Quest |
| `set_reward(quest, type, amount)` | 3 | Quest |
| `check_completion(quest)` | 1 | bool |
| `get_state(quest_id)` | 1 | Valor |

### save_load::
| Función | Args | Retorna |
|---------|------|---------|
| `create(slot, name)` | 2 | SaveSlot |
| `set_var(slot, key, value)` | 3 | bool |
| `get_var(slot, key)` | 2 | Valor |
| `save(slot)` | 1 | bool |
| `load(slot)` | 1 | SaveSlot |
| `list()` | 0 | Array |

---

## PIPELINE DE EJECUCIÓN

```
Código .rydit
    |
    v
┌─────────────┐
│  ry-lexer   │  Zero-copy scan -> Token<'a>
└──────┬──────┘
       |
       v
┌─────────────┐
│ ry-parser   │  Error recovery -> AST (Expr<'a>, Stmt<'a>)
└──────┬──────┘
       |
       v
┌─────────────┐
│evaluar_expr │  Evaluar expresiones (eval/mod.rs)
│ejecutar_stmt│ Ejecutar statements (main.rs)
└──────┬──────┘
       |
       v
   Valor (Num, Texto, Bool, Array, Vec2, Quest, SaveSlot)
       |
       v
┌─────────────┐
│   ry-gfx    │  SDL2/raylib render + FSR 1.0
│   ry3d-gfx  │  3D primitives
│  toolkit-ry │  UI widgets (5 temas)
│  ry-anim    │  Animaciones (41 funciones)
│ ry-physics  │  Física 2D + N-body
└─────────────┘
       |
       v
┌─────────────┐
│  ry-stream  │  LAN streaming (WebSocket)
│   ry-god    │  Security & Efficiency
└─────────────┘
```

---

## CRATES PUBLICABLES

| Crate | Versión | Estado | Notas |
|-------|---------|--------|-------|
| ry-god | 0.1.0 | ✅ crates.io | Security & efficiency |
| ry-stream | 0.1.0 | ✅ crates.io | LAN streaming |
| ry-core | 0.8.2 | Listo | Core traits |
| ry-lexer | 0.1.0 | Listo | Zero-copy |
| ry-parser | 0.1.0 | Listo | Error recovery |
| ry-anim | 0.12.0 | Listo | 41 funciones, 58 tests |
| ry-physics | 0.7.34 | Listo | 2D projectile + N-body |
| ry-gfx | 0.10.7 | Listo | Graphics FFI |
| ry3d-gfx | 0.1.0 | Listo | 3D primitives |
| toolkit-ry | 0.1.0 | Listo | UI toolkit (5 temas) |
| lizer | 0.11.2 | Listo | Legacy |
| ry-system-ry | 0.11.0 | ⚠️ | Falta license |

---

## DEMOS BINARIOS

| Demo | Descripción | Tamaño Release |
|------|-------------|----------------|
| demo_anime_ry | Showcase ry-anim v0.12.0 | 341K |
| demo_rigidbody | Física + colisiones SDL2 | 446K |
| demo_action_assets | Action assets + sprite anim | — |
| demo_illusions | Ilusiones ópticas | — |
| demo_effects | Efectos especiales | — |
| demo_science | Animaciones científicas | — |
| demo_50k_particulas | 50K partículas | — |
| demo_stream | ry-stream demo | — |
| demo_platformer_completo | Platformer completo | — |
| snake | Snake game | — |

---

<div align="center">

**Ry-Dit v0.13.0 -- ESTRUCTURA ACTUALIZADA**

*23 crates | ~27K+ líneas Rust | events-ry v0.1.0 | 95 tests | 2 crates publicados*

*Última actualización: 2026-04-05*

</div>
