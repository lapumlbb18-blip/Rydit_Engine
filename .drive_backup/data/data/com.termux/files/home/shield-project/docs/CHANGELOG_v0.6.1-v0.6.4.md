# CHANGELOG v0.6.1 - v0.6.4

## v0.6.4 (2026-03-24) - cargo fmt + Evaluación Split

### Style
- ✅ **cargo fmt** en todo el proyecto
  - 12 archivos formateados
  - 3,588 inserciones, 1,350 eliminaciones (whitespace)
  - Código consistente en todos los crates

### Fix
- ✅ **bench_lizer.rs** - imports unused eliminados
  - Removido `Parser` import (no usado)
  - 2 errors de compilación fixeados

### Docs
- ✅ **SPLIT_EVALUACION.md** creado (348 líneas)
  - Plan completo para split de main.rs (v0.7.0)
  - 6 fases, ~5 horas estimadas
  - Red de seguridad: 137 tests

### Métricas
- Tests: 137 passing ✅
- Binario: ~600 KB
- Warnings: 0
- Líneas: ~13,500

---

## v0.6.3 (2026-03-24) - Módulo FILES

### Features
- ✅ **files::read(ruta)** - Leer archivo completo
- ✅ **files::write(ruta, texto)** - Escribir archivo (sobrescribe)
- ✅ **files::append(ruta, texto)** - Añadir al final
- ✅ **files::exists(ruta)** - Verificar existencia
- ✅ **files::delete(ruta)** - Eliminar archivo

### Files Created
- `crates/modules/files.rydit` - Módulo embebido
- `demos/demo_files.rydit` - Demo completo (~130 líneas)
  - Guardar partida (JSON)
  - Cargar configuración
  - Sistema de logs
  - Manejo de errores

### Tests
- ✅ 4 tests nuevos:
  - test_files_write_and_read
  - test_files_append
  - test_files_exists
  - test_files_delete

### Métricas
- Tests: 137 (+4)
- Binario: ~600 KB
- Líneas Rust: +200

---

## v0.6.2 (2026-03-24) - Módulo REGEX

### Features
- ✅ **regex::match(patron, texto)** - Validar patrones (bool)
- ✅ **regex::replace(patron, reemplazo, texto)** - Reemplazar ocurrencias
- ✅ **regex::split(patron, texto)** - Dividir por patrón (array)
- ✅ **regex::find_all(patron, texto)** - Encontrar todos los matches
- ✅ **regex::capture(patron, texto)** - Capturar grupos (array)

### Dependencies
- ✅ regex crate 1.10 (+30-40 KB binario)

### Files Created
- `crates/modules/regex.rydit` - Módulo embebido
- `demos/demo_regex.rydit` - Demo completo (~100 líneas)
  - Validación de emails
  - Reemplazo de vocales
  - Split por comas
  - Extracción de números
  - Captura de grupos (nombre:valor, fecha)
  - Validación de teléfonos
  - Extracción de hashtags
  - Validación de URLs
- `screenshots/particulas.jpg` - Captura demo partículas
- `screenshots/particulas.mp4` - Video demo partículas

### Tests
- ✅ 7 tests nuevos:
  - test_regex_match_valido
  - test_regex_match_invalido
  - test_regex_replace
  - test_regex_split
  - test_regex_find_all
  - test_regex_capture
  - test_regex_email_validation

### Docs
- ✅ README.md actualizado con galería de partículas

### Métricas
- Tests: 133 (+7)
- Binario: ~590-610 KB (+30-40 KB)
- Líneas Rust: +200

---

## v0.6.1 (2026-03-24) - Limpieza Repositorio

### Critical Cleanup
- ✅ **144 archivos eliminados** del repositorio
  - 32,895 líneas eliminadas
  - Archivos peligrosos: `.rcloneignore`, `backup_*.sh`
  - Carpetas internas: `historial/`, `ruta_v0.5.0_v0.9.9/`, `archive/`, `context/`

### Security
- ✅ **.gitignore actualizado** (+40 líneas)
  - Excluye backup scripts (cuenta Google Drive)
  - Excluye archivos internos (planes, resúmenes)
  - Excluye carpetas históricas

### Docs
- ✅ **MANIFIESTO.md extendido**
  - Rendimiento estable (sin calentamiento, RAM <100 MB)
  - Portabilidad cruzada (Linux, Windows, WebAssembly)
  - Métricas comparativas (RyDit vs Godot vs Unity)

- ✅ **README.md actualizado**
  - MANIFIESTO integrado después de "¿Qué es RyDit?"
  - Galería: partículas.jpg + video mp4
  - Eliminada imagen game over

### Files Modified
- `.gitignore` - +40 líneas
- `README.md` - +100 líneas
- `MANIFIESTO.md` - +40 líneas

### Métricas
- Tests: 126
- Repositorio: -3 MB (sin target/)
- Archivos públicos: 50 principales

---

## Resumen v0.6.1 - v0.6.4

| Versión | Fecha | Features | Tests | Binario |
|---------|-------|----------|-------|---------|
| v0.6.1 | 2026-03-24 | Limpieza + MANIFIESTO | 126 | ~580 KB |
| v0.6.2 | 2026-03-24 | REGEX (5 funciones) | 133 (+7) | ~600 KB |
| v0.6.3 | 2026-03-24 | FILES (5 funciones) | 137 (+4) | ~600 KB |
| v0.6.4 | 2026-03-24 | cargo fmt + Eval Split | 137 | ~600 KB |

### Total Added (v0.6.1 - v0.6.4)
- **Tests:** +11 (126 → 137)
- **Funciones:** +10 (regex: 5, files: 5)
- **Demos:** +2 (demo_regex, demo_files)
- **Líneas Rust:** +400
- **Documentación:** +500 líneas (MANIFIESTO, README, SPLIT_EVALUACION)

### Próximo: v0.7.0
- Split de main.rs (4,200 → 10 módulos)
- main.rs: ~80 líneas
- Bindings, eval, game_loop, repl separados
