# 🔍 INFORME TÉCNICO: 13 ERRORES rydit-rs BINARIO

**Fecha**: 2026-04-02  
**Versión**: v0.11.2  
**Progreso**: 70 → 13 errores (83% completado)  
**Archivo principal**: `crates/rydit-rs/src/main.rs` (4659 líneas)

---

## 📊 RESUMEN EJECUTIVO

### ¿Hay líneas duplicadas?
**SÍ CONFIRMADO**: El archivo tiene código DUPLICADO por `sed` anterior.

**Patrones duplicados encontrados**:

| Patrón | Líneas | Veces que aparece | ¿Debería? |
|--------|--------|-------------------|-----------|
| `importing_stack.push(module.clone())` | 394, 1831, 4547 | 3 | ✅ Sí (3 funciones distintas) |
| `funcs.insert(name.clone(), ...)` | 1451, 4488 | 2 | ✅ Sí (gfx + migui) |
| `Expr::Texto(s) => Valor::Texto(s.clone())` | 2187, 3184 | 2 | ✅ Sí (gfx + migui) |
| `Stmt::Function { name, params, body }` | 241, 1450, 4487 | 3 | ⚠️ **POSIBLE DUPLICADO** |

**Conclusión**: NO hay duplicación accidental masiva. Cada función (`ejecutar_stmt`, `ejecutar_stmt_gfx`, `ejecutar_stmt_migui`) tiene su propia implementación. Los 13 errores son por **mismatch de tipos**, NO por duplicación.

---

## 🔴 LOS 13 ERRORES DETALLADOS

### **ERROR #1/13** - Línea 394
```
error[E0308]: mismatched types
   --> crates/rydit-rs/src/main.rs:394:34
    |
394 |             importing_stack.push(module.clone());
    |                             ---- ^^^^^^^^^^^^^^ expected `String`, found `&str`
```

**Contexto** (líneas 388-400):
```rust
// Función: ejecutar_stmt()
// Línea 394
importing_stack: &mut Vec<String>,  // ← El vector espera Strings
// ...
importing_stack.push(module.clone());  // ← module es &str
```

**Causa raíz**: `module` es `&str` pero `importing_stack` es `Vec<String>`

**Fix**:
```rust
// ANTES (línea 394)
importing_stack.push(module.clone());

// DESPUÉS
importing_stack.push(module.to_string());
```

---

### **ERROR #2/13** - Línea 1451
```
error[E0308]: mismatched types
    --> crates/rydit-rs/src/main.rs:1451:26
     |
1451 |             funcs.insert(name.clone(), ...);
     |                   ------ ^^^^^^^^^^^^ expected `String`, found `&str`
```

**Contexto** (líneas 1448-1455):
```rust
// Función: ejecutar_stmt_gfx()
// Línea 1451
Stmt::Function { name, params, body } => {
    funcs: HashMap<String, (Vec<String>, Vec<Stmt>)>
    name: &str  // ← del pattern match
    funcs.insert(name.clone(), (params.iter().map(|s| s.to_string()).collect(), body.clone()));
}
```

**Causa raíz**: `name` es `&str` pero `funcs` requiere `String` como key

**Fix**:
```rust
// ANTES (línea 1451)
funcs.insert(name.clone(), (params.iter().map(|s| s.to_string()).collect(), body.clone()));

// DESPUÉS
funcs.insert(name.to_string(), (params.iter().map(|s| s.to_string()).collect(), body.clone()));
```

---

### **ERROR #3/13** - Línea 1740
```
error[E0308]: mismatched types
    --> crates/rydit-rs/src/main.rs:1740:43
     |
1740 |                 let func_data = funcs.get(func_name).map(|(p, b)| (p.clone(), b.clone()));
     |                                       --- ^^^^^^^^^ expected `&_`, found `String`
```

**Contexto** (líneas 1735-1745):
```rust
// Función: ejecutar_stmt_gfx() - caso Stmt::Call
// Línea 1740
let func_name = callee.to_string();  // ← func_name es String
funcs: HashMap<String, ...>
funcs.get(func_name)  // ← .get() requiere referencia &String o &str
```

**Causa raíz**: `HashMap::get()` requiere referencia, no valor poseído

**Fix**:
```rust
// ANTES (línea 1740)
let func_data = funcs.get(func_name).map(|(p, b)| (p.clone(), b.clone()));

// DESPUÉS
let func_data = funcs.get(&func_name).map(|(p, b)| (p.clone(), b.clone()));
```

---

### **ERROR #4/13** - Línea 1831
```
error[E0308]: mismatched types
    --> crates/rydit-rs/src/main.rs:1831:38
     |
1831 |                 importing_stack.push(module.clone());
     |                                 ---- ^^^^^^^^^^^^^^ expected `String`, found `&str`
```

**Contexto** (líneas 1825-1835):
```rust
// Función: ejecutar_stmt_gfx() - caso Stmt::Import
// Línea 1831
importing_stack: &mut Vec<String>
module: &str  // ← del pattern match Stmt::Import { module, alias }
importing_stack.push(module.clone());
```

**Causa raíz**: Igual que ERROR #1 - `&str` vs `String`

**Fix**:
```rust
// ANTES (línea 1831)
importing_stack.push(module.clone());

// DESPUÉS
importing_stack.push(module.to_string());
```

---

### **ERROR #5/13** - Línea 2187
```
error[E0308]: mismatched types
    --> crates/rydit-rs/src/main.rs:2187:40
     |
2187 |         Expr::Texto(s) => Valor::Texto(s.clone()),
     |                           ------------ ^^^^^^^^^ expected `String`, found `&str`
```

**Contexto** (líneas 2183-2192):
```rust
// Función: evaluar_expr_gfx()
// Línea 2187
match expr {
    Expr::Texto(s) => {  // ← s es &str (AST nuevo usa &'a str)
        Valor::Texto(s.clone())  // ← Valor::Texto requiere String
    }
}

// blast-core/src/lib.rs:10
pub enum Valor {
    Texto(String),  // ← Requiere String poseído
    ...
}
```

**Causa raíz**: AST nuevo usa `&'a str` pero `Valor::Texto` requiere `String`

**Fix**:
```rust
// ANTES (línea 2187)
Expr::Texto(s) => Valor::Texto(s.clone()),

// DESPUÉS
Expr::Texto(s) => Valor::Texto(s.to_string()),
```

---

### **ERROR #6/13** - Línea 3184
```
error[E0308]: mismatched types
    --> crates/rydit-rs/src/main.rs:3184:40
     |
3184 |         Expr::Texto(s) => Valor::Texto(s.clone()),
     |                           ------------ ^^^^^^^^^ expected `String`, found `&str`
```

**Contexto** (líneas 3180-3188):
```rust
// Función: evaluar_expr_migui()
// Línea 3184
// MISMO PROBLEMA que ERROR #5 pero en otra función
match expr {
    Expr::Texto(s) => Valor::Texto(s.clone()),
}
```

**Causa raíz**: Idéntica a ERROR #5 - `&str` vs `String`

**Fix**:
```rust
// ANTES (línea 3184)
Expr::Texto(s) => Valor::Texto(s.clone()),

// DESPUÉS
Expr::Texto(s) => Valor::Texto(s.to_string()),
```

---

### **ERROR #7/13** - Línea 3186
```
error[E0277]: can't compare `&str` with `str`
    --> crates/rydit-rs/src/main.rs:3186:21
     |
3186 |             if name == "__INPUT__" {
     |                     ^^ no implementation for `&str == str`
```

**Contexto** (líneas 3184-3192):
```rust
// Función: evaluar_expr_migui()
// Línea 3186
Expr::Var(name) => {  // ← name es &&str (referencia doble por match)
    if name == "__INPUT__" {  // ← compara &&str con &str
        return executor.input("> ");
    }
}
```

**Causa raíz**: `name` es `&&str` (doble referencia) por el pattern match en `match expr`

**Análisis del tipo**:
```rust
// La firma de la función:
pub fn evaluar_expr_migui(
    expr: &Expr,  // ← referencia
    ...
) -> Valor {
    match expr {  // ← match sobre &Expr
        Expr::Var(name) => {  // ← name es &&str
            // porque Expr::Var(&'a str) y estamos matcheando &Expr
        }
    }
}
```

**Fix**:
```rust
// ANTES (línea 3186)
if name == "__INPUT__" {

// DESPUÉS (2 opciones)
// Opción 1: Dereferenciar
if *name == "__INPUT__" {

// Opción 2: Comparar con referencia
if name == &"__INPUT__" {

// Opción 3: Pattern match directo (MEJOR)
Expr::Var("__INPUT__") => return executor.input("> "),
Expr::Var(name) => {
    executor.leer(name).unwrap_or(Valor::Vacio)
}
```

**Recomendado**: Opción 3 (más idiomático)

---

### **ERROR #8/13** - Línea 4042
```
error[E0277]: the trait bound `std::string::String: Borrow<&str>` is not satisfied
    --> crates/rydit-rs/src/main.rs:4042:39
     |
4042 |             let func_data = funcs.get(&func_name).map(|(p, b)| (p.clone(), b.clone()));
     |                                   --- ^^^^^^^^^^ the trait `Borrow<&str>` is not implemented for `std::string::String`
```

**Contexto** (líneas 4028-4045):
```rust
// Función: evaluar_expr_migui() - caso Stmt::Call
// Línea 4042
let func_name = if func_name.contains("::") {
    if funcs.contains_key(func_name) {
        func_name.clone()  // ← String
    } else {
        &func_name.split("::").last().unwrap_or(func_name).to_string()  // ← &String
    }
} else {
    func_name.clone()  // ← String
};

// func_name es String
funcs.get(&func_name)  // ← &func_name es &String, pero HashMap espera &str
```

**Causa raíz**: Confusión de tipos en el if-else. Una rama devuelve `String`, otra `&String`

**Análisis**:
```rust
// Rama 1: func_name.clone() → String
// Rama 2: &func_name.split(...).to_string() → &String (referencia a temporal!)
// El tipo inferido es String pero la referencia &func_name es &String
```

**Fix**:
```rust
// ANTES (líneas 4028-4042)
let func_name = if func_name.contains("::") {
    if funcs.contains_key(func_name) {
        func_name.clone()
    } else {
        &func_name.split("::").last().unwrap_or(func_name).to_string()
    }
} else {
    func_name.clone()
};

let func_data = funcs.get(&func_name).map(|(p, b)| (p.clone(), b.clone()));

// DESPUÉS (simplificar)
let func_name = if func_name.contains("::") {
    func_name.split("::").last().unwrap_or(&func_name).to_string()
} else {
    func_name.clone()
};

let func_data = funcs.get(func_name.as_str()).map(|(p, b)| (p.clone(), b.clone()));
```

---

### **ERROR #9/13** - Línea 4488
```
error[E0308]: mismatched types
    --> crates/rydit-rs/src/main.rs:4488:26
     |
4488 |             funcs.insert(name.clone(), ...);
     |                   ------ ^^^^^^^^^^^^ expected `String`, found `&str`
```

**Contexto** (líneas 4487-4490):
```rust
// Función: ejecutar_stmt_migui()
// Línea 4488
// IDÉNTICO al ERROR #2
Stmt::Function { name, params, body } => {
    funcs.insert(name.clone(), (params.iter().map(|s| s.to_string()).collect(), body.clone()));
}
```

**Causa raíz**: `name` es `&str`, `funcs` requiere `String`

**Fix**:
```rust
// ANTES (línea 4488)
funcs.insert(name.clone(), (params.iter().map(|s| s.to_string()).collect(), body.clone()));

// DESPUÉS
funcs.insert(name.to_string(), (params.iter().map(|s| s.to_string()).collect(), body.clone()));
```

---

### **ERROR #10/13** - Línea 4492
```
error[E0277]: the trait bound `str: AsRef<rydit_parser::Expr<'_>>` is not satisfied
    --> crates/rydit-rs/src/main.rs:4492:61
     |
4492 |             let func_name = if let Expr::Var(name) = callee.as_ref() {
     |                                                             ^^^^^^
```

**Contexto** (líneas 4490-4500):
```rust
// Función: ejecutar_stmt_migui() - caso Stmt::Call
// Línea 4492
Stmt::Call { callee, args } => {
    callee: &Expr  // ← del pattern match
    let func_name = if let Expr::Var(name) = callee.as_ref() {
        // callee.as_ref() convierte &Expr a &Expr (no-op)
        // El problema: callee es &Expr, no Box<Expr>
    }
}
```

**Causa raíz**: `callee` es `&Expr`, no `Box<Expr>`. `.as_ref()` no es necesario.

**Fix**:
```rust
// ANTES (línea 4492)
let func_name = if let Expr::Var(name) = callee.as_ref() {
    name.to_string()
} else {
    String::new()
};

// DESPUÉS
let func_name = if let Expr::Var(name) = callee {
    name.to_string()
} else {
    String::new()
};
```

---

### **ERROR #11/13** - Línea 4501
```
error[E0308]: mismatched types
    --> crates/rydit-rs/src/main.rs:4501:48
     |
4501 |                     callee: Box::new(Expr::Var(func_name)),
     |                                      --------- ^^^^^^^^^ expected `&str`, found `String`
```

**Contexto** (líneas 4490-4505):
```rust
// Función: ejecutar_stmt_migui() - caso Stmt::Call
// Línea 4501
let func_name = ...;  // ← String

evaluar_expr_migui(
    &Expr::Call {
        callee: Box::new(Expr::Var(func_name)),  // ← Expr::Var requiere &str
        args: args.clone(),
    },
    ...
)

// rydit-parser/src/ast.rs:48
pub enum Expr<'a> {
    Var(&'a str),  // ← Requiere &str, no String
    ...
}
```

**Causa raíz**: `Expr::Var` usa lifetime `'a` con `&str`, no `String`

**Fix**:
```rust
// ANTES (línea 4501)
callee: Box::new(Expr::Var(func_name)),

// DESPUÉS
callee: Box::new(Expr::Var(&func_name)),
```

---

### **ERROR #12/13** - Línea 4522
```
error[E0277]: the trait bound `std::string::String: Borrow<&str>` is not satisfied
    --> crates/rydit-rs/src/main.rs:4522:39
     |
4522 |             if loaded_modules.contains(module) {
     |                               -------- ^^^^^^
```

**Contexto** (líneas 4515-4525):
```rust
// Función: ejecutar_stmt_migui() - caso Stmt::Import
// Línea 4522
Stmt::Import { module, alias } => {
    module: &str  // ← del pattern match
    loaded_modules: HashSet<String>
    loaded_modules.contains(module)  // ← module es &str, HashSet espera &String
}
```

**Causa raíz**: `HashSet::contains()` con `HashSet<String>` requiere `&str` (gracias a Deref), pero el compilador no puede inferir correctamente

**Fix**:
```rust
// ANTES (línea 4522)
if loaded_modules.contains(module) {

// DESPUÉS (2 opciones)
// Opción 1: Convertir a String
if loaded_modules.contains(&module.to_string()) {

// Opción 2: Usar as_str() explícito (MEJOR)
if loaded_modules.contains(module) {  // ← Esto DEBERÍA funcionar
// El problema real es que module es &str y HashSet<String>
// La solución es cambiar el tipo de loaded_modules
```

**Solución REAL**: El problema es el tipo de `loaded_modules`. Debería ser `HashSet<Arc<str>>` o aceptar ambos tipos.

**Fix recomendado**:
```rust
// ANTES
loaded_modules: &mut HashSet<String>

// DESPUÉS (cambiar firma de función)
loaded_modules: &mut HashSet<String>
// ...
if loaded_modules.contains(module.as_ref()) {  // ← module.as_ref() convierte &str a &str
```

**Fix práctico**:
```rust
// ANTES (línea 4522)
if loaded_modules.contains(module) {

// DESPUÉS
if loaded_modules.contains(&module.to_string()) {
```

---

### **ERROR #13/13** - Línea 4547
```
error[E0308]: mismatched types
    --> crates/rydit-rs/src/main.rs:4547:38
     |
4547 |                 importing_stack.push(module.clone());
     |                                 ---- ^^^^^^^^^^^^^^ expected `String`, found `&str`
```

**Contexto** (líneas 4545-4550):
```rust
// Función: ejecutar_stmt_migui() - caso Stmt::Import
// Línea 4547
// IDÉNTICO a ERROR #1 y #4
importing_stack: &mut Vec<String>
module: &str
importing_stack.push(module.clone());
```

**Causa raíz**: `&str` vs `String` (mismo patrón que errores #1 y #4)

**Fix**:
```rust
// ANTES (línea 4547)
importing_stack.push(module.clone());

// DESPUÉS
importing_stack.push(module.to_string());
```

---

## 📋 PLAN DE FIX PASO A PASO

### **FASE 1: Errores &str → String** (5 errores, 10 minutos)

**Errores**: #1, #2, #4, #5, #6, #9, #13

**Patrón**: Todos son `.clone()` en `&str` cuando se necesita `String`

**Comando sed**:
```bash
# ERROR #1 (línea 394)
sed -i '394s/module\.clone()/module.to_string()/' crates/rydit-rs/src/main.rs

# ERROR #2 (línea 1451)
sed -i '1451s/name\.clone()/name.to_string()/' crates/rydit-rs/src/main.rs

# ERROR #4 (línea 1831)
sed -i '1831s/module\.clone()/module.to_string()/' crates/rydit-rs/src/main.rs

# ERROR #5 (línea 2187)
sed -i '2187s/s\.clone()/s.to_string()/' crates/rydit-rs/src/main.rs

# ERROR #6 (línea 3184)
sed -i '3184s/s\.clone()/s.to_string()/' crates/rydit-rs/src/main.rs

# ERROR #9 (línea 4488)
sed -i '4488s/name\.clone()/name.to_string()/' crates/rydit-rs/src/main.rs

# ERROR #13 (línea 4547)
sed -i '4547s/module\.clone()/module.to_string()/' crates/rydit-rs/src/main.rs
```

---

### **FASE 2: Errores de Referencias** (3 errores, 5 minutos)

**Errores**: #3, #8, #12

**Patrón**: `.get()` requiere referencia, no valor

**Comando sed**:
```bash
# ERROR #3 (línea 1740)
sed -i '1740s/funcs\.get(func_name)/funcs.get(\&func_name)/' crates/rydit-rs/src/main.rs

# ERROR #12 (línea 4522) - REQUIRES MANUAL FIX
# Ver sección de fixes manuales abajo
```

---

### **FASE 3: Errores de Lógica** (5 errores, 15 minutos)

**Errores**: #7, #8, #10, #11

**Requieren fix manual** (cambios complejos):

#### **ERROR #7** (línea 3186) - Comparación &&str
```rust
// ANTES (líneas 3186-3190)
Expr::Var(name) => {
    if name == "__INPUT__" {
        return executor.input("> ");
    }
    executor.leer(name).unwrap_or(Valor::Vacio)
}

// DESPUÉS
Expr::Var("__INPUT__") => return executor.input("> "),
Expr::Var(name) => {
    executor.leer(name).unwrap_or(Valor::Vacio)
}
```

#### **ERROR #8** (líneas 4028-4042) - Lógica confusa
```rust
// ANTES (líneas 4028-4042)
let func_name = if func_name.contains("::") {
    if funcs.contains_key(func_name) {
        func_name.clone()
    } else {
        &func_name.split("::").last().unwrap_or(func_name).to_string()
    }
} else {
    func_name.clone()
};

let func_data = funcs.get(&func_name).map(|(p, b)| (p.clone(), b.clone()));

// DESPUÉS
let func_name = if func_name.contains("::") {
    func_name.split("::").last().unwrap_or(&func_name).to_string()
} else {
    func_name.clone()
};

let func_data = funcs.get(func_name.as_str()).map(|(p, b)| (p.clone(), b.clone()));
```

#### **ERROR #10-11** (líneas 4492-4501) - callee.as_ref()
```rust
// ANTES (líneas 4490-4501)
Stmt::Call { callee, args } => {
    let func_name = if let Expr::Var(name) = callee.as_ref() {
        name.to_string()
    } else {
        String::new()
    };

    let _ = evaluar_expr_migui(
        &Expr::Call {
            callee: Box::new(Expr::Var(func_name)),
            args: args.clone(),
        },
        ...
    );
}

// DESPUÉS
Stmt::Call { callee, args } => {
    let func_name = if let Expr::Var(name) = callee {
        name.to_string()
    } else {
        String::new()
    };

    let _ = evaluar_expr_migui(
        &Expr::Call {
            callee: Box::new(Expr::Var(&func_name)),
            args: args.clone(),
        },
        ...
    );
}
```

---

## 🧪 TEST DEBUG SIMPLIFICADO

Voy a crear un binario de test que compile rápido y muestre los tipos exactos:

```rust
// crates/rydit-rs/src/bin/debug_13_errors.rs
// Compilar: cargo build -p rydit-rs --bin debug_13_errors
// Este binario NO existe aún - es para diagnóstico

fn main() {
    println!("=== DEBUG 13 ERRORES ===");
    
    // ERROR #1, #4, #13: importing_stack.push(module.clone())
    let mut stack: Vec<String> = Vec::new();
    let module: &str = "test_module";
    // stack.push(module.clone());  // ❌ ERROR
    stack.push(module.to_string());  // ✅ FIX
    println!("✅ ERROR #1, #4, #13: module.to_string()");
    
    // ERROR #2, #9: funcs.insert(name.clone(), ...)
    use std::collections::HashMap;
    let mut funcs: HashMap<String, (Vec<String>, Vec<i32>)> = HashMap::new();
    let name: &str = "test_func";
    // funcs.insert(name.clone(), (vec![], vec![]));  // ❌ ERROR
    funcs.insert(name.to_string(), (vec![], vec![]));  // ✅ FIX
    println!("✅ ERROR #2, #9: name.to_string()");
    
    // ERROR #3: funcs.get(func_name)
    let func_name: String = "test_func".to_string();
    // let _ = funcs.get(func_name);  // ❌ ERROR
    let _ = funcs.get(&func_name);  // ✅ FIX
    println!("✅ ERROR #3: funcs.get(&func_name)");
    
    // ERROR #5, #6: Valor::Texto(s.clone())
    enum Valor { Texto(String) }
    let s: &str = "hello";
    // let _ = Valor::Texto(s.clone());  // ❌ ERROR
    let _ = Valor::Texto(s.to_string());  // ✅ FIX
    println!("✅ ERROR #5, #6: s.to_string()");
    
    // ERROR #7: if name == "__INPUT__"
    let name: &&str = &"__INPUT__";
    // if name == "__INPUT__" {}  // ❌ ERROR
    if *name == "__INPUT__" {}  // ✅ FIX
    println!("✅ ERROR #7: *name deref");
    
    // ERROR #8: funcs.get(&func_name) con String
    let func_name: String = "test".to_string();
    let _ = funcs.get(func_name.as_str());  // ✅ FIX
    println!("✅ ERROR #8: func_name.as_str()");
    
    // ERROR #10: callee.as_ref()
    enum Expr<'a> { Var(&'a str) }
    let callee: &Expr = &Expr::Var("test");
    // if let Expr::Var(name) = callee.as_ref() {}  // ❌ ERROR
    if let Expr::Var(name) = callee {  // ✅ FIX
        println!("✅ ERROR #10: callee sin .as_ref(), name = {}", name);
    }
    
    // ERROR #11: Expr::Var(func_name)
    let func_name: String = "test".to_string();
    // let _ = Expr::Var(func_name);  // ❌ ERROR
    let _ = Expr::Var(&func_name);  // ✅ FIX
    println!("✅ ERROR #11: Expr::Var(&func_name)");
    
    // ERROR #12: loaded_modules.contains(module)
    use std::collections::HashSet;
    let mut loaded: HashSet<String> = HashSet::new();
    let module: &str = "test";
    // loaded.contains(module);  // ❌ ERROR (a veces)
    loaded.contains(&module.to_string());  // ✅ FIX
    println!("✅ ERROR #12: loaded.contains(&module.to_string())");
    
    println!("\n=== TODOS LOS TIPOS VERIFICADOS ===");
}
```

---

## 🎯 ORDEN DE APLICACIÓN RECOMENDADO

### **Paso 1: Fixes automáticos con sed** (5 minutos)
```bash
cd /data/data/com.termux/files/home/shield-project

# 7 errores de .clone() → .to_string()
sed -i '394s/module\.clone()/module.to_string()/' crates/rydit-rs/src/main.rs
sed -i '1451s/name\.clone()/name.to_string()/' crates/rydit-rs/src/main.rs
sed -i '1831s/module\.clone()/module.to_string()/' crates/rydit-rs/src/main.rs
sed -i '2187s/s\.clone()/s.to_string()/' crates/rydit-rs/src/main.rs
sed -i '3184s/s\.clone()/s.to_string()/' crates/rydit-rs/src/main.rs
sed -i '4488s/name\.clone()/name.to_string()/' crates/rydit-rs/src/main.rs
sed -i '4547s/module\.clone()/module.to_string()/' crates/rydit-rs/src/main.rs

# 1 error de referencia
sed -i '1740s/funcs\.get(func_name)/funcs.get(\&func_name)/' crates/rydit-rs/src/main.rs

# Compilar para verificar
cargo build -p rydit-rs --bin rydit-rs 2>&1 | tee /tmp/fix1.log
```

**Resultado esperado**: 13 → 5 errores

---

### **Paso 2: Fixes manuales con edit** (15 minutos)

**Archivo**: `crates/rydit-rs/src/main.rs`

#### **Fix ERROR #7** (línea 3186-3190):
```rust
// Buscar:
Expr::Var(name) => {
    if name == "__INPUT__" {
        return executor.input("> ");
    }
    executor.leer(name).unwrap_or(Valor::Vacio)
}

// Reemplazar con:
Expr::Var("__INPUT__") => return executor.input("> "),
Expr::Var(name) => {
    executor.leer(name).unwrap_or(Valor::Vacio)
}
```

#### **Fix ERROR #8** (líneas 4028-4042):
```rust
// Buscar el bloque completo (líneas 4028-4042)
let func_name = if func_name.contains("::") {
    if funcs.contains_key(func_name) {
        func_name.clone()
    } else {
        &func_name.split("::").last().unwrap_or(func_name).to_string()
    }
} else {
    func_name.clone()
};

let func_data = funcs.get(&func_name).map(|(p, b)| (p.clone(), b.clone()));

// Reemplazar con:
let func_name = if func_name.contains("::") {
    func_name.split("::").last().unwrap_or(&func_name).to_string()
} else {
    func_name.clone()
};

let func_data = funcs.get(func_name.as_str()).map(|(p, b)| (p.clone(), b.clone()));
```

#### **Fix ERROR #10-11** (líneas 4490-4501):
```rust
// Buscar:
let func_name = if let Expr::Var(name) = callee.as_ref() {
    name.to_string()
} else {
    String::new()
};

// Reemplazar con:
let func_name = if let Expr::Var(name) = callee {
    name.to_string()
} else {
    String::new()
};
```

```rust
// Buscar más abajo (línea 4501):
callee: Box::new(Expr::Var(func_name)),

// Reemplazar con:
callee: Box::new(Expr::Var(&func_name)),
```

#### **Fix ERROR #12** (línea 4522):
```rust
// Buscar:
if loaded_modules.contains(module) {

// Reemplazar con:
if loaded_modules.contains(&module.to_string()) {
```

---

### **Paso 3: Verificación final** (2 minutos)
```bash
cargo build -p rydit-rs --bin rydit-rs 2>&1 | tee /tmp/fix_final.log
```

**Resultado esperado**: **0 errores** ✅

---

## 📊 CONCLUSIONES

### ¿Hay líneas duplicadas?
- **NO hay duplicación accidental masiva**
- Cada función (`ejecutar_stmt`, `ejecutar_stmt_gfx`, `ejecutar_stmt_migui`) tiene su propia implementación legítima
- Los patrones repetidos son **intencionales** (una copia por función)

### Causa raíz de los 13 errores:
1. **7 errores**: `&str` vs `String` (`.clone()` en `&str` no crea `String`)
2. **3 errores**: Referencias incorrectas en `.get()`
3. **2 errores**: Lógica compleja con lifetimes y tipos

### Complejidad:
- **Fáciles** (sed): 8 errores (62%)
- **Medios** (edit simple): 3 errores (23%)
- **Complejos** (edit cuidadoso): 2 errores (15%)

### Tiempo estimado:
- **Paso 1 (sed)**: 5 minutos
- **Paso 2 (manual)**: 15 minutos
- **Paso 3 (verificación)**: 2 minutos
- **Total**: ~22 minutos

---

<div align="center">

**🛡️ 13 ERRORES IDENTIFICADOS - LISTOS PARA FIX**

*83% progreso (70→13) | 22 minutos para 0 errores*

**Próximo: Aplicar fixes en orden**

</div>
