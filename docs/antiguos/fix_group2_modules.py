#!/usr/bin/env python3
"""
FIX GROUP 2: Sistema de Módulos (Imports)
- Fix 3 bloques idénticos de carga de módulos
- Lizer → Lexer
- parser.parse() → (program, errors)
- String/&str conversions
"""

import re
import subprocess
import sys

FILE = "crates/rydit-rs/src/main.rs"
BACKUP = "crates/rydit-rs/src/main.rs.backup_group2"

def backup():
    import shutil
    shutil.copy(FILE, BACKUP)
    print(f"✓ Backup creado: {BACKUP}")

def fix_imports_contains():
    """Fix importing_stack.contains(module) → &module.to_string()"""
    with open(FILE, 'r') as f:
        content = f.read()
    
    # 3 ocurrencias
    content = re.sub(
        r'importing_stack\.contains\(module\)',
        'importing_stack.contains(&module.to_string())',
        content
    )
    
    print("✓ Fix: importing_stack.contains()")

def fix_loaded_contains():
    """Fix loaded_modules.contains(module.as_str()) → loaded_modules.contains(module)"""
    with open(FILE, 'r') as f:
        content = f.read()
    
    # 3 ocurrencias - eliminar .as_str()
    content = re.sub(
        r'loaded_modules\.contains\(module\.as_str\(\)\)',
        'loaded_modules.contains(module)',
        content
    )
    
    print("✓ Fix: loaded_modules.contains()")

def fix_stack_push():
    """Fix importing_stack.push(module.clone()) → module.to_string()"""
    with open(FILE, 'r') as f:
        content = f.read()
    
    # 3 ocurrencias
    content = re.sub(
        r'importing_stack\.push\(module\.clone\(\)\)',
        'importing_stack.push(module.to_string())',
        content
    )
    
    print("✓ Fix: importing_stack.push()")

def fix_modules_insert():
    """Fix loaded_modules.insert(module.clone()) → module.to_string()"""
    with open(FILE, 'r') as f:
        content = f.read()
    
    # 3 ocurrencias
    content = re.sub(
        r'loaded_modules\.insert\(module\.clone\(\)\)',
        'loaded_modules.insert(module.to_string())',
        content
    )
    
    print("✓ Fix: loaded_modules.insert()")

def fix_lizer():
    """Fix Lizer::new() → Lexer::new()"""
    with open(FILE, 'r') as f:
        content = f.read()
    
    # 3 ocurrencias
    content = re.sub(r'Lizer::new\(', 'Lexer::new(', content)
    
    print("✓ Fix: Lizer → Lexer")

def fix_parser_parse():
    """Fix parser.parse() de Result a (Program, Vec<Error>)"""
    with open(FILE, 'r') as f:
        content = f.read()
    
    # Pattern 1: match parser.parse() { Ok(p) => p, Err(e) => { ... } }
    # Reemplazar con: let (program, errors) = parser.parse(); if !errors.is_empty() { ... }
    
    # Hay 3 bloques idénticos - usar contador
    count = 0
    
    def replace_parse(match):
        nonlocal count
        count += 1
        
        # Extraer el bloque de error handling
        err_block = match.group(1)
        
        new_code = f'''let (program, errors) = parser.parse();
                if !errors.is_empty() {{
                    eprintln!("[ERROR] {{}} errores en módulo", errors.len());
                    for e in &errors {{
                        eprintln!("  - {{}}", e);
                    }}
                    continue;
                }}'''
        
        return new_code
    
    # Pattern más específico para los 3 bloques
    old_pattern = r'match parser\.parse\(\)\s*\{\s*Ok\(p\)\s*=>\s*p,\s*Err\(e\)\s*=>\s*\{[^}]+\}\s*\}'
    
    content = re.sub(old_pattern, replace_parse, content, flags=re.DOTALL)
    
    print(f"✓ Fix: parser.parse() ({count} bloques)")

def fix_original_funcs_push():
    """Fix original_funcs.push(name.clone()) → name.to_string()"""
    with open(FILE, 'r') as f:
        content = f.read()
    
    # 3 ocurrencias
    content = re.sub(
        r'original_funcs\.push\(name\.clone\(\)\)',
        'original_funcs.push(name.to_string())',
        content
    )
    
    print("✓ Fix: original_funcs.push()")

def compile_test():
    print("\n🔨 Compilando para verificar...")
    result = subprocess.run(
        ["cargo", "build", "-p", "rydit-rs", "--bin", "rydit-rs"],
        capture_output=True,
        text=True
    )
    
    errors = result.stderr.count("error")
    if errors == 0:
        print("✅ ¡COMPILACIÓN EXITOSA!")
        return True
    else:
        print(f"⚠ Compilación falló con {errors} errores")
        print(result.stderr[-500:])  # Últimos 500 chars
        return False

def main():
    print("=" * 60)
    print("FIX GROUP 2: Sistema de Módulos")
    print("=" * 60)
    
    backup()
    
    fix_imports_contains()
    fix_loaded_contains()
    fix_stack_push()
    fix_modules_insert()
    fix_lizer()
    fix_parser_parse()
    fix_original_funcs_push()
    
    success = compile_test()
    
    if not success:
        print("\n⚠ Fix falló. ¿Revertir? (y/n): ", end="")
        if input().lower() == 'y':
            import shutil
            shutil.copy(BACKUP, FILE)
            print("✓ Revertido a backup")
        sys.exit(1)
    
    print("\n✅ GROUP 2 COMPLETADO")

if __name__ == "__main__":
    main()
