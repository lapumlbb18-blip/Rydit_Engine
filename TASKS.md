# 🛡️ Ry-Dit - Tareas Principales y Paralelas v0.14.0+

**Última actualización**: 2026-04-06
**Versión actual**: v0.14.0 ✅ ry-backend dual + migui conectado + ry-system-ry
**Próxima versión**: v0.15.0 - Demos Termux-X11 + v-shield platform layer

---

## 📊 RESUMEN RÁPIDO

| Métrica | Valor |
|---------|-------|
| **Crates** | 24 |
| **Errores** | 0 |
| **Tests** | 95/95 pasando |
| **Crates publicados** | 2 (ry-god + ry-stream) |
| **Commit** | `2c97cbb` |
| **Disco proyecto** | 5.5 GB (target 5.3 GB + cargo 1.3 GB) |

---

## 🔴 TAREAS PRINCIPALES (Prioridad Alta)

### 1. Demos funcionales Termux-X11 con RySystem
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🔴 ALTA |
| **EsFuerzo** | 6-8h |
| **Estado** | ⏳ Pendiente |

**Detalle**:
- Crear demo usando `RySystem` (ry-system-ry)
- Integrar menu_bar de migui con TTF real
- Conectar events-ry input unificado
- Mini viewport 3D en el panel (ry3d-gfx)

---

### 2. Optimizar ry-rs: desacoplar y completar
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🔴 ALTA |
| **EsFuerzo** | 8-12h |
| **Estado** | ⏳ Pendiente |

**Problemas identificados**:
- **Dos RyditModule traits incompatibles**: `ry-core` (JSON-plugin) vs `ry-rs/module.rs` (game-loop hooks)
- **main.rs**: 5004 líneas, 13 submodulos, 20 deps
- **No es lib**: sin sección `[lib]`, solo binario
- **Acoplamiento alto**: ry-rs depende de casi todo

**Plan de optimización**:
1. Unificar los dos RyditModule traits en uno solo
2. Agregar `[lib]` a ry-rs/Cargo.toml
3. Extraer eval/ a crate separado si es posible
4. Completar module.rs: MathModule → más módulos integrados

---

### 3. Completar HUD + Menús de sistema ry
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🔴 ALTA |
| **EsFuerzo** | 4-6h |
| **Estado** | ⏳ Pendiente |

**Detalle**:
- toolkit-ry: 18 widgets HUD listos, integrar en RySystem
- Menús de sistema: Save/Load, Settings, Debug overlay
- rybot CLI: conectar con RySystem para debug en vivo

---

### 4. Avanzar demos: niveles 1→2 y 3→4
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🔴 ALTA |
| **EsFuerzo** | 10-15h |
| **Estado** | ⏳ Pendiente |

**Demos a completar**:
- **Snake completa**: lógica de juego + input + score
- **Efectos de partículas**: balas, explosiones, trail (GPU instancing)
- **Trampas**: plataformas móviles, pinchos, puertas
- **Nivel 3→4**: Integrar physics + anim + collision + tilemap

---

## 🟡 TAREAS PARALELAS

### 5. Publicar 5+ crates en crates.io
| Campo | Valor |
|-------|-------|
| **EsFuerzo** | 4-6h |
| **Estado** | ⏳ Pendiente |

**Crates listos**:
- ry-backend v0.1.0 (nuevo, dual backend)
- events-ry v0.1.0 (input unificado)
- ry-anim v0.12.0 (41 funciones)
- ry-physics v0.7.34 (2D + N-body)
- toolkit-ry v0.1.0 (5 temas + 20+ widgets)
- lizer v0.11.2 (wrapper + AST cache)

**4 crates publicados antes**: actualizar y republicar

---

### 6. v-shield → Platform layer real
| Campo | Valor |
|-------|-------|
| **EsFuerzo** | 15-20h |
| **Estado** | ⏳ Pendiente |

**Análisis**: v-shield actualmente es solo wrapper de colores raylib. Necesita:
- Platform detection (Android/Linux/Windows/macOS)
- Config defaults por OS
- Dependency verification
- Platform report visual
- Reemplazar PlatformSync stubs en ry-gfx/render_queue.rs

---

### 7. blast-core → Completar executor
| Campo | Valor |
|-------|-------|
| **EsFuerzo** | 4-6h |
| **Estado** | ⏳ Pendiente |

**Análisis**: blast-core es variable store con scopes. Faltan:
- `ejecutar()`: ejecutar AST/bytecode
- `shock_wave()`: propagar cambios a módulos
- Integrar con eval/mod.rs de ry-rs

---

### 8. ry-gfx: completar PlatformSync + GPU instancing
| Campo | Valor |
|-------|-------|
| **EsFuerzo** | 6-8h |
| **Estado** | ⏳ Pendiente |

**Análisis**:
- `PlatformSync` en render_queue.rs: X11 FFI stubs (retorna null)
- `DoubleBuffer` implementado pero no usado activamente
- `GPUInstancer` funcional, solo usado por partículas
- Integrar con RySystem para demos

---

### 9. Avanzar un poco más al 3D
| Campo | Valor |
|-------|-------|
| **EsFuerzo** | 8-12h |
| **Estado** | ⏳ Pendiente |

**Detalle**:
- Mini viewport 3D en panel visual
- ry3d-gfx: más primitives (torus, cone, cylinder)
- Camera3D: orbit, first-person
- ry-backend raylib_draw: conectar con ry3d-gfx

---

### 10. CI/CD GitHub Actions
| Campo | Valor |
|-------|-------|
| **EsFuerzo** | 4-6h |
| **Estado** | ⏳ Pendiente |

**Detalle**:
- Build automático en Linux runner
- Tests automáticos
- Previene regressions
- Artifact: ELF release

---

## 🔮 TAREAS FUTURO (v0.16.0+)

| # | Tarea | EsFuerzo | Descripción |
|---|-------|----------|-------------|
| 11 | Editor visual | 24-32h | RySystem + migui + events-ry + menus |
| 12 | LAZOS Python bridge async | 20-30h | API completa + async |
| 13 | ry-stream: DNS + WASM | 12-16h | Async + wasm32 dependencia |
| 14 | ry-dit triple backend | 15-20h | raylib + SDL2 + WASM |
| 15 | bgfx_libs referencias | 10-15h | imgui, sokol, SDL3 inspiración |
| 16 | ry-geometry (Vec3/Mat4) | 12-16h | Base para 3D completo |

---

## ✅ TAREAS COMPLETADAS v0.14.0

| # | Tarea | Commit |
|---|-------|--------|
| 1 | ry-backend v0.1.0 creado | `9a0c4e7` |
| 2 | raylib_draw: 2D/3D drawing | `9a0c4e7` |
| 3 | sdl2_core: TTF profesional + mouse + input | `9a0c4e7` |
| 4 | migui conectado a ry-backend | `ebe7c81` |
| 5 | ry-system-ry creado (RySystem) | `ebe7c81` |
| 6 | Texto TTF anti-alias blended | `9a0c4e7` |
| 7 | Mouse: click, doble click, derecho, scroll | `9a0c4e7` |
| 8 | Touch Android: FingerDown/Motion/Up | `9a0c4e7` |
| 9 | Features backend: raylib/sdl2/dual/mobile | `9a0c4e7` |
| 10 | 24 crates compilando sin errores | `ebe7c81` |
| 11 | Documentación actualizada v0.14.0 | `2c97cbb` |

---

## 💾 ALMACENAMIENTO: Análisis y Limpieza

### **Estado actual**
| Ubicación | Tamaño |
|-----------|--------|
| `target/` (build artifacts) | 5.3 GB |
| `~/.cargo/registry/` (crates descargados) | ~1.0 GB |
| `~/.cargo/git/` (deps git) | ~100 MB |
| **Total proyecto** | **5.5 GB** |
| **Total home/cargo** | **~1.3 GB** |

### **¿Es normal?**
✅ **SÍ, es completamente normal.** Cargo:
- Descarga todas las dependencias en `~/.cargo/registry/` (no se borran)
- Compila todo en `target/` (incluye deps, debug info, múltiples perfiles)
- Cada `cargo build` recompila lo cambiado y sus dependientes
- `cargo clean` borra `target/` pero el próximo build tarda más

### **Qué se puede hacer**

| Acción | Libera | Costo |
|--------|--------|-------|
| `cargo clean` | ~5.3 GB | Siguiente build lento |
| `cargo clean -p ry-rs` | ~2 GB | Solo limpia ry-rs |
| Eliminar `~/.cargo/registry/src/` | ~500 MB | Re-descarga si se necesita |
| `[profile.dev] debug = false` | ~1 GB | Menos debug info |
| `CARGO_TARGET_DIR=/path/externo` | Mueve target | USB/SD externa |

### **Recomendación**
```bash
# Limpieza segura (no rompe nada)
cargo clean

# Config para menos debug info
# En Cargo.toml workspace:
[profile.dev]
debug = false      # Ya está
opt-level = 1      # Ya está

# Si el espacio es crítico:
# Mover target a SD externa
export CARGO_TARGET_DIR=/sdcard/rust-target
```

---

## 📋 DEPENDENCIAS ENTRE TAREAS

```
ry-backend ✅
    │
    ├──→ migui conectado ✅
    │       │
    │       └──→ ry-system-ry ✅
    │               │
    │               └──→ Demos Termux-X11 (Tarea 1)
    │                       │
    │                       └──→ Editor visual (Tarea 11)
    │
    └──→ v-shield platform layer (Tarea 6)
            │
            └──→ CI/CD (Tarea 10)

ry-rs optimización (Tarea 2)
    │
    ├──→ Unificar RyditModule traits
    │       └──→ blast-core ejecutar() (Tarea 7)
    │
    └──→ HUD + menús sistema ry (Tarea 3)
            │
            └──→ Demos niveles 1→2, 3→4 (Tarea 4)

LAZOS funcional ✅
    │
    └──→ LAZOS Python async (Tarea 12)
    └──→ ry-stream DNS + WASM (Tarea 13)
    └──→ ry-dit triple backend (Tarea 14)
```

---

## 📊 ANÁLISIS DE CRATES TEMPRANOS

### v-shield
| Campo | Valor |
|-------|-------|
| **Estado** | ⚠️ Mínimo (colores + init_window) |
| **Propósito original** | Platform layer multiplataforma |
| **Realidad actual** | Wrapper delgado de raylib |
| **Superado por** | ry-gfx (ya hace todo el rendering) |
| **Acción** | Expandir a platform layer real (detect OS, config, report) |

### blast-core
| Campo | Valor |
|-------|-------|
| **Estado** | ✅ Variable store completo |
| **Propósito** | Executor de valores del lenguaje |
| **Features** | Memoria con scopes, input/voz, arrays |
| **Faltante** | `ejecutar()` (AST/bytecode), `shock_wave()` (propagación) |
| **Acción** | Completar ejecutar() integrando con eval/mod.rs |

### PlatformSync (ry-gfx)
| Campo | Valor |
|-------|-------|
| **Estado** | ⚠️ Stubs X11 FFI |
| **Double buffering** | ✅ Implementado en DoubleBuffer |
| **Render queue** | ✅ Funcional (8192 commands, FIFO) |
| **GPU instancing** | ✅ OpenGL funcional |
| **Acción** | Completar PlatformSync con X11 real o remover stubs |

### RyditModule Trait (doble definición)
| Ubicación | Firma | Propósito |
|-----------|-------|-----------|
| `ry-core/src/lib.rs` | `execute(command, params) → ModuleResult` | JSON-plugin system |
| `ry-rs/src/module.rs` | `update(ctx), render(ctx)` | Game-loop hooks |
| **Problema** | ❌ Incompatibles | Dos traits con mismo nombre |
| **Acción** | Unificar en un solo trait con ambos usos |

---

<div align="center">

**🛡️ Ry-Dit v0.14.0 - Tareas Documentadas**

*0 errores | 24 crates | 95 tests | 10 tareas v0.14.0 completadas*

**Próxima: v0.15.0 - Demos Termux-X11 + optimización ry-rs**

</div>
