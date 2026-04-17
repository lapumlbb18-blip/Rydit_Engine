//! RyDit Tilemap System
//!
//! Implementación centralizada y pura de Tilemap.
//! Independiente de la capa de scripting.

use std::fs;

// ============================================================================
// TILE STRUCT
// ============================================================================

#[derive(Debug, Clone)]
pub struct Tile {
    pub id: u32,
    pub layer: u32,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            id: 0,
            layer: 0,
            flip_x: false,
            flip_y: false,
        }
    }
}

// ============================================================================
// TILESET STRUCT
// ============================================================================

pub struct Tileset {
    // Nota: Textura es 'static para compatibilidad con el sistema actual, 
    // pero idealmente usaremos AssetServer en el futuro
    pub texture: sdl2::render::Texture<'static>,
    pub tile_width: u32,
    pub tile_height: u32,
    pub columns: u32,
    pub total_tiles: u32,
}

impl std::fmt::Debug for Tileset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tileset")
            .field("tile_width", &self.tile_width)
            .field("tile_height", &self.tile_height)
            .field("columns", &self.columns)
            .field("total_tiles", &self.total_tiles)
            .finish()
    }
}

impl Tileset {
    pub fn get_tile_rect(&self, tile_id: u32) -> (u32, u32, u32, u32) {
        if self.columns == 0 { return (0, 0, self.tile_width, self.tile_height); }
        let col = tile_id % self.columns;
        let row = tile_id / self.columns;
        (col * self.tile_width, row * self.tile_height, self.tile_width, self.tile_height)
    }
}

// ============================================================================
// TILEMAP STRUCT
// ============================================================================

#[derive(Debug, Default)]
pub struct Tilemap {
    pub width: u32,
    pub height: u32,
    pub tile_size: u32,
    pub tiles: Vec<Vec<Tile>>,
    pub tileset: Option<Tileset>,
    pub tileset_path: Option<String>,
    pub layer_count: u32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub visible: bool,
}

impl Tilemap {
    pub fn new(width: u32, height: u32, tile_size: u32) -> Self {
        let tiles = vec![vec![Tile::default(); width as usize]; height as usize];
        Self {
            width, height, tile_size, tiles,
            tileset: None, tileset_path: None,
            layer_count: 1, offset_x: 0.0, offset_y: 0.0, visible: true,
        }
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile_id: u32, layer: u32) {
        if x < self.width && y < self.height {
            self.tiles[y as usize][x as usize] = Tile {
                id: tile_id, layer, flip_x: false, flip_y: false,
            };
        }
    }

    pub fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, tile_id: u32) {
        for dy in 0..h {
            for dx in 0..w {
                let px = x + dx;
                let py = y + dy;
                if px < self.width && py < self.height {
                    self.set_tile(px, py, tile_id, 0);
                }
            }
        }
    }

    pub fn draw_with_camera(&self, camera_x: f32, camera_y: f32, screen_w: i32, screen_h: i32) -> Vec<TileRenderCommand> {
        if !self.visible || self.tileset.is_none() { return Vec::new(); }
        let mut commands = Vec::new();
        let ts = self.tileset.as_ref().unwrap();
        let tile_px = self.tile_size;

        let start_x = ((camera_x / tile_px as f32).floor() as i32).max(0) as u32;
        let start_y = ((camera_y / tile_px as f32).floor() as i32).max(0) as u32;
        let end_x = (((camera_x + screen_w as f32) / tile_px as f32).ceil() as i32).min(self.width as i32) as u32;
        let end_y = (((camera_y + screen_h as f32) / tile_px as f32).ceil() as i32).min(self.height as i32) as u32;

        for y in start_y..end_y {
            for x in start_x..end_x {
                let tile = &self.tiles[y as usize][x as usize];
                if tile.id == 0 { continue; }
                let (sx, sy, sw, sh) = ts.get_tile_rect(tile.id);
                let dx = (x as f32 * tile_px as f32 - camera_x + self.offset_x) as i32;
                let dy = (y as f32 * tile_px as f32 - camera_y + self.offset_y) as i32;

                commands.push(TileRenderCommand {
                    source_x: sx, source_y: sy, source_w: sw, source_h: sh,
                    dest_x: dx, dest_y: dy, dest_w: tile_px, dest_h: tile_px,
                    flip_x: tile.flip_x, flip_y: tile.flip_y, tile_id: tile.id,
                });
            }
        }
        commands
    }

    pub fn import_csv(&mut self, path: &str) -> Result<(), String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut new_tiles = Vec::new();
        let mut new_width = 0u32;
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            let mut row = Vec::new();
            for cell in line.split(',') {
                let tile_id = cell.trim().parse::<u32>().unwrap_or(0);
                row.push(Tile { id: tile_id, ..Tile::default() });
                new_width = (row.len() as u32).max(new_width);
            }
            new_tiles.push(row);
        }
        self.tiles = new_tiles;
        self.height = self.tiles.len() as u32;
        self.width = new_width;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TileRenderCommand {
    pub source_x: u32, pub source_y: u32, pub source_w: u32, pub source_h: u32,
    pub dest_x: i32, pub dest_y: i32, pub dest_w: u32, pub dest_h: u32,
    pub flip_x: bool, pub flip_y: bool, pub tile_id: u32,
}
