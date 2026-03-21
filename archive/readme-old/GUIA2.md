# 📘 Shield Project - Guía Práctica de Uso (GUIA2.md)

**Versión:** v0.0.2  
**Nivel:** Usuario final  
**Tiempo de lectura:** 10 minutos

---

## 🚀 **Inicio Rápido**

### 1. Ejecutar tu Primer Script

```bash
# Script directo
cargo run -- -- "shield.init"

# Desde archivo
cargo run -- -- ejemplo.rydit
```

### 2. Crear Variables

```bash
# Números
cargo run -- -- "dark.slot x = 100"
cargo run -- -- "dark.slot delta.flow = 0.5"

# Texto
cargo run -- -- 'dark.slot nombre = "Heroe"'

# Jerarquía
cargo run -- -- "dark.slot jugador.vida = 100"
```

---

## 📖 **Referencia de Comandos**

### Comandos Principales

| Comando | Descripción | Ejemplo |
|---------|-------------|---------|
| `shield.init` | Inicializar | `shield.init` |
| `onda.core` | Acción | `onda.core` |
| `ryprime` | Operación | `ryprime` |

### Variables

| Sintaxis | Tipo | Ejemplo |
|----------|------|---------|
| `dark.slot x = N` | Número | `dark.slot x = 100` |
| `dark.slot x = N.N` | Decimal | `dark.slot x = 0.5` |
| `dark.slot x = "..."` | Texto | `dark.slot x = "Hola"` |

### Condicionales

```rydit
# Si la condición es verdadera (número != 0)
onif x onda.core blelse ryprime

# Sin else
onif x onda.core
```

### Operadores Lógicos

```rydit
# AND (ambos deben ser verdaderos)
onif a and b onda.core

# OR (al menos uno verdadero)
onif a or b onda.core

# NOT (invierte)
onif not x onda.core
```

### Ciclos

```rydit
# Mientras x sea verdadero (x != 0)
ryda x onda.core
```

**Nota:** El body del while es 1 statement. Para múltiples statements, espera bloques `{}` en v0.0.3.

---

## 🧪 **Ejemplos Completos**

### Ejemplo 1: Script Básico

```bash
cargo run -- -- "shield.init dark.slot x = 100 onda.core"
```

**Output:**
```
[SHIELD] Inicializando sistema...
[MEMORIA] x = 100
[BLAST-CORE] Ejecutando: onda.core
```

### Ejemplo 2: Condicionales

```bash
cargo run -- -- "dark.slot x = 10 onif x onda.core blelse ryprime"
```

**Output:**
```
[BLAST-CORE] Ejecutando: onda.core
```

### Ejemplo 3: Operadores Lógicos

```bash
# AND: 1 and 0 = false → ejecuta else
cargo run -- -- "dark.slot a = 1 dark.slot b = 0 onif a and b onda.core blelse ryprime"

# OR: 1 or 0 = true → ejecuta then
cargo run -- -- "dark.slot a = 1 dark.slot b = 0 onif a or b onda.core blelse ryprime"

# NOT: not 0 = true → ejecuta then
cargo run -- -- "dark.slot x = 0 onif not x onda.core blelse ryprime"
```

### Ejemplo 4: Ciclos

```bash
# Loop infinito (hasta 100 iteraciones)
cargo run -- -- "dark.slot x = 1 ryda x onda.core"
```

**Output:**
```
[BLAST-CORE] Ejecutando: onda.core
(repetido 100 veces)
```

---

## 💻 **REPL Interactivo**

### Iniciar

```bash
cargo run -- --repl
```

### Comandos del REPL

```
rydit> help          # Mostrar ayuda
rydit> dark.slot x = 100  # Crear variable
rydit> mem           # Ver memoria
rydit> clear         # Limpiar pantalla
rydit> exit          # Salir
```

### Ejemplo de Sesión

```
=== RYDIT REPL v0.0.2 ===
rydit> dark.slot x = 100
[RYDIT] 1 statements
[MEMORIA] x = 100

rydit> dark.slot y = 0.5
[RYDIT] 1 statements
[MEMORIA] y = 0.5

rydit> mem
=== MEMORIA RYDIT ===
  x = 100
  y = 0.5
=====================

rydit> exit
[REPL] Saliendo...
```

---

## 📁 **Archivos .rydit**

### Crear Script

```bash
cat > mi_script.rydit << 'EOF'
# Mi primer script
shield.init

dark.slot jugador.vida = 100
dark.slot jugador.nombre = "Heroe"

onda.core
ryprime
EOF
```

### Ejecutar

```bash
cargo run -- -- mi_script.rydit
```

### Error: Archivo No Existe

```bash
cargo run -- -- no_existe.rydit
# [ERROR] No se pudo leer el archivo
```

---

## ⚠️ **Manejo de Errores**

### Error: Carácter Inválido

```bash
cargo run -- -- "@#$"
# Los tokens inválidos se ignoran
```

### Error: String Sin Cerrar

```bash
cargo run -- -- '"hola'
# El parser intenta recuperar
```

---

## 🧪 **Tests Automáticos**

```bash
# Ejecutar todos los tests
cargo test

# Output esperado:
# running 22 tests
# test result: ok. 22 passed
```

---

## 📊 **Comandos de Referencia Rápida**

| Comando | Descripción | Tiempo |
|---------|-------------|--------|
| `cargo check` | Verificar errores | 1-5s |
| `cargo build` | Compilar | 3-60s |
| `cargo test` | Tests (22) | 5-10s |
| `cargo run -- -- "..."` | Ejecutar script | 1-5s |
| `cargo run -- --repl` | REPL interactivo | - |
| `cargo run -- -- archivo.rydit` | Desde archivo | 1-5s |

---

## 🎯 **Sintaxis Resumida**

```rydit
# Comentarios
# Esto es un comentario

# Inicializar
shield.init

# Variables
dark.slot x = 100
dark.slot nombre = "Heroe"

# Condicionales
onif x onda.core blelse ryprime

# Operadores
onif a and b onda.core
onif a or b onda.core
onif not x onda.core

# Ciclos
ryda x onda.core
```

---

## 🆘 **FAQ**

### ¿Cómo creo una variable?
```bash
cargo run -- -- "dark.slot x = 100"
```

### ¿Cómo uso condicionales?
```bash
cargo run -- -- "dark.slot x = 10 onif x onda.core"
```

### ¿Los ciclos terminan?
Solo si la condición eventualmente es falsa. Hay un límite de 100 iteraciones por seguridad.

### ¿Puedo usar múltiples statements en while?
No en v0.0.2. El body del while es 1 statement. Bloques `{}` en v0.0.3.

### ¿Cómo veo la memoria?
En REPL: `rydit> mem`

---

## 📈 **Próximas Features (v0.0.3)**

1. **Bloques `{}`** - Múltiples statements en while
2. **Funciones** - `rytmo mi_func() { ... }`
3. **Comparación** - `x > 10`, `x == y`
4. **Mejores errores** - Línea y columna

---

**Última actualización:** 2026-03-14  
**Versión:** v0.0.2
