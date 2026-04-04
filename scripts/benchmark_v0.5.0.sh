#!/bin/bash
# =============================================================================
# SCRIPT DE BENCHMARKING v0.5.0 - Ry-Dit
# =============================================================================
# 
# Uso: ./scripts/benchmark_v0.5.0.sh
#
# Este script ejecuta:
# 1. Tests de todos los crates
# 2. Benchmarks de lizer (lexer/parser)
# 3. Genera reporte de resultados
#
# =============================================================================

set -e  # Salir si hay error

echo "============================================================================="
echo "  Ry-Dit v0.5.0 - Benchmark Suite"
echo "============================================================================="
echo ""

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Directorio del proyecto
PROJECT_DIR="/data/data/com.termux/files/home/shield-project"
REPORT_DIR="$PROJECT_DIR/benchmarks"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Crear directorio de reportes
mkdir -p "$REPORT_DIR"

echo -e "${BLUE}📁 Directorio: $PROJECT_DIR${NC}"
echo ""

# =============================================================================
# 1. INFORMACIÓN DEL SISTEMA
# =============================================================================
echo "============================================================================="
echo "  1. Información del Sistema"
echo "============================================================================="
echo ""
echo "Fecha: $(date)"
echo "Rust version: $(rustc --version 2>/dev/null || echo 'No disponible')"
echo "Cargo version: $(cargo --version 2>/dev/null || echo 'No disponible')"
echo ""

# =============================================================================
# 2. COMPILACIÓN EN MODO RELEASE
# =============================================================================
echo "============================================================================="
echo "  2. Compilación en Modo Release"
echo "============================================================================="
echo ""
echo -e "${YELLOW}Compilando...${NC}"
cd "$PROJECT_DIR"
cargo build --release --bin rydit-rs 2>&1 | tail -5
echo ""
echo -e "${GREEN}✅ Compilación completada${NC}"
echo ""

# Tamaño del binario
if [ -f "$PROJECT_DIR/target/release/rydit-rs" ]; then
    BIN_SIZE=$(du -h "$PROJECT_DIR/target/release/rydit-rs" | cut -f1)
    echo "Tamaño del binario: ${BLUE}$BIN_SIZE${NC}"
    echo ""
fi

# =============================================================================
# 3. TEST SUITE COMPLETO
# =============================================================================
echo "============================================================================="
echo "  3. Test Suite Completo"
echo "============================================================================="
echo ""

# Contador de tests
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Test: lizer
echo -e "${BLUE}🧪 Tests: lizer (lexer/parser)${NC}"
LIZER_OUTPUT=$(cargo test --release -p lizer 2>&1)
LIZER_TESTS=$(echo "$LIZER_OUTPUT" | grep "test result:" | tail -1 | awk '{print $5}')
echo "   Resultado: $LIZER_TESTS"
echo ""

# Test: blast-core
echo -e "${BLUE}🧪 Tests: blast-core (executor)${NC}"
BLAST_OUTPUT=$(cargo test --release -p blast-core 2>&1)
BLAST_TESTS=$(echo "$BLAST_OUTPUT" | grep "test result:" | tail -1 | awk '{print $5}')
echo "   Resultado: $BLAST_TESTS"
echo ""

# Test: migui
echo -e "${BLUE}🧪 Tests: migui (GUI core)${NC}"
MIGUI_OUTPUT=$(cargo test --release -p migui 2>&1)
MIGUI_TESTS=$(echo "$MIGUI_OUTPUT" | grep "test result:" | tail -1 | awk '{print $5}')
echo "   Resultado: $MIGUI_TESTS"
echo ""

# Test: v-shield
echo -e "${BLUE}🧪 Tests: v-shield (wrapper raylib)${NC}"
VSHIELD_OUTPUT=$(cargo test --release -p v-shield 2>&1)
VSHIELD_TESTS=$(echo "$VSHIELD_OUTPUT" | grep "test result:" | tail -1 | awk '{print $5}')
echo "   Resultado: $VSHIELD_TESTS"
echo ""

# =============================================================================
# 4. BENCHMARKS (si está disponible nightly)
# =============================================================================
echo "============================================================================="
echo "  4. Benchmarks"
echo "============================================================================="
echo ""

# Verificar si tenemos nightly
RUST_VERSION=$(rustc --version 2>&1)
if echo "$RUST_VERSION" | grep -q "nightly"; then
    echo -e "${YELLOW}Ejecutando benchmarks de lizer...${NC}"
    echo ""
    
    # Ejecutar benchmarks
    if cargo bench -p lizer 2>&1 | tee "$REPORT_DIR/bench_lizer_$TIMESTAMP.txt"; then
        echo ""
        echo -e "${GREEN}✅ Benchmarks completados${NC}"
        echo "Reporte guardado en: $REPORT_DIR/bench_lizer_$TIMESTAMP.txt"
    else
        echo -e "${RED}⚠️ Benchmarks fallidos${NC}"
    fi
else
    echo -e "${YELLOW}⚠️ Benchmarks requieren Rust nightly${NC}"
    echo ""
    echo "Para ejecutar benchmarks:"
    echo "  1. Instalar nightly: rustup install nightly"
    echo "  2. Usar nightly: rustup default nightly"
    echo "  3. Ejecutar: cargo bench -p lizer"
    echo ""
fi

echo ""

# =============================================================================
# 5. RESUMEN DE MÉTRICAS
# =============================================================================
echo "============================================================================="
echo "  5. Resumen de Métricas"
echo "============================================================================="
echo ""

# Líneas de código
echo -e "${BLUE}📊 Líneas de Código Rust:${NC}"
if command -v wc &> /dev/null; then
    TOTAL_LINES=0
    for crate in lizer blast-core migui rydit-gfx v-shield rydit-rs; do
        if [ -f "$PROJECT_DIR/crates/$crate/src/lib.rs" ] || [ -f "$PROJECT_DIR/crates/$crate/src/main.rs" ]; then
            if [ -f "$PROJECT_DIR/crates/$crate/src/lib.rs" ]; then
                LINES=$(wc -l < "$PROJECT_DIR/crates/$crate/src/lib.rs")
            else
                LINES=$(wc -l < "$PROJECT_DIR/crates/$crate/src/main.rs")
            fi
            echo "   - $crate: $LINES líneas"
            TOTAL_LINES=$((TOTAL_LINES + LINES))
        fi
    done
    echo "   ${GREEN}Total: $TOTAL_LINES líneas${NC}"
fi
echo ""

# Número de demos
echo -e "${BLUE}📊 Demos Disponibles:${NC}"
DEMOS_COUNT=$(ls -1 "$PROJECT_DIR/demos/*.rydit" 2>/dev/null | wc -l)
echo "   ${GREEN}$DEMOS_COUNT demos en demos/${NC}"
echo ""

# =============================================================================
# 6. GENERAR REPORTE JSON
# =============================================================================
echo "============================================================================="
echo "  6. Generando Reporte"
echo "============================================================================="
echo ""

REPORT_FILE="$REPORT_DIR/reporte_v0.5.0_$TIMESTAMP.json"

cat > "$REPORT_FILE" << EOF
{
  "version": "v0.5.0",
  "fecha": "$(date -Iseconds)",
  "rust_version": "$(rustc --version 2>/dev/null || echo 'N/A')",
  "tests": {
    "lizer": "$LIZER_TESTS",
    "blast-core": "$BLAST_TESTS",
    "migui": "$MIGUI_TESTS",
    "v-shield": "$VSHIELD_TESTS"
  },
  "binario": {
    "tamaño": "$BIN_SIZE",
    "ruta": "target/release/rydit-rs"
  },
  "demos": $DEMOS_COUNT
}
EOF

echo -e "${GREEN}✅ Reporte guardado en: $REPORT_FILE${NC}"
echo ""

# =============================================================================
# 7. RESUMEN FINAL
# =============================================================================
echo "============================================================================="
echo "  📋 Resumen Final"
echo "============================================================================="
echo ""
echo "✅ Tests completados"
echo "✅ Benchmarks: $(if echo "$RUST_VERSION" | grep -q 'nightly'; then echo 'Ejecutados'; else echo 'Pendientes (requiere nightly)'; fi)"
echo "✅ Reporte generado: $REPORT_FILE"
echo ""
echo -e "${GREEN}¡Benchmark Suite v0.5.0 Completado!${NC}"
echo ""
echo "============================================================================="
