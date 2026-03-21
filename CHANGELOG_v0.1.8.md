# 🛡️ CHANGELOG v0.1.8 - Maduración del Lenguaje

**Fecha:** 2026-03-19
**Versión:** v0.1.8
**Estado:** ✅ **75 TESTS - 0 WARNINGS**

---

## 📝 Resumen

Esta versión madura el lenguaje RyDit con features esenciales para el manejo de strings, caracteres Unicode y símbolos:

1. **Escapes en strings** - Soporte completo para `\n`, `\t`, `\\`, `\r`
2. **Comillas simples** - Strings con `'...'` además de `"..."`
3. **UTF-8 completo** - Números e identificadores con caracteres Unicode
4. **Símbolos en identificadores** - `@`, `$`, `%`, `&`, `|`, `^`, `~`, `` ` ``

---

## ✨ Features Añadidas

### 1. Escapes en Strings [CRÍTICO]

**Antes:**
```rydit
voz "solo comillas escapadas \""
# \n \t \\ no funcionaban
```

**Ahora:**
```rydit
voz "Salto de línea:\nHola"
voz "Tabulación:\tHola"
voz "Backslash:\\\\Ruta"
voz "Retorno:\rHola"
```

**Implementación:**
```rust
// crates/lizer/src/lib.rs - Lexer string handling
if chars[i] == '\\' && i + 1 < chars.len() {
    match chars[i + 1] {
        '"' => { text.push('"'); i += 2; column += 2; }
        '\\' => { text.push('\\'); i += 2; column += 2; }
        'n' => { text.push('\n'); i += 2; column += 2; }
        't' => { text.push('\t'); i += 2; column += 2; }
        'r' => { text.push('\r'); i += 2; column += 2; }
        _ => { /* escape desconocido - literal */ }
    }
}
```

---

### 2. Comillas Simples

**Antes:**
```rydit
# Solo comillas dobles disponibles
dark.slot texto = "hola"
```

**Ahora:**
```rydit
# Comillas dobles
dark.slot texto1 = "hola"

# Comillas simples
dark.slot texto2 = 'hola'

# Mixtas
dark.slot mix1 = 'dijo "hola"'
dark.slot mix2 = "dijo 'hola'"
```

**Implementación:**
```rust
// Soporte para ambos tipos de comillas
if chars[i] == '"' || chars[i] == '\'' {
    let quote_char = chars[i];
    // ... manejo unificado con escapes
}
```

---

### 3. UTF-8 Completo

**Antes:**
```rust
// Solo ASCII
if chars[i].is_ascii_digit() { ... }
if chars[i].is_alphabetic() { ... }
```

**Ahora:**
```rust
// UTF-8 completo
if chars[i].is_numeric() { ... }  // Dígitos Unicode (①②③, 一 二 三)
if chars[i].is_alphabetic() || chars[i].is_alphanumeric() { ... }  // Ñ, ü, etc.
```

**Ejemplo:**
```rydit
dark.slot número_con_tilde = 100
dark.slot ① = "uno"  # Dígitos Unicode
voz "€100 £50 ¥200"  # Símbolos monetarios
```

---

### 4. Símbolos en Identificadores

**Antes:**
```rydit
# @ $ % causaban error
dark.slot x = 100  # Solo letras, números, _
```

**Ahora:**
```rydit
# Símbolos válidos al inicio de identificadores
dark.slot @usuario = "alucard18"
dark.slot $precio = 99.99
dark.slot %porcentaje = 50
dark.slot &amper = 123
dark.slot |pipe = 456
dark.slot ^caret = 789
dark.slot ~tilde = 101112
```

**Implementación:**
```rust
'@' | '$' | '%' | '&' | '|' | '^' | '~' | '`' => {
    let mut ident = String::new();
    ident.push(chars[i]);
    i += 1;
    column += 1;
    // Continuar leyendo alfanuméricos
    while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '.' || chars[i] == '_') {
        ident.push(chars[i]);
        i += 1;
        column += 1;
    }
    tokens.push(Token::Ident(ident));
}
```

---

## 🧪 Tests Añadidos

### 10 Tests Nuevos (65 → 75)

```rust
// Escapes
test_string_escape_newline
test_string_escape_tab
test_string_escape_backslash
test_string_escape_carriage_return
test_string_escape_multiple

// Comillas simples
test_string_comillas_simples
test_string_comillas_simples_escape
test_string_comillas_simples_con_doble_dentro
test_string_comillas_dobles_con_simple_dentro

// Símbolos
test_simbolos_identificadores
test_simbolos_variados
test_not_operador

// Fix
test_error_caracter_raro (actualizado para €£¥)
```

---

## 📊 Métricas de Cambio

| Métrica | v0.1.7 | v0.1.8 | Delta |
|---------|--------|--------|-------|
| Tests | 65 | 75 | +10 |
| Warnings | 0 | 0 | 0 |
| Líneas Rust | ~2100 | ~2223 | +123 |
| Features Lexer | 3 | 7 | +4 |

---

## 🔧 Archivos Modificados

| Archivo | Cambios | Líneas |
|---------|---------|--------|
| `crates/lizer/src/lib.rs` | Escapes + comillas + UTF-8 + símbolos | +123 |
| `demos/demo_maduracion_v0.1.8.rydit` | Demo nueva | +90 |
| `QWEN.md` | Actualización de memoria | +11 |
| `CHANGELOG_v0.1.8.md` | Nuevo archivo | ~150 |

---

## ✅ Verificación

### Build Sin Warnings
```bash
cargo build 2>&1 | grep -E "warning|error"
# Resultado: (vacío - sin warnings)
```

### Todos Los Tests Pasan
```bash
cargo test 2>&1 | grep "test result"
# Resultado: test result: ok. 75 passed; 0 failed
```

### Demo Funcional
```bash
cargo run --bin rydit-rs -- demos/demo_maduracion_v0.1.8.rydit
# Resultado: Demo ejecutada exitosamente
```

---

## 🎯 Impacto

### Antes (v0.1.7)
- ❌ Sin escapes `\n`, `\t`, `\\`
- ❌ Solo comillas dobles
- ❌ ASCII-only en números/identificadores
- ❌ Símbolos `@#$` causaban error

### Después (v0.1.8)
- ✅ Escapes completos en strings
- ✅ Comillas simples y dobles intercambiables
- ✅ UTF-8 completo (Unicode)
- ✅ Símbolos válidos en identificadores
- ✅ 75 tests passing, 0 warnings

---

## 📚 Lecciones Aprendidas

1. **UTF-8 es esencial:** `is_numeric()` vs `is_ascii_digit()` - diferencia entre lenguaje moderno y limitado.

2. **Comillas mixtas:** Permitir `'...'` y `"..."` intercambiables mejora UX para strings con comillas internas.

3. **Símbolos expresivos:** `@usuario`, `$precio`, `%porcentaje` hacen el código más legible.

4. **Escapes son fundamentales:** `\n`, `\t`, `\\` necesarios para strings multilínea y rutas.

---

## 🚀 Próxima Versión: v0.2.0

- Module system avanzado
- Imports entre módulos
- Cache de módulos
- Detección de imports circulares

---

**v0.1.8 - "Maduración del Lenguaje"** 🛡️

*Construido con ❤️ en Android/Termux*
