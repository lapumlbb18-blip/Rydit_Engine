use std::fs;

fn main() {
    let content = fs::read_to_string("test_math.rydit").expect("No se pudo leer");
    let tokens = lizer::Lizer::new(&content).scan();
    
    println!("Todos los tokens:");
    for (i, t) in tokens.iter().enumerate() {
        println!("  {}: {:?}", i, t);
    }
}
