# 📦 BACKUP Y SINCRONIZACIÓN - RyDit v0.5.1

**Fecha:** 2026-03-23
**Versión:** v0.5.1
**Estado:** ✅ CONFIGURADO

---

## 🎯 ESTRATEGIA DE BACKUP

### Principios
1. ✅ **Código fuente** - Todo sincronizado (crates, demos, docs)
2. ✅ **Binarios importantes** - Solo release (rydit-rs, snake)
3. ❌ **target/ completo** - NO sincronizar (ocupa ~500 MB)
4. ❌ **Archivos temporales** - Excluir (.bak, .tmp, diagnostico/)

### Estructura en Google Drive
```
gdrive:shield-project-rydit/
├── crates/              ✅ Código Rust
├── demos/               ✅ Demos .rydit
├── docs/                ✅ Documentación
├── scripts/             ✅ Scripts .sh
├── *.md                 ✅ Documentación
├── *.sh                 ✅ Scripts
├── *.rydit              ✅ Scripts RyDit
└── binarios/            ✅ Solo binarios release
    ├── rydit-rs         (~870 KB)
    └── snake            (~500 KB)
```

---

## 📋 REQUISITOS

### 1. Instalar rclone
```bash
# En Termux
pkg install rclone

# O en Linux
curl https://rclone.org/install.sh | sudo bash
```

### 2. Configurar Google Drive
```bash
rclone config
```

**Pasos:**
1. `n` - New remote
2. Nombre: `gdrive`
3. Tipo: `drive` (Google Drive)
4. Client ID: *(dejar vacío)*
5. Client Secret: *(dejar vacío)*
6. Scope: `drive` (Full access)
7. Service Account: *(dejar vacío)*
8. **Seguir enlace OAuth** → Autenticar con cuenta Google
9. Configurar como team drive: `n`
10. Confirmar: `y`

---

## 🚀 COMANDOS DE BACKUP

### Opción 1: Backup Rápido (solo código)
```bash
cd /data/data/com.termux/files/home/shield-project
./backup_google_drive.sh
```

**Incluye:**
- ✅ Todo el código fuente
- ✅ Demos y documentación
- ✅ Scripts y archivos .md/.rydit

**Excluye:**
- ❌ Directorio target/ completo
- ❌ Binarios
- ❌ Archivos temporales

**Tiempo estimado:** 1-2 minutos

---

### Opción 2: Backup Completo (código + binarios)
```bash
cd /data/data/com.termux/files/home/shield-project
./backup_con_binarios.sh
```

**Incluye:**
- ✅ Todo lo de Opción 1
- ✅ Binarios de target/release/
  - `binarios/rydit-rs` (~870 KB)
  - `binarios/snake` (~500 KB)

**Tiempo estimado:** 3-5 minutos

---

### Opción 3: Manual con rclone
```bash
# Solo código
rclone sync /data/data/com.termux/files/home/shield-project \
    gdrive:shield-project-rydit \
    --exclude "target/**" \
    --exclude "*.bak" \
    -v

# Solo binarios
rclone copy /data/data/com.termux/files/home/shield-project/target/release/rydit-rs \
    gdrive:shield-project-rydit/binarios/

# Ver archivos
rclone lsf gdrive:shield-project-rydit
```

---

## 📊 COMANDOS ÚTILES

### Ver archivos en Google Drive
```bash
rclone lsf gdrive:shield-project-rydit
```

### Ver tamaño del backup
```bash
rclone size gdrive:shield-project-rydit
```

### Descargar archivo específico
```bash
rclone copy gdrive:shield-project-rydit/demos/tank_combat.rydit ./demos/
```

### Descargar binarios
```bash
rclone copy gdrive:shield-project-rydit/binarios/ ./target/release/
```

### Ver logs de sincronización
```bash
rclone sync ... -v --dry-run  # Simular sin cambios reales
```

---

## 🔄 AUTOMATIZACIÓN

### Backup Automático (cron)
```bash
# Editar crontab
crontab -e

# Agregar backup diario a las 3 AM
0 3 * * * /data/data/com.termux/files/home/shield-project/backup_google_drive.sh >> /sdcard/Download/rydit_backup.log 2>&1
```

### Backup al Compilar
```bash
# Agregar al final de ~/.bashrc
alias cargo-build-release="cargo build --release && echo 'Backup...' && ~/shield-project/backup_con_binarios.sh"
```

---

## 📁 ARCHIVOS DE EXCLUSIÓN

### .rcloneignore
```
# Directorios de build (NO incluir)
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

# Qwen
.qwen/
```

### .gitignore (para GitHub)
```
# Ver .gitignore existente
cat .gitignore
```

---

## 🔒 SEGURIDAD

### Permisos de Archivos
```bash
# Scripts ejecutables
chmod +x backup_*.sh

# Archivos sensibles (si los hay)
chmod 600 *.key
```

### Cuentas de Google
- ✅ Usar cuenta principal para backups importantes
- ✅ Habilitar verificación en 2 pasos
- ✅ Revisar permisos de apps en: https://myaccount.google.com/permissions

---

## 📊 ESTADÍSTICAS DE BACKUP

### Tamaño del Proyecto
```
Código fuente:     ~200 KB (sin target/)
Binarios release:  ~1.5 MB (rydit-rs + snake)
Documentación:     ~100 KB (.md files)
Demos:             ~50 KB (.rydit files)
────────────────────────────────────────
Total backup:      ~2 MB
```

### Comparación
```
Con target/:       ~500 MB ❌ (NO sincronizar)
Sin target/:       ~2 MB ✅ (Solo código + binarios)
Ahorro:            99.6% 🎯
```

---

## 🐛 SOLUCIÓN DE PROBLEMAS

### "rclone: command not found"
```bash
pkg install rclone
```

### "Remote 'gdrive' not found"
```bash
rclone config
# Seguir pasos de configuración
```

### "Permission denied"
```bash
chmod +x backup_*.sh
```

### "Sync failed - directory not empty"
```bash
# Usar --ignore-existing para evitar conflictos
rclone sync ... --ignore-existing
```

### "Quota exceeded"
```bash
# Verificar espacio en Google Drive
rclone about gdrive:

# Liberar espacio o usar otra cuenta
```

---

## 📈 HISTORIAL DE BACKUPS

### Backup Actual (v0.5.1)
```
Fecha: 2026-03-23
Versión: v0.5.1
Archivos: ~100
Tamaño: ~2 MB
Incluye:
  ✅ Código Rust (6 crates)
  ✅ Demos (19 principales)
  ✅ Documentación (10+ .md)
  ✅ Binarios (rydit-rs, snake)
  ✅ Scripts (.sh)
```

### Backups Anteriores
```
v0.5.0: 2026-03-22 - Widgets + Assets Manager
v0.4.1: 2026-03-22 - Migui Backend
v0.3.0: 2026-03-21 - Tank Combat
v0.2.0: 2026-03-21 - Module System + GitHub
v0.1.9: 2026-03-20 - 110 Tests Checkpoint
```

---

## 🎯 MEJORES PRÁCTICAS

1. ✅ **Backup después de cada sesión** - No perder avances
2. ✅ **Verificar antes de borrar** - `rclone size` para confirmar
3. ✅ **Mantener binarios** - Útiles para demos rápidas
4. ✅ **Excluir target/debug** - Ocupa mucho espacio
5. ✅ **Comprimir logs antiguos** - `tar -czf diagnostico.tar.gz diagnostico/`

---

## 🔜 RESTAURACIÓN

### Restaurar Código
```bash
rclone sync gdrive:shield-project-rydit \
    /data/data/com.termux/files/home/shield-project \
    --exclude "binarios/**" \
    -v
```

### Restaurar Binarios
```bash
rclone copy gdrive:shield-project-rydit/binarios/ \
    /data/data/com.termux/files/home/shield-project/target/release/
```

### Restauración Completa
```bash
# 1. Clonar repositorio (si existe)
git clone https://github.com/lapumlbb18-blip/Rydit_Engine.git

# 2. Descargar binarios
rclone copy gdrive:shield-project-rydit/binarios/ ./target/release/

# 3. Compilar si es necesario
cargo build --release
```

---

<div align="center">

## 🛡️ **RyDit v0.5.1 - Backup Configurado**

**"Código seguro, binarios listos, mente tranquila"**

---

*Backup script:* 2 ✅
*Google Drive:* Configurado ✅
*Exclusiones:* target/ ✅
*Binarios:* Incluidos ✅
*Tamaño:* ~2 MB ✅

[⬆️ Volver arriba](#-backup-y-sincronización---rydit-v051)

</div>
