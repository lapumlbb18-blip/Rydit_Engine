# ry-physics

**Physics module for Ry-Dit — Projectile motion, Gravity, N-body simulation**

[![Crates.io](https://img.shields.io/crates/v/ry-physics.svg)](https://crates.io/crates/ry-physics)
[![Documentation](https://docs.rs/ry-physics/badge.svg)](https://docs.rs/ry-physics)
[![License](https://img.shields.io/crates/l/ry-physics.svg)](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE)

## Overview

`ry-physics` is a physics simulation library for the Ry-Dit game engine. It provides projectile motion calculations, gravitational N-body simulations, and more — all accessible via a clean JSON-based API.

## Installation

```toml
[dependencies]
ry-physics = "0.7.34"
ry-core = "0.8.2"
serde_json = "1.0"
```

## Quick Start

```rust
use ry_physics::PhysicsModule;
use ry_core::RyditModule;
use serde_json::json;

let module = PhysicsModule;

// Projectile motion: x0, y0, velocity (m/s), angle (degrees)
let result = module.execute("projectile", json!([0.0, 0.0, 50.0, 45.0]))?;
// Returns: [x_final, y_final, flight_time, max_height, range]

// N-body gravity: 2 bodies
let force = module.execute("nbody_2", json!([100.0, 200.0, 0.0, 0.0, 10.0, 0.0, 1.0]))?;
// Returns: [fx1, fy1, fx2, fy2, distance]

// N-body simulation: multiple bodies over time
let bodies = json!([
    [[100.0, 0.0, 0.0, 0.0, 0.0, 0.0],   // mass, x, y, vx, vy, is_static
     [100.0, 10.0, 0.0, 0.0, 0.0, 0.0],
     [50.0, 5.0, 5.0, 0.0, 0.0, 0.0]],
    0.016,   // dt (delta time)
    1.0      // G (gravitational constant)
]);
let result = module.execute("nbody_simulate", bodies)?;
// Returns: [[x, y, vx, vy], ...] updated positions
```

## API Reference

### Projectile Motion

```rust
module.execute("projectile", json!([x0, y0, velocity, angle_degrees]))
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `x0` | f64 | Initial X position |
| `y0` | f64 | Initial Y position |
| `velocity` | f64 | Initial velocity (m/s) |
| `angle_degrees` | f64 | Launch angle in degrees |

**Returns**: `[x_final, y_final, flight_time, max_height, range]`

**Formulas**:
- Flight time: `2 * vy / g`
- Max height: `vy² / (2 * g)`
- Range: `vx * flight_time`

### N-Body (2 Bodies)

```rust
module.execute("nbody_2", json!([m1, m2, x1, y1, x2, y2, G]))
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `m1`, `m2` | f64 | Masses of the two bodies |
| `x1`, `y1` | f64 | Position of body 1 |
| `x2`, `y2` | f64 | Position of body 2 |
| `G` | f64 | Gravitational constant (default: 6.674e-11) |

**Returns**: `[fx1, fy1, fx2, fy2, distance]`

**Formula**: `F = G * m1 * m2 / r²`

### N-Body Simulation (Multiple Bodies)

```rust
module.execute("nbody_simulate", json!([bodies, dt, G]))
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `bodies` | array | `[[mass, x, y, vx, vy, is_static], ...]` |
| `dt` | f64 | Delta time (default: 0.016 = ~60 FPS) |
| `G` | f64 | Gravitational constant (default: 6.674e-11) |

**Returns**: `[[x, y, vx, vy], ...]` — updated positions and velocities

**Algorithm**: O(n²) pairwise gravitational computation with Euler integration.

## Examples

### Ball Trajectory

```rust
// Kick a ball at 20 m/s, 30 degrees from ground level
let result = PhysicsModule.execute("projectile", json!([0.0, 0.0, 20.0, 30.0]));

let data = result.unwrap().as_array().unwrap();
let flight_time = data[2].as_f64().unwrap();
let max_height = data[3].as_f64().unwrap();
let range = data[4].as_f64().unwrap();

println!("Ball flies for {:.2}s, reaches {:.2}m high, lands {:.2}m away",
    flight_time, max_height, range);
```

### Orbital Simulation

```rust
// Sun + Earth + Moon simplified
let bodies = json!([
    // Sun: massive, static at center
    [1000.0, 0.0, 0.0, 0.0, 0.0, 1.0],
    // Earth: orbiting
    [10.0, 100.0, 0.0, 0.0, 3.0, 0.0],
    // Moon: orbiting Earth
    [1.0, 105.0, 0.0, 0.0, 5.0, 0.0],
]);

for step in 0..1000 {
    let result = PhysicsModule.execute("nbody_simulate", json!([bodies, 0.01, 1.0])).unwrap();
    println!("Step {}: {:?}", step, result);
}
```

### LAZOS Protocol

```bash
# Projectile
echo '{"method":"physics::projectile","params":[0,0,50,45]}' | rydit-rs --lazos

# N-body gravity
echo '{"method":"physics::nbody_2","params":[100,200,0,0,10,0,1]}' | rydit-rs --lazos

# N-body simulation
echo '{"method":"physics::nbody_simulate","params":[[[100,0,0,0,0,0],[100,10,0,0,0,0]],0.016,1]}' | rydit-rs --lazos
```

## Performance

- **O(n²) N-body**: Suitable for up to ~500 bodies in real-time
- **Zero allocations** in projectile calculation (pure math)
- **6 unit tests** ensuring correctness across all functions

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `ry-core` | 0.8.2 | Module trait system |
| `serde_json` | 1.0 | JSON serialization |
| `serde` | 1.0 | Derive macros |

## Roadmap

- [ ] RK4 integration (more accurate than Euler)
- [ ] Collision detection between bodies
- [ ] Soft-body physics
- [ ] Fluid dynamics (simplified)
- [ ] Electromagnetism simulation

## Contributing

Contributions are welcome! This crate is part of the **Ry-Dit** game engine project.

- **Repository**: https://github.com/lapumlbb18-blip/Ry-dit
- **Issues**: https://github.com/lapumlbb18-blip/Ry-dit/issues
- **Pull Requests**: Welcome!

Please read [CONTRIBUTING.md](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/CONTRIBUTING.md) for guidelines.

## License

MIT License - See [LICENSE](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE) for details.

---

<div align="center">

**ry-physics** — Physics simulations for Ry-Dit game engine 🚀🌍

*6 tests · 296 lines · Projectile + N-body gravity*

</div>
