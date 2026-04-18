// crates/ry-rs/src/rybot_stub.rs
//! RyBot stub — Motor central movido a crate independiente (crates/rybot/)
//! Este stub mantiene compatibilidad mientras se completa la migración.

#[derive(Default)]
pub struct RyBot;

impl RyBot {
    pub fn new() -> Self { Self }
    pub fn info(&self, _t: &str, _m: &str) {}
    pub fn begin_frame(&mut self) {}
    pub fn end_frame(&mut self, _ft: f32) {}
    pub fn record_render(&mut self, _ft: f32) {}
    pub fn set_entity_count(&mut self, _n: usize) {}
    pub fn save_status(&self, _p: &str) {}
    pub fn check_unused_modules(&self) {}
}
