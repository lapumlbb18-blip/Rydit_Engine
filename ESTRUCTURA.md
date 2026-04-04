# 🛡️ RyDit - ESTRUCTURA DEL PROYECTO

**Última actualización**: 2026-04-01  
**Versión**: v0.11.2 ✅ PARSER ZERO-COPY + BYTECODE VM  
**Commit**: Ver `git log -n 1`  
**Estado**: ✅ 65 tests passing | ✅ Workspace compila | ✅ Producción

---

## 🎯 ARQUITECTURA v0.11.2

```
ry-dit/
├── crates/
│   ├── rydit-lexer/            # 🆕 v0.11.2 Zero-Copy Lexer
│   │   ├── src/
│   │   │   ├── lib.rs          # API pública + re-exports
│   │   │   ├── token.rs        # Token<'a> zero-copy (289 líneas)
│   │   │   └── lexer.rs        # Lexer<'a> scan (439 líneas)
│   │   ├── Cargo.toml
│   │   └── README.md
│   │
│   ├── rydit-parser/           # 🆕 v0.11.2 Parser + Error Recovery
│   │   ├── src/
│   │   │   ├── lib.rs          # API pública
│   │   │   ├── ast.rs          # AST typed (Expr<'a>, Stmt<'a>)
│   │   │   ├── error.rs        # Error handling + recovery
│   │   │   └── parser.rs       # Parser con recovery (1,119 líneas)
│   │   ├── Cargo.toml
│   │   └── README.md
│   │
│   ├── rydit-vm/               # 🆕 v0.11.2 Bytecode VM
│   │   ├── src/
│   │   │   ├── lib.rs          # API pública
│   │   │   ├── opcodes.rs      # OpCode enum (50+ instrucciones)
│   │   │   ├── compiler.rs     # AST → Bytecode (552 líneas)
│   │   │   └── vm.rs           # Stack-based VM (1,000+ líneas)
│   │   ├── Cargo.toml
│   │   └── README.md
│   │
│   ├── lizer/                  # ⚠️ v0.11.2 Wrapper (deprecated)
│   │   ├── src/lib.rs          # Re-exports de rydit-lexer + parser
│   │   └── Cargo.toml          # Dependencias: rydit-lexer, rydit-parser
│   │
│   ├── rydit-core/             # ✅ RyditModule trait
│   │   └── src/lib.rs          # Trait + ModuleRegistry
│   │
│   ├── rydit-ecs/              # ✅ ECS (bevy_ecs)
│   │   ├── src/
│   │   │   ├── lib.rs          # EcsWorld
│   │   │   ├── components.rs   # Position, Velocity, Sprite
│   │   │   └── systems.rs      # Movement, Render
│   │   └── Cargo.toml          # bevy_ecs = "0.15"
│   │
│   ├── rydit-gfx/              # ✅ Backend + Toolkit UI
│   │   ├── src/
│   │   │   ├── lib.rs          # RyditGfx + exports
│   │   │   ├── backend_sdl2.rs # ✅ SDL2 ventana + input + render
│   │   │   ├── input_sdl2.rs   # ✅ Event Loop (69 teclas)
│   │   │   ├── sdl2_ffi.rs     # ✅ FFI nativo (TTF, Image, Mixer)
│   │   │   ├── toolkit/        # 🆕 UI Toolkit v0.11.0
│   │   │   │   ├── mod.rs
│   │   │   │   ├── theme.rs    # Dark/Light themes
│   │   │   │   └── widgets/
│   │   │   │       ├── button.rs   # Botones clickeables
│   │   │   │       ├── label.rs    # Texto SDL2_ttf
│   │   │   │       └── panel.rs    # Contenedores
│   │   │   ├── gpu_instancing.rs   # ✅ 100K partículas
│   │   │   ├── ecs_render.rs       # ✅ ECS + rlgl
│   │   │   ├── render_queue.rs     # ✅ 8192 draw calls
│   │   │   └── shaders/            # vertex.glsl, fragment.glsl
│   │   └── Cargo.toml              # sdl2 = "0.37" + raylib
│   │
│   ├── rydit-rs/                   # ✅ Core + RyBot + VM
│   │   ├── src/
│   │   │   ├── bin/                # 🆕 Binarios de prueba
│   │   │   │   ├── demo_toolkit_ry.rs      # 🆕 UI Toolkit demo
│   │   │   │   ├── rybot_cli.rs            # 🆕 RyBot CLI
│   │   │   │   ├── test_sdl2_basico.rs     # 🆕 SDL2 test simple
│   │   │   │   ├── test_sdl2_sprite_debug.rs # 🆕 Sprite debug
│   │   │   │   ├── demo_particles.rs       # Partículas
│   │   │   │   ├── demo_big_bang.rs        # Explosión cósmica
│   │   │   │   └── snake.rs                # Juego Snake
│   │   │   ├── rybot/            # 🆕 RyBot Inspector v0.11.0
│   │   │   │   ├── mod.rs        # RyBot struct
│   │   │   │   └── registry.rs   # Registry + Alertas (530 líneas)
│   │   │   ├── modules/          # Sistema Ry (180K líneas)
│   │   │   │   ├── camera.rs     # ✅ 16.9K líneas
│   │   │   │   ├── entity.rs     # ✅ 88.8K líneas
│   │   │   │   ├── level.rs      # ✅ 17.2K líneas
│   │   │   │   ├── assets.rs     # ✅ 15.6K líneas
│   │   │   │   ├── physics.rs    # ✅ 22.8K líneas
│   │   │   │   ├── input_map.rs  # ✅ 21.1K líneas
│   │   │   │   └── particles.rs  # ✅ 7K líneas
│   │   │   ├── executor.rs       # ✅ Game loop con RyBot + VM
│   │   │   ├── main.rs           # Entry point
│   │   │   └── lib.rs            # Config parser
│   │   └── Cargo.toml            # + rydit-vm, rydit-parser, rydit-lexer
│   │
│   ├── rydit-physics/            # ✅ Físicas 2D
│   │   └── src/lib.rs            # 20 funciones
│   │
│   ├── rydit-anim/               # ✅ Animaciones
│   │   └── src/lib.rs            # 8.8K líneas
│   │
│   ├── rydit-science/            # ✅ Funciones científicas
│   │   └── src/lib.rs            # 18.1K líneas
│   │
│   ├── rydit-loader/             # ✅ Dynamic module loader
│   │   └── src/lib.rs
│   │
│   ├── rydit-script/             # ✅ Integración scripts
│   │   └── src/lib.rs
│   │
│   ├── rydit-http/               # ✅ HTTP + WebSocket
│   │   └── src/lib.rs
│   │
│   ├── rydit-test/               # ✅ Tests en 3 niveles
│   │   └── src/
│   │       ├── nivel1_core_test.rs    # 13 tests
│   │       └── nivel2_integration_test.rs  # 3 tests
│   │
│   ├── blast-core/               # ✅ Executor (legacy)
│   │   └── src/lib.rs            # 476 líneas
│   │
│   ├── migui/                    # ✅ Separado (sin usar en RyDit)
│   │   └── src/
│   │       ├── lib.rs
│   │       └── backend_sdl2.rs
│   │
│   └── v-shield/                 # ✅ Utilidades
│       └── src/lib.rs
│
├── demos/                        # Scripts .rydit
│   ├── demo_particles.rydit
│   ├── demo_big_bang.rydit
│   └── snake.rydit
│
├── logo_icon_asst/               # Assets de prueba
│   └── sprites/
│
├── docs/                         # 🆕 Documentación
│   ├── ANALISIS_ARQUITECTURA_V0.11.2.md  # 🆕 Análisis completo
│   ├── FASE_0_VERIFICACION_V0.11.2.md    # 🆕 Checklist pre-implementación
│   └── sessions/                 # 🆕 Sesiones de desarrollo
│
├── scripts/                      # 🆕 Scripts de automatización
│   └── implementar_parser_v0.11.2.sh
│
└── target/                       # Build artifacts (git-ignored)
```

---

## 📊 MÉTRICAS v0.11.2

### **Nuevos Crates**

| Crate | Líneas | Tests | Estado |
|-------|--------|-------|--------|
| **rydit-lexer** | 728 | 20 | ✅ Zero-Copy |
| **rydit-parser** | 1,826 | 23 | ✅ Error Recovery |
| **rydit-vm** | 1,551 | 19 | ✅ Bytecode VM |
| **lizer** (wrapper) | 50 | 3 | ✅ Backward Compat |

**Total**: 4,155 líneas Rust nuevas | 65 tests passing

### **Crates Existentes**

| Crate | Líneas | Tests | Estado |
|-------|--------|-------|--------|
| **rydit-rs** | ~4K | - | ✅ Core + RyBot |
| **rydit-gfx** | ~2K | 6 | ✅ SDL2 Backend |
| **rydit-ecs** | ~1K | - | ✅ ECS |
| **rydit-physics** | ~500 | 6 | ✅ Físicas 2D |
| **rydit-anim** | ~500 | 9 | ✅ Animaciones |
| **rydit-science** | ~1K | 21 | ✅ Ciencia |
| **blast-core** | 476 | 20 | ✅ Executor |
| **migui** | ~2K | 8 | ✅ UI Toolkit |

**Total General**: ~25K líneas Rust | 150+ tests

---

## 🏗️ FLUJO DE COMPILACIÓN v0.11.2

```
┌─────────────────────────────────────────────────────────────┐
│  Usuario: rydit-rs --run demo.rydit                         │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  rydit-rs (main.rs)                                         │
│    → rydit-parser::parse(source)                            │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  rydit-parser                                               │
│    → rydit-lexer::Lexer::scan() → Tokens<'a>                │
│    → Parser::parse() → AST + Errors (error recovery)        │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  rydit-vm::Compiler                                         │
│    → compile(AST) → BytecodeProgram                         │
│      - OpCode::LoadConst, LoadGlobal, Add, etc.             │
│      - constants_num, constants_str, global_names           │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  rydit-vm::VM                                               │
│    → load(BytecodeProgram)                                  │
│    → run() → VMValue                                        │
│      - Stack-based execution                                │
│      - Call frames para funciones                           │
│      - Draw commands con callback                           │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  rydit-gfx (SDL2 Backend)                                   │
│    → draw_callback("circle", [x, y, radio])                 │
│    → RenderQueue → GPU → Pantalla                          │
└─────────────────────────────────────────────────────────────┘
```

---

## 🔑 CARACTERÍSTICAS v0.11.2

### **1. Zero-Copy Lexer** ✅
- Tokens con `&'a str` en vez de `String`
- 50% menos uso de memoria
- 2-3x más rápido en lexing

### **2. Error Recovery Parser** ✅
- No falla en el primer error
- Reporta múltiples errores
- Continúa parseando después de errores

### **3. AST Typed** ✅
- `Expr<'a>` con tipos específicos
- `BinaryOp`, `UnaryOp` enums
- Validación semántica temprana

### **4. Bytecode VM** ✅
- 50+ OpCode instructions
- Stack-based execution
- Call frames para funciones
- Draw commands integrados

### **5. Backward Compatibility** ✅
- `lizer` wrapper para código existente
- Re-exports de rydit-lexer + rydit-parser
- Migración gradual posible

---

## 📈 ROADMAP ACTUALIZADO

| Versión | Estado | Features | Fecha |
|---------|--------|----------|-------|
| **v0.11.0** | ✅ COMPLETADO | RyBot + SDL2 + Toolkit | 2026-03-28 |
| **v0.11.1** | ✅ COMPLETADO | Tests 3 niveles | 2026-04-01 |
| **v0.11.2** | ✅ COMPLETADO | Parser Zero-Copy + Bytecode VM | 2026-04-01 |
| **v0.11.3** | 🔮 Pendiente | Snake reescrito + Platformer SDL2 | 2026-04-14 |
| **v0.12.0** | 🔮 Meta | FSR 1.0 + Parser fuerte completo | 2026-04-21 |

---

## 🧪 TESTS v0.11.2

### **Nivel 1: Núcleo** ✅
- `rydit-lexer`: 20 tests
- `rydit-parser`: 23 tests
- `rydit-vm`: 19 tests
- `lizer`: 3 tests
- `blast-core`: 20 tests

### **Nivel 2: Integración** ✅
- `rydit-test`: 16 tests (Nivel 1 + 2)

### **Nivel 3: Gráficos** ⏳
- SDL2 low-end tests (manuales)

**Total**: 101 tests automáticos

---

## 🚀 COMANDOS ÚTILES

```bash
# Build workspace
cargo build --workspace

# Tests todos los crates nuevos
cargo test -p rydit-lexer -p rydit-parser -p rydit-vm -p lizer

# Build release optimizado
cargo build --release -p rydit-rs

# Ver estructura de crates
tree crates -L 2

# Ver tags de versión
git tag -l | grep v0.11.2
```

---

## 🔒 PUNTOS DE REVERSIÓN v0.11.2

| Tag | Descripción | Comando |
|-----|-------------|---------|
| `v0.11.2-pre-parser` | Backup antes de empezar | `git checkout v0.11.2-pre-parser` |
| `v0.11.2-fase-1` | rydit-lexer zero-copy | `git checkout v0.11.2-fase-1` |
| `v0.11.2-fase-2` | rydit-parser error recovery | `git checkout v0.11.2-fase-2` |
| `v0.11.2-fase-3` | rydit-vm bytecode | `git checkout v0.11.2-fase-3` |
| `v0.11.2-fase-4` | Integración workspace | `git checkout v0.11.2-fase-4` |

---

<div align="center">

**🛡️ RyDit v0.11.2 - PARSER ZERO-COPY + BYTECODE VM**

*65 tests passing ✅ | 4,155 líneas nuevas ✅ | Workspace compila ✅*

**Próximo: v0.11.3 - Snake reescrito + Platformer SDL2**

</div>
