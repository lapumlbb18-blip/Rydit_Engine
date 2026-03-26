#!/bin/bash
# =============================================================================
# RyDit Engine - Ejecutor de Tests
# =============================================================================
# Uso: ./run_tests.sh [opciones]
# =============================================================================

set -e

# Colores
ROJO='\033[0;31m'
VERDE='\033[0;32m'
AMARILLO='\033[1;33m'
AZUL='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Contadores
TESTS_PASSED=0
TESTS_FAILED=0
TESTS_TOTAL=0

# Funciones de logging
log_header() {
    echo ""
    echo -e "${CYAN}============================================================${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${CYAN}============================================================${NC}"
    echo ""
}

log_info() {
    echo -e "${AZUL}[INFO]${NC} $1"
}

log_success() {
    echo -e "${VERDE}[✓]${NC} $1"
}

log_warning() {
    echo -e "${AMARILLO}[⚠]${NC} $1"
}

log_error() {
    echo -e "${ROJO}[✗]${NC} $1"
}

# Header
log_header "🧪 RyDit Engine - Suite de Tests"

# Parsear argumentos
RUN_RUST_TESTS=true
RUN_VISUAL_TESTS=false
RUN_BENCHMARKS=false
VERBOSE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --visual)
            RUN_VISUAL_TESTS=true
            shift
            ;;
        --bench)
            RUN_BENCHMARKS=true
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        --help|-h)
            echo "Uso: $0 [opciones]"
            echo ""
            echo "Opciones:"
            echo "  --visual     Ejecutar tests visuales (requiere Termux-X11)"
            echo "  --bench      Ejecutar benchmarks"
            echo "  -v, --verbose  Mostrar output detallado"
            echo "  --help, -h   Mostrar esta ayuda"
            exit 0
            ;;
        *)
            shift
            ;;
    esac
done

# =============================================================================
# TESTS DE RUST (CRATES)
# =============================================================================

log_header "1️⃣  Tests de Rust (Crates)"

log_info "Ejecutando tests del workspace..."
echo ""

# Tests del workspace (excluyendo rydit-gfx que requiere X11)
if cargo test --workspace --exclude rydit-gfx --exclude rydit-rs 2>&1 | tee /tmp/rust_tests.log; then
    log_success "Tests de crates completados"
    
    # Extraer resultados
    PASSED=$(grep -o "[0-9]* passed" /tmp/rust_tests.log | tail -1 | grep -o "[0-9]*" || echo "0")
    FAILED=$(grep -o "[0-9]* failed" /tmp/rust_tests.log | tail -1 | grep -o "[0-9]*" || echo "0")
    
    echo ""
    log_info "Resultados:"
    echo -e "  ${VERDE}✓ Pasaron: $PASSED${NC}"
    
    if [ "$FAILED" != "0" ]; then
        echo -e "  ${ROJO}✗ Fallaron: $FAILED${NC}"
        TESTS_FAILED=$FAILED
    fi
else
    log_error "Algunos tests fallaron"
    TESTS_FAILED=1
fi

# =============================================================================
# TESTS DEL BINARIO
# =============================================================================

log_header "2️⃣  Tests del Binario (rydit-rs)"

log_info "Ejecutando tests del binario..."
echo ""

if cargo test -p rydit-rs 2>&1 | tee /tmp/bin_tests.log; then
    log_success "Tests del binario completados"
    
    PASSED=$(grep -o "[0-9]* passed" /tmp/bin_tests.log | tail -1 | grep -o "[0-9]*" || echo "0")
    echo ""
    log_info "Resultados:"
    echo -e "  ${VERDE}✓ Pasaron: $PASSED${NC}"
else
    log_error "Tests del binario fallaron"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# =============================================================================
# TESTS VISUALES (OPCIONAL)
# =============================================================================

if [ "$RUN_VISUAL_TESTS" = true ]; then
    log_header "3️⃣  Tests Visuales (Termux-X11)"
    
    # Verificar si estamos en Termux con X11
    if [ -f "/data/data/com.termux/files/usr/bin/bash" ]; then
        if [ -n "$DISPLAY" ]; then
            log_success "Entorno gráfico detectado: DISPLAY=$DISPLAY"
            
            # Verificar demos
            if [ -d "demos" ] && [ "$(ls -A demos/*.rydit 2>/dev/null)" ]; then
                log_info "Demos disponibles para test visual:"
                ls -1 demos/*.rydit | head -5 | while read demo; do
                    echo "  • $(basename $demo)"
                done
                
                echo ""
                log_info "¿Quieres ejecutar un demo visual?"
                read -p "Ejecutar demo visual? (s/n): " -n 1 -r
                echo
                
                if [[ $REPLY =~ ^[SsY]$ ]]; then
                    log_info "Ejecutando demo visual..."
                    ./scripts/run_demo.sh demo_ilusiones_opticas
                fi
            else
                log_warning "No hay demos disponibles"
            fi
        else
            log_warning "DISPLAY no configurado. ¿Quieres configurar Termux-X11?"
            read -p "Configurar Termux-X11? (s/n): " -n 1 -r
            echo
            
            if [[ $REPLY =~ ^[SsY]$ ]]; then
                export DISPLAY=:0
                export MESA_LOADER_DRIVER_OVERRIDE=zink
                export DRI3=1
                log_success "Variables de entorno configuradas"
                echo ""
                echo "Ahora ejecuta: termux-x11-nightly"
                echo "Luego vuelve a ejecutar: $0 --visual"
            fi
        fi
    else
        log_info "Tests visuales solo disponibles en Termux/Linux con X11"
    fi
fi

# =============================================================================
# BENCHMARKS (OPCIONAL)
# =============================================================================

if [ "$RUN_BENCHMARKS" = true ]; then
    log_header "4️⃣  Benchmarks"
    
    log_info "Ejecutando benchmarks..."
    echo ""
    
    if command -v cargo &> /dev/null; then
        # Verificar si hay benchmarks
        if [ -f "crates/lizer/benches/bench_lizer.rs" ]; then
            cargo bench -p lizer 2>&1 | tee /tmp/bench.log
            log_success "Benchmarks completados"
        else
            log_warning "No hay benchmarks disponibles"
        fi
    fi
fi

# =============================================================================
# RESUMEN FINAL
# =============================================================================

log_header "📊 Resumen Final"

# Contar tests totales
TOTAL_RUST=$(grep -o "[0-9]* passed" /tmp/rust_tests.log 2>/dev/null | tail -1 | grep -o "[0-9]*" || echo "0")
TOTAL_BIN=$(grep -o "[0-9]* passed" /tmp/bin_tests.log 2>/dev/null | tail -1 | grep -o "[0-9]*" || echo "0")
TOTAL_TESTS=$((TOTAL_RUST + TOTAL_BIN))

echo ""
echo -e "${CYAN}Tests de Rust (crates):${NC} $TOTAL_RUST passing"
echo -e "${CYAN}Tests de Binario:${NC} $TOTAL_BIN passing"
echo ""

if [ $TOTAL_TESTS -gt 0 ]; then
    echo -e "${VERDE}✓ Total: $TOTAL_TESTS tests passing${NC}"
else
    echo -e "${AMARILLO}⚠️  No se ejecutaron tests${NC}"
fi

echo ""

# Verificar si hay tests fallidos
if [ "$TESTS_FAILED" -gt 0 ]; then
    echo -e "${ROJO}✗ $TESTS_FAILED tests fallaron${NC}"
    echo ""
    echo "Revisa los logs:"
    echo "  • /tmp/rust_tests.log"
    echo "  • /tmp/bin_tests.log"
    exit 1
else
    echo -e "${VERDE}✅ ¡Todos los tests pasaron!${NC}"
    echo ""
fi

# Limpieza
rm -f /tmp/rust_tests.log /tmp/bin_tests.log /tmp/bench.log 2>/dev/null

exit 0
