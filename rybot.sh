#!/data/data/com.termux/files/usr/bin/bash
# =============================================================================
# Ry-Dit - Bot de Ayuda y Demo Launcher v0.2.0
# =============================================================================
# Menú interactivo para ejecutar demos, ver documentación y aprender RyDit
# Uso: ./rybot.sh
# =============================================================================

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
ORANGE='\033[38;5;208m'
NC='\033[0m' # No Color

# Variables de entorno para gráficos
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

# =============================================================================
# FUNCIONES AUXILIARES
# =============================================================================

print_header() {
    echo -e "${CYAN}"
    echo "╔═══════════════════════════════════════════════════════════╗"
    echo "║         🛡️  RYDIT ENGINE - BOT DE AYUDA v0.2.0          ║"
    echo "║     Motor de Videojuegos 2D + Lenguaje de Scripting       ║"
    echo "╚═══════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

print_menu() {
    echo -e "${YELLOW}┌────────────────────────────────────────────────────────┐${NC}"
    echo -e "${YELLOW}│                    MENÚ PRINCIPAL                      │${NC}"
    echo -e "${YELLOW}└────────────────────────────────────────────────────────┘${NC}"
    echo ""
    echo -e "  ${GREEN}1)${NC} Ejecutar Demos Gráficos"
    echo -e "  ${GREEN}2)${NC} Ejecutar Snake Game"
    echo -e "  ${GREEN}3)${NC} Ver Sintaxis del Lenguaje"
    echo -e "  ${GREEN}4)${NC} Ver Comandos Gráficos (draw.*)"
    echo -e "  ${GREEN}5)${NC} Ver Módulos Disponibles (stdlib)"
    echo -e "  ${GREEN}6)${NC} Ejecutar Script Personalizado"
    echo -e "  ${GREEN}7)${NC} Ver Documentación Completa"
    echo -e "  ${GREEN}0)${NC} Salir"
    echo ""
}

print_syntax() {
    echo -e "${CYAN}┌────────────────────────────────────────────────────────┐${NC}"
    echo -e "${CYAN}│               SINTAXIS DEL LENGUAJE RYDIT              │${NC}"
    echo -e "${CYAN}└────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    echo -e "${YELLOW}► Variables y Asignación:${NC}"
    echo "  dark.slot x = 100          # Declarar variable"
    echo "  dark.slot nombre = \"hola\"  # String"
    echo "  dark.slot lista = [1, 2, 3] # Array"
    echo ""
    
    echo -e "${YELLOW}► Condicionales:${NC}"
    echo "  onif x > 5 voz \"Mayor\" blelse voz \"Menor\""
    echo "  onif x > 5 {"
    echo "      voz \"Mayor\""
    echo "  } blelse {"
    echo "      voz \"Menor\""
    echo "  }"
    echo ""
    
    echo -e "${YELLOW}► Ciclos:${NC}"
    echo "  ryda x {                   # while (x)"
    echo "      voz x"
    echo "      dark.slot x = x - 1"
    echo "  }"
    echo ""
    echo "  cada item en lista {       # foreach"
    echo "      voz item"
    echo "  }"
    echo ""
    
    echo -e "${YELLOW}► Funciones:${NC}"
    echo "  rytmo saludar(nombre) {"
    echo "      voz \"Hola \" + nombre"
    echo "      return 1"
    echo "  }"
    echo ""
    
    echo -e "${YELLOW}► Gráficos:${NC}"
    echo "  shield.init                # Inicializar ventana"
    echo "  draw.circle(400, 300, 50, \"rojo\")"
    echo "  draw.rect(100, 100, 100, 100, \"verde\")"
    echo "  draw.line(0, 0, 800, 600, \"azul\")"
    echo "  draw.text(\"Hola\", 200, 100, 20, \"blanco\")"
    echo ""
    
    echo -e "${YELLOW}► Imports:${NC}"
    echo "  import math                # Importar módulo"
    echo "  import arrays as arr       # Con alias"
    echo "  dark.slot suma = math::sumar(10, 5)"
    echo ""
}

print_draw_commands() {
    echo -e "${CYAN}┌────────────────────────────────────────────────────────┐${NC}"
    echo -e "${CYAN}│              COMANDOS GRÁFICOS (draw.*)                │${NC}"
    echo -e "${CYAN}└────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    echo -e "${YELLOW}► Formas Básicas:${NC}"
    echo "  draw.circle(x, y, radio, \"color\")     # Círculo"
    echo "  draw.rect(x, y, ancho, alto, \"color\") # Rectángulo"
    echo "  draw.line(x1, y1, x2, y2, \"color\")    # Línea"
    echo ""
    
    echo -e "${YELLOW}► Formas v0.2.0 (NUEVO):${NC}"
    echo "  draw.triangle(x1, y1, x2, y2, x3, y3, \"color\")  # Triángulo"
    echo "  draw.ellipse(x, y, radius_h, radius_v, \"color\") # Elipse"
    echo "  draw.rectangle_lines(x, y, w, h, \"color\")       # Rectángulo (outline)"
    echo "  draw.ring(x, y, inner, outer, \"color\")          # Anillo"
    echo "  draw.line_thick(x1, y1, x2, y2, grosor, \"color\")# Línea gruesa"
    echo ""
    
    echo -e "${YELLOW}► Texto:${NC}"
    echo "  draw.text(\"mensaje\", x, y, tamano, \"color\")"
    echo ""
    
    echo -e "${YELLOW}► Colores Disponibles:${NC}"
    echo "  rojo, verde, azul, amarillo, blanco, negro"
    echo "  cyan, magenta, naranja, rosa, morado, cafe"
    echo "  gris, lima, azuloscuro, oliva, turquesa, vino"
    echo ""
}

print_modules() {
    echo -e "${CYAN}┌────────────────────────────────────────────────────────┐${NC}"
    echo -e "${CYAN}│                 MÓDULOS DISPONIBLES                    │${NC}"
    echo -e "${CYAN}└────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    echo -e "${YELLOW}► math:${NC}"
    echo "  math::sumar(a, b), math::restar(a, b)"
    echo "  math::multiplicar(a, b), math::dividir(a, b)"
    echo "  math::pow(base, exp), math::abs(x)"
    echo "  math::min(a, b), math::max(a, b)"
    echo ""
    
    echo -e "${YELLOW}► arrays:${NC}"
    echo "  arrays::length(arr), arrays::push(arr, elem)"
    echo "  arrays::pop(arr), arrays::reverse(arr)"
    echo "  arrays::find(arr, elem), arrays::clear(arr)"
    echo ""
    
    echo -e "${YELLOW}► strings:${NC}"
    echo "  strings::length(s), strings::upper(s)"
    echo "  strings::lower(s), strings::reverse(s)"
    echo "  strings::trim(s), strings::replace(s, old, new)"
    echo ""
    
    echo -e "${YELLOW}► io:${NC}"
    echo "  io::print(x), io::input(prompt)"
    echo "  io::read_file(path), io::write_file(path, content)"
    echo ""
    
    echo -e "${YELLOW}► random:${NC}"
    echo "  random::int(min, max)  # Número aleatorio entero"
    echo "  random::float()        # Número aleatorio decimal"
    echo ""
    
    echo -e "${YELLOW}► time:${NC}"
    echo "  time::now()            # Timestamp actual"
    echo "  time::date()           # Fecha legible"
    echo ""
    
    echo -e "${YELLOW}► json:${NC}"
    echo "  json::parse(str)       # Parsear JSON"
    echo "  json::stringify(obj)   # Convertir a JSON"
    echo ""
}

run_demo_menu() {
    echo -e "${YELLOW}┌────────────────────────────────────────────────────────┐${NC}"
    echo -e "${YELLOW}│                    DEMOS GRÁFICOS                      │${NC}"
    echo -e "${YELLOW}└────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    echo -e "  ${GREEN}1)${NC} Demo Formas v0.2.0 (triángulos, elipses)"
    echo -e "  ${GREEN}2)${NC} Demo Shapes (círculos, rectángulos)"
    echo -e "  ${GREEN}3)${NC} Demo Visual (colores)"
    echo -e "  ${GREEN}4)${NC} Demo Random (aleatoriedad)"
    echo -e "  ${GREEN}5)${NC} Demo Time (tiempo)"
    echo -e "  ${GREEN}6)${NC} Demo JSON (parseo)"
    echo -e "  ${GREEN}0)${NC} Volver al menú principal"
    echo ""
    echo -ne "${ORANGE}Selecciona una demo: ${NC}"
    read demo_option
    
    case $demo_option in
        1)
            echo -e "${GREEN}[OK] Ejecutando Demo Formas v0.2.0...${NC}"
            cargo run --bin rydit-rs -- --gfx demos/demo_formas_v0.2.0.rydit
            ;;
        2)
            echo -e "${GREEN}[OK] Ejecutando Demo Shapes...${NC}"
            cargo run --bin rydit-rs -- --gfx demos/demo_shapes.rydit
            ;;
        3)
            echo -e "${GREEN}[OK] Ejecutando Demo Visual...${NC}"
            cargo run --bin rydit-rs -- --gfx demos/demo_visual.rydit
            ;;
        4)
            echo -e "${GREEN}[OK] Ejecutando Demo Random...${NC}"
            cargo run --bin rydit-rs -- --gfx demos/demo_random.rydit
            ;;
        5)
            echo -e "${GREEN}[OK] Ejecutando Demo Time...${NC}"
            cargo run --bin rydit-rs -- --gfx demos/demo_time.rydit
            ;;
        6)
            echo -e "${GREEN}[OK] Ejecutando Demo JSON...${NC}"
            cargo run --bin rydit-rs -- --gfx demos/demo_json.rydit
            ;;
        0)
            return
            ;;
        *)
            echo -e "${RED}[ERROR] Opción no válida${NC}"
            ;;
    esac
}

run_custom_script() {
    echo -e "${YELLOW}┌────────────────────────────────────────────────────────┐${NC}"
    echo -e "${YELLOW}│              EJECUTAR SCRIPT PERSONALIZADO             │${NC}"
    echo -e "${YELLOW}└────────────────────────────────────────────────────────┘${NC}"
    echo ""
    echo -ne "Ingresa la ruta del script .rydit: "
    read script_path
    
    if [ -f "$script_path" ]; then
        echo -e "${GREEN}[OK] Ejecutando $script_path...${NC}"
        cargo run --bin rydit-rs -- --gfx "$script_path"
    else
        echo -e "${RED}[ERROR] El archivo '$script_path' no existe${NC}"
    fi
}

show_documentation() {
    echo -e "${YELLOW}┌────────────────────────────────────────────────────────┐${NC}"
    echo -e "${YELLOW}│                    DOCUMENTACIÓN                       │${NC}"
    echo -e "${YELLOW}└────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    echo "Archivos de documentación disponibles:"
    echo ""
    echo -e "  ${GREEN}1)${NC} README.md - Documentación principal"
    echo -e "  ${GREEN}2)${NC} GUIA_USUARIO_v0.1.8.md - Guía completa"
    echo -e "  ${GREEN}3)${NC} ROADMAP.md - Planificación"
    echo -e "  ${GREEN}4)${NC} CHANGELOG_v0.1.8.md - Cambios"
    echo -e "  ${GREEN}0)${NC} Volver"
    echo ""
    echo -ne "Selecciona: "
    read doc_option
    
    case $doc_option in
        1)
            if [ -f "README.md" ]; then
                head -100 README.md
            fi
            ;;
        2)
            if [ -f "GUIA_USUARIO_v0.1.8.md" ]; then
                head -100 GUIA_USUARIO_v0.1.8.md
            fi
            ;;
        3)
            if [ -f "ROADMAP.md" ]; then
                head -50 ROADMAP.md
            fi
            ;;
        4)
            if [ -f "CHANGELOG_v0.1.8.md" ]; then
                head -50 CHANGELOG_v0.1.8.md
            fi
            ;;
        0)
            return
            ;;
    esac
    
    echo ""
    echo -ne "${YELLOW}Presiona Enter para continuar...${NC}"
    read
}

# =============================================================================
# PROGRAMA PRINCIPAL
# =============================================================================

main() {
    clear
    print_header
    
    while true; do
        print_menu
        echo -ne "${ORANGE}Selecciona una opción: ${NC}"
        read option
        
        case $option in
            1)
                run_demo_menu
                ;;
            2)
                echo -e "${GREEN}[OK] Ejecutando Snake Game...${NC}"
                echo "Controles: Flechas (mover), P (pausa), SPACE (restart), ESC (salir)"
                cargo run --bin rydit-rs -- --gfx snake.rydit
                ;;
            3)
                print_syntax
                ;;
            4)
                print_draw_commands
                ;;
            5)
                print_modules
                ;;
            6)
                run_custom_script
                ;;
            7)
                show_documentation
                ;;
            0)
                echo -e "${CYAN}¡Gracias por usar Ry-Dit! 🛡️${NC}"
                exit 0
                ;;
            *)
                echo -e "${RED}[ERROR] Opción no válida${NC}"
                ;;
        esac
        
        echo ""
        echo -ne "${YELLOW}Presiona Enter para continuar...${NC}"
        read
        clear
        print_header
    done
}

# Ejecutar programa principal
main
