#!/bin/bash
# =============================================================================
# RyDit Engine - Sincronización a Google Drive
# =============================================================================
# Uso: ./sync_to_drive.sh [carpeta_backup]
# =============================================================================

set -e

# Colores
ROJO='\033[0;31m'
VERDE='\033[0;32m'
AMARILLO='\033[1;33m'
AZUL='\033[0;34m'
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
echo "  🌐 RyDit Engine - Sincronización a Google Drive"
echo "=================================================="
echo ""

# Verificar carpeta de backup
BACKUP_FOLDER="${1:-docs/backup_seguro_v0.8.0_scripts}"

if [ ! -d "$BACKUP_FOLDER" ]; then
    log_error "Carpeta de backup no encontrada: $BACKUP_FOLDER"
    echo ""
    echo "Carpetas de backup disponibles:"
    ls -1d docs/backup_seguro*/ 2>/dev/null | head -10
    exit 1
fi

log_info "Carpeta de backup: $BACKUP_FOLDER"

# Verificar si gdrive está instalado
if command -v gdrive &> /dev/null; then
    log_success "gdrive instalado"
    GD_CMD="gdrive"
elif command -v rclone &> /dev/null; then
    log_success "rclone instalado"
    GD_CMD="rclone"
else
    log_warning "Ni gdrive ni rclone están instalados"
    echo ""
    echo "Opciones:"
    echo "  1. Instalar gdrive: pkg install gdrive (Termux)"
    echo "  2. Instalar rclone: pkg install rclone (Termux)"
    echo "  3. Copiar manualmente la carpeta a Google Drive"
    echo ""
    log_info "¿Quieres instalar gdrive?"
    read -p "Instalar gdrive? (s/n): " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[SsY]$ ]]; then
        if [ -f "/data/data/com.termux/files/usr/bin/bash" ]; then
            log_info "Instalando gdrive..."
            pkg install gdrive -y 2>&1 | tail -3
            GD_CMD="gdrive"
            log_success "gdrive instalado"
        else
            log_error "gdrive solo disponible en Termux"
            log_info "Usa rclone o copia manual"
            exit 1
        fi
    else
        log_info "Puedes copiar manualmente la carpeta:"
        echo "  Carpeta: $(pwd)/$BACKUP_FOLDER"
        echo "  A: Google Drive > alucard18:shield-project-rydit > backup_seguro"
        exit 0
    fi
fi

# Sincronizar con gdrive
if [ "$GD_CMD" = "gdrive" ]; then
    log_info "Sincronizando con Google Drive..."
    echo ""
    
    # Verificar autenticación
    if ! gdrive list &>/dev/null; then
        log_warning "No estás autenticado en Google Drive"
        echo ""
        echo "Sigue estos pasos:"
        echo "  1. Ejecuta: gdrive init"
        echo "  2. Abre el URL en tu navegador"
        echo "  3. Autoriza la aplicación"
        echo "  4. Copia el código de autenticación"
        echo ""
        read -p "¿Ya estás autenticado? (s/n): " -n 1 -r
        echo
        
        if [[ ! $REPLY =~ ^[SsY]$ ]]; then
            gdrive init
        fi
    fi
    
    # Crear carpeta en Drive
    log_info "Buscando carpeta 'backup_seguro' en Google Drive..."
    FOLDER_ID=$(gdrive list --no-header | grep "backup_seguro" | awk '{print $1}' | head -1)
    
    if [ -z "$FOLDER_ID" ]; then
        log_info "Creando carpeta 'backup_seguro'..."
        FOLDER_ID=$(gdrive mkdir "backup_seguro" | grep "Created" | awk '{print $3}')
        log_success "Carpeta creada: $FOLDER_ID"
    else
        log_success "Carpeta encontrada: $FOLDER_ID"
    fi
    
    # Subir backup
    log_info "Subiendo backup a Google Drive..."
    echo ""
    
    # Crear subcarpeta con nombre del backup
    BACKUP_NAME=$(basename "$BACKUP_FOLDER")
    log_info "Nombre del backup: $BACKUP_NAME"
    
    # Subir archivos
    cd "$BACKUP_FOLDER"
    
    for file in *; do
        if [ -f "$file" ] || [ -d "$file" ]; then
            log_info "Subiendo: $file"
            gdrive upload -p "$FOLDER_ID" -r "$file" 2>&1 | tail -2
        fi
    done
    
    cd - > /dev/null
    
    log_success "Backup sincronizado a Google Drive"
    
    # Mostrar enlace
    echo ""
    log_info "Tu backup está en:"
    echo "  Google Drive > backup_seguro > $BACKUP_NAME"
    echo ""
    
# Sincronizar con rclone
elif [ "$GD_CMD" = "rclone" ]; then
    log_info "Sincronizando con rclone..."
    echo ""
    
    # Verificar configuración
    if ! rclone listremotes &>/dev/null; then
        log_warning "No hay remotes configurados en rclone"
        echo ""
        echo "Configura Google Drive:"
        echo "  1. Ejecuta: rclone config"
        echo "  2. Selecciona 'New remote'"
        echo "  3. Nombre: drive"
        echo "  4. Tipo: Google Drive"
        echo "  5. Sigue las instrucciones de autenticación"
        echo ""
        read -p "¿Quieres configurar rclone ahora? (s/n): " -n 1 -r
        echo
        
        if [[ $REPLY =~ ^[SsY]$ ]]; then
            rclone config
        else
            exit 1
        fi
    fi
    
    # Verificar remote 'drive'
    if ! rclone listremotes | grep -q "^drive:"; then
        log_error "No hay un remote llamado 'drive'"
        echo "Crea un remote llamado 'drive' para Google Drive"
        exit 1
    fi
    
    # Sincronizar
    log_info "Sincronizando carpeta..."
    rclone sync "$BACKUP_FOLDER" drive:backup_seguro/$(basename $BACKUP_FOLDER) --progress
    
    log_success "Backup sincronizado a Google Drive"
    echo ""
    log_info "Tu backup está en:"
    echo "  Google Drive > backup_seguro > $(basename $BACKUP_FOLDER)"
fi

echo ""
echo "=================================================="
echo "  ✅ Sincronización completada"
echo "=================================================="
echo ""

exit 0
