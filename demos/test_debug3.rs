use std::fs;

fn main() {
    let content = fs::read_to_string("test_math.rydit").expect("No se pudo leer");
    let tokens = lizer::Lizer::new(&content).scan();
    
    let mut parser = lizer::Parser::new(tokens);
    match parser.parse() {
        Ok(p) => {
            println!("Statements: {}", p.statements.len());
            for (i, s) in p.statements.iter().enumerate() {
                println!("  {}: {:?}", i, s);
            }
        },
        Err(e) => {
            println!("Error: {} en columna {}", e.message, e.column);
        }
    }
}
