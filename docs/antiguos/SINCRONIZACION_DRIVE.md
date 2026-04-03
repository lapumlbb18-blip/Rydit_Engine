# 🔄 Sincronización RyDit con Google Drive

**Versión**: v0.11.0
**Última actualización**: 2026-04-01

---

## 📋 CONFIGURACIÓN

### **Archivos de Configuración**

| Archivo | Propósito |
|---------|-----------|
| `.sync_exclude` | Lista de exclusiones |
| `.sync_drive.log.binaries` | Log de binarios sincronizados |
| `sync_drive.sh` | Script de sincronización |

---

## 📁 EXCLUSIONES (.sync_exclude)

```
# No sincronizar carpetas pesadas
target/
.git/

# No sincronizar archivos temporales
*.tmp
*.log
*.swp
*~

# Excepciones: binarios de prueba clave
!target/release/demo_toolkit_ry
!target/release/test_sdl2_*
!target/release/rybot_cli
!target/release/*.d
```

---

## 🔧 SCRIPT DE SINCRONIZACIÓN (sync_drive.sh)

```bash
#!/data/data/com.termux/files/usr/bin/bash
# sync_drive.sh - Sincronizar RyDit con Google Drive

set -e

PROJECT_DIR="/data/data/com.termux/files/home/shield-project"
DRIVE_DIR="/sdcard/Download/RyDit-Backup"
LOG_FILE="$PROJECT_DIR/.sync_drive.log.binaries"
EXCLUDE_FILE="$PROJECT_DIR/.sync_exclude"

echo "🛡️ Sincronizando RyDit con Google Drive..."
echo "Directorio: $PROJECT_DIR"
echo "Backup: $DRIVE_DIR"
echo ""

# Crear directorio de backup si no existe
mkdir -p "$DRIVE_DIR"

# Sincronizar código y documentación (excluyendo target/)
echo "📁 Sincronizando código y documentación..."
rsync -av --delete \
    --exclude-from="$EXCLUDE_FILE" \
    "$PROJECT_DIR/" \
    "$DRIVE_DIR/"

# Sincronizar binarios clave (solo release)
echo "📦 Sincronizando binarios de prueba..."
if [ -d "$PROJECT_DIR/target/release" ]; then
    rsync -av \
        --include "demo_toolkit_ry" \
        --include "test_sdl2_*" \
        --include "rybot_cli" \
        --include "*.d" \
        --exclude "*" \
        "$PROJECT_DIR/target/release/" \
        "$DRIVE_DIR/target/release/"
    
    # Log de binarios sincronizados
    echo "$(date '+%Y-%m-%d %H:%M:%S') - Binarios sincronizados:" >> "$LOG_FILE"
    ls -lh "$PROJECT_DIR/target/release/" | grep -E "demo_|test_|rybot" >> "$LOG_FILE"
    echo "" >> "$LOG_FILE"
fi

echo ""
echo "✅ Sincronización completada"
echo "Log: $LOG_FILE"
```

---

## 🚀 USO

### **Sincronización Manual**

```bash
# Ejecutar sincronización
./sync_drive.sh

# Ver log
cat .sync_drive.log.binaries
```

### **Sincronización Automática (Segundo Plano)**

```bash
# Agregar al crontab (cada 1 hora)
0 * * * * /data/data/com.termux/files/home/shield-project/sync_drive.sh

# O cada 30 minutos
*/30 * * * * /data/data/com.termux/files/home/shield-project/sync_drive.sh
```

---

## 📊 ESTADO DE SINCRONIZACIÓN

### **Archivos Sincronizados** ✅

- ✅ Todo el código fuente (`.rs`, `.toml`)
- ✅ Documentación (`.md`)
- ✅ Assets (`.png`, `.jpg`)
- ✅ Scripts (`.sh`)
- ✅ Binarios de prueba clave

### **Archivos NO Sincronizados** ❌

- ❌ `target/debug/` (compilación debug)
- ❌ `target/release/` (excepto binarios clave)
- ❌ `.git/` (repositorio)
- ❌ Archivos temporales

---

## 🔍 VERIFICACIÓN

```bash
# Verificar archivos sincronizados
ls -la /sdcard/Download/RyDit-Backup/

# Verificar binarios
ls -lh /sdcard/Download/RyDit-Backup/target/release/

# Ver log de sincronización
tail -20 .sync_drive.log.binaries
```

---

## 🛡️ SEGURIDAD

### **Backup Automático**

- ✅ Sincronización cada 30-60 minutos
- ✅ Solo archivos clave (código + binarios de prueba)
- ✅ Log de todas las sincronizaciones

### **Recuperación**

```bash
# Restaurar desde backup
rsync -av /sdcard/Download/RyDit-Backup/ /data/data/com.termux/files/home/shield-project/
```

---

## 📝 NOTAS

### **Binarios Clave**

Solo se sincronizan los binarios de prueba más importantes:

| Binario | Tamaño | Propósito |
|---------|--------|-----------|
| `demo_toolkit_ry` | ~300KB | Demo UI Toolkit |
| `test_sdl2_basico` | ~300KB | Test SDL2 básico |
| `test_sdl2_sprite_debug` | ~305KB | Debug de sprites |
| `rybot_cli` | ~280KB | RyBot CLI |
| `*.d` | ~1KB c/u | Debug info para desarrollo |

### **Espacio en Drive**

- **Código + Docs**: ~50 MB
- **Binarios clave**: ~5 MB
- **Total**: ~55 MB

---

<div align="center">

**🛡️ RyDit v0.11.0 - Sincronización Drive**

*Código ✅ | Binarios Clave ✅ | Backup Automático ✅*

</div>
