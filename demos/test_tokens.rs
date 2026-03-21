use lizer::Lizer;

fn main() {
    let code = r#"shield.init
import random
dark.slot cols = 30
dark.slot x = random::int(0, cols - 1)
"#;
    let tokens = Lizer::new(code).scan();
    for (i, token) in tokens.iter().enumerate() {
        println!("{:3}: {:?}", i, token);
    }
}
