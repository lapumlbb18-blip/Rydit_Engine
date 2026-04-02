# 🧪 Tests de RyDit v0.0.2

**Versión:** v0.0.2  
**Fecha:** 2026-03-14  
**Estado:** Parser AST + Condicionales + Operadores Lógicos + Ciclos

---

## ✅ **Feature 1: Parser con AST**

### Test 1.1: Script básico
```bash
cargo run -- -- "shield.init"
```
**Esperado:**
```
[RYDIT] 1 statements en AST
[SHIELD] Inicializando sistema...
```

### Test 1.2: Múltiples statements
```bash
cargo run -- -- "shield.init dark.slot x = 100 onda.core"
```
**Esperado:**
```
[RYDIT] 3 statements en AST
[SHIELD] Inicializando sistema...
[MEMORIA] x = 100
[BLAST-CORE] Ejecutando: onda.core
```

---

## ✅ **Feature 2: Variables y Memoria**

### Test 2.1: Variable numérica
```bash
cargo run -- -- "dark.slot x = 100"
```
**Esperado:**
```
[MEMORIA] x = 100
```

### Test 2.2: Variable decimal
```bash
cargo run -- -- "dark.slot delta.flow = 0.5"
```
**Esperado:**
```
[MEMORIA] delta.flow = 0.5
```

### Test 2.3: Variable texto
```bash
cargo run -- -- 'dark.slot nombre = "Heroe"'
```
**Esperado:**
```
[MEMORIA] nombre = Heroe
```

### Test 2.4: Jerarquía con puntos
```bash
cargo run -- -- "dark.slot jugador.vida = 100"
```
**Esperado:**
```
[MEMORIA] jugador.vida = 100
```

### Test 2.5: Múltiples variables
```bash
cargo run -- -- "dark.slot a = 1 dark.slot b = 2 dark.slot c = 3"
```
**Esperado:**
```
[MEMORIA]
  a = 1
  b = 2
  c = 3
```

---

## ✅ **Feature 3: Condicionales (onif/blelse)**

### Test 3.1: Condicional verdadero
```bash
cargo run -- -- "dark.slot x = 10 onif x onda.core blelse ryprime"
```
**Esperado:**
```
[BLAST-CORE] Ejecutando: onda.core
```

### Test 3.2: Condicional falso
```bash
cargo run -- -- "dark.slot x = 0 onif x onda.core blelse ryprime"
```
**Esperado:**
```
[BLAST-CORE] Ejecutando: ryprime
```

### Test 3.3: Condicional sin else
```bash
cargo run -- -- "dark.slot x = 10 onif x onda.core"
```
**Esperado:**
```
[BLAST-CORE] Ejecutando: onda.core
```

---

## ✅ **Feature 4: Operadores Lógicos**

### Test 4.1: AND (verdadero)
```bash
cargo run -- -- "dark.slot a = 1 dark.slot b = 1 onif a and b onda.core blelse ryprime"
```
**Esperado:**
```
[BLAST-CORE] Ejecutando: onda.core
```

### Test 4.2: AND (falso)
```bash
cargo run -- -- "dark.slot a = 1 dark.slot b = 0 onif a and b onda.core blelse ryprime"
```
**Esperado:**
```
[BLAST-CORE] Ejecutando: ryprime
```

### Test 4.3: OR (verdadero)
```bash
cargo run -- -- "dark.slot a = 1 dark.slot b = 0 onif a or b onda.core blelse ryprime"
```
**Esperado:**
```
[BLAST-CORE] Ejecutando: onda.core
```

### Test 4.4: OR (falso)
```bash
cargo run -- -- "dark.slot a = 0 dark.slot b = 0 onif a or b onda.core blelse ryprime"
```
**Esperado:**
```
[BLAST-CORE] Ejecutando: ryprime
```

### Test 4.5: NOT
```bash
cargo run -- -- "dark.slot x = 0 onif not x onda.core blelse ryprime"
```
**Esperado:**
```
[BLAST-CORE] Ejecutando: onda.core
```

---

## ✅ **Feature 5: Ciclos (ryda)**

### Test 5.1: While simple (loop infinito con límite)
```bash
cargo run -- -- "dark.slot x = 1 ryda x onda.core"
```
**Esperado:**
```
[BLAST-CORE] Ejecutando: onda.core
(repetido 100 veces - límite de seguridad)
```

### Test 5.2: While con condición externa
```bash
cargo run -- -- "dark.flag = 1 ryda dark.flag shield.init"
```
**Esperado:**
```
[SHIELD] Inicializando sistema...
(repetido 100 veces)
```

---

## ✅ **Feature 6: Archivos .rydit**

### Test 6.1: Ejecutar desde archivo
```bash
cargo run -- -- ejemplo.rydit
```
**Esperado:**
```
[RYDIT] Parseando: # Mi primer script RyDit
[RYDIT] X statements en AST
[MEMORIA] delta.flow = 0.5
...
```

### Test 6.2: Archivo no existe
```bash
cargo run -- -- no_existe.rydit
```
**Esperado:**
```
[ERROR] No se pudo leer el archivo 'no_existe.rydit': No such file or directory
```

---

## ✅ **Feature 7: REPL Interactivo**

### Test 7.1: Iniciar REPL
```bash
cargo run -- --repl
```
**Esperado:**
```
=== RYDIT REPL v0.0.2 ===
Escribe comandos RyDit y presiona Enter
Comandos: 'help', 'mem', 'clear', 'exit'

rydit>
```

### Test 7.2: Comando help
```
rydit> help
```
**Esperado:**
```
Comandos RyDit:
  shield.init          - Inicializar sistema
  onda.core            - Ejecutar acción
  ryprime              - Operación especial
  dark.slot x = N      - Crear variable
  onif x > 0 ... blelse - Condicional
```

### Test 7.3: Crear variable
```
rydit> dark.slot x = 100
```
**Esperado:**
```
[RYDIT] 1 statements
[MEMORIA] x = 100
```

### Test 7.4: Ver memoria
```
rydit> mem
```
**Esperado:**
```
=== MEMORIA RYDIT ===
  x = 100
=====================
```

### Test 7.5: Salir
```
rydit> exit
```
**Esperado:**
```
[REPL] Saliendo...
```

---

## ✅ **Feature 8: Manejo de Errores**

### Test 8.1: Carácter inválido
```bash
cargo run -- -- "@#$"
```
**Esperado:**
```
[ERROR] Carácter '@' no reconocido (col 1)
⚠️  1 error(s) encontrado(s)
```

### Test 8.2: String sin cerrar
```bash
cargo run -- -- '"hola'
```
**Esperado:**
```
[ERROR] String sin cerrar en línea 1, columna 1
⚠️  1 error(s) encontrado(s)
```

---

## 📊 **Resumen de Tests**

| Feature | Tests | Estado |
|---------|-------|--------|
| Parser AST | 2 | ✅ |
| Variables | 5 | ✅ |
| Condicionales | 3 | ✅ |
| Operadores Lógicos | 5 | ✅ |
| Ciclos | 2 | ⚠️ (limitado a 1 statement) |
| Archivos .rydit | 2 | ✅ |
| REPL | 5 | ✅ |
| Errores | 2 | ✅ |

**Total:** 26 tests  
**Pasando:** 26  
**Fallando:** 0

---

## 🎯 **Comandos de Test Rápido**

```bash
# Test 1: Parser + Variables
cargo run -- -- "shield.init dark.slot x = 100 onda.core"

# Test 2: Condicionales
cargo run -- -- "dark.slot x = 10 onif x onda.core blelse ryprime"

# Test 3: Operadores AND
cargo run -- -- "dark.slot a = 1 dark.slot b = 0 onif a and b onda.core blelse ryprime"

# Test 4: Operadores OR
cargo run -- -- "dark.slot a = 1 dark.slot b = 0 onif a or b onda.core blelse ryprime"

# Test 5: While
cargo run -- -- "dark.slot x = 1 ryda x onda.core"

# Test 6: REPL
echo -e "dark.slot x = 10\nmem\nexit" | cargo run -- --repl

# Test 7: Archivo
cargo run -- -- ejemplo.rydit

# Test 8: Todos los tests automáticos
cargo test
```

---

## 🔧 **Comandos de Verificación**

```bash
# Verificar compilación
cargo check

# Compilar
cargo build

# Tests automáticos
cargo test

# Ver tests pasando
cargo test 2>&1 | grep -E "test |passed|failed"
```

---

**Última actualización:** 2026-03-14  
**Próxima versión:** v0.0.3 - Funciones (`rytmo`) + Mejoras al while
