# 🔗 PROTOCOLO LAZOS - Implementación Nativa en main.rs

**Filosofía**: Simple sobre complejo. Nativo sobre externo.

---

## 💡 IDEA CENTRAL

**NO hacer**:
- ❌ Crate separado `rydit-lazos`
- ❌ Trait complejo `RyditModule`
- ❌ Librería externa

**SÍ hacer**:
- ✅ Funciones nativas en `main.rs`
- ✅ Loop stdin/stdout simple
- ✅ JSON-RPC básico
- ✅ Todo en el mismo binario

---

## 📝 IMPLEMENTACIÓN EN main.rs

### **Paso 1: Agregar flag --lazos en CLI**

```rust
// crates/rydit-rs/src/cli.rs

pub fn run() {
    let args: Vec<String> = env::args().collect();
    
    if args.contains(&"--lazos".to_string()) {
        // Modo LAZOS - Loop infinito stdin/stdout
        lazos_loop();
        return;
    }
    
    // Modo normal...
}
```

---

### **Paso 2: Loop LAZOS en main.rs**

```rust
// crates/rydit-rs/src/main.rs

use std::io::{self, BufRead, Write};
use serde_json::{json, Value};

/// Loop principal del Protocolo LAZOS
/// Lee comandos JSON desde stdin, ejecuta, responde por stdout
pub fn lazos_loop() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    eprintln!("[LAZOS] Protocolo iniciado - esperando comandos...");
    
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            // Parsear request JSON
            let request: Value = match serde_json::from_str(&line) {
                Ok(req) => req,
                Err(e) => {
                    let error = json!({
                        "jsonrpc": "2.0",
                        "error": format!("Invalid JSON: {}", e),
                        "id": null
                    });
                    writeln!(stdout, "{}", error).unwrap();
                    continue;
                }
            };
            
            // Ejecutar comando
            let response = ejecutar_comando_lazos(&request);
            
            // Responder
            writeln!(stdout, "{}", response).unwrap();
            stdout.flush().unwrap();
        }
    }
}

/// Ejecutar un comando LAZOS
fn ejecutar_comando_lazos(request: &Value) -> Value {
    let method = request["method"].as_str().unwrap_or("");
    let params = request["params"].as_array().unwrap_or(&vec![]);
    let id = request["id"].clone();
    
    // Routing de comandos
    let result = match method {
        // === BEZIER ===
        "science::bezier::linear" => bezier_linear(params),
        "science::bezier::quadratic" => bezier_quadratic(params),
        "science::bezier::cubic" => bezier_cubic(params),
        
        // === FÍSICA ===
        "physics::projectile" => physics_projectile(params),
        "physics::nbody_2" => physics_nbody_2(params),
        
        // === ESTADÍSTICAS ===
        "stats::mean" => stats_mean(params),
        "stats::median" => stats_median(params),
        
        // === COMANDOS DEL SISTEMA ===
        "system::version" => json!("v0.7.2.0"),
        "system::ping" => json!("pong"),
        
        // === ERROR ===
        _ => json!({"error": format!("Unknown method: {}", method)}),
    };
    
    json!({
        "jsonrpc": "2.0",
        "result": result,
        "id": id
    })
}
```

---

### **Paso 3: Funciones de Ejemplo**

```rust
// crates/rydit-rs/src/main.rs

fn bezier_linear(params: &Vec<Value>) -> Value {
    if params.len() != 5 {
        return json!({"error": "bezier::linear requires 5 params"});
    }
    
    let p0_x = params[0].as_f64().unwrap_or(0.0);
    let p0_y = params[1].as_f64().unwrap_or(0.0);
    let p1_x = params[2].as_f64().unwrap_or(0.0);
    let p1_y = params[3].as_f64().unwrap_or(0.0);
    let t = params[4].as_f64().unwrap_or(0.0).max(0.0).min(1.0);
    
    let x = (1.0 - t) * p0_x + t * p1_x;
    let y = (1.0 - t) * p0_y + t * p1_y;
    
    json!([x, y])
}

fn bezier_cubic(params: &Vec<Value>) -> Value {
    if params.len() != 9 {
        return json!({"error": "bezier::cubic requires 9 params"});
    }
    
    let p0_x = params[0].as_f64().unwrap_or(0.0);
    let p0_y = params[1].as_f64().unwrap_or(0.0);
    let p1_x = params[2].as_f64().unwrap_or(0.0);
    let p1_y = params[3].as_f64().unwrap_or(0.0);
    let p2_x = params[4].as_f64().unwrap_or(0.0);
    let p2_y = params[5].as_f64().unwrap_or(0.0);
    let p3_x = params[6].as_f64().unwrap_or(0.0);
    let p3_y = params[7].as_f64().unwrap_or(0.0);
    let t = params[8].as_f64().unwrap_or(0.0).max(0.0).min(1.0);
    
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let t2 = t * t;
    
    let x = mt2 * mt * p0_x + 3.0 * mt2 * t * p1_x + 3.0 * mt * t2 * p2_x + t2 * t * p3_x;
    let y = mt2 * mt * p0_y + 3.0 * mt2 * t * p1_y + 3.0 * mt * t2 * p2_y + t2 * t * p3_y;
    
    json!([x, y])
}

fn stats_mean(params: &Vec<Value>) -> Value {
    if params.len() != 1 {
        return json!({"error": "stats::mean requires 1 param"});
    }
    
    let arr = params[0].as_array().unwrap_or(&vec![]);
    let sum: f64 = arr.iter()
        .filter_map(|v| v.as_f64())
        .sum();
    
    if arr.is_empty() {
        return json!({"error": "Empty array"});
    }
    
    json!(sum / arr.len() as f64)
}
```

---

## 🎯 USO DESDE PYTHON

### **Script Python:**

```python
#!/usr/bin/env python3
# ry_lazo.py - Puente universal

import subprocess
import json

class RyLazo:
    def __init__(self):
        # Iniciar rydit-rs en modo LAZOS
        self.proc = subprocess.Popen(
            ["./target/release/rydit-rs", "--lazos"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            text=True,
            bufsize=1
        )
    
    def call(self, method, params=[]):
        """Llamar función de RyDit"""
        request = {
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        }
        
        # Enviar request
        self.proc.stdin.write(json.dumps(request) + "\n")
        self.proc.stdin.flush()
        
        # Recibir response
        response = json.loads(self.proc.stdout.readline())
        
        if "error" in response:
            raise Exception(response["error"])
        
        return response["result"]
    
    def close(self):
        self.proc.terminate()

# === EJEMPLO DE USO ===
if __name__ == "__main__":
    ry = RyLazo()
    
    # Usar Bezier desde Python
    print("=== BEZIER CÚBICA ===")
    punto = ry.call("science::bezier::cubic", 
        [0, 0, 30, 100, 70, 100, 100, 0, 0.5])
    print(f"t=0.5: {punto}")  # [50, 75]
    
    # Generar 10 puntos
    print("\n=== 10 PUNTOS ===")
    for i in range(10):
        t = i / 9.0
        p = ry.call("science::bezier::cubic",
            [0, 0, 30, 100, 70, 100, 100, 0, t])
        print(f"t={t:.2f}: {p}")
    
    # Estadísticas
    print("\n=== ESTADÍSTICAS ===")
    datos = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    media = ry.call("stats::mean", [datos])
    print(f"Media de {datos}: {media}")
    
    ry.close()
```

---

## 🚀 VENTAJAS DE ESTA APROXIMACIÓN

### **1. Simple:**
- ✅ Todo en `main.rs`
- ✅ Sin crates externos
- ✅ Sin traits complejos
- ✅ ~200 líneas de código

### **2. Nativo:**
- ✅ Parte del binario
- ✅ Sin overhead de IPC complejo
- ✅ stdin/stdout estándar

### **3. Universal:**
- ✅ Cualquier lenguaje puede usarlo
- ✅ Python, Node.js, C, Bash
- ✅ Solo necesita JSON + stdin/stdout

### **4. Seguro:**
- ✅ Sin sockets de red (por defecto)
- ✅ Solo stdin/stdout local
- ✅ Sin exposición a red

### **5. Rust:**
- ✅ Type safety en Rust
- ✅ Error handling robust
- ✅ Performance nativo

---

## 🔒 SEGURIDAD

### **No es vulnerable porque:**

1. **Sin red**: Solo stdin/stdout local
2. **Sin filesystem**: No lee/escribe archivos directamente
3. **Sin system calls**: No ejecuta comandos del SO
4. **Sandbox natural**: Solo funciones exponidas

### **Si quieres red (opcional):**

```rust
// Opcional: modo --lazos-net con TCP
if args.contains(&"--lazos-net".to_string()) {
    lazos_network_loop(8080);  // Puerto configurable
}
```

---

## 📊 COMPARATIVA

| Criterio | RyditModule | LAZOS Nativo |
|----------|-------------|--------------|
| **Líneas de código** | ~500 | ~200 |
| **Complejidad** | Alta (trait, generics) | Baja (JSON, match) |
| **Binario** | Requiere lib+bin | Funciona con binario |
| **Lenguajes** | Solo Rust | Cualquiera |
| **Performance** | Alta (compile-time) | Media (runtime JSON) |
| **Seguridad** | Alta | Alta (sin red) |
| **Universal** | ❌ No | ✅ Sí |

**Ganador**: **LAZOS Nativo** ✅

---

## 🎯 PRÓXIMOS PASOS

### **v0.7.2.0 - Implementación:**

**Día 1**: 
- Agregar flag `--lazos` en CLI
- Implementar `lazos_loop()` en main.rs
- Test: `echo '{"method":"system::ping"}' | rydit-rs --lazos`

**Día 2**:
- Agregar 10+ comandos (Bezier, Física, Stats)
- Test desde Python
- Documentación

**Día 3**:
- Contenedores, Mundos, Actores
- Demo completa

---

## 💬 CONCLUSIÓN

**Esta es la esencia de LAZOS:**

> No es un trait complejo. No es una librería externa.
> 
> Es **Rust nativo**, simple, directo en `main.rs`.
> 
> **Cualquiera** puede usarlo con solo saber JSON.
> 
> **Universal**. **Simple**. **Nuestro**.

---

<div align="center">

**🔗 PROTOCOLO LAZOS - Nativo en main.rs**

*~200 líneas. Universal. Sin miedo al éxito.*

</div>
