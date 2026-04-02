// crates/rydit-stream/src/lib.rs
// 🆕 LAN streaming for RyDit Engine

pub mod client;
pub mod server;
pub mod lan;
pub mod protocol;
pub mod portal;

// Re-exports públicos
pub use client::StreamClient;
pub use server::StreamServer;
pub use lan::{StreamService, ServiceInfo};
pub use portal::WebPortal;
pub use protocol::{RpcMessage, stream};

/// Versión del crate
pub const VERSION: &str = "0.1.0";

/// Iniciar streaming completo (server + portal)
pub fn start_streaming(stream_addr: &str, portal_port: u16) -> Result<(StreamServer, WebPortal), String> {
    let server = StreamServer::new(stream_addr);
    server.start();

    let mut portal = WebPortal::new(portal_port);
    portal.start();

    eprintln!("[STREAM] Server: {}", stream_addr);
    eprintln!("[PORTAL] Web: http://localhost:{}", portal_port);

    Ok((server, portal))
}

/// Registrar servicio mDNS
pub fn register_service(name: &str, port: u16) -> Result<StreamService, String> {
    StreamService::new(name, port)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_start_streaming() {
        let result = start_streaming("ws://127.0.0.1:0", 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_service() {
        let result = register_service("test", 8765);
        assert!(result.is_ok());
    }
}
