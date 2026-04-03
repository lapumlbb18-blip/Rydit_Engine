#!/usr/bin/env python3
"""
FIX MASTER SCRIPT - RyDit-Rs v0.11.2
Ejecuta todos los fixes en secuencia con verificación después de cada grupo
"""

import subprocess
import sys
import os
from datetime import datetime

GROUPS = [
    ("1A", "fix_group1_pattern.py", "Pattern Matching Stmt::Call"),
    ("2", "fix_group2_modules.py", "Sistema de Módulos"),
    ("3", "fix_group3_input.py", "Comparaciones Input"),
    ("4", "fix_group4_eval.py", "Eval/Mod.rs"),
    ("5", "fix_group5_executor.py", "Executor.rs"),
    ("6", "fix_group6_repl.py", "Repl.rs"),
]

def log(msg):
    print(f"\n{'='*60}")
    print(msg)
    print(f"{'='*60}\n")

def count_errors():
    """Contar errores actuales de compilación"""
    result = subprocess.run(
        ["cargo", "build", "-p", "rydit-rs", "--bin", "rydit-rs"],
        capture_output=True,
        text=True,
        cwd=os.path.dirname(os.path.abspath(__file__))
    )
    return result.stderr.count("error")

def run_group(group_id, script, description):
    """Ejecutar un grupo de fix"""
    log(f"GRUPO {group_id}: {description}")
    print(f"Script: {script}")
    
    result = subprocess.run(
        ["python3", script],
        cwd=os.path.dirname(os.path.abspath(__file__))
    )
    
    if result.returncode != 0:
        print(f"\n❌ GRUPO {group_id} FALLÓ")
        return False
    
    errors = count_errors()
    print(f"\n📊 Errores restantes: {errors}")
    return True

def main():
    log("🛠️  RYDIT-RS FIX MASTER - v0.11.2")
    print(f"Fecha: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    
    initial_errors = count_errors()
    print(f"\n📊 Errores iniciales: {initial_errors}")
    
    if initial_errors == 0:
        print("✅ ¡No hay errores que fixear!")
        return
    
    print(f"\n📋 Grupos a ejecutar: {len(GROUPS)}")
    for gid, script, desc in GROUPS:
        print(f"  {gid}. {desc} ({script})")
    
    print("\n⚠️  Esto aplicará fixes automáticamente")
    print("¿Continuar? (y/n): ", end="")
    if input().lower() != 'y':
        print("Cancelado")
        return
    
    completed = 0
    for group_id, script, description in GROUPS:
        if run_group(group_id, script, description):
            completed += 1
        else:
            print(f"\n⚠️  Deteniendo en Grupo {group_id}")
            break
    
    final_errors = count_errors()
    
    log("📊 RESULTADOS FINALES")
    print(f"Grupos completados: {completed}/{len(GROUPS)}")
    print(f"Errores iniciales: {initial_errors}")
    print(f"Errores finales: {final_errors}")
    print(f"Errores fixeados: {initial_errors - final_errors}")
    
    if final_errors == 0:
        print("\n🎉 ¡COMPILACIÓN EXITOSA!")
    else:
        print(f"\n⚠️  Aún hay {final_errors} errores pendientes")

if __name__ == "__main__":
    main()
