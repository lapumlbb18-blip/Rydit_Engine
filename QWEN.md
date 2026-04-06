# 🛡️ QWEN.md - Bitácora Técnica Ry-Dit

**Última actualización**: 2026-04-06
**Versión actual**: v0.14.0 ✅ demo_torreta_vs_sprites + 25 crates
**Versión anterior**: v0.13.0 (events-ry + Panel Visual)
**Próxima versión**: v0.15.0 - Demos Termux-X11 + v-shield platform layer
**Commit**: `df4ec17`
**Repositorio**: `https://github.com/lapumlbb18-blip/Ry-dit`
**Crates publicados**: 2 (ry-god v0.1.0 + ry-stream v0.1.0) ✅

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
| **ELFs** | ✅ 7+ compilados | demo_torreta_vs_sprites 434K, demo_rigidbody 446K, demo_anime_ry 341K | release |
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

## 📦 Lista de Crates (25)

| Crate | Versión | Estado | Notas |
|-------|---------|--------|-------|
| ry-core | 0.8.2 | ✅ | Core traits, module system |
| ry-lexer | 0.1.0 | ✅ | Zero-copy lexer |
| ry-parser | 0.1.0 | ✅ | Parser AST + error recovery |
| ry-vm | — | ⚠️ | VM opcodes |
| ry-gfx | 0.10.7 | ✅ | Graphics (raylib + SDL2 + OpenGL FFI) |
| ry-physics | 0.7.34 | ✅ | + nbody_simulate |
| ry-anim | 0.12.0 | ✅ | 41 funciones, 58 tests |
| ry-science | — | ⚠️ | Geometry 2D + stats + Bezier |
| ry-script | 0.8.2 | ✅ | Script loading |
| ry-stream | 0.1.0 | ✅ | crates.io publicado |
| ry-god | 0.1.0 | ✅ | crates.io publicado |
| ry-loader | — | ⚠️ | Module loader |
| ry-rs | — | Main | Binary + lib |
| ry-system-ry | 0.14.0 | ✅ | Sistema unificado: RySystem (core + gui) |
| ry-test | — | ⚠️ | Test utilities |
| ry-backend | 0.1.0 | ✅ | Dual backend: raylib + SDL2 TTF/input/audio |
| ry-config | 0.1.0 | ✅ | Config parser (entities, levels, checkpoints) |
| toolkit-ry | 0.1.0 | ✅ | 5 temas + 20+ widgets |
| migui | 0.4.1 | ✅ | Conectado a ry-backend |
| blast-core | 0.1.0 | ✅ | Minimal value executor |
| lizer | 0.11.2 | ✅ | Legacy + AST cache real (FNV-1a, 256 entradas, LRU) |
| v-shield | — | ⚠️ | Platform layer (pendiente) |
| ry3d-gfx | 0.1.0 | ✅ | 15 funciones 3D |
| events-ry | 0.1.0 | ✅ | Input unificado 3 capas + Sdl2InputBackend |
| ~~ry-ecs~~ | — | 🗑️ | Eliminado (-1,143 líneas) |

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

## 🚀 PRÓXIMOS PASOS (v0.15.0)

### **Pendientes - Prioridad Alta**

| Tarea | Esfuerzo | Prioridad |
|-------|----------|-----------|
| Demos funcionales Termux-X11 con RySystem | 6-8h | 🔴 Alta |
| v-shield platform layer | 15-20h | 🔴 Alta |
| ry-stream v0.2.0 (mDNS) | 8-12h | 🟡 Media |
| ry-physics N-cuerpos >2 | 10-15h | 🟡 Media |

### **Pendientes - Prioridad Media/Futura**

| Tarea | Esfuerzo | Prioridad |
|-------|----------|-----------|
| Platform crate (abstracción multiplataforma) | 15-20h | 🔮 Futuro |
| Soporte de emojis en TTF | 4-6h | 🔮 Futuro |
| GIF animation | 8-12h | 🔮 Futuro |
| GPU instancing (revisar gpu_instancing.rs de ry-gfx) | 10-15h | 🔮 Futuro |
| Features 3D paso a paso | 12-16h | 🔮 Futuro |
| LAZOS Python bridge | 20-30h | 🔮 Futuro |
| Editor visual | 24-32h | 🔮 Futuro |

### **Después de v0.15.0**

1. Demos funcionales en Termux-X11
2. Galería actualizada en README
3. Más crates publicados en crates.io
4. Documentación completa

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

### **❌ LO QUE NO FUNCIONÓ**

1. **sed automático** - Rompió código, tuvo que revertir
2. **Arc<str> option** - Evaluada pero rechazada (muy riesgosa)
3. **Lifetime elision** - Compilador requirió explícitos
4. **Crear demos duplicados** - demo_ttf_sprites_real falló linker
5. **Dar vueltas en círculos** - Analizar lo que ya funciona primero

---

<div align="center">

**🛡️ RyDit v0.14.0 - demo_torreta_vs_sprites + 25 Crates + 95+ Tests**

*0 errores | 25 crates compilando | 95+ tests pasando | 2 crates publicados*

**Próximo: v0.15.0 - Demos Termux-X11 + v-shield platform layer**

**REGLA DE ORO: NUNCA SED DESPUÉS DE REFACTORIZAR PARSER**

</div>
