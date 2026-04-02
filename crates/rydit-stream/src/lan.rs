// crates/rydit-stream/src/lan.rs
// 🆕 mDNS discovery para LAN (stub para v0.1.0)
// Nota: Implementación completa en próxima versión


/// Servicio de streaming RyDit en LAN
pub struct StreamService {
    name: String,
    #[allow(dead_code)]
    port: u16,
}

impl StreamService {
    /// Crear servicio mDNS (stub)
    pub fn new(name: &str, port: u16) -> Result<Self, String> {
        eprintln!(
            "[mDNS] Service registered (stub): {} on port {}",
            name, port
        );

        Ok(Self {
            name: name.to_string(),
            port,
        })
    }

    /// Buscar servicios RyDit en LAN (stub)
    pub fn discover() -> Result<Vec<ServiceInfo>, String> {
        // Stub: retornar vacío por ahora
        eprintln!("[mDNS] Discovery (stub) - no services found");
        Ok(Vec::new())
    }

    /// Detener servicio
    pub fn stop(&self) {
        eprintln!("[mDNS] Service stopped: {}", self.name);
    }
}

impl Drop for StreamService {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Información de servicio descubierto
#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub hostname: String,
    pub port: u16,
    pub addresses: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_new() {
        let service = StreamService::new("test", 8765);
        assert!(service.is_ok());
    }

    #[test]
    fn test_discover() {
        let result = StreamService::discover();
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}
