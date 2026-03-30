# rydit-anim

**Animation module for RyDit - Easing functions, Squash & Stretch, Disney principles**

[![Crates.io](https://img.shields.io/crates/v/rydit-anim.svg)](https://crates.io/crates/rydit-anim)
[![Documentation](https://docs.rs/rydit-anim/badge.svg)](https://docs.rs/rydit-anim)
[![License](https://img.shields.io/crates/l/rydit-anim.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine/blob/main/LICENSE)

## Overview

`rydit-anim` provides animation functions for the RyDit game engine, implementing easing functions and classic animation principles.

## Installation

```toml
[dependencies]
rydit-anim = "0.7.3"
```

## Usage

```rust
use rydit_anim::AnimModule;
use rydit_core::RyditModule;
use serde_json::json;

let module = AnimModule;

// Easing functions
let ease_in = module.execute("ease_in", json!([0.5]));
// Returns: 0.25 (quadratic ease in)

let ease_out = module.execute("ease_out", json!([0.5]));
// Returns: 0.75 (quadratic ease out)

// Squash & Stretch
let squash = module.execute("squash", json!([2.0]));
// Returns: scale factors for squash effect

// Anticipation
let anticipate = module.execute("anticipate", json!([100.0, 200.0, 20.0]));
// Returns: anticipation movement value
```

## Features

### Easing Functions
- `anim::ease_in` - Quadratic ease in (slow start)
- `anim::ease_out` - Quadratic ease out (slow end)
- `anim::ease_in_out` - Quadratic ease in-out (slow start and end)

### Squash & Stretch
- `anim::squash` - Squash effect for impact
  - Parameters: `factor` (e.g., 2.0 for 2x squash)
  - Returns: scale factors [scale_x, scale_y]

### Anticipation
- `anim::anticipate` - Anticipation movement
  - Parameters: `start, end, anticipation_amount`
  - Returns: anticipated starting position

## LAZOS Protocol

Use with RyDit's LAZOS protocol:

```bash
# Easing
echo '{"method":"anim::ease_in","params":[0.5]}' | rydit-rs --lazos

# Squash & Stretch
echo '{"method":"anim::squash","params":[2.0]}' | rydit-rs --lazos

# Anticipation
echo '{"method":"anim::anticipate","params":[100,200,20]}' | rydit-rs --lazos
```

## Examples

### Smooth Animation with Easing
```rust
// Animate from 0 to 100 over 60 frames
for frame in 0..60 {
    let t = frame as f64 / 60.0;
    let eased = module.execute("ease_in_out", json!([t])).unwrap();
    let value = eased.as_f64().unwrap() * 100.0;
    // Use value for animation
}
```

### Ball Bounce with Squash
```rust
// When ball hits ground, apply squash
let squash_factor = 2.0;
let result = module.execute("squash", json!([squash_factor]));
// Apply returned scale factors to sprite
```

## License

MIT License - See [LICENSE](https://github.com/lapumlbb18-blip/Rydit_Engine/blob/main/LICENSE) for details.

## Contributing

Contributions are welcome! Please open an issue or submit a PR at:
https://github.com/lapumlbb18-blip/Rydit_Engine
