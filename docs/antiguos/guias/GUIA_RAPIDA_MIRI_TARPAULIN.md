# 🛡️ GUÍA RÁPIDA: MIRI + TARPAULIN PARA RYDIT

**Fecha**: 2026-04-02  
**Versión**: v0.11.4-fase1b-name  
**Errores**: 69 en rydit-rs (binario legacy)  
**Crates seguros**: 4/4 (rydit-lexer, parser, vm, stream) ✅

---

## 🚀 COMANDOS RÁPIDOS

### 1. Testing Básico (Baseline)

```bash
# Todos los crates seguros
./scripts/test_miri_tarpaulin.sh baseline all

# Crate específico
./scripts/test_miri_tarpaulin.sh baseline rydit-lexer
```

### 2. MIRI (Detecta Undefined Behavior)

```bash
# Instalar MIRI (solo una vez)
rustup component add miri

# Test con MIRI (todos los crates seguros)
./scripts/test_miri_tarpaulin.sh miri all

# Crate específico
./scripts/test_miri_tarpaulin.sh miri rydit-parser
```

### 3. TARPAULIN (Code Coverage)

```bash
# Instalar TARPAULIN (solo una vez)
cargo install cargo-tarpaulin

# Coverage HTML (todos los crates)
./scripts/test_miri_tarpaulin.sh tarpaulin all

# Crate específico
./scripts/test_miri_tarpaulin.sh tarpaulin rydit-vm

# Ver reporte
firefox coverage/all/index.html  # Linux
start coverage\all\index.html   # Windows
open coverage/all/index.html    # macOS
```

### 4. Testing Completo

```bash
# Baseline + MIRI + TARPAULIN
./scripts/test_miri_tarpaulin.sh all
```

---

## 📊 ESTADO ACTUAL

### Crates Seguros (✅ MIRI Clean)

| Crate | Tests | Coverage Esperado | Estado |
|-------|-------|-------------------|--------|
| rydit-lexer | 21 | 95%+ | ✅ Listo |
| rydit-parser | 24 | 95%+ | ✅ Listo |
| rydit-vm | 19 | 90%+ | ✅ Listo |
| rydit-stream | 17 | 85%+ | ✅ Listo |

### Crate Peligroso (🔴 Requiere Fix)

| Crate | Errores | ¿UB Posible? | Estado |
|-------|---------|--------------|--------|
| rydit-rs (bin) | 69 | 🔴 SÍ | ❌ Roto |

---

## 🔍 INTERPRETAR RESULTADOS

### MIRI Output Normal

```
running 21 tests
test lexer::test::test_scan_number ... ok
test lexer::test::test_scan_string ... ok
...
test result: ok. 21 passed; 0 failed; 0 ignored

MIRI output: No undefined behavior detected
```

### MIRI Output con UB (PELIGRO)

```
error: Undefined Behavior: out-of-bounds memory access
  --> crates/rydit-rs/src/eval/mod.rs:54
   |
54 |     let x = ptr.read();  // ← UB detectado
   |             ^^^^^^^^^^

thread 'test_something' panicked at 'Miri raised an error', ...
```

**Acción**: `git reset --hard HEAD~1` (rollback inmediato)

---

### TARPAULIN Output Normal

```
Coverage Results:
|| Tested/Total Lines:
|| crates/rydit-lexer/src/lib.rs: 450/460 (97.8%)
|| crates/rydit-lexer/src/lexer.rs: 1200/1250 (96.0%)

Final Coverage: 95.5%
```

### TARPAULIN Output con Bajo Coverage (< 80%)

```
Final Coverage: 65.4%
ERROR: Coverage below threshold (80%)
```

**Acción**: Agregar tests para código no cubierto (ver archivo HTML)

---

## 🛠️ DEBUGGING CON MIRI

### Flags Avanzados de MIRI

```bash
# Detectar memory leaks
MIRIFLAGS="-Zmiri-check-number-validity" cargo miri test -p rydit-lexer

# Detectar borrow checker violations
MIRIFLAGS="-Zmiri-tag-raw-pointers" cargo miri test -p rydit-parser

# Detectar uninitialized memory
MIRIFLAGS="-Zmiri-check-uninit" cargo miri test -p rydit-vm

# Ver todo el output (verbose)
MIRIFLAGS="-Zmiri-verbose" cargo miri test -p rydit-stream
```

### MIRI + GDB (Debugging Avanzado)

```bash
# Ejecutar con debugger
MIRIFLAGS="-Zmiri-backtrace=full" cargo miri test -p rydit-rs -- --nocapture

# Si MIRI detecta UB, usa gdb para inspeccionar
gdb --args cargo miri test -p rydit-rs
```

---

## 📈 DEBUGGING CON TARPAULIN

### Ver Código No Testeado

1. Abrir `coverage/all/index.html` en navegador
2. Click en crate específico (ej: rydit-lexer)
3. Click en archivo específico (ej: lexer.rs)
4. Líneas en **rojo** = no testeadas
5. Líneas en **verde** = testeadas

### Agregar Tests para Código No Cubierto

```rust
// crates/rydit-lexer/tests/coverage_test.rs

#[test]
fn test_token_kind_display() {
    // Código no cubierto según TARPAULIN
    let token = TokenKind::Number;
    assert_eq!(format!("{}", token), "Number");
}
```

### Coverage Threshold en CI/CD

```bash
# Fail si coverage < 80%
cargo tarpaulin -p rydit-lexer --fail-under 80

# En GitHub Actions (.github/workflows/coverage.yml)
- name: Check Coverage
  run: cargo tarpaulin -p rydit-lexer --fail-under 80 --out Xml
```

---

## 🔄 ROLLBACK SEGURO

### Si MIRI Detecta UB

```bash
# Rollback inmediato
git reset --hard HEAD~1

# Verificar estado
git status
cargo test -p rydit-lexer -p rydit-parser -p rydit-vm -p rydit-stream

# Analizar causa
git log -n 1 --stat
```

### Si TARPAULIN Reporta < 80%

```bash
# No requiere rollback
# Solo agregar tests faltantes

# Ver reporte
firefox coverage/all/index.html

# Agregar tests y re-ejecutar
cargo tarpaulin -p rydit-lexer --out Html
```

---

## 📋 CHECKLIST PRE-FIX

### Antes de Fixear Errores

- [ ] Backup creado: `crates_backup_v0.11.2_*.tar.gz`
- [ ] Tag de rollback: `v0.11.4-fase1b-name`
- [ ] MIRI instalado: `rustup component add miri`
- [ ] TARPAULIN instalado: `cargo install cargo-tarpaulin`
- [ ] Tests baseline passing: `cargo test -p rydit-lexer ...`
- [ ] Branch creado: `git checkout -b fix/70-errores-miri`

### Después de Cada Fix

- [ ] Commit hecho: `git commit -m "Fix: ..."`
- [ ] MIRI test passing: `cargo miri test -p rydit-rs`
- [ ] Baseline tests passing: `cargo test -p rydit-lexer ...`
- [ ] Sin nuevos warnings: `cargo build 2>&1 | grep warning`

### Después de Todos los Fixes

- [ ] 0 errores de compilación
- [ ] 0 warnings
- [ ] MIRI clean (0 UB)
- [ ] TARPAULIN coverage > 80%
- [ ] Merge a main: `git merge fix/70-errores-miri`
- [ ] Tag nueva versión: `git tag v0.11.5-miri-clean`

---

## 🎯 EJEMPLO DE FIX SEGURO

### Paso 1: Identificar Error

```bash
cargo build -p rydit-rs --bin rydit-rs 2>&1 | grep "error\[E"
# Output: error[E0308]: mismatched types
```

### Paso 2: Analizar con MIRI

```bash
# Antes de fixear, test con MIRI
cargo miri test -p rydit-rs eval 2>&1 | grep -A 5 "Undefined Behavior"
# Output: (si hay UB, MIRI lo reporta)
```

### Paso 3: Fixear

```rust
// ANTES (roto)
Expr::Texto(s) => Valor::Texto(s.clone()),  // ← &str vs String

// DESPUÉS (fix)
Expr::Texto(s) => Valor::Texto(s.to_string()),  // ✅ String
```

### Paso 4: Commit + Test

```bash
git add crates/rydit-rs/src/eval/mod.rs
git commit -m "Fix: &str vs String en eval/mod.rs (E0308)"

# Test con MIRI
cargo miri test -p rydit-rs eval

# Verificar 0 UB
# Output: test result: ok. 0 passed; 0 failed; 0 UB
```

### Paso 5: Siguiente Error

```bash
# Repetir desde Paso 1
```

---

## 📞 SOPORTE RÁPIDO

### MIRI No Instala

```bash
# Verificar rustup
rustup --version

# Actualizar rustup
rustup update

# Reintentar instalación
rustup component add miri --toolchain nightly
```

### TARPAULIN Falló

```bash
# Verificar instalación
cargo tarpaulin --version

# Reinstalar
cargo install --force cargo-tarpaulin

# Verificar dependencias (Linux)
sudo apt-get install libssl-dev pkg-config libclang-dev
```

### Tests Fallback

```bash
# Limpiar build
cargo clean

# Rebuild
cargo build -p rydit-lexer -p rydit-parser -p rydit-vm -p rydit-stream

# Re-test
cargo test -p rydit-lexer -p rydit-parser -p rydit-vm -p rydit-stream --lib
```

---

## 📚 RECURSOS ADICIONALES

### Documentación Oficial

- **MIRI**: https://github.com/rust-lang/miri
- **TARPAULIN**: https://github.com/xd009642/tarpaulin
- **Rust Book**: https://doc.rust-lang.org/book/

### Informes Relacionados

- `INFORME_70_ERRORES_MIRI_TARPAULIN.md` - Informe completo
- `QWEN.md` - Bitácora técnica principal
- `ESTADO_COMPLETO_V0.11.0.md` - Estado del proyecto

---

<div align="center">

**🛡️ RyDit v0.11.4 - GUÍA MIRI/TARPAULIN**

*MIRI: Detecta UB | TARPAULIN: Coverage > 80% | Rollback: v0.11.4-fase1b-name*

**Próximo: Fix manual con verification**

</div>
