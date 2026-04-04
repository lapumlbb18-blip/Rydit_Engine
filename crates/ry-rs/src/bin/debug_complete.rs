// crates/rydit-rs/src/bin/debug_complete.rs
// 🐛 DEBUG COMPLETO - Todos los tipos de errores restantes

fn main() {
    println!("🔍 DEBUG COMPLETO - 21 Errores Restantes");
    println!("=========================================\n");

    // Error type 1: parser.parse() return type
    println!("ERROR TYPE 1: parser.parse() return type");
    debug_parser_parse();

    // Error type 2: importing_stack type mismatch
    println!("\nERROR TYPE 2: importing_stack type");
    debug_importing_stack();

    // Error type 3: HashMap borrow issues
    println!("\nERROR TYPE 3: HashMap borrow");
    debug_hashmap_borrow();

    // Error type 4: type mismatches (String vs &str)
    println!("\nERROR TYPE 4: String vs &str");
    debug_string_vs_str();

    println!("\n✅ DEBUG COMPLETO FINALIZADO");
    println!("=================================");
    println!("Siguiente paso: Aplicar fixes manuales");
}

fn debug_parser_parse() {
    use ry_parser::Parser;
    
    let source = "shield.init";
    let mut parser = Parser::from_source(source);
    
    // ✅ CORRECTO: parser.parse() retorna (Program, Vec<Error>)
    let (program, errors) = parser.parse();
    
    println!("  ✅ CORRECTO: let (program, errors) = parser.parse()");
    println!("     - Tipo: (Program<'a>, Vec<RyDitError>)");
    println!("     - program.statements.len() = {}", program.statements.len());
    println!("     - errors.len() = {}", errors.len());
    
    // ❌ INCORRECTO: match parser.parse() como Result
    // let result = parser.parse(); // Esto da error E0308
    println!("  ❌ INCORRECTO: match parser.parse() {{ Ok(p) => ... }}");
    println!("     - parser.parse() NO retorna Result");
    println!("     - Retorna: (Program, Vec<Error>)");
}

fn debug_importing_stack() {
    let importing_stack: Vec<String> = vec!["module1".to_string(), "module2".to_string()];
    let module: &str = "module1";
    
    // ✅ CORRECTO: contains() requiere &String
    let result1 = importing_stack.contains(&module.to_string());
    
    // ❌ INCORRECTO: contains() NO acepta &str directamente
    // let result2 = importing_stack.contains(module); // ERROR E0308
    
    println!("  ✅ CORRECTO: importing_stack.contains(&module.to_string())");
    println!("     - importing_stack: Vec<String>");
    println!("     - module: &str");
    println!("     - contains() requiere: &String");
    println!("     - result = {}", result1);
    
    println!("  ❌ INCORRECTO: importing_stack.contains(module)");
    println!("     - Error: expected &String, found &str");
    
    println!("  💡 SOLUCIONES:");
    println!("     1. importing_stack.contains(&module.to_string())");
    println!("     2. O cambiar a: importing_stack: Vec<&str>");
}

fn debug_hashmap_borrow() {
    use std::collections::HashMap;
    
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    map.insert("key".to_string(), vec!["value".to_string()]);
    
    let key: &str = "key";
    
    // ✅ CORRECTO: HashMap::get() acepta &str
    let result1 = map.get(key);
    
    // ✅ TAMBIÉN CORRECTO: con &String
    let result2 = map.get(&key.to_string());
    
    println!("  ✅ CORRECTO: map.get(key) con &str");
    println!("     - HashMap<String, Vec<String>>");
    println!("     - get() acepta: &str (implementa Borrow<str>)");
    println!("     - result = {:?}", result1);
    
    println!("  ✅ TAMBIÉN: map.get(&key.to_string())");
    println!("     - result = {:?}", result2);
    
    // ❌ INCORRECTO: insertar Vec<&str> en lugar de Vec<String>
    println!("  ❌ INCORRECTO: map.insert(key, vec![&str])");
    println!("     - Espera: Vec<String>");
    println!("     - Recibe: Vec<&str>");
}

fn debug_string_vs_str() {
    let string_val: String = "hello".to_string();
    let str_val: &str = "hello";
    
    // ✅ Conversiones correctas
    let str_from_string: &str = &string_val;
    let string_from_str: String = str_val.to_string();
    
    println!("  ✅ String → &str: &string_val");
    println!("     - string_val = \"{}\"", string_val);
    println!("     - str_from_string = \"{}\"", str_from_string);
    
    println!("  ✅ &str → String: str_val.to_string()");
    println!("     - str_val = \"{}\"", str_val);
    println!("     - string_from_str = \"{}\"", string_from_str);
    
    // ❌ Errores comunes
    println!("  ❌ COMÚN: func(String) cuando espera func(&str)");
    println!("     - Solución: &string_val o string_val.as_str()");
    
    println!("  ❌ COMÚN: func(&str) cuando espera func(String)");
    println!("     - Solución: str_val.to_string()");
}
