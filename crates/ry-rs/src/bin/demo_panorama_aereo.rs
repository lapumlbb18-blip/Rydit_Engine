//! Demo: Tilemap System v2.0 - Panorama Aéreo con AssetServer
//! 
//! Renderiza un mapa y carga un tileset usando la arquitectura AssetServer.

use ry_tilemap::{Tilemap, Tile};
use ry_loader::{AssetServer, AssetProvider};
use std::sync::Arc;

struct SpriteProvider;
impl AssetProvider for SpriteProvider {
    fn load_texture(&self, path: &str) -> Result<Vec<u8>, String> {
        // Simulamos carga de archivo real
        std::fs::read(path).map_err(|e| e.to_string())
    }
    fn load_audio(&self, _path: &str) -> Result<Vec<u8>, String> { Ok(vec![]) }
}

fn main() {
    let provider = Arc::new(SpriteProvider);
    let server = AssetServer::new(provider);
    
    let sprite_path = "logo_icon_asst/sprites/platform_16x16.png";
    println!("Intentando cargar asset: {}", sprite_path);
    
    match server.load_texture(sprite_path) {
        Ok(data) => println!("¡Éxito! Asset cargado. Tamaño: {} bytes", data.len()),
        Err(e) => println!("Error cargando asset: {}", e),
    }

    // Configuración del mundo
    let mut map = Tilemap::new(100, 50, 16); // Usamos 16 por el sprite
    map.fill_rect(5, 40, 20, 2, 1);
    
    let camera_x = 0.0;
    let camera_y = 0.0;
    let commands = map.draw_with_camera(camera_x, camera_y, 800, 600);
    
    println!("Renderizando {} tiles con sprite cargado", commands.len());
}
