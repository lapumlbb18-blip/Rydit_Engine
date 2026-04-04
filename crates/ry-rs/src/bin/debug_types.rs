// crates/rydit-rs/src/bin/debug_types.rs
// 🐛 Debug log para identificar tipos exactos en errores

fn main() {
    println!("🔍 DEBUG TYPE LOG - RyDit v0.11.4");
    println!("==================================\n");

    // Test 1: parser.parse() return type
    println!("TEST 1: parser.parse() return type");
    test_parser_parse();

    // Test 2: importing_stack type
    println!("\nTEST 2: importing_stack type");
    test_importing_stack();

    // Test 3: callee type
    println!("\nTEST 3: Stmt::Call callee type");
    test_callee_type();

    // Test 4: HashMap borrow
    println!("\nTEST 4: HashMap borrow type");
    test_hashmap_borrow();

    println!("\n✅ DEBUG LOG COMPLETADO");
}

fn test_parser_parse() {
    use ry_parser::Parser;
    
    let source = "shield.init";
    let mut parser = Parser::from_source(source);
    let (program, errors) = parser.parse();
    
    println!("  parser.parse() retorna: (Program, Vec<RyDitError>)");
    println!("  - program.statements.len() = {}", program.statements.len());
    println!("  - errors.len() = {}", errors.len());
    println!("  ✅ Tipo correcto: (Program<'a>, Vec<RyDitError>)");
}

fn test_importing_stack() {
    let importing_stack: Vec<String> = vec!["module1".to_string()];
    let module = "module1";
    
    // Test: contains requiere &String, NO &str
    let result1 = importing_stack.contains(&module.to_string());
    // let result2 = importing_stack.contains(module); // ❌ ERROR: expected &String
    
    println!("  importing_stack: Vec<String>");
    println!("  module: &str");
    println!("  - contains(&module.to_string()) = {}", result1);
    println!("  ❌ contains(module) NO funciona (expected &String, found &str)");
    println!("  ✅ SOLUCIÓN: importing_stack: Vec<&str> O module.to_string()");
}

fn test_callee_type() {
    use ry_parser::{Stmt, Expr};
    
    // Simular Stmt::Call con callee: &str
    let callee: &str = "mi_funcion";
    let args: Vec<Expr> = vec![];
    
    let call = Stmt::Call { callee, args };
    
    match call {
        Stmt::Call { callee, .. } => {
            println!("  Stmt::Call {{ callee, args }}");
            println!("  - callee type: &str");
            println!("  - callee value: {}", callee);
            println!("  ✅ callee.to_string() = {}", callee.to_string());
        }
        _ => {}
    }
}

fn test_hashmap_borrow() {
    use std::collections::HashMap;
    
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("clave".to_string(), 42);
    
    let key: &str = "clave";
    let result = map.get(key);
    
    println!("  HashMap<String, i32>");
    println!("  - get(&str) = {:?}", result);
    println!("  ✅ get() acepta: &str (no necesita &String)");
}
