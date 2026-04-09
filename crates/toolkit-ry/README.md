# toolkit-ry

**Game UI Toolkit for Ry-Dit - HUD, Menus, Dialogs, Inventory + 5 Pre-built Themes**

[![Crates.io](https://img.shields.io/crates/v/toolkit-ry.svg)](https://crates.io/crates/toolkit-ry)
[![Documentation](https://docs.rs/toolkit-ry/badge.svg)](https://docs.rs/toolkit-ry)
[![License](https://img.shields.io/crates/l/toolkit-ry.svg)](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE)
[![Ry-Dit](https://img.shields.io/badge/Ry--Dit-v0.16.0-blue)](https://github.com/lapumlbb18-blip/Ry-dit)

## Overview

`toolkit-ry` is a comprehensive UI toolkit for building game interfaces in Rust. Built on top of **migui** (immediate mode GUI) and **SDL2**, it provides ready-to-use widgets for health bars, mana bars, inventory systems, dialog boxes, menus, loading screens, minimaps, and more.

Includes **5 pre-built visual themes** (Dark, Light, Retro, Neon, Minimal) that can be customized or used out of the box.

## Features

### 🎨 5 Pre-built Themes

| Theme | Style | Use Case |
|-------|-------|----------|
| **Dark** | Modern dark mode | Default for most games |
| **Light** | Clean light | Casual, mobile games |
| **Retro** | 8-bit pixel art | Retro, indie games |
| **Neon** | Cyberpunk glow | Sci-fi, futuristic games |
| **Minimal** | Ultra clean, no borders | Minimalist, puzzle games |

### 📊 HUD Widgets

- **Health Bar** (HP) - With label and dynamic color
- **Mana Bar** (MP) - Magic/stamina resource
- **XP Bar** - Experience/progress tracking
- **Score Display** - Points counter
- **Gold/Currency** - Money display (with emoji 🪙)
- **Timer** - MM:SS format countdown
- **Full HUD** - Combined HP + MP + XP + Score in one call

### 🎮 Menu Systems

- **Main Menu** - Title + options with hover states
- **Pause Menu** - Continue, Options, Save, Exit
- **Game Over** - Score display + restart/menu options
- **Options Menu** - Volume slider, fullscreen toggle
- **Loading Screen** - Progress bar + percentage

### 💬 Dialog & Notifications

- **NPC Dialog** - Character name + text box
- **Message Box** - Confirmation dialogs with buttons
- **Notifications** - Toast-style popups

### 🎒 Inventory System

- **Inventory Slot** - Individual item with count
- **Inventory Grid** - Configurable cols × rows grid
- Item selection and hover states
- Stack count display

### 🗺️ Minimap

- Player position indicator
- Scaled to world coordinates
- Customizable size

## Installation

```toml
[dependencies]
toolkit-ry = "0.1.0"
migui = "0.4.1"
sdl2 = "0.37"
```

## Quick Start

### Basic Health Bar

```rust
use toolkit_ry::theme::Theme;
use toolkit_ry::widgets::*;

// Create a migui instance (assuming you have SDL2 + migui setup)
// let mut gui = migui::Migui::new();

let theme = Theme::dark();

draw_health_bar(
    &mut gui,
    20.0,  // x position
    20.0,  // y position
    200.0, // width
    18.0,  // height
    75.0,  // current HP
    100.0, // max HP
    &theme,
);
```

### Full HUD in One Call

```rust
use toolkit_ry::theme::Theme;
use toolkit_ry::widgets::draw_full_hud;

let theme = Theme::dark();

draw_full_hud(
    &mut gui,
    75.0,   // current HP
    100.0,  // max HP
    50.0,   // current MP
    100.0,  // max MP
    150.0,  // current XP
    200.0,  // max XP
    12500,  // score
    &theme,
);
```

### Main Menu

```rust
use toolkit_ry::theme::Theme;
use toolkit_ry::widgets::draw_main_menu;

let theme = Theme::retro();
let mut hover_state = -1;

let selected = draw_main_menu(
    &mut gui,
    "MY AWESOME GAME",
    &["Start Game", "Options", "Quit"],
    &theme,
    &mut hover_state,
);

if selected >= 0 {
    println!("Menu option {} selected!", selected);
}
```

### Inventory Grid

```rust
use toolkit_ry::theme::Theme;
use toolkit_ry::widgets::draw_inventory_grid;

let theme = Theme::dark();
let mut selected_slot = 0;

// Create inventory items
let items = vec![
    Some(("Sword".to_string(), 1)),
    Some(("Potion".to_string(), 5)),
    None, // Empty slot
    Some(("Key".to_string(), 1)),
    // ... up to cols × rows items
];

let selected = draw_inventory_grid(
    &mut gui,
    100.0,  // x
    100.0,  // y
    4,      // columns
    3,      // rows
    50.0,   // slot size
    4.0,    // spacing
    &items,
    &mut selected_slot,
    &theme,
);
```

### Dialog with NPC

```rust
use toolkit_ry::theme::Theme;
use toolkit_ry::widgets::draw_dialog;

let theme = Theme::light();

draw_dialog(
    &mut gui,
    "Old Wizard",           // NPC name
    "Ah, a young adventurer! I have a quest for you...",
    50.0,   // x
    400.0,  // y
    700.0,  // width
    150.0,  // height
    &theme,
);
```

### Loading Screen

```rust
use toolkit_ry::theme::Theme;
use toolkit_ry::widgets::draw_loading;

let theme = Theme::neon();

// Update progress each frame (0.0 to 1.0)
let progress = 0.65; // 65% loaded
draw_loading(&mut gui, "Loading world...", progress, &theme);
```

### Minimap

```rust
use toolkit_ry::theme::Theme;
use toolkit_ry::widgets::draw_minimap;

let theme = Theme::dark();

draw_minimap(
    &mut gui,
    1100.0, // x (top-right corner)
    20.0,   // y
    150.0,  // size
    500.0,  // player x in world
    300.0,  // player y in world
    2000.0, // world width
    1500.0, // world height
    &theme,
);
```

## API Reference

### Theme System

```rust
pub struct Theme {
    pub name: &'static str,
    // Colors
    pub bg_color: ColorRGBA,
    pub panel_bg: ColorRGBA,
    pub text_color: ColorRGBA,
    // ... 30+ configurable properties
}

impl Theme {
    pub fn dark() -> Self;
    pub fn light() -> Self;
    pub fn retro() -> Self;
    pub fn neon() -> Self;
    pub fn minimal() -> Self;
    
    pub fn by_name(name: &str) -> Self;
    pub fn all() -> &'static [fn() -> Self];
}
```

### ColorRGBA

```rust
pub struct ColorRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRGBA {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self;
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self;
    pub const fn transparent() -> Self;
}
```

### Widget Functions

#### HUD
| Function | Parameters | Description |
|----------|-----------|-------------|
| `draw_health_bar` | `gui, x, y, w, h, current, max, theme` | HP bar with label |
| `draw_mana_bar` | `gui, x, y, w, h, current, max, theme` | MP/mana bar |
| `draw_xp_bar` | `gui, x, y, w, h, current, max, theme` | Experience bar |
| `draw_score` | `gui, x, y, score, theme` | Score display |
| `draw_gold` | `gui, x, y, gold, theme` | Currency with emoji |
| `draw_timer` | `gui, x, y, seconds, theme` | MM:SS timer |
| `draw_full_hud` | `gui, hp, max_hp, mp, max_mp, xp, max_xp, score, theme` | Complete HUD |

#### Menus
| Function | Parameters | Returns |
|----------|-----------|---------|
| `draw_main_menu` | `gui, title, options, theme, hover_state` | `i32` selected index |
| `draw_pause_menu` | `gui, theme, hover_state` | `i32` selected index |
| `draw_game_over` | `gui, score, theme, hover_state` | `i32` selected index |
| `draw_options_menu` | `gui, theme, volume, fullscreen, hover_state` | `()` |
| `draw_loading` | `gui, text, progress, theme` | `()` |

#### Dialogs
| Function | Parameters | Returns |
|----------|-----------|---------|
| `draw_dialog` | `gui, npc_name, text, x, y, w, h, theme` | `()` |
| `draw_message_box` | `gui, title, message, buttons, theme` | `i32` button index |
| `draw_notification` | `gui, text, x, y, theme` | `()` |

#### Inventory
| Function | Parameters | Returns |
|----------|-----------|---------|
| `draw_inventory_slot` | `gui, x, y, size, item_name, count, selected, theme` | `()` |
| `draw_inventory_grid` | `gui, x, y, cols, rows, slot_size, spacing, items, selected, theme` | `Option<usize>` |

#### Other
| Function | Parameters | Description |
|----------|-----------|-------------|
| `draw_minimap` | `gui, x, y, size, player_x, player_y, world_w, world_h, theme` | Minimap display |
| `rgba_to_migui` | `ColorRGBA` | Convert to migui::Color |
| `rgba_to_sdl2` | `ColorRGBA` | Convert to sdl2::pixels::Color |

## Theme Customization

All theme properties are public, so you can easily create your own:

```rust
use toolkit_ry::theme::{Theme, ColorRGBA};

let my_theme = Theme {
    name: "My Custom Theme",
    bg_color: ColorRGBA::rgb(50, 50, 100),
    health_bar_fill: ColorRGBA::rgb(0, 255, 0),
    font_size: 18,
    ..Theme::dark() // Inherit dark theme as base
};
```

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `migui` | 0.4.1 | Immediate mode GUI backend |
| `sdl2` | 0.37 | SDL2 bindings for color types |
| `ry-gfx` | 0.10.8 | Graphics layer (optional) |

## Examples

### Complete Game Menu Flow

```rust
use toolkit_ry::theme::Theme;
use toolkit_ry::widgets::*;

enum GameState {
    MainMenu,
    Options,
    Playing,
    Paused,
    GameOver,
}

fn render_menu(gui: &mut migui::Migui, state: &mut GameState, theme: &Theme) {
    let mut hover = -1;
    
    match state {
        GameState::MainMenu => {
            let selected = draw_main_menu(
                gui,
                "EPIC QUEST",
                &["New Game", "Options", "Quit"],
                theme,
                &mut hover,
            );
            
            match selected {
                0 => *state = GameState::Playing,
                1 => *state = GameState::Options,
                2 => std::process::exit(0),
                _ => {}
            }
        }
        GameState::Paused => {
            let selected = draw_pause_menu(gui, theme, &mut hover);
            if selected == 0 {
                *state = GameState::Playing; // Continue
            }
        }
        GameState::GameOver => {
            let selected = draw_game_over(gui, 15000, theme, &mut hover);
            if selected == 0 {
                *state = GameState::Playing; // Restart
            }
        }
        _ => {}
    }
}
```

### RPG HUD Example

```rust
// Full HUD with HP/MP/XP bars
let theme = Theme::retro();
draw_full_hud(
    &mut gui,
    120.0,  // HP: 120/150
    150.0,
    45.0,   // MP: 45/80
    80.0,
    230.0,  // XP: 230/500
    500.0,
    8750,   // Score
    &theme,
);

// Add timer and gold
draw_timer(&mut gui, 600.0, 720.0, 345.0, &theme); // 5:45
draw_gold(&mut gui, 600.0, 50.0, 1250, &theme);    // 🪙 1250
```

## Performance

- **Immediate mode** - No retained state overhead
- **Zero allocations** in widget drawing (pure rendering)
- **Theme system** is copy-based (ColorRGBA is Copy)
- **10 unit tests** ensuring widget correctness

## Roadmap

- [ ] Scrollable panels
- [ ] Text input fields
- [ ] Dropdown menus
- [ ] Tooltip system
- [ ] Animated transitions
- [ ] Theme serialization (JSON/TOML)
- [ ] Custom widget macro
- [ ] Responsive layouts

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
| [migui](https://crates.io/crates/migui) | Immediate mode GUI backend |
| [ry-gfx](https://crates.io/crates/ry-gfx) | Graphics layer with GPU instancing |
| [ry-backend](https://crates.io/crates/ry-backend) | Dual backend (raylib + SDL2) |
| [ry-anim](https://crates.io/crates/ry-anim) | Animation system with Disney principles |

---

<div align="center">

**toolkit-ry** - Build game UIs faster with pre-built widgets and themes 🎮✨

*5 themes · 20+ widgets · HUD · Menus · Inventory · Dialogs*

</div>
