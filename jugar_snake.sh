#!/bin/bash
# Ejecutar Snake Game - Binario Dedicado
# No se cierra automáticamente, espera que presiones ESC

cd /data/data/com.termux/files/home/shield-project

echo "🐍 Snake Game - RyDit v0.1.0"
echo "Construido 100% en Android/Termux"
echo ""
echo "Controles:"
echo "  Flechas: Mover"
echo "  SPACE:   Restart"
echo "  ESC:     Salir"
echo ""

# Ejecutar binario dedicado
./target/debug/snake

echo ""
echo "¡Juego terminado!"
