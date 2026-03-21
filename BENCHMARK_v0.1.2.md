# 📊 Benchmark - Shield Project v0.1.2

**Fecha:** 2026-03-17
**Versión:** v0.1.2 (Librería Estándar)
**Plataforma:** Android/Termux

---

## ⚡ Build Times

| Perfil | Tiempo | Estado |
|--------|--------|--------|
| Release (con caché) | ~0.7s | ✅ |
| Dev (con caché) | ~1.9s | ✅ |
| Dev (clean) | ~30-40s | ⚡ |

---

## 🧪 Tests

| Tipo | Tests | Tiempo | Estado |
|------|-------|--------|--------|
| Rust (lizer) | 18 | <0.01s | ✅ |
| Rust (rydit-rs) | 36 | <0.01s | ✅ |
| Rust (rydit-gfx) | 3 | <0.01s | ✅ |
| Rust (blast-core) | 2 | <0.01s | ✅ |
| Rust (v-shield) | 0 | <0.01s | ✅ |
| Doc tests | 2 | ~0.29s | ✅ |
| **TOTAL** | **61** | **~2s** | ✅ |

---

## 🚀 Ejecución de Scripts

| Script | Tiempo | Estado |
|--------|--------|--------|
| `test_builtin_directo2.rydit` | ~0.21s | ✅ |
| `test_minimal.rydit` (import strings) | ~0.18s | ✅ |

---

## 📈 Métricas de Código

| Métrica | Valor |
|---------|-------|
| Líneas Rust | 4,611 |
| Líneas RyDit | 803 |
| **Total** | **5,414** |
| Crates | 6 |
| Módulos RyDit | 4 |

---

## 📦 Crates

```
blast-core   (audio/executor)
lizer        (lexer/parser)
rydit-core   (tipos/valores)
rydit-gfx    (gráficos/raylib)
rydit-rs     (binario principal)
v-shield     (video)
```

---

## 📚 Módulos RyDit

```
math.rydit    (8 funciones: sumar, restar, multiplicar, dividir, pow, abs, min, max)
arrays.rydit  (4 funciones: length, get, contains, index_of)
strings.rydit (7 funciones: length, upper, lower, concat, trim, substr, replace) ← v0.1.2
io.rydit      (6 funciones: read_file, write_file, file_exists, append_file, print, println) ← v0.1.2
```

---

## 🎯 Comparativa de Versiones

| Versión | Tests | Build | Líneas | Features |
|---------|-------|-------|--------|----------|
| v0.0.1 | 4 | ~5s | 500 | CLI |
| v0.0.10 | 59 | ~8s | 3000 | Parser fix |
| v0.1.0 | 60 | ~10s | 4500 | Snake game |
| v0.1.1 | 61 | ~1.2s | 5000 | Módulos |
| **v0.1.2** | **61** | **~0.7s** | **5414** | **Librería estándar** |

---

## 🔥 Optimizaciones v0.1.2

- ✅ sccache habilitado
- ✅ codegen-units=1 (menos RAM)
- ✅ nobuild en raylib (usa raylib nativo)
- ✅ Build incremental

---

**"Construido con ❤️ en Android/Termux"**
