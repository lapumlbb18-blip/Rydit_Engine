# rydit-science

**Science module for RyDit - Bezier curves, Statistics, Geometry, Optical illusions**

[![Crates.io](https://img.shields.io/crates/v/rydit-science.svg)](https://crates.io/crates/rydit-science)
[![Documentation](https://docs.rs/rydit-science/badge.svg)](https://docs.rs/rydit-science)
[![License](https://img.shields.io/crates/l/rydit-science.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine/blob/main/LICENSE)

## Overview

`rydit-science` provides mathematical and scientific functions for the RyDit game engine, including Bezier curves, statistics, and geometric optical illusions.

## Installation

```toml
[dependencies]
rydit-science = "0.7.3"
```

## Usage

```rust
use rydit_science::ScienceModule;
use rydit_core::RyditModule;
use serde_json::json;

let module = ScienceModule;

// Bezier curves
let result = module.execute("bezier::cubic", json!([0.0, 0.0, 30.0, 100.0, 70.0, 100.0, 100.0, 0.0, 0.5]));
// Returns: [50.0, 75.0]

// Statistics
let mean = module.execute("stats::mean", json!([1.0, 2.0, 3.0, 4.0, 5.0]));
// Returns: 3.0

// Geometry - Optical illusions
let penrose = module.execute("geometry::penrose", json!([400.0, 300.0, 100.0]));
// Returns: array of lines for drawing Penrose triangle
```

## Features

### Bezier Curves
- `bezier::linear` - Linear interpolation between 2 points
- `bezier::quadratic` - Quadratic Bezier with 1 control point
- `bezier::cubic` - Cubic Bezier with 2 control points

### Statistics
- `stats::mean` - Arithmetic mean
- `stats::median` - Median value
- `stats::min` - Minimum value
- `stats::max` - Maximum value

### Geometry (Optical Illusions)
- `geometry::penrose` - Penrose triangle (impossible object)
- `geometry::impossible_cube` - Necker cube variant
- `geometry::spiral` - Archimedean spiral
- `geometry::muller_lyer` - Müller-Lyer illusion
- `geometry::ponzo` - Ponzo perspective illusion

## LAZOS Protocol

Use with RyDit's LAZOS protocol:

```bash
# Bezier curve
echo '{"method":"science::bezier::cubic","params":[0,0,30,100,70,100,100,0,0.5]}' | rydit-rs --lazos

# Statistics
echo '{"method":"science::stats::mean","params":[[1,2,3,4,5]]}' | rydit-rs --lazos

# Geometry
echo '{"method":"science::geometry::penrose","params":[400,300,100]}' | rydit-rs --lazos
```

## License

MIT License - See [LICENSE](https://github.com/lapumlbb18-blip/Rydit_Engine/blob/main/LICENSE) for details.

## Contributing

Contributions are welcome! Please open an issue or submit a PR at:
https://github.com/lapumlbb18-blip/Rydit_Engine
