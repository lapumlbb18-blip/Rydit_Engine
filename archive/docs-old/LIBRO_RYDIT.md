# 📘 LIBRO RYDIT - Guía Completa del Lenguaje

**Versión:** v0.0.14 (Funciones en Expresiones)  
**Fecha:** 2026-03-16  
**Estado:** ✅ Funcional - Listo para uso experimental

---

## 🎯 ¿Qué es RyDit?

RyDit es un lenguaje de scripting diseñado para **Android/Termux** con integración nativa de gráficos (raylib) y audio.

**Filosofía:**
- Simple pero poderoso
- Optimizado para dispositivos con poca RAM
- Sintaxis única y expresiva
- Integración perfecta con gráficos

---

## 📋 Tabla de Contenidos

1. [Inicio Rápido](#inicio-rápido)
2. [Sintaxis Básica](#sintaxis-básica)
3. [Variables](#variables)
4. [Operadores](#operadores)
5. [Control de Flujo](#control-de-flujo)
6. [Funciones](#funciones)
7. [Arrays](#arrays)
8. [Gráficos](#gráficos)
9. [Ejemplos Completos](#ejemplos-completos)

---

## 🚀 Inicio Rápido

### Instalación

```bash
# En Termux (Android)
pkg install raylib
git clone <tu-repo>
cd shield-project
cargo build
```

### Primer Script

```rydit
# Hola Mundo
shield.init
voz "Hola Mundo"
```

### Ejecutar

```bash
# Modo texto
cargo run -- "script.rydit"

# Modo gráfico
cargo run -- --gfx "script.rydit"
```

---

## 📝 Sintaxis Básica

### Comentarios

```rydit
# Esto es un comentario
voz "Hola"  # Comentario en línea
```

### Statements vs Expresiones

```rydit
# Statement - ejecuta una acción
voz "Hola"

# Expresión - evalúa a un valor
dark.slot x = 5 + 3  # 5 + 3 es una expresión
```

---

## 📦 Variables

### Declaración

```rydit
# dark.slot declara variables
dark.slot x = 100
dark.slot nombre = "Heroe"
dark.slot activo = true
dark.slot precio = 19.99
```

### Tipos

| Tipo | Ejemplo | Descripción |
|------|---------|-------------|
| `num` | `42`, `3.14` | Números (f64) |
| `texto` | `"Hola"` | Strings |
| `bool` | `true`, `false` | Booleanos |
| `array` | `[1, 2, 3]` | Arrays |

### Nombres con Puntos

```rydit
# Los puntos permiten nombres compuestos
dark.slot jugador.vida = 100
dark.slot delta.time = 0.016
```

---

## 🔢 Operadores

### Aritméticos

```rydit
dark.slot a = 10 + 5   # Suma: 15
dark.slot b = 10 - 5   # Resta: 5
dark.slot c = 10 * 5   # Multiplicación: 50
dark.slot d = 10 / 5   # División: 2
```

### Comparación

```rydit
dark.slot x = 10 > 5    # true
dark.slot y = 10 < 5    # false
dark.slot z = 10 = 10   # true (igualdad)
```

### Lógicos

```rydit
dark.slot a = true and false  # false
dark.slot b = true or false   # true
dark.slot c = not true        # false
```

### Strings

```rydit
# Concatenación con +
dark.slot saludo = "Hola " + "Mundo"
voz saludo  # "Hola Mundo"
```

### Paréntesis

```rydit
# Alteran precedencia
dark.slot x = (2 + 3) * 4    # 20
dark.slot y = 2 + (3 * 4)    # 14
```

---

## 🎛️ Control de Flujo

### Condicionales (onif/blelse)

```rydit
dark.slot edad = 18

onif edad >= 18 {
    voz "Mayor de edad"
} blelse {
    voz "Menor de edad"
}
```

### Ciclos (ryda)

```rydit
# While loop
dark.slot x = 10
ryda x > 0 {
    voz x
    dark.slot x = x - 1
}
```

### For Each (cada)

```rydit
# Iterar sobre arrays
dark.slot numeros = [1, 2, 3, 4, 5]

cada n en numeros {
    voz n
}
```

### Break

```rydit
# Salir de loops
ryda true {
    voz "Infinito..."
    break  # Sale del loop
}
```

---

## 🎨 Funciones

### Definición

```rydit
# rytmo define funciones
rytmo saludar() {
    voz "Hola!"
}

# Llamar función
saludar()
```

### Parámetros y Retorno

```rydit
# Función con parámetros y retorno
rytmo sumar(a, b) {
    return a + b
}

# Usar en expresiones
dark.slot resultado = sumar(5, 3)
voz resultado  # 8
```

### Composición de Funciones

```rydit
rytmo cuadrado(x) {
    return x * x
}

# Funciones anidadas
dark.slot x = cuadrado(sumar(2, 3))
voz x  # 25 (2+3=5, 5*5=25)
```

### Funciones con Texto

```rydit
rytmo saludar(nombre) {
    return "Hola " + nombre
}

dark.slot saludo = saludar("Mundo")
voz saludo  # "Hola Mundo"
```

---

## 📊 Arrays

### Creación

```rydit
# Array literal
dark.slot numeros = [1, 2, 3, 4, 5]

# Array vacío
dark.slot vacio = []
```

### Indexación

```rydit
dark.slot x = [10, 20, 30]
voz x[0]  # 10
voz x[1]  # 20
```

### Arrays Multidimensionales

```rydit
# Array de arrays
dark.slot matriz = [[1, 2], [3, 4]]
voz matriz[0][0]  # 1
voz matriz[1][1]  # 4
```

### Iteración

```rydit
dark.slot frutas = ["manzana", "banana", "cereza"]

cada fruta en frutas {
    voz fruta
}
```

---

## 🎮 Gráficos

### Inicialización

```rydit
# Modo gráfico requiere --gfx flag
shield.init

# Los comandos draw.* solo funcionan en modo gráfico
draw.circle(400, 300, 50, "rojo")
```

### Formas Básicas

```rydit
# Círculo
draw.circle(x, y, radio, "color")

# Rectángulo
draw.rect(x, y, ancho, alto, "color")

# Línea
draw.line(x1, y1, x2, y2, "color")

# Texto
draw.text("Hola", x, y, tamano, "color")
```

### Colores Soportados

```
"rojo", "verde", "azul", "negro", "blanco",
"amarillo", "gris", "naranja", "morado"
```

### Game Loop

```rydit
shield.init

dark.slot x = 400
dark.slot y = 300

ryda true {
    # Limpiar pantalla
    draw.rect(0, 0, 800, 600, "negro")
    
    # Dibujar círculo
    draw.circle(x, y, 50, "rojo")
    
    # Input
    onif tecla_presionada("escape") {
        break
    }
}
```

---

## 📚 Ejemplos Completos

### Ejemplo 1: Calculadora

```rydit
# calculadora.rydit
shield.init

rytmo sumar(a, b) { return a + b }
rytmo restar(a, b) { return a - b }
rytmo multiplicar(a, b) { return a * b }
rytmo dividir(a, b) { return a / b }

# Uso
dark.slot x = 10
dark.slot y = 5

voz "Suma:"
voz sumar(x, y)

voz "Resta:"
voz restar(x, y)

voz "Multiplicación:"
voz multiplicar(x, y)

voz "División:"
voz dividir(x, y)
```

### Ejemplo 2: Fibonacci

```rydit
# fibonacci.rydit
shield.init

rytmo fibonacci(n) {
    onif n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

# Imprimir primeros 10 números
dark.slot i = 0
ryda i < 10 {
    voz fibonacci(i)
    dark.slot i = i + 1
}
```

### Ejemplo 3: Juego Simple

```rydit
# juego.rydit
shield.init

dark.slot jugador_x = 400
dark.slot jugador_y = 300
dark.slot velocidad = 5

ryda true {
    # Limpiar
    draw.rect(0, 0, 800, 600, "negro")
    
    # Dibujar jugador
    draw.circle(jugador_x, jugador_y, 20, "verde")
    
    # Input
    onif tecla_presionada("arrow_up") {
        dark.slot jugador_y = jugador_y - velocidad
    }
    onif tecla_presionada("arrow_down") {
        dark.slot jugador_y = jugador_y + velocidad
    }
    onif tecla_presionada("arrow_left") {
        dark.slot jugador_x = jugador_x - velocidad
    }
    onif tecla_presionada("arrow_right") {
        dark.slot jugador_x = jugador_x + velocidad
    }
    onif tecla_presionada("escape") {
        break
    }
}
```

---

## 🛠️ Comandos de Terminal

### Compilar y Ejecutar

```bash
# Modo texto
cargo run -- "script.rydit"

# Modo gráfico
cargo run -- --gfx "script.rydit"

# REPL interactivo
cargo run -- --repl
```

### Tests

```bash
# Ejecutar todos los tests
cargo test

# Test específico
cargo test test_sumar
```

### Build

```bash
# Build debug
cargo build

# Build release (optimizado)
cargo build --release

# Verificar errores
cargo check
```

---

## 📖 Referencia Rápida

### Keywords

| Keyword | Descripción |
|---------|-------------|
| `shield.init` | Inicializar sistema |
| `dark.slot` | Declarar variable |
| `voz` | Imprimir valor |
| `onif/blelse` | Condicional |
| `ryda` | While loop |
| `cada/en` | For each loop |
| `rytmo` | Definir función |
| `return` | Retornar valor |
| `break` | Salir de loop |

### Built-in Functions

| Función | Descripción |
|---------|-------------|
| `voz(expr)` | Imprimir expresión |
| `input()` | Leer input de usuario |
| `tecla_presionada("tecla")` | Verificar teclado (gráficos) |
| `sumar(a, b, ...)` | Sumar números |
| `restar(a, b)` | Restar dos números |
| `multiplicar(a, b, ...)` | Multiplicar números |
| `dividir(a, b)` | Dividir dos números |

---

## 🎓 Próximos Pasos

1. **Prueba los ejemplos** - Copia y ejecuta los scripts de ejemplo
2. **Modifica el código** - Cambia valores y observa qué pasa
3. **Crea tus propios scripts** - Empieza pequeño, crece gradualmente
4. **Reporta bugs** - Encuentra problemas, ayuda a mejorar

---

## 📞 Soporte

- **GitHub:** [tu-repo]
- **Documentación:** diagnostico/
- **Ejemplos:** *.rydit en el root

---

**¡Feliz scripting con RyDit! 🚀**

*Versión v0.0.14 - Funciones en Expresiones*
