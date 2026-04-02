// crates/rydit-rs/src/bin/debug_6_errors.rs
// 🐛 DEBUG TEST - 6 Errores Restantes

fn main() {
    println!("🔍 DEBUG TEST - 6 Errores Restantes");
    println!("====================================\n");

    // ERROR #1-2: E0308 if/else type mismatch
    println!("ERROR #1-2: E0308 if/else type mismatch");
    debug_e0308_if_else();

    // ERROR #3-5: E0597 lifetime issues
    println!("\nERROR #3-5: E0597 lifetime issues");
    debug_e0597_lifetimes();

    // ERROR #6: Compilation failed
    println!("\nERROR #6: Compilation failed (resultado de los anteriores)");
    println!("  ✅ Se resuelve al fixear #1-5");

    println!("\n✅ DEBUG TEST COMPLETADO");
    println!("========================");
    println!("Fixes identificados - Listo para aplicar");
}

fn debug_e0308_if_else() {
    // Scenario: if/else branches return different types
    
    println!("  Scenario: if/else con tipos incompatibles");
    
    // ❌ ERROR:
    // if condition {
    //     some_value  // Type A
    // } else {
    //     other_value // Type B (different!)
    // }
    
    // ✅ FIX: Ambos branches deben retornar el mismo tipo
    let condition = true;
    let result: Option<String> = if condition {
        Some("value".to_string())
    } else {
        None  // Mismo tipo: Option<String>
    };
    
    println!("  ✅ FIX: Ambos branches retornan Option<String>");
    println!("     - if branch: Some(value)");
    println!("     - else branch: None");
    println!("     - result = {:?}", result);
    
    // También puede ser:
    let result2: String = if condition {
        "value".to_string()
    } else {
        String::new()  // Mismo tipo: String
    };
    
    println!("  ✅ ALSO: Ambos branches retornan String");
    println!("     - result2 = '{}'", result2);
}

fn debug_e0597_lifetimes() {
    // Scenario: Variable no vive suficiente
    
    println!("  Scenario: Variable que se destruye antes de tiempo");
    
    // ❌ ERROR #3: `input` does not live long enough
    // fn process() -> &str {
    //     let input = String::from("hello");
    //     &input  // ERROR: input se destruye al final del fn
    // }
    
    // ✅ FIX: Retornar String poseído, no referencia
    fn process_fixed() -> String {
        let input = String::from("hello");
        input  // ✅ String poseído, no referencia
    }
    
    let result = process_fixed();
    println!("  ✅ FIX #3: Retornar String, no &str");
    println!("     - result = '{}'", result);
    
    // ❌ ERROR #4: `module_content` does not live long enough
    // let module_content = fs::read_to_string(&path)?;
    // let tokens = Lexer::new(&module_content).scan();
    // usar_tokens_en_loop(&tokens);  // module_content se destruye antes
    
    // ✅ FIX: Mantener module_content vivo mientras se usen tokens
    let module_content = String::from("shield.init");
    {
        let tokens = debug_lexer_scan(&module_content);
        println!("  ✅ FIX #4: module_content vive mientras se usan tokens");
        println!("     - tokens.len() = {}", tokens);
    }
    // module_content todavía vivo aquí
    
    // ❌ ERROR #5: `content` does not live long enough
    // Similar a ERROR #4
    
    // ✅ FIX: Mismo patrón - mantener content vivo
    let content = String::from("dark.slot x = 100");
    {
        let program = debug_parser_parse(&content);
        println!("  ✅ FIX #5: content vive mientras se usa program");
        println!("     - program.statements = {}", program);
    }
    // content todavía vivo aquí
}

fn debug_lexer_scan(source: &str) -> usize {
    // Simular lexer scan
    source.len()
}

fn debug_parser_parse(_source: &str) -> usize {
    // Simular parser parse
    1  // 1 statement
}
