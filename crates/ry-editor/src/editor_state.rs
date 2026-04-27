use crate::viewport_manager::ViewportManager;
use crate::viewport_controller::ViewportController;
use rybot::{RybotEngine, RybotGui};
use migui::Migui;
use ry_gfx::RyditGfx;

pub struct EditorState {
    pub gui: Migui,
    pub rybot_gui: RybotGui,
    pub engine: RybotEngine,
    pub viewports: ViewportManager,
    pub viewport_ctrl: ViewportController,
    pub is_running: bool,
}

impl EditorState {
    pub fn new() -> Self {
        let mut rybot_gui = RybotGui::new();
        rybot_gui.open = true;

        Self {
            gui: Migui::new(),
            rybot_gui,
            engine: RybotEngine::new(),
            viewports: ViewportManager::new(),
            viewport_ctrl: ViewportController::new(),
            is_running: true,
        }
    }

    /// Actualiza el estado del editor
    pub fn update(&mut self, _gfx: &mut RyditGfx) {
        // La lógica de actualización del motor vendría aquí
        // self.engine.update(0.016);
    }
}
