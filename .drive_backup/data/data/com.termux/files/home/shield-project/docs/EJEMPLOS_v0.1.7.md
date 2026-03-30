# 🎮 Ejemplos y Demos - RyDit v0.1.7

Este directorio contiene ejemplos y demos del lenguaje RyDit.

---

## 🐍 Snake Game

**Archivo:** `snake.rydit`  
**Versión:** v0.1.7  
**Estado:** ✅ Completo y jugable

### Características

- 🎮 Control de serpiente con flechas direccionales
- 🍎 Comida aleatoria con módulo `random`
- 🐍 Cuerpo de serpiente que crece
- 🏆 Sistema de score y high score
- ⏸️ Pausa con tecla 'P'
- 🔄 Restart con SPACE
- 📈 Velocidad progresiva (aumenta cada 50 puntos)

### Ejecutar

```bash
# Modo gráfico (requiere display)
cargo run --bin rydit-rs -- --gfx snake.rydit

# O con el launcher
./snake_launcher.sh
```

### Controles

| Tecla | Acción |
|-------|--------|
| ↑ | Mover arriba |
| → | Mover derecha |
| ↓ | Mover abajo |
| ← | Mover izquierda |
| P | Pausa |
| SPACE | Reiniciar |
| ESC | Salir |

### Mecánicas

1. **Objetivo:** Comer la comida roja sin chocar
2. **Score:** +10 puntos por comida
3. **Velocidad:** Aumenta cada 50 puntos
4. **Game Over:** Chocar con paredes o propio cuerpo
5. **High Score:** Se mantiene durante la sesión

---

## 📚 Otros Ejemplos

### Test de Imports

**Archivo:** `tests/test_imports_fixes.rydit`

Demuestra el sistema de módulos con cache y aliases:

```rydit
import math
import math  # Usa cache, no re-ejecuta

import arrays as arr
import arrays as arr2  # Ambos aliases funcionan

voz math::sumar(5, 3)
voz arr::length([1, 2, 3])
```

### Test Aritmética

**Archivo:** `tests/test_aritmetica.rydit`

Operaciones matemáticas básicas:

```rydit
shield.init
dark.slot a = 10
dark.slot b = 3
voz a + b  # 13
voz a * b  # 30
```

### Test Funciones

**Archivo:** `tests/test_funciones.rydit`

Definición y llamado de funciones:

```rydit
rytmo saludar(nombre) {
    voz "Hola " + nombre
}
saludar("Mundo")
```

---

## 🔧 Módulos Disponibles

| Módulo | Funciones | Ejemplo |
|--------|-----------|---------|
| `math` | 8 funciones | `math::sumar(2, 3)` |
| `arrays` | 10 funciones | `arrays::push([1,2], 3)` |
| `strings` | 7+ funciones | `strings::upper("hola")` |
| `io` | 4+ funciones | `io::mkdir("test")` |
| `json` | 2 funciones | `json::parse("[1,2]")` |
| `random` | 3 funciones | `random::int(1, 6)` |
| `time` | 2 funciones | `time::now()` |

---

## 📝 Sintaxis Básica

### Variables

```rydit
dark.slot x = 100
dark.slot texto = "hola"
dark.slot lista = [1, 2, 3]
```

### Condicionales

```rydit
onif x > 50 {
    voz "Mayor"
}
```

### Loops

```rydit
ryda x < 10 {
    voz x
    dark.slot x = x + 1
}

cada elemento en lista {
    voz elemento
}
```

### Funciones

```rydit
rytmo sumar(a, b) {
    return a + b
}

dark.slot resultado = sumar(2, 3)
```

### Imports

```rydit
import math
import arrays as arr
import strings

voz math::sumar(2, 3)
voz arr::length([1, 2, 3])
```

---

## 🚀 Más Ejemplos

Para más ejemplos, ver:

- `tests/` - Tests automáticos
- `ejemplo.rydit` - Ejemplo básico
- `ejemplo_cada.rydit` - Ejemplo de loop `cada`
- `ejemplo_gfx.rydit` - Ejemplo gráfico

---

**Documentación completa:** `README.md` en la raíz del proyecto
