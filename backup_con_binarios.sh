#!/data/data/com.termux/files/usr/bin/bash
# ============================================================================
# BACKUP RYDIT v0.5.3 - CON BINARIOS
# ============================================================================
# Backup completo incluyendo SOLO binarios de target/release
# Uso: ./backup_con_binarios.sh
# ============================================================================

set -e

echo "========================================"
echo "  🛡️ RyDit Backup + Binarios"
echo "========================================"
echo ""

REMOTE="alucard18"
REMOTE_DIR="shield-project-rydit"
PROJECT_DIR="/data/data/com.termux/files/home/shield-project"
BINARIES_DIR="$REMOTE_DIR/binarios"

# Verificar rclone
if ! command -v rclone &> /dev/null; then
    echo "❌ rclone no está instalado"
    echo "Instalar: pkg install rclone"
    exit 1
fi

echo "☁️  Remote: $REMOTE:$REMOTE_DIR"
echo ""

# Paso 1: Backup del código (sin target/)
echo "📦 Paso 1: Backup del código..."
rclone sync "$PROJECT_DIR" "${REMOTE}:${REMOTE_DIR}" \
    --exclude "target/**" \
    --exclude "**/target/**" \
    --exclude "*.bak" \
    --exclude "*.tmp" \
    --exclude "diagnostico/**" \
    --exclude "QWEN.md" \
    --exclude ".git/**" \
    --exclude ".qwen/**" \
    -v 2>&1 | tail -5

echo "✅ Código sincronizado"
echo ""

# Paso 2: Backup de binarios (solo release)
echo "📦 Paso 2: Backup de binarios..."

if [ -d "$PROJECT_DIR/target/release" ]; then
    # Crear directorio de binarios en Google Drive
    rclone mkdir "${REMOTE}:${BINARIES_DIR}" 2>/dev/null || true
    
    # Subir solo binarios específicos
    BINARIES=(
        "rydit-rs"
        "snake"
    )
    
    for bin in "${BINARIES[@]}"; do
        if [ -f "$PROJECT_DIR/target/release/$bin" ]; then
            echo "   Subiendo $bin..."
            rclone copyto "$PROJECT_DIR/target/release/$bin" \
                "${REMOTE}:${BINARIES_DIR}/$bin" \
                --progress
        fi
    done
    
    echo ""
    echo "✅ Binarios sincronizados en: ${REMOTE}:${BINARIES_DIR}"
else
    echo "⚠️  Directorio target/release no encontrado"
    echo "   Ejecutar: cargo build --release"
fi

echo ""
echo "========================================"
echo "  ✅ Backup Completo"
echo "========================================"
echo ""
echo "📂 Estructura en Google Drive:"
echo ""
echo "shield-project-rydit/"
echo "├── crates/           ✅"
echo "├── demos/            ✅"
echo "├── docs/             ✅"
echo "├── scripts/          ✅"
echo "├── *.md              ✅"
echo "├── *.sh              ✅"
echo "├── *.rydit           ✅"
echo "└── binarios/         ✅"
echo "    ├── rydit-rs      (~870 KB)"
echo "    └── snake         (~500 KB)"
echo ""
echo "📊 Tamaño total:"
rclone size "${REMOTE}:${REMOTE_DIR}" 2>/dev/null || echo "   (calculando...)"
echo ""
echo "🔍 Ver archivos:"
echo "   rclone lsf ${REMOTE}:${REMOTE_DIR}"
echo ""
