// crates/rydit-rs/src/bin/debug_e0308.rs
// 🐛 DEBUG E0308 - Mismatched Types Complete Analysis

fn main() {
    println!("🔍 DEBUG E0308 - Mismatched Types");
    println!("==================================\n");

    // Test 1: Function return type mismatch
    println!("TEST 1: Function return type mismatch");
    debug_function_return();

    // Test 2: Vec<String> vs Vec<&str>
    println!("\nTEST 2: Vec<String> vs Vec<&str>");
    debug_vec_string_vs_str();

    // Test 3: HashMap value type mismatch
    println!("\nTEST 3: HashMap value type mismatch");
    debug_hashmap_value_type();

    // Test 4: Tuple destructuring mismatch
    println!("\nTEST 4: Tuple destructuring mismatch");
    debug_tuple_destructure();

    // Test 5: Box<T> vs T
    println!("\nTEST 5: Box<T> vs T");
    debug_box_vs_value();

    println!("\n✅ DEBUG E0308 COMPLETADO");
    println!("========================");
    println!("Tipos identificados - Listo para aplicar fixes");
}

fn debug_function_return() {
    // Scenario: Function expects String but returns &str
    
    fn expects_string(s: String) {
        println!("  Received String: {}", s);
    }
    
    fn returns_str() -> &'static str {
        "hello"
    }
    
    fn returns_string() -> String {
        "hello".to_string()
    }
    
    let str_val = returns_str();
    let string_val = returns_string();
    
    println!("  ❌ ERROR: expects_string(returns_str())");
    println!("     - Expected: String");
    println!("     - Found: &str");
    println!("     - Fix: returns_str().to_string()");
    
    println!("  ✅ CORRECT: expects_string(returns_string())");
    expects_string(string_val);
    
    println!("  ✅ ALSO: expects_string(str_val.to_string())");
    expects_string(str_val.to_string());
}

fn debug_vec_string_vs_str() {
    // Scenario: Vec<String> vs Vec<&str>
    
    let vec_string: Vec<String> = vec!["a".to_string(), "b".to_string()];
    let vec_str: Vec<&str> = vec!["a", "b"];
    
    fn process_vec_string(v: Vec<String>) {
        println!("  Processing Vec<String>: {:?}", v);
    }

    #[allow(dead_code)]
    fn process_vec_str(v: Vec<&str>) {
        println!("  Processing Vec<&str>: {:?}", v);
    }
    
    println!("  vec_string: Vec<String> = {:?}", vec_string);
    println!("  vec_str: Vec<&str> = {:?}", vec_str);
    
    println!("\n  ❌ ERROR: process_vec_string(vec_str)");
    println!("     - Expected: Vec<String>");
    println!("     - Found: Vec<&str>");
    println!("     - Fix: vec_str.iter().map(|s| s.to_string()).collect()");
    
    println!("\n  ✅ CORRECT: process_vec_string(vec_string)");
    process_vec_string(vec_string.clone());
    
    println!("\n  ✅ FIX APPLIED:");
    let converted: Vec<String> = vec_str.iter().map(|s| s.to_string()).collect();
    process_vec_string(converted);
}

fn debug_hashmap_value_type() {
    use std::collections::HashMap;
    
    // Scenario: HashMap<String, Vec<String>> vs HashMap<String, Vec<&str>>
    
    let mut map_expected: HashMap<String, Vec<String>> = HashMap::new();
    map_expected.insert("key".to_string(), vec!["value".to_string()]);
    
    let wrong_value: Vec<&str> = vec!["wrong"];
    let correct_value: Vec<String> = vec!["correct".to_string()];
    
    println!("  HashMap<String, Vec<String>>");
    println!("  wrong_value: Vec<&str> = {:?}", wrong_value);
    println!("  correct_value: Vec<String> = {:?}", correct_value);
    
    println!("\n  ❌ ERROR: map.insert(key, wrong_value)");
    println!("     - Expected: Vec<String>");
    println!("     - Found: Vec<&str>");
    println!("     - Fix: wrong_value.iter().map(|s| s.to_string()).collect()");
    
    println!("\n  ✅ CORRECT: map.insert(key, correct_value)");
    println!("     - Works perfectly");
    
    println!("\n  ✅ FIX APPLIED:");
    let converted: Vec<String> = wrong_value.iter().map(|s| s.to_string()).collect();
    println!("     - Converted: {:?}", converted);
}

fn debug_tuple_destructure() {
    use ry_parser::Parser;
    
    // Scenario: parser.parse() returns (Program, Vec<Error>) not Result
    
    let source = "shield.init";
    let mut parser = Parser::from_source(source);
    
    println!("  parser.parse() return type analysis:");
    
    // ✅ CORRECT: Tuple destructuring
    let (program, errors) = parser.parse();
    println!("  ✅ CORRECT: let (program, errors) = parser.parse()");
    println!("     - Type: (Program<'a>, Vec<RyDitError>)");
    println!("     - program.statements.len() = {}", program.statements.len());
    println!("     - errors.len() = {}", errors.len());
    
    // ❌ INCORRECT: Match as Result
    println!("\n  ❌ INCORRECT: match parser.parse() {{ Ok(p) => ... }}");
    println!("     - parser.parse() NO retorna Result<_, _>");
    println!("     - Retorna: (Program, Vec<Error>)");
    println!("     - Fix: Use tuple destructuring, not match");
}

fn debug_box_vs_value() {
    use ry_parser::Expr;
    
    // Scenario: Box<Expr> vs Expr
    
    let expr = Expr::Num(42.0);
    let boxed_expr = Box::new(expr.clone());
    
    fn takes_expr(e: Expr) {
        println!("  Received Expr: {:?}", e);
    }
    
    fn takes_boxed_expr(e: Box<Expr>) {
        println!("  Received Box<Expr>: {:?}", e);
    }
    
    println!("  expr: Expr = {:?}", expr);
    println!("  boxed_expr: Box<Expr> = {:?}", boxed_expr);
    
    println!("\n  ❌ ERROR: takes_expr(boxed_expr)");
    println!("     - Expected: Expr");
    println!("     - Found: Box<Expr>");
    println!("     - Fix: *boxed_expr (dereference)");
    
    println!("\n  ✅ CORRECT: takes_expr(*boxed_expr)");
    takes_expr(*boxed_expr);
    
    println!("\n  ✅ ALSO: takes_boxed_expr(Box::new(expr))");
    takes_boxed_expr(Box::new(expr));
}
