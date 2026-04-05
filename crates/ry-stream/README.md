# ry-stream — LAN Streaming for Ry-Dit

WebSocket-based LAN streaming server with embedded web portal for the Ry-Dit game engine.

## Features

- **WebSocket Server**: Thread-based server with broadcast support
- **JSON-RPC Protocol**: Standardized messaging for scene/entity streaming
- **Web Portal**: Embedded HTML interface served via simple HTTP server
- **Client Library**: WebSocket client with RPC helpers
- **Zero Extra Dependencies**: Uses only `tungstenite`, `serde`, `serde_json`

## Usage

```rust
use ry_stream::{StreamServer, WebPortal};

// Start streaming server
let server = StreamServer::new("ws://127.0.0.1:8765");
server.start();

// Start web portal
let mut portal = WebPortal::new(8080);
portal.start();

// Broadcast scene updates
server.broadcast_rpc("stream.update", json!({
    "entities": [...],
    "delta": 0.016
}), 0)?;
```

## Architecture

```
┌─────────────┐     WebSocket      ┌──────────────┐
│  Ry-Dit     │ ──────────────────► │  Web Browser │
│  Server     │    JSON-RPC         │  Portal      │
│  (port 8765)│                     │  (port 8080) │
└─────────────┘                     └──────────────┘
```

## License

MIT
