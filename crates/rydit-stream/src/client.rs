// crates/rydit-stream/src/client.rs
// 🆕 WebSocket client para streaming RyDit

use tungstenite::{connect, Message, WebSocket, Error, stream::MaybeTlsStream};
use std::net::TcpStream;

/// Cliente de streaming RyDit
pub struct StreamClient {
    ws: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    url: String,
}

impl StreamClient {
    /// Crear nuevo cliente
    pub fn new() -> Self {
        Self {
            ws: None,
            url: String::new(),
        }
    }

    /// Conectar a servidor
    pub fn connect(&mut self, url: &str) -> Result<(), String> {
        match connect(url) {
            Ok((ws, _)) => {
                self.ws = Some(ws);
                self.url = url.to_string();
                eprintln!("[STREAM CLIENT] Connected to {}", url);
                Ok(())
            }
            Err(e) => Err(format!("Connection error: {}", e)),
        }
    }

    /// Enviar mensaje
    pub fn send(&mut self, data: &str) -> Result<(), String> {
        if let Some(ws) = &mut self.ws {
            ws.send(Message::Text(data.into()))
                .map_err(|e| format!("Send error: {}", e))?;
            Ok(())
        } else {
            Err("Not connected".to_string())
        }
    }

    /// Recibir mensaje
    pub fn recv(&mut self) -> Result<String, String> {
        if let Some(ws) = &mut self.ws {
            match ws.read() {
                Ok(Message::Text(text)) => Ok(text),
                Ok(Message::Binary(data)) => Ok(format!("[binary] {} bytes", data.len())),
                Ok(Message::Ping(_)) | Ok(Message::Pong(_)) => self.recv(), // Auto-handle ping/pong
                Ok(Message::Close(_)) => Err("Connection closed".to_string()),
                Ok(Message::Frame(_)) => self.recv(),
                Err(Error::Io(_)) => Err("Connection lost".to_string()),
                Err(e) => Err(format!("Recv error: {}", e)),
            }
        } else {
            Err("Not connected".to_string())
        }
    }

    /// Enviar JSON-RPC request
    pub fn send_rpc(&mut self, method: &str, params: serde_json::Value, id: u64) -> Result<(), String> {
        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id,
        });
        self.send(&msg.to_string())
    }

    /// Desconectar
    pub fn disconnect(&mut self) {
        self.ws = None;
        self.url.clear();
    }

    /// Verificar si está conectado
    pub fn is_connected(&self) -> bool {
        self.ws.is_some()
    }

    /// Obtener URL
    pub fn url(&self) -> &str {
        &self.url
    }
}

impl Default for StreamClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let client = StreamClient::new();
        assert!(!client.is_connected());
    }

    #[test]
    fn test_client_disconnect() {
        let mut client = StreamClient::new();
        client.disconnect();
        assert!(!client.is_connected());
    }
}
