#!/data/data/com.termux/files/usr/bin/bash
# Script para ejecutar demos de RyDit en Termux-X11

# Configurar variables de entorno
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1
export PULSE_SERVER=127.0.0.1

# Verificar que X11 esté disponible
if ! xset q &>/dev/null; then
    echo "ERROR: No se pudo conectar a X11 en DISPLAY=$DISPLAY"
    echo ""
    echo "Asegurate de que Termux-X11 esté corriendo:"
    echo "  termux-x11 :0 -xstartup xfce4-session &"
    exit 1
fi

echo "✅ X11 disponible en DISPLAY=$DISPLAY"
echo "✅ Driver: $MESA_LOADER_DRIVER_OVERRIDE"
echo "✅ DRI3: $DRI3"
echo ""

# Obtener el demo a ejecutar
DEMO_FILE="${1:-ejemplos_gfx/ejemplo_gfx.rydit}"

if [ ! -f "$DEMO_FILE" ]; then
    echo "ERROR: El archivo '$DEMO_FILE' no existe"
    exit 1
fi

echo "🎮 Ejecutando: $DEMO_FILE"
echo "================================"

# Ejecutar RyDit
cd /data/data/com.termux/files/home/shield-project
./target/release/rydit-rs --gfx "$DEMO_FILE"
