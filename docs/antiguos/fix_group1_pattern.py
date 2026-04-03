#!/usr/bin/env python3
"""
FIX GROUP 1A: Pattern Matching Stmt::Call
- Cambia Stmt::Call { name, args } → Stmt::Call { callee, args }
- Agrega extracción de func_name desde callee
- 2 ubicaciones: líneas 1450 y 4454
"""

import re
import subprocess
import sys

FILE = "crates/rydit-rs/src/main.rs"
BACKUP = "crates/rydit-rs/src/main.rs.backup_group1"

def backup():
    """Crear backup del archivo"""
    import shutil
    shutil.copy(FILE, BACKUP)
    print(f"✓ Backup creado: {BACKUP}")

def fix_pattern_1450():
    """Fix línea 1450: Stmt::Call { name, args }"""
    with open(FILE, 'r') as f:
        content = f.read()
    
    # Pattern 1: Stmt::Call { name, args } => {
    old_pattern = r'Stmt::Call\s*\{\s*name,\s*args\s*\}\s*=>\s*\{'
    new_pattern = 'Stmt::Call { callee, args } => {\n            let func_name = if let Expr::Var(name) = callee.as_ref() {\n                *name\n            } else {\n                return Valor::Error("Call requiere función válida".to_string());\n            };'
    
    content = re.sub(old_pattern, new_pattern, content, count=1)
    
    with open(FILE, 'w') as f:
        f.write(content)
    
    print("✓ Fix aplicado en línea ~1450")

def fix_pattern_4454():
    """Fix línea 4454: Stmt::Call { name, args } => {"""
    with open(FILE, 'r') as f:
        content = f.read()
    
    # Contar ocurrencias para saber cuál es la segunda
    matches = list(re.finditer(r'Stmt::Call\s*\{\s*name,\s*args\s*\}\s*=>\s*\{', content))
    if len(matches) < 2:
        print(f"⚠ Solo se encontraron {len(matches)} ocurrencias, se esperan 2")
        return False
    
    # Reemplazar la segunda ocurrencia
    match = matches[1]
    old_text = match.group(0)
    new_text = 'Stmt::Call { callee, args } => {\n            let func_name = if let Expr::Var(name) = callee.as_ref() {\n                *name\n            } else {\n                return Valor::Error("Call requiere función válida".to_string());\n            };'
    
    content = content[:match.start()] + new_text + content[match.end():]
    
    with open(FILE, 'w') as f:
        f.write(content)
    
    print("✓ Fix aplicado en línea ~4454")

def fix_expr_call_4458():
    """Fix línea 4458: Expr::Call { name: ... } → Expr::Call { callee: ... }"""
    with open(FILE, 'r') as f:
        content = f.read()
    
    # Pattern: name: name.clone() dentro de Expr::Call
    old_pattern = r'Expr::Call\s*\{\s*name:\s*name\.clone\(\),'
    new_pattern = 'Expr::Call { callee: Box::new(Expr::Var(func_name)),'
    
    content = re.sub(old_pattern, new_pattern, content, count=1)
    
    with open(FILE, 'w') as f:
        f.write(content)
    
    print("✓ Fix aplicado en línea ~4458")

def compile_test():
    """Compilar para verificar fix"""
    print("\n🔨 Compilando para verificar...")
    result = subprocess.run(
        ["cargo", "build", "-p", "rydit-rs", "--bin", "rydit-rs", "2>&1"],
        shell=True,
        capture_output=True,
        text=True
    )
    
    if result.returncode == 0:
        print("✅ ¡COMPILACIÓN EXITOSA!")
        return True
    else:
        errors = result.stderr.count("error")
        print(f"⚠ Compilación falló con {errors} errores")
        return False

def main():
    print("=" * 60)
    print("FIX GROUP 1A: Pattern Matching Stmt::Call")
    print("=" * 60)
    
    backup()
    
    fix_pattern_1450()
    fix_pattern_4454()
    fix_expr_call_4458()
    
    success = compile_test()
    
    if not success:
        print("\n⚠ Fix falló. ¿Revertir? (y/n): ", end="")
        if input().lower() == 'y':
            import shutil
            shutil.copy(BACKUP, FILE)
            print("✓ Revertido a backup")
        sys.exit(1)
    
    print("\n✅ GROUP 1A COMPLETADO")

if __name__ == "__main__":
    main()
