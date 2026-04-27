use raylib::ffi::{RenderTexture2D, LoadRenderTexture, UnloadRenderTexture, BeginTextureMode, EndTextureMode, ClearBackground, Color};
use std::collections::HashMap;

pub struct ViewportManager {
    pub viewports: HashMap<String, Viewport>,
}

impl ViewportManager {
    pub fn new() -> Self {
        Self { viewports: HashMap::new() }
    }

    pub fn add_viewport(&mut self, id: &str, width: i32, height: i32) {
        self.viewports.insert(id.to_string(), Viewport::new(width, height));
    }
}

pub struct Viewport {
    pub target: RenderTexture2D,
    pub width: i32,
    pub height: i32,
    pub pan_x: f32,
    pub pan_y: f32,
    pub zoom: f32,
    pub grid_enabled: bool,
}

impl Viewport {
    pub fn new(width: i32, height: i32) -> Self {
        unsafe {
            let target = LoadRenderTexture(width, height);
            Self { 
                target, 
                width, 
                height,
                pan_x: 0.0,
                pan_y: 0.0,
                zoom: 1.0,
                grid_enabled: true,
            }
        }
    }

    pub fn begin(&self) {
        unsafe {
            BeginTextureMode(self.target);
            ClearBackground(Color { r: 50, g: 50, b: 60, a: 255 });
        }
    }

    pub fn end(&self) {
        unsafe {
            EndTextureMode();
        }
    }
}

impl Drop for Viewport {
    fn drop(&mut self) {
        unsafe {
            UnloadRenderTexture(self.target);
        }
    }
}
