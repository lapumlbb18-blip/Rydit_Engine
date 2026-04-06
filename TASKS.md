# 🛡️ Ry-Dit - Tareas Principales y Paralelas v0.13.0+

**Última actualización**: 2026-04-05
**Versión actual**: v0.13.0 ✅ events-ry + Panel Visual + RyBot IPC + LAZOS
**Próxima versión**: v0.14.0 - migui texto real + v-shield platform layer

---

## 📊 RESUMEN RÁPIDO

| Métrica | Valor |
|---------|-------|
| **Crates** | 23 |
| **Errores** | 0 |
| **Tests** | 95/95 pasando |
| **Crates publicados** | 2 (ry-god + ry-stream) |
| **Commit** | `118ee6a` |

---

## 🔴 TAREAS PRINCIPALES (v0.14.0)

### 1. migui Texto Real (ab_glyph)
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🔴 ALTA |
| **EsFuerzo** | 4-6h |
| **Bloquea** | TODOS los widgets muestran texto legible |
| **Estado** | ⏳ Pendiente |

**Detalle**:
- `migui/font_native.rs` usa placeholder (rectángulos como texto)
- Implementar `ab_glyph` para renderizado real de fuentes TTF
- Desbloquea: consola visual, HUD, menús, editor

**Archivos**:
- `crates/migui/src/font_native.rs` → implementar render_text real
- `crates/migui/src/backend_sdl2.rs` → conectar con SDL2_ttf si disponible

---

### 2. Sdl2InputBackend en Demo Real
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🔴 ALTA |
| **EsFuerzo** | 2-3h |
| **Bloquea** | Panel visual con input real |
| **Estado** | ⏳ Pendiente |

**Detalle**:
- `events-ry` tiene `Sdl2InputBackend` feature-gated
- Conectar con `demo_panel_visual` para input real
- Reemplazar input simulado por SDL2 event_pump

**Archivos**:
- `crates/ry-rs/src/bin/demo_panel_visual.rs`
- `crates/events-ry/src/sdl2_backend.rs`

---

### 3. v-shield Platform Layer
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🟡 MEDIA |
| **EsFuerzo** | 15-20h |
| **Bloquea** | Multiplataforma real |
| **Estado** | ⏳ Pendiente |

**Detalle**:
- Detectar plataforma (Android/Linux/Windows/macOS/iOS)
- Config defaults por OS
- Verificar deps disponibles
- Platform report visual

**Archivos**:
- `crates/v-shield/src/lib.rs` → expandir de wrapper mínimo a platform layer

---

### 4. Consola Visual en ry-gfx
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🟡 MEDIA |
| **EsFuerzo** | 3-4h |
| **Bloquea** | Output visual de shell |
| **Estado** | ⏳ Pendiente |

**Detalle**:
- Integrar `events-ry::Shell` output en ry-gfx render
- Panel de consola con scroll y colores por tipo
- Conectar con `demo_panel_visual`

---

### 5. ry-stream v0.2.0 (mDNS)
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🟡 MEDIA |
| **EsFuerzo** | 8-12h |
| **Bloquea** | LAN streaming mejorado |
| **Estado** | ⏳ Pendiente |

**Detalle**:
- mDNS para auto-descubrimiento
- Portal web mejorado
- Multi-client support

---

### 6. ry-physics N-cuerpos >2
| Campo | Valor |
|-------|-------|
| **Prioridad** | 🟡 MEDIA |
| **EsFuerzo** | 10-15h |
| **Bloquea** | Simulaciones gravitacionales reales |
| **Estado** | ⏳ Pendiente |

**Detalle**:
- Actual `nbody_simulate` soporta N cuerpos pero es O(n²)
- Optimizar con Barnes-Hut o similar
- Integrar con ry-gfx para visualización

---

## 🟡 TAREAS PARALELAS

### 7. HybridBackend (SDL2 input + raylib render)
| Campo | Valor |
|-------|-------|
| **EsFuerzo** | 6-8h |
| **Estado** | ⏳ Pendiente |

**Detalle**: Ideal para Termux-X11. SDL2 captura input, raylib renderiza.

---

### 8. RaylibInputBackend
| Campo | Valor |
|-------|-------|
| **EsFuerzo** | 4-6h |
| **Estado** | ⏳ Pendiente |

**Detalle**: Conectar raylib input → events-ry InputEvent. Limitado en Termux.

---

### 9. 37 Warnings dead_code → API doc
| Campo | Valor |
|-------|-------|
| **EsFuerzo** | 2-3h |
| **Estado** | ⏳ Pendiente |

**Detalle**: Quest system, config_parser, entity system son API futura. Documentar como tal.

---

### 10. Integrar toolkit-ry en demo_panel_visual
| Campo | Valor |
|-------|-------|
| **EsFuerzo** | 3-4h |
| **Estado** | ⏳ Pendiente |

**Detalle**: Usar widgets reales de toolkit-ry en lugar de block text.

---

## 🔮 TAREAS FUTURO (v0.15.0+)

| # | Tarea | EsFuerzo | Descripción |
|---|-------|----------|-------------|
| 11 | LAZOS Python bridge completo | 20-30h | API completa + async |
| 12 | Editor visual básico | 24-32h | migui + toolkit-ry + events-ry |
| 13 | bgfx_libs referencias | 10-15h | imgui, sokol, SDL3 inspiración |
| 14 | ry-geometry (Vec3/Mat4) | 12-16h | Base para 3D completo |
| 15 | CI/CD GitHub Actions | 4-6h | Build automatizado |
| 16 | Publicar 5+ crates en crates.io | 4-6h | ry-core, ry-lexer, ry-parser, ry-anim, ry-physics |

---

## ✅ TAREAS COMPLETADAS v0.13.0

| # | Tarea | EsFuerzo | Commit |
|---|-------|----------|--------|
| 1 | events-ry v0.1.0 completo | 7 archivos | `2933a38` |
| 2 | Sdl2InputBackend feature-gated | 1 archivo | `2b1cc64` |
| 3 | demo_panel_visual SDL2 | 535 líneas | `2f48a44` |
| 4 | Limpieza warnings 146→37 | 14 archivos | `ef11b5f` |
| 5 | README actualizado a v0.13.0 | 1 archivo | `0f70d6b` |
| 6 | lizer AST cache real | 215 líneas | `d919a3c` |
| 7 | RyBot CLI IPC completo | 4 archivos | `118ee6a` |
| 8 | LAZOS JSON-RPC + Python verificado | Tests | `118ee6a` |
| 9 | snake movido a pendientes | 1 archivo | `d919a3c` |
| 10 | Tag v0.13.0 creado | git tag | - |

---

## 📋 DEPENDENCIAS ENTRE TAREAS

```
migui texto real (1)
    │
    ├──→ Integrar toolkit-ry en demo (10)
    │       │
    │       └──→ Consola visual (4)
    │               │
    │               └──→ Editor visual (12)
    │
    └──→ ry-system-ry unificador
            │
            └──→ v-shield platform layer (3)

events-ry completo (✅)
    │
    ├──→ Sdl2InputBackend en demo (2)
    │       │
    │       └──→ HybridBackend (7)
    │
    └──→ RaylibInputBackend (8)

LAZOS funcional (✅)
    │
    └──→ LAZOS Python bridge completo (11)
```

---

<div align="center">

**🛡️ Ry-Dit v0.13.0 - Tareas Documentadas**

*0 errores | 23 crates | 95 tests | 10 tareas principales completadas*

**Próxima: v0.14.0 - migui texto real + v-shield**

</div>
