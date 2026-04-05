# 🛡️ QWEN.md - Bitácora Técnica Ry-Dit

**Última actualización**: 2026-04-05
**Versión actual**: v0.12.0 ✅ ry-anim v0.12.0 + Action Assets
**Versión anterior**: v0.11.5 (0 errores + lifetimes)
**Próxima versión**: v0.13.0 - ry-input + Demos completos
**Commit**: `405a945`
**Repositorio**: `https://github.com/lapumlbb18-blip/Ry-dit`
**Crates publicados**: ry-god v0.1.0 + ry-stream v0.1.0 ✅

---

## 🎉 v0.12.0 COMPLETADA

### ✅ **ESTADO ACTUAL: 22 CRATES COMPILANDO | 0 ERRORES | 58 TESTS**

| Sistema | Estado | Cambios | Notas |
|---------|--------|---------|-------|
| **ry-anim** | ✅ v0.12.0 | 41 funciones | 58 tests, 4 demos |
| **ry-stream** | ✅ crates.io | v0.1.0 publicado | LAN streaming |
| **ry-god** | ✅ crates.io | v0.1.0 publicado | Security & Efficiency |
| **Crates** | ✅ 22/22 | 0 errores | Workspace completo |
| **ELFs** | ✅ 2 compilados | demo_anime_ry 341K, demo_rigidbody 446K | release |
| **Bins** | ✅ ~31 | src/bin/ | Demos + tests |

**Total**: ry-anim completo | 2 crates publicados | Push + Sync ✅

---

## 📋 METODOLOGÍA APLICADA v0.12.0

### **Sesión Completa (todo lo hecho)**

| # | Feature | Estado | Detalles |
|---|---------|--------|----------|
| 1 | Math avanzado | ✅ | 23 funciones (pow, log, derivadas, integrales, PI, E...) |
| 2 | Arrays completos | ✅ | 16 funciones (push, pop, len, insert, remove, contains...) |
| 3 | Vec2 tipo nativo | ✅ | 22 operaciones (add, sub, scale, normalize, dot, cross...) |
| 4 | toolkit-ry v0.1.0 | ✅ | 5 temas + 20+ widgets UI |
| 5 | ry3d-gfx v0.1.0 | ✅ | 15 funciones 3D (cube, sphere, cylinder, grid...) |
| 6 | Fix input Android | ✅ | SDL_TEXTINPUT + 7 hints SDL2 |
| 7 | FSR 1.0 | ✅ | Shaders embebidos |
| 8 | Quest System | ✅ | 10 funciones |
| 9 | Save/Load System | ✅ | 10 funciones |
| 10 | One-way platforms | ✅ | 2 funciones |
| 11 | ry-stream crates.io | ✅ | v0.1.0 publicado |
| 12 | ry-ecs eliminado | ✅ | -1,143 líneas |
| 13 | nbody_simulate → ry-physics | ✅ | Movido correctamente |
| 14 | ry-anim v0.8→v0.12 | ✅ | 41 funciones, 58 tests, 4 demos |
| 15 | Fix linking raylib | ✅ | build.rs corregido |
| 16 | demo_anime_ry ELF | ✅ | 341K release |
| 17 | Documentos nuevos | ✅ | 9 creados |
| 18 | Docs organizados | ✅ | 17 archivos en docs/ |

### **ry-anim Evolución**

| Versión | Features | Tests | Demos |
|---------|----------|-------|-------|
| v0.8.0 | 15 Disney completo | 28 | — |
| v0.9.0 | 21 + 6 ilusiones | 35 | demo_illusions |
| v0.10.0 | 27 + 6 efectos | 42 | demo_effects |
| v0.11.0 | 35 + 8 ciencia | 50 | demo_science |
| v0.12.0 | 41 + 6 action_assets | 58 | demo_action_assets |

---

## 📦 Lista de Crates (22)

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
| ry-rs | — | Main | Binary principal |
| ry-system-ry | 0.11.0 | ⚠️ | Universal system (SDL2) |
| ry-test | — | ⚠️ | Test utilities |
| toolkit-ry | 0.1.0 | ✅ | 5 temas + 20+ widgets |
| migui | — | ✅ | 12 widgets UI |
| blast-core | 0.1.0 | ✅ | Minimal value executor |
| lizer | 0.11.2 | ✅ | Legacy lexer wrapper |
| v-shield | — | ⚠️ | Platform layer (pendiente) |
| ry3d-gfx | 0.1.0 | ✅ | 15 funciones 3D |
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

---

## 🚀 PRÓXIMOS PASOS (v0.13.0)

### **Pendientes - Prioridad Alta**

| Tarea | Esfuerzo | Prioridad |
|-------|----------|-----------|
| ry-input crate (SDL2 input + raylib render) | 10-15h | 🔴 Alta |
| Sprite animation en juegos reales | 15-20h | 🟡 Media |
| v-shield platform layer | 15-20h | 🟡 Media |
| ry-stream v0.2.0 (mDNS) | 8-12h | 🟡 Media |
| ry-physics N-cuerpos >2 | 10-15h | 🟡 Media |
| LAZOS Python bridge | 20-30h | 🔮 Futuro |
| Editor visual | 24-32h | 🔮 Futuro |

### **Después de v0.13.0**

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
git add -A && git commit -m "mensaje" && git tag -a v0.12.0
```

---

## 🎯 LECCIONES APRENDIDAS

### **✅ LO QUE SÍ FUNCIONÓ**

1. **Debug tests antes de fixear** - Identificar tipos exactos
2. **Agente para inspección** - Análisis profundo de errores
3. **Fix manual (NO sed)** - Control total de cambios
4. **Commits frecuentes** - Puntos de reversión claros
5. **Tags descriptivos** - Cada fix importante tiene tag
6. **Cargo clippy --fix** - Identifica warnings ocultos

### **❌ LO QUE NO FUNCIONÓ**

1. **sed automático** - Rompió código, tuvo que revertir
2. **Arc<str> option** - Evaluada pero rechazada (muy riesgosa)
3. **Lifetime elision** - Compilador requirió explícitos

---

<div align="center">

**🛡️ RyDit v0.12.0 - ry-anim Action Assets + 58 Tests**

*0 errores | 22 crates compilando | 58 tests pasando | 2 crates publicados*

**Próximo: v0.13.0 - ry-input + Demos completos**

**REGLA DE ORO: NUNCA SED DESPUÉS DE REFACTORIZAR PARSER**

</div>

## Qwen Added Memories
- CLAVE ARQUITECTÓNICA: Shell + Text Input es el corazón del editor visual ry-dit. No es solo capturar teclas individuales, sino: 1) TextInput para composición de strings (rutas, nombres, comandos), 2) Shell para ejecutar comandos (carga assets, debug, REPL rydit), 3) Console para output en tiempo real. Sin esto, el editor visual no funciona. events-ry debe tener 3 capas: InputEvent (raw) → TextInput (composición) → Shell (ejecución). HybridBackend preferido: SDL2 input + raylib render en Termux-X11.
