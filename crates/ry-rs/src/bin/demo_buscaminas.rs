// demo_buscaminas.rs
// Buscaminas clásico — Click izquierdo revela, click derecho marca bandera
//
// Controles:
// Click izquierdo: Revelar tile
// Click derecho: Colocar/quitar bandera 🚩
// R: Reiniciar juego
// ESC: Salir

use ry_gfx::backend_sdl2::Sdl2Backend;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

// ============================================================================
// CONSTANTES
// ============================================================================
const GRID_W: usize = 16;
const GRID_H: usize = 16;
const MINES: usize = 40;
const TILE_SIZE: i32 = 36;
const HUD_H: i32 = 60;
const SCREEN_W: i32 = GRID_W as i32 * TILE_SIZE;
const SCREEN_H: i32 = GRID_H as i32 * TILE_SIZE + HUD_H;

// ============================================================================
// TILE STATE
// ============================================================================
#[derive(Clone, Copy, PartialEq)]
enum TileState {
    Hidden,
    Revealed,
    Flagged,
}

// ============================================================================
// GRID
// ============================================================================
struct Grid {
    mines: [[bool; GRID_W]; GRID_H],
    numbers: [[i32; GRID_W]; GRID_H],
    state: [[TileState; GRID_W]; GRID_H],
    game_over: bool,
    won: bool,
    first_click: bool,
    flags_placed: usize,
}

impl Grid {
    fn new() -> Self {
        Self {
            mines: [[false; GRID_W]; GRID_H],
            numbers: [[0; GRID_W]; GRID_H],
            state: [[TileState::Hidden; GRID_W]; GRID_H],
            game_over: false,
            won: false,
            first_click: true,
            flags_placed: 0,
        }
    }

    fn place_mines(&mut self, safe_x: usize, safe_y: usize) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut placed = 0;

        while placed < MINES {
            let x = rng.gen_range(0..GRID_W);
            let y = rng.gen_range(0..GRID_H);

            // No poner mina en primer click ni alrededores
            if (x as i32 - safe_x as i32).abs() <= 1 && (y as i32 - safe_y as i32).abs() <= 1 {
                continue;
            }
            if self.mines[y][x] { continue; }

            self.mines[y][x] = true;
            placed += 1;
        }

        self.calc_numbers();
    }

    fn calc_numbers(&mut self) {
        for y in 0..GRID_H {
            for x in 0..GRID_W {
                if self.mines[y][x] { continue; }
                let mut count = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx >= 0 && nx < GRID_W as i32 && ny >= 0 && ny < GRID_H as i32 {
                            if self.mines[ny as usize][nx as usize] {
                                count += 1;
                            }
                        }
                    }
                }
                self.numbers[y][x] = count;
            }
        }
    }

    fn reveal(&mut self, x: usize, y: usize) {
        if self.game_over || self.won { return; }
        if x >= GRID_W || y >= GRID_H { return; }
        if self.state[y][x] != TileState::Hidden { return; }

        // Primer click: colocar minas seguras
        if self.first_click {
            self.place_mines(x, y);
            self.first_click = false;
        }

        if self.mines[y][x] {
            self.game_over = true;
            return;
        }

        self.flood_reveal(x, y);
        self.check_win();
    }

    fn flood_reveal(&mut self, x: usize, y: usize) {
        if x >= GRID_W || y >= GRID_H { return; }
        if self.state[y][x] != TileState::Hidden { return; }
        if self.mines[y][x] { return; }

        self.state[y][x] = TileState::Revealed;

        if self.numbers[y][x] == 0 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 { continue; }
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < GRID_W as i32 && ny >= 0 && ny < GRID_H as i32 {
                        self.flood_reveal(nx as usize, ny as usize);
                    }
                }
            }
        }
    }

    fn toggle_flag(&mut self, x: usize, y: usize) {
        if self.game_over || self.won { return; }
        if x >= GRID_W || y >= GRID_H { return; }

        match self.state[y][x] {
            TileState::Hidden => {
                self.state[y][x] = TileState::Flagged;
                self.flags_placed += 1;
            }
            TileState::Flagged => {
                self.state[y][x] = TileState::Hidden;
                self.flags_placed -= 1;
            }
            _ => {}
        }
    }

    fn check_win(&mut self) {
        let mut revealed = 0;
        for y in 0..GRID_H {
            for x in 0..GRID_W {
                if self.state[y][x] == TileState::Revealed {
                    revealed += 1;
                }
            }
        }
        if revealed == GRID_W * GRID_H - MINES {
            self.won = true;
        }
    }

    fn reveal_all_mines(&mut self) {
        for y in 0..GRID_H {
            for x in 0..GRID_W {
                if self.mines[y][x] {
                    self.state[y][x] = TileState::Revealed;
                }
            }
        }
    }
}

// ============================================================================
// TEXT HELPER
// ============================================================================
fn crear_textura<'a>(
    font: &Option<ry_gfx::sdl2_ffi::FontFFI>,
    texto: &str,
    r: u8, g: u8, b: u8,
    tc: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) -> Option<sdl2::render::Texture<'a>> {
    if let Some(f) = font {
        if let Ok(sp) = f.render_text_blended(texto, r, g, b) {
            unsafe {
                let s = sdl2::surface::Surface::from_ll(sp as *mut sdl2::sys::SDL_Surface);
                if let Ok(t) = tc.create_texture_from_surface(&s) {
                    return Some(std::mem::transmute(t));
                }
            }
        }
    }
    None
}

// ============================================================================
// MAIN
// ============================================================================
fn main() -> Result<(), String> {
    println!("💣 RyDit - Buscaminas");
    println!("Click izq: Revelar | Click der: Bandera | R: Reiniciar | ESC: Salir\n");

    let mut backend = Sdl2Backend::new("Buscaminas", SCREEN_W, SCREEN_H)?;

    for p in &["/system/fonts/DroidSans.ttf", "/data/data/com.termux/files/usr/share/fonts/noto-sans/NotoSans-Regular.ttf"] {
        if std::path::Path::new(p).exists() {
            let _ = backend.load_font(p, 16);
            let _ = backend.load_font(p, 20);
            break;
        }
    }

    let tc = &backend.canvas.texture_creator();
    let mut grid = Grid::new();
    let mut txt_hud: Option<sdl2::render::Texture<'static>> = None;
    let mut txt_msg: Option<sdl2::render::Texture<'static>> = None;

    fn update_hud(grid: &Grid, tc: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
                   font: &Option<ry_gfx::sdl2_ffi::FontFFI>) -> (Option<sdl2::render::Texture<'static>>, Option<sdl2::render::Texture<'static>>) {
        let flags = MINES - grid.flags_placed;
        let hud_txt = crear_textura(font, &format!("💣 Minas: {} | 🚩 Banderas: {}", MINES, flags), 255, 255, 255, tc)
            .map(|t| unsafe { std::mem::transmute(t) });

        let msg_txt = if grid.game_over {
            crear_textura(font, "💥 GAME OVER — Presiona R para reiniciar", 255, 80, 80, tc)
                .map(|t| unsafe { std::mem::transmute(t) })
        } else if grid.won {
            crear_textura(font, "🎉 ¡GANASTE! — Presiona R para reiniciar", 80, 255, 80, tc)
                .map(|t| unsafe { std::mem::transmute(t) })
        } else {
            None
        };

        (hud_txt, msg_txt)
    }

    let (h, m) = update_hud(&grid, tc, &backend.font);
    txt_hud = h;
    txt_msg = m;

    let mut running = true;
    'run: loop {
        for ev in backend.event_pump.poll_iter() {
            match ev {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false; break 'run;
                }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    grid = Grid::new();
                    let (h, m) = update_hud(&grid, tc, &backend.font);
                    txt_hud = h; txt_msg = m;
                }
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    if grid.game_over || grid.won { continue; }
                    let tx = (x as i32 / TILE_SIZE) as usize;
                    let ty = ((y as i32 - HUD_H) / TILE_SIZE) as usize;
                    if tx >= GRID_W || ty >= GRID_H { continue; }

                    match mouse_btn {
                        MouseButton::Left => {
                            if grid.state[ty][tx] != TileState::Flagged {
                                grid.reveal(tx, ty);
                                if grid.game_over { grid.reveal_all_mines(); }
                                let (_, m) = update_hud(&grid, tc, &backend.font);
                                txt_msg = m;
                            }
                        }
                        MouseButton::Right => {
                            if grid.state[ty][tx] != TileState::Revealed {
                                grid.toggle_flag(tx, ty);
                                let (h, _) = update_hud(&grid, tc, &backend.font);
                                txt_hud = h;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // ---- RENDER ----
        backend.canvas.set_draw_color(Color::RGB(30, 30, 40));
        backend.canvas.clear();

        // HUD fondo
        backend.canvas.set_draw_color(Color::RGB(20, 20, 30));
        let _ = backend.canvas.fill_rect(Rect::new(0, 0, SCREEN_W as u32, HUD_H as u32));

        // HUD texto
        if let Some(ref tex) = txt_hud {
            let q = tex.query();
            let _ = backend.canvas.copy(tex, None, Rect::new(10, 20, q.width, q.height));
        }

        // Grid
        for y in 0..GRID_H {
            for x in 0..GRID_W {
                let px = x as i32 * TILE_SIZE;
                let py = y as i32 * TILE_SIZE + HUD_H;
                let state = grid.state[y][x];
                let is_mine = grid.mines[y][x];

                match state {
                    TileState::Hidden => {
                        backend.canvas.set_draw_color(Color::RGB(80, 80, 100));
                        let _ = backend.canvas.fill_rect(Rect::new(px + 1, py + 1, TILE_SIZE as u32 - 2, TILE_SIZE as u32 - 2));
                        // Borde 3D
                        backend.canvas.set_draw_color(Color::RGB(120, 120, 140));
                        let _ = backend.canvas.draw_rect(Rect::new(px + 1, py + 1, TILE_SIZE as u32 - 2, TILE_SIZE as u32 - 2));
                    }
                    TileState::Revealed => {
                        if is_mine && grid.game_over {
                            backend.canvas.set_draw_color(Color::RGB(200, 50, 50));
                            let _ = backend.canvas.fill_rect(Rect::new(px + 1, py + 1, TILE_SIZE as u32 - 2, TILE_SIZE as u32 - 2));
                            // 💣 emoji como círculo rojo
                            backend.canvas.set_draw_color(Color::RGB(255, 255, 255));
                            let _ = backend.canvas.fill_rect(Rect::new(px + 12, py + 12, 12, 12));
                        } else {
                            backend.canvas.set_draw_color(Color::RGB(50, 50, 65));
                            let _ = backend.canvas.fill_rect(Rect::new(px + 1, py + 1, TILE_SIZE as u32 - 2, TILE_SIZE as u32 - 2));

                            // Número
                            let n = grid.numbers[y][x];
                            if n > 0 {
                                let colores = [
                                    Color::RGB(0, 150, 255),   // 1 - azul
                                    Color::RGB(0, 180, 0),      // 2 - verde
                                    Color::RGB(255, 50, 50),    // 3 - rojo
                                    Color::RGB(0, 0, 150),      // 4 - azul oscuro
                                    Color::RGB(150, 0, 0),      // 5 - rojo oscuro
                                    Color::RGB(0, 150, 150),    // 6 - cyan
                                    Color::RGB(0, 0, 0),        // 7 - negro
                                    Color::RGB(128, 128, 128),  // 8 - gris
                                ];
                                let c = colores[(n - 1) as usize];
                                backend.canvas.set_draw_color(c);
                                let _ = backend.canvas.fill_rect(Rect::new(px + 14, py + 14, 8, 8));
                            }
                        }
                    }
                    TileState::Flagged => {
                        backend.canvas.set_draw_color(Color::RGB(80, 80, 100));
                        let _ = backend.canvas.fill_rect(Rect::new(px + 1, py + 1, TILE_SIZE as u32 - 2, TILE_SIZE as u32 - 2));
                        // 🚩 bandera = triángulo rojo
                        backend.canvas.set_draw_color(Color::RGB(255, 50, 50));
                        let _ = backend.canvas.fill_rect(Rect::new(px + 14, py + 8, 4, 20));
                        let _ = backend.canvas.fill_rect(Rect::new(px + 14, py + 8, 14, 10));
                    }
                }
            }
        }

        // Mensaje game over / win
        if let Some(ref tex) = txt_msg {
            let q = tex.query();
            let _ = backend.canvas.copy(tex, None, Rect::new(
                (SCREEN_W - q.width as i32) / 2,
                (SCREEN_H - q.height as i32) / 2,
                q.width,
                q.height,
            ));
        }

        backend.canvas.present();
    }

    println!("\n✅ Buscaminas cerrado");
    Ok(())
}
