## 📦 Lista de Crates (24)

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
| ry-system-ry | 0.14.0 | ✅ | Sistema unificado: RySystem |
| ry-test | — | ⚠️ | Test utilities |
| **ry-backend** | **0.1.0** | ✅ **NUEVO** | Dual backend: raylib + SDL2 TTF |
| toolkit-ry | 0.1.0 | ✅ | 5 temas + 20+ widgets |
| migui | 0.4.1 | ✅ | Conectado a ry-backend |
| blast-core | 0.1.0 | ✅ | Minimal value executor |
| lizer | 0.11.2 | ✅ | Legacy + AST cache real |
| v-shield | — | ⚠️ | Platform layer (pendiente) |
| ry3d-gfx | 0.1.0 | ✅ | 15 funciones 3D |
| events-ry | 0.1.0 | ✅ | Input unificado + TextInput + Shell |
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
| v-shield platform layer | 15-20h | 🟡 Media |
| ry-stream v0.2.0 (mDNS) | 8-12h | 🟡 Media |
| ry-physics N-cuerpos >2 | 10-15h | 🟡 Media |
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
4. **Debug tests antes de fixear** - Identificar tipos exactos
5. **Agente para inspección** - Análisis profundo de errores
6. **Fix manual (NO sed)** - Control total de cambios
7. **Commits frecuentes** - Puntos de reversión claros
8. **Tags descriptivos** - Cada fix importante tiene tag
9. **Cargo clippy --fix** - Identifica warnings ocultos

### **❌ LO QUE NO FUNCIONÓ**

1. **sed automático** - Rompió código, tuvo que revertir
2. **Arc<str> option** - Evaluada pero rechazada (muy riesgosa)
3. **Lifetime elision** - Compilador requirió explícitos
4. **Crear demos duplicados** - demo_ttf_sprites_real falló linker
5. **Dar vueltas en círculos** - Analizar lo que ya funciona primero

---

<div align="center">

**🛡️ RyDit v0.14.0 - ry-backend dual + migui conectado + ry-system-ry**

*0 errores | 24 crates compilando | 95 tests pasando | 2 crates publicados*

**Próximo: v0.15.0 - Demos Termux-X11 + v-shield platform layer**

**REGLA DE ORO: NUNCA SED DESPUÉS DE REFACTORIZAR PARSER**

</div>
