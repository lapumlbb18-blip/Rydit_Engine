# 📊 PROGRESO FIX 13 ERRORES - ACTUALIZACIÓN

**Fecha**: 2026-04-02  
**Inicio**: 70 errores  
**Actual**: 15 errores (79% progreso)  
**Fijados directamente**: 8 errores

---

## ✅ ERRORES FIXEADOS (8/13 originales)

| # | Línea | Error | Fix aplicado | Estado |
|---|-------|-------|--------------|--------|
| 1 | 394 | `module.clone()` → `String` | `module.to_string()` | ✅ |
| 2 | 1451 | `name.clone()` → `String` | `name.to_string()` | ✅ |
| 3 | 1740 | `funcs.get(func_name)` | `funcs.get(&func_name)` | ✅ |
| 4 | 1831 | `module.clone()` → `String` | `module.to_string()` | ✅ |
| 5 | 2187 | `Valor::Texto(s.clone())` | `Valor::Texto(s.to_string())` | ✅ |
| 6 | 3184 | `Valor::Texto(s.clone())` | `Valor::Texto(s.to_string())` | ✅ |
| 9 | 4488 | `name.clone()` → `String` | `name.to_string()` | ✅ |
| 13 | 4547 | `module.clone()` → `String` | `module.to_string()` | ✅ |

---

## ⚠️ ERRORES COMPLEJOS (5 originales → 15 actuales)

### Problema de Fondo
El AST nuevo (`rydit-parser`) usa **lifetimes** (`Stmt<'a>`, `Expr<'a>`) pero el código en `main.rs` fue escrito para AST **sin lifetimes**.

**Síntomas**:
- `Stmt<'a>` tiene `&'a str` en todos lados
- `funcs` era `HashMap<String, (Vec<String>, Vec<Stmt>)>` 
- Ahora necesita `HashMap<String, (Vec<String>, Vec<Stmt<'stmt>>)>` 
- Esto causa **cascada de cambios de lifetime** en todo el código

### Errores Actuales

| Tipo | Cantidad | Ejemplo |
|------|----------|---------|
| **E0308** (mismatched types) | 2 | `if`/`else` tipos incompatibles |
| **E0597** (borrowed value dropped) | 5 | `body`, `input`, `program.statements` |
| **E0621** (explicit lifetime required) | 3 | `funcs`, `program` |
| **E0483** (use of moved value) | 2 | `module_content`, `body` |
| **E0515** (returns reference to temporary) | 3 | Varios |

---

## 🔧 SOLUCIONES POSIBLES

### Opción 1: Pelear con Lifetimes (NO RECOMENDADO)
**Tiempo estimado**: 2-3 horas  
**Riesgo**: Alto (pueden aparecer más errores)

**Pasos**:
1. Agregar lifetimes a TODAS las funciones
2. Cambiar TODOS los tipos `Vec<Stmt>` → `Vec<Stmt<'a>>`
3. Fixear 20-30 errores adicionales en cascada

**Pros**:
- Código más eficiente (zero-copy)
- Sigue el diseño original del parser

**Contras**:
- Muy complejo
- Propenso a más errores
- Difícil de mantener

---

### Opción 2: Usar `Arc<str>` en el Parser (RECOMENDADO)
**Tiempo estimado**: 30 minutos  
**Riesgo**: Bajo

**Idea**: Cambiar el AST para usar `Arc<str>` en lugar de `&'a str`

**Cambios en `crates/rydit-parser/src/ast.rs`**:
```rust
// ANTES (con lifetimes)
pub enum Stmt<'a> {
    Command(&'a str),
    Assign { name: &'a str, value: Expr<'a> },
    Function { name: &'a str, params: Vec<&'a str>, body: Vec<Stmt<'a>> },
}

// DESPUÉS (sin lifetimes, con Arc)
use std::sync::Arc;

pub enum Stmt {
    Command(Arc<str>),
    Assign { name: Arc<str>, value: Expr },
    Function { name: Arc<str>, params: Vec<Arc<str>>, body: Vec<Stmt> },
}
```

**Ventajas**:
- ✅ Elimina TODOS los lifetimes
- ✅ Zero-copy relativo (Arc es compartido)
- ✅ Código más simple
- ✅ Sin cascada de errores

**Desventajas**:
- ⚠️ Overhead mínimo de Arc (~16 bytes por string)
- ⚠️ Requiere cambiar lexer/parser

---

### Opción 3: Owned AST en Parser (ALTERNATIVA)
**Tiempo estimado**: 45 minutos  
**Riesgo**: Bajo

**Idea**: Usar `String` en lugar de `&str` en el AST

**Cambios**:
```rust
// AST con Strings poseídos
pub enum Stmt {
    Command(String),
    Assign { name: String, value: Expr },
    Function { name: String, params: Vec<String>, body: Vec<Stmt> },
}
```

**Ventajas**:
- ✅ Sin lifetimes
- ✅ Código más simple
- ✅ Sin problemas de borrowing

**Desventajas**:
- ⚠️ Más allocaciones (cada string se clona)
- ⚠️ ~10-20% más lento en parsing

---

## 📋 RECOMENDACIÓN

### **Usar Opción 2 (Arc<str>)**

**Razones**:
1. **Balance perfecto**: Zero-copy relativo + sin lifetimes
2. **Cambios mínimos**: Solo en parser/lexer
3. **Sin cascada**: No requiere cambiar main.rs
4. **Rendimiento**: Arc es compartido, no hay clones innecesarios

**Plan**:

**Paso 1**: Cambiar AST en `rydit-parser/src/ast.rs` (15 min)
```bash
# Reemplazar &'a str con Arc<str>
sed -i "s/&'a str/Arc<str>/g" crates/rydit-parser/src/ast.rs
# Agregar import
sed -i '1i use std::sync::Arc;' crates/rydit-parser/src/ast.rs
```

**Paso 2**: Fixear lexer para devolver `Arc<str>` (15 min)
```rust
// crates/rydit-lexer/src/lib.rs
use std::sync::Arc;

pub fn scan(&self) -> Vec<Token> {
    // En vez de: Token::Ident(&self.source[start..end])
    // Usar: Token::Ident(Arc::from(&self.source[start..end]))
}
```

**Paso 3**: Re-compilar y verificar (5 min)
```bash
cargo build -p rydit-rs --bin rydit-rs
```

**Resultado esperado**: **0 errores** ✅

---

## 🎯 CONCLUSIÓN

**Progreso actual**: 79% (70 → 15 errores)

**Bloqueo actual**: Lifetimes en cascada

**Solución recomendada**: Cambiar parser a `Arc<str>` (30 min)

**Tiempo total estimado**:
- Fixes iniciales: 20 min ✅ (completado)
- Fix con lifetimes: 30 min (pendiente)
- **Total**: ~50 minutos para 0 errores

---

<div align="center">

**🛡️ 79% COMPLETADO - 15 ERRORES RESTANTES**

*Siguiente: Cambiar parser a Arc<str> para eliminar lifetimes*

</div>
