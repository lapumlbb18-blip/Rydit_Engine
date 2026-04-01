#!/data/data/com.termux/files/usr/bin/bash
# sync_drive.sh - Sincronizar RyDit con Google Drive (segundo plano)
# v0.11.0 - Solo binarios clave + código

set -e

PROJECT_DIR="/data/data/com.termux/files/home/shield-project"
DRIVE_DIR="/sdcard/Download/RyDit-Backup"
LOG_FILE="$PROJECT_DIR/.sync_drive.log.binaries"
EXCLUDE_FILE="$PROJECT_DIR/.sync_exclude"
PID_FILE="$PROJECT_DIR/.sync.pid"

# Verificar si ya está ejecutando
if [ -f "$PID_FILE" ]; then
    OLD_PID=$(cat "$PID_FILE")
    if kill -0 "$OLD_PID" 2>/dev/null; then
        echo "⚠️ Sincronización ya en progreso (PID: $OLD_PID)"
        exit 0
    fi
fi

# Guardar PID
echo $$ > "$PID_FILE"

echo "🛡️ Sincronizando RyDit con Google Drive..."
echo "Inicio: $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# Crear directorio de backup si no existe
mkdir -p "$DRIVE_DIR"

# Sincronizar código y documentación (excluyendo target/)
echo "📁 Sincronizando código y documentación..."
rsync -av --delete \
    --exclude-from="$EXCLUDE_FILE" \
    "$PROJECT_DIR/" \
    "$DRIVE_DIR/" 2>&1 | head -20

# Sincronizar binarios clave (solo release)
echo ""
echo "📦 Sincronizando binarios de prueba..."
if [ -d "$PROJECT_DIR/target/release" ]; then
    rsync -av \
        --include "demo_toolkit_ry" \
        --include "test_sdl2_*" \
        --include "rybot_cli" \
        --include "*.d" \
        --exclude "*" \
        "$PROJECT_DIR/target/release/" \
        "$DRIVE_DIR/target/release/" 2>&1 | head -20
    
    # Log de binarios sincronizados
    echo "" >> "$LOG_FILE"
    echo "$(date '+%Y-%m-%d %H:%M:%S') - Binarios sincronizados:" >> "$LOG_FILE"
    ls -lh "$PROJECT_DIR/target/release/" 2>/dev/null | grep -E "demo_|test_|rybot|\.d$" >> "$LOG_FILE" 2>/dev/null || true
fi

# Limpiar PID file
rm -f "$PID_FILE"

echo ""
echo "✅ Sincronización completada"
echo "Fin: $(date '+%Y-%m-%d %H:%M:%S')"
echo "Log: $LOG_FILE"
