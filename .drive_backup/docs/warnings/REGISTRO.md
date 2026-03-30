# 📋 Registro de Warnings - RyDit

**Última actualización:** 2026-03-15
**Versión:** v0.0.5 (Input Seguro + Warnings Fix)
**Total warnings:** 2 activos (3 resueltos en esta sesión)

---

## 🚨 Warnings Activos

| ID | Fecha | Warning | Ubicación | Estado | Prioridad |
|----|-------|---------|-----------|--------|-----------|
| [WARN-002](#warn-002) | 2026-03-15 | Sin manejo de errores de división | `main.rs:237` | 🟡 Pendiente | Baja |

---

## ✅ Warnings Resueltos

| ID | Fecha | Warning | Solución | Tests |
|----|-------|---------|----------|-------|
| WARN-001 | 2026-03-15 | `unwrap()` en I/O stdin | `if let Err()` en vez de `unwrap()` | ✅ 3 |
| WARN-003 | 2026-03-15 | Variables no usadas | Cambiar a `_var` | 0 |
| WARN-004 | 2026-03-15 | `unused_variables` en lizer | `_start_col` | ✅ N/A |
| WARN-005 | 2026-03-15 | `unreachable_patterns` en main | Eliminar `Stmt::Block` duplicado | ✅ N/A |

---

## 📌 Warnings Detallados

### WARN-001: ✅ RESUELTO - `unwrap()` en I/O stdin

**Ubicación:**
- `crates/rydit-rs/src/main.rs:281` (REPL)
- `crates/blast-core/src/lib.rs:96` (input)

**Código problemático (ANTES):**
```rust
// main.rs:281
io::stdout().flush().unwrap();

// blast-core:96
io::stdin().read_line(&mut input).unwrap();
```

**Causa:**
`unwrap()` panicará si:
- stdin se cierra inesperadamente
- Error de E/O en terminal (Termux puede tener comportamientos raros)
- Pipe roto en redirección

**Solución implementada:**
```rust
// blast-core/src/lib.rs - input()
if let Err(e) = io::stdout().flush() {
    eprintln!("[WARNING] Error en flush de stdout: {}", e);
    return Valor::Error("Error de I/O en input".to_string());
}

let mut input = String::new();
if let Err(e) = io::stdin().read_line(&mut input) {
    eprintln!("[WARNING] Error al leer stdin: {}", e);
    return Valor::Error("Error de lectura en input".to_string());
}

// rydit-rs/src/main.rs - REPL
if let Err(e) = io::stdout().flush() {
    eprintln!("[REPL ERROR] Flush falló: {}", e);
    break;
}
```

**Tests agregados:**
- [x] `test_input_numero` - Verifica parseo de números
- [x] `test_input_texto` - Verifica parseo de texto
- [x] `test_input_vacio` - Verifica input vacío

**Estado:** ✅ Resuelto (v0.0.5)

---

### WARN-002: Sin manejo de errores de división

**Ubicación:** `crates/rydit-rs/src/main.rs:237`

**Código actual:**
```rust
lizer::BinOp::Div => {
    if r != 0.0 { 
        Valor::Num(l / r) 
    } else { 
        Valor::Error("División por cero".to_string()) 
    }
}
```

**Causa:**
El error se retorna como `Valor::Error` pero:
- No hay warning en compile-time
- El usuario no sabe que hubo error hasta runtime
- Podría propagarse silenciosamente

**Solución propuesta:**
```rust
// Opción A: Warning en compile-time (parser)
// Detectar división por cero constante: x / 0

// Opción B: Runtime warning con logging
Valor::Error(msg) => {
    eprintln!("[WARNING] Error en operación: {}", msg);
    // ...
}

// Opción C: Sistema de warnings estructurado
pub struct Warning {
    pub kind: WarningKind,
    pub message: String,
}

pub enum WarningKind {
    DivisionByZero,
    UnusedVariable,
    TypeMismatch,
}
```

**Tests pendientes:**
- [ ] `test_division_por_cero` - Verificar que retorna Error
- [ ] `test_division_normal` - Verificar que funciona

**Estado:** 🟡 Pendiente (prioridad baja - ya maneja el error)

---

### WARN-004: ✅ RESUELTO - `unused_variables` en lizer

**Ubicación:** `crates/lizer/src/lib.rs:302`

**Código problemático:**
```rust
let start_col = column;  // variable no usada
```

**Causa:**
Variable declarada pero no usada en el lexer.

**Solución implementada:**
```rust
let _start_col = column;  // Reservado para futuros errores detallados
```

**Estado:** ✅ Resuelto (v0.0.5)

---

### WARN-005: ✅ RESUELTO - `unreachable_patterns` en main.rs

**Ubicación:** `crates/rydit-rs/src/main.rs:169`

**Código problemático:**
```rust
Stmt::Block(stmts) => { ... }  // Duplicado en match
```

**Causa:**
`Stmt::Block` se matchea dos veces en el mismo match.

**Solución implementada:**
Eliminar el segundo match de `Stmt::Block` (línea 169).

**Estado:** ✅ Resuelto (v0.0.5)

---

## 📊 Estadísticas

| Estado | Cantidad |
|--------|----------|
| 🟡 Activos | 1 |
| ✅ Resueltos | 5 |
| 🔴 Críticos | 0 |

---

## 🎯 Política de Warnings

1. **Todo warning debe tener:**
   - ID único (WARN-XXX)
   - Fecha de creación
   - Ubicación exacta
   - Causa raíz
   - Solución propuesta
   - Tests asociados

2. **Prioridades:**
   - 🔴 **Crítica:** Crash o pérdida de datos
   - 🟡 **Media:** Funciona pero frágil
   - 🟢 **Baja:** Mejora de calidad de vida

3. **Meta v0.0.4:**
   - [x] Arrays implementados
   - [x] 10+ tests de arrays (11 tests agregados)
   - [x] Documentación actualizada
   - [x] 0 warnings críticos ✅
   - [x] Fix WARN-005 (unreachable_patterns) ✅

4. **Meta v0.0.5:**
   - [x] Fix WARN-001 (unwrap en I/O) ✅
   - [x] Fix WARN-004 (unused_variables) ✅
   - [x] Tests de división por cero ✅
   - [ ] Sistema de warnings en runtime (v0.0.6)
   - [ ] Gráficos raylib (v0.0.6)

---

## 🧪 Tests de Warnings

Agregar en `crates/lizer/src/lib.rs` o `crates/rydit-rs/src/main.rs`:

```rust
#[cfg(test)]
mod warning_tests {
    #[test]
    fn test_division_por_cero() {
        // Verificar que división por cero retorna Error, no panic
        let resultado = evaluar("dark.slot x = 10 / 0");
        assert!(resultado.contiene_error());
    }

    #[test]
    fn test_input_stdin_cerrado() {
        // TODO: Simular stdin cerrado
        // Esto requiere mock de std::io
    }
}
```

---

**Próxima revisión:** v0.0.4 (Arrays)  
**Responsable:** Equipo RyDit
