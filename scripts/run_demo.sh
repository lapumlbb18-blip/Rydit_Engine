#!/bin/bash
# =============================================================================
# Ry-Dit - Lanzador de Demos
# =============================================================================
# Uso: ./run_demo.sh <nombre_demo> [opciones]
# =============================================================================

set -e

# Colores
ROJO='\033[0;31m'
VERDE='\033[0;32m'
AMARILLO='\033[1;33m'
AZUL='\033[0;34m'
NC='\033[0m'

# Funciones de ayuda
mostrar_ayuda() {
    echo "Uso: $0 <nombre_demo> [opciones]"
    echo ""
    echo "Demos disponibles:"
    if [ -d "demos" ]; then
        ls -1 demos/*.rydit 2>/dev/null | xargs -n1 basename | sed 's/\.rydit//' | while read demo; do
            echo "  • $demo"
        done
    else
        echo "  (no hay demos en el directorio actual)"
    fi
    echo ""
    echo "Opciones:"
    echo "  -h, --help     Mostrar esta ayuda"
    echo "  -l, --list     Listar demos disponibles"
    echo "  -r, --release  Usar binario de release"
    echo "  -d, --debug    Usar binario de debug"
    echo "  --no-x11       No configurar X11 automáticamente"
    echo ""
    echo "Ejemplos:"
    echo "  $0 snake"
    echo "  $0 demo_ilusiones_opticas --release"
    echo "  $0 --list"
}

listar_demos() {
    echo -e "${AZUL}Demos disponibles:${NC}"
    echo ""
    
    if [ -d "demos" ]; then
        count=0
        for demo in demos/*.rydit; do
            if [ -f "$demo" ]; then
                count=$((count + 1))
                echo "  $count. $(basename $demo .rydit)"
            fi
        done
        
        if [ $count -eq 0 ]; then
            echo "  (no hay demos)"
        fi
    else
        echo "  (directorio 'demos' no encontrado)"
    fi
    
    echo ""
}

# Parsear argumentos
USE_RELEASE=true
CONFIGURE_X11=true
DEMO_NAME=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            mostrar_ayuda
            exit 0
            ;;
        -l|--list)
            listar_demos
            exit 0
            ;;
        -r|--release)
            USE_RELEASE=true
            shift
            ;;
        -d|--debug)
            USE_RELEASE=false
            shift
            ;;
        --no-x11)
            CONFIGURE_X11=false
            shift
            ;;
        -*)
            echo -e "${ROJO}Opción desconocida: $1${NC}"
            mostrar_ayuda
            exit 1
            ;;
        *)
            DEMO_NAME="$1"
            shift
            ;;
    esac
done

# Verificar nombre del demo
if [ -z "$DEMO_NAME" ]; then
    echo -e "${AMARILLO}⚠️  No se especificó un demo${NC}"
    echo ""
    listar_demos
    exit 1
fi

# Buscar el archivo del demo
DEMO_FILE=""

if [ -f "demos/${DEMO_NAME}.rydit" ]; then
    DEMO_FILE="demos/${DEMO_NAME}.rydit"
elif [ -f "demos/${DEMO_NAME}.ry" ]; then
    DEMO_FILE="demos/${DEMO_NAME}.ry"
elif [ -f "${DEMO_NAME}.rydit" ]; then
    DEMO_FILE="${DEMO_NAME}.rydit"
else
    echo -e "${ROJO}✗ Demo no encontrado: $DEMO_NAME${NC}"
    echo ""
    echo "Demos disponibles:"
    ls -1 demos/*.rydit 2>/dev/null | xargs -n1 basename | sed 's/\.rydit//' | head -10
    exit 1
fi

# Determinar binario
if [ "$USE_RELEASE" = true ]; then
    if [ -f "target/release/rydit-rs" ]; then
        BINARY="target/release/rydit-rs"
    elif [ -f "target/release/rydit-rs.exe" ]; then
        BINARY="target/release/rydit-rs.exe"
    elif command -v rydit &> /dev/null; then
        BINARY="rydit"
    else
        echo -e "${ROJO}✗ Binario de release no encontrado${NC}"
        echo "Compila con: cargo build --release"
        exit 1
    fi
else
    if [ -f "target/debug/rydit-rs" ]; then
        BINARY="target/debug/rydit-rs"
    elif [ -f "target/debug/rydit-rs.exe" ]; then
        BINARY="target/debug/rydit-rs.exe"
    else
        echo -e "${ROJO}✗ Binario de debug no encontrado${NC}"
        echo "Compila con: cargo build"
        exit 1
    fi
fi

# Configurar entorno gráfico (si es necesario y no se desactivó)
if [ "$CONFIGURE_X11" = true ]; then
    # Detectar si estamos en Termux
    if [ -f "/data/data/com.termux/files/usr/bin/bash" ]; then
        echo -e "${AZUL}Configurando entorno para Termux-X11...${NC}"
        export DISPLAY=:0
        export MESA_LOADER_DRIVER_OVERRIDE=zink
        export DRI3=1
        echo "  DISPLAY=$DISPLAY"
        echo "  Driver: $MESA_LOADER_DRIVER_OVERRIDE"
        echo "  DRI3: $DRI3"
    fi
fi

# Ejecutar demo
echo ""
echo -e "${VERDE}🎮 Ejecutando: ${DEMO_NAME}${NC}"
echo -e "${AZUL}Archivo: ${DEMO_FILE}${NC}"
echo -e "${AZUL}Binario: ${BINARY}${NC}"
echo ""

# Verificar si el binario soporta --gfx
if $BINARY --help 2>&1 | grep -q "\-\-gfx"; then
    $BINARY --gfx "$DEMO_FILE"
else
    $BINARY "$DEMO_FILE"
fi

echo ""
echo -e "${VERDE}✓ Demo finalizado${NC}"
