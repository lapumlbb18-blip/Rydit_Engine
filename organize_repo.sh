#!/bin/bash
# 🛡️ organize_repo.sh - Organiza el repositorio RyDit v0.10.0
# Fecha: 2026-03-29

set -e

echo "🛡️ Organizando repositorio RyDit para v0.10.0..."
echo ""

# Crear carpetas de destino
echo "📁 Creando carpetas de destino..."
mkdir -p docs/sessions
mkdir -p docs/archive
mkdir -p archive/backups

# 1. Mover archivos de documentación a docs/
echo "📁 Moviendo archivos de documentación..."
for file in ARBOL_TAREAS_SISTEMA_UNIVERSAL_RY.md \
            ASSETS_MANAGER_IMPLEMENTADO.md \
            GUIA_ENTITY_SYSTEM_V0.9.0.md \
            GUIA_RAPIDA_TERMUX_X11_V0.9.0.md \
            HTTP_WEBSOCKET_IMPLEMENTADO.md \
            MANIFIESTO.md \
            PROGRESO_SISTEMA_UNIVERSAL_RY.md \
            SISTEMA_UNIVERSAL_RY_COMPLETADO.md \
            SISTEMA_UNIVERSAL_RY_ESTADO_REAL.md \
            TAREAS_PROXIMA_SESION.md \
            TODO_V0.9.1.md \
            VERIFICACION_PRODUCCION_V0.9.0.md \
            EVALUACION_HONESTA_V0.9.0_CON_VISION.txt \
            PLAN_DE_ACCION_V0.9.0_A_V1.0.0.txt \
            VISION_FINAL_DEL_CREADOR.txt \
            ARQUITECTURA_FINAL_CLARA_MAESTRO.txt \
            ARQUITECTURA_RYDIT_VERDAD_CLARA.txt \
            CAMBIOS_V0.10.0.md; do
    if [ -f "$file" ]; then
        mv "$file" docs/
        echo "  ✓ $file → docs/"
    fi
done

# 2. Mover archivos históricos a docs/archive/
echo ""
echo "📁 Moviendo archivos históricos a docs/archive/..."
for file in ESTADO_DEL_CODIGO_V0.8.4.md \
            HALLAZGOS_2026_03_27.md \
            PLANIFICACION_V0.5.1_AUDIO_HTTP.md \
            PLANIFICACION_V0.8.3_V0.9.0.md \
            ROADMAP_ACTUALIZADO_V0.9.0.md \
            README_EN.md \
            SESION_ACTUAL_PROXIMOS_PASOS.md; do
    if [ -f "$file" ]; then
        mv "$file" docs/archive/
        echo "  ✓ $file → docs/archive/"
    fi
done

# 3. Mover sesiones a docs/sessions/
echo ""
echo "📁 Moviendo sesiones a docs/sessions/..."
for file in SESION_*.md; do
    if [ -f "$file" ]; then
        mv "$file" docs/sessions/
        echo "  ✓ $file → docs/sessions/"
    fi
done

# 4. Mover CHANGELOGs a docs/archive/
echo ""
echo "📁 Moviendo CHANGELOGs a docs/archive/..."
if [ -d "docs" ]; then
    for changelog in docs/CHANGELOG_*.md; do
        if [ -f "$changelog" ] && [ "$changelog" != "docs/CHANGELOG.md" ]; then
            mv "$changelog" docs/archive/
            echo "  ✓ $changelog → docs/archive/"
        fi
    done
fi

# 5. Consolidar backups en archive/backups/
echo ""
echo "📁 Consolidando backups en archive/backups/..."
for dir in docs/backup_*/; do
    if [ -d "$dir" ]; then
        dirname=$(basename "$dir")
        mv "$dir" archive/backups/
        echo "  ✓ $dirname → archive/backups/"
    fi
done

# 6. Mover carpetas temporales a archive/
echo ""
echo "📁 Moviendo carpetas temporales a archive/..."
for dir in historial ruta_v0.5.0_v0.9.9 context; do
    if [ -d "$dir" ]; then
        mv "$dir" archive/
        echo "  ✓ $dir → archive/"
    fi
done

# 7. Eliminar carpetas temporales de prueba
echo ""
echo "🗑️ Eliminando carpetas de prueba..."
rm -rf test_dir3/ 2>/dev/null && echo "  ✓ test_dir3/ eliminada" || true
rm -rf __pycache__/ 2>/dev/null && echo "  ✓ __pycache__/ eliminado" || true

# 8. Mover otros archivos de planificación
echo ""
echo "📁 Moviendo archivos de planificación..."
for file in PLANIFICACION_*.md TODO_*.md; do
    if [ -f "$file" ]; then
        mv "$file" docs/archive/
        echo "  ✓ $file → docs/archive/"
    fi
done

echo ""
echo "✅ ¡Organización completada!"
echo ""
echo "📊 Resumen:"
echo "  ─────────────────────────────────────"
echo "  📄 docs/              : Documentación pública"
echo "  📄 docs/archive/      : Documentación histórica"
echo "  📄 docs/sessions/     : Resúmenes de sesión"
echo "  📄 archive/backups/   : Todos los backups consolidados"
echo "  📄 archive/           : Carpetas temporales"
echo "  ─────────────────────────────────────"
echo ""
echo "🎯 Estructura lista para v0.10.0"
echo ""
echo "📝 Próximos pasos:"
echo "  1. Revisar git status"
echo "  2. Actualizar .gitignore si es necesario"
echo "  3. Commit: 'chore: organize repository for v0.10.0'"
echo "  4. ¡Comenzar GPU Instancing!"
echo ""
