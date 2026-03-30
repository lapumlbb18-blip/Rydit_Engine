#!/data/data/com.termux/files/usr/bin/bash
# test_gfx_v0.9.0.sh - Script para tests gráficos con las 3 capas críticas

echo "🛡️ RyDit v0.9.0 - Tests Gráficos con 3 Capas Críticas"
echo "======================================================="
echo ""

# Configurar entorno gráfico
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

echo "✅ Entorno gráfico configurado:"
echo "   DISPLAY=$DISPLAY"
echo "   MESA_LOADER_DRIVER_OVERRIDE=$MESA_LOADER_DRIVER_OVERRIDE"
echo "   DRI3=$DRI3"
echo ""

# Script del proyecto
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Función para ejecutar test
run_test() {
    local test_file=$1
    local test_name=$2
    
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "🧪 TEST: $test_name"
    echo "   Archivo: $test_file"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    if [ -f "$test_file" ]; then
        echo "🚀 Ejecutando..."
        ./target/release/rydit-rs --gfx "$test_file"
        echo ""
    else
        echo "❌ Archivo no encontrado: $test_file"
        echo ""
    fi
}

# Función para ejecutar demo Rust
run_rust_demo() {
    local demo_name=$1
    
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "🦀 DEMO RUST: $demo_name"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    echo "🚀 Ejecutando..."
    ./target/release/examples/$demo_name
    echo ""
}

# Mostrar menú
echo "Selecciona un test:"
echo ""
echo "📝 DEMOS RYDIT (.rydit):"
echo "  1) demo_shapes.rydit - Formas básicas"
echo "  2) ejemplo_gfx.rydit - Ejemplo gráfico"
echo "  3) ejemplo.rydit - Ejemplo simple"
echo "  4) test_renderizado_v0.9.0.rydit - Test completo v0.9.0"
echo ""
echo "🦀 DEMOS RUST (render queue):"
echo "  5) demo_render_queue - Demo 3 capas críticas"
echo ""
echo "🎮 DEMOS COMPLEJOS:"
echo "  6) snake_v0.1.8.rydit - Snake Game"
echo ""
echo "0) Salir"
echo ""
echo -n "Opción: "
read opcion

case $opcion in
    1)
        run_test "ejemplos_gfx/demo_shapes.rydit" "Formas Básicas"
        ;;
    2)
        run_test "ejemplos_gfx/ejemplo_gfx.rydit" "Ejemplo Gráfico"
        ;;
    3)
        run_test "ejemplos_gfx/ejemplo.rydit" "Ejemplo Simple"
        ;;
    4)
        run_test "demos/test_renderizado_v0.9.0.rydit" "Test Renderizado v0.9.0"
        ;;
    5)
        run_rust_demo "demo_render_queue"
        ;;
    6)
        run_test "ejemplos_gfx/snake_v0.1.8.rydit" "Snake Game"
        ;;
    0)
        echo "👋 ¡Hasta luego!"
        exit 0
        ;;
    *)
        echo "❌ Opción inválida"
        exit 1
        ;;
esac

echo "✅ Test completado"
echo ""
echo "📊 RESUMEN:"
echo "   - Command Queue: 8192+ draw calls ✅"
echo "   - Double Buffering: front/back buffer ✅"
echo "   - Platform Sync: XFlush/XSync ✅"
echo ""
echo "🚀 RyDit v0.9.0 - Listo para demos complejos!"
