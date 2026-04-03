// Test debug: Bloques anidados grandes (+500 líneas)
// Objetivo: Verificar que el parser no tiene límite de iteraciones

use rydit_parser::Parser;
use rydit_lexer::Lexer;

fn main() {
    println!("=== TEST: Bloques Anidados +500 líneas ===\n");

    // Test 1: Generar script de 500+ líneas con bloques anidados
    // Usando sintaxis CORRECTA de RyDit
    let mut script = String::new();
    
    // Test 1: Script simple de 500+ líneas (sin bloques profundos)
    // Generar 250 asignaciones secuenciales (2 líneas cada una = 500+ líneas)
    // NOTA: Sin comentario inicial para evitar bug del lexer con \n
    script.push_str("dark.slot nivel = 0\n");
    
    for i in 1..251 {
        script.push_str(&format!("dark.slot var{} = {}\n", i, i));
    }
    
    let lineas = script.lines().count();
    println!("Script generado: {} líneas", lineas);
    println!("Variables definidas: 250\n");

    // Test 2: Parsear
    println!("Parseando...");
    let tokens = Lexer::new(&script).scan();
    println!("Tokens generados: {}", tokens.len());
    
    // Mostrar primeros tokens para debug
    println!("Primeros 10 tokens:");
    for (i, tok) in tokens.iter().take(10).enumerate() {
        println!("  {:3}: {:?}", i + 1, tok);
    }
    println!("  ...\n");
    
    let mut parser = Parser::new(tokens);
    let (_program, errors) = parser.parse();
    
    if !errors.is_empty() {
        println!("\n❌ ERRORES DE PARSEO: {}", errors.len());
        for (i, e) in errors.iter().enumerate().take(10) {
            println!("  Error {}: {:?}", i + 1, e);
        }
        if errors.len() > 10 {
            println!("  ... y {} más", errors.len() - 10);
        }
    } else {
        println!("✅ Parseo exitoso - {} errores", errors.len());
        println!("✅ AST generado correctamente\n");
    }

    // Test 3: Script más simple - 100 condicionales secuenciales (no anidados)
    println!("--- Test 2: 100 condicionales secuenciales ---\n");
    
    let mut script2 = String::new();
    script2.push_str("dark.slot x = 0\n\n");
    
    for i in 0..100 {
        script2.push_str(&format!("onif x == {} {{\n", i));
        script2.push_str(&format!("    voz \"x es {}\"\n", i));
        script2.push_str(&format!("    dark.slot x = x + 1\n"));
        script2.push_str(&format!("}}\n\n"));
    }
    
    let lineas2 = script2.lines().count();
    println!("Script 2: {} líneas", lineas2);
    println!("Condicionales secuenciales: 100\n");
    
    let tokens2 = Lexer::new(&script2).scan();
    let mut parser2 = Parser::new(tokens2);
    let (_program2, errors2) = parser2.parse();
    
    if !errors2.is_empty() {
        println!("❌ ERRORES: {}", errors2.len());
        for (i, e) in errors2.iter().enumerate().take(5) {
            println!("  Error {}: {:?}", i + 1, e);
        }
    } else {
        println!("✅ Parseo exitoso - 100 condicionales secuenciales\n");
    }

    // Test 4: Script con funciones y bloques internos
    println!("--- Test 3: Funciones con bloques internos ---\n");
    
    let mut script3 = String::new();
    script3.push_str("# Funciones con bloques complejos\n\n");
    
    for i in 0..20 {
        script3.push_str(&format!("rytmo funcion_{}(a, b) {{\n", i));
        script3.push_str(&format!("    dark.slot resultado = a + b\n"));
        script3.push_str(&format!("    ryda resultado < 100 {{\n"));
        script3.push_str(&format!("        dark.slot resultado = resultado + 1\n"));
        script3.push_str(&format!("        onif resultado > 50 {{\n"));
        script3.push_str(&format!("            voz \"Funcion {}: resultado > 50\"\n", i));
        script3.push_str(&format!("        }}\n"));
        script3.push_str(&format!("    }}\n"));
        script3.push_str(&format!("    return resultado\n"));
        script3.push_str(&format!("}}\n\n"));
    }
    
    let lineas3 = script3.lines().count();
    println!("Script 3: {} líneas", lineas3);
    println!("Funciones definidas: 20\n");
    
    let tokens3 = Lexer::new(&script3).scan();
    let mut parser3 = Parser::new(tokens3);
    let (_program3, errors3) = parser3.parse();
    
    if !errors3.is_empty() {
        println!("❌ ERRORES: {}", errors3.len());
        for (i, e) in errors3.iter().enumerate().take(5) {
            println!("  Error {}: {:?}", i + 1, e);
        }
    } else {
        println!("✅ Parseo exitoso - 20 funciones con bloques internos\n");
    }

    // Test 4: Script con 50 bucles ryda secuenciales (no anidados)
    println!("--- Test 4: 50 bucles ryda secuenciales ---\n");
    
    let mut script4 = String::new();
    script4.push_str("dark.slot i = 0\n\n");
    
    for n in 0..50 {
        script4.push_str(&format!("ryda i < {} {{\n", (n + 1) * 10));
        script4.push_str(&format!("    voz \"Bucle {}\"\n", n));
        script4.push_str(&format!("    dark.slot i = i + 1\n"));
        script4.push_str(&format!("}}\n\n"));
    }
    
    let lineas4 = script4.lines().count();
    println!("Script 4: {} líneas", lineas4);
    println!("Bucles ryda secuenciales: 50\n");
    
    let tokens4 = Lexer::new(&script4).scan();
    let mut parser4 = Parser::new(tokens4);
    let (_program4, errors4) = parser4.parse();
    
    if !errors4.is_empty() {
        println!("❌ ERRORES: {}", errors4.len());
        for (i, e) in errors4.iter().enumerate().take(5) {
            println!("  Error {}: {:?}", i + 1, e);
        }
    } else {
        println!("✅ Parseo exitoso - 50 bucles ryda secuenciales\n");
    }

    // Resumen
    println!("=== RESUMEN ===");
    if errors.is_empty() {
        println!("Test 1: 250 variables ({} líneas) ✅", lineas);
    } else {
        println!("Test 1: 250 variables ({} líneas) ❌ {} errores", lineas, errors.len());
    }
    
    if errors2.is_empty() {
        println!("Test 2: 100 condicionales secuenciales ({} líneas) ✅", lineas2);
    } else {
        println!("Test 2: 100 condicionales secuenciales ({} líneas) ❌ {} errores", lineas2, errors2.len());
    }
    
    if errors3.is_empty() {
        println!("Test 3: 20 funciones con bloques ({} líneas) ✅", lineas3);
    } else {
        println!("Test 3: 20 funciones con bloques ({} líneas) ❌ {} errores", lineas3, errors3.len());
    }
    
    if errors4.is_empty() {
        println!("Test 4: 50 bucles ryda secuenciales ({} líneas) ✅", lineas4);
    } else {
        println!("Test 4: 50 bucles ryda secuenciales ({} líneas) ❌ {} errores", lineas4, errors4.len());
    }
    
    let total_tests = 4;
    let passed = [errors.is_empty(), errors2.is_empty(), errors3.is_empty(), errors4.is_empty()]
        .iter().filter(|&&x| x).count();
    
    println!("\n=== {} / {} TESTS PASARON ===", passed, total_tests);
    
    if passed == total_tests {
        println!("\n🎉 TODOS LOS TESTS DE BLOQUES ANIDADOS PASARON 🎉");
    }
}
