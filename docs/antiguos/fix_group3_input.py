#!/usr/bin/env python3
"""
FIX GROUP 3: Comparaciones de Input (E0277)
- Fix 11 comparaciones name == "x" → *name == "x"
"""

import re, subprocess, sys

FILE = "crates/rydit-rs/src/main.rs"
BACKUP = "crates/rydit-rs/src/main.rs.backup_group3"

def main():
    print("=" * 60)
    print("FIX GROUP 3: Comparaciones de Input")
    print("=" * 60)
    
    import shutil
    shutil.copy(FILE, BACKUP)
    print(f"✓ Backup: {BACKUP}")
    
    with open(FILE, 'r') as f:
        content = f.read()
    
    # Fix: name == "..." → *name == "..." en el bloque de input
    # Solo en el contexto específico (líneas ~1268-1278)
    old = '''if name == "x"
                || name == "y"
                || name == "velocidad"
                || name == "frame"
                || name == "click"
                || name == "mx"
                || name == "my"
                || name == "balas"
                || name == "tanque_x"
                || name == "tanque_y"
                || name == "angulo"'''
    
    new = '''if *name == "x"
                || *name == "y"
                || *name == "velocidad"
                || *name == "frame"
                || *name == "click"
                || *name == "mx"
                || *name == "my"
                || *name == "balas"
                || *name == "tanque_x"
                || *name == "tanque_y"
                || *name == "angulo"'''
    
    if old in content:
        content = content.replace(old, new)
        print("✓ Fix: 11 comparaciones de input")
    else:
        print("⚠ Pattern no encontrado")
        sys.exit(1)
    
    with open(FILE, 'w') as f:
        f.write(content)
    
    # Compilar
    print("\n🔨 Compilando...")
    result = subprocess.run(["cargo", "build", "-p", "rydit-rs", "--bin", "rydit-rs"], capture_output=True, text=True)
    
    if result.returncode == 0:
        print("✅ ¡EXITOSO!")
    else:
        errors = result.stderr.count("error")
        print(f"⚠ {errors} errores")
        if input("¿Revertir? (y/n): ").lower() == 'y':
            shutil.copy(BACKUP, FILE)
            print("✓ Revertido")
        sys.exit(1)
    
    print("\n✅ GROUP 3 COMPLETADO")

if __name__ == "__main__":
    main()
