# INFORME TÉCNICO: 28 ERRORES RESTANTES rydit-rs BIN

**Fecha**: 2026-04-02  
**Versión**: v0.11.2  
**Progreso**: 70 → 28 errores (60% completado)  
**Archivo**: `crates/rydit-rs/src/main.rs` + `crates/rydit-rs/src/eval/mod.rs`

---

## 📊 RESUMEN EJECUTIVO

| Métrica | Valor |
|---------|-------|
| **Errores totales** | 28 |
| **E0308 (mismatched types)** | 20 (71.4%) |
| **E0277 (trait bounds)** | 5 (17.9%) |
| **E0425 (undefined value)** | 1 (3.6%) |
| **E0599 (method not found)** | 1 (3.6%) |
| **Líneas afectadas** | ~4,662 líneas |
| **Funciones críticas** | 7 funciones principales |

---

## 🔍 ANÁLISIS DETALLADO POR ERROR

### **GRUPO 1: ERRORES E0425/E0599 (CRÍTICOS - 2 errores)**

#### ERROR #1 - E0425: Variable no encontrada
```
## ERROR #1
- Archivo: crates/rydit-rs/src/main.rs:3191
- Tipo: E0425 (cannot find value `func_name` in this scope)
- Código:
  ```rust
  // líneas 3189-3193
  Expr::Texto(s) => Valor::Texto(s.clone()),
  Expr::Var(name) => {
      if func_name == "__INPUT__" {  // ← func_name no existe aquí
          return executor.input("> ");
      }
  ```
- Función: `evaluar_expr_gfx()`
- Causa: `func_name` se define en `evaluar_stmt_gfx()` pero se usa en función anidada
- Fix: Pasar `func_name` como parámetro o remover lógica incorrecta
- Prioridad: 🔴 CRÍTICA (bloquea compilación)
- Impacto: 1 error directo
```

#### ERROR #2 - E0599: Método no encontrado
```
## ERROR #2
- Archivo: crates/rydit-rs/src/eval/mod.rs:1420
- Tipo: E0599 (no method named `into_string` found for enum `Result<T, E>`)
- Código:
  ```rust
  // líneas 1418-1422
  };
  return match ureq::get(&url).call().into_string() {  // ← .call() retorna Result
      Ok(response) => Valor::Texto(response),
      Err(e) => Valor::Error(e),
  ```
- Función: `evaluar_stmt()` (http::get)
- Causa: `.call()` retorna `Result<Response, Error>`, necesita unwrap antes de `.into_string()`
- Fix: `.call().and_then(|r| r.into_string())` o `.call().expect("HTTP").into_string()`
- Prioridad: 🔴 CRÍTICA (bloquea compilación)
- Impacto: 1 error directo
```

---

### **GRUPO 2: ERRORES E0277 (Trait Bounds - 5 errores)**

#### ERROR #3 - E0277: AsRef<Expr> no implementado para str
```
## ERROR #3
- Archivo: crates/rydit-rs/src/main.rs:251
- Tipo: E0277 (the trait bound `str: AsRef<rydit_parser::Expr<'_>>` is not satisfied)
- Código:
  ```rust
  // líneas 249-253
  Stmt::Call { callee, args } => {
      // Extraer nombre de función
      let func_name = if let Expr::Var(name) = callee.as_ref() {  // ← callee es &Expr, .as_ref() es incorrecto
          name.to_string()
  ```
- Función: `evaluar_stmt()`
- Causa: `callee` ya es `&Expr` (por pattern matching), `.as_ref()` intenta convertir `&Expr` a `AsRef<Expr>`
- Fix: `if let Expr::Var(name) = callee` (sin .as_ref())
- Prioridad: 🔴 ALTA (causa errores en cascada)
- Impacto: 4 errores relacionados (líneas 251, 1458, 4497, 4506)
```

#### ERROR #4, #11, #21, #23 - E0277 (Variaciones del mismo patrón)
```
## ERROR #4
- Archivo: crates/rydit-rs/src/main.rs:1458
- Tipo: E0277 (mismo patrón que ERROR #3)
- Función: `evaluar_stmt_migui()`
- Fix: Mismo fix que ERROR #3

## ERROR #11
- Archivo: crates/rydit-rs/src/main.rs:4497
- Tipo: E0277 (mismo patrón que ERROR #3)
- Función: `ejecutar_stmt_migui()`
- Fix: Mismo fix que ERROR #3

## ERROR #23
- Archivo: crates/rydit-rs/src/main.rs:4506
- Tipo: E0308 (consecuencia de ERROR #4)
- Código: `Expr::Var(func_name)` espera `&str`, recibe `String`
- Fix: `Expr::Var(&func_name)` después de fixear ERROR #4
```

#### ERROR #18, #25 - E0277: Borrow<&str> no implementado
```
## ERROR #18
- Archivo: crates/rydit-rs/src/main.rs:4047
- Tipo: E0277 (the trait bound `String: Borrow<&str>` is not satisfied)
- Código:
  ```rust
  // líneas 4045-4049
  };
  let func_data = funcs.get(&func_name).map(|(p, b)| (p.clone(), b.clone()));  // ← &String no puede borrow como &&str
  ```
- Función: `evaluar_call_migui()`
- Causa: `funcs` es `HashMap<String, ...>`, `func_name` es `String`, `&func_name` es `&String`, pero HashMap::get necesita `&str`
- Fix: `funcs.get(func_name.as_str())` o `funcs.get(&func_name[..])`
- Prioridad: 🟡 MEDIA
- Impacto: 1 error directo

## ERROR #25
- Archivo: crates/rydit-rs/src/main.rs:4527
- Tipo: E0277 (mismo patrón que ERROR #18)
- Función: `ejecutar_stmt_migui()`
- Fix: Mismo fix que ERROR #18
```

---

### **GRUPO 3: ERRORES E0308 (Type Mismatches - 20 errores)**

#### ERROR #5 - E0308: Option<bool> vs bool
```
## ERROR #5
- Archivo: crates/rydit-rs/src/main.rs:255
- Tipo: E0308 (mismatched types: expected `Option<bool>`, found `bool`)
- Código:
  ```rust
  // líneas 253-257
      } else {
          println!("[WARNING] Call requiere función válida");
          return (false, None);  // ← debería ser (Some(false), None)
      }
  ```
- Función: `evaluar_stmt()`
- Causa: Return type es `(Option<bool>, Option<Valor>)`, pero retorna `(bool, None)`
- Fix: `return (Some(false), None);`
- Prioridad: 🟡 MEDIA
- Impacto: 1 error directo
```

#### ERROR #6, #12, #22, #26 - E0308: &String vs &&str
```
## ERROR #6
- Archivo: crates/rydit-rs/src/main.rs:341
- Tipo: E0308 (expected `&String`, found `&&str`)
- Código:
  ```rust
  // líneas 339-343
  // DEUDA #2 FIX: Detectar import cíclico
  if importing_stack.contains(module) {  // ← module es &str, importing_stack es Vec<String>
      println!("[ERROR] Importe cíclico detectado: '{}'", module);
  ```
- Función: `evaluar_stmt()`
- Causa: `importing_stack: Vec<String>`, `.contains()` necesita `&String`, pero `module: &str`
- Fix: `importing_stack.contains(&module.to_string())` o cambiar tipo de stack a `Vec<&str>`
- Prioridad: 🔴 ALTA (causa errores en cascada)
- Impacto: 4 errores relacionados (líneas 341, 399, 406, 407)

## ERROR #12
- Archivo: crates/rydit-rs/src/main.rs:1785
- Tipo: E0308 (mismo patrón que ERROR #6)
- Función: `evaluar_stmt_gfx()`
- Fix: Mismo fix que ERROR #6

## ERROR #22
- Archivo: crates/rydit-rs/src/main.rs:4522
- Tipo: E0308 (mismo patrón que ERROR #6)
- Función: `ejecutar_stmt_migui()`
- Fix: Mismo fix que ERROR #6

## ERROR #26
- Archivo: crates/rydit-rs/src/main.rs:4552
- Tipo: E0308 (consecuencia del patrón ERROR #6)
- Función: `ejecutar_stmt_migui()`
- Fix: Mismo fix que ERROR #6
```

#### ERROR #7, #13, #27 - E0308: String vs &str (push)
```
## ERROR #7
- Archivo: crates/rydit-rs/src/main.rs:399
- Tipo: E0308 (expected `String`, found `&str`)
- Código:
  ```rust
  // líneas 397-401
  // Agregar al stack de imports en progreso
  importing_stack.push(module.clone());  // ← .clone() en &str da &str, no String
  ```
- Función: `evaluar_stmt()`
- Causa: `module: &str`, `.clone()` da `&str`, pero `Vec<String>::push()` necesita `String`
- Fix: `importing_stack.push(module.to_string());`
- Prioridad: 🔴 ALTA (consecuencia de ERROR #6)
- Impacto: 1 error directo

## ERROR #13
- Archivo: crates/rydit-rs/src/main.rs:1837
- Tipo: E0308 (mismo patrón que ERROR #7)
- Función: `evaluar_stmt_gfx()`
- Fix: Mismo fix que ERROR #7

## ERROR #27
- Archivo: crates/rydit-rs/src/main.rs:4552
- Tipo: E0308 (mismo patrón que ERROR #7)
- Función: `ejecutar_stmt_migui()`
- Fix: Mismo fix que ERROR #7
```

#### ERROR #8, #9, #14, #15, #28, #29 - E0308: Parser return type
```
## ERROR #8
- Archivo: crates/rydit-rs/src/main.rs:406
- Tipo: E0308 (expected `(Program<'_>, Vec<RyDitError>)`, found `Result<_, _>`)
- Código:
  ```rust
  // líneas 404-408
  let program = match parser.parse() {
      Ok(p) => p,  // ← parser.parse() retorna (Program, Vec<Error>), no Result
      Err(e) => {
  ```
- Función: `evaluar_stmt()`
- Causa: `Parser::parse()` cambió signature de `Result<Program, Error>` a `(Program, Vec<Error>)`
- Fix:
  ```rust
  let (program, errors) = parser.parse();
  if !errors.is_empty() {
      // Manejar errores
  }
  ```
- Prioridad: 🔴 ALTA (causa errores en cascada)
- Impacto: 6 errores relacionados (líneas 406, 407, 1844, 1845, 4558, 4559)

## ERROR #9
- Archivo: crates/rydit-rs/src/main.rs:407
- Tipo: E0308 (mismo patrón que ERROR #8)
- Función: `evaluar_stmt()`
- Fix: Mismo fix que ERROR #8

## ERROR #14
- Archivo: crates/rydit-rs/src/main.rs:1844
- Tipo: E0308 (mismo patrón que ERROR #8)
- Función: `evaluar_stmt_gfx()`
- Fix: Mismo fix que ERROR #8

## ERROR #15
- Archivo: crates/rydit-rs/src/main.rs:1845
- Tipo: E0308 (mismo patrón que ERROR #8)
- Función: `evaluar_stmt_gfx()`
- Fix: Mismo fix que ERROR #8

## ERROR #28
- Archivo: crates/rydit-rs/src/main.rs:4558
- Tipo: E0308 (mismo patrón que ERROR #8)
- Función: `ejecutar_stmt_migui()`
- Fix: Mismo fix que ERROR #8

## ERROR #29
- Archivo: crates/rydit-rs/src/main.rs:4559
- Tipo: E0308 (mismo patrón que ERROR #8)
- Función: `ejecutar_stmt_migui()`
- Fix: Mismo fix que ERROR #8
```

#### ERROR #10, #20 - E0308: Vec<String> vs Vec<&str>
```
## ERROR #10
- Archivo: crates/rydit-rs/src/main.rs:1455
- Tipo: E0308 (expected `Vec<String>`, found `Vec<&str>`)
- Código:
  ```rust
  // líneas 1453-1457
  Stmt::Function { name, params, body } => {
      funcs.insert(name.clone(), (params.clone(), body.clone()));  // ← params: Vec<&str>, se espera Vec<String>
  }
  ```
- Función: `evaluar_stmt_migui()`
- Causa: `params: Vec<&str>` (del AST), pero funcs espera `HashMap<String, (Vec<String>, ...)>`
- Fix: `params.iter().map(|s| s.to_string()).collect()`
- Prioridad: 🔴 ALTA (causa errores en cascada)
- Impacto: 2 errores relacionados (líneas 1455 nombre, 1455 params)

## ERROR #20
- Archivo: crates/rydit-rs/src/main.rs:4493
- Tipo: E0308 (mismo patrón que ERROR #10)
- Función: `ejecutar_stmt_migui()`
- Fix: Mismo fix que ERROR #10
```

#### ERROR #16 - E0308: Valor vs Option<bool>
```
## ERROR #16
- Archivo: crates/rydit-rs/src/main.rs:1461
- Tipo: E0308 (expected `Option<bool>`, found `Valor`)
- Código:
  ```rust
  // líneas 1459-1463
      } else {
          return Valor::Error("Call requiere función válida".to_string());  // ← Función retorna (Option<bool>, Option<Valor>)
      }
  ```
- Función: `evaluar_stmt_migui()`
- Causa: Return type de `evaluar_stmt_migui()` es `(Option<bool>, Option<Valor>)`, pero retorna `Valor`
- Fix: `return (None, Some(Valor::Error(...)));`
- Prioridad: 🟡 MEDIA
- Impacto: 1 error directo
```

#### ERROR #17, #19 - E0308: String vs &str (Texto)
```
## ERROR #17
- Archivo: crates/rydit-rs/src/main.rs:2192
- Tipo: E0308 (expected `String`, found `&str`)
- Código:
  ```rust
  // líneas 2190-2194
  match expr {
      Expr::Texto(s) => Valor::Texto(s.clone()),  // ← s: &str, Valor::Texto necesita String
      Expr::Var(name) => {
  ```
- Función: `evaluar_expr_gfx()`
- Causa: `Expr::Texto(&'a str)`, pero `Valor::Texto(String)`
- Fix: `Valor::Texto(s.to_string())`
- Prioridad: 🟢 BAJA
- Impacto: 1 error directo

## ERROR #19
- Archivo: crates/rydit-rs/src/main.rs:3189
- Tipo: E0308 (mismo patrón que ERROR #17)
- Función: `evaluar_expr_migui()`
- Fix: Mismo fix que ERROR #17
```

---

## 🎯 ROOT CAUSES IDENTIFICADOS

### **Root Cause #1: Parser API Change** (CAUSA 6 ERRORES)
```
Cambio: Parser::parse() : Result<Program, Error> → (Program, Vec<Error>)
Afecta: Líneas 406, 407, 1844, 1845, 4558, 4559
Fix: Cambiar pattern match de Result a tuple destructuring
```

### **Root Cause #2: importing_stack Type Mismatch** (CAUSA 8 ERRORES)
```
Problema: importing_stack: Vec<String>, module: &str
Afecta: Líneas 341, 399, 1785, 1837, 4522, 4552
Fix: Unificar tipos (o todo String o todo &str)
```

### **Root Cause #3: callee.as_ref() Incorrecto** (CAUSA 4 ERRORES)
```
Problema: callee ya es &Expr por pattern matching, .as_ref() es incorrecto
Afecta: Líneas 251, 1458, 4497, 4506
Fix: Remover .as_ref()
```

### **Root Cause #4: funcs HashMap Type Mismatch** (CAUSA 4 ERRORES)
```
Problema: funcs: HashMap<String, (Vec<String>, ...)>, params: Vec<&str>
Afecta: Líneas 1455, 4493
Fix: Convertir params a Vec<String>
```

### **Root Cause #5: func_name Scope** (CAUSA 1 ERROR)
```
Problema: func_name se usa fuera de scope en evaluar_expr_gfx()
Afecta: Línea 3191
Fix: Pasar como parámetro o remover lógica
```

### **Root Cause #6: ureq API** (CAUSA 1 ERROR)
```
Problema: .call() retorna Result, necesita unwrap antes de .into_string()
Afecta: Línea 1420
Fix: .call().and_then(|r| r.into_string())
```

---

## 📋 PLAN DE FIX PASO A PASO

### **FASE 1: CRÍTICOS (Día 1)** ⏱️ 2-3 horas

| # | Error | Archivo:Línea | Fix | Tiempo |
|---|-------|---------------|-----|--------|
| 1 | E0425 | main.rs:3191 | Pasar `func_name` como parámetro a `evaluar_expr_gfx()` | 30 min |
| 2 | E0599 | eval/mod.rs:1420 | `.call().and_then(|r| r.into_string())` | 15 min |
| 3 | E0277 | main.rs:251 | Remover `.as_ref()` de `callee` | 15 min |
| 8-9 | E0308 | main.rs:406,407 | Fix parser return type (tuple) | 30 min |

**Subtotal Fase 1**: 4 errores críticos, 1.5 horas

---

### **FASE 2: ALTA PRIORIDAD (Día 1-2)** ⏱️ 3-4 horas

| # | Error | Archivo:Línea | Fix | Tiempo |
|---|-------|---------------|-----|--------|
| 6,12,22 | E0308 | main.rs:341,1785,4522 | `importing_stack.contains(&module.to_string())` | 30 min |
| 7,13,27 | E0308 | main.rs:399,1837,4552 | `importing_stack.push(module.to_string())` | 30 min |
| 14-15 | E0308 | main.rs:1844,1845 | Fix parser return type (gfx) | 30 min |
| 28-29 | E0308 | main.rs:4558,4559 | Fix parser return type (migui) | 30 min |
| 4,11,21 | E0277 | main.rs:1458,4497 | Remover `.as_ref()` (migui) | 30 min |
| 23 | E0308 | main.rs:4506 | `Expr::Var(&func_name)` | 15 min |

**Subtotal Fase 2**: 12 errores, 2.5 horas

---

### **FASE 3: MEDIA PRIORIDAD (Día 2)** ⏱️ 2-3 horas

| # | Error | Archivo:Línea | Fix | Tiempo |
|---|-------|---------------|-----|--------|
| 5 | E0308 | main.rs:255 | `(Some(false), None)` | 15 min |
| 10,20 | E0308 | main.rs:1455,4493 | `params.iter().map(|s| s.to_string()).collect()` | 30 min |
| 16 | E0308 | main.rs:1461 | `(None, Some(Valor::Error(...)))` | 30 min |
| 18,25 | E0277 | main.rs:4047,4527 | `funcs.get(func_name.as_str())` | 30 min |

**Subtotal Fase 3**: 6 errores, 1.75 horas

---

### **FASE 4: BAJA PRIORIDAD (Día 2)** ⏱️ 1 hora

| # | Error | Archivo:Línea | Fix | Tiempo |
|---|-------|---------------|-----|--------|
| 17,19 | E0308 | main.rs:2192,3189 | `Valor::Texto(s.to_string())` | 30 min |

**Subtotal Fase 4**: 2 errores, 0.5 horas

---

## 📊 ESTIMACIÓN DE TIEMPO REALISTA

| Fase | Errores | Tiempo Estimado | Buffer (20%) | Total |
|------|---------|-----------------|--------------|-------|
| **Fase 1** | 4 | 1.5h | 0.3h | 1.8h |
| **Fase 2** | 12 | 2.5h | 0.5h | 3.0h |
| **Fase 3** | 6 | 1.75h | 0.35h | 2.1h |
| **Fase 4** | 2 | 0.5h | 0.1h | 0.6h |
| **TOTAL** | **24** | **6.25h** | **1.25h** | **7.5h** |

**Nota**: 28 errores reportados, pero 4 son duplicados exactos (mismos fixes aplican múltiples veces)

**Tiempo Total**: **7-8 horas de trabajo concentrado**

---

## ⚠️ RIESGOS IDENTIFICADOS

### **Riesgo Alto 🔴**
1. **Parser API Change**: Fix puede romper otros usos de `parser.parse()` en el código
   - Mitigación: Buscar todos los usos de `Parser::parse()` con grep
   - Backup: Revertir cambios si rompe tests existentes

2. **importing_stack Type**: Cambiar tipo puede afectar lógica de detección de imports cíclicos
   - Mitigación: Testear con imports anidados después del fix
   - Backup: Mantener tipo actual y solo fixear comparisons

### **Riesgo Medio 🟡**
3. **func_name Scope**: Pasar como parámetro puede requerir cambios en signature de funciones anidadas
   - Mitigación: Seguir chain de llamadas para identificar todos los cambios necesarios
   - Backup: Usar variable global o remover lógica de `__INPUT__`

4. **HashMap Borrow**: Cambiar de `&func_name` a `func_name.as_str()` puede no ser consistente
   - Mitigación: Aplicar mismo patrón en todos los usos de `funcs.get()`
   - Backup: Cambiar tipo de HashMap keys

### **Riesgo Bajo 🟢**
5. **String vs &str conversions**: `.to_string()` puede causar allocs adicionales
   - Mitigación: Perfil después del fix para identificar hotspots
   - Backup: Usar Cow<str> si performance es crítico

---

## 🎯 RECOMENDACIÓN ESTRATÉGICA

### **Estrategia: FIX POR GRUPOS (RECOMENDADA)**

**Racional**:
- ✅ Cada grupo es independiente
- ✅ Permite commits atómicos
- ✅ Fácil reversión si algo falla
- ✅ Tests intermedios después de cada grupo

**Plan de Commits**:
```
1. commit: "fix: E0425/E0599 críticos (func_name scope, ureq API)"
   - Archivos: main.rs, eval/mod.rs
   - Errores: 2
   - Push: ✅

2. commit: "fix: E0277 callee.as_ref() pattern (4 errores)"
   - Archivos: main.rs (3 funciones)
   - Errores: 4
   - Push: ✅

3. commit: "fix: E0308 parser return type tuple (6 errores)"
   - Archivos: main.rs (3 funciones)
   - Errores: 6
   - Push: ✅

4. commit: "fix: E0308 importing_stack type mismatch (6 errores)"
   - Archivos: main.rs (3 funciones)
   - Errores: 6
   - Push: ✅

5. commit: "fix: E0308/E0277 restantes (HashMap, params, returns)"
   - Archivos: main.rs, eval/mod.rs
   - Errores: 8
   - Push: ✅
```

**Total**: 5 commits, 5 pushes, puntos de reversión claros

---

### **Estrategia Alternativa: FIX DE UNA VEZ**

**Racional**:
- ✅ Más rápido (sin overhead de commits)
- ❌ Difícil debuggear si algo falla
- ❌ Sin puntos de reversión intermedios

**Plan**:
```
1. Aplicar todos los fixes de una vez
2. cargo build para verificar
3. 1 commit grande: "fix: 28 errores restantes rydit-rs bin"
4. Push único
```

**Solo recomendado si**:
- Tiempo es crítico (< 2 horas)
- Confianza alta en fixes
- Tests automáticos para verificar

---

## 📝 CONCLUSIÓN

**Estado Actual**: 60% completado (70 → 28 errores)  
**Tiempo Restante**: 7-8 horas  
**Complejidad**: Media (patrones repetidos, root causes identificados)  
**Riesgo**: Medio (5 riesgos identificados, todos mitigables)

**Recomendación Final**: 
1. **Estrategia por grupos** (5 commits)
2. **Comenzar con Fase 1** (críticos)
3. **Testear después de cada fase**
4. **Backup antes de Fase 2** (git tag)

**Próximo Paso**: Ejecutar Fase 1 inmediatamente.

---

<div align="center">

**🛡️ RyDit v0.11.2 - INFORME 28 ERRORES**

*60% completado | 7-8 horas restantes | 5 root causes*

**Próximo: Fase 1 (Críticos) - 1.8 horas**

</div>
