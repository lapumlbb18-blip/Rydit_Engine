# Ry-Dit - Estructura del Proyecto v0.19.2

**Última actualización**: 2026-04-13
**Versión**: v0.19.2 ✅ Rybot Subsystems + SDL2 Helpers + War Spacio + Iluminación 2D + Flexbox
**Crates**: 25 | **Publicados**: 12 | **Demos**: 24+ | **Tests**: ~260

---

## 📁 Estructura del Workspace

```
shield-project/
├── Cargo.toml                    # Workspace definition
├── README.md                     # Documentación principal
├── ROADMAP.md                    # Plan de versiones
├── TASKS.md                      # Tareas completadas y pendientes
├── BITACORA.md                   # Bitácora técnica
├── ESTRUCTURA.md                 # Este archivo
├── MANIFIESTO.md                 # Filosofía Low-End First
├── CONTRIBUTING.md               # Guía de contribución
├── LICENSE                       # MIT License
│
├── docs/                         # 🆕 Ignorado en git
│   └── GUIA_USUARIO.md           # Guía para el usuario final
│
├── launchers/                    # 🆕 Ignorado en git
│   ├── launcher_militar.sh       # 🆕 Demo Militar
│   ├── launcher_emoji_utf8.sh    # 🆕 Demo Emoji Atlas UTF-8
│   ├── launcher_audio_mixer.sh   # 🆕 Demo Audio Mixer
│   ├── launcher_anime_v2.sh      # Snake Anime v2
│   ├── launcher_buscaminas.sh    # Buscaminas
│   ├── launcher_hud_camera.sh    # HUD + Cámara
│   ├── launcher_gpu_instancing.sh# GPU Instancing
│   ├── launcher_fsr.sh           # FSR 1.0
│   ├── launcher_torreta.sh       # Torreta vs Sprites
│   └── launcher_sdl2.sh          # SDL2 base
│
├───crates/
│   ├───ry-editor/      (🆕 v0.22.0: Entorno de Desarrollo Consolidado)
│   ├── ry-core/                  # ✅ Core trait + registry (crates.io)
│   ├── ry-lexer/                 # Zero-copy lexer
│   ├── ry-parser/                # AST parser
│   ├── ry-vm/                    # VM opcodes
│   ├── ry-gfx/                   # ✅ GPU Instancing + FSR (crates.io)
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── camera.rs         # Camera2D con zoom+rotación
│   │       ├── gpu_instancing.rs # GPU instancing + smoothstep AA
│   │       ├── fsr.rs            # FSR 1.0 upscaling
│   │       ├── render_queue.rs   # Command queue + double buffering
│   │       ├── backend_sdl2.rs   # Sdl2Backend
│   │       └── shaders/          # vertex.glsl, fragment.glsl, fsr_*.glsl
│   ├── ry-physics/               # ✅ Projectile + N-body (crates.io)
│   ├── ry-anim/                  # ✅ 12 Disney + action_sprite (crates.io)
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── disney.rs         # 12 principios Disney
│   │       ├── illusions.rs      # 6 ilusiones ópticas
│   │       ├── effects.rs        # Bloom, glow, blur, morph
│   │       ├── science_anim.rs   # 8 animaciones científicas
│   │       ├── action_assets.rs  # 6 funciones sprite animation (math)
│   │       ├── action_sprite.rs  # SpriteSheet, AnimationClip, AnimatedSprite
│   │       └── particles.rs      # Sistema de partículas
│   ├── ry-science/               # ✅ Bezier + stats + illusions (crates.io)
│   ├── ry-config/                # ✅ Config parser zero-deps (crates.io)
│   ├── ry-rs/                    # Main binary
│   │   └── src/
│   │       ├── main.rs           # Entry point
│   │       ├── lib.rs            # Stub lib
│   │       ├── modules/          # 16 módulos internos
│   │       │   ├── entity.rs     # Entity system (2859 líneas)
│   │       │   ├── tilemap.rs    # Tilemap v2.0 con texturas + CSV
│   │       │   ├── physics.rs    # Módulo physics interno
│   │       │   └── ...           # 13 módulos más
│   │       ├── eval/             # Scripting evaluator
│   │       └── bin/              # 24+ demos
│   │           ├── demo_war_spacio.rs     # 🆕 Galaga + Gravitación + SDL2 Helpers
│   │           ├── demo_militar.rs        # Soldado + partículas + granadas
│   │           ├── demo_emoji_utf8.rs     # 25+ emojis + UTF-8 fix
│   │           ├── demo_audio_mixer.rs    # 4 buses + spatial 2D + fade
│   │           ├── demo_anime_ry_v2.rs    # Snake Anime v2
│   │           ├── demo_buscaminas.rs     # Buscaminas 16×16
│   │           ├── demo_action_sprite.rs  # Sprite animation
│   │           ├── demo_hud_camera.rs     # HUD + Cámara 2D
│   │           ├── demo_gpu_instancing.rs # 50K partículas
│   │           ├── demo_fsr.rs            # FSR 1.0
│   │           ├── demo_torreta_vs_sprites.rs # Juego completo
│   │           └── ...                    # 12+ demos más
│   ├── ry-stream/                # ✅ LAN streaming (crates.io)
│   ├── ry-god/                   # ✅ Security (crates.io)
│   ├── ry-backend/               # ✅ Dual backend (crates.io)
│   ├── toolkit-ry/               # ✅ UI toolkit + 5 themes (crates.io)
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── theme.rs          # 5 temas predefinidos
│   │       ├── widgets.rs        # 18+ widgets HUD
│   │       └── world_hud.rs      # EntityHUD, DebugInfo, StatsHUD, Minimap
│   ├── migui/                    # ✅ Immediate mode GUI (crates.io)
│   ├── blast-core/               # Minimal executor
│   ├── lizer/                    # Legacy + AST cache
│   ├── v-shield/                 # ✅ Platform layer + sync (crates.io)
│   ├── ry3d-gfx/                 # 3D graphics + texto 3D + modelos
│   ├── events-ry/                # Input unificado
│   ├── ry-loader/                # Module loader
│   ├── ry-script/                # Script loading
│   ├── ry-input/                 # ✅ Input map configurable + macros K! M! P! PA!
│   ├── rybot/                    # ✅ Motor central + SceneTree + CLI/GUI + 6 subsistemas conectados
│   │   └── src/
│   │       ├── lib.rs            # RybotEngine, EngineState, EngineStats
│   │       ├── subsystems.rs     # Input/Physics/Animation/Science/Render/Network
│   │       ├── scene_tree.rs     # SceneTree, SceneNode, NodeType, parse .ryscene
│   │       ├── templates.rs      # ProjectTemplate, create_project, list_templates
│   │       ├── cli.rs            # CLI: new, templates, info
│   │       └── gui.rs            # GUI con migui (4 paneles)
│   └── ry-gfx/
│       └── src/
│           └── sdl2_helpers.rs   # 🆕 SDL2 helpers: velocity_color, blend_additive, newtonian_gravity, audio_procedural
│
└── logo_icon_asst/               # Assets disponibles
    └── sprites/                  # Sprites actuales (tank, helicopter, crate, etc.)
```

---

## 📊 Crates por Estado

### Publicados en crates.io (12)
| Crate | Versión | Tests |
|-------|---------|-------|
| ry-god | 0.1.0 | — |
| ry-stream | 0.2.0 | 17 |
| v-shield | 0.2.0 | 26 |
| ry-backend | 0.1.0 | — |
| migui | 0.4.1 | — |
| ry-gfx | 0.10.8 | — |
| ry-core | 0.8.2 | 9 |
| ry-anim | 0.12.0 | 65 |
| toolkit-ry | 0.1.0 | 14 |
| ry-config | 0.1.0 | 3 |
| ry-physics | 0.7.34 | 6 |
| ry-science | 0.7.34 | 21 |

### Sin publicar aún (6 con README pendiente)
| Crate | README | Tests |
|-------|--------|-------|
| ry-lexer | ❌ | — |
| ry-parser | ❌ | — |
| events-ry | ❌ | — |
| ry-loader | ❌ | — |
| blast-core | ❌ | — |
| ry3d-gfx | ✅ | 3 |

### Sin publicar (nuevos)
| Crate | README | Tests |
|-------|--------|-------|
| ry-input | ❌ | 18 |
| rybot | ❌ | 33 |

### Eliminados
| Crate | Razón |
|-------|-------|
| ry-test | Código muerto (0 tests, 0 código) |
| ry-ecs | Eliminado v0.13.1 |

---

## 🎮 Demos por Categoría

### Juegos Completos
| Demo | Features |
|------|----------|
| demo_war_spacio | 🆕 Galaga + Gravitación Newtoniana + SDL2 Helpers + Color velocidad |
| demo_militar | Soldado + partículas + granadas arco + salto |
| demo_anime_ry_v2 | Snake + manzanas + bombas + entidades + minimap |
| demo_buscaminas | 16×16 grid + 40 minas + flood fill |
| demo_torreta_vs_sprites | 3 niveles + cámara + AI + audio |

### Tecnología GPU
| Demo | Features |
|------|----------|
| demo_gpu_instancing | 50K partículas + smoothstep AA |
| demo_fsr | FSR 1.0 upscale 960→1280 |
| demo_hud_camera | HUD + cámara 2D + health bars |

### Tecnología Engine
| Demo | Features |
|------|----------|
| demo_emoji_utf8 | 🆕 25+ emojis como sprites + UTF-8 fix |
| demo_audio_mixer | 🆕 4 buses + spatial 2D + fade |
| demo_action_sprite | Sprite sheet + state machine |
| demo_rigidbody | Física + colisiones |
| demo_panel_visual | 4 paneles + consola |
| demo_menu_bar | Dear ImGui menus |
| demo_anime_ry | ry-anim showcase |

### Diagnósticos
| Demo | Uso |
|------|-----|
| gpu_debug | 9 partículas debug |
| gpu_solid | Quads sólidos |
| gpu_triangle | Triángulo NDC |
| gpu_circle_test | Círculos raylib |

---

<div align="center">

**Ry-Dit v0.19.2 — Estructura del Proyecto**

*25 crates · 12 publicados · ~260 tests · 24+ demos · 0 errores*

*Rybot subsystems conectados: Input + Physics + Animation + Science + Render + Network*

</div>
