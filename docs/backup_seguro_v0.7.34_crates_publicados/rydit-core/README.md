# rydit-core

**Core trait and registry for RyDit modules**

[![Crates.io](https://img.shields.io/crates/v/rydit-core.svg)](https://crates.io/crates/rydit-core)
[![Documentation](https://docs.rs/rydit-core/badge.svg)](https://docs.rs/rydit-core)
[![License](https://img.shields.io/crates/l/rydit-core.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine/blob/main/LICENSE)

## Overview

`rydit-core` provides the core trait (`RyditModule`) and registry system for the RyDit game engine. It allows creating modular, extensible game engine components.

## Installation

```toml
[dependencies]
rydit-core = "0.7.3"
```

## Usage

```rust
use rydit_core::{RyditModule, ModuleRegistry, ModuleResult};
use serde_json::json;

// Implement the trait for your module
struct MyModule;

impl RyditModule for MyModule {
    fn name(&self) -> &'static str { "my_module" }
    fn version(&self) -> &'static str { "1.0.0" }
    fn register(&self) -> std::collections::HashMap<&'static str, &'static str> {
        let mut cmds = std::collections::HashMap::new();
        cmds.insert("ping", "Ping command");
        cmds
    }
    fn execute(&self, command: &str, params: serde_json::Value) -> ModuleResult {
        match command {
            "ping" => Ok(json!({"status": "pong"})),
            _ => Err(rydit_core::ModuleError {
                code: "UNKNOWN".to_string(),
                message: format!("Unknown command: {}", command),
            }),
        }
    }
}

// Register and use modules
let mut registry = ModuleRegistry::new();
registry.register(Box::new(MyModule));

let result = registry.execute("my_module", "ping", json!([]));
```

## Features

- **RyditModule trait**: Define module interface
- **ModuleRegistry**: Register and manage multiple modules
- **ModuleResult/ModuleError**: Standardized error handling
- **Send + Sync**: Thread-safe modules

## License

MIT License - See [LICENSE](https://github.com/lapumlbb18-blip/Rydit_Engine/blob/main/LICENSE) for details.

## Contributing

Contributions are welcome! Please open an issue or submit a PR at:
https://github.com/lapumlbb18-blip/Rydit_Engine
