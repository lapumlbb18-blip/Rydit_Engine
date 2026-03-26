#!/bin/bash
# Launcher para Snake Game - RyDit v0.1.0
# Este script mantiene la ventana abierta incluso después de Game Over

cd /data/data/com.termux/files/home/shield-project

echo "========================================"
echo "  🐍 Snake Game - RyDit v0.1.0"
echo "  Construido 100% en Android/Termux"
echo "========================================"
echo ""
echo "Controles:"
echo "  Flechas: Mover la serpiente"
echo "  SPACE:   Reiniciar después de Game Over"
echo "  ESC:     Salir del juego"
echo ""
echo "Iniciando..."
echo ""

# Ejecutar Snake en loop (se reinicia si se cierra)
while true; do
    ./target/debug/rydit-rs --gfx snake_completo.rydit
    
    # Si el usuario presionó ESC, salir del loop
    echo ""
    echo "¿Jugar de nuevo? (s/n)"
    read -r respuesta
    if [[ "$respuesta" != "s" && "$respuesta" != "S" && "$respuesta" != "" ]]; then
        break
    fi
done

echo ""
echo "¡Gracias por jugar Snake en RyDit!"
echo "GitHub: github.com/tu-usuario/rydit-language"
echo ""
