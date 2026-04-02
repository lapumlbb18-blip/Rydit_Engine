# 🛡️ PLAN MAESTRO v0.10.4 - PRIMERA PARTE: RECONEXIÓN

**Documento de Trabajo para Modo Agente**

**Fecha**: 2026-03-31  
**Versión**: v0.10.4  
**Estado**: ⏸️ Pendiente de autorización  
**Autorización requerida**: Modo agente con permisos completos

---

## 📋 RESUMEN EJECUTIVO

Este documento describe la **Fase 1: Reconexión Parser → Eval → Modules**, que es el 98% del trabajo pendiente mencionado en la sesión anterior.

**Objetivo principal**: Reconectar el flujo de evaluación que está roto entre:
```
Parser (lizer) → ??? (conexiones faltantes) → Eval (eval/mod.rs) → Modules (12 módulos)
```

**Duración estimada**: 2-3 días de trabajo en modo agente  
**Archivos críticos**: 15-20 archivos .rs  
**Riesgo**: Medio (requiere comprensión profunda del flujo)

---

## 🔍 DIAGNÓSTICO ACTUAL

### Lo Que Sabemos (Sesión Anterior)

| Sistema | Estado | Ubicación | Problema |
|---------|--------|-----------|----------|
| **Input Map** | ✅ Código existe | `modules/input_map.rs` (500+ líneas) | ⚠️ No integrado al game loop |
| **Assets** | ✅ Funcionaba antes | `modules/assets.rs` | ❌ Texturas no cargan ahora |
| **Physics 2D** | ✅ 20 funciones | `modules/physics.rs` | ⚠️ Solo detección, sin respuesta |
| **Camera 2D** | ✅ 15 funciones | `modules/camera.rs` | ⚠️ No transforma draw calls |
| **Entity System** | ✅ 50+ funciones | `modules/entity.rs` | ⚠️ Sin ECS real |
| **Parser Lizer** | ⚠️ 3K líneas | `crates/lizer/src/lib.rs` | Sobrecargado, hace trabajo de core |
| **Eval Module** | ❓ 98% conectado | `crates/rydit-rs/src/eval/mod.rs` | **Conexiones faltantes** |
| **Main.rs** | ⚠️ 4K líneas | `crates/rydit-rs/src/main.rs` | Muy poco para ser core (debería ser 20K+) |

### Hipótesis del Problema

1. **Parser → Eval**: El parser genera AST pero no llega correctamente al evaluador
2. **Eval → Modules**: El evaluador no encuentra/usa los módulos registrados
3. **Modules → Game Loop**: Los módulos existen pero no se llaman en el loop principal
4. **Assets → rydit-gfx**: Algo cambió en rydit-gfx que rompió la carga de texturas

---

## 🎯 OBJETIVOS DE LA FASE 1

### Objetivo 1: Mapear Conexiones Faltantes

**Tareas**:
1. [ ] Leer `eval/mod.rs` completo (línea por línea)
2. [ ] Leer `main.rs` completo (game loop actual)
3. [ ] Listar TODOS los módulos en `modules/`
4. [ ] Trazar el flujo: parser → eval → modules → render
5. [ ] Identificar puntos de ruptura

**Entregable**: Diagrama de flujo con conexiones rotas marcadas en rojo

---

### Objetivo 2: Integrar Input Map al Game Loop

**Estado actual**: `modules/input_map.rs` tiene 500+ líneas pero NO se usa en el game loop

**Tareas**:
1. [ ] Encontrar dónde se debería llamar `input_map::poll_events()`
2. [ ] Conectar con `rydit-gfx::get_input()` o similar
3. [ ] Integrar con game loop en `main.rs` o `executor.rs`
4. [ ] Crear demo que use `input_map::is_pressed("accion")`

**Criterio de éxito**: Demo que responde a 10+ teclas diferentes

---

### Objetivo 3: Fix Carga de Assets

**Estado actual**: `assets::load_texture()` existe pero las texturas no se ven

**Hipótesis**:
- rydit-gfx cambió su API interna
- FFI directo está roto (debería usar RenderQueue)
- Problema de rutas/paths de archivos

**Tareas**:
1. [ ] Investigar `modules/assets.rs` (cómo carga texturas)
2. [ ] Investigar `rydit-gfx/src/lib.rs` (cómo recibe texturas)
3. [ ] Comparar con screenshots/ que muestran que SÍ funcionaba antes
4. [ ] Fixear: o restaurar API antigua o migrar a RenderQueue

**Criterio de éxito**: Sprite visible en demo (tanque.png o similar)

---

### Objetivo 4: Reconectar Parser → Eval

**Problema**: El 98% de las conexiones están hechas, pero ese 2% faltante rompe todo

**Tareas**:
1. [ ] Leer `lizer/src/lib.rs` (parser) - ¿qué devuelve?
2. [ ] Leer `eval/mod.rs` - ¿qué recibe?
3. [ ] Buscar `evaluar_stmt_gfx()` o similar
4. [ ] Trazar: statement → parser → AST → eval → ejecución
5. [ ] Identificar dónde se pierde la conexión

**Criterio de éxito**: Demo .rydit simple funciona (círculo + input)

---

### Objetivo 5: RyditModule Trait → Modules

**Estado actual**: El trait RyditModule existe pero no está implementado en todos los módulos

**Tareas**:
1. [ ] Buscar `trait RyditModule` en `rydit-core` o similar
2. [ ] Listar módulos que NO lo implementan
3. [ ] Implementar trait en cada módulo faltante
4. [ ] Registry carga módulos dinámicamente

**Criterio de éxito**: `ModuleRegistry::load_all()` carga los 12 módulos

---

## 📁 ARCHIVOS A INVESTIGAR (PRIORIDAD)

### Prioridad 🔴 CRÍTICA
1. `crates/rydit-rs/src/eval/mod.rs` - **El 98% faltante**
2. `crates/rydit-rs/src/main.rs` - Game loop actual
3. `crates/rydit-rs/src/modules/input_map.rs` - Input Map (500+ líneas)
4. `crates/rydit-rs/src/modules/assets.rs` - Carga de texturas
5. `crates/rydit-gfx/src/lib.rs` - FFI gráfico

### Prioridad 🟡 ALTA
6. `crates/rydit-rs/src/modules/physics.rs` - Físicas 2D
7. `crates/rydit-rs/src/modules/camera.rs` - Cámara 2D
8. `crates/rydit-rs/src/modules/entity.rs` - Entity System
9. `crates/lizer/src/lib.rs` - Parser
10. `crates/rydit-core/src/lib.rs` - RyditModule trait

### Prioridad 🟢 MEDIA
11-20. Resto de módulos en `modules/`

---

## 🔧 HERRAMIENTAS DE DIAGNÓSTICO

### Comandos que usaré en modo agente

```bash
# 1. Ver estructura de módulos
ls -la crates/rydit-rs/src/modules/

# 2. Buscar conexiones parser → eval
grep -r "evaluar_stmt" crates/rydit-rs/src/

# 3. Buscar uso de input_map
grep -r "input_map::" crates/rydit-rs/src/

# 4. Buscar carga de assets
grep -r "load_texture\|draw_texture" crates/rydit-rs/src/

# 5. Ver game loop
grep -A 20 "fn game_loop\|ryda frame" crates/rydit-rs/src/main.rs

# 6. Ver trait RyditModule
grep -A 10 "trait RyditModule" crates/rydit-core/src/

# 7. Compilar y ver errores
cargo build --release 2>&1 | head -100
```

---

## 📊 MÉTRICAS DE ÉXITO FASE 1

| Métrica | Antes | Después | Objetivo |
|---------|-------|---------|----------|
| **Conexiones parser→eval** | 98% | 100% | ✅ 2% faltante |
| **Input Map integrado** | ❌ No | ✅ Sí | ✅ 10+ teclas |
| **Assets cargando** | ❌ No | ✅ Sí | ✅ 1+ sprite visible |
| **Módulos con trait** | ~50% | 100% | ✅ 12/12 módulos |
| **Demo .rydit funciona** | ⚠️ Parcial | ✅ Completo | ✅ círculo + input + sprite |

---

## ⚠️ RIESGOS Y MITIGACIÓN

### Riesgo 1: Código muy acoplado
**Mitigación**: Refactorizar gradual, no big-bang

### Riesgo 2: Romper algo que funciona
**Mitigación**: Tests después de cada cambio pequeño

### Riesgo 3: No encontrar el bug en 2-3 días
**Mitigación**: Timebox, si no se encuentra → pedir ayuda al usuario

### Riesgo 4: Cambios en rydit-gfx incompatibles
**Mitigación**: O restaurar API antigua o migrar a RenderQueue (plan B)

---

## 📝 ENTREGABLES FASE 1

Al finalizar esta fase, entregaré:

1. ✅ **Diagrama de flujo** parser → eval → modules
2. ✅ **Lista de conexiones rotas** y cómo se arreglaron
3. ✅ **Demo funcional** que usa Input Map + Assets
4. ✅ **Todos los módulos** con trait RyditModule implementado
5. ✅ **Documento de cambios** (qué archivos se modificaron y por qué)

---

## 🚀 SIGUIENTE FASE (Documento 2)

Una vez completada la **Fase 1: Reconexión**, procederé con:

**Fase 2: Platform Sync + rydit-input crate**

Ver documento: `2da_parte_reconexion.md`

---

## ✋ AUTORIZACIÓN REQUERIDA

**Solicito autorización para**:

- [ ] Leer TODOS los archivos listados arriba (15-20 archivos)
- [ ] Ejecutar comandos de diagnóstico (grep, cargo build, etc.)
- [ ] Modificar archivos críticos (eval/mod.rs, main.rs, modules/*.rs)
- [ ] Crear demos de prueba para validar cambios
- [ ] Refactorizar código si es necesario (sin romper funcionalidad)

**Modo de trabajo**: Agente autónomo con reportes cada 2-3 horas

**Señal de inicio**: Usuario responde "autorizado" o "procede"

---

<div align="center">

**🛡️ RyDit v0.10.4 - FASE 1: RECONEXIÓN**

*Parser → Eval → Modules | 2-3 días | 15-20 archivos*

**Próximo: Autorización del usuario**

</div>

---

**Notas para el agente**:
- Este documento es tu hoja de ruta
- Si encuentras algo inesperado, actualiza el plan
- Prioriza: Input Map + Assets primero
- No refactorices de más (solo lo necesario para reconectar)
- Tests pequeños después de cada cambio
