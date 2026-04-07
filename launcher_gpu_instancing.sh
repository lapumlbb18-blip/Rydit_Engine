#!/data/data/com.termux/files/usr/bin/bash
# launcher_gpu_instancing.sh — Lanza demo GPU Instancing con Zink DRI3
# Detecta automáticamente DISPLAY y configura Zink + GPU Adreno

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BIN="$SCRIPT_DIR/target/release/demo_gpu_instancing"

if [ ! -f "$BIN" ]; then
    echo "⚠️  Binario no encontrado. Compilando..."
    cargo build --bin demo_gpu_instancing --release --manifest-path "$SCRIPT_DIR/Cargo.toml"
fi

# Detectar DISPLAY activa
if [ -n "$DISPLAY" ]; then
    echo "✅ DISPLAY detectada: $DISPLAY"
elif [ -S /tmp/.X11-unix/X0 ]; then
    export DISPLAY=:0
    echo "✅ DISPLAY seteada a :0 (socket encontrado)"
else
    echo "⚠️  No se detectó DISPLAY. Usando :0"
    export DISPLAY=:0
fi

# Configurar Zink + GPU Adreno
export MESA_LOADER_DRIVER_OVERRIDE=zink
export GALLIUM_DRIVER=zink

echo "🚀 Lanzando demo_gpu_instancing..."
echo "   DISPLAY=$DISPLAY"
echo "   MESA_LOADER_DRIVER_OVERRIDE=$MESA_LOADER_DRIVER_OVERRIDE"
echo "   GALLIUM_DRIVER=$GALLIUM_DRIVER"
echo "========================================"

exec "$BIN" "$@"
