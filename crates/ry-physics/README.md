# rydit-physics

**Physics module for RyDit - Projectile motion, Gravity, N-body simulation**

[![Crates.io](https://img.shields.io/crates/v/rydit-physics.svg)](https://crates.io/crates/rydit-physics)
[![Documentation](https://docs.rs/rydit-physics/badge.svg)](https://docs.rs/rydit-physics)
[![License](https://img.shields.io/crates/l/rydit-physics.svg)](https://github.com/lapumlbb18-blip/Ry-Dit/blob/main/LICENSE)

## Overview

`rydit-physics` provides physics simulations for the RyDit game engine, including projectile motion and gravitational N-body simulations.

## Installation

```toml
[dependencies]
rydit-physics = "0.7.3"
```

## Usage

```rust
use rydit_physics::PhysicsModule;
use rydit_core::RyditModule;
use serde_json::json;

let module = PhysicsModule;

// Projectile motion
let trajectory = module.execute("projectile", json!([0.0, 0.0, 50.0, 45.0]));
// Returns: {x, y, height, distance, time}

// N-body gravity (2 bodies)
let gravity = module.execute("nbody_2", json!([100.0, 200.0, 0.0, 0.0, 10.0, 0.0, 1.0]));
// Returns: gravitational force between two bodies
```

## Features

### Projectile Motion
- `physics::projectile` - Calculate projectile trajectory
  - Parameters: `x0, y0, velocity, angle_degrees`
  - Returns: trajectory data with height, distance, flight time

### N-Body Gravity
- `physics::nbody_2` - Two-body gravitational simulation
  - Parameters: `m1, m2, x1, y1, x2, y2, G`
  - Returns: gravitational force and acceleration

## LAZOS Protocol

Use with RyDit's LAZOS protocol:

```bash
# Projectile
echo '{"method":"physics::projectile","params":[0,0,50,45]}' | rydit-rs --lazos

# Gravity simulation
echo '{"method":"physics::nbody_2","params":[100,200,0,0,10,0,1]}' | rydit-rs --lazos
```

## Examples

### Projectile Trajectory
```rust
// Launch from (0,0) at 50 m/s, 45 degrees
let result = module.execute("projectile", json!([0.0, 0.0, 50.0, 45.0]));
```

### Gravitational Force
```rust
// Earth (m=100) and Moon (m=10) at distance 50
let result = module.execute("nbody_2", json!([100.0, 10.0, 0.0, 0.0, 50.0, 0.0, 1.0]));
```

## License

MIT License - See [LICENSE](https://github.com/lapumlbb18-blip/Ry-Dit/blob/main/LICENSE) for details.

## Contributing

Contributions are welcome! Please open an issue or submit a PR at:
https://github.com/lapumlbb18-blip/Ry-Dit
