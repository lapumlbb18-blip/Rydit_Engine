// crates/rydit-stream/src/protocol.rs
// 🆕 JSON-RPC protocol para streaming

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Mensaje JSON-RPC 2.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcMessage {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
}

impl RpcMessage {
    /// Crear request
    pub fn request(method: &str, params: Value, id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: Some(method.to_string()),
            params: Some(params),
            result: None,
            error: None,
            id: Some(id),
        }
    }

    /// Crear response exitoso
    pub fn response(result: Value, id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: None,
            params: None,
            result: Some(result),
            error: None,
            id: Some(id),
        }
    }

    /// Crear response con error
    pub fn error(msg: &str, id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: None,
            params: None,
            result: None,
            error: Some(msg.to_string()),
            id: Some(id),
        }
    }

    /// Serializar a string JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserializar desde string JSON
    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    /// Verificar si es request
    pub fn is_request(&self) -> bool {
        self.method.is_some()
    }

    /// Verificar si es response
    pub fn is_response(&self) -> bool {
        self.result.is_some() || self.error.is_some()
    }
}

/// Tipos de mensajes de streaming
pub mod stream {
    use super::*;

    /// Datos de escena para streaming
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SceneData {
        pub bytecode: Option<Value>,
        pub entities: Vec<EntityData>,
        pub delta: f64,
    }

    /// Datos de entidad
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EntityData {
        pub id: u64,
        pub x: f64,
        pub y: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sprite: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub width: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub height: Option<f64>,
    }

    /// Crear mensaje de update
    pub fn update(entities: Vec<EntityData>, delta: f64) -> RpcMessage {
        RpcMessage::request(
            "stream.update",
            json!({
                "entities": entities,
                "delta": delta,
            }),
            0,
        )
    }

    /// Crear mensaje de escena
    pub fn scene(data: SceneData) -> RpcMessage {
        RpcMessage::request(
            "stream.scene",
            json!({ "data": data }),
            1,
        )
    }

    /// Crear mensaje de acción (client → server)
    pub fn action(name: &str) -> RpcMessage {
        RpcMessage::request(
            "stream.action",
            json!({ "action": name }),
            2,
        )
    }
}

/// Tipos de mensajes de portal
pub mod portal {
    use super::*;

    /// Crear mensaje de status
    pub fn status(online: bool, clients: u64) -> RpcMessage {
        RpcMessage::response(
            json!({
                "online": online,
                "clients": clients,
            }),
            0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpc_request() {
        let msg = RpcMessage::request("test", json!({"key": "value"}), 1);
        assert_eq!(msg.jsonrpc, "2.0");
        assert_eq!(msg.method.as_ref().unwrap(), "test");
        assert!(msg.is_request());
    }

    #[test]
    fn test_rpc_response() {
        let msg = RpcMessage::response(json!({"result": "ok"}), 1);
        assert!(msg.result.is_some());
        assert!(msg.is_response());
    }

    #[test]
    fn test_rpc_error() {
        let msg = RpcMessage::error("test error", 1);
        assert_eq!(msg.error.unwrap(), "test error");
    }

    #[test]
    fn test_rpc_serialize() {
        let msg = RpcMessage::request("test", json!({"x": 1}), 1);
        let json = msg.to_json().unwrap();
        assert!(json.contains("test"));
    }

    #[test]
    fn test_rpc_deserialize() {
        let json = r#"{"jsonrpc":"2.0","method":"test","id":1}"#;
        let msg: RpcMessage = RpcMessage::from_json(json).unwrap();
        assert_eq!(msg.method.unwrap(), "test");
    }

    #[test]
    fn test_stream_update() {
        let entities = vec![
            stream::EntityData {
                id: 1,
                x: 100.0,
                y: 200.0,
                sprite: None,
                color: Some("red".to_string()),
                width: None,
                height: None,
            },
        ];
        let msg = stream::update(entities, 0.016);
        assert_eq!(msg.method.unwrap(), "stream.update");
    }
}
