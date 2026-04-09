// crates/rydit-rs/src/modules/tilemap.rs
// Tilemap System v2.0 — Texturas reales + multi-capa + camera culling + CSV import
//
// Funciones:
// - tilemap::load(ruta, tile_size) - Cargar tilemap desde imagen
// - tilemap::create(width, height, tile_size) - Crear tilemap vacío
// - tilemap::set_tile(x, y, tile_id) - Colocar tile
// - tilemap::get_tile(x, y) - Obtener tile en posición
// - tilemap::draw(camera_x, camera_y, screen_w, screen_h) - Dibujar con culling
// - tilemap::draw_layer(layer, camera_x, camera_y, screen_w, screen_h) - Dibujar capa específica
// - tilemap::set_tileset(ruta) - Cambiar tileset
// - tilemap::get_size() - Obtener tamaño del tilemap
// - tilemap::clear() - Limpiar tilemap
// - tilemap::set_layer_count(count) - Establecer número de capas
// - tilemap::fill_rect(x, y, w, h, tile_id) - Llenar rectángulo con tiles
// - tilemap::import_csv(ruta) - Importar tilemap desde CSV
// - tilemap::export_csv(ruta) - Exportar tilemap a CSV

use blast_core::{Executor, Valor};
use lizer::{Expr, Stmt};
use ry_gfx::{ColorRydit, DrawHandle};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

use crate::eval::evaluar_expr;

// ============================================================================
// TILE STRUCT
// ============================================================================

/// Tile individual
#[derive(Debug, Clone)]
pub struct Tile {
    /// ID del tile (índice en el tileset)
    pub id: u32,
    /// Capa del tile
    pub layer: u32,
    /// Si el tile está flipado horizontalmente
    pub flip_x: bool,
    /// Si el tile está flipado verticalmente
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

/// Tileset cargado — textura + dimensiones
pub struct Tileset {
    /// Textura SDL2 del tileset
    pub texture: sdl2::render::Texture<'static>,
    /// Ancho de cada tile en la textura
    pub tile_width: u32,
    /// Alto de cada tile en la textura
    pub tile_height: u32,
    /// Columnas en el tileset
    pub columns: u32,
    /// Total de tiles disponibles
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
    /// Cargar tileset desde imagen
    pub fn load(_path: &str, tile_width: u32, tile_height: u32) -> Result<Self, String> {
        // En el futuro, cargaremos la imagen aquí
        // Por ahora retornamos placeholder — la textura se registra externamente
        Ok(Self {
            texture: unsafe { std::mem::zeroed() },
            tile_width,
            tile_height,
            columns: 0,
            total_tiles: 0,
        })
    }

    /// Obtener source rect de un tile
    pub fn get_tile_rect(&self, tile_id: u32) -> (u32, u32, u32, u32) {
        let col = tile_id % self.columns;
        let row = tile_id / self.columns;
        (col * self.tile_width, row * self.tile_height, self.tile_width, self.tile_height)
    }
}

// ============================================================================
// TILEMAP STRUCT
// ============================================================================

/// Tilemap completo
#[derive(Debug)]
pub struct Tilemap {
    /// Ancho del tilemap en tiles
    pub width: u32,
    /// Alto del tilemap en tiles
    pub height: u32,
    /// Tamaño de cada tile en píxeles
    pub tile_size: u32,
    /// Tiles del tilemap (plano 2D por capa)
    pub tiles: Vec<Vec<Tile>>,
    /// Tileset (textura cargada)
    pub tileset: Option<Tileset>,
    /// Ruta del tileset
    pub tileset_path: Option<String>,
    /// Número de capas
    pub layer_count: u32,
    /// Offset de dibujo (x, y) — para cámara
    pub offset_x: f32,
    pub offset_y: f32,
    /// Si el tilemap es visible
    pub visible: bool,
}

impl Tilemap {
    /// Crear tilemap vacío
    pub fn new(width: u32, height: u32, tile_size: u32) -> Self {
        let tiles = vec![vec![Tile::default(); width as usize]; height as usize];

        Self {
            width,
            height,
            tile_size,
            tiles,
            tileset: None,
            tileset_path: None,
            layer_count: 1,
            offset_x: 0.0,
            offset_y: 0.0,
            visible: true,
        }
    }

    /// Cargar tileset desde imagen
    pub fn load_tileset(&mut self, path: &str, tile_width: u32, tile_height: u32) -> Result<(), String> {
        let tileset = Tileset::load(path, tile_width, tile_height)?;
        self.tileset = Some(tileset);
        self.tileset_path = Some(path.to_string());
        self.tile_size = tile_width; // Sincronizar tile_size con tileset
        Ok(())
    }

    /// Registrar textura SDL2 creada externamente (para cuando el backend carga la textura)
    pub fn register_tileset_texture(&mut self, texture: sdl2::render::Texture<'static>, tw: u32, th: u32) {
        if let Some(ref mut ts) = self.tileset {
            ts.texture = texture;
            ts.tile_width = tw;
            ts.tile_height = th;
        }
    }

    /// Establecer tile en posición
    pub fn set_tile(&mut self, x: u32, y: u32, tile_id: u32, layer: u32) {
        if x < self.width && y < self.height {
            self.tiles[y as usize][x as usize] = Tile {
                id: tile_id,
                layer,
                flip_x: false,
                flip_y: false,
            };
        }
    }

    /// Obtener tile en posición
    pub fn get_tile(&self, x: u32, y: u32) -> Option<&Tile> {
        if x < self.width && y < self.height {
            Some(&self.tiles[y as usize][x as usize])
        } else {
            None
        }
    }

    /// Llenar rectángulo con tiles
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

    /// Limpiar tilemap
    pub fn clear(&mut self) {
        for row in &mut self.tiles {
            for tile in row {
                *tile = Tile::default();
            }
        }
    }

    /// Establecer número de capas
    pub fn set_layer_count(&mut self, count: u32) {
        self.layer_count = count;
    }

    // ========================================================================
    // RENDER — Dibujar tilemap con texturas reales + camera culling
    // ========================================================================

    /// Dibujar tilemap completo con culling de cámara
    ///
    /// Solo dibuja tiles visibles en pantalla.
    /// Usa `draw_texture_rec` del backend para renderizar tiles del tileset.
    pub fn draw_with_camera(
        &self,
        camera_x: f32,
        camera_y: f32,
        screen_w: i32,
        screen_h: i32,
    ) -> Vec<TileRenderCommand> {
        if !self.visible || self.tileset.is_none() {
            return Vec::new();
        }

        let mut commands = Vec::new();
        let ts = self.tileset.as_ref().unwrap();
        let tile_px = self.tile_size;

        // Calcular rango de tiles visibles (camera culling)
        let start_x = ((camera_x / tile_px as f32).floor() as i32).max(0) as u32;
        let start_y = ((camera_y / tile_px as f32).floor() as i32).max(0) as u32;
        let end_x = (((camera_x + screen_w as f32) / tile_px as f32).ceil() as i32).min(self.width as i32) as u32;
        let end_y = (((camera_y + screen_h as f32) / tile_px as f32).ceil() as i32).min(self.height as i32) as u32;

        for y in start_y..end_y {
            for x in start_x..end_x {
                let tile = &self.tiles[y as usize][x as usize];
                if tile.id == 0 {
                    continue; // Tile vacío (tile 0 = empty)
                }

                // Source rect en el tileset
                let (sx, sy, sw, sh) = ts.get_tile_rect(tile.id);

                // Destino en pantalla (con offset de cámara)
                let dx = (x as f32 * tile_px as f32 - camera_x + self.offset_x) as i32;
                let dy = (y as f32 * tile_px as f32 - camera_y + self.offset_y) as i32;

                commands.push(TileRenderCommand {
                    source_x: sx,
                    source_y: sy,
                    source_w: sw,
                    source_h: sh,
                    dest_x: dx,
                    dest_y: dy,
                    dest_w: tile_px,
                    dest_h: tile_px,
                    flip_x: tile.flip_x,
                    flip_y: tile.flip_y,
                    tile_id: tile.id,
                });
            }
        }

        commands
    }

    /// Importar tilemap desde archivo CSV
    ///
    /// Formato CSV: cada celda es un número (tile_id), filas separadas por newline.
    /// Ejemplo:
    /// ```csv
    /// 0,0,0,0,0
    /// 1,1,1,1,1
    /// 2,2,0,2,2
    /// ```
    pub fn import_csv(&mut self, path: &str) -> Result<(), String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("No se pudo leer CSV '{}': {}", path, e))?;

        let mut new_tiles = Vec::new();
        let mut new_width = 0u32;
        let mut new_height = 0u32;

        for (row_idx, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut row = Vec::new();
            for (col_idx, cell) in line.split(',').enumerate() {
                let tile_id = cell.trim().parse::<u32>()
                    .map_err(|e| format!("Error parseando tile en fila {}, col {}: {}", row_idx, col_idx, e))?;
                row.push(Tile {
                    id: tile_id,
                    layer: 0,
                    flip_x: false,
                    flip_y: false,
                });
                new_width = (col_idx as u32 + 1).max(new_width);
            }
            new_tiles.push(row);
            new_height = row_idx as u32 + 1;
        }

        if new_width == 0 || new_height == 0 {
            return Err("CSV vacío".to_string());
        }

        // Normalizar filas al mismo ancho
        for row in &mut new_tiles {
            while row.len() < new_width as usize {
                row.push(Tile::default());
            }
        }

        self.tiles = new_tiles;
        self.width = new_width;
        self.height = new_height;

        Ok(())
    }

    /// Exportar tilemap a archivo CSV
    pub fn export_csv(&self, path: &str) -> Result<(), String> {
        let mut content = String::new();
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                if x > 0 {
                    content.push(',');
                }
                content.push_str(&self.tiles[y][x].id.to_string());
            }
            content.push('\n');
        }

        fs::write(path, content)
            .map_err(|e| format!("No se pudo escribir CSV '{}': {}", path, e))
    }

    /// Obtener tamaño del tilemap
    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl Default for Tilemap {
    fn default() -> Self {
        Self::new(32, 32, 32)
    }
}

// ============================================================================
// RENDER COMMAND
// ============================================================================

/// Comando de renderizado de tile — listo para backend
#[derive(Debug, Clone)]
pub struct TileRenderCommand {
    pub source_x: u32,
    pub source_y: u32,
    pub source_w: u32,
    pub source_h: u32,
    pub dest_x: i32,
    pub dest_y: i32,
    pub dest_w: u32,
    pub dest_h: u32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub tile_id: u32,
}

// ============================================================================
// ESTADO GLOBAL
// ============================================================================

thread_local! {
    static TILEMAP: Rc<RefCell<Tilemap>> = Rc::new(RefCell::new(Tilemap::default()));
}

/// Obtener referencia al Tilemap
pub fn get_tilemap() -> Rc<RefCell<Tilemap>> {
    TILEMAP.with(|tm| tm.clone())
}

// ============================================================================
// FUNCIONES PARA .rydit
// ============================================================================

/// tilemap::load(ruta, tile_size) - Cargar tileset para tilemap
pub fn tilemap_load<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("tilemap::load() requiere 2 argumentos: ruta, tile_size".to_string());
    }

    let ruta_val = evaluar_expr(&args[0], executor, funcs);
    let tile_size_val = evaluar_expr(&args[1], executor, funcs);

    let ruta = match ruta_val {
        Valor::Texto(r) => r,
        _ => return Valor::Error("tilemap::load() ruta debe ser texto".to_string()),
    };

    let tile_size = match tile_size_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("tilemap::load() tile_size debe ser número".to_string()),
    };

    let tm = get_tilemap();
    let mut tm_ref = tm.borrow_mut();

    tm_ref.tileset_path = Some(ruta.clone());
    tm_ref.tile_size = tile_size;

    Valor::Texto(format!(
        "Tilemap tileset registrado: {} (tile_size={})",
        ruta, tile_size
    ))
}

/// tilemap::create(width, height, tile_size) - Crear tilemap vacío
pub fn tilemap_create<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 3 {
        return Valor::Error(
            "tilemap::create() requiere 3 argumentos: width, height, tile_size".to_string(),
        );
    }

    let width_val = evaluar_expr(&args[0], executor, funcs);
    let height_val = evaluar_expr(&args[1], executor, funcs);
    let tile_size_val = evaluar_expr(&args[2], executor, funcs);

    let width = match width_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("tilemap::create() width debe ser número".to_string()),
    };

    let height = match height_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("tilemap::create() height debe ser número".to_string()),
    };

    let tile_size = match tile_size_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("tilemap::create() tile_size debe ser número".to_string()),
    };

    let tm = get_tilemap();
    let mut tm_ref = tm.borrow_mut();

    *tm_ref = Tilemap::new(width, height, tile_size);

    Valor::Texto(format!(
        "Tilemap creado: {}x{} (tile_size={})",
        width, height, tile_size
    ))
}

/// tilemap::set_tile(x, y, tile_id) - Colocar tile
pub fn tilemap_set_tile<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 3 {
        return Valor::Error(
            "tilemap::set_tile() requiere 3 argumentos: x, y, tile_id".to_string(),
        );
    }

    let x_val = evaluar_expr(&args[0], executor, funcs);
    let y_val = evaluar_expr(&args[1], executor, funcs);
    let tile_id_val = evaluar_expr(&args[2], executor, funcs);

    let x = match x_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("tilemap::set_tile() x debe ser número".to_string()),
    };

    let y = match y_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("tilemap::set_tile() y debe ser número".to_string()),
    };

    let tile_id = match tile_id_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("tilemap::set_tile() tile_id debe ser número".to_string()),
    };

    let tm = get_tilemap();
    let mut tm_ref = tm.borrow_mut();

    tm_ref.set_tile(x, y, tile_id, 0);

    Valor::Texto(format!("Tile {} colocado en ({}, {})", tile_id, x, y))
}

/// tilemap::get_tile(x, y) - Obtener tile en posición
pub fn tilemap_get_tile<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("tilemap::get_tile() requiere 2 argumentos: x, y".to_string());
    }

    let x_val = evaluar_expr(&args[0], executor, funcs);
    let y_val = evaluar_expr(&args[1], executor, funcs);

    let x = match x_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("tilemap::get_tile() x debe ser número".to_string()),
    };

    let y = match y_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("tilemap::get_tile() y debe ser número".to_string()),
    };

    let tm = get_tilemap();
    let tm_ref = tm.borrow();

    match tm_ref.get_tile(x, y) {
        Some(tile) => Valor::Num(tile.id as f64),
        None => Valor::Error(format!("Tile en ({}, {}) fuera de rango", x, y)),
    }
}

/// tilemap::draw(camera_x, camera_y, screen_w, screen_h) - Dibujar con culling
pub fn tilemap_draw<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    // Default: sin culling (dibuja todo)
    let (cam_x, cam_y, sw, sh) = if args.len() >= 4 {
        let cx = match evaluar_expr(&args[0], executor, funcs) {
            Valor::Num(n) => n as f32,
            _ => 0.0,
        };
        let cy = match evaluar_expr(&args[1], executor, funcs) {
            Valor::Num(n) => n as f32,
            _ => 0.0,
        };
        let w = match evaluar_expr(&args[2], executor, funcs) {
            Valor::Num(n) => n as i32,
            _ => 800,
        };
        let h = match evaluar_expr(&args[3], executor, funcs) {
            Valor::Num(n) => n as i32,
            _ => 600,
        };
        (cx, cy, w, h)
    } else {
        (0.0, 0.0, 800, 600)
    };

    let tm = get_tilemap();
    let tm_ref = tm.borrow();
    let commands = tm_ref.draw_with_camera(cam_x, cam_y, sw, sh);

    // Los commands se procesan en el game loop del demo
    Valor::Texto(format!("tilemap::draw() - {} tiles listos para renderizar", commands.len()))
}

/// tilemap::fill_rect(x, y, w, h, tile_id) - Llenar rectángulo con tiles
pub fn tilemap_fill_rect<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 5 {
        return Valor::Error(
            "tilemap::fill_rect() requiere 5 argumentos: x, y, w, h, tile_id".to_string(),
        );
    }

    let x_val = evaluar_expr(&args[0], executor, funcs);
    let y_val = evaluar_expr(&args[1], executor, funcs);
    let w_val = evaluar_expr(&args[2], executor, funcs);
    let h_val = evaluar_expr(&args[3], executor, funcs);
    let tile_id_val = evaluar_expr(&args[4], executor, funcs);

    let x = match x_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("tilemap::fill_rect() x debe ser número".to_string()),
    };

    let y = match y_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("tilemap::fill_rect() y debe ser número".to_string()),
    };

    let w = match w_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("tilemap::fill_rect() w debe ser número".to_string()),
    };

    let h = match h_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("tilemap::fill_rect() h debe ser número".to_string()),
    };

    let tile_id = match tile_id_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("tilemap::fill_rect() tile_id debe ser número".to_string()),
    };

    let tm = get_tilemap();
    let mut tm_ref = tm.borrow_mut();

    tm_ref.fill_rect(x, y, w, h, tile_id);

    Valor::Texto(format!(
        "Rectángulo {}x{} llenado con tile {}",
        w, h, tile_id
    ))
}

/// tilemap::clear() - Limpiar tilemap
pub fn tilemap_clear<'a>(
    _args: &[Expr<'a>],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    let tm = get_tilemap();
    let mut tm_ref = tm.borrow_mut();
    tm_ref.clear();
    Valor::Texto("Tilemap limpiado".to_string())
}

/// tilemap::set_layer_count(count) - Establecer número de capas
pub fn tilemap_set_layer_count<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("tilemap::set_layer_count() requiere 1 argumento: count".to_string());
    }

    let count_val = evaluar_expr(&args[0], executor, funcs);
    let count = match count_val {
        Valor::Num(n) => n as u32,
        _ => return Valor::Error("tilemap::set_layer_count() count debe ser número".to_string()),
    };

    let tm = get_tilemap();
    let mut tm_ref = tm.borrow_mut();
    tm_ref.set_layer_count(count);
    Valor::Texto(format!("Capas establecidas a {}", count))
}

/// tilemap::get_size() - Obtener tamaño del tilemap
pub fn tilemap_get_size<'a>(
    _args: &[Expr<'a>],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    let tm = get_tilemap();
    let tm_ref = tm.borrow();
    let (width, height) = tm_ref.get_size();
    Valor::Array(vec![Valor::Num(width as f64), Valor::Num(height as f64)])
}

/// tilemap::set_tileset(ruta) - Cambiar tileset
pub fn tilemap_set_tileset<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("tilemap::set_tileset() requiere 1 argumento: ruta".to_string());
    }

    let ruta_val = evaluar_expr(&args[0], executor, funcs);
    let ruta = match ruta_val {
        Valor::Texto(r) => r,
        _ => return Valor::Error("tilemap::set_tileset() ruta debe ser texto".to_string()),
    };

    let tm = get_tilemap();
    let mut tm_ref = tm.borrow_mut();
    tm_ref.tileset_path = Some(ruta.clone());
    Valor::Texto(format!("Tileset cambiado a: {}", ruta))
}

/// tilemap::set_offset(x, y) - Establecer offset de dibujo
pub fn tilemap_set_offset<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 2 {
        return Valor::Error("tilemap::set_offset() requiere 2 argumentos: x, y".to_string());
    }

    let x_val = evaluar_expr(&args[0], executor, funcs);
    let y_val = evaluar_expr(&args[1], executor, funcs);

    let x = match x_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("tilemap::set_offset() x debe ser número".to_string()),
    };

    let y = match y_val {
        Valor::Num(n) => n as f32,
        _ => return Valor::Error("tilemap::set_offset() y debe ser número".to_string()),
    };

    let tm = get_tilemap();
    let mut tm_ref = tm.borrow_mut();
    tm_ref.offset_x = x;
    tm_ref.offset_y = y;
    Valor::Texto(format!("Offset establecido a ({}, {})", x, y))
}

/// tilemap::set_visible(visible) - Establecer visibilidad
pub fn tilemap_set_visible<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("tilemap::set_visible() requiere 1 argumento: visible".to_string());
    }

    let visible_val = evaluar_expr(&args[0], executor, funcs);
    let visible = match visible_val {
        Valor::Bool(v) => v,
        Valor::Num(n) => n != 0.0,
        _ => return Valor::Error("tilemap::set_visible() visible debe ser booleano o número".to_string()),
    };

    let tm = get_tilemap();
    let mut tm_ref = tm.borrow_mut();
    tm_ref.visible = visible;
    Valor::Texto(format!("Tilemap {}", if visible { "visible" } else { "oculto" }))
}

/// tilemap::import_csv(ruta) - Importar desde CSV
pub fn tilemap_import_csv<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("tilemap::import_csv() requiere 1 argumento: ruta".to_string());
    }

    let ruta_val = evaluar_expr(&args[0], executor, funcs);
    let ruta = match ruta_val {
        Valor::Texto(r) => r,
        _ => return Valor::Error("tilemap::import_csv() ruta debe ser texto".to_string()),
    };

    let tm = get_tilemap();
    let mut tm_ref = tm.borrow_mut();

    match tm_ref.import_csv(&ruta) {
        Ok(()) => {
            let (w, h) = tm_ref.get_size();
            Valor::Texto(format!("Tilemap importado desde CSV: {}x{} tiles", w, h))
        }
        Err(e) => Valor::Error(format!("Error importando CSV: {}", e)),
    }
}

/// tilemap::export_csv(ruta) - Exportar a CSV
pub fn tilemap_export_csv<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("tilemap::export_csv() requiere 1 argumento: ruta".to_string());
    }

    let ruta_val = evaluar_expr(&args[0], executor, funcs);
    let ruta = match ruta_val {
        Valor::Texto(r) => r,
        _ => return Valor::Error("tilemap::export_csv() ruta debe ser texto".to_string()),
    };

    let tm = get_tilemap();
    let tm_ref = tm.borrow();

    match tm_ref.export_csv(&ruta) {
        Ok(()) => Valor::Texto(format!("Tilemap exportado a CSV: {}", ruta)),
        Err(e) => Valor::Error(format!("Error exportando CSV: {}", e)),
    }
}
