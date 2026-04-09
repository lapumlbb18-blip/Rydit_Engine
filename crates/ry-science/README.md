# ry-science

**Science module for Ry-Dit — Bezier curves, Statistics, Geometry, Optical illusions**

[![Crates.io](https://img.shields.io/crates/v/ry-science.svg)](https://crates.io/crates/ry-science)
[![Documentation](https://docs.rs/ry-science/badge.svg)](https://docs.rs/ry-science)
[![License](https://img.shields.io/crates/l/ry-science.svg)](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE)

## Overview

`ry-science` is a mathematics and geometry library for the Ry-Dit game engine. It provides Bezier curve calculations, statistical functions, and geometric optical illusion generators — all accessible via a clean JSON-based API.

## Features

### 📈 Bezier Curves
- **Linear** — Interpolation between 2 points: `P(t) = (1-t)·P0 + t·P1`
- **Quadratic** — 2 points + 1 control point
- **Cubic** — 2 points + 2 control points (industry standard for animation)

### 📊 Statistics
- **Mean** — Arithmetic average
- **Median** — Middle value of sorted array
- **Min / Max** — Extremes of dataset

### 🔷 Geometry (Optical Illusions)
- **Penrose Triangle** — Impossible tribar
- **Impossible Cube** — Necker cube variant
- **Archimedean Spiral** — Parametric spiral points
- **Müller-Lyer** — Arrow length illusion
- **Ponzo** — Perspective length illusion

## Installation

```toml
[dependencies]
ry-science = "0.7.34"
ry-core = "0.8.2"
serde_json = "1.0"
```

## Quick Start

```rust
use ry_science::ScienceModule;
use ry_core::RyditModule;
use serde_json::json;

let module = ScienceModule;

// Bezier cubic curve
let point = module.execute("bezier::cubic", json!([
    0.0, 0.0,    // P0
    30.0, 100.0, // P1 (control)
    70.0, 100.0, // P2 (control)
    100.0, 0.0,  // P3
    0.5          // t
]))?;
// Returns: [50.0, 75.0]

// Statistics
let mean = module.execute("stats::mean", json!([1.0, 2.0, 3.0, 4.0, 5.0]))?;
// Returns: 3.0

let median = module.execute("stats::median", json!([1.0, 2.0, 3.0, 4.0]))?;
// Returns: 2.5

// Geometry - Penrose triangle coordinates
let lines = module.execute("geometry::penrose", json!([400.0, 300.0, 100.0]))?;
// Returns: [[x1,y1,x2,y2], ...] — lines to draw
```

## API Reference

### Bezier Curves

#### Linear (2 points)
```rust
module.execute("bezier::linear", json!([p0_x, p0_y, p1_x, p1_y, t]))
```
**Formula**: `P(t) = (1-t)·P0 + t·P1`

#### Quadratic (3 points)
```rust
module.execute("bezier::quadratic", json!([p0_x, p0_y, p1_x, p1_y, p2_x, p2_y, t]))
```
**Formula**: `P(t) = (1-t)²·P0 + 2(1-t)·t·P1 + t²·P2`

#### Cubic (4 points)
```rust
module.execute("bezier::cubic", json!([p0_x, p0_y, p1_x, p1_y, p2_x, p2_y, p3_x, p3_y, t]))
```
**Formula**: `P(t) = (1-t)³·P0 + 3(1-t)²·t·P1 + 3(1-t)·t²·P2 + t³·P3`

| Parameter | Type | Description |
|-----------|------|-------------|
| `p0..pN` | f64 | Control point coordinates (x, y pairs) |
| `t` | f64 | Parameter (0.0–1.0, clamped automatically) |

**Returns**: `[x, y]` — point on curve at parameter `t`

### Statistics

```rust
module.execute("stats::mean", json!([1.0, 2.0, 3.0]))     // Returns: 2.0
module.execute("stats::median", json!([1.0, 2.0, 3.0, 4.0])) // Returns: 2.5
module.execute("stats::min", json!([3.0, 1.0, 4.0]))     // Returns: 1.0
module.execute("stats::max", json!([3.0, 1.0, 4.0]))     // Returns: 4.0
```

| Function | Input | Output |
|----------|-------|--------|
| `stats::mean` | `[f64, ...]` | `f64` average |
| `stats::median` | `[f64, ...]` | `f64` middle value |
| `stats::min` | `[f64, ...]` | `f64` minimum |
| `stats::max` | `[f64, ...]` | `f64` maximum |

### Geometry (Optical Illusions)

#### Penrose Triangle
```rust
module.execute("geometry::penrose", json!([center_x, center_y, size]))
```
Returns: `[[x1,y1,x2,y2], ...]` — 12 lines forming the impossible triangle.

#### Impossible Cube
```rust
module.execute("geometry::impossible_cube", json!([center_x, center_y, size]))
```
Returns: 14 lines forming an ambiguous/Necker cube.

#### Archimedean Spiral
```rust
module.execute("geometry::spiral", json!([center_x, center_y, turns, radius, points_per_turn]))
```
Returns: `[[x, y], ...]` — parametric spiral points.

#### Müller-Lyer Illusion
```rust
module.execute("geometry::muller_lyer", json!([center_x, center_y, length]))
```
Returns: 10 lines forming the classic arrow illusion.

#### Ponzo Illusion
```rust
module.execute("geometry::ponzo", json!([center_x, center_y, height, width_top, width_bottom]))
```
Returns: 6 lines forming the perspective illusion.

## Examples

### Smooth Animation Path

```rust
// Create a smooth animation path using cubic Bezier
let control_points = [0.0, 0.0, 50.0, 200.0, 150.0, 200.0, 200.0, 0.0];

for frame in 0..60 {
    let t = frame as f64 / 60.0;
    let mut params = control_points.to_vec();
    params.push(t);

    let point = ScienceModule.execute("bezier::cubic", json!(params)).unwrap();
    let coords = point.as_array().unwrap();
    let x = coords[0].as_f64().unwrap();
    let y = coords[1].as_f64().unwrap();

    println!("Frame {}: position ({}, {})", frame, x, y);
}
```

### Data Analysis

```rust
// Analyze game scores
let scores = json!([150.0, 220.0, 180.0, 310.0, 275.0]);

let mean = ScienceModule.execute("stats::mean", scores.clone()).unwrap();
let median = ScienceModule.execute("stats::median", scores.clone()).unwrap();
let max = ScienceModule.execute("stats::max", scores).unwrap();

println!("Mean: {}, Median: {}, Best: {}", mean, median, max);
```

### Drawing Optical Illusions

```rust
// Draw a Penrose triangle using the returned coordinates
let lines = ScienceModule.execute("geometry::penrose", json!([400.0, 300.0, 150.0])).unwrap();

for line in lines.as_array().unwrap() {
    let coords = line.as_array().unwrap();
    let x1 = coords[0].as_f64().unwrap();
    let y1 = coords[1].as_f64().unwrap();
    let x2 = coords[2].as_f64().unwrap();
    let y2 = coords[3].as_f64().unwrap();

    // draw_line(x1, y1, x2, y2);
}
```

### LAZOS Protocol

```bash
# Bezier curve
echo '{"method":"science::bezier::cubic","params":[0,0,30,100,70,100,100,0,0.5]}' | rydit-rs --lazos

# Statistics
echo '{"method":"science::stats::mean","params":[1,2,3,4,5]}' | rydit-rs --lazos

# Geometry
echo '{"method":"science::geometry::penrose","params":[400,300,100]}' | rydit-rs --lazos
```

## Performance

- **O(n) Bezier** — Single evaluation, no iterations
- **O(n log n) Median** — Sort-based, efficient for typical datasets
- **Zero allocations** for statistical functions (single-pass where possible)
- **21 unit tests** ensuring correctness across all functions

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `ry-core` | 0.8.2 | Module trait system |
| `serde_json` | 1.0 | JSON serialization |
| `serde` | 1.0 | Derive macros |

## Roadmap

- [ ] Higher-order Bezier curves (N control points)
- [ ] Standard deviation and variance
- [ ] Percentile calculations
- [ ] More optical illusions (Kanizsa triangle, Hering illusion)
- [ ] Fractal generation (Mandelbrot, Koch snowflake)

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

**ry-science** — Math, stats, and geometry for Ry-Dit game engine 📐📊

*21 tests · 988 lines · Bezier + Stats + 5 optical illusions*

</div>
