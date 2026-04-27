//! # Rybot — Motor Central de Ry-Dit
//!
//! Orquesta todos los crates del ecosistema Ry-Dit:
//! - **Input** → ry-input (input map configurable)
//! - **Físicas** → ry-physics (proyectiles, N-body, gravedad)
//! - **Animación** → ry-anim (12 principios Disney, action sprite)
//! - **Ciencia** → ry-science (Bezier, ondas, L-System, ilusiones)
//! - **Config** → ry-config (parser de entidades, niveles)
//! - **Render 2D** → ry-gfx (GPU instancing, FSR, transiciones)
//! - **Render 3D** → ry3d-gfx (primitivas, modelos, controles táctiles)
//! - **GUI** → migui (widgets, HUD, temas)
//! - **Red** → ry-stream (LAN streaming, WebSocket)
//!
//! ## Filosofía
//!
//! Rybot es como el **SceneTree de Godot** + **GDScript**:
//! es el sistema que mueve los hilos del motor.
//!
//! ```text
//! Rybot
//! ├── Escena activa
//! │   ├── Nodos (entidades, cámaras, luces)
//! │   ├── Subsistemas (input, físicas, animación, audio)
//! │   └── Recursos (texturas, sonidos, scripts)
//! ├── CLI → Crear proyectos, compilar, deploy
//! └── GUI → Inspector visual, propiedades, escena
//! ```
//!
//! ## Uso
//!
//! ```rust,ignore
//! use rybot::RybotEngine;
//!
//! let mut engine = RybotEngine::new();
//! engine.load_scene("mi_nivel.ryscene");
//! while engine.is_running() {
//!     engine.update();
//! }
//! ```

#![allow(missing_docs)]

use std::collections::HashMap;
use std::path::Path;

// ============================================================================
// SUBSISTEMAS
// ============================================================================

mod subsystems;

pub use subsystems::{
    InputSubsystem, PhysicsSubsystem, AnimationSubsystem, ScienceSubsystem,
    RenderSubsystem, NetworkSubsystem,
};

// ============================================================================
// SCENE TREE
// ============================================================================

mod scene_tree;

pub use scene_tree::{SceneTree, SceneNode, NodeType};

// ============================================================================
// PROJECT TEMPLATES
// ============================================================================

mod templates;

pub use templates::{ProjectTemplate, create_project, list_templates};

// ============================================================================
// CLI
// ============================================================================

pub mod cli;

// ============================================================================
// GUI
// ============================================================================

pub mod gui;

pub use gui::RybotGui;

// ============================================================================
// RYBOT ENGINE — El orquestador central
// ============================================================================

/// Estado del motor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineState {
    /// Inicializando
    Initializing,
    /// Corriendo normalmente
    Running,
    /// Pausado
    Paused,
    /// Cargando escena
    Loading,
    /// Apagando
    ShuttingDown,
}

/// El motor central de Ry-Dit
///
/// Conecta todos los crates y orquesta el game loop.
pub struct RybotEngine {
    /// Estado del motor
    state: EngineState,

    /// Árbol de escena activo
    scene: SceneTree,

    /// Subsistemas
    input: InputSubsystem,
    physics: PhysicsSubsystem,
    animation: AnimationSubsystem,
    science: ScienceSubsystem,
    render: RenderSubsystem,
    network: NetworkSubsystem,

    /// Config global
    config: HashMap<String, String>,

    /// FPS target
    target_fps: u32,

    /// Frame count
    frame: u64,
}

impl Default for RybotEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RybotEngine {
    /// Crear motor nuevo
    pub fn new() -> Self {
        Self {
            state: EngineState::Initializing,
            scene: SceneTree::new(),
            input: InputSubsystem::new(),
            physics: PhysicsSubsystem::new(),
            animation: AnimationSubsystem::new(),
            science: ScienceSubsystem::new(),
            render: RenderSubsystem::new(),
            network: NetworkSubsystem::new(),
            config: HashMap::new(),
            target_fps: 60,
            frame: 0,
        }
    }

    /// Iniciar el motor
    pub fn start(&mut self) {
        if self.state == EngineState::Initializing {
            self.state = EngineState::Running;
            println!("[RYBOT] Motor iniciado");
        }
    }

    /// Obtener referencia al árbol de escena
    pub fn scene(&self) -> &SceneTree {
        &self.scene
    }

    /// Obtener referencia mutable al árbol de escena
    pub fn scene_mut(&mut self) -> &mut SceneTree {
        &mut self.scene
    }

    /// Acceder al subsistema de input
    pub fn input(&self) -> &InputSubsystem {
        &self.input
    }

    /// Acceder al subsistema de input (mutable)
    pub fn input_mut(&mut self) -> &mut InputSubsystem {
        &mut self.input
    }

    /// Acceder al subsistema de físicas
    pub fn physics(&self) -> &PhysicsSubsystem {
        &self.physics
    }

    /// Acceder al subsistema de físicas (mutable)
    pub fn physics_mut(&mut self) -> &mut PhysicsSubsystem {
        &mut self.physics
    }

    /// Acceder al subsistema de animación
    pub fn animation(&self) -> &AnimationSubsystem {
        &self.animation
    }

    /// Acceder al subsistema de animación (mutable)
    pub fn animation_mut(&mut self) -> &mut AnimationSubsystem {
        &mut self.animation
    }

    /// Acceder al subsistema de ciencia
    pub fn science(&self) -> &ScienceSubsystem {
        &self.science
    }

    /// Acceder al subsistema de render
    pub fn render(&self) -> &RenderSubsystem {
        &self.render
    }

    /// Acceder al subsistema de red
    pub fn network(&self) -> &NetworkSubsystem {
        &self.network
    }

    /// Verificar si el motor está corriendo
    pub fn is_running(&self) -> bool {
        matches!(self.state, EngineState::Running)
    }

    /// Verificar si está pausado
    pub fn is_paused(&self) -> bool {
        matches!(self.state, EngineState::Paused)
    }

    /// Obtener frame actual
    pub fn frame(&self) -> u64 {
        self.frame
    }

    /// Obtener estado del motor
    pub fn state(&self) -> EngineState {
        self.state
    }

    /// Pausar el motor
    pub fn pause(&mut self) {
        if self.state == EngineState::Running {
            self.state = EngineState::Paused;
        }
    }

    /// Reanudar el motor
    pub fn resume(&mut self) {
        if self.state == EngineState::Paused {
            self.state = EngineState::Running;
        }
    }

    /// Cargar escena desde archivo
    pub fn load_scene(&mut self, path: &str) -> Result<(), String> {
        self.state = EngineState::Loading;

        let scene_path = Path::new(path);
        if !scene_path.exists() {
            self.state = EngineState::Running;
            return Err(format!("Escena '{}' no encontrada", path));
        }

        // Parsear escena
        let content = std::fs::read_to_string(scene_path)
            .map_err(|e| format!("Error leyendo escena: {}", e))?;

        self.scene = SceneTree::parse(&content)?;

        self.state = EngineState::Running;
        Ok(())
    }

    /// Actualizar un frame del motor
    pub fn update(&mut self, delta_time: f32) {
        if self.state != EngineState::Running {
            return;
        }

        self.frame += 1;

        // 1. Input
        self.input.update();

        // 2. Física
        if self.physics.enabled() {
            self.physics.update(delta_time);
        }

        // 3. Animación
        if self.animation.enabled() {
            self.animation.update(delta_time);
        }

        // 4. Ciencia (simulaciones)
        if self.science.enabled() {
            self.science.update(delta_time);
        }

        // 5. Red (streaming/multiplayer)
        if self.network.enabled() {
            self.network.update(delta_time);
        }

        // 6. Scene tree update
        self.scene.update(delta_time);

        // 7. Render (se delega al backend externo)
        self.render.update();
    }

    /// Apagar el motor
    pub fn shutdown(&mut self) {
        self.state = EngineState::ShuttingDown;
    }

    /// Configurar valor
    pub fn set_config(&mut self, key: &str, value: &str) {
        self.config.insert(key.to_string(), value.to_string());
    }

    /// Obtener config
    pub fn get_config(&self, key: &str) -> Option<&str> {
        self.config.get(key).map(|s| s.as_str())
    }

    /// Obtener todas las stats del motor
    pub fn get_stats(&self, pan_x: f32, pan_y: f32, zoom: f32) -> EngineStats {
        EngineStats {
            frame: self.frame,
            state: self.state,
            scene_nodes: self.scene.node_count(),
            input_actions: self.input.action_count(),
            physics_enabled: self.physics.enabled(),
            animation_enabled: self.animation.enabled(),
            network_enabled: self.network.enabled(),
            target_fps: self.target_fps,
            pan_x,
            pan_y,
            zoom,
        }
    }
}

/// Estadísticas del motor
#[derive(Debug, Clone)]
pub struct EngineStats {
    pub frame: u64,
    pub state: EngineState,
    pub scene_nodes: usize,
    pub input_actions: usize,
    pub physics_enabled: bool,
    pub animation_enabled: bool,
    pub network_enabled: bool,
    pub target_fps: u32,
    pub pan_x: f32,
    pub pan_y: f32,
    pub zoom: f32,
}

impl std::fmt::Display for EngineStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "🛡️ Rybot Engine Stats")?;
        writeln!(f, "  Frame: {}", self.frame)?;
        writeln!(f, "  Estado: {:?}", self.state)?;
        writeln!(f, "  Nodos en escena: {}", self.scene_nodes)?;
        writeln!(f, "  Acciones input: {}", self.input_actions)?;
        writeln!(f, "  Físicas: {}", if self.physics_enabled { "✅" } else { "❌" })?;
        writeln!(f, "  Animación: {}", if self.animation_enabled { "✅" } else { "❌" })?;
        writeln!(f, "  Red: {}", if self.network_enabled { "✅" } else { "❌" })?;
        writeln!(f, "  Target FPS: {}", self.target_fps)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = RybotEngine::new();
        assert!(engine.is_running());
        assert_eq!(engine.frame(), 0);
        assert_eq!(engine.scene().node_count(), 1); // root node
    }

    #[test]
    fn test_pause_resume() {
        let mut engine = RybotEngine::new();
        assert!(engine.is_running());

        engine.pause();
        assert!(engine.is_paused());

        engine.resume();
        assert!(engine.is_running());
    }

    #[test]
    fn test_update_increments_frame() {
        let mut engine = RybotEngine::new();
        assert_eq!(engine.frame(), 0);

        engine.update(0.016);
        assert_eq!(engine.frame(), 1);

        engine.update(0.016);
        assert_eq!(engine.frame(), 2);
    }

    #[test]
    fn test_update_paused_does_nothing() {
        let mut engine = RybotEngine::new();
        engine.pause();

        let frame_before = engine.frame();
        engine.update(0.016);
        assert_eq!(engine.frame(), frame_before);
    }

    #[test]
    fn test_shutdown() {
        let mut engine = RybotEngine::new();
        engine.shutdown();
        assert!(!engine.is_running());
        assert!(!engine.is_paused());
    }

    #[test]
    fn test_config() {
        let mut engine = RybotEngine::new();
        engine.set_config("window_width", "800");
        engine.set_config("window_height", "600");

        assert_eq!(engine.get_config("window_width"), Some("800"));
        assert_eq!(engine.get_config("window_height"), Some("600"));
        assert_eq!(engine.get_config("nonexistent"), None);
    }

    #[test]
    fn test_stats() {
        let mut engine = RybotEngine::new();
        engine.update(0.016);

        let stats = engine.get_stats();
        assert_eq!(stats.frame, 1);
        assert!(matches!(stats.state, EngineState::Running));
        assert_eq!(stats.target_fps, 60);
    }

    #[test]
    fn test_stats_display() {
        let engine = RybotEngine::new();
        let stats = engine.get_stats();
        let output = format!("{}", stats);
        assert!(output.contains("Rybot Engine Stats"));
        assert!(output.contains("Frame: 0"));
    }

    #[test]
    fn test_load_scene_nonexistent() {
        let mut engine = RybotEngine::new();
        let result = engine.load_scene("archivo_inexistente.ryscene");
        assert!(result.is_err());
    }

    #[test]
    fn test_project_templates_available() {
        let templates = list_templates();
        assert!(!templates.is_empty());
        assert!(templates.iter().any(|t| t.name() == "game2d"));
        assert!(templates.iter().any(|t| t.name() == "game3d"));
    }
}
