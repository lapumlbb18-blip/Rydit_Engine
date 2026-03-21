# 🧪 Pruebas CLI - ALERTA-001 Completada

**Fecha:** 2026-03-14  
**Estado:** ✅ COMPLETADO

---

## 📋 Comandos Probados

### 1. Default (sin argumentos)

```bash
$ cargo run
```

**Output:**
```
--- SHIELD SYSTEM: MODO COMANDANTE ---
[RYDIT] Ejecutando: shield.init
[BLAST-CORE]: Motor de audio en guardia.
Escaneando código guerrero...
[RYDIT] 1 tokens encontrados
[BLAST-CORE]: Impacto sónico detectado.
Token procesado por Shield
--- SISTEMA PROTEGIDO ---
```

**Resultado:** ✅ Usa `shield.init` por default

---

### 2. Script Personalizado

```bash
$ cargo run -- "onda.core"
```

**Output:**
```
--- SHIELD SYSTEM: MODO COMANDANTE ---
[RYDIT] Ejecutando: onda.core
[BLAST-CORE]: Motor de audio en guardia.
Escaneando código guerrero...
[RYDIT] 1 tokens encontrados
[BLAST-CORE]: Impacto sónico detectado.
Token procesado por Shield
--- SISTEMA PROTEGIDO ---
```

**Resultado:** ✅ Acepta script como argumento

---

### 3. Con Separador `--` (Estilo RyDit)

```bash
$ cargo run -- -- "shield.init dark.slot delta.flow = 0.5"
```

**Output:**
```
--- SHIELD SYSTEM: MODO COMANDANTE ---
[RYDIT] Ejecutando: shield.init dark.slot delta.flow = 0.5
[BLAST-CORE]: Motor de audio en guardia.
Escaneando código guerrero...
[RYDIT] 5 tokens encontrados
[BLAST-CORE]: Impacto sónico detectado. (x5)
Token procesado por Shield (x5)
--- SISTEMA PROTEGIDO ---
```

**Resultado:** ✅ Parsea 5 tokens correctamente

---

### 4. Múltiples Comandos con Aura

```bash
$ cargo run -- -- "shield.init onda.core prime.ish"
```

**Output:**
```
--- SHIELD SYSTEM: MODO COMANDANTE ---
[RYDIT] Ejecutando: shield.init onda.core prime.ish
[BLAST-CORE]: Motor de audio en guardia.
Escaneando código guerrero...
[RYDIT] 3 tokens encontrados
[BLAST-CORE]: Impacto sónico detectado. (x3)
Token procesado por Shield (x3)
--- SISTEMA PROTEGIDO ---
```

**Resultado:** ✅ Comandos múltiples con aura

---

## 📊 Resumen de Pruebas

| Prueba | Comando | Tokens | Estado |
|--------|---------|--------|--------|
| Default | `cargo run` | 1 | ✅ |
| Personalizado | `cargo run -- "onda.core"` | 1 | ✅ |
| Variables | `cargo run -- -- "dark.slot delta.flow = 0.5"` | 5 | ✅ |
| Múltiple | `cargo run -- -- "shield.init onda.core prime.ish"` | 3 | ✅ |

**Total:** 4/4 pruebas exitosas

---

## 🎯 Estilo RyDit Confirmado

```bash
# Corto y directo
cargo run -- "shield.init"

# Variables con jerarquía
cargo run -- -- "dark.slot delta.flow = 0.5"

# Múltiples comandos
cargo run -- -- "shield.init onda.core prime.ish"

# Futuro (cuando exista binario rydit)
rydit run -- "shield.init"
rydit run script.rydit
```

---

## 📝 Código Implementado

```rust
// crates/rydit-rs/src/main.rs
use lizer::Lizer;
use blast_core::BlastCore;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let default_script = "shield.init";
    
    let script = if let Some(pos) = args.iter().position(|x| x == "--") {
        if args.len() > pos + 1 {
            &args[pos + 1]
        } else {
            default_script
        }
    } else if args.len() > 1 {
        &args[1]
    } else {
        default_script
    };

    println!("--- SHIELD SYSTEM: MODO COMANDANTE ---");
    println!("[RYDIT] Ejecutando: {}", script);

    let audio = BlastCore::despertar();
    let tokens = Lizer::new(script).scan();

    println!("Escaneando código guerrero...");
    println!("[RYDIT] {} tokens encontrados", tokens.len());

    for _token in tokens {
        audio.shock_wave();
        println!("Token procesado por Shield");
    }

    println!("--- SISTEMA PROTEGIDO ---");
}
```

---

## ✅ Criterio de Aceptación

- [x] `rydit run -- "shield.init"` funciona (vía cargo run)
- [x] Si no paso argumento, usa `shield.init` (default con aura)
- [x] Mensaje claro mostrando qué script se ejecuta
- [x] Soporta múltiples comandos en una línea

---

**ALERTA-001:** ✅ COMPLETADA  
**Próxima:** ALERTA-003 (Ejecutor con memoria)
