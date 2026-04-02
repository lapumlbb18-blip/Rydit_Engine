#!/bin/bash
# 🛡️ Script de Implementación - Parser v0.11.2
# Uso: ./scripts/implementar_parser_v0.11.2.sh [fase]

set -e  # Salir en error

FASE=${1:-"0"}  # Fase por defecto: 0

echo "🛡️ RyDit Parser v0.11.2 - Implementación"
echo "========================================="
echo "Fase: $FASE"
echo ""

case $FASE in
    "0")
        echo "📦 Fase 0: Preparación..."
        echo ""
        
        # Verificar backup
        if [ ! -f "backup_crates_v0.11.2_*.tar.gz" ]; then
            echo "⚠️  Backup no encontrado, creando..."
            tar -czf backup_crates_v0.11.2_$(date +%Y%m%d_%H%M%S).tar.gz crates/
            echo "✅ Backup creado"
        else
            echo "✅ Backup ya existe"
        fi
        
        # Crear tag de reversión
        if ! git tag -l | grep -q "v0.11.2-pre-parser"; then
            echo "📌 Creando tag de reversión..."
            git tag v0.11.2-pre-parser
            echo "✅ Tag creado: v0.11.2-pre-parser"
        else
            echo "✅ Tag de reversión ya existe"
        fi
        
        # Crear branch feature
        if ! git branch | grep -q "feature/parser-v0.11.2"; then
            echo "🌿 Creando branch feature..."
            git checkout -b feature/parser-v0.11.2
            echo "✅ Branch creado: feature/parser-v0.11.2"
        else
            echo "✅ Branch feature ya existe"
            git checkout feature/parser-v0.11.2
        fi
        
        # Ejecutar tests baseline
        echo "🧪 Ejecutando tests baseline..."
        cargo test --workspace --quiet 2>&1 | tail -5
        echo "✅ Tests baseline completados"
        
        echo ""
        echo "✅ Fase 0 completada!"
        echo "   Próximo paso: ./scripts/implementar_parser_v0.11.2.sh 1"
        ;;
        
    "1")
        echo "📦 Fase 1: rydit-lexer (zero-copy)..."
        echo ""
        
        # Crear estructura de directorios
        echo "📁 Creando crates/rydit-lexer/..."
        mkdir -p crates/rydit-lexer/src
        mkdir -p crates/rydit-lexer/tests
        mkdir -p crates/rydit-lexer/benches
        
        # Crear Cargo.toml
        cat > crates/rydit-lexer/Cargo.toml << 'EOF'
[package]
name = "rydit-lexer"
version = "0.11.2"
edition = "2021"
description = "Zero-copy lexer para RyDit"
authors = ["RyDit Team"]
license = "MIT"

[dependencies]

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "bench_lexer"
harness = false
EOF
        
        echo "✅ Crate rydit-lexer creado"
        echo ""
        echo "⚠️  Ahora implementar manualmente:"
        echo "   1. crates/rydit-lexer/src/token.rs (Token<'a>)"
        echo "   2. crates/rydit-lexer/src/lexer.rs (Lizer<'a>)"
        echo "   3. crates/rydit-lexer/src/lib.rs (re-exports)"
        echo "   4. crates/rydit-lexer/tests/lexer_test.rs (50 tests)"
        echo ""
        echo "📖 Ver: docs/ANALISIS_ARQUITECTURA_V0.11.2.md - Fase 1"
        echo ""
        echo "✅ Fase 1 iniciada!"
        echo "   Próximo paso: Implementar código, luego ./scripts/implementar_parser_v0.11.2.sh 2"
        ;;
        
    "2")
        echo "📦 Fase 2: rydit-parser (error recovery)..."
        echo ""
        
        # Crear estructura
        mkdir -p crates/rydit-parser/src/ast
        mkdir -p crates/rydit-parser/tests
        mkdir -p crates/rydit-parser/benches
        
        # Crear Cargo.toml
        cat > crates/rydit-parser/Cargo.toml << 'EOF'
[package]
name = "rydit-parser"
version = "0.11.2"
edition = "2021"
description = "Parser con error recovery para RyDit"
authors = ["RyDit Team"]
license = "MIT"

[dependencies]
rydit-lexer = { path = "../rydit-lexer" }

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "bench_parser"
harness = false
EOF
        
        echo "✅ Crate rydit-parser creado"
        echo ""
        echo "⚠️  Ahora implementar manualmente:"
        echo "   1. crates/rydit-parser/src/ast/ (Expr, Stmt typed)"
        echo "   2. crates/rydit-parser/src/parser.rs (error recovery)"
        echo "   3. crates/rydit-parser/src/validation.rs (semantic validation)"
        echo "   4. crates/rydit-parser/tests/parser_test.rs (100 tests)"
        echo ""
        echo "📖 Ver: docs/ANALISIS_ARQUITECTURA_V0.11.2.md - Fase 2"
        echo ""
        echo "✅ Fase 2 iniciada!"
        ;;
        
    "3")
        echo "📦 Fase 3: rydit-vm (bytecode)..."
        echo ""
        
        mkdir -p crates/rydit-vm/src
        mkdir -p crates/rydit-vm/tests
        mkdir -p crates/rydit-vm/benches
        
        cat > crates/rydit-vm/Cargo.toml << 'EOF'
[package]
name = "rydit-vm"
version = "0.11.2"
edition = "2021"
description = "Bytecode VM para RyDit"
authors = ["RyDit Team"]
license = "MIT"

[dependencies]
rydit-parser = { path = "../rydit-parser" }

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "bench_vm"
harness = false
EOF
        
        echo "✅ Crate rydit-vm creado"
        echo ""
        echo "⚠️  Ahora implementar manualmente:"
        echo "   1. crates/rydit-vm/src/bytecode.rs (OpCode)"
        echo "   2. crates/rydit-vm/src/compiler.rs (AST → Bytecode)"
        echo "   3. crates/rydit-vm/src/vm.rs (stack-based VM)"
        echo "   4. crates/rydit-vm/tests/vm_test.rs (100 tests)"
        echo ""
        echo "📖 Ver: docs/ANALISIS_ARQUITECTURA_V0.11.2.md - Fase 3"
        echo ""
        echo "✅ Fase 3 iniciada!"
        ;;
        
    "4")
        echo "📦 Fase 4: Integración..."
        echo ""
        
        echo "⚠️  Pasos manuales:"
        echo "   1. Actualizar Cargo.toml del workspace"
        echo "   2. Agregar rydit-lexer, rydit-parser, rydit-vm"
        echo "   3. Actualizar rydit-rs/Cargo.toml"
        echo "   4. Actualizar blast-core/Cargo.toml"
        echo "   5. Ejecutar: cargo test --workspace"
        echo ""
        echo "📖 Ver: docs/ANALISIS_ARQUITECTURA_V0.11.2.md - Fase 4"
        echo ""
        echo "✅ Fase 4 iniciada!"
        ;;
        
    "5")
        echo "📦 Fase 5: Optimización y documentación..."
        echo ""
        
        mkdir -p docs
        mkdir -p benches
        
        echo "⚠️  Pasos manuales:"
        echo "   1. Crear docs/MIGRACION_V0.11.2.md"
        echo "   2. Crear docs/BENCHMARKS_V0.11.2.md"
        echo "   3. Agregar benchmarks en crates/*/benches/"
        echo "   4. Ejecutar: cargo bench --workspace"
        echo ""
        echo "✅ Fase 5 iniciada!"
        ;;
        
    *)
        echo "❌ Fase inválida. Usar 0-5"
        echo ""
        echo "Fases disponibles:"
        echo "  0 - Preparación (backup, tag, branch)"
        echo "  1 - rydit-lexer (zero-copy)"
        echo "  2 - rydit-parser (error recovery)"
        echo "  3 - rydit-vm (bytecode)"
        echo "  4 - Integración"
        echo "  5 - Optimización + docs"
        ;;
esac

echo ""
echo "📖 Documentación completa: docs/ANALISIS_ARQUITECTURA_V0.11.2.md"
