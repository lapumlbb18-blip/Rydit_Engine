// crates/rydit-stream/src/server.rs
// 🆕 WebSocket server simple con threads (sin tokio)

use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use tungstenite::{accept, Error, Message};

/// Cliente conectado al servidor
pub struct Client {
    ws: tungstenite::WebSocket<TcpStream>,
    addr: String,
}

/// Servidor de streaming RyDit
pub struct StreamServer {
    clients: Arc<Mutex<Vec<Client>>>,
    running: Arc<Mutex<bool>>,
    addr: String,
}

impl StreamServer {
    /// Crear nuevo servidor
    pub fn new(addr: &str) -> Self {
        Self {
            clients: Arc::new(Mutex::new(Vec::new())),
            running: Arc::new(Mutex::new(false)),
            addr: addr.to_string(),
        }
    }

    /// Iniciar servidor (thread separado)
    pub fn start(&self) {
        let mut running = self.running.lock().unwrap();
        *running = true;
        drop(running);

        let clients = self.clients.clone();
        let running = self.running.clone();
        let addr = self.addr.clone();

        thread::spawn(move || {
            let listener = TcpListener::bind(&addr).expect("Failed to bind");
            eprintln!("[STREAM SERVER] Listening on {}", addr);

            for stream in listener.incoming() {
                if !*running.lock().unwrap() {
                    break;
                }

                match stream {
                    Ok(stream) => match accept(stream) {
                        Ok(ws) => {
                            let addr = ws.get_ref().peer_addr().unwrap().to_string();
                            eprintln!("[STREAM] Client connected: {}", addr);

                            let mut clients = clients.lock().unwrap();
                            clients.push(Client { ws, addr });
                        }
                        Err(e) => eprintln!("[STREAM] WebSocket error: {}", e),
                    },
                    Err(e) => eprintln!("[STREAM] Connection error: {}", e),
                }
            }
        });
    }

    /// Broadcast a todos los clientes
    pub fn broadcast(&self, data: &str) -> Result<(), String> {
        let mut clients = self.clients.lock().unwrap();
        let mut to_remove = Vec::new();

        for (i, client) in clients.iter_mut().enumerate() {
            match client.ws.send(Message::Text(data.into())) {
                Ok(_) => {}
                Err(Error::Io(_)) => {
                    to_remove.push(i);
                }
                Err(e) => eprintln!("[STREAM] Send error: {}", e),
            }
        }

        // Remover clientes desconectados
        for i in to_remove.into_iter().rev() {
            let client = clients.remove(i);
            eprintln!("[STREAM] Client disconnected: {}", client.addr);
        }

        Ok(())
    }

    /// Broadcast JSON-RPC
    pub fn broadcast_rpc(
        &self,
        method: &str,
        params: serde_json::Value,
        id: u64,
    ) -> Result<(), String> {
        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id,
        });
        self.broadcast(&msg.to_string())
    }

    /// Detener servidor
    pub fn stop(&self) {
        let mut running = self.running.lock().unwrap();
        *running = false;
    }

    /// Número de clientes conectados
    pub fn client_count(&self) -> usize {
        self.clients.lock().unwrap().len()
    }

    /// Verificar si está corriendo
    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }
}

impl Drop for StreamServer {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_new() {
        let server = StreamServer::new("ws://127.0.0.1:0");
        assert!(!server.is_running());
        assert_eq!(server.client_count(), 0);
    }

    #[test]
    fn test_server_start_stop() {
        let server = StreamServer::new("ws://127.0.0.1:0");
        server.start();
        assert!(server.is_running());
        server.stop();
        assert!(!server.is_running());
    }
}
