#!/bin/bash
# =============================================================================
# RyDit Engine - Instalador para Android/Termux
# =============================================================================
# Versión: v0.8.0
# Plataforma: Android con Termux
# Arquitectura: AArch64, ARMv7, ARMv8L
# =============================================================================

set -e  # Salir en caso de error

# Colores
ROJO='\033[0;31m'
VERDE='\033[0;32m'
AMARILLO='\033[1;33m'
AZUL='\033[0;34m'
NC='\033[0m' # No Color

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
echo "  🛡️  RyDit Engine - Instalador para Termux"
echo "  Versión: v0.8.0"
echo "=================================================="
echo ""

# Verificar que estamos en Termux
log_info "Verificando entorno..."

if [ ! -f "/data/data/com.termux/files/usr/bin/bash" ]; then
    log_error "Esto no es Termux. Por favor usa el instalador apropiado para tu sistema."
    echo ""
    echo "Para Linux: ./scripts/install-linux.sh"
    echo "Para Windows: powershell -ExecutionPolicy Bypass -File scripts/install-windows.ps1"
    exit 1
fi

log_success "Termux detectado"

# Verificar arquitectura
ARCH=$(uname -m)
log_info "Arquitectura: $ARCH"

case $ARCH in
    aarch64|arm64)
        log_success "ARM64 (AArch64) - Arquitectura soportada"
        ;;
    armv7l|armv8l)
        log_success "ARM (32-bit) - Arquitectura soportada"
        ;;
    x86_64)
        log_warning "x86_64 - Puede haber problemas de compatibilidad"
        ;;
    *)
        log_warning "Arquitectura no probada: $ARCH"
        ;;
esac

# Actualizar paquetes
log_info "Actualizando paquetes del sistema..."
pkg update -y 2>&1 | tail -3
pkg upgrade -y 2>&1 | tail -3
log_success "Paquetes actualizados"

# Instalar Rust
log_info "Verificando Rust..."

if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    log_success "Rust ya instalado: $RUST_VERSION"
else
    log_info "Instalando Rust..."
    pkg install rust -y 2>&1 | tail -3
    log_success "Rust instalado"
fi

# Verificar Cargo
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    log_success "Cargo disponible: $CARGO_VERSION"
else
    log_error "Cargo no encontrado. Reinstala Rust."
    exit 1
fi

# Instalar dependencias de raylib
log_info "Instalando dependencias gráficas (raylib)..."

DEPENDENCIAS="xorg-xrandr libx11 libxi libxfixes libxrender libxdamage libxss"

for dep in $DEPENDENCIAS; do
    if pkg list-installed 2>/dev/null | grep -q "^$dep/"; then
        log_success "$dep ya instalado"
    else
        log_info "Instalando $dep..."
        pkg install $dep -y 2>&1 | tail -2
        log_success "$dep instalado"
    fi
done

# Instalar herramientas opcionales
log_info "Verificando herramientas opcionales..."

if command -v git &> /dev/null; then
    log_success "Git disponible"
else
    log_info "Instalando Git..."
    pkg install git -y 2>&1 | tail -2
    log_success "Git instalado"
fi

# Instalar sccache (opcional, acelera compilación)
if command -v sccache &> /dev/null; then
    log_success "sccache disponible (compilación rápida)"
else
    log_info "sccache no instalado. ¿Quieres instalarlo? (acelera compilaciones)"
    read -p "Instalar sccache? (s/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[SsY]$ ]]; then
        pkg install sccache -y 2>&1 | tail -2
        log_success "sccache instalado"
    fi
fi

# Compilar proyecto
echo ""
log_info "¿Deseas compilar RyDit Engine ahora?"
read -p "Compilar? (s/n): " -n 1 -r
echo

if [[ $REPLY =~ ^[SsY]$ ]]; then
    log_info "Compilando RyDit Engine..."
    echo ""
    
    # Build release
    cargo build --release 2>&1 | tee /tmp/rydit_build.log | tail -20
    
    # Verificar si la compilación fue exitosa
    if [ -f "target/release/rydit-rs" ]; then
        log_success "¡Compilación exitosa!"
        
        # Tamaño del binario
        BIN_SIZE=$(ls -lh target/release/rydit-rs | awk '{print $5}')
        log_info "Tamaño del binario: $BIN_SIZE"
        
        # Crear symlink
        echo ""
        log_info "Creando symlink en PATH..."
        ln -sf $(pwd)/target/release/rydit-rs $PREFIX/bin/rydit 2>/dev/null || {
            log_warning "No se pudo crear symlink global. Puedes ejecutar con: ./target/release/rydit-rs"
        }
        
        if [ -f "$PREFIX/bin/rydit" ]; then
            log_success "Comando 'rydit' disponible globalmente"
        fi
    else
        log_error "La compilación falló. Revisa /tmp/rydit_build.log"
        exit 1
    fi
else
    log_info "Puedes compilar después con: cargo build --release"
fi

# Configurar Termux-X11 (opcional)
echo ""
log_info "¿Deseas configurar Termux-X11 para gráficos?"
read -p "Configurar Termux-X11? (s/n): " -n 1 -r
echo

if [[ $REPLY =~ ^[SsY]$ ]]; then
    log_info "Configurando Termux-X11..."
    
    # Instalar Termux-X11
    if pkg list-installed 2>/dev/null | grep -q "termux-x11-nightly"; then
        log_success "Termux-X11 ya instalado"
    else
        log_info "Instalando Termux-X11..."
        pkg install termux-x11-nightly -y 2>&1 | tail -2
        log_success "Termux-X11 instalado"
    fi
    
    # Crear script de configuración
    cat > $PREFIX/bin/rydit-x11 << 'EOF'
#!/bin/bash
# RyDit - Lanzador con Termux-X11

export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

echo "🛡️ RyDit Engine - Iniciando con Termux-X11"
echo "   DISPLAY=$DISPLAY"
echo "   Driver: $MESA_LOADER_DRIVER_OVERRIDE"
echo ""

# Iniciar Termux-X11 si no está corriendo
if ! pgrep -x "termux-x11" > /dev/null; then
    echo "Iniciando Termux-X11..."
    termux-x11-nightly &
    sleep 2
fi

# Ejecutar RyDit
if [ -n "$1" ]; then
    rydit --gfx "$1"
else
    rydit --repl
fi
EOF

    chmod +x $PREFIX/bin/rydit-x11
    log_success "Script 'rydit-x11' creado"
    
    echo ""
    echo "Para ejecutar RyDit con gráficos:"
    echo "  1. Ejecuta: rydit-x11"
    echo "  2. O usa: termux-x11-nightly && rydit --gfx demo.rydit"
fi

# Crear script de demo
cat > $PREFIX/bin/rydit-demo << 'EOF'
#!/bin/bash
# RyDit - Lanzador de demos

if [ -z "$1" ]; then
    echo "Uso: rydit-demo <nombre_demo>"
    echo ""
    echo "Demos disponibles:"
    ls -1 demos/*.rydit 2>/dev/null | xargs -n1 basename | sed 's/\.rydit//' | nl
    exit 1
fi

export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

rydit --gfx "demos/${1}.rydit"
EOF

chmod +x $PREFIX/bin/rydit-demo
log_success "Script 'rydit-demo' creado"

# Resumen final
echo ""
echo "=================================================="
echo "  ✅ ¡Instalación de RyDit Engine completada!"
echo "=================================================="
echo ""
echo "📦 Comandos disponibles:"
echo ""
if [ -f "$PREFIX/bin/rydit" ]; then
    echo "  • rydit              - Ejecutar RyDit"
    echo "  • rydit --repl       - Modo REPL interactivo"
    echo "  • rydit --gfx <file> - Ejecutar demo gráfico"
fi
echo "  • rydit-demo <demo>  - Ejecutar demo (si configuraste X11)"
echo "  • rydit-x11          - Iniciar con Termux-X11 (si configuraste)"
echo ""
echo "📚 Documentación:"
echo "  • README.md          - Documentación principal"
echo "  • docs/              - Más documentación"
echo ""
echo "🎮 Primeros pasos:"
echo "  1. Ejecuta: rybot"
echo "  2. O prueba: rydit --repl"
echo ""
echo "🛠️  Desarrollo:"
echo "  • cargo build --release  - Compilar"
echo "  • cargo test             - Ejecutar tests"
echo "  • cargo run --bin rydit-rs -- --repl"
echo ""
echo "=================================================="
echo ""

# Ofrecer ejecutar Rybot
log_info "¿Quieres ejecutar Rybot ahora?"
read -p "Ejecutar rybot? (s/n): " -n 1 -r
echo

if [[ $REPLY =~ ^[SsY]$ ]]; then
    if [ -f "./rybot.sh" ]; then
        ./rybot.sh
    else
        log_warning "rybot.sh no encontrado en el directorio actual"
    fi
fi

exit 0
