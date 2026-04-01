// crates/rydit-rs/src/rybot/mod.rs
// RyBot - Inspector, Registry y CLI para RyDit
// v0.11.0 - Fusión RyBot + RyditModule + Alertas

pub mod registry;

pub use registry::*;

// ============================================================================
// RYBOT CORE
// ============================================================================

use std::collections::HashMap;

/// RyBot - Inspector y Registry central
pub struct RyBot {
    /// Registry de módulos y eventos
    pub registry: Registry,
    
    /// ¿RyBot está activo?
    is_open: bool,
    
    /// Modo: CLI, UI, o Ambos
    mode: RyBotMode,
}

/// Modo de RyBot
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RyBotMode {
    CLI,
    UI,
    Both,
    Off,
}

impl RyBot {
    /// Crear nuevo RyBot
    pub fn new() -> Self {
        Self {
            registry: Registry::new(),
            is_open: true,
            mode: RyBotMode::Both,
        }
    }
    
    /// Iniciar frame RyBot
    pub fn begin_frame(&mut self) {
        self.registry.next_frame();
    }
    
    /// Finalizar frame RyBot
    pub fn end_frame(&mut self, frame_time_ms: f32) {
        self.registry.update_fps(frame_time_ms);
    }
    
    /// ¿RyBot está activo?
    pub fn is_open(&self) -> bool {
        self.is_open
    }
    
    /// Abrir/cerrar RyBot
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }
    
    /// Setear modo
    pub fn set_mode(&mut self, mode: RyBotMode) {
        self.mode = mode;
    }
    
    /// Obtener modo
    pub fn mode(&self) -> RyBotMode {
        self.mode
    }
    
    // ==================== REGISTRY WRAPPERS ====================
    
    /// Registrar módulo
    pub fn register_module(&mut self, name: &str, version: &str) {
        self.registry.register_module(name, version);
    }
    
    /// Loguear evento
    pub fn log(&mut self, source: &str, action: &str, data: Option<Valor>) {
        let event_source = match source {
            "parser" => EventSource::Parser,
            "core" => EventSource::Core,
            "user" => EventSource::User,
            _ => EventSource::Module(source.to_string()),
        };
        
        self.registry.log_event(event_source, action, data);
    }
    
    /// Obtener estado para CLI
    pub fn status(&self) -> String {
        self.registry.export_status()
    }
    
    // ==================== MÉTRICAS ====================
    
    /// Registrar tiempo de parse
    pub fn record_parse(&mut self, time_ms: f32) {
        self.registry.record_parse_time(time_ms);
        self.log("parser", "parse_complete", Some(Valor::Num(time_ms)));
    }
    
    /// Registrar tiempo de eval
    pub fn record_eval(&mut self, time_ms: f32) {
        self.registry.record_eval_time(time_ms);
    }
    
    /// Registrar tiempo de render
    pub fn record_render(&mut self, time_ms: f32) {
        self.registry.record_render_time(time_ms);
    }
    
    /// Actualizar contador de entidades
    pub fn set_entity_count(&mut self, count: usize) {
        self.registry.set_entity_count(count);
    }
    
    // ==================== ALERTAS ====================
    
    /// Alerta de información
    pub fn info(&mut self, source: &str, message: &str) {
        self.registry.info(source, message);
    }
    
    /// Alerta de warning (no bloqueante)
    pub fn warn(&mut self, source: &str, message: &str) {
        self.registry.warn(source, message);
    }
    
    /// Alerta de error (no bloqueante)
    pub fn error(&mut self, source: &str, message: &str) {
        self.registry.error(source, message);
    }
    
    /// Reportar import no usado
    pub fn report_unused_import(&mut self, module: &str, import: &str) {
        self.registry.report_unused_import(module, import);
    }
    
    /// Verificar módulos no usados
    pub fn check_unused_modules(&mut self) {
        self.registry.check_unused_modules();
    }
    
    /// Obtener estado de módulo
    pub fn get_module_state(&self, name: &str) -> Option<ModuleState> {
        self.registry.get_module(name).map(|m| m.state)
    }
    
    /// Obtener alertas activas
    pub fn get_alerts(&self) -> Vec<&RyBotAlert> {
        self.registry.get_alerts()
    }
    
    /// Contar alertas
    pub fn alert_count(&self) -> usize {
        self.registry.alert_count()
    }
}

impl Default for RyBot {
    fn default() -> Self {
        Self::new()
    }
}
