#!/data/data/com.termux/files/usr/bin/bash
# 🚀 launcher_fase2.sh — Ry-Dit Fase 2 (Genética + Kepler + L-System)
# Detecta DISPLAY automáticamente y configura GPU Adreno con Zink

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
BIN="$PROJECT_DIR/target/release/demo_fase2_completo"

echo "🛡️ Ry-Dit — Fase 2: Ciencia Avanzada + Física Nuclear"

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

echo "🚀 Lanzando demo_fase2_completo..."
echo "   DISPLAY=$DISPLAY"
echo "   MESA_LOADER_DRIVER_OVERRIDE=$MESA_LOADER_DRIVER_OVERRIDE"
echo "   GALLIUM_DRIVER=$GALLIUM_DRIVER"
echo "========================================"

if [ ! -f "$BIN" ]; then
    echo "⚠️  Binario no encontrado. Compilando..."
    cargo build --bin demo_fase2_completo --release --manifest-path "$PROJECT_DIR/Cargo.toml"
fi

exec "$BIN" "$@"
