#!/data/data/com.termux/files/usr/bin/bash
# Script de prueba para demos gráficos de RyDit en Termux-X11
# Uso: ./test_demos_x11.sh

echo "========================================"
echo "  RyDit v0.1.9 - Test Demos en X11"
echo "========================================"
echo ""

# Establecer variables de entorno
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

echo "[OK] Variables de entorno establecidas"
echo "    DISPLAY=$DISPLAY"
echo "    MESA_LOADER=$MESA_LOADER_DRIVER_OVERRIDE"
echo ""

# Verificar que los demos existen
echo "Verificando demos..."
echo ""

if [ -f "./target/debug/examples/demo" ]; then
    echo "[OK] demo (rydit-gfx v0.0.7) - 517 KB"
else
    echo "[ERROR] demo no encontrado"
fi

if [ -f "./target/release/rydit-rs" ]; then
    echo "[OK] rydit-rs (binario principal) - 736 KB"
else
    echo "[ERROR] rydit-rs no encontrado"
fi

if [ -f "./ejemplos_gfx/demo_shapes.rydit" ]; then
    echo "[OK] demo_shapes.rydit"
else
    echo "[ERROR] demo_shapes.rydit no encontrado"
fi

if [ -f "./snake_perfect.rydit" ]; then
    echo "[OK] snake_perfect.rydit"
else
    echo "[ERROR] snake_perfect.rydit no encontrado"
fi

echo ""
echo "========================================"
echo "  MENÚ DE DEMOS"
echo "========================================"
echo ""
echo "1. Demo rydit-gfx (v0.0.7) - Binario directo"
echo "   - Círculo rojo animado"
echo "   - Rectángulo verde"
echo "   - Línea azul"
echo "   - Texto"
echo ""
echo "2. demo_shapes.rydit"
echo "   - Círculos concéntricos"
echo "   - Rectángulos de colores"
echo "   - Líneas"
echo ""
echo "3. snake_perfect.rydit"
echo "   - Snake Game completo"
echo "   - Grid retro"
echo "   - Puntuación"
echo ""
echo "4. ejemplo_gfx.rydit"
echo "   - Formas básicas"
echo "   - Múltiples círculos"
echo ""
echo "5. Salir"
echo ""
echo "========================================"
echo ""

read -p "Selecciona un demo (1-5): " opcion

case $opcion in
    1)
        echo ""
        echo "Iniciando demo rydit-gfx (v0.0.7)..."
        echo "Presiona ESC para salir"
        echo ""
        ./target/debug/examples/demo
        ;;
    2)
        echo ""
        echo "Iniciando demo_shapes.rydit..."
        echo "Presiona ESC para salir"
        echo ""
        ./target/release/rydit-rs --gfx ejemplos_gfx/demo_shapes.rydit
        ;;
    3)
        echo ""
        echo "Iniciando snake_perfect.rydit..."
        echo "Flechas: Mover | SPACE: Restart | ESC: Salir"
        echo ""
        ./target/release/rydit-rs --gfx snake_perfect.rydit
        ;;
    4)
        echo ""
        echo "Iniciando ejemplo_gfx.rydit..."
        echo "Presiona ESC para salir"
        echo ""
        ./target/release/rydit-rs --gfx ejemplos_gfx/ejemplo_gfx.rydit
        ;;
    5)
        echo "Saliendo..."
        exit 0
        ;;
    *)
        echo "Opción no válida"
        exit 1
        ;;
esac

echo ""
echo "Demo finalizado."
echo ""
