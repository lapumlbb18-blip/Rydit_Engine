# ry-anim

**Animation module for Ry-Dit - 12 Disney Principles + Visual Effects + Science Animations**

[![Crates.io](https://img.shields.io/crates/v/ry-anim.svg)](https://crates.io/crates/ry-anim)
[![Documentation](https://docs.rs/ry-anim/badge.svg)](https://docs.rs/ry-anim)
[![License](https://img.shields.io/crates/l/ry-anim.svg)](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE)
[![Ry-Dit](https://img.shields.io/badge/Ry--Dit-v0.16.0-blue)](https://github.com/lapumlbb18-blip/Ry-dit)

## Overview

`ry-anim` is a comprehensive animation library implementing the **12 principles of Disney animation**, visual effects (bloom, chromatic aberration, motion blur), optical illusions, science-based animations, and sprite animation systems.

Designed for game engines and creative coding, it provides **58+ tested functions** covering easing, squash & stretch, anticipation, follow-through, arcs, secondary action, timing, exaggeration, and more.

## Features

### 🎨 12 Disney Animation Principles

| Principle | Function | Description |
|-----------|----------|-------------|
| **Squash & Stretch** | `squash()`, `stretch()` | Deformation while maintaining volume |
| **Anticipation** | `anticipate()` | Wind-up before main action |
| **Follow Through** | `follow_through()` | Parts continue moving after stop |
| **Overlapping Action** | `overlapping_action()` | Different parts at different speeds |
| **Slow In & Slow Out** | `ease_in()`, `ease_out()`, `ease_in_out()` | Easing functions |
| **Arcs** | `arc_path()` | Curved trajectories |
| **Secondary Action** | `secondary_action()` | Supporting movements |
| **Timing** | `timing()` | Keyframe interpolation |
| **Exaggeration** | `exaggerate()` | Amplify motion for appeal |
| **Solid Drawing** | `solid_rotation()` | 3D perspective awareness |
| **Appeal** | `appeal()` | Make shapes more attractive |
| **Pose-to-Pose** | `pose_to_pose()` | Interpolation between key poses |

### ✨ Visual Effects

- **Bloom** - Diffuse glow on bright areas
- **Chromatic Aberration** - RGB channel separation
- **Motion Blur** - Directional blur for movement
- **Neon Glow** - Configurable neon effect
- **Particle Trails** - Trailing particle effects
- **Morph Shapes** - Smooth shape transitions

### 🔬 Science Animations

- **Wave Interference** - Physics wave patterns
- **Pendulum Waves** - Synchronized pendulum motion
- **Cell Division** - Biological mitosis animation
- **Chemical Crystallization** - Crystal formation
- **L-System Trees** - Procedural plant growth
- **Tusi Couple** - Historical mathematical mechanism
- **Walk Cycle** - Fauna walking animation
- **Flight Pattern** - Bird flapping animation

### 🎭 Optical Illusions

- **Rotating Snakes** - Circular motion illusion
- **Café Wall** - Parallel lines appear tilted
- **Troxler Fading** - Disappearance by fixation
- **Pulsing Star** - Pulsating star effect
- **Zöllner Effect** - Non-parallel line illusion
- **Motion-Induced Blindness** - Disappearing dots

### 🎬 Sprite Animation System

- **Frame Animation** - Frame-by-frame sprite playback
- **Sprite Sheet Parser** - Parse sprite sheet images
- **Animation State Machine** - Manage animation states
- **Animation Blending** - Smooth transitions between states
- **Sprite Events** - Trigger events at specific frames
- **Sprite Flip** - Horizontal/vertical sprite flipping

## Installation

```toml
[dependencies]
ry-anim = "0.7.34"
ry-core = "0.8.2"
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
```

## Quick Start

### Easing Functions

```rust
use ry_anim::AnimModule;
use ry_core::RyditModule;
use serde_json::json;

let anim = AnimModule;

// Quadratic ease in (slow start)
let eased = anim.execute("ease_in", json!([0.5]))?;
assert_eq!(eased.as_f64().unwrap(), 0.25); // 0.5²

// Quadratic ease out (slow end)
let eased = anim.execute("ease_out", json!([0.5]))?;
assert_eq!(eased.as_f64().unwrap(), 0.75); // 0.5 * (2.0 - 0.5)

// Ease in-out (S-curve)
let eased = anim.execute("ease_in_out", json!([0.5]))?;
assert_eq!(eased.as_f64().unwrap(), 0.5);
```

### Squash & Stretch

```rust
// Impact squash (compress vertically, expand horizontally)
let squash = anim.execute("squash", json!([2.0]))?;
// Returns: [2.0, 0.5] - 2x wide, 0.5x tall

// Stretch (expand vertically, compress horizontally)
let stretch = anim.execute("stretch", json!([1.5]))?;
// Returns: [0.67, 1.5] - 0.67x wide, 1.5x tall
```

### Anticipation

```rust
// Move from 100 to 200 with 20 units of anticipation
let anticipate = anim.execute("anticipate", json!([100.0, 200.0, 20.0]))?;
// Returns: 80.0 (moves back 20 before going forward)
```

### Arc Path

```rust
// Curved trajectory from (0, 0) to (10, 0) with curvature 5
let arc = anim.execute("arc_path", json!([0.0, 0.0, 10.0, 0.0, 5.0, 0.5]))?;
// Returns: [x, y] position at t=0.5 on the arc
```

### Keyframe Timing

```rust
// Interpolate between keyframes: [(frame, value), ...]
let keyframes = json!([[0.0, 0.0], [30.0, 100.0], [60.0, 0.0]]);
let value = anim.execute("timing", json!([keyframes, 15.0]))?;
// Returns: 50.0 (halfway to 100 at frame 15)
```

### Sprite Animation

```rust
// Frame-by-frame animation
let frame = anim.execute("frame_animation", json!([0, 10, 5]))?;
// Current frame: 5 of 10 total frames

// Flip sprite horizontally
let flipped = anim.execute("sprite_flip", json!(["horizontal"]))?;
// Returns flip direction
```

## API Reference

### Core Module

All functions are accessed through the `AnimModule` struct implementing `RyditModule`:

```rust
pub struct AnimModule;

impl RyditModule for AnimModule {
    fn name(&self) -> &'static str { "anim" }
    fn version(&self) -> &'static str { "0.12.0" }
    fn register(&self) -> HashMap<&'static str, &'static str>;
    fn execute(&self, command: &str, params: Value) -> ModuleResult;
}
```

### Available Commands

#### Easing
| Command | Parameters | Returns | Description |
|---------|------------|---------|-------------|
| `ease_in` | `[t: f64]` | `f64` | Quadratic ease in: t² |
| `ease_out` | `[t: f64]` | `f64` | Quadratic ease out: t(2-t) |
| `ease_in_out` | `[t: f64]` | `f64` | Quadratic ease in-out |

#### Deformation
| Command | Parameters | Returns | Description |
|---------|------------|---------|-------------|
| `squash` | `[factor: f64]` | `[f64, f64]` | [scale_x, scale_y] for squash |
| `stretch` | `[factor: f64]` | `[f64, f64]` | [scale_x, scale_y] for stretch |

#### Motion
| Command | Parameters | Returns | Description |
|---------|------------|---------|-------------|
| `anticipate` | `[pos, target, amount]` | `f64` | Anticipated position |
| `follow_through` | `[amp, decay, freq, t]` | `f64` | Follow-through oscillation |
| `overlapping_action` | `[base, offsets, t]` | `f64` | Layered movement |
| `arc_path` | `[sx, sy, ex, ey, curvature, t]` | `[f64, f64]` | Position on arc |
| `secondary_action` | `[primary, offset, amp, t]` | `[f64, f64]` | [primary, secondary] |
| `timing` | `[keyframes, frame]` | `f64` | Interpolated value |
| `exaggerate` | `[base, factor, t]` | `f64` | Exaggerated value |
| `solid_rotation` | `[x, y, z, angle]` | `[f64; 3]` | 3D rotation |
| `appeal` | `[shape, factor]` | `Value` | Enhanced shape |
| `pose_to_pose` | `[pose1, pose2, t]` | `Value` | Interpolated pose |

#### Visual Effects
| Command | Parameters | Returns |
|---------|------------|---------|
| `neon_glow` | `[color, intensity, spread]` | `Value` |
| `motion_blur` | `[direction, strength]` | `Value` |
| `chromatic_aberration` | `[offset]` | `Value` |
| `bloom_effect` | `[threshold, intensity]` | `Value` |
| `particle_trails` | `[count, length, color]` | `Value` |
| `morph_shapes` | `[shape1, shape2, t]` | `Value` |

#### Science
| Command | Parameters | Returns |
|---------|------------|---------|
| `wave_interference` | `[sources, resolution]` | `Value` |
| `pendulum_waves` | `[count, duration]` | `Value` |
| `cell_division` | `[stage]` | `Value` |
| `chemical_crystallization` | `[progress]` | `Value` |
| `lsystem_tree` | `[iterations, angle]` | `Value` |
| `tusi_couple` | `[radius, angle]` | `Value` |
| `walk_cycle` | `[phase, speed]` | `Value` |
| `flight_pattern` | `[wing_angle, speed]` | `Value` |

#### Optical Illusions
| Command | Parameters | Returns |
|---------|------------|---------|
| `rotating_snakes` | `[count, radius, t]` | `Value` |
| `cafe_wall` | `[rows, cols, offset]` | `Value` |
| `troxler_fading` | `[fixation, elements]` | `Value` |
| `pulsing_star` | `[size, phase]` | `Value` |
| `zollner_effect` | `[lines, hatch_angle]` | `Value` |
| `motion_blindness` | `[targets, t]` | `Value` |

#### Sprites
| Command | Parameters | Returns |
|---------|------------|---------|
| `frame_animation` | `[total, current, speed]` | `Value` |
| `sprite_sheet_parse` | `[path, cols, rows]` | `Value` |
| `animation_state` | `[state, params]` | `Value` |
| `animation_blend` | `[from, to, t]` | `Value` |
| `sprite_events` | `[frame, events]` | `Value` |
| `sprite_flip` | `[direction]` | `Value` |

## Performance

- **Zero allocations** in easing functions (pure math)
- **JSON-based** command interface for scripting flexibility
- **58 unit tests** ensuring correctness across all functions
- **2900+ lines** of production-ready animation code

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `ry-core` | 0.8.2 | Module trait system |
| `serde_json` | 1.0 | JSON serialization |
| `serde` | 1.0 | Derive macros |

## Examples

### Ball Bounce Animation

```rust
use ry_anim::AnimModule;
use ry_core::RyditModule;
use serde_json::json;

let anim = AnimModule;

// Simulate ball falling and bouncing
let mut y = 0.0;
let mut velocity = 0.0;
let gravity = 9.8;

for frame in 0..120 {
    let t = frame as f64 / 60.0;
    
    // Apply gravity with ease
    velocity += anim.execute("ease_in", json!([gravity * 0.016]))?
        .as_f64().unwrap();
    y += velocity;
    
    // Bounce with squash
    if y >= 100.0 {
        y = 100.0;
        velocity *= -0.7; // Damping
        let squash = anim.execute("squash", json!([1.3]))?;
        println!("Bounce! Squash factor: {:?}", squash);
    }
}
```

### Smooth Camera Follow

```rust
// Camera follows player with anticipation and arc motion
let camera_pos = 50.0;
let player_pos = 150.0;

// Anticipate movement
let anticipate = anim.execute("anticipate", 
    json!([camera_pos, player_pos, 10.0]))?;

// Smooth arc path
let arc = anim.execute("arc_path", 
    json!([camera_pos, 0.0, player_pos, 0.0, 20.0, 0.5]))?;

println!("Camera moves from {} along arc {:?}", camera_pos, arc);
```

### Walk Cycle Animation

```rust
// 4-phase walk cycle
for phase in 0..100 {
    let t = phase as f64 / 100.0;
    let walk = anim.execute("walk_cycle", json!([t, 1.0]))?;
    // Apply walk animation to character
}
```

## Roadmap

- [ ] Cubic and quartic easing curves
- [ ] 3D bone animation system
- [ ] Inverse kinematics
- [ ] Morph targets
- [ ] Animation curves editor
- [ ] GPU-accelerated particle systems

## Contributing

Contributions are welcome! This crate is part of the **Ry-Dit** game engine project.

- **Repository**: https://github.com/lapumlbb18-blip/Ry-dit
- **Issues**: https://github.com/lapumlbb18-blip/Ry-dit/issues
- **Pull Requests**: Welcome!

Please read [CONTRIBUTING.md](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/CONTRIBUTING.md) for guidelines.

## License

MIT License - See [LICENSE](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE) for details.

## Related Crates

| Crate | Description |
|-------|-------------|
| [ry-core](https://crates.io/crates/ry-core) | Core traits and module system |
| [ry-gfx](https://crates.io/crates/ry-gfx) | Graphics layer with GPU instancing |
| [ry-physics](https://crates.io/crates/ry-physics) | Physics simulation |
| [ry-backend](https://crates.io/crates/ry-backend) | Dual backend (raylib + SDL2) |
| [migui](https://crates.io/crates/migui) | Immediate mode GUI |

---

<div align="center">

**ry-anim** - Bringing Disney magic to Rust game development 🎬✨

*58 tests · 2900+ lines · 12 Disney principles · 6 optical illusions · 8 science animations*

</div>
