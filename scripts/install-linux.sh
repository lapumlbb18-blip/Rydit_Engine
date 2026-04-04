#!/bin/bash
# =============================================================================
# Ry-Dit - Instalador para Linux
# =============================================================================
# Versión: v0.8.0
# Plataformas: Debian, Ubuntu, Fedora, Arch Linux
# Arquitectura: x86_64, aarch64
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
echo "  🛡️  Ry-Dit - Instalador para Linux"
echo "  Versión: v0.8.0"
echo "=================================================="
echo ""

# Detectar distribución
log_info "Detectando distribución de Linux..."

if [ -f /etc/debian_version ]; then
    DISTRO="debian"
    DISTRO_NAME=$(cat /etc/os-release | grep "PRETTY_NAME" | cut -d'"' -f2)
    log_success "Debian/Ubuntu detectado: $DISTRO_NAME"
    
    # Comandos de instalación
    UPDATE_CMD="sudo apt-get update"
    INSTALL_CMD="sudo apt-get install -y"
    
elif [ -f /etc/fedora-release ]; then
    DISTRO="fedora"
    DISTRO_NAME=$(cat /etc/os-release | grep "PRETTY_NAME" | cut -d'"' -f2)
    log_success "Fedora detectado: $DISTRO_NAME"
    
    UPDATE_CMD="sudo dnf update -y"
    INSTALL_CMD="sudo dnf install -y"
    
elif [ -f /etc/arch-release ]; then
    DISTRO="arch"
    DISTRO_NAME="Arch Linux"
    log_success "Arch Linux detectado"
    
    UPDATE_CMD="sudo pacman -Sy"
    INSTALL_CMD="sudo pacman -S --noconfirm"
    
elif [ -f /etc/redhat-release ]; then
    DISTRO="rhel"
    DISTRO_NAME=$(cat /etc/redhat-release)
    log_success "RedHat/CentOS detectado: $DISTRO_NAME"
    
    UPDATE_CMD="sudo yum update -y"
    INSTALL_CMD="sudo yum install -y"
    
else
    DISTRO="unknown"
    log_warning "Distribución no detectada automáticamente"
    echo "Por favor instala las dependencias manualmente:"
    echo "  - Rust: https://rustup.rs"
    echo "  - raylib: https://github.com/raysan5/raylib"
    exit 1
fi

# Verificar arquitectura
ARCH=$(uname -m)
log_info "Arquitectura: $ARCH"

case $ARCH in
    x86_64|amd64)
        log_success "x86_64 (AMD64) - Arquitectura principal soportada"
        ;;
    aarch64|arm64)
        log_success "ARM64 (AArch64) - Arquitectura soportada"
        ;;
    armv7l)
        log_warning "ARMv7 (32-bit) - Puede requerir compilación desde fuente"
        ;;
    *)
        log_warning "Arquitectura no probada: $ARCH"
        ;;
esac

# Actualizar sistema
log_info "Actualizando paquetes del sistema..."
eval $UPDATE_CMD 2>&1 | tail -3
log_success "Paquetes actualizados"

# Instalar Rust
log_info "Verificando Rust..."

if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    log_success "Rust ya instalado: $RUST_VERSION"
else
    log_info "Instalando Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    
    # Cargar Rust en el PATH actual
    if [ -f "$HOME/.cargo/env" ]; then
        source $HOME/.cargo/env
    fi
    
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
log_info "Instalando dependencias de raylib..."

case $DISTRO in
    debian|ubuntu)
        DEPENDENCIAS="libasound2-dev libx11-dev libxi-dev libxrandr-dev libgl1-mesa-dev libxcursor-dev libxinerama-dev"
        eval $INSTALL_CMD $DEPENDENCIAS 2>&1 | tail -3
        ;;
    fedora)
        DEPENDENCIAS="alsa-lib-devel libX11-devel libXi-devel libXrandr-devel mesa-libGL-devel libXcursor-devel libXinerama-devel"
        eval $INSTALL_CMD $DEPENDENCIAS 2>&1 | tail -3
        ;;
    arch)
        DEPENDENCIAS="alsa-lib libx11 libxi libxrandr mesa libxcursor libxinerama"
        eval $INSTALL_CMD $DEPENDENCIAS 2>&1 | tail -3
        ;;
    rhel)
        DEPENDENCIAS="alsa-lib-devel libX11-devel libXi-devel libXrandr-devel mesa-libGL-devel"
        eval $INSTALL_CMD $DEPENDENCIAS 2>&1 | tail -3
        ;;
esac

log_success "Dependencias de raylib instaladas"

# Instalar herramientas opcionales
log_info "Verificando herramientas opcionales..."

if command -v git &> /dev/null; then
    log_success "Git disponible"
else
    log_info "Instalando Git..."
    eval $INSTALL_CMD git 2>&1 | tail -2
    log_success "Git instalado"
fi

# Instalar sccache (opcional)
if command -v sccache &> /dev/null; then
    log_success "sccache disponible (compilación rápida)"
else
    log_info "¿Quieres instalar sccache? (acelera compilaciones futuras)"
    read -p "Instalar sccache? (s/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[SsY]$ ]]; then
        case $DISTRO in
            debian|ubuntu)
                eval $INSTALL_CMD sccache 2>&1 | tail -2 || log_warning "sccache no disponible en repositorios"
                ;;
            fedora)
                eval $INSTALL_CMD sccache 2>&1 | tail -2 || log_warning "sccache no disponible en repositorios"
                ;;
            arch)
                eval $INSTALL_CMD sccache 2>&1 | tail -2 || log_warning "sccache no disponible en repositorios"
                ;;
            *)
                log_warning "Instala sccache manualmente: https://github.com/mozilla/sccache"
                ;;
        esac
    fi
fi

# Compilar proyecto
echo ""
log_info "¿Deseas compilar Ry-Dit ahora?"
read -p "Compilar? (s/n): " -n 1 -r
echo

if [[ $REPLY =~ ^[SsY]$ ]]; then
    log_info "Compilando Ry-Dit..."
    echo ""
    
    # Build release
    cargo build --release 2>&1 | tee /tmp/rydit_build.log | tail -20
    
    # Verificar compilación
    if [ -f "target/release/rydit-rs" ]; then
        log_success "¡Compilación exitosa!"
        
        # Tamaño del binario
        BIN_SIZE=$(ls -lh target/release/rydit-rs | awk '{print $5}')
        log_info "Tamaño del binario: $BIN_SIZE"
        
        # Instalar binario globalmente
        echo ""
        log_info "Instalando binario globalmente..."
        sudo cp target/release/rydit-rs /usr/local/bin/rydit 2>/dev/null || {
            log_warning "No se pudo copiar a /usr/local/bin. Puedes usar: ./target/release/rydit-rs"
        }
        
        if [ -f "/usr/local/bin/rydit" ]; then
            log_success "Comando 'rydit' disponible globalmente"
        fi
    else
        log_error "La compilación falló. Revisa /tmp/rydit_build.log"
        exit 1
    fi
else
    log_info "Puedes compilar después con: cargo build --release"
fi

# Crear script de demo
cat > /tmp/rydit-demo << 'EOF'
#!/bin/bash
# RyDit - Lanzador de demos

if [ -z "$1" ]; then
    echo "Uso: rydit-demo <nombre_demo>"
    echo ""
    echo "Demos disponibles:"
    ls -1 demos/*.rydit 2>/dev/null | xargs -n1 basename | sed 's/\.rydit//' | nl
    exit 1
fi

rydit --gfx "demos/${1}.rydit"
EOF

sudo cp /tmp/rydit-demo /usr/local/bin/rydit-demo
sudo chmod +x /usr/local/bin/rydit-demo
log_success "Script 'rydit-demo' creado"

# Resumen final
echo ""
echo "=================================================="
echo "  ✅ ¡Instalación de Ry-Dit completada!"
echo "=================================================="
echo ""
echo "📦 Comandos disponibles:"
echo ""
if [ -f "/usr/local/bin/rydit" ]; then
    echo "  • rydit              - Ejecutar RyDit"
    echo "  • rydit --repl       - Modo REPL interactivo"
    echo "  • rydit --gfx <file> - Ejecutar demo gráfico"
    echo "  • rydit-demo <demo>  - Ejecutar demo fácilmente"
fi
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
