// 🧪 TEST DEBUG SIMPLIFICADO - 13 ERRORES
// Propósito: Verificar tipos exactos de los 13 errores
// Compilar: cargo build -p rydit-rs --bin debug_13_errors
// Ejecutar: ./target/debug/debug_13_errors

use std::collections::{HashMap, HashSet};

// Simular tipos del proyecto
#[derive(Clone, Debug)]
#[allow(dead_code)]
enum Valor {
    Texto(String),
    Num(f64),
    Bool(bool),
    Vacio,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Expr<'a> {
    Num(f64),
    Texto(&'a str),
    Var(&'a str),
    Bool(bool),
}

fn main() {
    println!("=== 🔍 DEBUG 13 ERRORES rydit-rs ===\n");
    
    // =========================================================================
    // ERROR #1, #4, #13: importing_stack.push(module.clone())
    // =========================================================================
    println!("ERROR #1, #4, #13: &str → String conversion");
    println!("----------------------------------------------");
    let mut importing_stack: Vec<String> = Vec::new();
    let module: &str = "test_module";
    
    // ❌ ERROR E0308: expected `String`, found `&str`
    // importing_stack.push(module.clone());
    
    // ✅ FIX: Usar .to_string() para convertir &str → String
    importing_stack.push(module.to_string());
    println!("  module = '{}'", module);
    println!("  module.to_string() = {:?} (tipo: String)", module.to_string());
    println!("  ✅ importing_stack.push(module.to_string())\n");
    
    // =========================================================================
    // ERROR #2, #9: funcs.insert(name.clone(), ...)
    // =========================================================================
    println!("ERROR #2, #9: HashMap key &str → String");
    println!("----------------------------------------------");
    let mut funcs: HashMap<String, (Vec<String>, Vec<i32>)> = HashMap::new();
    let name: &str = "test_func";
    
    // ❌ ERROR E0308: expected `String`, found `&str`
    // funcs.insert(name.clone(), (vec![], vec![]));
    
    // ✅ FIX: name.to_string() para la key
    funcs.insert(name.to_string(), (vec![], vec![]));
    println!("  name = '{}'", name);
    println!("  funcs.insert(name.to_string(), ...)");
    println!("  ✅ HashMap key creada correctamente\n");
    
    // =========================================================================
    // ERROR #3: funcs.get(func_name)
    // =========================================================================
    println!("ERROR #3: HashMap::get() requiere referencia");
    println!("----------------------------------------------");
    let func_name: String = "test_func".to_string();
    
    // ❌ ERROR E0308: expected `&_`, found `String`
    // let _ = funcs.get(func_name);
    
    // ✅ FIX: Pasar referencia &func_name
    let _ = funcs.get(&func_name);
    println!("  func_name = {:?} (tipo: String)", func_name);
    println!("  funcs.get(&func_name) ✅");
    println!("  HashMap::get() siempre requiere referencia\n");
    
    // =========================================================================
    // ERROR #5, #6: Valor::Texto(s.clone())
    // =========================================================================
    println!("ERROR #5, #6: Enum variant &str → String");
    println!("----------------------------------------------");
    let s: &str = "hello";
    
    // ❌ ERROR E0308: expected `String`, found `&str`
    // let _ = Valor::Texto(s.clone());
    
    // ✅ FIX: s.to_string()
    let valor = Valor::Texto(s.to_string());
    println!("  s = '{}'", s);
    println!("  Valor::Texto(s.to_string()) = {:?}", valor);
    println!("  ✅ Variantes de enum requieren ownership\n");
    
    // =========================================================================
    // ERROR #7: if name == "__INPUT__"
    // =========================================================================
    println!("ERROR #7: Doble referencia &&str");
    println!("----------------------------------------------");
    let expr = Expr::Var("__INPUT__");
    
    // Simular el match que causa doble referencia
    match &expr {
        Expr::Var(name) => {
            // name es &&str aquí
            println!("  name tipo: &&str (doble referencia)");
            
            // ❌ ERROR E0277: can't compare `&str` with `str`
            // if name == "__INPUT__" {}
            
            // ✅ FIX 1: Dereferenciar
            if *name == "__INPUT__" {
                println!("  ✅ *name == \"__INPUT__\" (dereferencia)");
            }
            
            // ✅ FIX 2: Pattern match directo (mejor)
        }
        _ => {}
    }
    
    // Mejor solución: pattern match directo
    match &expr {
        Expr::Var("__INPUT__") => {
            println!("  ✅ Pattern match directo: Expr::Var(\"__INPUT__\")");
        }
        _ => {}
    }
    println!();
    
    // =========================================================================
    // ERROR #8: funcs.get(&func_name) con String
    // =========================================================================
    println!("ERROR #8: String vs &str en HashMap::get()");
    println!("----------------------------------------------");
    let func_name: String = "mod::func".to_string();
    
    // Lógica compleja que causa confusión de tipos
    let func_name_normalized = if func_name.contains("::") {
        func_name.split("::").last().unwrap_or(&func_name).to_string()
    } else {
        func_name.clone()
    };
    
    // ❌ ERROR E0277: Borrow<&str> not implemented for String
    // let _ = funcs.get(&func_name_normalized);
    
    // ✅ FIX: Usar .as_str() para obtener &str de String
    let _ = funcs.get(func_name_normalized.as_str());
    println!("  func_name_normalized.as_str() ✅");
    println!("  HashMap::get() acepta &str gracias a Deref\n");
    
    // =========================================================================
    // ERROR #10: callee.as_ref()
    // =========================================================================
    println!("ERROR #10: callee.as_ref() innecesario");
    println!("----------------------------------------------");
    let callee: &Expr = &Expr::Var("my_func");
    
    // ❌ ERROR E0277: str: AsRef<Expr> not satisfied
    // if let Expr::Var(name) = callee.as_ref() {}
    
    // ✅ FIX: callee ya es &Expr, no necesita .as_ref()
    if let Expr::Var(name) = callee {
        println!("  callee = Expr::Var({})", name);
        println!("  ✅ if let Expr::Var(name) = callee (sin .as_ref())");
    }
    println!();
    
    // =========================================================================
    // ERROR #11: Expr::Var(func_name)
    // =========================================================================
    println!("ERROR #11: Expr::Var con String vs &str");
    println!("----------------------------------------------");
    let func_name: String = "my_func".to_string();
    
    // Expr::Var(&'a str) requiere &str, no String
    // ❌ ERROR E0308: expected `&str`, found `String`
    // let _ = Expr::Var(func_name);
    
    // ✅ FIX: Pasar referencia &func_name
    let expr_call = Expr::Var(&func_name);
    println!("  func_name = {:?} (String)", func_name);
    println!("  Expr::Var(&func_name) = {:?}", expr_call);
    println!("  ✅ Lifetime &'a str permite referencias\n");
    
    // =========================================================================
    // ERROR #12: loaded_modules.contains(module)
    // =========================================================================
    println!("ERROR #12: HashSet<String>::contains(&str)");
    println!("----------------------------------------------");
    let mut loaded_modules: HashSet<String> = HashSet::new();
    let module: &str = "test_module";
    
    loaded_modules.insert("test_module".to_string());
    
    // HashSet<String>::contains() acepta &str gracias a Deref
    // PERO el compilador a veces necesita ayuda
    // ❌ ERROR E0277: Borrow<&str> not implemented for String
    
    // ✅ FIX: Convertir module a String para contains()
    if loaded_modules.contains(&module.to_string()) {
        println!("  ✅ loaded_modules.contains(&module.to_string())");
    }

    // También funciona con &str directo
    let module_str: &str = module;
    if loaded_modules.contains(module_str) {
        println!("  ✅ loaded_modules.contains(module_str)");
    }
    println!();
    
    // =========================================================================
    // RESUMEN
    // =========================================================================
    println!("=== 📊 RESUMEN DE FIXES ===\n");
    println!("  ERROR #1, #4, #13: module.to_string()");
    println!("  ERROR #2, #9: name.to_string()");
    println!("  ERROR #3: funcs.get(&func_name)");
    println!("  ERROR #5, #6: s.to_string()");
    println!("  ERROR #7: *name O pattern match directo");
    println!("  ERROR #8: func_name.as_str()");
    println!("  ERROR #10: callee (sin .as_ref())");
    println!("  ERROR #11: Expr::Var(&func_name)");
    println!("  ERROR #12: loaded_modules.contains(&module.to_string())");
    println!("\n=== ✅ TODOS LOS TIPOS VERIFICADOS ===");
}
