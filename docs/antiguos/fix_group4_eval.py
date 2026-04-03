#!/usr/bin/env python3
"""
FIX GROUP 4: Eval/Mod.rs (8 errores)
- Expr::Texto(s.clone()) → s.to_string()
- name == "__INPUT__" → *name
- ureq HTTP methods
"""

import re, subprocess, sys, shutil

FILE = "crates/rydit-rs/src/eval/mod.rs"
BACKUP = "crates/rydit-rs/src/eval/mod.rs.backup_group4"

def main():
    print("=" * 60)
    print("FIX GROUP 4: Eval/Mod.rs")
    print("=" * 60)
    
    shutil.copy(FILE, BACKUP)
    
    with open(FILE, 'r') as f:
        content = f.read()
    
    fixes = 0
    
    # 1. Expr::Texto(s.clone()) → Valor::Texto(s.to_string())
    content = re.sub(r'Expr::Texto\(s\) => Valor::Texto\(s\.clone\(\)\)', 
                     'Expr::Texto(s) => Valor::Texto(s.to_string())', content)
    print("✓ Fix: Expr::Texto")
    fixes += 1
    
    # 2. if name == "__INPUT__" → if *name == "__INPUT__"
    content = re.sub(r'if name == "__INPUT__"', 'if *name == "__INPUT__"', content)
    print("✓ Fix: __INPUT__ comparison")
    fixes += 1
    
    # 3. ureq::get().call().into_string() → .call().expect().into_string()
    content = re.sub(r'ureq::get\(&url\)\.call\(\)\.into_string\(\)',
                     'ureq::get(&url).call().expect("HTTP GET failed").into_string()', content)
    print("✓ Fix: ureq GET")
    fixes += 1
    
    # 4. ureq::delete().call().into_string()
    content = re.sub(r'ureq::delete\(&url\)\.call\(\)\.into_string\(\)',
                     'ureq::delete(&url).call().expect("HTTP DELETE failed").into_string()', content)
    print("✓ Fix: ureq DELETE")
    fixes += 1
    
    # 5. Fix unwrap_or(Err("...".to_string())) → unwrap_or_else(|e| Err(e.to_string()))
    content = re.sub(r'\.unwrap_or\(Err\("([^"]+)"\.to_string\(\)\)\)',
                     r'.unwrap_or_else(|e| Err(e.to_string()))', content)
    print("✓ Fix: unwrap_or HTTP error")
    fixes += 1
    
    with open(FILE, 'w') as f:
        f.write(content)
    
    print(f"\n🔨 Compilando...")
    result = subprocess.run(["cargo", "build", "-p", "rydit-rs", "--bin", "rydit-rs"], capture_output=True, text=True)
    
    if result.returncode == 0:
        print("✅ ¡EXITOSO!")
    else:
        errors = result.stderr.count("error")
        print(f"⚠ {errors} errores restantes")
        if input("¿Revertir? (y/n): ").lower() == 'y':
            shutil.copy(BACKUP, FILE)
        sys.exit(1)
    
    print(f"\n✅ GROUP 4 COMPLETADO ({fixes} fixes)")

if __name__ == "__main__":
    main()
