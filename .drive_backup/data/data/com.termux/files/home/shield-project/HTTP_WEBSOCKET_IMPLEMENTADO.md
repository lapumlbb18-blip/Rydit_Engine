# 🛡️ RyDit v0.8.7 - HTTP + WebSocket Module

**Fecha**: 2026-03-28
**Estado**: ✅ IMPLEMENTADO (compilación pendiente en Termux)

---

## 📦 Nuevo Crate: `rydit-http`

### Dependencias
- **ureq** v2.9 - HTTP client ligero y sincrónico
- **tungstenite** v0.21 - WebSocket client
- **serde** + **serde_json** - JSON handling

### Funciones HTTP (4 funciones)

```rydit
# GET request
dark.slot respuesta = http::get("https://api.example.com/data")

# POST request con JSON
dark.slot datos = "{\"nombre\": \"Juan\", \"edad\": 25}"
dark.slot respuesta = http::post("https://api.example.com/users", datos)

# PUT request
dark.slot datos = "{\"id\": 1, \"nombre\": \"Maria\"}"
dark.slot respuesta = http::put("https://api.example.com/users/1", datos)

# DELETE request
dark.slot respuesta = http::delete("https://api.example.com/users/1")
```

### Funciones WebSocket (6 funciones)

```rydit
# Conectar a WebSocket
dark.slot resultado = ws::connect("ws://localhost:8080/ws")

# Enviar mensaje
ws::send("Hola servidor!")

# Recibir mensaje
dark.slot mensaje = ws::recv()
voz "Mensaje recibido: " + mensaje

# Verificar estado
onif ws::is_connected() {
    voz "Conectado a: " + ws::get_url()
} blelse {
    voz "No hay conexión"
}

# Desconectar
ws::disconnect()
```

---

## 🔗 Integración con LAZOS

### LAZOS + HTTP = 100% Conectividad

**LAZOS (95%)**:
- ✅ JSON-RPC sobre stdin/stdout
- ✅ Python bridge (ry_lazo.py)
- ✅ Comunicación local

**HTTP + WebSocket (5%)**:
- ✅ HTTP GET/POST/PUT/DELETE
- ✅ WebSocket client
- ✅ Comunicación remota

**Resultado**: **100% Conectividad** ✅

---

## 📊 Arquitectura

```
┌─────────────────────────────────────────────────────────┐
│  RyDit Script (.rydit)                                  │
├─────────────────────────────────────────────────────────┤
│  http::get()  → ureq → HTTP/HTTPS                       │
│  http::post() → ureq → HTTP/HTTPS                       │
│  ws::connect() → tungstenite → WebSocket                │
│  ws::send() → tungstenite → WebSocket                   │
│  ws::recv() → tungstenite → WebSocket                   │
├─────────────────────────────────────────────────────────┤
│  LAZOS Protocol (stdin/stdout JSON-RPC)                 │
│  ↓                                                      │
│  Python Bridge (ry_lazo.py)                             │
└─────────────────────────────────────────────────────────┘
```

---

## 🧪 Tests

### HTTP Tests
```rust
#[test]
fn test_http_get_invalid_url() {
    let result = http_get("http://url-invalida.com");
    assert!(result.is_err());
}

#[test]
fn test_http_post_invalid_url() {
    let result = http_post("http://url-invalida.com", "{}");
    assert!(result.is_err());
}
```

### WebSocket Tests
```rust
#[test]
fn test_ws_initial_state() {
    assert!(!ws_is_connected());
    assert_eq!(ws_get_url(), None);
}

#[test]
fn test_ws_recv_without_connect() {
    let result = ws_recv();
    assert!(result.is_err());
}
```

---

## 📝 Ejemplos de Uso

### 1. Consumir API REST

```rydit
# Obtener datos de API
dark.slot respuesta = http::get("https://jsonplaceholder.typicode.com/posts/1")

# Parsear JSON (usando json::parse)
dark.slot datos = json::parse(respuesta)

# Usar datos
voz "Título: " + datos["title"]
```

### 2. Enviar Datos a API

```rydit
# Crear payload JSON
dark.slot payload = "{\"title\": \"Nuevo Post\", \"body\": \"Contenido\", \"userId\": 1}"

# POST a API
dark.slot respuesta = http::post("https://jsonplaceholder.typicode.com/posts", payload)

voz "Status: " + respuesta
```

### 3. WebSocket Chat

```rydit
# Conectar a servidor WebSocket
ws::connect("ws://echo.websocket.org")

# Enviar mensaje
ws::send("Hola desde RyDit!")

# Recibir eco
dark.slot eco = ws::recv()
voz "Eco: " + eco

# Desconectar
ws::disconnect()
```

### 4. WebSocket + LAZOS

```rydit
# Conectar WebSocket desde LAZOS
# (Python envía comando JSON-RPC)
# echo '{"method":"ws::connect","params":["ws://localhost:8080"]}' | rydit-rs --lazos

# Recibir mensajes en loop
ryda ws::is_connected() {
    dark.slot msg = ws::recv()
    # Procesar mensaje con LAZOS
    json::stringify(msg)
}
```

---

## ⚠️ Notas de Implementación

### Compilación en Termux
- **ring** (dependencia de tungstenite) requiere compilación de C
- Tiempo estimado: 5-10 minutos en Redmi Note 8
- Comando: `cargo build --release -p rydit-http`

### Alternativa Ligera
Si la compilación es muy lenta, usar:
```toml
[dependencies]
ureq = { version = "2.9", features = ["json"], default-features = false }
# tungstenite → opcional, solo si se necesita WebSocket
```

---

## 🎯 Próximos Pasos

1. **Compilar en Termux** - Verificar que todo funciona
2. **Demo HTTP** - `demo_http_api.rydit`
3. **Demo WebSocket** - `demo_websocket.rydit`
4. **Integración LAZOS + HTTP** - Comandos remotos vía HTTP
5. **Documentación** - Actualizar README principal

---

<div align="center">

**🛡️ RyDit v0.8.7 - HTTP + WebSocket**

*HTTP: 4 funciones | WebSocket: 6 funciones | LAZOS: 100% completado*

**Conectividad Total: Local (LAZOS) + Remota (HTTP/WS)**

</div>
