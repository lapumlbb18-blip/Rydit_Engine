#!/data/data/com.termux/files/usr/bin/bash
# Script para ejecutar demos de RyDit en Termux-X11
# v0.10.2 - Actualizado para scene_runner

# Configurar variables de entorno
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1
export PULSE_SERVER=127.0.0.1

# Verificar que X11 esté disponible
if ! xset q &>/dev/null; then
    echo "❌ ERROR: No se pudo conectar a X11 en DISPLAY=$DISPLAY"
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
DEMO_FILE="${1:-demos/nivel_config.rydit}"

if [ ! -f "$DEMO_FILE" ]; then
    echo "❌ ERROR: El archivo '$DEMO_FILE' no existe"
    exit 1
fi

echo "🎮 Ejecutando: $DEMO_FILE"
echo "================================"

# Detectar tipo de demo y usar binario correcto
cd /data/data/com.termux/files/home/shield-project

case "$DEMO_FILE" in
    *nivel*.rydit|*config*.rydit)
        # Demo de configuración (v0.10.2 - Inversión de Control)
        echo "📦 Usando: scene_runner (Inversión de Control)"
        ./target/release/scene_runner "$DEMO_FILE"
        ;;
    *ecs*.rydit|*entity*.rydit)
        # Demo ECS
        echo "🧬 Usando: ecs_demo_10k"
        ./target/release/ecs_demo_10k
        ;;
    *gpu*.rydit|*particulas*.rydit|*particles*.rydit)
        # Demo GPU
        echo "🚀 Usando: gpu_demo_100k"
        ./target/release/gpu_demo_100k
        ;;
    *test_minimo*.rydit|*diagnostico*.rydit)
        # Demo simple
        echo "🔍 Usando: scene_runner"
        ./target/release/scene_runner "$DEMO_FILE"
        ;;
    *)
        # Default: scene_runner
        echo "📦 Usando: scene_runner"
        ./target/release/scene_runner "$DEMO_FILE"
        ;;
esac
