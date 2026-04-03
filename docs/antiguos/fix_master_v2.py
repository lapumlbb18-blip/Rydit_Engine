#!/usr/bin/env python3
"""
FIX MASTER SCRIPT v2 - RyDit-Rs v0.11.4
Fixea errores específicos identificados en el análisis
"""

import subprocess
import sys
import os
import re
from datetime import datetime

def log(msg):
    print(f"\n{'='*60}")
    print(msg)
    print(f"{'='*60}\n")

def count_errors():
    """Contar errores actuales"""
    result = subprocess.run(
        ["cargo", "build", "-p", "rydit-rs", "--bin", "rydit-rs"],
        capture_output=True,
        text=True
    )
    return result.stderr.count("error")

def apply_fix(description, pattern, replacement, files):
    """Aplicar fix con regex"""
    print(f"\n🔧 {description}")
    
    for filepath in files:
        if not os.path.exists(filepath):
            continue
            
        with open(filepath, 'r') as f:
            content = f.read()
        
        # Crear backup
        backup = filepath + ".backup_v2"
        if not os.path.exists(backup):
            with open(backup, 'w') as f:
                f.write(content)
            print(f"  ✓ Backup: {backup}")
        
        # Aplicar fix
        new_content, count = re.subn(pattern, replacement, content)
        if count > 0:
            with open(filepath, 'w') as f:
                f.write(new_content)
            print(f"  ✓ {count} reemplazos en {filepath}")
        else:
            print(f"  ⚠ No se encontraron patrones en {filepath}")

def main():
    log("🛠️  RYDIT-RS FIX MASTER v2 - Análisis de Errores")
    print(f"Fecha: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")

    initial_errors = count_errors()
    print(f"\n📊 Errores iniciales: {initial_errors}")

    if initial_errors == 0:
        print("✅ ¡No hay errores!")
        return

    # FIX 1: E0433 - Lizer → Lexer (4 errores)
    log("FIX 1: Lizer → Lexer (E0433)")
    apply_fix(
        "Lizer → Lexer",
        r'\bLizer\b',
        'Lexer',
        ['crates/rydit-rs/src/main.rs', 'crates/rydit-rs/src/repl.rs']
    )

    # FIX 2: E0658 - .as_str() inestable (3 errores)
    log("FIX 2: .as_str() → .to_string() (E0658)")
    apply_fix(
        ".as_str() → .to_string()",
        r'\.as_str\(\)',
        '.to_string()',
        ['crates/rydit-rs/src/main.rs']
    )

    # FIX 3: E0599 - .into_string() en Result (2 errores)
    log("FIX 3: ureq .into_string() fix (E0599)")
    apply_fix(
        "ureq response handling",
        r'ureq::get\(&url\)\.call\(\)\.into_string\(\)',
        'ureq::get(&url).call().map(|r| r.into_string()).unwrap_or(Err("HTTP error".to_string()))',
        ['crates/rydit-rs/src/eval/mod.rs']
    )

    # FIX 4: E0026/E0027 - Stmt::Call { name } → { callee } (2 errores)
    log("FIX 4: Stmt::Call callee (E0026/E0027)")
    apply_fix(
        "Stmt::Call { name, args } → { callee, args }",
        r'Stmt::Call\s*\{\s*name,',
        'Stmt::Call { callee,',
        ['crates/rydit-rs/src/main.rs', 'crates/rydit-rs/src/eval/mod.rs']
    )

    # FIX 5: E0277 - &str vs str comparison (12 errores)
    log("FIX 5: &str vs str comparison (E0277)")
    apply_fix(
        "func_name == &str → func_name == str",
        r'func_name == &"([^"]+)"',
        r'func_name == "\1"',
        ['crates/rydit-rs/src/main.rs']
    )

    # COMPILAR PARA VERIFICAR
    log("🔨 COMPILANDO PARA VERIFICAR")
    errors = count_errors()
    print(f"\n📊 Errores después de fixes: {errors}")
    print(f"📉 Errores fixeados: {initial_errors - errors}")

    if errors == 0:
        print("\n🎉 ¡COMPILACIÓN EXITOSA!")
    else:
        print(f"\n⚠️  Aún hay {errors} errores")
        print("\n💡 Próximos pasos:")
        print("  1. Revisar errores E0308 (type mismatch) - 41 errores")
        print("  2. Fix manual de name/func_name scope - 6 errores")
        print("  3. Fixear ureq post/put - 2 errores")

if __name__ == "__main__":
    main()
