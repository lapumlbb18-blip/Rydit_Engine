# 🛡️ INFORME FORENSE DE ERRORES - v0.11.2

**Fecha**: 2026-04-02  
**Total Errores**: 76  
**Paquete**: `rydit-rs`  
**Binario**: `rydit-rs`  

---

## 📊 RESUMEN EJECUTIVO

### Distribución por Tipo de Error

| Tipo | Código | Cantidad | % | Peligrosidad |
|------|--------|----------|---|--------------|
| **Type Mismatch** | E0308 | 38 | 50% | 🟡 MEDIO |
| **Cannot Find Value** | E0425 | 6 | 8% | 🟢 BAJO |
| **Trait Not Implemented** | E0277 | 13 | 17% | 🟢 BAJO |
| **Pattern/Struct Field** | E0026, E0027 | 2 | 3% | 🔴 ALTO |
| **Undeclared Type** | E0433 | 4 | 5% | 🟢 BAJO |
| **Method Not Found** | E0599 | 3 | 4% | 🟢 BAJO |
| **Unstable Feature** | E0658 | 3 | 4% | 🟢 BAJO |
| **Other** | Varios | 7 | 9% | 🟡 MEDIO |

### Distribución por Archivo

| Archivo | Errores | % | Peligrosidad Promedio |
|---------|---------|---|----------------------|
| `main.rs` | 58 | 76% | 🟡 MEDIO |
| `eval/mod.rs` | 11 | 14% | 🟢 BAJO |
| `repl.rs` | 3 | 4% | 🟢 BAJO |
| `executor.rs` | 2 | 3% | 🟢 BAJO |
| `cli.rs` | 1 | 1% | 🟢 BAJO |
| `modules/level.rs` | 1 | 1% | 🟢 BAJO |

---

## 🔍 ANÁLISIS DETALLADO POR ARCHIVO

### ARCHIVO: `crates/rydit-rs/src/main.rs` (58 errores)

#### **ERROR #1-6: Variables no encontradas (`name` → `func_name`)**

**ERROR #1**
- **Línea**: 309
- **Tipo**: E0425 (cannot find value `name`)
- **Código**:
  ```rust
  // Línea 306-312
  } else {
      println!("[ERROR] Función '{}' no definida", name);
      //                                            ^^^^ ERROR
  }
  ```
- **Contexto**: Función `ejecutar_stmt()`, bloque `Stmt::Call`
- **Causa**: Script automático cambió `name` por `func_name` pero dejó una referencia
- **Fix**: Cambiar `name` → `func_name`
- **Peligrosidad**: 🟢 BAJO (1 línea, cambio mecánico)

**ERROR #2**
- **Línea**: 1739
- **Tipo**: E0425
- **Código**:
  ```rust
  // Línea 1736-1742
  } else {
      let func_data = funcs.get(name).map(|(p, b)| (p.clone(), b.clone()));
      //                                ^^^^ ERROR
  ```
- **Contexto**: Función `evaluar_expr_gfx()`, llamada a función de usuario
- **Fix**: Cambiar `name` → `func_name`
- **Peligrosidad**: 🟢 BAJO

**ERROR #3**
- **Línea**: 2187
- **Tipo**: E0425
- **Código**:
  ```rust
  // Línea 2184-2190
  Expr::Var(name) => {
      if func_name == "__INPUT__" {
         // ^^^^^^^^^ ERROR - func_name no existe en este scope
      }
  ```
- **Contexto**: Función `evaluar_expr_gfx()`, match `Expr::Var`
- **Causa**: `func_name` fue introducido pero no está en scope aquí
- **Fix**: Cambiar `func_name` → `name` (aquí SÍ es `name`)
- **Peligrosidad**: 🟢 BAJO

**ERROR #4**
- **Línea**: 3057
- **Tipo**: E0425
- **Código**:
  ```rust
  // Línea 3054-3060
  }
  
  Valor::Error(format!("Función '{}' no soportada", name))
  //                                                    ^^^^ ERROR
  ```
- **Contexto**: Función `evaluar_expr_gfx()`, caso default
- **Fix**: Cambiar `name` → `func_name`
- **Peligrosidad**: 🟢 BAJO

**ERROR #5**
- **Línea**: 3168
- **Tipo**: E0425
- **Código**: Similar al ERROR #3 pero en `evaluar_expr_migui()`
- **Fix**: Cambiar `func_name` → `name`
- **Peligrosidad**: 🟢 BAJO

**ERROR #6**
- **Línea**: 4462
- **Tipo**: E0425
- **Código**:
  ```rust
  // Línea 4459-4465
  Stmt::Call { name, args } => {
      let _ = evaluar_expr_migui(
          &Expr::Call { callee: Box::new(Expr::Var(func_name)),
                                              // ^^^^^^^^^ ERROR
  ```
- **Contexto**: Función `ejecutar_stmt_migui()`
- **Causa**: Pattern incorrecto (ver ERRORES CRÍTICOS abajo)
- **Fix**: Requiere reestructurar pattern matching completo
- **Peligrosidad**: 🔴 ALTO (relacionado con ERROR CRÍTICO #1)

---

#### **ERROR #7-18: Comparaciones `&str` vs `str` (E0277)**

**ERROR #7-16** (10 errores idénticos)
- **Líneas**: 1268-1278
- **Tipo**: E0277 (can't compare `&str` with `str`)
- **Código**:
  ```rust
  // Línea 1265-1280
  Stmt::Assign { name, value } => {
      let valor = evaluar_expr_gfx(value, executor, input, funcs);
      if name == "x"  // ← ERROR: name es &&str, "x" es &str
          || name == "y"
          || name == "velocidad"
          // ... 10 comparaciones en total
  ```
- **Contexto**: Función `ejecutar_stmt_gfx()`, logging de asignaciones
- **Causa**: `name` es `&&str` (doble referencia) por el pattern matching
- **Fix**: Cambiar `name` → `*name` O `if &name == "x"`
- **Peligrosidad**: 🟢 BAJO (cambio mecánico en 10 líneas)

---

#### **ERROR #19-24: Type Mismatch en HashMap (`&str` vs `String`)**

**ERROR #19-20**
- **Línea**: 244
- **Tipo**: E0308 (mismatched types)
- **Código**:
  ```rust
  // Línea 241-246
  Stmt::Function { name, params, body } => {
      funcs.insert(name.clone(), (params.clone(), body.clone()));
      //             ^^^^^^^^^^^^ ERROR: expected String, found &str
      //                          ^^^^^^^^^^^^^^ ERROR: expected Vec<String>, found Vec<&str>
  ```
- **Contexto**: Función `ejecutar_stmt()`, registro de funciones
- **Causa**: `funcs` es `HashMap<String, (Vec<String>, Vec<Stmt>)>` pero `name` es `&str`
- **Fix**: 
  ```rust
  funcs.insert(name.to_string(), (params.iter().map(|s| s.to_string()).collect(), body.clone()));
  ```
- **Peligrosidad**: 🟡 MEDIO (requiere entender estructura de funcs)

**ERROR #21-22**
- **Línea**: 1448
- **Tipo**: E0308
- **Código**: Similar al anterior pero en `ejecutar_stmt_gfx()`
- **Fix**: Mismo fix que ERROR #19-20
- **Peligrosidad**: 🟡 MEDIO

**ERROR #23-24**
- **Línea**: 4457
- **Tipo**: E0308
- **Código**: Similar pero en `ejecutar_stmt_migui()`
- **Fix**: Mismo fix
- **Peligrosidad**: 🟡 MEDIO

---

#### **ERROR #25-27: Pattern Matching INCORRECTO en Stmt::Call** 🔴 CRÍTICO

**ERROR #25**
- **Línea**: 248
- **Tipo**: E0277 (trait bound not satisfied)
- **Código**:
  ```rust
  // Línea 246-254
  Stmt::Call { callee, args } => {
      let func_name = if let Expr::Var(name) = callee.as_ref() {
          *name  // ← ERROR: callee es &&str, no Box<Expr>
  ```
- **Contexto**: Función `ejecutar_stmt()`
- **Causa CRÍTICA**: El AST cambió, `Stmt::Call` ahora tiene `callee: &'a str` NO `Box<Expr>`
- **Fix**: 
  ```rust
  Stmt::Call { callee, args } => {
      let func_name = callee;  // callee YA ES &str
  ```
- **Peligrosidad**: 🔴 ALTO (cambio de arquitectura del AST)

**ERROR #26**
- **Línea**: 252
- **Tipo**: E0308
- **Código**: Consecuencia del ERROR #25
- **Fix**: Depende del fix del ERROR #25
- **Peligrosidad**: 🔴 ALTO

**ERROR #27**
- **Línea**: 1451
- **Tipo**: E0277
- **Código**: Similar en `ejecutar_stmt_gfx()`
- **Fix**: Mismo fix que ERROR #25
- **Peligrosidad**: 🔴 ALTO

---

#### **ERROR #28-33: Import de módulos (Lizer no encontrado)**

**ERROR #28**
- **Línea**: 395
- **Tipo**: E0433 (use of undeclared type `Lizer`)
- **Código**:
  ```rust
  // Línea 391-397
  importing_stack.push(module.clone());
  
  let tokens = Lizer::new(&module_content).scan();
  //           ^^^^^ ERROR
  ```
- **Contexto**: Función `ejecutar_stmt()`, manejo de `Stmt::Import`
- **Causa**: `Lizer` no está importado en `main.rs`
- **Fix**: Agregar `use rydit_lexer::Lizer;` en imports (línea ~50)
- **Peligrosidad**: 🟢 BAJO

**ERROR #29-30**
- **Línea**: 399-400
- **Tipo**: E0308
- **Código**:
  ```rust
  let program = match parser.parse() {
      Ok(p) => p,  // ← ERROR: parser.parse() devuelve (Program, Vec<Error>)
      Err(e) => {  // ← ERROR: ya no hay Err
  ```
- **Contexto**: Mismo que ERROR #28
- **Causa**: API de parser cambió a devolver tupla `(Program, Vec<RyDitError>)`
- **Fix**:
  ```rust
  let (program, errors) = parser.parse();
  if !errors.is_empty() {
      for e in &errors {
          println!("[WARNING] {}", e);
      }
  }
  ```
- **Peligrosidad**: 🟡 MEDIO (cambio de API)

**ERROR #31-33**
- **Líneas**: 1833, 4510, 68 (repl.rs)
- **Tipo**: E0433, E0308
- **Código**: Similar al ERROR #28-30
- **Fix**: Mismo fix + agregar import en cada archivo
- **Peligrosidad**: 🟢 BAJO

---

#### **ERROR #34-38: Feature inestable `str_as_str`**

**ERROR #34**
- **Línea**: 345
- **Tipo**: E0658 (use of unstable library feature `str_as_str`)
- **Código**:
  ```rust
  // Línea 341-347
  if let Some(alias_name) = alias {
      alias_name.as_str()  // ← ERROR: as_str() en &str es inestable
  ```
- **Contexto**: Función `ejecutar_stmt()`, manejo de imports
- **Causa**: `alias` es `&str`, no `String`. `as_str()` en `&str` es inestable
- **Fix**: Simplemente usar `alias_name` directamente (ya es &str)
  ```rust
  if let Some(alias_name) = alias {
      alias_name  // Ya es &str
  ```
- **Peligrosidad**: 🟢 BAJO

**ERROR #35-38**
- **Líneas**: 1789, 4483, 345
- **Tipo**: E0658
- **Código**: Similar al ERROR #34
- **Fix**: Mismo fix
- **Peligrosidad**: 🟢 BAJO

---

#### **ERROR #39-44: Slice/Vec type mismatch**

**ERROR #39**
- **Línea**: 334
- **Tipo**: E0308
- **Código**:
  ```rust
  // Línea 330-336
  for s in &program.statements {
      if let Stmt::Function { name, .. } = s {
          original_funcs.push(name);  // ← ERROR: expected String, found &str
  ```
- **Contexto**: Función `ejecutar_stmt()`, colección de funciones originales
- **Fix**: Cambiar `name` → `name.to_string()`
- **Peligrosidad**: 🟢 BAJO

**ERROR #40**
- **Línea**: 392
- **Tipo**: E0308
- **Código**: Similar al ERROR #39
- **Fix**: Mismo fix
- **Peligrosidad**: 🟢 BAJO

**ERROR #41**
- **Línea**: 411
- **Tipo**: E0308
- **Código**: Similar
- **Fix**: Mismo fix
- **Peligrosidad**: 🟢 BAJO

**ERROR #42**
- **Línea**: 434
- **Tipo**: E0308
- **Código**:
  ```rust
  // Línea 430-436
  for s in &program.statements {
      if let Stmt::Function { name, .. } = s {
          funcs_to_remove.insert(name);  // ← ERROR: HashSet<String>, name es &str
  ```
- **Fix**: Cambiar `name` → `name.to_string()`
- **Peligrosidad**: 🟢 BAJO

**ERROR #43-44**
- **Líneas**: 1849, 4525
- **Tipo**: E0308
- **Código**: Similar
- **Fix**: Mismo fix
- **Peligrosidad**: 🟢 BAJO

---

#### **ERROR #45-54: Expr::BinOp no existe (cambió a Expr::Binary)**

**ERROR #45**
- **Línea**: 4075
- **Tipo**: E0599 (no variant named `BinOp`)
- **Código**:
  ```rust
  // Línea 4073-4078
  Valor::Error(format!("Función '{}' no soportada en expresiones", func_name))
  Expr::BinOp { left, op, right } => {  // ← ERROR: variante se llama Binary, no BinOp
  ```
- **Contexto**: Función `evaluar_expr_migui()`
- **Causa**: El AST cambió `Expr::BinOp` por `Expr::Binary`
- **Fix**: Cambiar `Expr::BinOp` → `Expr::Binary`
- **Peligrosidad**: 🟢 BAJO (cambio de nombre)

**ERROR #46-54**
- **Líneas**: 1619, 1647 (2 errores), 1667, 1830, 2185, 3166, 4508
- **Tipo**: E0308
- **Código**: Varios errores de type mismatch relacionados con AST
- **Fix**: Revisar cada caso individualmente
- **Peligrosidad**: 🟡 MEDIO

---

#### **ERROR #55-58: Pattern Stmt::Call incorrecto** 🔴 CRÍTICO

**ERROR #55**
- **Línea**: 4459
- **Tipo**: E0026 (variant does not have a field named `name`)
- **Código**:
  ```rust
  // Línea 4459-4463
  Stmt::Call { name, args } => {  // ← ERROR: campo se llama callee, no name
  ```
- **Contexto**: Función `ejecutar_stmt_migui()`
- **Causa CRÍTICA**: AST cambió, `Stmt::Call` tiene campo `callee`, no `name`
- **Fix**:
  ```rust
  Stmt::Call { callee, args } => {
      let func_name = callee;  // callee es &str
  ```
- **Peligrosidad**: 🔴 ALTO (cambio de arquitectura)

**ERROR #56**
- **Línea**: 4459
- **Tipo**: E0027 (pattern does not mention field `callee`)
- **Código**: Mismo que ERROR #55
- **Fix**: Mismo fix
- **Peligrosidad**: 🔴 ALTO

**ERROR #57**
- **Línea**: 4478
- **Tipo**: E0308
- **Código**: Consecuencia del ERROR #55
- **Fix**: Depende del fix del ERROR #55
- **Peligrosidad**: 🔴 ALTO

**ERROR #58**
- **Línea**: 4544
- **Tipo**: E0308
- **Código**: Consecuencia del ERROR #55
- **Fix**: Depende del fix del ERROR #55
- **Peligrosidad**: 🔴 ALTO

---

### ARCHIVO: `crates/rydit-rs/src/eval/mod.rs` (11 errores)

#### **ERROR #59-60: Type mismatch en Expr::Texto**

**ERROR #59**
- **Línea**: 54
- **Tipo**: E0308
- **Código**:
  ```rust
  // Línea 51-56
  ) -> Valor {
      match expr {
          Expr::Num(n) => Valor::Num(*n),
          Expr::Texto(s) => Valor::Texto(s.clone()),  // ← ERROR: s es &str, espera String
  ```
- **Contexto**: Función `evaluar_expr()`
- **Causa**: `Expr::Texto(&'a str)` pero `Valor::Texto(String)`
- **Fix**: Cambiar `s.clone()` → `s.to_string()`
- **Peligrosidad**: 🟢 BAJO

**ERROR #60**
- **Línea**: 57
- **Tipo**: E0277
- **Código**:
  ```rust
  // Línea 55-60
  Expr::Var(name) => {
      if name == "__INPUT__" {  // ← ERROR: name es &&str
  ```
- **Fix**: Cambiar `name` → `*name` O `&name == "__INPUT__"`
- **Peligrosidad**: 🟢 BAJO

---

#### **ERROR #61-62: HashMap key type mismatch**

**ERROR #61**
- **Línea**: 908
- **Tipo**: E0277
- **Código**:
  ```rust
  // Línea 905-912
  let func_name_final = if func_name.contains("::") {
      if funcs.contains_key(&func_name) {  // ← ERROR: funcs es HashMap<String, _>
  ```
- **Contexto**: Función `evaluar_expr()`, resolución de funciones
- **Fix**: Cambiar `&func_name` → `func_name` (si func_name es String)
- **Peligrosidad**: 🟢 BAJO

**ERROR #62**
- **Línea**: 912
- **Tipo**: E0308 (if/else incompatible types)
- **Código**:
  ```rust
  if funcs.contains_key(&func_name) {
      func_name.clone()  // String
  } else {
      func_name.split("::").last().unwrap_or(&func_name).to_string()  // También String
  }
  ```
- **Causa**: Error en mensaje, probablemente ya está bien
- **Fix**: Revisar contexto completo
- **Peligrosidad**: 🟡 MEDIO

---

#### **ERROR #63-69: HTTP functions (ureq API)**

**ERROR #63**
- **Línea**: 1399
- **Tipo**: E0599
- **Código**:
  ```rust
  // Línea 1396-1402
  return match ureq::get(&url).call().into_string() {
  //                             ^^^^^^^^^^^ ERROR: call() devuelve Result, no Response
  ```
- **Contexto**: Función `evaluar_expr()`, `http::get()`
- **Causa**: API de ureq cambió, hay que hacer `.call().expect("msg").into_string()`
- **Fix**:
  ```rust
  return match ureq::get(&url).call().expect("HTTP GET failed").into_string() {
  ```
- **Peligrosidad**: 🟢 BAJO

**ERROR #64-65**
- **Línea**: 1423, 1425
- **Tipo**: E0308
- **Código**:
  ```rust
  // Línea 1420-1427
  return match ureq::post(&url).send_string(&data)
      .map(|r| r.into_string())
      .unwrap_or(Err("POST error".to_string())) {  // ← ERROR: Err espera ureq::Error, no String
      Ok(response) => Valor::Texto(response),
      Err(e) => Valor::Error(e),  // ← ERROR: e es ureq::Error, espera String
  ```
- **Fix**:
  ```rust
  return match ureq::post(&url).send_string(&data) {
      Ok(r) => match r.into_string() {
          Ok(response) => Valor::Texto(response),
          Err(e) => Valor::Error(e.to_string()),
      },
      Err(e) => Valor::Error(e.to_string()),
  };
  ```
- **Peligrosidad**: 🟡 MEDIO

**ERROR #66-69**
- **Líneas**: 1445, 1447, 1459
- **Tipo**: E0308, E0599
- **Código**: Similar a ERROR #64-65 pero para `http::put()` y `http::delete()`
- **Fix**: Mismo fix
- **Peligrosidad**: 🟡 MEDIO

---

### ARCHIVO: `crates/rydit-rs/src/repl.rs` (3 errores)

#### **ERROR #70-72: Lizer + parser.parse()**

**ERROR #70**
- **Línea**: 68
- **Tipo**: E0433
- **Código**:
  ```rust
  // Línea 65-70
  let tokens = Lizer::new(input).scan();  // ← ERROR: Lizer no importado
  ```
- **Fix**: Agregar `use rydit_lexer::Lizer;` en imports
- **Peligrosidad**: 🟢 BAJO

**ERROR #71-72**
- **Línea**: 72, 85
- **Tipo**: E0308
- **Código**:
  ```rust
  // Línea 68-88
  match parser.parse() {
      Ok(program) => {  // ← ERROR: parser.parse() devuelve tupla, no Result
      // ...
      Err(e) => {  // ← ERROR: ya no hay Err
  ```
- **Fix**:
  ```rust
  let (program, errors) = parser.parse();
  if !errors.is_empty() {
      for e in &errors {
          println!("[ERROR] {}", e);
      }
  }
  // Usar program directamente
  ```
- **Peligrosidad**: 🟡 MEDIO

---

### ARCHIVO: `crates/rydit-rs/src/executor.rs` (2 errores)

#### **ERROR #73-74: Function registration type mismatch**

**ERROR #73-74**
- **Línea**: 424
- **Tipo**: E0308
- **Código**:
  ```rust
  // Línea 421-426
  Stmt::Function { name, params, body } => {
      funcs.insert(name.clone(), (params.clone(), body.clone()));
      //             ^^^^^^^^^^^^ ERROR: expected String, found &str
      //                          ^^^^^^^^^^^^^^ ERROR: expected Vec<String>, found Vec<&str>
  ```
- **Contexto**: Función `cargar_programa_migui()`
- **Fix**:
  ```rust
  funcs.insert(
      name.to_string(),
      (params.iter().map(|s| s.to_string()).collect(), body.clone())
  );
  ```
- **Peligrosidad**: 🟡 MEDIO

---

### ARCHIVO: `crates/rydit-rs/src/cli.rs` (1 error)

#### **ERROR #75: Warning no tratado**

**ERROR #75**
- **Línea**: 83
- **Tipo**: Warning tratado como error
- **Código**: Probablemente unused variable o similar
- **Fix**: Revisar warning específico
- **Peligrosidad**: 🟢 BAJO

---

### ARCHIVO: `crates/rydit-rs/src/modules/level.rs` (1 error)

#### **ERROR #76: Warning no tratado**

**ERROR #76**
- **Línea**: 247-250
- **Tipo**: Warning tratado como error
- **Código**: Probablemente unused variables
- **Fix**: Revisar warning específico
- **Peligrosidad**: 🟢 BAJO

---

## 🎯 CLASIFICACIÓN POR PELIGROSIDAD

### 🔴 ALTO (12 errores - 16%)

**Características**:
- Requieren entender cambios de arquitectura del AST
- Pattern matching incorrecto en `Stmt::Call`
- No son fixes mecánicos, hay que reestructurar código

**Archivos críticos**:
- `main.rs`: Líneas 246-254, 1448-1454, 4459-4463

**Fix strategy**:
1. Primero entender el nuevo AST (`Stmt::Call { callee: &'a str, args: Vec<Expr> }`)
2. Reescribir pattern matching para usar `callee` directamente
3. Testear después de cada cambio

---

### 🟡 MEDIO (24 errores - 32%)

**Características**:
- Type mismatches que requieren conversiones explícitas
- Cambios de API (`parser.parse()` ahora devuelve tupla)
- HTTP functions con ureq API cambiada

**Archivos**:
- `main.rs`: Líneas 244, 334, 392, 411, 434
- `eval/mod.rs`: Líneas 1423-1427, 1445-1449
- `executor.rs`: Línea 424
- `repl.rs`: Líneas 72, 85

**Fix strategy**:
1. Agregar `.to_string()` donde sea necesario
2. Cambiar `match parser.parse()` por desestructuración de tupla
3. Fixear HTTP functions con `.expect().into_string()`

---

### 🟢 BAJO (40 errores - 52%)

**Características**:
- Fixes mecánicos de 1 línea
- Variables renombradas incorrectamente (`name` → `func_name`)
- Imports faltantes (`Lizer`)
- Comparaciones `&str` vs `str`

**Archivos**:
- `main.rs`: Líneas 309, 1268-1278, 1739, 2187, 3057, 3168, 345, 395-400, 1833-1838, 4510-4515
- `eval/mod.rs`: Líneas 54, 57, 908
- `repl.rs`: Línea 68

**Fix strategy**:
1. Buscar y reemplazar (con cuidado de contexto)
2. Agregar imports faltantes
3. Agregar `.to_string()` o `*` para dereferenciar

---

## 📋 GUÍA DE FIX MANUAL PASO A PASO

### FASE 1: FIXES SEGUROS (🟢 BAJO) - 2-3 horas

#### Paso 1.1: Agregar imports faltantes (15 min)

**Archivos**: `main.rs`, `repl.rs`

```rust
// En crates/rydit-rs/src/main.rs (línea ~50)
use rydit_lexer::Lizer;

// En crates/rydit-rs/src/repl.rs (línea ~9)
use rydit_lexer::Lizer;
```

**Verificación**:
```bash
cargo check -p rydit-rs --bin rydit-rs 2>&1 | grep "E0433"
# Debería desaparecer error E0433
```

---

#### Paso 1.2: Fix `name` → `func_name` (30 min)

**Archivos**: `main.rs`

**Líneas exactas**:
- Línea 309: `name` → `func_name`
- Línea 1739: `name` → `func_name`
- Línea 3057: `name` → `func_name`

**Líneas donde es al revés** (`func_name` → `name`):
- Línea 2187: `func_name` → `name`
- Línea 3168: `func_name` → `name`

**Verificación**:
```bash
cargo check -p rydit-rs --bin rydit-rs 2>&1 | grep "E0425"
# Deberían desaparecer 6 errores E0425
```

---

#### Paso 1.3: Fix comparaciones `&str` vs `str` (30 min)

**Archivo**: `main.rs`

**Líneas 1268-1278**:
```rust
// ANTES
if name == "x"
    || name == "y"
    // ...

// DESPUÉS
if *name == "x"
    || *name == "y"
    // ...
```

**Verificación**:
```bash
cargo check -p rydit-rs --bin rydit-rs 2>&1 | grep "E0277"
# Deberían desaparecer 10 errores E0277
```

---

#### Paso 1.4: Fix feature inestable `as_str()` (15 min)

**Archivo**: `main.rs`

**Líneas**: 345, 1789, 4483

```rust
// ANTES
if let Some(alias_name) = alias {
    alias_name.as_str()

// DESPUÉS
if let Some(alias_name) = alias {
    alias_name  // Ya es &str
```

**Verificación**:
```bash
cargo check -p rydit-rs --bin rydit-rs 2>&1 | grep "E0658"
# Deberían desaparecer 3 errores E0658
```

---

#### Paso 1.5: Fix type mismatches simples (30 min)

**Archivo**: `main.rs`

**Líneas**: 334, 392, 411, 434, 1849, 4525

```rust
// ANTES
original_funcs.push(name);

// DESPUÉS
original_funcs.push(name.to_string());
```

**Verificación**:
```bash
cargo check -p rydit-rs --bin rydit-rs 2>&1 | grep "E0308" | wc -l
# Debería reducirse de ~38 a ~20
```

---

### FASE 2: FIXES MEDIOS (🟡 MEDIO) - 3-4 horas

#### Paso 2.1: Fix parser.parse() API (1 hora)

**Archivos**: `main.rs`, `repl.rs`

**Líneas en main.rs**: 399-400, 1837-1838, 4514-4515

```rust
// ANTES
let program = match parser.parse() {
    Ok(p) => p,
    Err(e) => {
        println!("[ERROR] {}", e);

// DESPUÉS
let (program, errors) = parser.parse();
if !errors.is_empty() {
    for e in &errors {
        println!("[WARNING] Error parseando: {}", e);
    }
}
```

**Verificación**:
```bash
cargo check -p rydit-rs --bin rydit-rs 2>&1 | grep "E0308" | wc -l
# Debería reducirse significativamente
```

---

#### Paso 2.2: Fix HTTP functions (1 hora)

**Archivo**: `eval/mod.rs`

**Líneas**: 1399, 1423-1427, 1445-1449, 1459

```rust
// ANTES (línea 1399)
return match ureq::get(&url).call().into_string() {
    Ok(response) => Valor::Texto(response),
    Err(e) => Valor::Error(e),

// DESPUÉS
return match ureq::get(&url).call() {
    Ok(response) => match response.into_string() {
        Ok(text) => Valor::Texto(text),
        Err(e) => Valor::Error(e.to_string()),
    },
    Err(e) => Valor::Error(e.to_string()),
};
```

**Repetir para `http::post()`, `http::put()`, `http::delete()`**

**Verificación**:
```bash
cargo check -p rydit-rs --bin rydit-rs 2>&1 | grep "E0599"
# Deberían desaparecer errores E0599
```

---

#### Paso 2.3: Fix function registration (1 hora)

**Archivos**: `main.rs`, `executor.rs`

**Líneas**: 244-245, 1448, 424, 4457

```rust
// ANTES
funcs.insert(name.clone(), (params.clone(), body.clone()));

// DESPUÉS
funcs.insert(
    name.to_string(),
    (params.iter().map(|s| s.to_string()).collect(), body.clone())
);
```

**Verificación**:
```bash
cargo check -p rydit-rs --bin rydit-rs 2>&1 | grep "E0308" | wc -l
# Debería quedar < 10
```

---

### FASE 3: FIXES CRÍTICOS (🔴 ALTO) - 4-6 horas

#### Paso 3.1: Fix Stmt::Call pattern matching (2-3 horas)

**Archivos**: `main.rs`

**Líneas críticas**: 246-254, 1448-1454, 4459-4463

**Contexto**: El AST cambió de:
```rust
// AST viejo (roto)
Stmt::Call {
    callee: Box<Expr<'a>>,
    args: Vec<Expr<'a>>,
}

// AST nuevo (correcto)
Stmt::Call {
    callee: &'a str,  // ¡AHORA ES &str DIRECTO!
    args: Vec<Expr<'a>>,
}
```

**Fix en `ejecutar_stmt()` (línea 246-254)**:

```rust
// ANTES (ROTO)
Stmt::Call { callee, args } => {
    let func_name = if let Expr::Var(name) = callee.as_ref() {
        *name
    } else {
        println!("[WARNING] Call requiere función válida");
        return (false, None);
    };

// DESPUÉS (CORRECTO)
Stmt::Call { callee, args } => {
    let func_name = callee;  // callee YA ES &str, no necesita pattern matching
    
    // El resto del código sigue igual
    if func_name == "sumar" || func_name == "restar" {
        // ...
```

**Fix en `ejecutar_stmt_gfx()` (línea 1448-1454)**:

```rust
// ANTES
Stmt::Call { callee, args } => {
    let func_name = if let Expr::Var(name) = callee.as_ref() {
        *name

// DESPUÉS
Stmt::Call { callee, args } => {
    let func_name = callee;  // Directo
```

**Fix en `ejecutar_stmt_migui()` (línea 4459-4473)**:

```rust
// ANTES (ROTO)
Stmt::Call { name, args } => {
    let _ = evaluar_expr_migui(
        &Expr::Call { callee: Box::new(Expr::Var(func_name)),

// DESPUÉS (CORRECTO)
Stmt::Call { callee, args } => {
    let func_name = callee;  // Usar callee directamente
    let _ = evaluar_expr_migui(
        &Expr::Call { callee: Box::new(Expr::Var(func_name)),
```

**Verificación**:
```bash
cargo check -p rydit-rs --bin rydit-rs 2>&1 | grep "E0026\|E0027"
# Deberían desaparecer errores E0026, E0027
```

---

#### Paso 3.2: Fix Expr::BinOp → Expr::Binary (1 hora)

**Archivo**: `main.rs`

**Línea**: 4075

```rust
// ANTES
Expr::BinOp { left, op, right } => {

// DESPUÉS
Expr::Binary { left, op, right } => {
```

**Verificación**:
```bash
cargo check -p rydit-rs --bin rydit-rs 2>&1 | grep "E0599"
# Debería desaparecer
```

---

#### Paso 3.3: Testear exhaustivamente (1-2 horas)

**Después de cada fix crítico**:

```bash
# 1. Compilar
cargo build -p rydit-rs --bin rydit-rs

# 2. Si compila, ejecutar demo simple
./target/release/rydit-rs --gfx demos/test_simple.rydit

# 3. Verificar que no haya crashes
# 4. Verificar que input funcione
# 5. Verificar que render funcione
```

---

## 🚫 ZONAS DE "NO TOCAR"

### 1. **Código de partículas** (`main.rs` líneas 1600-1700)
- **Razón**: Funciona correctamente, tests passing
- **Señal**: Comentarios con `particles::` están bien

### 2. **Input map SDL2** (`main.rs` líneas 2100-2200)
- **Razón**: Input funciona con SDL2, no tocar
- **Señal**: `input.es_presionada()` funciona

### 3. **Render queue** (`main.rs` líneas 1400-1500)
- **Razón**: Render queue integrada funciona
- **Señal**: `DrawCommand` está bien usado

### 4. **Módulos level.rs y entity.rs**
- **Razón**: Solo warnings menores, no errores críticos
- **Señal**: Compilan sin errors E0XXX

---

## ⏱️ ESTIMACIÓN DE TIEMPO REAL

### Escenario Optimista (sin imprevistos)
| Fase | Tiempo | Errores |
|------|--------|---------|
| FASE 1 (🟢 Bajo) | 2 horas | 40 errores |
| FASE 2 (🟡 Medio) | 3 horas | 24 errores |
| FASE 3 (🔴 Alto) | 4 horas | 12 errores |
| **TOTAL** | **9 horas** | **76 errores** |

### Escenario Realista (con debugging)
| Fase | Tiempo | Errores |
|------|--------|---------|
| FASE 1 (🟢 Bajo) | 3 horas | 40 errores |
| FASE 2 (🟡 Medio) | 5 horas | 24 errores |
| FASE 3 (🔴 Alto) | 8 horas | 12 errores |
| **TOTAL** | **16 horas** | **76 errores** |

### Escenario Pesimista (bugs inesperados)
| Fase | Tiempo | Errores |
|------|--------|---------|
| FASE 1 (🟢 Bajo) | 4 horas | 40 errores |
| FASE 2 (🟡 Medio) | 8 horas | 24 errores |
| FASE 3 (🔴 Alto) | 12 horas | 12 errores |
| **TOTAL** | **24 horas** | **76 errores** |

---

## ✅ CHECKLIST DE VERIFICACIÓN

### Después de FASE 1:
- [ ] 0 errores E0425
- [ ] 0 errores E0433
- [ ] 0 errores E0658
- [ ] < 30 errores E0308
- [ ] < 10 errores E0277

### Después de FASE 2:
- [ ] 0 errores E0599
- [ ] < 15 errores E0308
- [ ] Parser.parse() fixeado en todos lados
- [ ] HTTP functions fixeadas

### Después de FASE 3:
- [ ] 0 errores E0026
- [ ] 0 errores E0027
- [ ] 0 errores E0308
- [ ] 0 errores E0277
- [ ] **COMPILACIÓN EXITOSA** ✅

### Después de compilar:
- [ ] `cargo build -p rydit-rs --bin rydit-rs` exitoso
- [ ] Binario creado: `target/release/rydit-rs`
- [ ] Demo simple ejecuta sin crash
- [ ] Input SDL2 funciona
- [ ] Render funciona

---

## 📞 PROTOCOLO DE EMERGENCIA

### Si algo sale mal:

**1. Backup actual**:
```bash
tar -czf backup_pre_fix_v0.11.2_$(date +%Y%m%d_%H%M%S).tar.gz crates/rydit-rs/src/
```

**2. Revertir cambios**:
```bash
git checkout crates/rydit-rs/src/main.rs
git checkout crates/rydit-rs/src/eval/mod.rs
git checkout crates/rydit-rs/src/repl.rs
```

**3. Verificar estado**:
```bash
cargo build -p rydit-rs --bin rydit-rs 2>&1 | grep "error\[E"
# Debería mostrar 76 errores de nuevo
```

---

## 🎯 CONCLUSIÓN

**76 errores NO son tantos como parecen**:
- 40 errores (52%) son fixes mecánicos de 1 línea 🟢
- 24 errores (32%) requieren entender contexto 🟡
- 12 errores (16%) son críticos pero bien documentados 🔴

**Tiempo realista**: 16 horas (2 días de trabajo)

**Recomendación**:
1. Empezar por FASE 1 (fixes seguros, confianza alta)
2. Continuar con FASE 2 (ganar momentum)
3. Atacar FASE 3 con calma (entender bien el AST)

**NO USAR SCRIPTS AUTOMÁTICOS** - Fix manual con contexto es más seguro.

---

<div align="center">

**🛡️ RyDit v0.11.2 - INFORME FORENSE COMPLETO**

*76 errores | 3 fases | 16 horas estimadas*

**Próximo: FIX MANUAL SEGURO (SIN SCRIPTS)**

</div>
