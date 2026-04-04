// crates/rydit-rs/src/bin/debug_mayusculas.rs
// 🐛 DEBUG: ¿Las mayúsculas en tipos afectan E0308?

fn main() {
    println!("🔍 DEBUG: Mayúsculas en Tipos E0308");
    println!("====================================\n");

    // Test 1: String vs string (minúscula no existe en Rust)
    println!("TEST 1: String (S mayúscula)");
    test_string_type();

    // Test 2: Vec<String> vs Vec<string>
    println!("\nTEST 2: Vec<String> (S mayúscula)");
    test_vec_string();

    // Test 3: &str vs &Str (Str no existe)
    println!("\nTEST 3: &str (todo minúscula)");
    test_str_type();

    // Test 4: Consistencia en mensajes de error
    println!("\nTEST 4: Mensajes de error de compilación");
    test_error_messages();

    println!("\n✅ DEBUG MAYÚSCULAS COMPLETADO");
    println!("==============================");
    println!("CONCLUSIÓN: Rust es CASE-SENSITIVE");
    println!("  - String ✅ (S mayúscula)");
    println!("  - str ✅ (todo minúscula)");
    println!("  - string ❌ (no existe)");
    println!("  - Str ❌ (no existe)");
}

fn test_string_type() {
    // ✅ CORRECTO: String con S mayúscula
    let string_val: String = "hello".to_string();
    println!("  ✅ String (S mayúscula): {}", string_val);
    
    // ❌ INCORRECTO: string con s minúscula (no existe en Rust)
    // let string_val: string = "hello"; // ERROR: no existe 'string'
    
    println!("  ❌ string (s minúscula): NO EXISTE en Rust");
    println!("  💡 Rust es CASE-SENSITIVE para tipos");
}

fn test_vec_string() {
    // ✅ CORRECTO: Vec<String> con S mayúscula
    let vec_string: Vec<String> = vec!["a".to_string(), "b".to_string()];
    println!("  ✅ Vec<String> (S mayúscula): {:?}", vec_string);
    
    // ❌ INCORRECTO: Vec<string> con s minúscula
    // let vec_string: Vec<string> = vec!["a", "b"]; // ERROR: no existe 'string'
    
    println!("  ❌ Vec<string> (s minúscula): NO EXISTE");
}

fn test_str_type() {
    // ✅ CORRECTO: &str todo minúscula
    let str_val: &str = "hello";
    println!("  ✅ &str (todo minúscula): {}", str_val);
    
    // ❌ INCORRECTO: &Str con S mayúscula (no existe)
    // let str_val: &Str = "hello"; // ERROR: no existe 'Str'
    
    println!("  ❌ &Str (S mayúscula): NO EXISTE");
}

fn test_error_messages() {
    println!("  Mensajes de error de Rust SON consistentes:");
    println!("  - expected `String`, found `&str`");
    println!("  - expected `Vec<String>`, found `Vec<&str>`");
    println!("  - Las mayúsculas en errores SON CORRECTAS");
    println!("  - NO son el problema de E0308");
    
    println!("\n  💡 Los errores E0308 son por TIPOS INCORRECTOS,");
    println!("     NO por mayúsculas/minúsculas en el mensaje");
}
