#!/bin/bash
# 🧪 Test de Scripts .rydit con rydit-rs
# v0.11.4 - FSR 1.0 + Stream

# Exportar variables para Termux-X11
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

echo "🧪 Testeando Scripts .rydit"
echo "============================"
echo ""

# Directorio de demos
DEMOS_DIR="demos"

# Verificar si existen demos
if [ ! -d "$DEMOS_DIR" ]; then
    echo "❌ No existe directorio $DEMOS_DIR"
    exit 1
fi

# Listar demos disponibles
echo "📂 Demos disponibles:"
ls -1 $DEMOS_DIR/*.rydit 2>/dev/null | while read demo; do
    echo "  - $(basename $demo)"
done
echo ""

# Testear demo específico o todos
if [ -n "$1" ]; then
    DEMO_TO_RUN="$DEMOS_DIR/$1"
    if [ -f "$DEMO_TO_RUN" ]; then
        echo "🚀 Ejecutando: $DEMO_TO_RUN"
        echo ""
        cargo run --release -- --gfx "$DEMO_TO_RUN"
    else
        echo "❌ Demo no encontrado: $DEMO_TO_RUN"
        exit 1
    fi
else
    echo "💡 Uso: $0 <demo.rydit>"
    echo ""
    echo "Ejemplos:"
    echo "  $0 nivel1.rydit"
    echo "  $0 demo_simple_desde_cero.rydit"
    echo ""
    echo "O ejecuta directamente:"
    echo "  cargo run --release -- --gfx demos/nivel1.rydit"
fi
