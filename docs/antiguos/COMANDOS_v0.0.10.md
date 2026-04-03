# 📚 COMANDOS RYDIT v0.0.10

**Estado:** Prototipo Funcional  
**Fecha:** 2026-03-16  
**Honesto:** Esto es lo que SÍ funciona actualmente

---

## ⚠️ LIMITACIONES CONOCIDAS

**IMPORTANTE:** Las funciones tienen un bug de diseño:
- Los parámetros **sobrescriben** variables globales
- Los argumentos **no se pasan** realmente a las funciones
- No hay scope local real

Esto se arreglará en v0.0.11.

---

## 🎯 COMANDOS DISPONIBLES

### Variables

```rydit
# Declarar variable
dark.slot nombre = "valor"
dark.slot edad = 25
dark.slot delta = 0.5
dark.slot activo = true
dark.slot vacio = vacío
dark.slot lista = [1, 2, 3]
dark.slot matriz = [[1, 2], [3, 4]]
```

**Funciona:** ✅ 100%

---

### Operadores

```rydit
# Aritméticos
dark.slot suma = 5 + 3
dark.slot resta = 10 - 4
dark.slot mult = 6 * 7
dark.slot div = 100 / 4
dark.slot mod = 17 % 5

# Comparación
dark.slot mayor = 10 > 5
dark.slot menor = 3 < 8
dark.slot igual = 5 == 5

# Lógicos
dark.slot y = true and false
dark.slot o = true or false
dark.slot no = not true
```

**Funciona:** ✅ 100%

---

### Condicionales

```rydit
# Onif básico
onif edad >= 18 {
    voz "Mayor de edad"
}

# Onif con blelse
onif nota >= 6 {
    voz "Aprobado"
} blelse {
    voz "Reprobado"
}

# Múltiples condiciones
onif dia == "lunes" {
    voz "Inicio de semana"
} onif dia == "viernes" {
    voz "Fin de semana"
} blelse {
    voz "Día normal"
}
```

**Funciona:** ✅ 100%

---

### Ciclos

```rydit
# Ryda (while) básico
dark.slot i = 0
ryda i < 10 {
    voz i
    i = i + 1
}

# Ryda con break
ryda true {
    voz "Infinito... o no"
    break
}

# Ryda con cada (for each)
dark.slot colores = ["rojo", "verde", "azul"]
cada color en colores {
    voz color
}

# Ryda con array numérico
dark.slot nums = [1, 2, 3, 4, 5]
cada n en nums {
    voz n * 2
}
```

**Funciona:** ✅ 100%

---

### Funciones (Sintaxis)

```rydit
# Definir función
rytmo saludar(nombre) {
    voz "Hola " + nombre
}

# Llamar función
saludar("Mundo")

# Función con retorno (limitado)
rytmo sumar(a, b) {
    dark.slot resultado = a + b
    return resultado
}

# Llamar con argumentos (BUG: no se pasan realmente)
sumar(5, 3)  # Imprime pero no usa los args
```

**Funciona:** ⚠️ **Parcialmente**
- ✅ Sintaxis funciona
- ❌ Argumentos no se pasan realmente
- ❌ Parámetros sobrescriben variables globales

---

### Arrays

```rydit
# Crear array
dark.slot lista = [1, 2, 3, 4, 5]

# Acceder por índice
voz lista[0]  # 1
voz lista[2]  # 3

# Modificar por índice
lista[1] = 100

# Array multidimensional
dark.slot tablero = [
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0]
]

voz tablero[1][2]  # 0

# Cada en array multidimensional
cada fila en tablero {
    cada celda en fila {
        voz celda
    }
}
```

**Funciona:** ✅ 100%

---

### Input/Voz

```rydit
# Voz (print)
voz "Hola Mundo"
voz 42
voz true

# Input (limitado)
dark.slot nombre = input("Tu nombre: ")
voz "Hola " + nombre

# Input numérico
dark.slot edad = input("Tu edad: ")
# Si ingresa número: Valor::Num
# Si ingresa texto: Valor::Texto
```

**Funciona:** ✅ 100%

---

### Gráficos (Modo --gfx)

```rydit
# Requiere: cargo run -- --gfx script.rydit

# Dibujar círculo
draw.circle(400, 300, 50, "rojo")

# Dibujar rectángulo
draw.rect(100, 100, 200, 150, "azul")

# Dibujar línea
draw.line(0, 0, 800, 600, "verde")

# Dibujar texto
draw.text("Hola RyDit", 300, 300, 40, "blanco")

# Colores disponibles
# "rojo", "verde", "azul", "amarillo", "negro",
# "blanco", "naranja", "rosa", "morado", "gris"
```

**Funciona:** ✅ 100% (con --gfx)

---

### Input en Gráficos

```rydit
# Detectar tecla presionada (por frame)
onif tecla_presionada("escape") {
    break
}

# Teclas disponibles
# "escape", "enter", "space", "up", "down",
# "left", "right", "a", "b", "c", etc.
```

**Funciona:** ✅ 100% (con --gfx)

---

## 🎮 EJEMPLO COMPLETO: Snake

```rydit
# snake.rydit
# Ejecutar: cargo run -- --gfx snake.rydit

shield.init

dark.slot tablero_x = 30
dark.slot tablero_y = 25
dark.slot celda_size = 20

dark.slot serpiente_x = 15
dark.slot serpiente_y = 12
dark.slot direccion_x = 0
dark.slot direccion_y = 0

dark.slot comida_x = 10
dark.slot comida_y = 10
dark.slot score = 0

dark.slot game_over = false
dark.slot frame_counter = 0

ryda not game_over {
    # Input
    onif tecla_presionada("arrow_up") {
        direccion_x = 0
        direccion_y = -1
    }
    onif tecla_presionada("arrow_down") {
        direccion_x = 0
        direccion_y = 1
    }
    onif tecla_presionada("arrow_left") {
        direccion_x = -1
        direccion_y = 0
    }
    onif tecla_presionada("arrow_right") {
        direccion_x = 1
        direccion_y = 0
    }
    onif tecla_presionada("escape") {
        break
    }

    # Limitar velocidad (cada 10 frames)
    onif frame_counter % 10 == 0 {
        # Mover serpiente
        serpiente_x = serpiente_x + direccion_x
        serpiente_y = serpiente_y + direccion_y

        # Colisión con paredes
        onif serpiente_x < 0 {
            game_over = true
        }
        onif serpiente_x >= tablero_x {
            game_over = true
        }
        onif serpiente_y < 0 {
            game_over = true
        }
        onif serpiente_y >= tablero_y {
            game_over = true
        }

        # Comer comida
        onif serpiente_x == comida_x and serpiente_y == comida_y {
            score = score + 1
            comida_x = random(tablero_x)
            comida_y = random(tablero_y)
        }
    }

    frame_counter = frame_counter + 1

    # Dibujar
    draw.rect(0, 0, tablero_x * celda_size, tablero_y * celda_size, "negro")
    draw.circle(
        serpiente_x * celda_size + celda_size / 2,
        serpiente_y * celda_size + celda_size / 2,
        celda_size / 2,
        "verde"
    )
    draw.circle(
        comida_x * celda_size + celda_size / 2,
        comida_y * celda_size + celda_size / 2,
        celda_size / 2,
        "rojo"
    )
    draw.text("Score: " + score, 10, 10, 20, "blanco")

    onif game_over {
        draw.text("GAME OVER", 300, 300, 40, "rojo")
        draw.text("SPACE para reiniciar", 280, 350, 20, "blanco")
        
        onif tecla_presionada("space") {
            game_over = false
            serpiente_x = 15
            serpiente_y = 12
            score = 0
        }
    }
}
```

**Funciona:** ✅ 100%

---

## 🧪 COMANDOS NO DISPONIBLES (Aún)

```rydit
# ❌ No hay módulos
import "mi_archivo.rydit"

# ❌ No hay structs/clases
struct Jugador {
    nombre: texto,
    score: num
}

# ❌ No hay pattern matching
match valor {
    1 => voz "uno",
    2 => voz "dos",
    _ => voz "otro"
}

# ❌ No hay manejo de errores
try {
    # código
} catch {
    # error
}

# ❌ No hay async/await (planeado para v0.0.11)
async rytmo descargar(url) {
    let datos = await fetch(url)
}
```

---

## 📊 ESTADO DE CADA COMANDO

| Comando | Estado | Tests | Notas |
|---------|--------|-------|-------|
| `dark.slot` | ✅ 100% | ✅ | Variables, arrays |
| `voz` | ✅ 100% | ✅ | Print |
| `input` | ✅ 100% | ✅ | Input seguro |
| `onif/blelse` | ✅ 100% | ✅ | Condicionales |
| `ryda` | ✅ 100% | ✅ | While loop |
| `cada` | ✅ 100% | ✅ | For each |
| `break` | ✅ 100% | ✅ | Sale de loops |
| `rytmo` | ⚠️ 50% | ❌ | Bug de scope |
| `return` | ✅ 100% | ✅ | Retorna valor |
| `draw.*` | ✅ 100% | ✅ | Solo --gfx |
| `tecla_presionada` | ✅ 100% | ✅ | Solo --gfx |
| `import` | ❌ 0% | ❌ | No implementado |
| `async/await` | ❌ 0% | ❌ | Planeado v0.0.11 |

---

## 🔧 COMANDO DE EJECUCIÓN

```bash
# Modo texto
cargo run -- script.rydit

# Modo gráfico (requiere Termux:X11)
cargo run -- --gfx script.rydit
cargo run -- -g script.rydit  # Shortcut

# Verificar código
cargo check

# Ejecutar tests
cargo test

# Build completo
cargo build
```

---

## 📝 NOTAS IMPORTANTES

1. **Funciones Bug:** Los parámetros de funciones sobrescriben variables globales. No usar funciones con parámetros del mismo nombre que variables globales.

2. **Gráficos:** Requiere `--gfx` flag y Termux:X11 para display.

3. **Arrays:** Índices fuera de rango causan error en runtime.

4. **Input:** `input()` puede retornar `Valor::Num` o `Valor::Texto` dependiendo del input.

---

## 🎯 PRÓXIMAMENTE (v0.0.11+)

- ✅ Fix de scope en funciones
- ✅ Argumentos que se pasan realmente
- ✅ Async/await con tasks
- ✅ Thread pool básico
- ✅ Channels (send/recv)
- ✅ Módulos (`import`)

---

**Última actualización:** 2026-03-16  
**Versión:** v0.0.10 (Prototipo Funcional)
