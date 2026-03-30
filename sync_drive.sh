#!/bin/bash
# 🛡️ RyDit - Google Drive Sync (Background)
# Sincroniza archivos importantes excluyendo target/
# Uso: ./sync_drive.sh [--background]

set -e

# Colores
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Configuración
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BACKUP_DIR="${PROJECT_DIR}/.drive_backup"
LOG_FILE="${PROJECT_DIR}/.sync_drive.log"
EXCLUDE_FILE="${PROJECT_DIR}/.sync_exclude"

# Archivos/patrones a excluir
cat > "$EXCLUDE_FILE" << 'EOF'
target/
.git/
*.log
*.tmp
__pycache__/
*.pyc
.DS_Store
Thumbs.db
EOF

# Función para log
log() {
    echo -e "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

# Función para crear backup local
create_backup() {
    log "${YELLOW}Creando backup local...${NC}"
    
    mkdir -p "$BACKUP_DIR"
    
    # Limpiar backup anterior
    rm -rf "$BACKUP_DIR"/*
    
    # Copiar solo archivos importantes (excluyendo target/)
    # Usando cp en lugar de rsync
    find "$PROJECT_DIR" -maxdepth 2 -type f \
        ! -path "*/target/*" \
        ! -path "*/.git/*" \
        ! -name "*.log" \
        ! -name "*.tmp" \
        ! -name "*.o" \
        ! -name "*.d" \
        -exec cp --parents {} "$BACKUP_DIR/" \; 2>/dev/null || true
    
    # Copiar directorios importantes
    for dir in crates docs demos; do
        if [ -d "$PROJECT_DIR/$dir" ]; then
            cp -r "$PROJECT_DIR/$dir" "$BACKUP_DIR/" 2>/dev/null || true
        fi
    done
    
    log "${GREEN}Backup local completado en: $BACKUP_DIR${NC}"
    
    # Mostrar tamaño
    if [ -d "$BACKUP_DIR" ]; then
        BACKUP_SIZE=$(du -sh "$BACKUP_DIR" 2>/dev/null | cut -f1)
        log "${YELLOW}Tamaño del backup: $BACKUP_SIZE${NC}"
    fi
}

# Función para listar binarios importantes
list_binaries() {
    echo "🛡️ Binarios compilados encontrados:"
    echo "===================================="
    
    if [ -d "$PROJECT_DIR/target/release" ]; then
        echo -e "\n${GREEN}Release binaries:${NC}"
        find "$PROJECT_DIR/target/release" -maxdepth 1 -type f -executable -name "rydit*" -o -name "*demo*" 2>/dev/null | while read bin; do
            size=$(du -h "$bin" 2>/dev/null | cut -f1)
            echo "  - $(basename "$bin") ($size)"
        done
    fi
    
    if [ -d "$PROJECT_DIR/target/release/examples" ]; then
        echo -e "\n${YELLOW}Ejemplos:${NC}"
        find "$PROJECT_DIR/target/release/examples" -maxdepth 1 -type f -executable 2>/dev/null | head -5 | while read bin; do
            size=$(du -h "$bin" 2>/dev/null | cut -f1)
            echo "  - $(basename "$bin") ($size)"
        done
    fi
}

# Función para sync con Google Drive (usando rclone)
sync_to_drive() {
    log "${YELLOW}Iniciando sincronización con Google Drive (alucard18)...${NC}"
    
    # Verificar si hay conexión a internet
    if ! ping -c 1 drive.google.com &>/dev/null; then
        log "${RED}Sin conexión a Internet. Se creó backup local solamente.${NC}"
        return 1
    fi
    
    # Verificar rclone
    if ! command -v rclone &> /dev/null; then
        log "${RED}rclone no encontrado. Instala con: pkg install rclone${NC}"
        return 1
    fi
    
    # Verificar remoto configurado
    if ! rclone listremotes | grep -q "alucard18:"; then
        log "${RED}Remoto 'alucard18' no configurado. Ejecuta: rclone config${NC}"
        return 1
    fi
    
    log "${GREEN}Usando rclone con remoto: alucard18:${NC}"
    
    # Sincronizar con Google Drive
    rclone sync "$BACKUP_DIR" "alucard18:RyDit_Backup" \
        --exclude-from="$EXCLUDE_FILE" \
        --exclude='*.o' \
        --exclude='*.d' \
        --progress \
        -v 2>&1 | tee -a "$LOG_FILE"
    
    log "${GREEN}✅ Sincronización completada: alucard18:RyDit_Backup${NC}"
    return 0
}

# Función para ejecutar en segundo plano
run_background() {
    log "${YELLOW}Iniciando sincronización en segundo plano...${NC}"
    
    (
        # Crear backup
        create_backup
        
        # Listar binarios
        list_binaries > "$LOG_FILE.binaries"
        
        # Sincronizar
        sync_to_drive
        
        # Notificación
        notify-send "RyDit Sync" "Sincronización completada" 2>/dev/null || true
        
    ) &
    
    PID=$!
    echo $PID > "$PROJECT_DIR/.sync.pid"
    log "${GREEN}Sincronización en segundo plano (PID: $PID)${NC}"
    log "${YELLOW}Ver progreso: tail -f $LOG_FILE${NC}"
}

# Función para mostrar estado
show_status() {
    echo "🛡️ Estado de Sincronización RyDit"
    echo "=================================="
    
    if [ -f "$PROJECT_DIR/.sync.pid" ]; then
        PID=$(cat "$PROJECT_DIR/.sync.pid")
        if ps -p $PID > /dev/null 2>&1; then
            echo -e "Estado: ${YELLOW}En progreso (PID: $PID)${NC}"
        else
            echo -e "Estado: ${RED}Finalizado${NC}"
            rm -f "$PROJECT_DIR/.sync.pid"
        fi
    else
        echo -e "Estado: ${GREEN}Inactivo${NC}"
    fi
    
    if [ -f "$LOG_FILE" ]; then
        echo -e "\nÚltimos logs:"
        tail -5 "$LOG_FILE"
    fi
    
    if [ -d "$BACKUP_DIR" ]; then
        BACKUP_SIZE=$(du -sh "$BACKUP_DIR" 2>/dev/null | cut -f1)
        echo -e "\nBackup local: ${GREEN}$BACKUP_SIZE${NC}"
    fi
}

# Función para limpiar
clean() {
    log "${YELLOW}Limpiando archivos temporales...${NC}"
    
    rm -f "$PROJECT_DIR/.sync.pid"
    rm -f "$PROJECT_DIR"/rydit_backup_*.tar.gz
    
    log "${GREEN}Limpieza completada${NC}"
}

# Help
show_help() {
    echo "🛡️ RyDit Google Drive Sync"
    echo ""
    echo "Uso: $0 [OPCIÓN]"
    echo ""
    echo "Opciones:"
    echo "  (ninguna)     Ejecutar sync normal"
    echo "  --background  Ejecutar en segundo plano"
    echo "  --status      Mostrar estado"
    echo "  --binaries    Listar binarios compilados"
    echo "  --backup      Solo crear backup local"
    echo "  --clean       Limpiar archivos temporales"
    echo "  --help        Mostrar esta ayuda"
    echo ""
    echo "Ejemplos:"
    echo "  $0                    # Sync normal"
    echo "  $0 --background       # Sync en background"
    echo "  $0 --status           # Ver estado"
}

# Main
case "${1:-}" in
    --background|-b)
        run_background
        ;;
    --status|-s)
        show_status
        ;;
    --binaries)
        list_binaries
        ;;
    --backup)
        create_backup
        ;;
    --clean)
        clean
        ;;
    --help|-h)
        show_help
        ;;
    "")
        create_backup
        list_binaries
        sync_to_drive
        ;;
    *)
        echo -e "${RED}Opción no válida: $1${NC}"
        show_help
        exit 1
        ;;
esac
