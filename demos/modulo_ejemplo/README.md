# 🦀 Módulo de Ejemplo para Ry-Dit

**Demo de módulo dinámico cargable en runtime**

---

## 📖 Descripción

Este módulo demuestra cómo crear un módulo Rust que puede cargarse dinámicamente en el **Sistema Universal Ry v0.8.2+**.

**Características:**
- ✅ Carga dinámica (.so en Linux)
- ✅ Implementa trait `RyditModule`
- ✅ 6 comandos disponibles
- ✅ Metadata completa
- ✅ 8 tests automáticos

---

## 🚀 Comandos Disponibles

| Comando | Parámetros | Descripción | Ejemplo |
|---------|------------|-------------|---------|
| `saludar` | `[nombre]` | Saluda a una persona | `["Rust"]` |
| `despedir` | `[nombre]` | Despide a una persona | `["Amigo"]` |
| `sumar` | `[a, b]` | Suma dos números | `[5.0, 3.0]` |
| `multiplicar` | `[a, b]` | Multiplica dos números | `[4.0, 5.0]` |
| `pi` | `[]` | Retorna el valor de PI | `[]` |
| `cuadrado` | `[numero]` | Calcula el cuadrado | `[7.0]` |
| `info` | `[]` | Información del módulo | `[]` |

---

## 🔨 Compilación

### Requisitos
- Rust 1.70+
- rydit-core v0.8.2+

### Compilar
```bash
cd demos/modulo_ejemplo
cargo build --release
```

### Output
- **Linux**: `target/release/libmodulo_ejemplo.so` (532 KB)
- **Windows**: `target/release/modulo_ejemplo.dll`
- **macOS**: `target/release/libmodulo_ejemplo.dylib`

---

## 📦 Carga en RyDit

### Opción 1: Protocolo LAZOS

```bash
# Copiar el módulo al directorio de RyDit
cp target/release/libmodulo_ejemplo.so /path/to/shield-project/

# Iniciar RyDit en modo LAZOS
rydit-rs --lazos

# En otra terminal, enviar comandos:
echo '{"method":"module::list"}' | rydit-rs --lazos
echo '{"method":"module::info","params":["modulo_ejemplo"]}' | rydit-rs --lazos

# Ejecutar comandos del módulo
echo '{"method":"saludar","params":["Mundo"]}' | rydit-rs --lazos
echo '{"method":"sumar","params":[5, 3]}' | rydit-rs --lazos
echo '{"method":"pi","params":[]}' | rydit-rs --lazos
```

### Opción 2: Desde Rust

```rust
use rydit_loader::DynamicModuleLoader;

let mut loader = DynamicModuleLoader::new();

// Cargar módulo (Linux)
#[cfg(not(target_os = "android"))]
loader.load_library("libmodulo_ejemplo.so").unwrap();

// Listar módulos
for name in loader.list_modules() {
    println!("Módulo cargado: {}", name);
}

// Ejecutar comando
let registry = loader.registry();
let module = registry.get("modulo_ejemplo").unwrap();

let result = module.execute("saludar", json!(["Rust"])).unwrap();
println!("{}", result); // "¡Hola, Rust! ..."
```

---

## 🧪 Tests

```bash
cd demos/modulo_ejemplo
cargo test --release
```

**Tests:**
- ✅ test_modulo_nombre
- ✅ test_saludar
- ✅ test_sumar
- ✅ test_multiplicar
- ✅ test_pi
- ✅ test_cuadrado
- ✅ test_info
- ✅ test_register

**Resultado:** 8/8 passing ✅

---

## 📝 Código de Ejemplo

### Estructura del Módulo

```rust
use rydit_core::{RyditModule, ModuleMetadata, ModuleResult};
use serde_json::{json, Value};
use std::collections::HashMap;

pub struct MiModulo;

impl RyditModule for MiModulo {
    fn name(&self) -> &'static str { "mi_modulo" }
    fn version(&self) -> &'static str { "1.0.0" }
    
    fn register(&self) -> HashMap<&'static str, &'static str> {
        let mut cmds = HashMap::new();
        cmds.insert("mi_comando", "Descripción");
        cmds
    }
    
    fn execute(&self, command: &str, params: Value) -> ModuleResult {
        match command {
            "mi_comando" => Ok(json!("Resultado")),
            _ => Err(...)
        }
    }
    
    fn metadata(&self) -> ModuleMetadata {
        ModuleMetadata::new()
            .with_name("mi_modulo")
            .with_version("1.0.0")
            .with_description("Mi módulo personalizado")
    }
}

// Función exportada para carga dinámica
#[no_mangle]
pub extern "C" fn create_module() -> *mut dyn RyditModule {
    Box::into_raw(Box::new(MiModulo))
}
```

---

## 🎯 Casos de Uso

### 1. Módulo Matemático
```rust
// Comandos: math::sin, math::cos, math::tan, math::sqrt
echo '{"method":"math::sin","params":[1.57]}' | rydit-rs --lazos
```

### 2. Módulo de Utilidades
```rust
// Comandos: utils::hash, utils::encrypt, utils::compress
echo '{"method":"utils::hash","params":["texto"]}' | rydit-rs --lazos
```

### 3. Módulo de Juego
```rust
// Comandos: game::save, game::load, game::reset
echo '{"method":"game::save","params":["slot1"]}' | rydit-rs --lazos
```

---

## ⚠️ Notas Importantes

### Android
La carga dinámica de bibliotecas (.so) en Android requiere permisos especiales y puede no funcionar en todos los dispositivos. Para Android, se recomienda usar **ScriptModule** (scripts .rydit).

### FFI-Safe
El warning `extern fn uses type dyn RyditModule, which is not FFI-safe` es esperado. Los traits de Rust no tienen equivalente en C, pero funciona para nuestro caso de uso interno.

### Memory Safety
La función `create_module()` usa `Box::into_raw()` para transferir ownership. El loader es responsable de hacer drop correctamente cuando se descarga el módulo.

---

## 📊 Métricas

| Métrica | Valor |
|---------|-------|
| **Tamaño** | 532 KB |
| **Tests** | 8 passing |
| **Comandos** | 7 disponibles |
| **Build time** | ~50s |
| **Dependencias** | rydit-core, serde_json |

---

## 🔗 Referencias

- **rydit-core**: [crates/rydit-core](../../crates/rydit-core)
- **rydit-loader**: [crates/rydit-loader](../../crates/rydit-loader)
- **Documentación Sistema Universal Ry**: [SISTEMA_UNIVERSAL_RY_COMPLETADO.md](../../SISTEMA_UNIVERSAL_RY_COMPLETADO.md)

---

<div align="center">

**🛡️ Ry-Dit - Módulo de Ejemplo v1.0.0**

*Demo de carga dinámica con Sistema Universal Ry v0.8.2*

</div>
