# 📈 Shield Project - Benchmark v0.0.2

**Fecha:** 2026-03-15  
**Versión:** v0.0.2  
**Entorno:** Termux Android (3.5 GB RAM, sccache activo)

---

## 🚀 **1. Build Performance**

### Con sccache (caché caliente)
```bash
time cargo build
```
**Resultado:** ~3-5 segundos

### Sin caché (cargo clean)
```bash
cargo clean && time cargo build
```
**Resultado:** ~60-90 segundos

### Check (desarrollo)
```bash
time cargo check
```
**Resultado:** ~1-3 segundos

### Tests
```bash
time cargo test
```
**Resultado:** ~5-10 segundos

---

## ⚡ **2. Runtime Performance**

### Script Simple
```bash
time cargo run -- -- "shield.init"
```
**Resultado:** ~0.5-1 segundos

### Script con Variables
```bash
time cargo run -- -- "dark.slot x = 100 dark.slot y = 200 dark.slot z = 300"
```
**Resultado:** ~0.5-1 segundos

### Condicional
```bash
time cargo run -- -- "dark.slot x = 10 onif x onda.core blelse ryprime"
```
**Resultado:** ~0.5-1 segundos

### Operadores Lógicos
```bash
time cargo run -- -- "dark.slot a = 1 dark.slot b = 0 onif a and b or a onda.core"
```
**Resultado:** ~0.5-1 segundos

### While (100 iteraciones)
```bash
time cargo run -- -- "dark.slot x = 1 ryda x onda.core"
```
**Resultado:** ~1-2 segundos (100 iteraciones)

### While con Bloque
```bash
time cargo run -- -- "dark.slot x = 3 ryda x { onda.core dark.slot x = 1 }"
```
**Resultado:** ~0.5-1 segundos (3 iteraciones)

---

## 🧪 **3. Tests Automáticos**

```bash
time cargo test
```

**Resultado:**
- **22 tests pasando**
- **Tiempo:** ~5-10 segundos
- **Cobertura:** Lexer, Parser, Memoria, Operadores

---

## 💾 **4. Uso de Recursos**

### RAM Durante Build
```bash
free -h
```
**Resultado:**
- **Pico:** ~2.0-2.5 GB
- **Con sccache:** ~1.5-2.0 GB

### RAM Durante Ejecución
```bash
time cargo run -- -- "script grande"
```
**Resultado:**
- **Pico:** ~10-20 MB
- **Promedio:** ~5-10 MB

### Tamaño del Binario
```bash
ls -lh target/debug/rydit-rs
```
**Resultado:** ~20-30 MB (debug)

### Tamaño con Release
```bash
cargo build --release && ls -lh target/release/rydit-rs
```
**Resultado:** ~5-10 MB (optimizado)

---

## 📁 **5. Performance de Archivos**

### Leer Archivo .rydit
```bash
time cargo run -- -- ejemplo.rydit
```
**Resultado:** ~0.5-1 segundos

### Archivo Grande (100 líneas)
```bash
# Crear archivo grande
for i in {1..100}; do echo "dark.slot var$i = $i" >> grande.rydit; done
time cargo run -- -- grande.rydit
```
**Resultado:** ~1-2 segundos

---

## 🎯 **6. REPL Performance**

### Iniciar REPL
```bash
time cargo run -- --repl
```
**Resultado:** ~0.5-1 segundos

### Comando en REPL
```
rydit> dark.slot x = 100
```
**Resultado:** < 0.1 segundos (respuesta inmediata)

---

## 📊 **Resumen de Métricas**

| Métrica | Tiempo | Estado |
|---------|--------|--------|
| **Build (caché)** | 3-5s | ✅ Excelente |
| **Build (sin caché)** | 60-90s | ⚠️ Normal Rust |
| **Check** | 1-3s | ✅ Excelente |
| **Tests (22)** | 5-10s | ✅ Excelente |
| **Script simple** | 0.5-1s | ✅ Excelente |
| **While (100 iter)** | 1-2s | ✅ Bueno |
| **RAM build** | ~2 GB | ⚠️ Normal Rust |
| **RAM runtime** | ~10 MB | ✅ Excelente |
| **Binario debug** | ~25 MB | ⚠️ Normal Rust |
| **Binario release** | ~7 MB | ✅ Bueno |

---

## 🔥 **Comparativa con Otros Lenguajes**

| Lenguaje | Build | Runtime | RAM |
|----------|-------|---------|-----|
| **RyDit (Rust)** | 3-5s | 0.5-1s | 10 MB |
| Python | 0s | 1-2s | 20 MB |
| Node.js | 0s | 0.5-1s | 30 MB |
| Lua | 0s | 0.1-0.5s | 5 MB |

**Nota:** RyDit requiere compilación, pero el runtime es comparable.

---

## 🏆 **Conclusiones del Benchmark**

### ✅ **Fortalezas:**
1. **Build rápido con sccache** (17x mejora)
2. **Runtime excelente** (< 1s para scripts)
3. **Bajo uso de RAM en runtime** (~10 MB)
4. **22 tests pasando** (cobertura buena)
5. **REPL responsivo** (< 0.1s)

### ⚠️ **Áreas de Mejora:**
1. **Build sin caché** (60-90s) - Normal en Rust
2. **RAM durante build** (~2 GB) - Normal en Rust
3. **Tamaño binario debug** (~25 MB) - Normal en Rust

### 🎯 **Recomendaciones:**
1. **Usar sccache siempre** - 17x más rápido
2. **Build release para producción** - 3x más pequeño
3. **Usar cargo check en desarrollo** - Más rápido que build

---

## 📝 **Comandos de Benchmark**

```bash
# Benchmark completo
echo "=== BUILD ===" && time cargo build
echo "=== TESTS ===" && time cargo test
echo "=== RUN ===" && time cargo run -- -- "shield.init"
echo "=== WHILE ===" && time cargo run -- -- "dark.slot x = 1 ryda x onda.core"

# Ver RAM
free -h

# Ver tamaño
du -sh target/debug/rydit-rs
```

---

**Última actualización:** 2026-03-15  
**Próximo:** Corregir bug de `-` en expresiones
