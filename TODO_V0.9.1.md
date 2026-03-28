# 🛡️ RyDit v0.9.1 - LISTA DE TAREAS PENDIENTES

**Fecha**: 2026-03-28
**Versión actual**: v0.9.0 ✅ Entity System Completado
**Próxima versión**: v0.9.1 - Tests + Fixes + Multi-plataforma
**Prioridad**: 🔴 ALTA → 🟡 MEDIA → 🟢 BAJA

---

## 📋 TAREAS DE PULIDO (v0.9.1)

### 🔴 PRIORIDAD 1: Tests Gráficos en Termux-X11

| ID | Tarea | Descripción | Dificultad | Tiempo |
|----|-------|-------------|------------|--------|
| **T1.1** | Test Entity System | Verificar player, enemy, boss en Termux-X11 | 🟡 Media | 2 horas |
| **T1.2** | Test Colisiones | Verificar collision::check() en tiempo real | 🟢 Baja | 1 hora |
| **T1.3** | Test Cámara 2D | Verificar camera::follow() con jugador | 🟢 Baja | 1 hora |
| **T1.4** | Test Area2D | Verificar triggers y overlapping | 🟢 Baja | 1 hora |
| **T1.5** | Test Trampas | Verificar trap::set_visible(false) y daño | 🟢 Baja | 1 hora |
| **T1.6** | Test Monedas | Verificar coin::collect() y valores | 🟢 Baja | 30 min |
| **T1.7** | Test IA Enemigo | Verificar enemy::update_ai() chase | 🟡 Media | 1 hora |
| **T1.8** | Test Boss Fases | Verificar boss::transition_to_phase() | 🟡 Media | 1 hora |

**Demo de prueba**: `demo_entity_test_v0.9.1.rydit`

---

### 🔴 PRIORIDAD 2: Fixear Warnings Restantes

| ID | Warning | Archivo | Línea | Fix | Dificultad |
|----|---------|---------|-------|-----|------------|
| **W2.1** | `dead_code` - `Entity.width` | entity.rs | ~1670 | `#[allow(dead_code)]` | 🟢 Baja |
| **W2.2** | `dead_code` - `Entity.height` | entity.rs | ~1671 | `#[allow(dead_code)]` | 🟢 Baja |
| **W2.3** | `dead_code` - `Area2D.id` | entity.rs | ~1747 | `#[allow(dead_code)]` | 🟢 Baja |
| **W2.4** | `unused_variables` | Varios | Varios | Prefix con `_` | 🟢 Baja |

**Comando**: `cargo clippy --workspace --all-targets`

---

### 🟡 PRIORIDAD 3: Features/Módulos Complementarios

| ID | Feature | Descripción | Dificultad | Tiempo |
|----|---------|-------------|------------|--------|
| **F3.1** | `item::` Module | Power-ups (speed, shield, damage) | 🟡 Media | 3 horas |
| **F3.2** | `inventory::` Module | Sistema de inventario para jugador | 🔴 Alta | 5 horas |
| **F3.3** | `dialogue::` Module | Sistema de diálogos tipo RPG | 🟡 Media | 3 horas |
| **F3.4** | `quest::` Module | Sistema de quests/misiones | 🔴 Alta | 6 horas |
| **F3.5** | `particle::` Module | Sistema de partículas integrado | 🔴 Alta | 8 horas |
| **F3.6** | `audio_sfx::` Module | Efectos de sonido (steps, hits) | 🟡 Media | 3 horas |
| **F3.7** | `animation::` Module | Sprite animation (frames) | 🔴 Alta | 6 horas |
| **F3.8** | `tilemap::` Module | Carga y render de tilemaps | 🔴 Alta | 8 horas |

**Priorizar**: F3.1, F3.3, F3.6 (más útiles para demos)

---

### 🔴 PRIORIDAD 4: Evaluar Runners Multi-plataforma

#### **Objetivo**: Determinar complejidad de compilación para:
- Windows (x86_64)
- Linux (x86_64)
- Android (ARM32/ARM64)
- macOS (x86_64/ARM64)

| ID | Plataforma | Tarea | Dificultad | Tiempo |
|----|------------|-------|------------|--------|
| **R4.1** | **Windows** | Evaluar `cargo build --target x86_64-pc-windows-msvc` | 🔴 Alta | 4 horas |
| **R4.2** | **Linux** | Evaluar `cargo build --target x86_64-unknown-linux-gnu` | 🟡 Media | 2 horas |
| **R4.3** | **Android ARM32** | Evaluar `cargo build --target armv7-linux-androideabi` | 🔴 Alta | 4 horas |
| **R4.4** | **Android ARM64** | Evaluar `cargo build --target aarch64-linux-android` | 🟡 Media | 2 horas |
| **R4.5** | **macOS** | Evaluar `cargo build --target x86_64-apple-darwin` | 🟡 Media | 2 horas |
| **R4.6** | **macOS ARM64** | Evaluar `cargo build --target aarch64-apple-darwin` | 🟡 Media | 2 horas |

#### **Checklist por plataforma**:

**Windows**:
- [ ] ¿raylib disponible?
- [ ] ¿tungstenite compila?
- [ ] ¿ureq compila?
- [ ] ¿Dependencies de C (ring) compilan?
- [ ] ¿Tests passing?
- [ ] ¿Binario < 5 MB?

**Linux**:
- [ ] ¿raylib disponible?
- [ ] ¿Dependencies instalables vía apt?
- [ ] ¿Tests passing?
- [ ] ¿Binario estático posible?

**Android**:
- [ ] ¿NDK configurado?
- [ ] ¿Termux compatible?
- [ ] ¿Zink/Vulkan funciona?
- [ ] ¿Tests passing?

**macOS**:
- [ ] ¿Xcode CLI tools?
- [ ] ¿raylib disponible?
- [ ] ¿Tests passing?
- [ ] ¿Universal binary posible?

#### **Entregables**:
- `docs/COMPILACION_MULTIPLATAFORMA.md` - Guía completa
- `scripts/build_windows.sh` - Script Windows
- `scripts/build_linux.sh` - Script Linux
- `scripts/build_android.sh` - Script Android
- `scripts/build_macos.sh` - Script macOS
- Tabla comparativa de complejidad

---

## 📊 CRONOGRAMA ESTIMADO

### Semana 1 (v0.9.1-alpha)
- [ ] T1.1 - T1.8: Tests Gráficos (8 horas)
- [ ] W2.1 - W2.4: Fix Warnings (1 hora)
- [ ] R4.1 - R4.6: Evaluar Runners (16 horas)

**Total**: 25 horas (~3-4 días)

### Semana 2 (v0.9.1-beta)
- [ ] F3.1: item:: Module (3 horas)
- [ ] F3.3: dialogue:: Module (3 horas)
- [ ] F3.6: audio_sfx:: Module (3 horas)
- [ ] Documentación multi-plataforma (4 horas)

**Total**: 13 horas (~2 días)

### Semana 3 (v0.9.1-rc)
- [ ] F3.2: inventory:: Module (5 horas)
- [ ] F3.4: quest:: Module (6 horas)
- [ ] Tests multi-plataforma (8 horas)
- [ ] Demo completa (4 horas)

**Total**: 23 horas (~3 días)

### Semana 4 (v1.0.0-release)
- [ ] F3.5: particle:: Module (8 horas)
- [ ] F3.7: animation:: Module (6 horas)
- [ ] F3.8: tilemap:: Module (8 horas)
- [ ] 20+ demos (10 horas)
- [ ] Documentación final (8 horas)
- [ ] Release v1.0.0 (4 horas)

**Total**: 44 horas (~5-6 días)

---

## 🎯 CRITERIOS DE ACEPTACIÓN

### Tests Gráficos (T1.x)
- ✅ Todos los tests passing en Termux-X11
- ✅ 60 FPS estables con 10+ entidades
- ✅ Colisiones detectadas correctamente
- ✅ Cámara follow suave (sin jitter)
- ✅ Trampas invisibles funcionan
- ✅ Monedas se recolectan

### Warnings (W2.x)
- ✅ `cargo clippy` = 0 warnings
- ✅ `cargo fmt` aplicado
- ✅ Tests passing después de fixes

### Features (F3.x)
- ✅ Cada módulo con 5+ funciones
- ✅ 3+ tests por módulo
- ✅ Documentación en docs/
- ✅ Ejemplo de uso en ejemplos/

### Runners (R4.x)
- ✅ Guía de compilación por plataforma
- ✅ Scripts de build funcionales
- ✅ Tabla de complejidad completada
- ✅ Binarios generados para cada plataforma

---

## 📈 MÉTRICAS DE PROGRESO

| Fase | Tareas | Completadas | % |
|------|--------|-------------|---|
| **Tests Gráficos** | 8 | 0 | 0% |
| **Fix Warnings** | 4 | 0 | 0% |
| **Features** | 8 | 0 | 0% |
| **Runners** | 6 | 0 | 0% |
| **TOTAL** | **26** | **0** | **0%** |

---

## 🛠️ HERRAMIENTAS NECESARIAS

### Para Tests Gráficos
- Termux-X11 instalado
- DISPLAY=:0 configurado
- zink + DRI3 activos
- Demo de prueba creada

### Para Fix Warnings
- `cargo clippy --workspace --all-targets`
- Editor de código (vim/nano)
- `cargo fmt --all`

### Para Features
- crates/rydit-rs/src/modules/
- eval/mod.rs para integración
- Tests en tests/mod.rs

### Para Runners
- Docker (opcional, para cross-compilation)
- QEMU (para ARM testing)
- VMs (Windows, macOS)
- GitHub Actions (CI/CD)

---

## 📝 NOTAS ADICIONALES

### Notas sobre Tests Gráficos
- Priorizar T1.1, T1.2, T1.3 (más críticos)
- Crear demo mínima que se pueda ejecutar en < 1 min
- Capturar screenshots para documentación

### Notas sobre Warnings
- La mayoría son `dead_code` por campos de structs
- Usar `#[allow(dead_code)]` solo si es intencional
- Considerar remover campos no usados

### Notas sobre Features
- Empezar por F3.1 (item::) - más simple
- F3.5 (particle::) depende de renderizado fix
- F3.8 (tilemap::) requiere assets draw real

### Notas sobre Runners
- Windows es el más complejo (ring no compila fácil)
- Android ARM64 ya funciona (Termux actual)
- macOS requiere Xcode (puede ser blocker)
- Considerar GitHub Actions para CI automático

---

## 🚀 PRÓXIMOS PASOS INMEDIATOS

1. **HOY**: Empezar con T1.x (Tests Gráficos)
   - Crear `demo_entity_test_v0.9.1.rydit`
   - Ejecutar en Termux-X11
   - Documentar resultados

2. **MAÑANA**: W2.x (Fix Warnings)
   - `cargo clippy` para identificar todos
   - Fixear uno por uno
   - Verificar tests passing

3. **DÍA 3**: R4.x (Evaluar Runners)
   - Empezar por Linux (más fácil)
   - Luego Android ARM64 (ya conocido)
   - Luego Windows (más complejo)
   - Finalmente macOS

4. **DÍA 4-5**: F3.x (Features)
   - Empezar por F3.1 (item::)
   - Luego F3.3 (dialogue::)
   - Luego F3.6 (audio_sfx::)

---

<div align="center">

**🛡️ RyDit v0.9.1 - ROADMAP DE PULIDO**

*26 tareas | ~65 horas estimadas | 3-4 semanas*

**Objetivo: v1.0.0 Release Estable**

</div>

---

**Última actualización**: 2026-03-28
**Estado**: 📋 Pendiente de inicio
**Responsable**: Equipo RyDit
