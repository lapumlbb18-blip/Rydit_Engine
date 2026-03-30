// crates/rydit-http/src/lib.rs
// HTTP + WebSocket para RyDit Engine
//
// HTTP Functions:
// - http::get(url) - GET request
// - http::post(url, data) - POST request
// - http::put(url, data) - PUT request
// - http::delete(url) - DELETE request
//
// WebSocket Functions:
// - ws::connect(url) - Conectar a WebSocket
// - ws::send(message) - Enviar mensaje
// - ws::recv() - Recibir mensaje
// - ws::close() - Cerrar conexión
// - ws::is_connected() - Verificar estado

use std::sync::{Arc, Mutex};

// ============================================================================
// ESTADO GLOBAL DE WEBSOCKET
// ============================================================================

/// Estado de la conexión WebSocket
pub struct WebSocketState {
    socket:
        Option<tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>>,
    url: String,
    connected: bool,
}

impl WebSocketState {
    pub fn new() -> Self {
        Self {
            socket: None,
            url: String::new(),
            connected: false,
        }
    }
}

impl Default for WebSocketState {
    fn default() -> Self {
        Self::new()
    }
}

// Estado global thread-local para WebSocket
thread_local! {
    static WS_STATE: Arc<Mutex<WebSocketState>> = Arc::new(Mutex::new(WebSocketState::new()));
}

// ============================================================================
// HTTP FUNCTIONS
// ============================================================================

/// http::get(url) - Realizar GET request
pub fn http_get(url: &str) -> Result<String, String> {
    match ureq::get(url).call() {
        Ok(response) => match response.into_string() {
            Ok(body) => Ok(body),
            Err(e) => Err(format!("Error leyendo respuesta: {}", e)),
        },
        Err(e) => Err(format!("Error en GET request: {}", e)),
    }
}

/// http::post(url, data) - Realizar POST request con JSON
pub fn http_post(url: &str, data: &str) -> Result<String, String> {
    match ureq::post(url)
        .set("Content-Type", "application/json")
        .send_string(data)
    {
        Ok(response) => match response.into_string() {
            Ok(body) => Ok(body),
            Err(e) => Err(format!("Error leyendo respuesta: {}", e)),
        },
        Err(e) => Err(format!("Error en POST request: {}", e)),
    }
}

/// http::put(url, data) - Realizar PUT request con JSON
pub fn http_put(url: &str, data: &str) -> Result<String, String> {
    match ureq::put(url)
        .set("Content-Type", "application/json")
        .send_string(data)
    {
        Ok(response) => match response.into_string() {
            Ok(body) => Ok(body),
            Err(e) => Err(format!("Error leyendo respuesta: {}", e)),
        },
        Err(e) => Err(format!("Error en PUT request: {}", e)),
    }
}

/// http::delete(url) - Realizar DELETE request
pub fn http_delete(url: &str) -> Result<String, String> {
    match ureq::delete(url).call() {
        Ok(response) => match response.into_string() {
            Ok(body) => Ok(body),
            Err(e) => Err(format!("Error leyendo respuesta: {}", e)),
        },
        Err(e) => Err(format!("Error en DELETE request: {}", e)),
    }
}

// ============================================================================
// WEBSOCKET FUNCTIONS
// ============================================================================

/// ws::connect(url) - Conectar a servidor WebSocket
pub fn ws_connect(url: &str) -> Result<String, String> {
    WS_STATE.with(|ws_state| {
        let mut state = ws_state.lock().unwrap();

        // Si ya está conectado, cerrar primero
        if state.connected {
            let _ = ws_disconnect_internal(&mut state);
        }

        // Conectar
        match tungstenite::connect(url) {
            Ok((socket, _response)) => {
                state.socket = Some(socket);
                state.url = url.to_string();
                state.connected = true;
                Ok(format!("ws::connect() - Conectado a '{}'", url))
            }
            Err(e) => Err(format!("ws::connect() Error conectando a '{}': {}", url, e)),
        }
    })
}

/// Función interna para desconectar
fn ws_disconnect_internal(state: &mut WebSocketState) -> Result<String, String> {
    if let Some(mut socket) = state.socket.take() {
        let _ = socket.close(None);
    }
    state.connected = false;
    state.url.clear();
    Ok("ws::disconnect() - Desconectado".to_string())
}

/// ws::disconnect() - Desconectar de WebSocket
pub fn ws_disconnect() -> Result<String, String> {
    WS_STATE.with(|ws_state| {
        let mut state = ws_state.lock().unwrap();
        ws_disconnect_internal(&mut state)
    })
}

/// ws::send(message) - Enviar mensaje a WebSocket
pub fn ws_send(message: &str) -> Result<String, String> {
    WS_STATE.with(|ws_state| {
        let mut state = ws_state.lock().unwrap();

        if !state.connected {
            return Err(
                "ws::send() No hay conexión WebSocket. Usa ws::connect() primero.".to_string(),
            );
        }

        if let Some(ref mut socket) = state.socket {
            match socket.send(tungstenite::Message::Text(message.to_string())) {
                Ok(_) => Ok(format!(
                    "ws::send() - Mensaje enviado ({} bytes)",
                    message.len()
                )),
                Err(e) => {
                    state.connected = false;
                    Err(format!("ws::send() Error enviando: {}", e))
                }
            }
        } else {
            Err("ws::send() Socket no disponible".to_string())
        }
    })
}

/// ws::recv() - Recibir mensaje de WebSocket
pub fn ws_recv() -> Result<String, String> {
    WS_STATE.with(|ws_state| {
        let mut state = ws_state.lock().unwrap();

        if !state.connected {
            return Err(
                "ws::recv() No hay conexión WebSocket. Usa ws::connect() primero.".to_string(),
            );
        }

        if let Some(ref mut socket) = state.socket {
            match socket.read() {
                Ok(msg) => match msg {
                    tungstenite::Message::Text(text) => Ok(text),
                    tungstenite::Message::Binary(data) => {
                        Ok(format!("[BINARY] {} bytes", data.len()))
                    }
                    tungstenite::Message::Ping(_) => Ok("[PING] recibido".to_string()),
                    tungstenite::Message::Pong(_) => Ok("[PONG] recibido".to_string()),
                    tungstenite::Message::Close(_) => {
                        state.connected = false;
                        Ok("[CLOSE] Conexión cerrada por el servidor".to_string())
                    }
                    _ => Ok("[UNKNOWN] Mensaje desconocido".to_string()),
                },
                Err(e) => {
                    state.connected = false;
                    Err(format!("ws::recv() Error recibiendo: {}", e))
                }
            }
        } else {
            Err("ws::recv() Socket no disponible".to_string())
        }
    })
}

/// ws::is_connected() - Verificar si hay conexión activa
pub fn ws_is_connected() -> bool {
    WS_STATE.with(|ws_state| {
        let state = ws_state.lock().unwrap();
        state.connected
    })
}

/// ws::get_url() - Obtener URL de conexión actual
pub fn ws_get_url() -> Option<String> {
    WS_STATE.with(|ws_state| {
        let state = ws_state.lock().unwrap();
        if state.connected {
            Some(state.url.clone())
        } else {
            None
        }
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_get_invalid_url() {
        // Test con URL inválida
        let result = http_get("http://url-invalida-que-no-existe-12345.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_http_post_invalid_url() {
        // Test con URL inválida
        let result = http_post("http://url-invalida-que-no-existe-12345.com", "{}");
        assert!(result.is_err());
    }

    #[test]
    fn test_ws_initial_state() {
        // WebSocket debería estar desconectado inicialmente
        assert!(!ws_is_connected());
        assert_eq!(ws_get_url(), None);
    }

    #[test]
    fn test_ws_recv_without_connect() {
        // Intentar recibir sin conectar debería fallar
        let result = ws_recv();
        assert!(result.is_err());
    }

    #[test]
    fn test_ws_send_without_connect() {
        // Intentar enviar sin conectar debería fallar
        let result = ws_send("test");
        assert!(result.is_err());
    }

    #[test]
    fn test_http_functions_exist() {
        // Verificar que las funciones existen
        let _ = http_get;
        let _ = http_post;
        let _ = http_put;
        let _ = http_delete;
    }

    #[test]
    fn test_ws_functions_exist() {
        // Verificar que las funciones existen
        let _ = ws_connect;
        let _ = ws_disconnect;
        let _ = ws_send;
        let _ = ws_recv;
        let _ = ws_is_connected;
        let _ = ws_get_url;
    }
}
