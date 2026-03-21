# 🛣️ Shield Project - Roadmap

**Versión Actual:** v0.0.1 (Pre-Alpha)

Este documento describe el plan de desarrollo del lenguaje RyDit y el ecosistema Shield.

---

## 📍 Versión Actual: v0.0.1 (Pre-Alpha)

### ✅ Completado

- [x] Arquitectura multi-crate funcional
- [x] Compilación en Termux con raylib nativo
- [x] Lexer básico (Lizer) tokeniza scripts
- [x] Motor de audio (BlastCore) responde a tokens
- [x] Wrapper gráfico (VShield) con raylib
- [x] sccache configurado para builds rápidos
- [x] Documentación completa (README + Guía)
- [x] Optimizaciones de RAM para dispositivos < 4GB

### ⚠️ Limitaciones Actuales

- Script hardcodeado en `main.rs` (requiere recompilar para cambios)
- Lexer usa `split_whitespace()` (no parsea estructura real)
- Tokens no se ejecutan, solo imprimen
- Sin manejo de errores real
- Sin tests automatizados

---

## 🎯 Próxima Versión: v0.0.2 (Alpha Temprana)

### Objetivos Principales

1. **CLI Básica** - Ejecutar scripts sin recompilar
2. **Parser Mejorado** - Entender estructura, no solo palabras
3. **Ejecución Real** - Que los tokens hagan algo

### Features Planeadas

```bash
# Pasar script por línea de comandos
cargo run -- "shield.init onda.core"

# Leer desde archivo
cargo run -- script.rydit

# Ver tokens generados
cargo run -- --tokens script.rydit
```

### Criterios de Aceptación

- [ ] Poder cambiar script sin recompilar
- [ ] Parser que entienda `dark.slot x = 100` como asignación
- [ ] BlastCore ejecute acciones reales (no solo prints)
- [ ] Manejo básico de errores de sintaxis

### Tiempo Estimado

1-2 semanas de desarrollo

---

## 🚀 Versión: v0.1.0 (Alpha Pública)

### Objetivos Principales

1. **Lenguaje Funcional** - Scripts ejecutables reales
2. **Documentación Pública** - README para usuarios
3. **Ejemplos Incluidos** - Scripts de demostración

### Features Planeadas

- [ ] Parser completo con AST
- [ ] Variables y expresiones matemáticas
- [ ] Condicionales (`onif` / `blelse`)
- [ ] Funciones básicas (`ryprime`)
- [ ] Sistema de tipos básico
- [ ] Errores descriptivos
- [ ] 5-10 scripts de ejemplo
- [ ] Tests básicos del lexer y parser

### Ejemplo de Script v0.1.0

```rydit
shield.init
dark.slot jugador_vida = 100
dark.slot enemigo = "dragon"

onif jugador_vida > 50
    onda.core
    ryprime ataque = 25
blelse
    dark.slot huir = true
```

### Criterios de Aceptación

- [ ] Scripts complejos ejecutan sin errores
- [ ] Documentación para nuevos usuarios
- [ ] Al menos 100 líneas de tests
- [ ] Binario < 10 MB

### Tiempo Estimado

1-2 meses de desarrollo

---

## 🔮 Versiones Futuras

### v0.2.0 (Beta Temprana)

- [ ] VShield con ventana gráfica funcional
- [ ] Sistema de eventos
- [ ] Integración audio-gráficos
- [ ] REPL interactivo
- [ ] Debugger básico

### v0.3.0 (Beta Pública)

- [ ] Librería estándar básica
- [ ] Sistema de módulos
- [ ] Imports entre archivos
- [ ] Documentación de API
- [ ] Benchmark de rendimiento

### v1.0.0 (Release Candidata)

- [ ] Lenguaje estable
- [ ] API documentada
- [ ] Tests exhaustivos
- [ ] Ejemplos avanzados
- [ ] Comunidad inicial

---

## 📊 Estado por Componente

| Componente | v0.0.1 | v0.0.2 | v0.1.0 | v1.0.0 |
|------------|--------|--------|--------|--------|
| **Lizer (Lexer)** | ✅ Básico | ⚠️ Mejorado | ✅ Completo | ✅ Estable |
| **Parser** | ❌ No existe | ⚠️ Básico | ✅ AST | ✅ Completo |
| **Ejecutor** | ❌ Prints | ⚠️ Acciones | ✅ Real | ✅ Optimizado |
| **BlastCore** | ⚠️ Simulado | ⚠️ Básico | ✅ Funcional | ✅ Completo |
| **VShield** | ⚠️ Stub | ⚠️ Básico | ✅ Ventana | ✅ Gráficos |
| **CLI** | ❌ Hardcodeado | ✅ Básica | ✅ Completa | ✅ Avanzada |
| **Tests** | ❌ No hay | ⚠️ Mínimos | ✅ Básicos | ✅ Exhaustivos |
| **Docs** | ✅ Internas | ⚠️ Básicas | ✅ Usuario | ✅ Completa |

---

## 🤔 Decisiones Pendientes

### 1. Sintaxis del Lenguaje

**Opción A: Estilo actual (palabras clave)**
```
shield.init dark.slot x = 100
```

**Opción B: Estilo tradicional**
```
init
let x = 100
```

**Opción C: Estilo funcional**
```
init()
let(x, 100)
```

> **Decisión pendiente:** Esperando feedback de pruebas tempranas

---

### 2. Sistema de Tipos

**Opción A: Dinámico (como JavaScript)**
```
x = 100
x = "hola"  # Válido
```

**Opción B: Estático (como Rust)**
```
let x: num = 100
x = "hola"  # Error
```

**Opción C: Inferido (como TypeScript)**
```
let x = 100  # Infere num
x = "hola"   # Error
```

> **Decisión pendiente:** Probablemente Opción A para v0.1.0, evaluar después

---

### 3. Ejecución de Gráficos

**Opción A: Immediate mode (raylib clásico)**
```
draw.circle(100, 100, 50, RED)
```

**Opción B: Componentes**
```
component Circle { x: 100, y: 100, r: 50, color: RED }
```

> **Decisión pendiente:** Opción A para v0.1.0 por simplicidad

---

## 📝 Cómo Contribuir a Decisiones

Si estás desarrollando este proyecto, considera:

1. **Probar la v0.0.1** - Entiende la arquitectura actual
2. **Identificar dolores** - ¿Qué es incómodo de usar?
3. **Proponer sintaxis** - ¿Cómo debería verse el código?
4. **Priorizar features** - ¿Qué es más urgente?

---

## 🔄 Actualizaciones del Roadmap

Este documento se actualiza cuando:

- Se completa una versión mayor
- Se toman decisiones de diseño
- Se agregan nuevas features planeadas
- Cambian las prioridades

---

**Última actualización:** 2026-03-14
**Próxima revisión:** Después de v0.0.2
