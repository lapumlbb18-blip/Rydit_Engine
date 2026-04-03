#!/usr/bin/env python3
"""
FIX GROUP 6: Repl.rs (3 errores)
- Lizer → Lexer
- parser.parse() Result → (Program, Vec<Error>)
"""

import re, subprocess, sys, shutil

FILE = "crates/rydit-rs/src/repl.rs"
BACKUP = "crates/rydit-rs/src/repl.rs.backup_group6"

def main():
    print("=" * 60)
    print("FIX GROUP 6: Repl.rs")
    print("=" * 60)
    
    shutil.copy(FILE, BACKUP)
    
    with open(FILE, 'r') as f:
        content = f.read()
    
    # 1. Lizer → Lexer
    content = re.sub(r'Lizer::new\(', 'Lexer::new(', content)
    print("✓ Fix: Lizer → Lexer")
    
    # 2. Fix parser.parse() match
    old = '''match parser.parse() {
                    Ok(program) => {
                        println!("[RYDIT] {} statements", program.statements.len());
                        // Ejecutar statements
                        for stmt in &program.statements {
                            crate::ejecutar_stmt(
                                stmt,
                                &mut executor,
                                &mut funcs,
                                &mut loaded_modules,
                                &mut importing_stack,
                            );
                        }
                    }
                    Err(e) => {
                        println!("[ERROR] {}", e);
                    }
                }'''
    
    new = '''let (program, errors) = parser.parse();
                if errors.is_empty() {
                    println!("[RYDIT] {} statements", program.statements.len());
                    for stmt in &program.statements {
                        crate::ejecutar_stmt(
                            stmt,
                            &mut executor,
                            &mut funcs,
                            &mut loaded_modules,
                            &mut importing_stack,
                        );
                    }
                } else {
                    println!("[ERROR] {{}} errores", errors.len());
                    for e in &errors {
                        println!("  - {{}}", e);
                    }
                }'''
    
    if old in content:
        content = content.replace(old, new)
        print("✓ Fix: parser.parse() match")
    else:
        print("⚠ Pattern no encontrado (puede variar whitespace)")
    
    with open(FILE, 'w') as f:
        f.write(content)
    
    print("\n🔨 Compilando...")
    result = subprocess.run(["cargo", "build", "-p", "rydit-rs", "--bin", "rydit-rs"], capture_output=True, text=True)
    
    if result.returncode == 0:
        print("✅ ¡EXITOSO!")
    else:
        errors = result.stderr.count("error")
        print(f"⚠ {errors} errores")
        if input("¿Revertir? (y/n): ").lower() == 'y':
            shutil.copy(BACKUP, FILE)
        sys.exit(1)
    
    print("\n✅ GROUP 6 COMPLETADO")

if __name__ == "__main__":
    main()
