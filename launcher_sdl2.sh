#!/data/data/com.termux/files/usr/bin/bash
# launcher_sdl2.sh - Launcher para apps SDL2 con GPU activada
# Uso: ./launcher_sdl2.sh <nombre_de_la_app>

# Configurar entorno GPU - Zink + DRI3 1
export MESA_LOADER_DRIVER_OVERRIDE=zink
export GALLIUM_DRIVER=zink
export DRI3=1
export LIBGL_ALWAYS_SOFTWARE=0  # Forzar GPU hardware

# ✅ v0.13.0: Configurar SDL2 para Android/Termux-X11
export SDL_VIDEODRIVER=x11
export SDL_RENDER_DRIVER=opengles2
export SDL_HINT_VIDEO_X11_FORCE_EGL=1
export SDL_HINT_ANDROID_SEPARATE_MOUSE_AND_TOUCH=1
export SDL_HINT_TOUCH_MOUSE_EVENTS=1
export SDL_HINT_ENABLE_SCREEN_KEYBOARD=1

# Directorio del proyecto
cd /data/data/com.termux/files/home/shield-project

# Verificar argumentos
if [ -z "$1" ]; then
    echo "========================================"
    echo "  RyDit SDL2 Launcher"
    echo "========================================"
    echo ""
    echo "Uso: ./launcher_sdl2.sh <nombre_app>"
    echo ""
    echo "Apps disponibles:"
    echo "  - test_sdl2_sprite_debug"
    echo "  - test_sdl2_sprite_simple"
    echo "  - test_sdl2_sprites"
    echo "  - test_sdl2_basico"
    echo "  - test_sdl2_simple"
    echo "  - test_sdl2_ttf"
    echo ""
    echo "Ejemplo: ./launcher_sdl2.sh test_sdl2_sprite_debug"
    echo ""
    exit 1
fi

# Binario a ejecutar
BINARIO="./target/release/$1"

# Verificar que existe
if [ ! -f "$BINARIO" ]; then
    echo "❌ Error: No existe $BINARIO"
    echo ""
    echo "¿Compilaste la app primero?"
    echo "  cargo build --release --bin $1"
    echo ""
    exit 1
fi

# Información
echo "========================================"
echo "  Lanzando: $1"
echo "  GPU: Zink (Vulkan → OpenGL)"
echo "  DISPLAY: $DISPLAY"
echo "========================================"
echo ""

# Ejecutar
$BINARIO

# Código de salida
EXIT_CODE=$?
echo ""
echo "========================================"
echo "  App cerrada (código: $EXIT_CODE)"
echo "========================================"

exit $EXIT_CODE
