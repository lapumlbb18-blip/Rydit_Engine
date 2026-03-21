# 🔄 Flujo de Trabajo - Shield Project (RyDit)

**Versión:** v0.0.12 (Diagnóstico Avanzado con Crítica Exigente)
**Última actualización:** 2026-03-16

---

## 📋 Resumen Rápido

```bash
# ==================== FLUJO RÁPIDO (durante desarrollo) ====================
cargo test 2>&1 | grep -E "test result|warning"

# ==================== FLUJO COMPLETO (cada 3-4 cambios) ====================
cargo build 2>&1 | tail -10

# ==================== FLUJO BACKUP (antes de cerrar sesión) ================
cargo build && cargo test
time cargo build
time cargo test
rclone sync ./ alucard18:/shield-project-rydit --exclude 'target/**'
```

---

## 🎯 Flujo Detallado por Etapa

### 1️⃣ INICIO DE SESIÓN

```bash
# Verificar estado del proyecto
cargo check

# Verificar tests rápidos
cargo test 2>&1 | grep -E "test result|passed|failed"

# Verificar sccache (opcional)
sccache --show-stats
```

**Tiempo estimado:** 5-10 segundos

**Qué buscar:**
- ✅ `Finished dev [optimized] target(s)`
- ✅ `test result: ok`
- ✅ Sin errores de compilación

---

### 2️⃣ DURANTE DESARROLLO (Cada cambio pequeño)

```bash
# Test rápido de validación (~1-3s)
cargo test 2>&1 | grep -E "test result|warning"

# Si hay error, ver detalles
cargo test 2>&1 | tail -20
```

**Tiempo estimado:** 1-3 segundos (con sccache)

**Qué buscar:**
- ✅ `test result: ok. X passed; 0 failed`
- ⚠️ Warnings nuevos (anotar para después)
- ❌ Errores de compilación (fix inmediato)

**Acciones:**
| Resultado | Acción |
|-----------|--------|
| ✅ Todo pasa | Continuar codificando |
| ⚠️ Warning nuevo | Anotar, fix en pausa |
| ❌ Error | Fix inmediato, no continuar |

---

### 3️⃣ CADA 3-4 CAMBIOS (Build diagnóstico)

```bash
# Build completo con diagnóstico de warnings (~5-10s)
cargo build 2>&1 | tail -10

# Verificar warnings específicos
cargo build 2>&1 | grep -E "warning|error"
```

**Tiempo estimado:** 5-10 segundos (con sccache)

**Qué buscar:**
- ✅ `Finished dev [optimized] target(s)`
- ⚠️ Lista de warnings (priorizar)
- ❌ Errores de compilación

**Prioridad de Warnings:**

| Prioridad | Tipo | Acción |
|-----------|------|--------|
| 🔴 Alta | `error[E...]` | Fix inmediato |
| 🟡 Media | `unused_variables`, `unused_mut` | Fix en pausa |
| 🟢 Baja | `dead_code`, `mismatched_lifetime` | Documentar, fix después |

---

### 4️⃣ MANEJO DE ERRORES

```bash
# Ver error completo
cargo build 2>&1 | grep -A 10 "error"

# Ver línea específica del error
cargo build 2>&1 | grep -A 5 "error\[E"

# Si es error de tipo
cargo check 2>&1 | grep "expected\|found"
```

**Tipos comunes de errores:**

| Error | Causa | Solución |
|-------|-------|----------|
| `cannot find value` | Variable no declarada | Agregar `dark.slot` |
| `expected type` | Tipo incorrecto | Revisar expresión |
| `borrowed value does not live long` | Lifetime issue | Revisar referencias |
| `no method named` | Método inexistente | Verificar API |

---

### 5️⃣ BREAKS ESTRATÉGICOS (Función Clave)

**Importante:** Los breaks en momentos críticos previenen errores y burnout.

#### 🚨 NIVELES DE BREAK

| Nivel | Cuándo | Duración | Señal |
|-------|--------|----------|-------|
| 🔴 **CRÍTICO** | Error de compilación no resuelto en 15min | 10-15 min | "Mismo error 3+ veces" |
| 🟡 **IMPORTANTE** | Feature completada exitosamente | 5-10 min | "Build + Tests OK" |
| 🟢 **MÍNIMO** | Cada 3-4 cambios de código | 2-3 min | "Cansancio visual" |

---

#### 🔴 BREAK CRÍTICO (10-15 min)

**Se activa cuando:**
- ❌ Mismo error de compilación 3+ veces
- ❌ Bug no entendido después de 15 min
- ❌ Frustración creciente (tecleo agresivo)
- ❌ No ves lo obvio (error de sintaxis simple)

**Qué hacer:**
```bash
# 1. Guardar cambios
git add .  # O backup parcial

# 2. Anotar estado actual
echo "ERROR: [descripción breve]" >> diagnostico/BREAKS.txt
echo "INTENTOS: 3+" >> diagnostico/BREAKS.txt
echo "PAUSA: $(date)" >> diagnostico/BREAKS.txt

# 3. Alejarse de la pantalla
# - Caminar por la habitación
# - Estirar piernas y brazos
# - Respirar profundo 5 veces
# - Beber agua

# 4. Al regresar (mente fresca):
# - Leer error con calma
# - Usar cargo check --verbose
# - Pedir ayuda si persiste
```

**Ejemplo Real (v0.0.7 rydit-gfx):**
```
PROBLEMA: raylib símbolos no encontrados
INTENTOS: 5+ builds fallidos
BREAK: 15 min caminando
SOLUCIÓN: build.rs con pkg-config (obvio después del break)
```

---

#### 🟡 BREAK IMPORTANTE (5-10 min)

**Se activa cuando:**
- ✅ Feature completada (build + tests OK)
- ✅ Integración exitosa
- ✅ Bug complejo resuelto
- ✅ Sesión > 1 hora sin pausa

**Qué hacer:**
```bash
# 1. Celebrar logro
echo "✅ FEATURE: [nombre] completada - $(date)" >> diagnostico/LOGROS.txt

# 2. Commit mental (o real)
# - Anotar qué funcionó
# - Anotar lecciones aprendidas

# 3. Descanso activo:
# - Estirar cuello y hombros
# - Mirar a lo lejos (20-20-20 rule)
# - Snack ligero si hay hambre

# 4. Preparar siguiente tarea:
# - Revisar checklist
# - Anotar próximo objetivo
```

**Ejemplo Real (v0.0.8 integración):**
```
LOGRO: rydit-gfx integrado con rydit-rs
BUILD: ✅ 0 errores
TESTS: ✅ 51 pasando
BREAK: 10 min (estirar + agua)
SIGUIENTE: Documentación de sesión
```

---

#### 🟢 BREAK MÍNIMO (2-3 min)

**Se activa cuando:**
- ⏰ Cada 3-4 cambios de código
- 👀 Cansancio visual leve
- 🤔 Necesitas perspectiva fresca

**Qué hacer:**
```bash
# 1. Pausa micro:
# - Cerrar ojos 5 segundos
# - Parpadear 10 veces (sequedad ocular)
# - Rotar cuello (3 círculos cada lado)

# 2. Respiración:
# - Inhalar 4 segundos
# - Retener 4 segundos
# - Exhalar 4 segundos
# - Repetir 3 veces

# 3. Continuar con mente fresca
```

**Regla 20-20-20 (Cada 20 min):**
- Mirar objeto a **20 pies** (6 metros)
- Durante **20 segundos**
- Parpadear **20 veces**

---

#### 📊 INTEGRACIÓN EN FLUJO DE TRABAJO

```
┌─────────────────────────────────────────────────────────────┐
│              FLUJO CON BREAKS ESTRATÉGICOS                   │
├─────────────────────────────────────────────────────────────┤
│  1. Inicio: cargo check + cargo test (5s)                   │
│  2. Codificar → test rápido (2s) → repetir (x3-4)           │
│  3. 🟢 BREAK MÍNIMO (2-3 min) ← Cansancio visual            │
│  4. Cada 3-4 cambios: cargo build (10s)                     │
│  5. 🟡 BREAK IMPORTANTE (5-10 min) ← Feature completa       │
│  6. Feature completa: build + tests + benchmark (30s)       │
│  7. 🔴 BREAK CRÍTICO (si aplica) ← Error persistente        │
│  8. Documentación: diagnostico/*.txt (10min)                │
│  9. 🟡 BREAK IMPORTANTE (5 min) ← Sesión completada         │
│ 10. Final: backup + SESSION_STATE.md (60s)                  │
└─────────────────────────────────────────────────────────────┘
```

---

#### 🧠 CIENCIA DETRÁS DE LOS BREAKS

**Por qué funcionan:**

1. **Modo Difuso del Cerebro:**
   - Durante el break, el cerebro procesa en background
   - Soluciones "emergen" después de descansar
   - Ejemplo: Baño, ducha, caminar = ideas claras

2. **Prevención de Tunnel Vision:**
   - Enfoque prolongado → ceguera de contexto
   - Break resetea perspectiva
   - Errores obvios se vuelven visibles

3. **Memoria y Consolidación:**
   - Breaks permiten consolidar aprendizaje
   - Lo codificado se fija en memoria a largo plazo
   - Próxima sesión: más rápido

---

#### 📝 REGISTRO DE BREAKS (Opcional pero Recomendado)

```bash
# Crear archivo de tracking
cat >> diagnostico/BREAKS.txt << 'EOF'
================================================================================
                    REGISTRO DE BREAKS - v0.0.X
================================================================================

FECHA: $(date +%Y-%m-%d)

BREAK #1 | 🔴 CRÍTICO | 10:30 AM | 15 min
Motivo: Error de compilación (lifetime en DrawHandle)
Actividad: Caminata por la casa
Solución después del break: Agregar<'_> en signature ✅

BREAK #2 | 🟡 IMPORTANTE | 11:45 AM | 10 min
Motivo: rydit-gfx integrado exitosamente
Actividad: Estiramientos + agua
Siguiente: Documentación

BREAK #3 | 🟢 MÍNIMO | 12:30 PM | 3 min
Motivo: Cansancio visual (2 horas de código)
Actividad: Regla 20-20-20
Continuar: Tests de integración

================================================================================
EOF
```

**Beneficios del registro:**
- Patrones de errores recurrentes
- Tiempos óptimos de break personales
- Mejora continua del flujo

---

#### ⚠️ SEÑALES DE ALERTA (BREAK INMEDIATO)

**Si experimentas 2+ de estos, BREAK CRÍTICO YA:**

- [ ] Dolor de cabeza leve
- [ ] Irritabilidad con el código
- [ ] Mismo error 5+ veces
- [ ] No puedes leer lo que escribiste
- [ ] Postura encorvada por 1+ hora
- [ ] Ojos secos/rojos
- [ ] Hambre pero ignorando

**Acción:**
```bash
echo "🚨 ALERTA: Break crítico necesario - $(date)" >> diagnostico/BREAKS.txt
# Cerrar editor inmediatamente
# Alejarse de la pantalla
# 15 min mínimo de descanso
```

---

#### 🎯 RECOMENDACIÓN PERSONALIZADA (v0.0.8+)

**Para desarrollo de RyDit:**

| Tipo de Sesión | Breaks Sugeridos |
|----------------|------------------|
| Integración nueva (rydit-gfx) | 🟡 Cada 45 min, 🔴 si error > 15min |
| Tests automáticos | 🟢 Cada 10 tests, 🟡 cada 20 tests |
| Documentación | 🟢 Cada 30 min, 🟡 cada documento |
| Debugging complejo | 🔴 Cada 15 min si atascado |
| Feature nueva | 🟡 Al completar, 🟢 durante desarrollo |

---

### 6️⃣ MANEJO DE WARNINGS

```bash
# Listar todos los warnings
cargo build 2>&1 | grep "warning:"

# Contar warnings
cargo build 2>&1 | grep -c "warning:"

# Ver warning específico con contexto
cargo build 2>&1 | grep -A 3 "warning:"
```

**Estrategia de fix:**

1. **Durante desarrollo:** Ignorar warnings no críticos
2. **Cada 3-4 cambios:** Fix warnings de media prioridad
3. **Antes de backup:** Fix o documentar warnings restantes

**Warnings conocidos (v0.0.8):**
- `dead_code` (width, height en RyditGfx) - 🟢 Baja
- `mismatched_lifetime_syntaxes` (begin_draw) - 🟢 Baja

---

### 7️⃣ DOCUMENTACIÓN DE SESIÓN (Antes de Backup)

**Importante:** Esta tarea se realiza ANTES de finalizar sesión.

```bash
# ==================== CREAR ARCHIVOS DE DIAGNÓSTICO ====================

# 1. Sesión completada (resumen de la sesión)
cat > diagnostico/SESION_v0.0.X_*.txt << 'EOF'
================================================================================
                    SESIÓN COMPLETADA - v0.0.X [NOMBRE]
================================================================================
FECHA: $(date +%Y-%m-%d)
TIEMPO TOTAL: ~X horas
ESTADO: ✅ COMPLETADA

LOGROS:
-------
✅ [Logro 1]
✅ [Logro 2]

MÉTRICAS:
---------
TESTS: X pasando
BUILD: X.XXs

ARCHIVOS MODIFICADOS:
---------------------
- archivo1.rs
- archivo2.toml
================================================================================
EOF

# 2. Diagnóstico Real (estado emocional y realista)
# Editar: diagnostico/DIAGNOSTICO_REAL_$(date +%Y-%m-%d).txt

# 3. Diagnóstico Avanzado (análisis técnico)
# Editar: diagnostico/DIAGNOSTICO_AVANZADO_$(date +%Y-%m-%d).txt

# 4. Contexto próximas versiones (planificación)
# Editar: diagnostico/CONTEXTO_PROXIMAS_VERSIONES.txt
```

---

### 7.1️⃣ DIAGNÓSTICO AVANZADO CON CRÍTICA EXIGENTE (NUEVO - v0.0.12+)

**⚠️ IMPORTANTE:** Este documento es **ESENCIAL** para:
- ✅ Ver con claridad el estado real del proyecto
- ✅ Anticipar problemas futuros
- ✅ Mantener contexto en tiempo real entre sesiones
- ✅ Mejora continua de la calidad del código

**Propósito:** El diagnóstico avanzado con crítica exigente te ayuda a ver el proyecto con objetividad, identificar debilidades antes de que sean críticas, y mantener un registro honesto del progreso.

```bash
# ==================== DIAGNÓSTICO AVANZADO CON CRÍTICA ====================

cat > diagnostico/DIAGNOSTICO_AVANZADO_v0.0.X_ANALISIS_CRITICO.txt << 'EOF'
================================================================================
                    DIAGNÓSTICO AVANZADO v0.0.X - ANÁLISIS CRÍTICO
                    Shield Project - RyDit Language
================================================================================

FECHA: $(date +%Y-%m-%d)
VERSIÓN: v0.0.X
ESTADO: ✅ [Estado actual]

================================================================================
                         🐛 ERRORES/BUGS DETECTADOS
================================================================================

CUÁNDO OCURRIÓ:
---------------
[Describir cuándo y cómo ocurrió el error]

SÍNTOMAS:
---------
1. [Síntoma 1]
2. [Síntoma 2]

CAUSA RAÍZ TÉCNICA:
-------------------
[Explicación técnica detallada de la causa raíz]

================================================================================
                         ⚡ REACCIÓN VERSÁTIL
================================================================================

FASE 1: DIAGNÓSTICO (X minutos)
-------------------------------
- [Pasos de diagnóstico]

FASE 2: ANÁLISIS (X minutos)
----------------------------
- [Análisis realizado]

FASE 3: SOLUCIÓN (X minutos)
----------------------------
- [Solución implementada]

FASE 4: PREVENCIÓN (X minutos)
------------------------------
- [Tests de regresión agregados]
- [Documentación actualizada]

TIEMPO TOTAL: ~X minutos

================================================================================
                         🔧 CAMBIOS/CORRECCIONES
================================================================================

CAMBIO #1: [Nombre del cambio]
------------------------------
ARCHIVO: ruta/al/archivo.rs

PROBLEMA:
[Describir problema]

SOLUCIÓN:
[Describir solución]

IMPACTO:
- [Impacto 1]
- [Impacto 2]

================================================================================
                         📊 ANÁLISIS HONESTO Y EXIGENTE
================================================================================

FORTALEZAS DEL PROYECTO:
------------------------

1. **[FORTALEZA 1]** ⭐⭐⭐⭐⭐
   [Descripción detallada]

2. **[FORTALEZA 2]** ⭐⭐⭐⭐
   [Descripción detallada]

DEBILIDADES DEL PROYECTO:
-------------------------

1. **[DEBILIDAD 1]** ⭐⭐
   [Descripción detallada]
   IMPACTO: [Impacto en el proyecto]

2. **[DEBILIDAD 2]** ⭐⭐⭐
   [Descripción detallada]
   SOLUCIÓN FUTURA: [Cómo se planea fixear]

AMENAZAS TÉCNICAS:
------------------

1. **[AMENAZA 1]**
   PROBABILIDAD: [Alta/Media/Baja]
   MITIGACIÓN: [Cómo prevenir]

OPORTUNIDADES:
--------------

1. **[OPORTUNIDAD 1]**
   [Descripción]
   IMPACTO POTENCIAL: [Alto/Medio/Bajo]

================================================================================
                         🌍 COMPARACIÓN TÉCNICA
================================================================================

RYDIT vs [LENGUAJE SIMILAR]:
----------------------------
| Característica | RyDit v0.0.X | [Lenguaje] |
|----------------|--------------|------------|
| **Feature 1**  | Estado       | Estado     |
| **Feature 2**  | Estado       | Estado     |

VEREDICTO: [Análisis honesto de la comparación]

================================================================================
                         🏆 NIVEL DEL PROYECTO RYDIT
================================================================================

CLASIFICACIÓN ACTUAL: **[Pre-Alpha / Alpha / Beta]**

NIVEL DE MADUREZ:
-----------------
| Componente | Nivel | Comentario |
|------------|-------|------------|
| **Lexer** | ⭐⭐⭐⭐ | [Comentario] |
| **Parser** | ⭐⭐⭐⭐ | [Comentario] |
| **Executor** | ⭐⭐⭐ | [Comentario] |

COMPARACIÓN CON ESTÁNDARES DE LA INDUSTRIA:
-------------------------------------------

**Para un proyecto personal:**
- ✅ [Fortaleza]
- ⚠️ [Debilidad]

**Para un proyecto open-source público:**
- ⚠️ [Falta]
- ❌ [Muy lejos]

**Para producción:**
- ❌ [Limitación crítica]

NIVEL GENERAL: **v0.0.X - [Estado]**

[Análisis honesto del nivel actual]

POTENCIAL:
----------
- **Como herramienta personal:** ⭐⭐⭐⭐⭐
- **Como proyecto open-source:** ⭐⭐⭐
- **Como producto comercial:** ⭐
- **Como proyecto de aprendizaje:** ⭐⭐⭐⭐⭐

================================================================================
                         📋 LECCIONES APRENDIDAS
================================================================================

1. **[LECCIÓN 1]**
   [Descripción de la lección]
   APLICACIÓN FUTURA: [Cómo aplicar en el futuro]

2. **[LECCIÓN 2]**
   [Descripción de la lección]

================================================================================
                         📊 MÉTRICAS FINALES v0.0.X
================================================================================

CÓDIGO:
- Líneas totales: ~XXXX+
- Crates: X
- Tests: XX
- Warnings: X
- Build (caché): X.Xs
- RAM runtime: ~XX MB

DOCUMENTACIÓN:
- README.md: ✅ Actualizado
- SESSION_STATE.md: ✅ Actualizado
- Diagnósticos: ✅ X+ archivos
- Contexto próxima sesión: ✅ Creado

BACKUP:
- Google Drive: [ruta]
- Última sync: $(date +%Y-%m-%d)
- Tamaño: ~XX KB (sin target/)

================================================================================
                         💭 REFLEXIÓN FINAL
================================================================================

LO QUE SE LOGRÓ EN ESTA SESIÓN:
-------------------------------
- [Logro 1]
- [Logro 2]

LO QUE FALTA:
-------------
- [Pendiente 1]
- [Pendiente 2]

¿VALE LA PENA CONTINUAR?
------------------------
**SÍ**, porque:
1. [Razón 1]
2. [Razón 2]

**PERO**, con condiciones:
1. [Condición 1]
2. [Condición 2]

================================================================================
                         FIN DEL DIAGNÓSTICO AVANZADO v0.0.X
================================================================================

PRÓXIMA REVISIÓN: v0.0.X+1
ESTADO TÉCNICO: [SÓLIDO / ESTABLE / EN DESARROLLO]
DEUDAS TÉCNICAS: X (todas no críticas / X críticas)
PROBABILIDAD DE ÉXITO: XX%

================================================================================
EOF
```

**Tiempo estimado:** 15-20 minutos

**Criterio de éxito:**
- ✅ Diagnóstico avanzado creado con crítica exigente
- ✅ Errores/bugs documentados con causa raíz
- ✅ Fortalezas y debilidades listadas honestamente
- ✅ Comparación técnica con otros lenguajes
- ✅ Nivel del proyecto evaluado
- ✅ Lecciones aprendidas registradas
- ✅ Métricas finales actualizadas

---

### 7.2️⃣ ARCHIVOS DE DIAGNÓSTICO A CREAR/ACTUALIZAR

| Archivo | Propósito | Cuándo |
|---------|-----------|--------|
| `SESION_v0.0.X_*.txt` | Resumen de sesión | Cada sesión |
| `DIAGNOSTICO_AVANZADO_v0.0.X_ANALISIS_CRITICO.txt` | **Análisis crítico exigente** | **Cada sesión** |
| `DIAGNOSTICO_REAL_*.txt` | Estado realista | Cada sesión |
| `CONTEXTO_PROXIMAS_VERSIONES.txt` | Planificación | Cada versión |
| `BINDINGS_*.txt` | Documentación técnica | Cuando aplique |

**Contenido mínimo de cada archivo:**

1. **SESION_v0.0.X_*.txt:**
   - Fecha y tiempo total
   - Logros de la sesión
   - Métricas (tests, build time)
   - Archivos modificados

2. **DIAGNOSTICO_AVANZADO_v0.0.X_ANALISIS_CRITICO.txt (NUEVO - IMPORTANTE):**
   - 🐛 Errores/bugs detectados con causa raíz
   - ⚡ Reacción versátil (fases de diagnóstico/solución)
   - 🔧 Cambios/correcciones detallados
   - 📊 **Análisis honesto y exigente** (fortalezas, debilidades, amenazas, oportunidades)
   - 🌍 **Comparación técnica** con lenguajes similares
   - 🏆 **Nivel del proyecto** (clasificación honesta)
   - 📋 Lecciones aprendidas
   - 📊 Métricas finales
   - 💭 Reflexión final

3. **DIAGNOSTICO_REAL_*.txt:**
   - Lo que se logró (hechos, no drama)
   - Comparativa con versiones anteriores
   - Proyección realista
   - Riesgos reales (no dramáticos)
   - Fortalezas reales

4. **CONTEXTO_PROXIMAS_VERSIONES.txt:**
   - Estado actual
   - Próximas versiones (v0.0.X+1, v0.0.X+2)
   - Timeline estimado
   - Plan de tests
   - Documentación pendiente
   - Riesgos y mitigación

**Tiempo estimado:** 15-20 minutos

**Criterio de éxito:**
- ✅ 5 archivos de diagnóstico creados/actualizados
- ✅ **Diagnóstico avanzado con crítica exigente completo**
- ✅ Información consistente entre archivos
- ✅ Próxima sesión claramente definida

---

### 8️⃣ ANTES DE BACKUP (Flujo completo)

```bash
# ==================== BUILD COMPLETO ====================
echo "=== BUILD COMPLETO ==="
cargo build 2>&1 | grep -E "warning|error|Finished"

# ==================== TESTS COMPLETOS ====================
echo "=== TESTS ==="
cargo test 2>&1 | grep "test result"

# ==================== BENCHMARK ====================
echo "=== BENCHMARK ==="
echo "Build time:"
time cargo build 2>&1 | grep "Finished"

echo "Test time:"
time cargo test 2>&1 | grep "test result"

# ==================== VERIFICAR ESTADO ====================
echo "=== ESTADO FINAL ==="
cargo test 2>&1 | grep -E "test result|passed|failed"
```

**Tiempo estimado:** 15-30 segundos

**Criterio de éxito:**
- ✅ `Finished dev [optimized] target(s)`
- ✅ `test result: ok. X passed; 0 failed`
- ✅ Warnings documentados o fixeados

---

### 9️⃣ BACKUP Y SINCRONIZACIÓN

```bash
# ==================== BACKUP A GOOGLE DRIVE ====================
rclone sync ./ alucard18:/shield-project-rydit --exclude 'target/**'

# ==================== VERIFICAR BACKUP ====================
rclone lsd alucard18:/shield-project-rydit

# ==================== CONTAR ARCHIVOS ====================
rclone ls alucard18:/shield-project-rydit | wc -l
```

**Tiempo estimado:** 30-60 segundos (depende de conexión)

**Qué se sincroniza:**
- ✅ Todo el código fuente (`crates/`)
- ✅ Documentación (`docs/`, `*.md`, `*.txt`)
- ✅ Scripts (`scripts/`)
- ✅ Ejemplos (`*.rydit`)
- ❌ `target/**` (excluido por `.rcloneignore`)

---

## 📊 Flujo Completo en un Solo Script

```bash
#!/bin/bash
# Flujo completo de trabajo - Shield Project

echo "🔍 === INICIO DE SESIÓN ==="
cargo check

echo "🧪 === TESTS RÁPIDOS ==="
cargo test 2>&1 | grep -E "test result|warning"

echo "🔨 === BUILD COMPLETO ==="
cargo build 2>&1 | grep -E "warning|error|Finished"

echo "📊 === BENCHMARK ==="
echo "Build time:"
time cargo build 2>&1 | grep "Finished"
echo "Test time:"
time cargo test 2>&1 | grep "test result"

echo "💾 === BACKUP ==="
rclone sync ./ alucard18:/shield-project-rydit --exclude 'target/**'
echo "✅ Backup completado"

echo "📈 === RESUMEN ==="
cargo test 2>&1 | grep "test result"
```

---

## 🎯 Flujo para Integración v0.0.8

### Secuencia Recomendada

```bash
# 1. Testear rydit-gfx independiente
cd crates/rydit-gfx && cargo test

# 2. Agregar dependencia a rydit-rs
# Editar crates/rydit-rs/Cargo.toml

# 3. Testear integración
cd ../.. && cargo check

# 4. Build completo
cargo build 2>&1 | tail -10

# 5. Tests completos
cargo test 2>&1 | grep "test result"

# 6. Ejecutar ejemplo
cargo run -- -- "shield.init"

# 7. Benchmark
time cargo build
time cargo test

# 8. Backup
rclone sync ./ alucard18:/shield-project-rydit --exclude 'target/**'
```

---

## 📋 Checklist por Sesión

### Al Inicio
- [ ] `cargo check` pasa
- [ ] `cargo test` pasa
- [ ] Sin errores de compilación

### Durante Desarrollo (cada cambio)
- [ ] `cargo test` rápido pasa
- [ ] Anotar warnings nuevos

### Cada 3-4 Cambios
- [ ] `cargo build` completo
- [ ] Revisar warnings acumulados
- [ ] Fix warnings de media prioridad

### Al Final (antes de backup)
- [ ] `cargo build` sin errores
- [ ] `cargo test` todos pasan
- [ ] Warnings documentados o fixeados
- [ ] **Documentación de sesión creada** (diagnostico/*.txt)
- [ ] Benchmark ejecutado
- [ ] Backup completado
- [ ] Estado documentado en `diagnostico/` y `SESSION_STATE.md`

---

## 🔧 Comandos de Diagnóstico

```bash
# Ver estado de sccache
sccache --show-stats

# Limpiar caché (si hay problemas)
sccache --stop-server
sccache --start-server

# Build limpio completo
cargo clean && cargo build

# Ver tamaño de target
du -sh target/

# Contar líneas de código
find crates -name "*.rs" | xargs wc -l

# Ver tests pasando
cargo test 2>&1 | grep "test result"
```

---

## 📊 Métricas de Referencia (v0.0.8)

| Métrica | Valor |
|---------|-------|
| Build (caché) | 0.33s ⚡ |
| Build (sin caché) | 60-90s |
| Tests (51) | 1.6s ⚡ |
| RAM build | ~2 GB |
| RAM runtime | ~10 MB |
| Warnings activos | 2 (baja) |
| Crates | 5 ✅ |
| Tests | 48 ✅ |

---

### 🔟 SOLUCIÓN DE PROBLEMAS

### Problema: Build lento

```bash
# Verificar sccache
sccache --show-stats

# Reiniciar sccache
sccache --stop-server && sccache --start-server

# Rebuild limpio
cargo clean && cargo build
```

### Problema: Tests fallan

```bash
# Ver detalles del fallo
cargo test 2>&1 | grep -A 10 "FAILED"

# Ejecutar test específico
cargo test nombre_del_test

# Ejecutar crate específico
cargo test -p lizer
```

### Problema: Backup falla

```bash
# Verificar conexión
rclone lsd alucard18:

# Verificar configuración
rclone config show alucard18

# Backup manual con verbose
rclone sync ./ alucard18:/shield-project-rydit --exclude 'target/**' -v
```

---

## 📝 Notas Importantes

1. **Nunca hacer `cargo clean`** antes de backup (pierdes caché de sccache)
2. **Siempre ejecutar `cargo test`** antes de backup
3. **Documentar warnings** en README si no se fixean
4. **Backup después de cada feature** completada
5. **Familia primero** - pausas cada 1-2 horas

---

## 🌟 Flujo Equilibrado (Recomendado)

```
┌─────────────────────────────────────────────────────────────┐
│              SESIÓN DE DESARROLLO CON BREAKS                 │
├─────────────────────────────────────────────────────────────┤
│  1. Inicio: cargo check + cargo test (5s)                   │
│  2. Codificar → test rápido (2s) → repetir (x3-4)           │
│  3. 🟢 BREAK MÍNIMO (2-3 min) ← Cansancio visual            │
│  4. Cada 3-4 cambios: cargo build (10s)                     │
│  5. 🟡 BREAK IMPORTANTE (5-10 min) ← Feature completa       │
│  6. Feature completa: build + tests + benchmark (30s)       │
│  7. 🔴 BREAK CRÍTICO (si aplica) ← Error persistente        │
│  8. Documentación: diagnostico/*.txt (10min)                │
│  9. 🟡 BREAK IMPORTANTE (5 min) ← Sesión completada         │
│ 10. Final: backup + SESSION_STATE.md (60s)                  │
└─────────────────────────────────────────────────────────────┘
```

**Leyenda:**
- 🟢 BREAK MÍNIMO: 2-3 min (respiración, ojos)
- 🟡 BREAK IMPORTANTE: 5-10 min (estirar, agua)
- 🔴 BREAK CRÍTICO: 10-15 min (caminar, despejar)

---

**Fin del documento de flujo de trabajo**

**Última actualización:** v0.0.8 (2026-03-16) - Breaks estratégicos agregados
**Próxima actualización:** v0.0.9 (después de Snake game)
