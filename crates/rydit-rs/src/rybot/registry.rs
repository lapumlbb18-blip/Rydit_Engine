// crates/rydit-rs/src/rybot/registry.rs
// RyBot Registry - Registro central de módulos y eventos
// v0.11.0 - Fusión RyBot + RyditModule

use std::collections::HashMap;
use std::time::Instant;

// ============================================================================
// TIPOS BÁSICOS
// ============================================================================

/// ID único de entidad
pub type EntityId = u32;

/// Timestamp en nanosegundos
pub type Timestamp = u64;

/// Fuente de un evento
#[derive(Debug, Clone)]
pub enum EventSource {
    Parser,
    Script(String),
    Module(String),
    Core,
    User,
}

/// Valor dinámico (compatible con lizer)
#[derive(Debug, Clone)]
pub enum Valor {
    Num(f32),
    Texto(String),
    Bool(bool),
    Lista(Vec<Valor>),
    Mapa(HashMap<String, Valor>),
    Nada,
}

// ============================================================================
// EVENTO RYDIT
// ============================================================================

/// Evento registrado por RyBot
#[derive(Debug, Clone)]
pub struct RyditEvent {
    pub frame: u32,
    pub timestamp: Timestamp,
    pub source: EventSource,
    pub action: String,
    pub data: Option<Valor>,
}

impl RyditEvent {
    pub fn new(frame: u32, source: EventSource, action: &str, data: Option<Valor>) -> Self {
        Self {
            frame,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as Timestamp,
            source,
            action: action.to_string(),
            data,
        }
    }
}

// ============================================================================
// INFO DE MÓDULO
// ============================================================================

/// Información de un módulo registrado
#[derive(Debug)]
pub struct ModuleInfo {
    pub name: String,
    pub version: String,
    pub enabled: bool,
    pub update_time_ms: f32,
    pub render_time_ms: f32,
    pub metadata: HashMap<String, String>,
}

impl ModuleInfo {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            enabled: true,
            update_time_ms: 0.0,
            render_time_ms: 0.0,
            metadata: HashMap::new(),
        }
    }
}

// ============================================================================
// MÉTRICAS
// ============================================================================

/// Métricas de rendimiento
#[derive(Debug, Default)]
pub struct Metrics {
    pub fps: f32,
    pub frame_time_ms: f32,
    pub parse_time_ms: f32,
    pub eval_time_ms: f32,
    pub render_time_ms: f32,
    pub entity_count: usize,
    pub module_count: usize,
    pub event_count: usize,
    pub memory_mb: f32,
}

impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Calcular FPS desde frame_time
    pub fn calc_fps(&mut self, frame_time_ms: f32) {
        self.frame_time_ms = frame_time_ms;
        self.fps = if frame_time_ms > 0.0 {
            1000.0 / frame_time_ms
        } else {
            0.0
        };
    }
}

// ============================================================================
// REGISTRY
// ============================================================================

/// Registry central de RyBot
pub struct Registry {
    /// Módulos registrados
    modules: HashMap<String, ModuleInfo>,
    
    /// Eventos recientes (circular buffer)
    events: Vec<RyditEvent>,
    max_events: usize,
    
    /// Métricas actuales
    metrics: Metrics,
    
    /// Frame actual
    current_frame: u32,
    
    /// Instante de inicio
    start_time: Instant,
}

impl Registry {
    /// Crear nuevo registry
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            events: Vec::with_capacity(1000),
            max_events: 1000,
            metrics: Metrics::new(),
            current_frame: 0,
            start_time: Instant::now(),
        }
    }
    
    // ==================== MÓDULOS ====================
    
    /// Registrar módulo
    pub fn register_module(&mut self, name: &str, version: &str) {
        let info = ModuleInfo::new(name, version);
        self.modules.insert(name.to_string(), info);
        self.metrics.module_count = self.modules.len();
        
        self.log_event(
            EventSource::Core,
            "module_registered",
            Some(Valor::Texto(format!("{} v{}", name, version))),
        );
    }
    
    /// Obtener info de módulo
    pub fn get_module(&self, name: &str) -> Option<&ModuleInfo> {
        self.modules.get(name)
    }
    
    /// Obtener info mutable de módulo
    pub fn get_module_mut(&mut self, name: &str) -> Option<&mut ModuleInfo> {
        self.modules.get_mut(name)
    }
    
    /// Listar módulos
    pub fn list_modules(&self) -> Vec<&ModuleInfo> {
        self.modules.values().collect()
    }
    
    /// Habilitar/deshabilitar módulo
    pub fn set_module_enabled(&mut self, name: &str, enabled: bool) {
        if let Some(module) = self.modules.get_mut(name) {
            module.enabled = enabled;
        }
    }
    
    /// Actualizar tiempo de update de módulo
    pub fn record_module_update(&mut self, name: &str, time_ms: f32) {
        if let Some(module) = self.modules.get_mut(name) {
            module.update_time_ms = time_ms;
        }
    }
    
    /// Actualizar tiempo de render de módulo
    pub fn record_module_render(&mut self, name: &str, time_ms: f32) {
        if let Some(module) = self.modules.get_mut(name) {
            module.render_time_ms = time_ms;
        }
    }
    
    // ==================== EVENTOS ====================
    
    /// Loguear evento
    pub fn log_event(&mut self, source: EventSource, action: &str, data: Option<Valor>) {
        let event = RyditEvent::new(self.current_frame, source, action, data);
        
        self.events.push(event);
        self.metrics.event_count = self.events.len();
        
        // Mantener buffer circular
        if self.events.len() > self.max_events {
            self.events.remove(0);
        }
    }
    
    /// Obtener eventos recientes
    pub fn get_events(&self, limit: usize) -> Vec<&RyditEvent> {
        self.events.iter().rev().take(limit).collect()
    }
    
    /// Obtener eventos filtrados por fuente
    pub fn get_events_by_source(&self, source: &EventSource) -> Vec<&RyditEvent> {
        self.events
            .iter()
            .filter(|e| matches!((&e.source, source), (EventSource::Module(a), EventSource::Module(b)) if a == b))
            .collect()
    }
    
    /// Limpiar eventos
    pub fn clear_events(&mut self) {
        self.events.clear();
        self.metrics.event_count = 0;
    }
    
    // ==================== MÉTRICAS ====================
    
    /// Actualizar FPS
    pub fn update_fps(&mut self, frame_time_ms: f32) {
        self.metrics.calc_fps(frame_time_ms);
    }
    
    /// Obtener métricas
    pub fn get_metrics(&self) -> &Metrics {
        &self.metrics
    }
    
    /// Actualizar contador de entidades
    pub fn set_entity_count(&mut self, count: usize) {
        self.metrics.entity_count = count;
    }
    
    /// Registrar tiempo de parse
    pub fn record_parse_time(&mut self, time_ms: f32) {
        self.metrics.parse_time_ms = time_ms;
    }
    
    /// Registrar tiempo de eval
    pub fn record_eval_time(&mut self, time_ms: f32) {
        self.metrics.eval_time_ms = time_ms;
    }
    
    /// Registrar tiempo de render
    pub fn record_render_time(&mut self, time_ms: f32) {
        self.metrics.render_time_ms = time_ms;
    }
    
    // ==================== FRAME ====================
    
    /// Incrementar frame
    pub fn next_frame(&mut self) {
        self.current_frame += 1;
    }
    
    /// Obtener frame actual
    pub fn current_frame(&self) -> u32 {
        self.current_frame
    }
    
    /// Obtener tiempo de ejecución
    pub fn uptime(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }
    
    // ==================== CLI EXPORT ====================
    
    /// Exportar estado para CLI
    pub fn export_status(&self) -> String {
        let mut output = String::new();
        
        output.push_str(&format!("=== RyBot Status ===\n"));
        output.push_str(&format!("Frame: {}\n", self.current_frame));
        output.push_str(&format!("Uptime: {:?}\n", self.uptime()));
        output.push_str(&format!("\n"));
        
        output.push_str(&format!("=== Metrics ===\n"));
        output.push_str(&format!("FPS: {:.1}\n", self.metrics.fps));
        output.push_str(&format!("Frame time: {:.2}ms\n", self.metrics.frame_time_ms));
        output.push_str(&format!("Parse time: {:.2}ms\n", self.metrics.parse_time_ms));
        output.push_str(&format!("Eval time: {:.2}ms\n", self.metrics.eval_time_ms));
        output.push_str(&format!("Render time: {:.2}ms\n", self.metrics.render_time_ms));
        output.push_str(&format!("Entities: {}\n", self.metrics.entity_count));
        output.push_str(&format!("Modules: {}\n", self.metrics.module_count));
        output.push_str(&format!("Events: {}\n", self.metrics.event_count));
        output.push_str(&format!("\n"));
        
        output.push_str(&format!("=== Modules ===\n"));
        for module in self.list_modules() {
            let status = if module.enabled { "✓" } else { "✗" };
            output.push_str(&format!(
                "  {} {} v{} (update: {:.2}ms, render: {:.2}ms)\n",
                status, module.name, module.version, module.update_time_ms, module.render_time_ms
            ));
        }
        
        output
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}
