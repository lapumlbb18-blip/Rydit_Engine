// crates/rydit-rs/src/modules/tilemap.rs
// Tilemap System - Sistema de Tilemap para RyDit
//
// Funciones:
// - tilemap::load(ruta, tile_size) - Cargar tilemap desde imagen
// - tilemap::create(width, height, tile_size) - Crear tilemap vacío
// - tilemap::set_tile(x, y, tile_id) - Colocar tile
// - tilemap::get_tile(x, y) - Obtener tile en posición
// - tilemap::draw() - Dibujar tilemap completo
// - tilemap::draw_layer(layer) - Dibujar capa específica
// - tilemap::set_tileset(ruta) - Cambiar tileset
// - tilemap::get_size() - Obtener tamaño del tilemap
// - tilemap::clear() - Limpiar tilemap
// - tilemap::set_layer_count(count) - Establecer número de capas
// - tilemap::fill_rect(x, y, w, h, tile_id) - Llenar rectángulo con tiles

use blast_core::{Executor, Valor};
use lizer::{Expr, Stmt};
use ry_gfx::{ColorRydit, DrawHandle};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::eval::evaluar_expr;

// ============================================================================
// TILEMAP STRUCT
// ============================================================================

/// Tile individual
#[derive(Debug, Clone)]
pub struct Tile {
    /// ID del tile (índice en el tileset)
    pub id: u32,
    /// Capa del tile
    #[allow(dead_code)] // Para futuras capas múltiples
    pub layer: u32,
    /// Si el tile está flipado horizontalmente
    #[allow(dead_code)] // Para futuros flips de sprites
    pub flip_x: bool,
    /// Si el tile está flipado verticalmente
    #[allow(dead_code)] // Para futuros flips de sprites
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

/// Tilemap completo
#[derive(Debug)]
pub struct Tilemap {
    /// Ancho del tilemap en tiles
    pub width: u32,
    /// Alto del tilemap en tiles
    pub height: u32,
    /// Tamaño de cada tile en píxeles
    pub tile_size: u32,
    /// Tiles del tilemap (plano 2D)
    pub tiles: Vec<Vec<Tile>>,
    /// Tileset (imagen con todos los tiles)
    pub tileset_id: Option<String>,
    /// Número de capas
    pub layer_count: u32,
    /// Offset de dibujo (x, y)
    pub offset_x: f32,
    pub offset_y: f32,
    /// Si el tilemap es visible
    pub visible: bool,
}

impl Tilemap {
    /// Crear tilemap vacío
    pub fn new<'a>(width: u32, height: u32, tile_size: u32) -> Self {
        let tiles = vec![vec![Tile::default(); width as usize]; height as usize];

        Self {
            width,
            height,
            tile_size,
            tiles,
            tileset_id: None,
            layer_count: 1,
            offset_x: 0.0,
            offset_y: 0.0,
            visible: true,
        }
    }

    /// Cargar tilemap desde archivo de imagen
    #[allow(dead_code)] // Para futura carga de tilesets desde imagen
    pub fn from_image<'a>(ruta: &str, tile_size: u32) -> Result<Self, String> {
        // En el futuro, esto cargará una imagen y la dividirá en tiles
        // Por ahora, creamos un tilemap vacío y registramos el tileset

        // Calcular tamaño del tilemap basado en la imagen
        // (esto requeriría cargar la imagen primero)
        let width = 32; // Default
        let height = 32; // Default

        let mut tilemap = Self::new(width, height, tile_size);
        tilemap.tileset_id = Some(ruta.to_string());

        Ok(tilemap)
    }

    /// Establecer tile en posición
    pub fn set_tile<'a>(&mut self, x: u32, y: u32, tile_id: u32, layer: u32) {
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
    pub fn get_tile<'a>(&self, x: u32, y: u32) -> Option<&Tile> {
        if x < self.width && y < self.height {
            Some(&self.tiles[y as usize][x as usize])
        } else {
            None
        }
    }

    /// Llenar rectángulo con tiles
    pub fn fill_rect<'a>(&mut self, x: u32, y: u32, w: u32, h: u32, tile_id: u32) {
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

    /// Limpiar tilemap (todos los tiles a 0)
    pub fn clear<'a>(&mut self) {
        for row in &mut self.tiles {
            for tile in row {
                *tile = Tile::default();
            }
        }
    }

    /// Establecer número de capas
    pub fn set_layer_count<'a>(&mut self, count: u32) {
        self.layer_count = count;
    }

    /// Dibujar tilemap completo
    #[allow(dead_code)] // El dibujo se hace en el game loop
    pub fn draw<'a>(&self, draw: &mut DrawHandle) {
        if !self.visible {
            return;
        }

        // Si no hay tileset, no dibujar
        if self.tileset_id.is_none() {
            return;
        }

        // Dibujar cada tile como rectángulo de color
        // En el futuro, esto usará el tileset real con assets::draw
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.id == 0 {
                    continue; // Tile vacío
                }

                // Calcular posición en píxeles
                let px = self.offset_x + (x as f32 * self.tile_size as f32);
                let py = self.offset_y + (y as f32 * self.tile_size as f32);

                // Dibujar tile como rectángulo verde
                draw.draw_rectangle(
                    px as i32,
                    py as i32,
                    self.tile_size as i32,
                    self.tile_size as i32,
                    ColorRydit::Verde,
                );
            }
        }
    }

    /// Obtener tamaño del tilemap
    pub fn get_size<'a>(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl Default for Tilemap {
    fn default() -> Self {
        Self::new(32, 32, 32)
    }
}

// ============================================================================
// ESTADO GLOBAL
// ============================================================================

thread_local! {
    static TILEMAP: Rc<RefCell<Tilemap>> = Rc::new(RefCell::new(Tilemap::default()));
}

/// Obtener referencia al Tilemap
pub fn get_tilemap<'a>() -> Rc<RefCell<Tilemap>> {
    TILEMAP.with(|tm| tm.clone())
}

// ============================================================================
// FUNCIONES PARA .rydit
// ============================================================================

/// tilemap::load(ruta, tile_size) - Cargar tilemap desde imagen
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

    tm_ref.tileset_id = Some(ruta.clone());
    tm_ref.tile_size = tile_size;

    Valor::Texto(format!(
        "Tilemap cargado: {} (tile_size={})",
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

/// tilemap::draw() - Dibujar tilemap completo
pub fn tilemap_draw<'a>(
    _args: &[Expr<'a>],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    // El dibujo real se hace en el game loop
    // Esta función solo marca que se debe dibujar
    Valor::Texto("tilemap::draw() - tilemap marcado para dibujar".to_string())
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

    tm_ref.tileset_id = Some(ruta.clone());

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
        _ => {
            return Valor::Error(
                "tilemap::set_visible() visible debe ser booleano o número".to_string(),
            )
        }
    };

    let tm = get_tilemap();
    let mut tm_ref = tm.borrow_mut();

    tm_ref.visible = visible;

    Valor::Texto(format!(
        "Tilemap {}",
        if visible { "visible" } else { "oculto" }
    ))
}
