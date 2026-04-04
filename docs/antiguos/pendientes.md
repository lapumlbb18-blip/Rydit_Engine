# 📋 Pendientes RyDit v0.11.6 - Sumatoria Total

**Fecha**: 2026-04-02
**Versión**: v0.11.6

---

## 🔴 P0 - Críticos (Fixeados)

| Tarea | Estado | Commit |
|-------|--------|--------|
| Eliminar println! de blast-core | ✅ | eace3c0 |
| Fix use lizer → rydit_parser en collision.rs | ✅ | eace3c0 |
| Reemplazar static mut con OnceLock | ✅ | eace3c0 |
| Fix lexer char-index → byte-index | ✅ | 25eb538 |

## 🟡 P1 - Importantes (Parcialmente)

| Tarea | Estado | Notas |
|-------|--------|-------|
| Short-circuit &&/\|\| | ✅ | eace3c0 |
| tecla_presionada() warning | ✅ | eace3c0 |
| Error estricto collision functions | ✅ | eace3c0 |
| ModuleRegistry no se usa | ⏳ | Requiere refactor |
| eval/mod.rs 3,520 líneas | ⏳ | 2-3 semanas |
| main.rs 4,650 líneas | ⏳ | 2-3 semanas |

---

## 📊 Sumatoria Total de Crates Evaluados

| Crate | Líneas | Tests | Clippy | Madurez | Pendientes Críticos |
|-------|--------|-------|--------|---------|-------------------|
| **rydit-lexer** | ~3K | 74 ✅ | 0 ✅ | **9/10** | Nested blocks |
| **rydit-parser** | ~3K | 24 ✅ | 0 ✅ | **8/10** | Error recovery |
| **blast-core** | ~2K | 20 ✅ | 0 ✅ | **8/10** | ScopeGuard, memory limit |
| **rydit-vm** | ~2K | 19 ✅ | 0 ✅ | **8/10** | Integración completa |
| **rydit-science** | ~2K | 21 ✅ | 0 ✅ | **7/10** | Vec2, lerp, N-body |
| **rydit-anim** | ~1K | 11 ✅ | 0 ✅ | **6/10** | 9 principios, keyframes |
| **rydit-gfx** | 5,607 | 31 ✅ | 8 ⚠️ | **5/10** | GPU instancing, FSR, backend dual |
| **rydit-ecs** | 697 | 5 ✅ | 1 ⚠️ | **4/10** | No es ECS real, bevy_ecs muerto |
| **v-shield** | 470 | 7 ✅ | 0 ✅ | **2/10** | Código duplicado, eliminar |

### Totales del Proyecto

| Métrica | Valor |
|---------|-------|
| **Líneas totales** | ~28,000+ |
| **Tests totales** | 260+ |
| **Warnings clippy** | 9 |
| **Madurez promedio** | ~6.5/10 |

---

## 🎯 Lo Que Falta Para v1.0.0

### rydit-gfx (5/10)
| Feature | Estado | Esfuerzo |
|---------|--------|----------|
| GPU Instancing integrado | ❌ Stub | 2-3 semanas |
| FSR render-to-texture | ❌ Stub | 2-3 semanas |
| Backend unificado (trait) | ❌ Dual conflict | 1-2 semanas |
| Audio memory safety | ❌ Leaks | 1 semana |
| Texture loading SDL2 | ❌ Stub | 1 semana |
| Camera apply integrado | ❌ NOOP | 1 semana |
| UI Toolkit integrado | ❌ Orphaned | 1-2 semanas |

### rydit-ecs (4/10)
| Feature | Estado | Esfuerzo |
|---------|--------|----------|
| ECS real con queries | ❌ No existe | 2-3 semanas |
| Collision detection | ❌ Missing | 2 semanas |
| Eliminar bevy_ecs muerto | ⏳ Dead code | 1 día |
| Spatial partitioning | ❌ Missing | 1 semana |
| N-body G escalado | ❌ G real = 0 visible | 2 horas |

### v-shield (2/10)
| Tarea | Estado | Esfuerzo |
|-------|--------|----------|
| Eliminar o mergear | ⏳ Duplicado | 1 día |

### rydit-science (7/10)
| Feature | Estado | Esfuerzo |
|---------|--------|----------|
| Tipo Vec2/Vec3 | ❌ Missing | 2 horas |
| lerp(a,b,t) | ❌ Missing | 30 min |
| N-body gravity | ❌ Missing | 1-2 semanas |

### rydit-anim (6/10)
| Feature | Estado | Esfuerzo |
|---------|--------|----------|
| 9 principios Disney | ❌ Missing | 2 semanas |
| Keyframes system | ❌ Missing | 1 semana |
| Timeline | ❌ Missing | 1 semana |

---

## 📅 Timeline Realista

| Versión | Contenido | Tiempo |
|---------|-----------|--------|
| **v0.11.7** | Delete v-shield, fix bevy_ecs, fix audio leaks | 1 semana |
| **v0.12.0** | GPU Instancing integrado + demo funcional | 3 semanas |
| **v0.12.1** | FSR pipeline + render-to-texture | 3 semanas |
| **v0.12.2** | Collision system + ECS queries | 3 semanas |
| **v0.13.0** | Backend trait + feature flags | 2 semanas |
| **v0.13.1** | UI Toolkit integrado | 2 semanas |
| **v0.14.0** | Parser fuerte + error recovery | 3 semanas |
| **v1.0.0** | **Todo arriba + 3 demos jugables** | **~4-5 meses** |

---

## 🎯 Recomendación Estratégica

### NO Hacer
- ❌ NO agregar más features hasta completar las existentes
- ❌ NO mantener v-shield (eliminar o mergear)
- ❌ NO mantener bevy_ecs como dependencia muerta
- ❌ NO mantener dual backend sin feature flags

### SÍ Hacer
- ✅ Elegir UN backend principal (SDL2 para Android)
- ✅ Completar pipeline gráfica end-to-end
- ✅ Un demo jugable que use todo (Snake o Tank)
- ✅ v1.0.0 = "un juego funciona end-to-end"

---

<div align="center">

**📋 RyDit v0.11.6 - Sumatoria Total**

*9 crates evaluados | ~28K líneas | 260+ tests | Madurez: 6.5/10*

**Próximo: v0.11.7 - Limpieza de deuda técnica**

</div>
