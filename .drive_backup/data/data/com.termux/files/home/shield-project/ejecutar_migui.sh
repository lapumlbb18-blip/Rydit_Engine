#!/data/data/com.termux/files/usr/bin/bash
# Script para ejecutar la demo migui backend v0.4.1

echo "=========================================="
echo "  RyDit migui v0.4.1 - Demo Backend"
echo "=========================================="
echo ""
echo "Ejecutando migui con backend raylib..."
echo "Presiona ESC para salir"
echo ""

cd /data/data/com.termux/files/home/shield-project
./target/release/rydit-rs --migui demos/demo_migui_backend.rydit

echo ""
echo "Demo finalizado."
