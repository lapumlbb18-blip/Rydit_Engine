# 🧪 Tests - Shield Project

**Contenido:** Todos los archivos de test `.rydit` del proyecto.

## Archivos Actuales

| Archivo | Descripción |
|---------|-------------|
| `test_simple.rydit` | Test básico del lenguaje |
| `test_aritmetica.rydit` | Test de operaciones aritméticas |
| `test_funciones.rydit` | Test de funciones básicas |
| `test_funciones_retorno.rydit` | Test de funciones con retorno |
| `test_funciones_expresiones.rydit` | Test de funciones en expresiones |
| `test_math.rydit` | Test del módulo math |
| `test_modulos.rydit` | Test del sistema de módulos |
| `test_modulos_simple.rydit` | Test simple de módulos |

## Uso

```bash
# Ejecutar todos los tests
cargo test

# Ejecutar un test específico
cargo run --bin rydit-rs tests/test_simple.rydit
```

## Para v0.1.2 (Librería Estándar)

Se agregarán:
- `stdlib/strings.rydit` - Tests de librería de strings
- `stdlib/io.rydit` - Tests de entrada/salida
- `stdlib/math.rydit` - Tests extendidos de math

---

**Nota:** Mantén los tests activos aquí. Los tests antiguos/archivados van a `archive/tests-old/`.
