# 🛡️ V-Shield - Platform Layer + Sync Primitives

**Platform detection + synchronization primitives for the Ry-Dit game engine.**

[![Version](https://img.shields.io/badge/version-v0.2.0-blue.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Tests](https://img.shields.io/badge/tests-26%20passing-green.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE)

---

## 📦 Qué incluye

| Módulo | Descripción |
|--------|-------------|
| **`platform`** | Detección automática de OS (Linux, Windows, macOS, Android, iOS, WASM) |
| **`sync`** | Primitivas de sincronización (Mutex, RwLock, Barrier, Condvar) |
| **`platform_sync`** | Sincronización de renderizado (X11, OpenGL, Auto) |
| **`graphics`** | Colores RyDit + init de ventana (feature `graphics`, usa raylib) |

## 🚀 Quick Start

```rust
use v_shield::sync::{Mutex, RwLock};
use v_shield::platform::{current_platform, PlatformConfig};
use v_shield::platform_sync::PlatformSync;

// Platform detection
let p = current_platform();
println!("Running on: {} ({})", p.name(), p.arch());

// Config defaults por plataforma
let config = PlatformConfig::for_current();

// Sync primitives (thread-safe)
let data = Mutex::new(vec![1, 2, 3]);
let cache = RwLock::new(String::new());

// Platform sync para renderizado (al final de cada frame)
let mut sync = PlatformSync::new();
// sync.sync();
```

## 🔧 Features

| Feature | Descripción | Dependencias extra | Tamaño |
|---------|-------------|-------------------|--------|
| `native` (default) | `std::sync` para Linux/Windows/macOS | Ninguna | 0 KB |
| `wasm` | Fallback para WASM (Barrier, Condvar) | Ninguna | 0 KB |
| `graphics` (default) | Colores raylib + `init_window()` | raylib | ~50 KB |
| `async-tokio` | Wrappers async (`tokio::sync`) | tokio (solo `sync`) | ~80 KB |
| `rt-linux` | Linux real-time (pendiente) | rtsc | ~20 KB |

### Uso sin graphics (más liviano)

```toml
[dependencies]
v-shield = { version = "0.2", default-features = false, features = ["native"] }
```

### Uso con async

```toml
[dependencies]
v-shield = { version = "0.2", features = ["native", "async-tokio"] }
```

```rust
use v_shield::sync::async_wrappers::{AsyncMutex, AsyncRwLock};
```

## 📱 Para Termux/Android

Funciona sin cambios en Termux. Dependencias de sistema necesarias:

```bash
pkg install clang libx11-dev libxcb-dev
```

## 🏗️ Arquitectura

```
v-shield/
├── src/
│   ├── lib.rs              # Public API + re-exports
│   ├── platform/
│   │   └── mod.rs          # Platform detection + PlatformConfig
│   ├── platform_sync.rs    # Render sync (X11/OpenGL)
│   └── sync/
│       ├── mod.rs          # Sync module + Barrier/Condvar fallbacks
│       ├── mutex.rs        # Mutex wrapper (std or tokio)
│       └── rwlock.rs       # RwLock wrapper (std or tokio)
```

## 📊 Tests

```bash
cargo test -p v-shield
# 22 unit tests + 4 doc tests = 26 passing ✅
```

## 🔄 Integración con otros crates

### ry-gfx (graphics)
```rust
use ry_gfx::render_queue::{PlatformSync, RenderQueue, DrawCommand};
// PlatformSync viene de v-shield (re-export)
```

### ry-stream (networking)
```rust
use v_shield::sync::Mutex;  // Thread-safe Mutex para clientes
use ry_stream::server::StreamServer;
```

## 🎯 Filosofía

**Low-End First**: Funciona en un Redmi Note 8 con Adreno 610.
- Sin features pesados por defecto
- `tokio` solo con feature `sync` (~80KB), NO runtime full
- `raylib` es opcional (feature `graphics`)
- Zero dependencias extra en modo `native`

## 📝 Changelog

### v0.2.0 (actual)
- ✅ Platform detection (Linux, Windows, macOS, Android, iOS, WASM)
- ✅ Sync primitives (Mutex, RwLock, Barrier, Condvar)
- ✅ Platform Sync migrado de ry-gfx (ahora centralizado)
- ✅ Features configurables (native, wasm, graphics, async-tokio)
- ✅ 26 tests pasando

### v0.1.0
- Colores raylib + ColorRyDit enum
- `init_window()` wrapper
