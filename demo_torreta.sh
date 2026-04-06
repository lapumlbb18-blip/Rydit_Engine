#!/data/data/com.termux/files/usr/bin/bash
# demo_torreta_vs_sprites - Launcher con Zink + DRI3 1
# GPU: Vulkan → OpenGL (no llvmpipe software)
# Uso: ./demo_torreta.sh

# ============================================================
# GPU: Zink (Vulkan → OpenGL translation)
# ============================================================
export MESA_LOADER_DRIVER_OVERRIDE=zink
export GALLIUM_DRIVER=zink
export DRI3=1
export LIBGL_ALWAYS_SOFTWARE=0  # Forzar GPU hardware

# ============================================================
# Display: Termux-X11
# ============================================================
export DISPLAY=:0
export SDL_VIDEODRIVER=x11
export SDL_RENDER_DRIVER=opengles2
export SDL_HINT_VIDEO_X11_FORCE_EGL=1

# ============================================================
# Android/Termux-X11 hints
# ============================================================
export SDL_HINT_ANDROID_SEPARATE_MOUSE_AND_TOUCH=1
export SDL_HINT_TOUCH_MOUSE_EVENTS=1
export SDL_HINT_ENABLE_SCREEN_KEYBOARD=1
export SDL_HINT_IME_SHOW_UI=1

# ============================================================
# Directorio del proyecto
# ============================================================
cd /data/data/com.termux/files/home/shield-project

BINARIO="./target/release/demo_torreta_vs_sprites"

if [ ! -f "$BINARIO" ]; then
    echo "Compilando demo_torreta_vs_sprites..."
    cargo build --release --bin demo_torreta_vs_sprites
    if [ $? -ne 0 ]; then
        echo "Error compilando!"
        exit 1
    fi
fi

echo "============================================"
echo "  RyDit - Torreta vs Sprites"
echo "  GPU: Zink (Vulkan -> OpenGL)"
echo "  DRI3: 1"
echo "  DISPLAY: $DISPLAY"
echo "============================================"
echo ""

# Verificar GPU activa
echo "GPU detectada:"
glxinfo 2>/dev/null | grep "OpenGL renderer" | head -1 || echo "  (glxinfo no disponible)"
echo ""

$BINARIO
EXIT_CODE=$?

echo ""
echo "============================================"
echo "  Demo cerrado (codigo: $EXIT_CODE)"
echo "============================================"

exit $EXIT_CODE
