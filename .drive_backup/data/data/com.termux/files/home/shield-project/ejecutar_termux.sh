#!/data/data/com.termux/usr/bin/bash
# ejecutar_termux.sh - Script helper para ejecutar RyDit en Termux-X11

# ========================================
# Configuración automática de entorno
# ========================================

echo "🛡️ RyDit Engine v0.5.1 - Termux-X11"
echo "======================================"

# Detectar Termux
if [ -d "/data/data/com.termux" ]; then
    echo "✅ Termux detectado"
    
    # Configurar DISPLAY
    export DISPLAY=:0
    echo "📺 DISPLAY=:0"
    
    # Configurar driver zink
    export MESA_LOADER_DRIVER_OVERRIDE=zink
    echo "🎮 GPU Driver: zink"
    
    # Configurar DRI3
    export DRI3=1
    echo "⚙️  DRI3=1"
    
    echo "======================================"
    echo "✅ Entorno gráfico configurado"
    echo ""
else
    echo "⚠️  No es Termux - Usando configuración por defecto"
    echo ""
fi

# ========================================
# Mostrar ayuda
# ========================================
if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
    echo "Uso: $0 [opciones] [archivo.rydit]"
    echo ""
    echo "Opciones:"
    echo "  --gfx <archivo>  Ejecutar en modo gráfico"
    echo "  --repl           Iniciar REPL interactivo"
    echo "  --config         Mostrar configuración"
    echo "  --help, -h       Mostrar esta ayuda"
    echo ""
    echo "Demos disponibles:"
    echo "  demo_audio_player.rydit       - Reproductor de audio"
    echo "  demo_particles_editor.rydit   - Editor de partículas"
    echo "  demo_assets_test.rydit        - Test de assets"
    echo ""
    exit 0
fi

# ========================================
# Mostrar configuración
# ========================================
if [ "$1" == "--config" ]; then
    echo "=== CONFIGURACIÓN ACTUAL ==="
    echo "DISPLAY: $DISPLAY"
    echo "MESA_LOADER_DRIVER_OVERRIDE: $MESA_LOADER_DRIVER_OVERRIDE"
    echo "DRI3: $DRI3"
    echo "============================"
    exit 0
fi

# ========================================
# Ejecutar RyDit
# ========================================
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

if [ "$1" == "--gfx" ] && [ -n "$2" ]; then
    echo "🚀 Ejecutando: $2"
    ./target/release/rydit-rs --gfx "$2"
elif [ "$1" == "--repl" ]; then
    echo "🚀 Iniciando REPL"
    ./target/release/rydit-rs --repl
elif [ -n "$1" ]; then
    echo "🚀 Ejecutando: $1"
    ./target/release/rydit-rs "$1"
else
    echo "🎮 RyDit Engine v0.5.1"
    echo ""
    echo "Comandos rápidos:"
    echo "  ./ejecutar_termux.sh --gfx demo_audio_player.rydit"
    echo "  ./ejecutar_termux.sh --gfx demo_particles_editor.rydit"
    echo "  ./ejecutar_termux.sh --repl"
    echo ""
    echo "O usa --help para más opciones"
fi
