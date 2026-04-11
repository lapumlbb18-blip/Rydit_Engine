#!/bin/bash
# Launcher: Demo Militar con Partículas
# Sprites procedulares + sistema de partículas ry-anim

set -e

echo "🎖️  Lanzando Demo Militar - RyDit..."

# Auto-detectar DISPLAY o usar framebuffer
if [ -z "$DISPLAY" ] && [ -z "$WAYLAND_DISPLAY" ]; then
    echo "⚠️  No hay display gráfico detectado"
    echo "   Establece DISPLAY=:0 si tienes X11"
    exit 1
fi

# Usar Zink si está disponible para mejor rendimiento GPU
if command -v zink &> /dev/null; then
    echo "🎮  Usando Zink (OpenGL sobre Vulkan)"
    export GALLIUM_DRIVER=zink
fi

# Ejecutar demo
cd /data/data/com.termux/files/home/shield-project
cargo run --bin demo_militar --release
