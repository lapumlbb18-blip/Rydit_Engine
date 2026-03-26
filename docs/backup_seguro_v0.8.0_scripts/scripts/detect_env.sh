#!/bin/bash
# =============================================================================
# RyDit Engine - Detección de Entorno
# =============================================================================
# Uso: ./detect_env.sh
# =============================================================================

# Colores
ROJO='\033[0;31m'
VERDE='\033[0;32m'
AMARILLO='\033[1;33m'
AZUL='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Funciones de logging
log_info() {
    echo -e "${AZUL}[INFO]${NC} $1"
}

log_success() {
    echo -e "${VERDE}[✓]${NC} $1"
}

log_warning() {
    echo -e "${AMARILLO}[⚠]${NC} $1"
}

log_error() {
    echo -e "${ROJO}[✗]${NC} $1"
}

# Header
echo ""
echo "=================================================="
echo "  🔍 RyDit Engine - Detección de Entorno"
echo "=================================================="
echo ""

# =============================================================================
# SISTEMA OPERATIVO
# =============================================================================

echo -e "${MAGENTA}SISTEMA OPERATIVO${NC}"
echo "----------------------------------------"

if [ -f "/data/data/com.termux/files/usr/bin/bash" ]; then
    echo -e "  ${VERDE}✓${NC} Android/Termux"
    
    # Versión de Termux
    if command -v pkg &> /dev/null; then
        TERMUX_VER=$(pkg --version 2>&1 | head -1)
        echo "    └─ $TERMUX_VER"
    fi
    
    # Versión de Android
    if [ -f "/system/build.prop" ]; then
        ANDROID_VER=$(grep "ro.build.version.release" /system/build.prop 2>/dev/null | cut -d'=' -f2)
        if [ -n "$ANDROID_VER" ]; then
            echo "    └─ Android $ANDROID_VER"
        fi
    fi
    
elif [ -f "/etc/debian_version" ]; then
    echo -e "  ${VERDE}✓${NC} Linux/Debian"
    
    DISTRO=$(cat /etc/os-release | grep "PRETTY_NAME" | cut -d'"' -f2)
    echo "    └─ $DISTRO"
    
elif [ -f "/etc/fedora-release" ]; then
    echo -e "  ${VERDE}✓${NC} Linux/Fedora"
    
    FEDORA_VER=$(cat /etc/fedora-release)
    echo "    └─ $FEDORA_VER"
    
elif [ -f "/etc/arch-release" ]; then
    echo -e "  ${VERDE}✓${NC} Linux/Arch"
    
elif [ -f "/etc/redhat-release" ]; then
    echo -e "  ${VERDE}✓${NC} Linux/RedHat"
    
    RHEL_VER=$(cat /etc/redhat-release)
    echo "    └─ $RHEL_VER"
    
elif command -v powershell &> /dev/null || command -v pwsh &> /dev/null; then
    echo -e "  ${VERDE}✓${NC} Windows"
    
    # Versión de Windows (desde PowerShell)
    if command -v powershell &> /dev/null; then
        WIN_VER=$(powershell -Command "(Get-CimInstance Win32_OperatingSystem).Caption" 2>/dev/null)
    else
        WIN_VER=$(pwsh -Command "(Get-CimInstance Win32_OperatingSystem).Caption" 2>/dev/null)
    fi
    
    if [ -n "$WIN_VER" ]; then
        echo "    └─ $WIN_VER"
    fi
    
else
    echo -e "  ${AMARILLO}⚠️${NC} Sistema no detectado"
    echo "    └─ $(uname -s) $(uname -r)"
fi

echo ""

# =============================================================================
# ARQUITECTURA
# =============================================================================

echo -e "${MAGENTA}ARQUITECTURA${NC}"
echo "----------------------------------------"

ARCH=$(uname -m)
echo "  Máquina: $ARCH"

case $ARCH in
    aarch64|arm64)
        echo -e "  ${VERDE}✓${NC} ARM64 (AArch64) - Soportado"
        echo "    └─ Ideal para Termux"
        ;;
    armv7l|armv8l)
        echo -e "  ${VERDE}✓${NC} ARM (32-bit) - Soportado"
        echo "    └─ Puede requerir compilación desde fuente"
        ;;
    x86_64|amd64)
        echo -e "  ${VERDE}✓${NC} x86_64 (AMD64) - Soportado"
        echo "    └─ Arquitectura principal para Linux/Windows"
        ;;
    i386|i686)
        echo -e "  ${AMARILLO}⚠️${NC} x86 (32-bit) - Soporte limitado"
        ;;
    *)
        echo -e "  ${AMARILLO}⚠️${NC} Arquitectura no probada"
        ;;
esac

echo ""

# =============================================================================
# RUST Y CARGO
# =============================================================================

echo -e "${MAGENA}RUST Y CARGO${NC}"
echo "----------------------------------------"

if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo -e "  ${VERDE}✓${NC} Rust: $RUST_VERSION"
    
    # Verificar versión mínima (1.70)
    RUST_MINOR=$(rustc --version | cut -d'.' -f2 | cut -d' ' -f1)
    if [ "$RUST_MINOR" -ge 70 ]; then
        echo -e "    ${VERDE}└─ Versión compatible (≥1.70)${NC}"
    else
        echo -e "    ${ROJO}└─ Versión muy antigua (<1.70)${NC}"
        echo -e "    ${AMARILLO}└─ Actualiza: rustup update${NC}"
    fi
else
    echo -e "  ${ROJO}✗${NC} Rust: No instalado"
    echo -e "    ${AMARILLO}└─ Instala: https://rustup.rs${NC}"
fi

if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    echo -e "  ${VERDE}✓${NC} Cargo: $CARGO_VERSION"
else
    echo -e "  ${ROJO}✗${NC} Cargo: No instalado"
fi

# Toolchain
if command -v rustup &> /dev/null; then
    RUSTUP_DEFAULT=$(rustup default 2>/dev/null)
    echo -e "  ${AZUL}ℹ${NC} Rustup: $RUSTUP_DEFAULT"
fi

echo ""

# =============================================================================
# DEPENDENCIAS GRÁFICAS (RAYLIB)
# =============================================================================

echo -e "${MAGENTA}DEPENDENCIAS GRÁFICAS (raylib)${NC}"
echo "----------------------------------------"

# Verificar X11
if [ -n "$DISPLAY" ]; then
    echo -e "  ${VERDE}✓${NC} X11: DISPLAY=$DISPLAY"
    
    # Verificar si xdpyinfo está disponible
    if command -v xdpyinfo &> /dev/null; then
        RES=$(xdpyinfo 2>/dev/null | grep 'dimensions:' | awk '{print $4}')
        if [ -n "$RES" ]; then
            echo -e "    ${AZUL}└─ Resolución: $RES${NC}"
        fi
    fi
else
    echo -e "  ${AMARILLO}⚠️${NC} X11: DISPLAY no configurado"
    
    if [ -f "/data/data/com.termux/files/usr/bin/bash" ]; then
        echo -e "    ${AZUL}└─ En Termux: export DISPLAY=:0${NC}"
        echo -e "    ${AZUL}└─ Luego ejecuta: termux-x11-nightly${NC}"
    fi
fi

# Verificar librerías de raylib
echo ""
echo "  Librerías de sistema:"

check_lib() {
    local lib=$1
    local name=$2
    
    if ldconfig -p 2>/dev/null | grep -q "$lib"; then
        echo -e "    ${VERDE}✓${NC} $name"
    elif pkg list-installed 2>/dev/null | grep -q "$lib"; then
        echo -e "    ${VERDE}✓${NC} $name (Termux)"
    else
        echo -e "    ${AMARILLO}⚠️${NC} $name (no encontrada)"
    fi
}

check_lib "libX11" "libX11"
check_lib "libXi" "libXi"
check_lib "libXrandr" "libXrandr"
check_lib "libasound" "ALSA (audio)"
check_lib "libGL" "OpenGL"

echo ""

# =============================================================================
# GIT
# =============================================================================

echo -e "${MAGENTA}GIT${NC}"
echo "----------------------------------------"

if command -v git &> /dev/null; then
    GIT_VERSION=$(git --version)
    echo -e "  ${VERDE}✓${NC} Git: $GIT_VERSION"
else
    echo -e "  ${ROJO}✗${NC} Git: No instalado"
    echo -e "    ${AMARILLO}└─ Instala para clonar el repositorio${NC}"
fi

echo ""

# =============================================================================
# ESPACIO EN DISCO
# =============================================================================

echo -e "${MAGENTA}ESPACIO EN DISCO${NC}"
echo "----------------------------------------"

if command -v df &> /dev/null; then
    # Espacio total y libre
    DISK_INFO=$(df -h . 2>/dev/null | awk 'NR==2 {print $2, $4}')
    TOTAL=$(echo $DISK_INFO | cut -d' ' -f1)
    FREE=$(echo $DISK_INFO | cut -d' ' -f2)
    
    echo -e "  Total: $TOTAL"
    echo -e "  Libre: ${VERDE}$FREE${NC}"
    
    # Verificar si hay suficiente espacio (mínimo 1GB)
    FREE_NUM=$(df . 2>/dev/null | awk 'NR==2 {print $4}' | sed 's/[A-Za-z]//g')
    
    if [ -n "$FREE_NUM" ]; then
        if [ "$FREE_NUM" -lt 1048576 ]; then
            echo -e "  ${ROJO}⚠️  Poco espacio (<1GB)${NC}"
        elif [ "$FREE_NUM" -lt 5242880 ]; then
            echo -e "  ${AMARILLO}⚠️  Espacio limitado (<5GB)${NC}"
        else
            echo -e "  ${VERDE}✓${NC} Espacio suficiente"
        fi
    fi
else
    echo -e "  ${AMARILLO}⚠️${NC} No se pudo determinar"
fi

echo ""

# =============================================================================
# MEMORIA RAM
# =============================================================================

echo -e "${MAGENTA}MEMORIA RAM${NC}"
echo "----------------------------------------"

if command -v free &> /dev/null; then
    RAM_INFO=$(free -h | awk 'NR==2 {print $2, $7}')
    TOTAL=$(echo $RAM_INFO | cut -d' ' -f1)
    FREE=$(echo $RAM_INFO | cut -d' ' -f2)
    
    echo -e "  Total: $TOTAL"
    echo -e "  Libre: ${VERDE}$FREE${NC}"
    
    # Verificar si hay suficiente RAM (mínimo 2GB)
    FREE_MB=$(free -m | awk 'NR==2 {print $7}')
    
    if [ -n "$FREE_MB" ]; then
        if [ "$FREE_MB" -lt 1024 ]; then
            echo -e "  ${ROJO}⚠️  Poca RAM (<1GB)${NC}"
        elif [ "$FREE_MB" -lt 2048 ]; then
            echo -e "  ${AMARILLO}⚠️  RAM limitada (<2GB)${NC}"
        else
            echo -e "  ${VERDE}✓${NC} RAM suficiente"
        fi
    fi
else
    echo -e "  ${AMARILLO}⚠️${NC} No se pudo determinar"
fi

echo ""

# =============================================================================
# PROYECTO RYDIT
# =============================================================================

echo -e "${MAGENTA}PROYECTO RYDIT${NC}"
echo "----------------------------------------"

# Verificar si estamos en el directorio del proyecto
if [ -f "Cargo.toml" ]; then
    echo -e "  ${VERDE}✓${NC} Directorio del proyecto detectado"
    
    # Contar crates
    CRATES_COUNT=$(ls -d crates/*/ 2>/dev/null | wc -l)
    echo -e "  Crates: ${CYAN}$CRATES_COUNT${NC}"
    
    # Contar demos
    DEMOS_COUNT=$(ls demos/*.rydit 2>/dev/null | wc -l)
    echo -e "  Demos: ${CYAN}$DEMOS_COUNT${NC}"
    
    # Contar tests
    TESTS_COUNT=$(grep -r "#\[test\]" crates/*/src/*.rs 2>/dev/null | wc -l)
    echo -e "  Tests: ${CYAN}$TESTS_COUNT${NC}"
    
    # Líneas de código
    RUST_LINES=$(find crates -name "*.rs" -exec cat {} \; 2>/dev/null | wc -l)
    RYDIT_LINES=$(find demos -name "*.rydit" -exec cat {} \; 2>/dev/null | wc -l)
    
    echo -e "  Líneas Rust: ${CYAN}$RUST_LINES${NC}"
    echo -e "  Líneas RyDit: ${CYAN}$RYDIT_LINES${NC}"
    
    # Binarios
    if [ -f "target/release/rydit-rs" ]; then
        BIN_SIZE=$(ls -lh target/release/rydit-rs | awk '{print $5}')
        echo -e "  Binario release: ${VERDE}$BIN_SIZE${NC}"
    elif [ -f "target/debug/rydit-rs" ]; then
        BIN_SIZE=$(ls -lh target/debug/rydit-rs | awk '{print $5}')
        echo -e "  Binario debug: ${AMARILLO}$BIN_SIZE${NC}"
    else
        echo -e "  Binarios: ${AMARILLO}No compilado${NC}"
    fi
else
    echo -e "  ${AMARILLO}⚠️${NC} No estás en el directorio del proyecto"
    echo -e "    ${AZUL}└─ Busca Cargo.toml${NC}"
fi

echo ""

# =============================================================================
# RECOMENDACIONES
# =============================================================================

echo -e "${MAGENTA}RECOMENDACIONES${NC}"
echo "----------------------------------------"

RECOMMENDATIONS=0

# Rust
if ! command -v rustc &> /dev/null; then
    echo -e "  ${ROJO}1.${NC} Instala Rust: ${AZUL}https://rustup.rs${NC}"
    RECOMMENDATIONS=$((RECOMMENDATIONS + 1))
fi

# X11
if [ -z "$DISPLAY" ] && [ -f "/data/data/com.termux/files/usr/bin/bash" ]; then
    echo -e "  ${AMARILLO}2.${NC} Configura Termux-X11:"
    echo -e "     ${AZUL}export DISPLAY=:0${NC}"
    echo -e "     ${AZUL}export MESA_LOADER_DRIVER_OVERRIDE=zink${NC}"
    echo -e "     ${AZUL}export DRI3=1${NC}"
    RECOMMENDATIONS=$((RECOMMENDATIONS + 1))
fi

# Espacio
FREE_NUM=$(df . 2>/dev/null | awk 'NR==2 {print $4}' | sed 's/[A-Za-z]//g')
if [ -n "$FREE_NUM" ] && [ "$FREE_NUM" -lt 1048576 ]; then
    echo -e "  ${ROJO}3.${NC} Libera espacio en disco (<1GB libre)"
    RECOMMENDATIONS=$((RECOMMENDATIONS + 1))
fi

# RAM
FREE_MB=$(free -m | awk 'NR==2 {print $7}')
if [ -n "$FREE_MB" ] && [ "$FREE_MB" -lt 1024 ]; then
    echo -e "  ${AMARILLO}4.${NC} Cierra aplicaciones (RAM <1GB)"
    RECOMMENDATIONS=$((RECOMMENDATIONS + 1))
fi

if [ $RECOMMENDATIONS -eq 0 ]; then
    echo -e "  ${VERDE}✓${NC} ¡Todo está listo para desarrollar!"
fi

echo ""
echo "=================================================="
echo ""

exit 0
