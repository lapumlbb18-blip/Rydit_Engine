#!/usr/bin/env python3
"""
FIX GROUP 5: Executor.rs (2 errores)
- funcs.insert(name.clone(), (params.clone(), ...))
"""

import re, subprocess, sys, shutil

FILE = "crates/rydit-rs/src/executor.rs"
BACKUP = "crates/rydit-rs/src/executor.rs.backup_group5"

def main():
    print("=" * 60)
    print("FIX GROUP 5: Executor.rs")
    print("=" * 60)
    
    shutil.copy(FILE, BACKUP)
    
    with open(FILE, 'r') as f:
        content = f.read()
    
    # Fix: funcs.insert(name.clone(), (params.clone(), body.clone()))
    old = 'funcs.insert(name.clone(), (params.clone(), body.clone()));'
    new = 'funcs.insert(name.to_string(), (params.iter().map(|s| s.to_string()).collect(), body.clone()));'
    
    if old in content:
        content = content.replace(old, new)
        print("✓ Fix: funcs.insert()")
    else:
        print("⚠ Pattern no encontrado")
    
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
    
    print("\n✅ GROUP 5 COMPLETADO")

if __name__ == "__main__":
    main()
