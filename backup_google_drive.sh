#!/data/data/com.termux/files/usr/bin/bash
# ============================================================================
# BACKUP RYDIT v0.5.3 - Google Drive con Rclone
# ============================================================================
# Uso: ./backup_google_drive.sh
# ============================================================================

set -e

echo "========================================"
echo "  🛡️ RyDit Backup - Google Drive"
echo "========================================"
echo ""

# Verificar rclone
if ! command -v rclone &> /dev/null; then
    echo "❌ rclone no está instalado"
    echo ""
    echo "Instalar con:"
    echo "  pkg install rclone"
    echo "  o"
    echo "  curl https://rclone.org/install.sh | sudo bash"
    echo ""
    exit 1
fi

echo "✅ rclone detectado: $(rclone version | head -1)"
echo ""

# Configuración
REMOTE="alucard18"
REMOTE_DIR="shield-project-rydit"
PROJECT_DIR="/data/data/com.termux/files/home/shield-project"
EXCLUDE_FILE="$PROJECT_DIR/.rcloneignore"

echo "📁 Directorio: $PROJECT_DIR"
echo "☁️  Remote: $REMOTE:$REMOTE_DIR"
echo ""

# Verificar que el remote esté configurado
if ! rclone listremotes | grep -q "^${REMOTE}:$"; then
    echo "❌ Remote '$REMOTE' no configurado"
    echo ""
    echo "Configurar con:"
    echo "  rclone config"
    echo "  1) new remote"
    echo "  2) Nombre: gdrive"
    echo "  3) Tipo: drive"
    echo "  4) Seguir autenticación OAuth"
    echo ""
    exit 1
fi

echo "✅ Remote '$REMOTE' configurado"
echo ""

# Crear archivo de exclusión si no existe
if [ ! -f "$EXCLUDE_FILE" ]; then
    echo "Creando .rcloneignore..."
    cat > "$EXCLUDE_FILE" << EOF
# Directorios de build (NO incluir en backup)
target/
target/debug/
target/release/
**/target/

# Archivos temporales
*.bak
*.tmp
*.swp
*~

# Logs y diagnóstico antiguo
diagnostico/
historial/diagnostico-old/

# QWEN.md (contexto local de sesión)
QWEN.md

# Git
.git/
.gitignore

# IDE
.vscode/
.idea/
EOF
    echo "✅ .rcloneignore creado"
    echo ""
fi

# Mostrar estadísticas del directorio
echo "📊 Estadísticas del directorio:"
echo "   Archivos totales: $(find "$PROJECT_DIR" -type f | wc -l)"
echo "   Tamaño total: $(du -sh "$PROJECT_DIR" 2>/dev/null | cut -f1)"
echo ""

# Backup EXCLUYENDO target/
echo "🔄 Iniciando backup (excluyendo target/)..."
echo ""

rclone sync "$PROJECT_DIR" "${REMOTE}:${REMOTE_DIR}" \
    --exclude "target/**" \
    --exclude "**/target/**" \
    --exclude "*.bak" \
    --exclude "*.tmp" \
    --exclude "diagnostico/**" \
    --exclude "QWEN.md" \
    --exclude ".git/**" \
    --exclude ".gitignore" \
    --exclude ".qwen/**" \
    -v --progress

echo ""
echo "========================================"
echo "  ✅ Backup Completado"
echo "========================================"
echo ""
echo "📦 Archivos sincronizados en:"
echo "   Google Drive: ${REMOTE}:${REMOTE_DIR}"
echo ""
echo "📊 Tamaño en nube:"
rclone size "${REMOTE}:${REMOTE_DIR}" 2>/dev/null || echo "   (calculando...)"
echo ""
echo "🔍 Ver archivos:"
echo "   rclone ls ${REMOTE}:${REMOTE_DIR}"
echo ""
