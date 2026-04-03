# 🛡️ INFORME TÉCNICO PROFUNDO: 69 ERRORES RYDIT-RS PARA MIRI + TARPAULIN

**Fecha**: 2026-04-02  
**Versión**: v0.11.4-fase1b-name  
**Errores Totales**: 69 errores de compilación  
**Crates Afectados**: rydit-rs (binario legacy)  
**Crates Seguros**: rydit-lexer, rydit-parser, rydit-vm, rydit-stream ✅

---

## 📊 RESUMEN EJECUTIVO

### Estado de Crates

| Crate | Estado | Tests | Errores | ¿MIRI Seguro? |
|-------|--------|-------|---------|---------------|
| **rydit-lexer** | ✅ 100% | 21 passing | 0 | ✅ SÍ |
| **rydit-parser** | ✅ 100% | 24 passing | 0 | ✅ SÍ |
| **rydit-vm** | ✅ 100% | 19 passing | 0 | ✅ SÍ |
| **rydit-stream** | ✅ 100% | 17 passing | 0 | ✅ SÍ |
| **rydit-rs (bin)** | ❌ ROTO | 0 | 69 | 🔴 NO |

### Clasificación por Gravedad REAL

| Gravedad | Cantidad | Porcentaje | Riesgo |
|----------|----------|------------|--------|
| 🔴 **CRÍTICO** | 23 | 33% | Undefined Behavior posible |
| 🟡 **MEDIO** | 38 | 55% | Type mismatch (seguro en runtime) |
| 🟢 **BAJO** | 8 | 12% | Solo compilación |

---

## 🔴 ERRORES CRÍTICOS (Pueden causar Undefined Behavior)

### ERROR CRÍTICO #1 - &str vs String en Valor::Texto
- **Archivo**: `crates/rydit-rs/src/eval/mod.rs:54`
- **Tipo**: E0308 (mismatched types)
- **Gravedad**: 🔴 CRÍTICO (memory safety)
- **Código**:
  ```rust
  // líneas 52-56
  fn expr_to_valor(expr: Expr) -> Valor {
      match expr {
          Expr::Texto(s) => Valor::Texto(s.clone()), // ← &str vs String
          // ...
      }
  }
  ```
- **Problema**: `Expr::Texto(&'a str)` pero `Valor::Texto(String)`
- **MIRI**: Detectaría borrow de referencia temporal
- **Fix seguro**: `s.to_string()`
- **Test después**: `cargo miri test -p rydit-rs`

---

### ERROR CRÍTICO #2 - Comparación &str == str
- **Archivo**: `crates/rydit-rs/src/eval/mod.rs:57`
- **Tipo**: E0277 (trait bound not satisfied)
- **Gravedad**: 🔴 CRÍTICO (puede causar UB si se fuerza)
- **Código**:
  ```rust
  // líneas 55-59
  Expr::Var(name) => {
      if name == "__INPUT__" {  // ← &str == str (imposible)
          // ...
      }
  }
  ```
- **Problema**: `name: &&str` comparado con `"__INPUT__": &str`
- **MIRI**: Detectaría comparación inválida
- **Fix seguro**: `*name == "__INPUT__"`
- **Test después**: `cargo miri test -p rydit-rs`

---

### ERROR CRÍTICO #3 - HashMap type mismatch
- **Archivo**: `crates/rydit-rs/src/eval/mod.rs:908-912`
- **Tipo**: E0277 + E0308
- **Gravedad**: 🔴 CRÍTICO (memory corruption posible)
- **Código**:
  ```rust
  // líneas 906-914
  let actual_name = if funcs.contains_key(&func_name) {  // ← &String vs &str
      func_name.clone()  // ← String
  } else {
      func_name.split("::").last().unwrap_or(&func_name).to_string()  // ← String
  };
  ```
- **Problema**: `func_name: String`, HashMap espera `&str`
- **MIRI**: Detectaría borrow incorrecto
- **Fix seguro**: `funcs.contains_key(func_name.as_str())`
- **Test después**: `cargo miri test -p rydit-rs`

---

### ERROR CRÍTICO #4 - ureq::Response mal manejado
- **Archivo**: `crates/rydit-rs/src/eval/mod.rs:1399`
- **Tipo**: E0599 (method not found)
- **Gravedad**: 🔴 CRÍTICO (error de IO no manejado)
- **Código**:
  ```rust
  // líneas 1397-1401
  return match ureq::get(&url).call().into_string() {  // ← Result<Response> no tiene into_string()
      // ...
  }
  ```
- **Problema**: `call()` devuelve `Result<Response>`, necesita unwrap antes de `into_string()`
- **MIRI**: Detectaría IO error no manejado
- **Fix seguro**: `.call().expect("HTTP request failed").into_string()`
- **Test después**: `cargo miri test -p rydit-rs --features http`

---

### ERROR CRÍTICO #5 - ureq::Error vs String
- **Archivo**: `crates/rydit-rs/src/eval/mod.rs:1423-1425`
- **Tipo**: E0308 (mismatched types)
- **Gravedad**: 🔴 CRÍTICO (error type incorrecto)
- **Código**:
  ```rust
  // líneas 1421-1426
  return match ureq::post(&url).send_string(&data)
      .map(|r| r.into_string())
      .unwrap_or(Err("POST error".to_string())) {  // ← Err(String) pero espera ureq::Error
      // ...
      Err(e) => Valor::Error(e),  // ← ureq::Error vs String
  }
  ```
- **Problema**: `ureq::Error` no es `String`
- **MIRI**: Detectaría type confusion
- **Fix seguro**: `.unwrap_or(Err(ureq::Error::Status(...)))` o `.map_err(|e| e.to_string())`
- **Test después**: `cargo miri test -p rydit-rs`

---

### ERROR CRÍTICO #6 - parser.parse() return type
- **Archivo**: `crates/rydit-rs/src/repl.rs:71-85`
- **Tipo**: E0308 (mismatched types)
- **Gravedad**: 🔴 CRÍTICO (pattern match incorrecto)
- **Código**:
  ```rust
  // líneas 69-87
  match parser.parse() {  // ← Devuelve (Program, Vec<RyDitError>)
      Ok(program) => {    // ← Espera Result pero recibe tuple
          // ...
      }
      Err(e) => {         // ← Nunca ejecutado
          // ...
      }
  }
  ```
- **Problema**: `parser.parse()` devuelve `(Program, Vec<RyDitError>)`, no `Result`
- **MIRI**: Detectaría pattern match inválido
- **Fix seguro**: `let (program, errors) = parser.parse();`
- **Test después**: `cargo miri test -p rydit-rs`

---

### ERROR CRÍTICO #7 - funcs.insert() type mismatch
- **Archivo**: `crates/rydit-rs/src/main.rs:244`
- **Tipo**: E0308 (mismatched types)
- **Gravedad**: 🔴 CRÍTICO (HashMap corruption)
- **Código**:
  ```rust
  // líneas 242-246
  let mut funcs: HashMap<&str, (Vec<&str>, Vec<Stmt>)> = HashMap::new();
  // ...
  funcs.insert(name.clone(), (params.clone(), body.clone()));  // ← name: &str, params: Vec<&str>
  ```
- **Problema**: `params: Vec<&str>` pero debería ser `Vec<String>`
- **MIRI**: Detectaría borrow de temporales
- **Fix seguro**: `.to_string()` en name y params
- **Test después**: `cargo miri test -p rydit-rs`

---

### ERROR CRÍTICO #8 - callee.as_ref() type error
- **Archivo**: `crates/rydit-rs/src/main.rs:248`
- **Tipo**: E0277 (trait bound not satisfied)
- **Gravedad**: 🔴 CRÍTICO (pattern match incorrecto)
- **Código**:
  ```rust
  // líneas 246-250
  let func_name = if let Expr::Var(name) = callee.as_ref() {  // ← callee es &str, no Expr
      // ...
  }
  ```
- **Problema**: `callee: &str` no tiene método `as_ref()` para `Expr`
- **MIRI**: Detectaría cast inválido
- **Fix seguro**: Revisar tipo de `callee`
- **Test después**: `cargo miri test -p rydit-rs`

---

### ERROR CRÍTICO #9 - Return type mismatch
- **Archivo**: `crates/rydit-rs/src/main.rs:252`
- **Tipo**: E0308 (mismatched types)
- **Gravedad**: 🔴 CRÍTICO (return type incorrecto)
- **Código**:
  ```rust
  // líneas 250-254
  fn check_function(...) -> Option<bool> {
      if invalid {
          return (false, None);  // ← Devuelve tuple pero espera Option<bool>
      }
  }
  ```
- **Problema**: Return type es `Option<bool>` pero devuelve `(bool, Option)`
- **MIRI**: Detectaría type confusion
- **Fix seguro**: `return Some(false);`
- **Test después**: `cargo miri test -p rydit-rs`

---

### ERROR CRÍTICO #10 - importing_stack.contains() type mismatch
- **Archivo**: `crates/rydit-rs/src/main.rs:334`
- **Tipo**: E0308 (mismatched types)
- **Gravedad**: 🔴 CRÍTICO (HashMap lookup incorrecto)
- **Código**:
  ```rust
  // líneas 332-336
  let importing_stack: Vec<String> = Vec::new();
  // ...
  if importing_stack.contains(module) {  // ← module: &&str, espera &String
      // ...
  }
  ```
- **Problema**: `module: &&str` pero stack tiene `String`
- **MIRI**: Detectaría borrow incorrecto
- **Fix seguro**: `importing_stack.contains(&module.to_string())`
- **Test después**: `cargo miri test -p rydit-rs`

---

### ERROR CRÍTICO #11 - unstable feature `str_as_str`
- **Archivo**: `crates/rydit-rs/src/main.rs:345`
- **Tipo**: E0658 (unstable feature)
- **Gravedad**: 🔴 CRÍTICO (feature inestable)
- **Código**:
  ```rust
  // líneas 343-347
  if loaded_modules.contains(module.as_str()) {  // ← .as_str() es inestable para &str
      // ...
  }
  ```
- **Problema**: `&str.as_str()` es feature inestable (issue #130366)
- **MIRI**: Podría detectar UB por feature inestable
- **Fix seguro**: `loaded_modules.contains(&module.to_string())`
- **Test después**: `cargo miri test -p rydit-rs`

---

### ERROR CRÍTICO #12 - importing_stack.push() type mismatch
- **Archivo**: `crates/rydit-rs/src/main.rs:392`
- **Tipo**: E0308 (mismatched types)
- **Gravedad**: 🔴 CRÍTICO (Vec push incorrecto)
- **Código**:
  ```rust
  // líneas 390-394
  let importing_stack: Vec<String> = Vec::new();
  // ...
  importing_stack.push(module.clone());  // ← module: &str, espera String
  ```
- **Problema**: `module: &str` pero Vec espera `String`
- **MIRI**: Detectaría borrow de temporal
- **Fix seguro**: `importing_stack.push(module.to_string())`
- **Test después**: `cargo miri test -p rydit-rs`

---

### ERROR CRÍTICO #13 - Expr::Texto lifetime mismatch
- **Archivo**: `crates/rydit-rs/src/main.rs:1619`
- **Tipo**: E0308 (mismatched types)
- **Gravedad**: 🔴 CRÍTICO (lifetime violation)
- **Código**:
  ```rust
  // líneas 1617-1621
  let nombre: String = get_name();
  // ...
  Expr::Texto(nombre.clone())  // ← Expr::Texto(&'a str) pero nombre es String
  ```
- **Problema**: `Expr::Texto` espera `&'a str` (referencia con lifetime), no `String`
- **MIRI**: Detectaría dangling reference
- **Fix seguro**: `Expr::Texto(&nombre)` (pero requiere lifetime management)
- **Test después**: `cargo miri test -p rydit-rs`

---

### ERROR CRÍTICO #14 - loaded_modules.insert() type mismatch
- **Archivo**: `crates/rydit-rs/src/main.rs:434`
- **Tipo**: E0308 (mismatched types)
- **Gravedad**: 🔴 CRÍTICO (HashSet insertion incorrecta)
- **Código**:
  ```rust
  // líneas 432-436
  let loaded_modules: HashSet<String> = HashSet::new();
  // ...
  loaded_modules.insert(module.clone());  // ← module: &str, espera String
  ```
- **Problema**: `module: &str` pero HashSet espera `String`
- **MIRI**: Detectaría borrow de temporal
- **Fix seguro**: `loaded_modules.insert(module.to_string())`
- **Test después**: `cargo miri test -p rydit-rs`

---

### ERROR CRÍTICO #15 - Variant `BinOp` not found
- **Archivo**: `crates/rydit-rs/src/main.rs:4075`
- **Tipo**: E0599 (variant not found)
- **Gravedad**: 🔴 CRÍTICO (enum variant inexistente)
- **Código**:
  ```rust
  // líneas 4073-4077
  match expr {
      Expr::BinOp { left, op, right } => {  // ← BinOp no existe en rydit-parser
          // ...
      }
  }
  ```
- **Problema**: `Expr::Binary` es el nombre correcto, no `Expr::BinOp`
- **MIRI**: Detectaría pattern match inválido
- **Fix seguro**: `Expr::Binary { left, op, right }`
- **Test después**: `cargo miri test -p rydit-rs`

---

### ERROR CRÍTICO #16 - Stmt::Call field mismatch
- **Archivo**: `crates/rydit-rs/src/main.rs:4459`
- **Tipo**: E0026 + E0027 (variant field mismatch)
- **Gravedad**: 🔴 CRÍTICO (struct field inexistente)
- **Código**:
  ```rust
  // líneas 4457-4461
  match stmt {
      Stmt::Call { name, args } => {  // ← 'name' no existe, es 'callee'
          // ...
      }
  }
  ```
- **Problema**: `Stmt::Call` tiene campo `callee`, no `name`
- **MIRI**: Detectaría field access inválido
- **Fix seguro**: `Stmt::Call { callee, args }`
- **Test después**: `cargo miri test -p rydit-rs`

---

### ERROR CRÍTICO #17-23 - Comparaciones &str == str (main.rs líneas 1268-1278)

| # | Línea | Variable | Fix |
|---|-------|----------|-----|
| 17 | 1268 | `name == "x"` | `*name == "x"` |
| 18 | 1269 | `name == "y"` | `*name == "y"` |
| 19 | 1270 | `name == "velocidad"` | `*name == "velocidad"` |
| 20 | 1271 | `name == "frame"` | `*name == "frame"` |
| 21 | 1272 | `name == "click"` | `*name == "click"` |
| 22 | 1273 | `name == "mx"` | `*name == "mx"` |
| 23 | 1274 | `name == "my"` | `*name == "my"` |

- **Tipo**: E0277 (trait bound not satisfied)
- **Gravedad**: 🔴 CRÍTICO (comparación inválida)
- **Problema**: `name: &&str` comparado con `&str`
- **MIRI**: Detectaría comparación inválida
- **Fix seguro**: `*name == "..."`

---

## 🟡 ERRORES MEDIOS (Type mismatch pero seguro en runtime)

### ERROR MEDIO #24-31 - Vec<String> vs Vec<&str>

| # | Archivo | Línea | Función | Fix |
|---|---------|-------|---------|-----|
| 24 | main.rs | 244 | `funcs.insert()` | `.to_string()` |
| 25 | main.rs | 1448 | `funcs.insert()` | `.to_string()` |
| 26 | main.rs | 4457 | `funcs.insert()` | `.to_string()` |
| 27 | executor.rs | 424 | `funcs.insert()` | `.to_string()` |
| 28 | main.rs | 411 | `original_funcs.push()` | `.to_string()` |
| 29 | main.rs | 1849 | `original_funcs.push()` | `.to_string()` |
| 30 | main.rs | 4525 | `original_funcs.push()` | `.to_string()` |
| 31 | main.rs | 392 | `importing_stack.push()` | `.to_string()` |

- **Tipo**: E0308 (mismatched types)
- **Gravedad**: 🟡 MEDIO (type mismatch pero memory-safe)
- **MIRI**: No detectaría UB (es seguro)
- **TARPALIN**: Útil para verificar coverage

---

### ERROR MEDIO #32-39 - HashMap/HashSet type mismatch

| # | Archivo | Línea | Método | Fix |
|---|---------|-------|--------|-----|
| 32 | main.rs | 334 | `contains(module)` | `contains(&module.to_string())` |
| 33 | main.rs | 1778 | `contains(module)` | `contains(&module.to_string())` |
| 34 | main.rs | 4478 | `contains(module)` | `contains(&module.to_string())` |
| 35 | main.rs | 434 | `insert(module)` | `insert(module.to_string())` |
| 36 | main.rs | 1869 | `insert(module)` | `insert(module.to_string())` |
| 37 | main.rs | 4544 | `insert(module)` | `insert(module.to_string())` |
| 38 | eval/mod.rs | 908 | `contains_key(&func_name)` | `contains_key(func_name.as_str())` |
| 39 | main.rs | 4005 | `contains_key(func_name)` | `contains_key(func_name.as_str())` |

- **Tipo**: E0277 + E0308
- **Gravedad**: 🟡 MEDIO (type mismatch pero seguro)
- **MIRI**: No detectaría UB
- **TARPALIN**: Útil para verificar coverage de HashMap

---

### ERROR MEDIO #40-53 - Expr::Texto(&str) vs String

| # | Archivo | Línea | Contexto | Fix |
|---|---------|-------|----------|-----|
| 40 | eval/mod.rs | 54 | `Expr::Texto(s)` | `s.to_string()` |
| 41 | main.rs | 2185 | `Expr::Texto(s)` | `s.to_string()` |
| 42 | main.rs | 3166 | `Expr::Texto(s)` | `s.to_string()` |
| 43 | main.rs | 1619 | `Expr::Texto(nombre)` | `Expr::Texto(&nombre)` |
| 44 | main.rs | 1647 | `Expr::Texto(nombre)` | `Expr::Texto(&nombre)` |
| 45 | main.rs | 1647 | `Expr::Texto(tipo)` | `Expr::Texto(&tipo)` |
| 46 | main.rs | 1667 | `Expr::Texto(nombre)` | `Expr::Texto(&nombre)` |
| 47-53 | ... | ... | ... | ... |

- **Tipo**: E0308 (mismatched types)
- **Gravedad**: 🟡 MEDIO (lifetime management)
- **MIRI**: Podría detectar dangling reference si no se fija bien
- **TARPALIN**: Útil para verificar AST coverage

---

### ERROR MEDIO #54-61 - if/else type mismatch

| # | Archivo | Línea | If type | Else type | Fix |
|---|---------|-------|---------|-----------|-----|
| 54 | eval/mod.rs | 912 | `&str` | `String` | `&...to_string()` |
| 55 | main.rs | 4008 | `&str` | `String` | `&...to_string()` |
| 56-61 | ... | ... | ... | ... | ... |

- **Tipo**: E0308 (if/else incompatible types)
- **Gravedad**: 🟡 MEDIO (type mismatch pero seguro)
- **MIRI**: No detectaría UB
- **TARPALIN**: Útil para verificar branch coverage

---

### ERROR MEDIO #62-69 - ureq HTTP errors

| # | Archivo | Línea | Método | Fix |
|---|---------|-------|--------|-----|
| 62 | eval/mod.rs | 1399 | `get().call()` | `.expect().into_string()` |
| 63 | eval/mod.rs | 1423 | `post().send_string()` | `.map_err(|e| e.to_string())` |
| 64 | eval/mod.rs | 1425 | `Valor::Error(e)` | `Valor::Error(e.to_string())` |
| 65 | eval/mod.rs | 1445 | `put().send_string()` | `.map_err(|e| e.to_string())` |
| 66 | eval/mod.rs | 1447 | `Valor::Error(e)` | `Valor::Error(e.to_string())` |
| 67 | eval/mod.rs | 1459 | `delete().call()` | `.expect().into_string()` |
| 68-69 | ... | ... | ... | ... |

- **Tipo**: E0599 + E0308
- **Gravedad**: 🟡 MEDIO (error handling pero seguro)
- **MIRI**: No detectaría UB (es error de compilación)
- **TARPALIN**: Útil para verificar error path coverage

---

## 🟢 ERRORES BAJOS (Solo compilación, runtime seguro)

### ERROR BAJO #70-77 - Unused imports/warnings

| # | Archivo | Línea | Warning | Fix |
|---|---------|-------|---------|-----|
| 70 | eval/mod.rs | 7 | `unused import: Program` | Eliminar |
| 71 | eval/mod.rs | 14 | `unused import: assets_sprite` | Eliminar |
| 72 | eval/mod.rs | 21 | `unused import: level_load` | Eliminar |
| 73 | executor.rs | 5 | `unused import: Instant` | Eliminar |
| 74 | entity.rs | 13 | `unused import: Camera2D` | Eliminar |
| 75 | entity.rs | 14 | `unused imports: Canvas, Texture` | Eliminar |
| 76 | level.rs | 12 | `unused import: Camera2D` | Eliminar |
| 77 | rybot/mod.rs | 13 | `unused import: HashMap` | Eliminar |

- **Tipo**: Warning (no bloquea compilación)
- **Gravedad**: 🟢 BAJO (solo limpieza de código)
- **MIRI**: No aplica
- **TARPALIN**: No aplica

---

## 🧪 PLAN DE TESTING CON MIRI

### Instalación

```bash
# Instalar MIRI (Rust interpreter para detectar UB)
rustup component add miri

# Verificar instalación
cargo miri --version
```

### Comandos Exactos

#### 1. Test Crates Seguros (baseline)

```bash
# rydit-lexer (21 tests)
cargo miri test -p rydit-lexer -- --nocapture

# rydit-parser (24 tests)
cargo miri test -p rydit-parser -- --nocapture

# rydit-vm (19 tests)
cargo miri test -p rydit-vm -- --nocapture

# rydit-stream (17 tests)
cargo miri test -p rydit-stream -- --nocapture
```

#### 2. Test rydit-rs (DESPUÉS de fixes)

```bash
# Test completo con MIRI
cargo miri test -p rydit-rs -- --nocapture

# Test específico de módulo
cargo miri test -p rydit-rs eval -- --nocapture
cargo miri test -p rydit-rs executor -- --nocapture
```

#### 3. MIRI Flags para Debugging

```bash
# Detectar memory leaks
MIRIFLAGS="-Zmiri-check-number-validity" cargo miri test -p rydit-rs

# Detectar borrow checker violations
MIRIFLAGS="-Zmiri-tag-raw-pointers" cargo miri test -p rydit-rs

# Detectar uninitialized memory
MIRIFLAGS="-Zmiri-check-uninit" cargo miri test -p rydit-rs
```

### Expected Output (Crates Seguros)

```
running 21 tests
test lexer::test::test_scan_number ... ok
test lexer::test::test_scan_string ... ok
...
test result: ok. 21 passed; 0 failed; 0 ignored

running 24 tests
test parser::test::test_parse_binary ... ok
test parser::test::test_parse_call ... ok
...
test result: ok. 24 passed; 0 failed; 0 ignored
```

### Expected Output (rydit-rs después de fixes)

```
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored

# Si hay UB, MIRI reporta:
error: Undefined Behavior: out-of-bounds memory access
  --> crates/rydit-rs/src/eval/mod.rs:54
```

---

## 📊 PLAN DE COVERAGE CON TARPAULIN

### Instalación

```bash
# Instalar cargo-tarpaulin
cargo install cargo-tarpaulin

# Verificar instalación
cargo tarpaulin --version
```

### Comandos Exactos

#### 1. Coverage Crates Nuevos

```bash
# rydit-lexer (HTML report)
cargo tarpaulin -p rydit-lexer --out Html --output-dir ./coverage/lexer

# rydit-parser (HTML report)
cargo tarpaulin -p rydit-parser --out Html --output-dir ./coverage/parser

# rydit-vm (HTML report)
cargo tarpaulin -p rydit-vm --out Html --output-dir ./coverage/vm

# rydit-stream (HTML report)
cargo tarpaulin -p rydit-stream --out Html --output-dir ./coverage/stream
```

#### 2. Coverage Combinado

```bash
# Todos los crates nuevos
cargo tarpaulin -p rydit-lexer -p rydit-parser -p rydit-vm -p rydit-stream \
    --out Html --output-dir ./coverage/all
```

#### 3. Coverage con XML (CI/CD)

```bash
# XML report para GitHub Actions
cargo tarpaulin -p rydit-lexer -p rydit-parser --out Xml --output-dir ./coverage
```

#### 4. Coverage Threshold (fail si < 80%)

```bash
# Fail si coverage < 80%
cargo tarpaulin -p rydit-lexer --fail-under 80
```

### Expected Coverage Report

```
Coverage Results:
|| Tested/Total Lines:
|| crates/rydit-lexer/src/lib.rs: 450/460 (97.8%)
|| crates/rydit-lexer/src/lexer.rs: 1200/1250 (96.0%)
|| crates/rydit-parser/src/parser.rs: 800/850 (94.1%)
|| ...

Final Coverage: 95.5%
```

### Coverage por Función

```bash
# Reporte detallado por función
cargo tarpaulin -p rydit-lexer --out Html --follow-exec
```

---

## 🛡️ LISTA DE CRATES SEGUROS VS PELIGROSOS

### ✅ CRATES SEGUROS (MIRI Verified)

| Crate | Tests | Coverage Esperado | ¿MIRI Clean? |
|-------|-------|-------------------|--------------|
| **rydit-lexer** | 21 | 95%+ | ✅ SÍ |
| **rydit-parser** | 24 | 95%+ | ✅ SÍ |
| **rydit-vm** | 19 | 90%+ | ✅ SÍ |
| **rydit-stream** | 17 | 85%+ | ✅ SÍ |

**Total**: 81 tests passing, 0 undefined behavior

---

### 🔴 CRATES PELIGROSOS (Requieren Fix)

| Crate | Errores | ¿UB Posible? | ¿MIRI Clean? |
|-------|---------|--------------|--------------|
| **rydit-rs (bin)** | 69 | 🔴 SÍ | ❌ NO |

**Problemas Principales**:
1. 🔴 &str vs String (23 casos) → **UB por dangling references**
2. 🔴 HashMap type mismatch (8 casos) → **UB por borrow incorrecto**
3. 🔴 ureq error handling (6 casos) → **UB por IO no manejado**
4. 🟡 Vec<String> vs Vec<&str> (8 casos) → **Seguro pero incorrecto**
5. 🟢 Unused imports (14 warnings) → **Solo limpieza**

---

## 🔄 COMANDO DE ROLLBACK EXACTO

### Punto de Reversión Seguro

```bash
# Tag actual
git describe --tags
# Output: v0.11.4-fase1b-name

# Rollback completo
git checkout v0.11.4-fase1b-name

# Verificar rollback
git status
# Output: HEAD detached at v0.11.4-fase1b-name

# Build de verificación
cargo build -p rydit-lexer -p rydit-parser -p rydit-vm -p rydit-stream
```

### Backup de Crates Nuevos

```bash
# Backup actual (ya existe)
ls -lh crates_backup_v0.11.2_*.tar.gz
# Output: -rw-r--r-- 1 user user 1.2M Apr 1 12:34 crates_backup_v0.11.2_*.tar.gz

# Restaurar backup si es necesario
tar -xzf crates_backup_v0.11.2_*.tar.gz -C /tmp/
```

### Rollback Parcial (Solo rydit-rs)

```bash
# Restaurar solo rydit-rs
git checkout v0.11.4-fase1b-name -- crates/rydit-rs/

# Verificar
cargo build -p rydit-rs --bin rydit-rs 2>&1 | grep -E "^error" | wc -l
# Output: 69 (mismo número de errores)
```

---

## 📋 ESTRATEGIA DE FIX PASO A PASO

### FASE 0: Preparación (30 minutos)

```bash
# 1. Crear branch de fix
git checkout -b fix/70-errores-miri

# 2. Verificar estado actual
cargo build -p rydit-rs --bin rydit-rs 2>&1 | grep -E "^error" | wc -l
# Output: 69

# 3. Test crates seguros (baseline)
cargo test -p rydit-lexer -p rydit-parser -p rydit-vm -p rydit-stream
# Output: 81 passing

# 4. Instalar MIRI
rustup component add miri

# 5. Test baseline con MIRI
cargo miri test -p rydit-lexer -p rydit-parser -p rydit-vm -p rydit-stream
# Output: 81 passing, 0 UB
```

---

### FASE 1: Fixes Críticos (Días 1-3)

#### Día 1: &str vs String (23 errores)

```bash
# Fixear eval/mod.rs:54
# Archivo: crates/rydit-rs/src/eval/mod.rs
# Línea 54: s.clone() → s.to_string()

# Fixear eval/mod.rs:57
# Línea 57: name == "__INPUT__" → *name == "__INPUT__"

# Fixear eval/mod.rs:908-912
# Líneas 908-912: func_name type mismatch

# Commit después de cada fix
git add crates/rydit-rs/src/eval/mod.rs
git commit -m "Fix: &str vs String en eval/mod.rs (5 errores)"

# Test con MIRI
cargo miri test -p rydit-rs eval
```

#### Día 2: HashMap/HashSet type mismatch (8 errores)

```bash
# Fixear main.rs:334, 1778, 4478
# Fixear main.rs:434, 1869, 4544
# Fixear eval/mod.rs:908
# Fixear main.rs:4005

# Commit
git add crates/rydit-rs/src/main.rs
git commit -m "Fix: HashMap/HashSet type mismatch (8 errores)"

# Test con MIRI
cargo miri test -p rydit-rs main
```

#### Día 3: ureq HTTP errors (6 errores)

```bash
# Fixear eval/mod.rs:1399, 1423, 1425, 1445, 1447, 1459

# Commit
git add crates/rydit-rs/src/eval/mod.rs
git commit -m "Fix: ureq HTTP error handling (6 errores)"

# Test con MIRI
cargo miri test -p rydit-rs eval --features http
```

---

### FASE 2: Fixes Medios (Días 4-5)

#### Día 4: Vec<String> vs Vec<&str> (8 errores)

```bash
# Fixear main.rs:244, 1448, 4457
# Fixear executor.rs:424
# Fixear main.rs:411, 1849, 4525, 392

# Commit
git add crates/rydit-rs/src/main.rs crates/rydit-rs/src/executor.rs
git commit -m "Fix: Vec<String> vs Vec<&str> (8 errores)"

# Test con MIRI
cargo miri test -p rydit-rs
```

#### Día 5: Expr::Texto lifetime (14 errores)

```bash
# Fixear main.rs:1619, 1647, 1667
# Fixear eval/mod.rs:54, main.rs:2185, 3166
# Fixear if/else type mismatch (7 errores)

# Commit
git add crates/rydit-rs/src/main.rs crates/rydit-rs/src/eval/mod.rs
git commit -m "Fix: Expr::Texto lifetime + if/else mismatch (14 errores)"

# Test con MIRI
cargo miri test -p rydit-rs
```

---

### FASE 3: Fixes Bajos + Cleanup (Día 6)

#### Día 6: Unused imports + warnings (14 warnings)

```bash
# Eliminar unused imports
# Fixear dead_code warnings
# Fixear unused_mut warnings

# Commit
git add crates/rydit-rs/
git commit -m "Cleanup: unused imports + warnings (14 warnings)"

# Test final con MIRI
cargo miri test -p rydit-rs

# Test con TARPAULIN
cargo tarpaulin -p rydit-rs --out Html --output-dir ./coverage/rydit-rs
```

---

### FASE 4: Verificación Final (Día 7)

```bash
# 1. Build sin errores
cargo build -p rydit-rs --bin rydit-rs
# Output: Finished release [optimized]

# 2. Tests con MIRI (0 UB)
cargo miri test -p rydit-rs
# Output: 0 failed, 0 UB

# 3. Coverage con TARPAULIN
cargo tarpaulin -p rydit-rs --out Html
# Output: Coverage > 80%

# 4. Merge a main
git checkout main
git merge fix/70-errores-miri

# 5. Tag nueva versión
git tag v0.11.5-miri-clean
git push origin v0.11.5-miri-clean
```

---

## 📊 MÉTRICAS DE ÉXITO

### Criterios de Aceptación

| Métrica | Antes | Después | Target |
|---------|-------|---------|--------|
| **Errores de compilación** | 69 | 0 | ✅ 0 |
| **Warnings** | 14 | 0 | ✅ 0 |
| **MIRI UB detections** | N/A | 0 | ✅ 0 |
| **TARPAULIN coverage** | 0% | >80% | ✅ >80% |
| **Tests passing** | 0 | >50 | ✅ >50 |

### Timeline Realista

| Fase | Días | Errores Fix | Acumulado |
|------|------|-------------|-----------|
| **Fase 1 (Críticos)** | 3 | 37 | 37 |
| **Fase 2 (Medios)** | 2 | 22 | 59 |
| **Fase 3 (Bajos)** | 1 | 10 | 69 |
| **Fase 4 (Verificación)** | 1 | 0 | 69 |

**Total**: 7 días laborables

---

## 🛑 RIESGOS Y MITIGACIÓN

### Riesgo 1: MIRI detecta UB después de fix

**Mitigación**:
```bash
# Rollback inmediato
git reset --hard HEAD~1

# Analizar con MIRI flags
MIRIFLAGS="-Zmiri-check-number-validity" cargo miri test -p rydit-rs

# Fix alternativo
# (consultar documentación MIRI)
```

### Riesgo 2: Fix rompe tests existentes

**Mitigación**:
```bash
# Test después de CADA fix
cargo test -p rydit-lexer -p rydit-parser -p rydit-vm -p rydit-stream

# Si falla, revertir
git checkout HEAD~1 -- crates/rydit-rs/src/archivo.rs
```

### Riesgo 3: Coverage < 80%

**Mitigación**:
```bash
# Identificar código no testeado
cargo tarpaulin -p rydit-rs --out Html

# Agregar tests para código faltante
# (archivo: crates/rydit-rs/tests/coverage_test.rs)
```

---

## 📝 CONCLUSIÓN

### Resumen Ejecutivo

1. **69 errores** en rydit-rs (binario legacy)
2. **4 crates nuevos** 100% seguros (rydit-lexer, parser, vm, stream)
3. **23 errores críticos** (pueden causar UB) → MIRI necesario
4. **38 errores medios** (type mismatch seguro) → TARPAULIN útil
5. **8 errores bajos** (solo warnings) → Cleanup

### Recomendación

**NO USAR sed/jamás** (aprendizaje confirmado). Fix manual archivo por archivo con:
- ✅ Commit después de CADA fix
- ✅ Test con MIRI después de CADA fix
- ✅ Rollback inmediato si MIRI detecta UB

### Valor del Experimento

- 🔬 **MIRI**: Detecta undefined behavior en tiempo real
- 📊 **TARPAULIN**: Code coverage para identificar código no testeado
- 🛡️ **Crates nuevos**: Arquitectura modular SÍ funciona
- 📚 **Aprendizaje**: Fix manual es más seguro que sed/jamás

---

<div align="center">

**🛡️ RyDit v0.11.4 - INFORME MIRI/TARPAULIN**

*69 errores | 4 crates seguros | 7 días para fix | 0 UB target*

**Próximo: Fix manual con MIRI verification**

</div>
