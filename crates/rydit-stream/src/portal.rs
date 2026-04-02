// crates/rydit-stream/src/portal.rs
// 🆕 Web portal embebido (HTML + HTTP server simple)

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};

/// Portal web embebido para RyDit Stream
pub struct WebPortal {
    html: &'static str,
    port: u16,
    running: bool,
}

impl WebPortal {
    /// Crear portal
    pub fn new(port: u16) -> Self {
        Self {
            html: include_str!("../resources/portal.html"),
            port,
            running: false,
        }
    }

    /// Iniciar servidor HTTP (thread separado)
    pub fn start(&mut self) {
        self.running = true;
        let port = self.port;
        let html = self.html;

        thread::spawn(move || {
            let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
                .expect("Failed to bind portal");
            
            eprintln!("[PORTAL] Serving at http://localhost:{}", port);

            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        let mut buffer = [0; 1024];
                        let _ = stream.read(&mut buffer);

                        // HTTP response simple
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            html.len(),
                            html
                        );

                        let _ = stream.write_all(response.as_bytes());
                        let _ = stream.flush();
                    }
                    Err(e) => eprintln!("[PORTAL] Connection error: {}", e),
                }
            }
        });
    }

    /// Detener portal
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Verificar si está corriendo
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Obtener puerto
    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Drop for WebPortal {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portal_new() {
        let portal = WebPortal::new(8080);
        assert_eq!(portal.port(), 8080);
        assert!(!portal.is_running());
    }

    #[test]
    fn test_portal_start_stop() {
        let mut portal = WebPortal::new(8080);
        portal.start();
        assert!(portal.is_running());
        portal.stop();
        assert!(!portal.is_running());
    }
}
