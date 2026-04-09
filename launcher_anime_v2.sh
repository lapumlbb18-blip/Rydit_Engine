#!/data/data/com.termux/files/usr/bin/bash
# ============================================================================
# 🛡️ RyDit - Launcher: Snake Anime v2
# Pipeline: Zink + DRI3 + OpenGL ES → Termux-X11 (:0)
#
# 🐍 Snake + 🍎 Manzanas + 💣 Bombas + 👾 Entidades + 🗺️ Minimap
# WASD/Flechas: Mover | R: Reiniciar | F: Debug | ESC: Salir
# ============================================================================

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}🐍 RyDit - Snake Anime v2${NC}"
echo -e "${CYAN}=============================${NC}"

# Detectar DISPLAY
if [ -n "$DISPLAY" ]; then
    DETECTED_DISPLAY="$DISPLAY"
    echo -e "${GREEN}✅ DISPLAY: $DETECTED_DISPLAY${NC}"
else
    DETECTED_DISPLAY=$(cat /proc/*/environ 2>/dev/null | tr '\0' '\n' | grep -m1 "^DISPLAY=" | cut -d= -f2)
    if [ -n "$DETECTED_DISPLAY" ]; then
        echo -e "${GREEN}✅ DISPLAY detectada: $DETECTED_DISPLAY${NC}"
    else
        DETECTED_DISPLAY=":0"
        echo -e "${YELLOW}⚠️  Usando default: $DETECTED_DISPLAY${NC}"
    fi
fi

echo ""
echo -e "${CYAN}Pipeline: Zink + DRI3 → Termux-X11 ($DETECTED_DISPLAY)${NC}"
echo ""

export DISPLAY="$DETECTED_DISPLAY"
export MESA_LOADER_DRIVER_OVERRIDE=zink
export GALLIUM_DRIVER=zink
export __GL_SYNC_TO_VBLANK=0

cd /data/data/com.termux/files/home/shield-project
echo -e "${GREEN}🚀 Lanzando Snake Anime v2...${NC}"
echo -e "${YELLOW}WASD/Flechas: Mover | R: Reiniciar | F: Debug | ESC: Salir${NC}"
echo ""

./target/release/demo_anime_ry_v2

echo ""
echo -e "${GREEN}✅ Snake Anime v2 cerrado${NC}"
