use std::fs;

fn main() {
    let content = fs::read_to_string("test_math.rydit").expect("No se pudo leer");
    println!("Contenido ({} chars, {} lineas):", content.len(), content.lines().count());
    println!("Primeros 100 chars: {:?}", content.chars().take(100).collect::<String>());
    
    let tokens = lizer::Lizer::new(&content).scan();
    println!("Tokens: {}", tokens.len());
    
    for (i, t) in tokens.iter().enumerate() {
        if i < 20 {
            println!("  {}: {:?}", i, t);
        }
    }
    
    let mut parser = lizer::Parser::new(tokens);
    match parser.parse() {
        Ok(p) => println!("Statements: {}", p.statements.len()),
        Err(e) => println!("Error: {}", e),
    }
}
