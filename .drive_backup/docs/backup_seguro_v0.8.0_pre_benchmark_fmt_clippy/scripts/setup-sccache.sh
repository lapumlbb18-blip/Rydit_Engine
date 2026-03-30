#!/bin/bash
# =============================================================================
# SHIELD PROJECT - Setup de sccache para builds rápidos
# =============================================================================
# Este script configura sccache para acelerar la compilación de Rust
# 
# Uso: bash scripts/setup-sccache.sh
# =============================================================================

set -e

echo "=============================================="
echo "  SHIELD PROJECT - Setup de sccache"
echo "=============================================="
echo ""

# Verificar si sccache ya está instalado
if command -v sccache &> /dev/null; then
    echo "[✓] sccache ya está instalado"
    SCCACHE_VERSION=$(sccache --version | head -n1)
    echo "    $SCCACHE_VERSION"
else
    echo "[!] sccache no está instalado"
    echo ""
    echo "Instalando sccache desde pkg..."
    
    # Intentar instalar desde pkg (Termux)
    if command -v pkg &> /dev/null; then
        pkg update -y
        pkg install sccache -y
        echo "[✓] sccache instalado"
    else
        echo "[ERROR] No se encontró 'pkg'. ¿Estás en Termux?"
        echo ""
        echo "Instalación manual:"
        echo "  - Termux: pkg install sccache"
        echo "  - Linux: cargo install sccache"
        echo "  - macOS: brew install sccache"
        exit 1
    fi
fi

echo ""
echo "----------------------------------------------"
echo "  Configurando variables de entorno"
echo "----------------------------------------------"

# Detectar shell
SHELL_NAME=$(basename "$SHELL")
RC_FILE=""

case "$SHELL_NAME" in
    bash)
        RC_FILE="$HOME/.bashrc"
        ;;
    zsh)
        RC_FILE="$HOME/.zshrc"
        ;;
    fish)
        RC_FILE="$HOME/.config/fish/config.fish"
        ;;
    *)
        echo "[!] Shell no reconocido: $SHELL_NAME"
        echo "    Se usará .bashrc por defecto"
        RC_FILE="$HOME/.bashrc"
        ;;
esac

echo "Shell detectado: $SHELL_NAME"
echo "Archivo de configuración: $RC_FILE"
echo ""

# Verificar si ya está configurado
if grep -q "RUSTC_WRAPPER=sccache" "$RC_FILE" 2>/dev/null; then
    echo "[✓] RUSTC_WRAPPER ya está configurado en $RC_FILE"
else
    echo "[*] Agregando RUSTC_WRAPPER a $RC_FILE..."
    
    # Agregar configuración
    cat >> "$RC_FILE" << 'EOF'

# sccache para compilación rápida de Rust (Shield Project)
export RUSTC_WRAPPER=sccache
export SCCACHE_DIR="$HOME/.cache/sccache"
export SCCACHE_CACHE_SIZE="2G"
EOF
    
    echo "[✓] Configuración agregada"
fi

# Para fish shell, la sintaxis es diferente
if [ "$SHELL_NAME" = "fish" ]; then
    if ! grep -q "RUSTC_WRAPPER" "$RC_FILE" 2>/dev/null; then
        cat >> "$RC_FILE" << 'EOF'

# sccache para compilación rápida de Rust (Shield Project)
set -x RUSTC_WRAPPER sccache
set -x SCCACHE_DIR "$HOME/.cache/sccache"
set -x SCCACHE_CACHE_SIZE "2G"
EOF
    fi
fi

echo ""
echo "----------------------------------------------"
echo "  Iniciando sccache"
echo "----------------------------------------------"

# Iniciar sccache en background
if sccache --start-server 2>/dev/null; then
    echo "[✓] Servidor de sccache iniciado"
else
    echo "[!] No se pudo iniciar el servidor (puede que ya esté corriendo)"
fi

echo ""
echo "----------------------------------------------"
echo "  Verificación"
echo "----------------------------------------------"

# Mostrar estadísticas actuales
echo "Estadísticas de sccache:"
sccache --show-stats 2>/dev/null || echo "  (sin datos aún)"

echo ""
echo "=============================================="
echo "  ¡Setup completado!"
echo "=============================================="
echo ""
echo "Para aplicar los cambios, ejecuta:"
echo ""
if [ "$SHELL_NAME" != "fish" ]; then
    echo "  source $RC_FILE"
else
    echo "  source $RC_FILE"
fi
echo ""
echo "O reinicia tu terminal."
echo ""
echo "----------------------------------------------"
echo "  Comandos útiles de sccache"
echo "----------------------------------------------"
echo ""
echo "  sccache --show-stats     # Ver estadísticas de caché"
echo "  sccache --zero-stats     # Limpiar estadísticas"
echo "  sccache --stop-server    # Detener servidor"
echo "  sccache -s               # Alias corto para stats"
echo ""
echo "----------------------------------------------"
echo "  Mejora esperada"
echo "----------------------------------------------"
echo ""
echo "  Build limpio:     60s → 20-30s (2-3x más rápido)"
echo "  Build incremental: 10s → 2-5s  (2-4x más rápido)"
echo ""
