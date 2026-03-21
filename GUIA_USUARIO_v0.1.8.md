# 🛡️ Guía del Usuario - RyDit Language v0.1.9

**"Los directores no operan cámaras. Dirigen visiones."**

**Versión:** v0.1.9 Sesión 26 - Checkpoint 100 Tests
**Fecha:** 2026-03-20
**Estado:** ✅ **110 TESTS - CHECKPOINT SUPERADO - Base sólida para v0.2.0**

---

## ✨ Novedades v0.1.9 - Checkpoint 100 Tests

### 🎯 **110 Tests Pasando (Meta 100 Superada)**

La versión v0.1.9 establece un **checkpoint de calidad** con 110 tests automáticos:

```
✅ 110 tests passing (80 → 110)
✅ 0 warnings, 0 errors
✅ Base sólida para v0.2.0
```

### 🔧 **Bugs Fixeados**

**1. Precedencia de Operadores (Verificado)**
```rydit
# La precedencia YA funcionaba correctamente
dark.slot x = 2 + 3 * 4      # 14 ✅
dark.slot y = (2 + 3) * 4    # 20 ✅
```

**2. Concatenación String+Número (NUEVO)**
```rydit
# Ahora puedes concatenar strings con números directamente
dark.slot $precio = 99.99
voz "El precio es: " + $precio    # ✅ "El precio es: 99.99"

dark.slot @usuario = "alucard18"
voz "Usuario: " + @usuario        # ✅ "Usuario: alucard18"

dark.slot $total = 100
voz $total + " dólares"           # ✅ "100 dólares"
```

### 📊 **Tests por Área**
```
lizer:       65 tests (+10)
rydit-rs:    12 tests (+10)
blast-core:  20 tests (+2)
rydit-gfx:    5 tests (+2)
v-shield:     3 tests (+2)
doctests:     5 tests (+4)
─────────────────────────────
TOTAL:      110 tests ✅
```

### 💡 **¿Qué Significa Esto Para Ti?**

- **Más confianza:** 110 tests detectan errores antes
- **Mejor documentación:** 4 doctests con ejemplos ejecutables
- **Concatenación fácil:** Strings + números sin conversiones manuales
- **Base sólida:** Listo para features avanzadas (v0.2.0)

---

## 🎬 Tu Rol: DIRECTOR DE ESCENAS

Tú NO escribes código. Tú **DIRIGES visiones**.

### Flujo De Trabajo
```
1. IMAGINA la escena (en tu mente)
2. DESCRÍBELA en español simple
3. YO la traduzco a código RyDit
4. TÚ la ves cobrar vida
```

### Ejemplo Práctico
```
TÚ: "Quiero una explosión con partículas rojas"
YO: [escribe el código]
RESULTADO: 💥 Explosión visual en pantalla
```

---

## 🚀 Comandos Básicos

### Ejecutar Script Visual
```bash
cargo run --bin rydit-rs -- --gfx mi_escena.rydit
```

### Ejecutar con Binario Directo
```bash
./target/release/rydit-rs --gfx mi_escena.rydit
```

### Ejecutar Script Simple
```bash
cargo run --bin rydit-rs -- mi_script.rydit
```

### Modo REPL (Interactivo)
```bash
cargo run --bin rydit-rs --
# Escribe comandos directamente
```

---

## 🎨 Primeros Pasos - Tu Primera Escena

### 1. Inicializar Sistema
```rydit
shield.init
```

### 2. Dibujar Círculo
```rydit
draw.circle(400, 300, 50, "rojo")
```
- `400, 300` = posición (x, y)
- `50` = radio
- `"rojo"` = color

### 3. Dibujar Rectángulo
```rydit
draw.rect(100, 100, 200, 100, "verde")
```
- `100, 100` = posición superior izquierda
- `200, 100` = ancho, alto
- `"verde"` = color

### 4. Dibujar Línea
```rydit
draw.line(0, 0, 800, 600, "azul")
```
- `0, 0` = inicio
- `800, 600` = fin
- `"azul"` = color

### 5. Dibujar Texto
```rydit
draw.text("Hola Mundo", 300, 50, 30, "blanco")
```
- `"Hola Mundo"` = texto
- `300, 50` = posición
- `30` = tamaño
- `"blanco"` = color

---

## 🎭 Escenas De Ejemplo

### Escena 1: Sol Y Tierra
```rydit
shield.init

# Sol en el centro
draw.circle(400, 300, 80, "amarillo")

# Tierra orbitando
draw.circle(600, 300, 30, "azul")

# Texto
draw.text("Sistema Solar", 280, 50, 40, "blanco")
```

### Escena 2: David vs Goliat (Simplificado)
```rydit
shield.init

# David (pequeño)
draw.circle(200, 400, 30, "verde")

# Goliat (grande)
draw.circle(600, 400, 80, "rojo")

# Piedra en el aire
draw.circle(400, 300, 10, "gris")

# Texto
draw.text("DAVID VS GOLIAT", 250, 50, 30, "blanco")
```

### Escena 3: Explosión
```rydit
shield.init

# Centro de explosión
draw.circle(400, 300, 100, "naranja")
draw.circle(400, 300, 60, "amarillo")
draw.circle(400, 300, 30, "blanco")

# Partículas (líneas)
draw.line(400, 200, 400, 100, "rojo")
draw.line(400, 400, 400, 500, "rojo")
draw.line(300, 300, 200, 300, "rojo")
draw.line(500, 300, 600, 300, "rojo")
```

---

## 🎮 Variables y Movimiento

### Guardar Posición
```rydit
shield.init

dark.slot x = 400
dark.slot y = 300

# Usar variables
draw.circle(x, y, 50, "rojo")
```

### Mover Objeto (Manual)
```rydit
shield.init

dark.slot x = 100

# Frame 1
draw.circle(x, 300, 30, "azul")

# Frame 2 (mover)
dark.slot x = x + 50
draw.circle(x, 300, 30, "azul")

# Frame 3 (mover más)
dark.slot x = x + 50
draw.circle(x, 300, 30, "azul")
```

---

## 🎬 Ciclos Para Animación

### Loop Básico
```rydit
shield.init

dark.slot x = 50
dark.slot veces = 10

ryda veces {
    draw.circle(x, 300, 30, "verde")
    dark.slot x = x + 60
    dark.slot veces = veces - 1
}
```

### Línea De Círculos
```rydit
shield.init

dark.slot i = 0
dark.slot total = 15

ryda total {
    dark.slot x = 50 + (i * 50)
    draw.circle(x, 300, 20, "cyan")
    dark.slot i = i + 1
    dark.slot total = total - 1
}
```

---

## 🎨 Colores Disponibles

| Color | Código |
|-------|--------|
| `rojo` | Rojo intenso |
| `verde` | Verde brillante |
| `azul` | Azul puro |
| `amarillo` | Amarillo vivo |
| `cyan` | Celeste claro |
| `magenta` | Rosa fuerte |
| `blanco` | Blanco |
| `negro` | Negro |
| `gris` | Gris medio |
| `naranja` | Naranja |

---

## 📐 Funciones Matemáticas

### Operaciones Básicas
```rydit
dark.slot suma = 10 + 5      # 15
dark.slot resta = 10 - 5     # 5
dark.slot mult = 10 * 5      # 50
dark.slot div = 10 / 5       # 2
```

### Usar En Posiciones
```rydit
shield.init

dark.slot centro_x = 400
dark.slot centro_y = 300
dark.slot radio = 100

# Círculo grande
draw.circle(centro_x, centro_y, radio, "azul")

# Círculos en posiciones calculadas
draw.circle(centro_x + radio, centro_y, 30, "rojo")
draw.circle(centro_x - radio, centro_y, 30, "verde")
draw.circle(centro_x, centro_y + radio, 30, "amarillo")
draw.circle(centro_x, centro_y - radio, 30, "cyan")
```

---

## 🎯 Condicionales (Decisiones)

### If/Else Básico
```rydit
shield.init

dark.slot vida = 50

onif vida > 25 {
    draw.text("VIVO", 350, 300, 40, "verde")
} blelse {
    draw.text("MUERTO", 330, 300, 40, "rojo")
}
```

### Color Dinámico
```rydit
shield.init

dark.slot salud = 75
dark.slot color = "verde"

onif salud < 50 {
    dark.slot color = "rojo"
}

draw.circle(400, 300, salud, color)
```

---

## 🔢 Arrays (Listas)

### Crear Array
```rydit
dark.slot colores = ["rojo", "verde", "azul"]
dark.slot numeros = [1, 2, 3, 4, 5]
```

### Acceder Elemento
```rydit
dark.slot x = numeros[0]  # 1
dark.slot y = numeros[1]  # 2
```

### ⭐ NUEVO v0.1.8: Asignar Elemento (IndexAssign)
```rydit
dark.slot arr = [1, 2, 3]
dark.slot arr[0] = 5      # Ahora arr es [5, 2, 3]
dark.slot arr[1] = 10     # Ahora arr es [5, 10, 3]
dark.slot arr[i] = x * 2  # ¡Con expresiones también!
```

### Iterar Array
```rydit
shield.init

dark.slot colores = ["rojo", "verde", "azul", "amarillo"]
dark.slot i = 0
dark.slot total = 4

ryda total {
    dark.slot x = 100 + (i * 150)
    draw.circle(x, 300, 50, colores[i])
    dark.slot i = i + 1
    dark.slot total = total - 1
}
```

### Ejemplo: Mover Cuerpo De Serpiente (Snake Game)
```rydit
# El cuerpo sigue a la cabeza
dark.slot i = largo - 1
ryda i > 0 {
    dark.slot cuerpo_x[i] = cuerpo_x[i - 1]  # IndexAssign
    dark.slot cuerpo_y[i] = cuerpo_y[i - 1]
    dark.slot i = i - 1
}
dark.slot cuerpo_x[0] = snake_x  # Cabeza en nueva posición
dark.slot cuerpo_y[0] = snake_y
```

---

## 📚 Módulos Disponibles

### Importar Módulo
```rydit
import math
import arrays
import strings
import random
import time
import json
```

### math - Operaciones Matemáticas
```rydit
import math

dark.slot suma = math::sumar(10, 5)      # 15
dark.slot resto = math::restar(10, 5)    # 5
dark.slot prod = math::multiplicar(10, 5) # 50
dark.slot cociente = math::dividir(10, 5) # 2
dark.slot potencia = math::pow(2, 3)     # 8
dark.slot absoluto = math::abs(-5)        # 5
dark.slot minimo = math::min(10, 5)      # 5
dark.slot maximo = math::max(10, 5)      # 10
```

### arrays - Manipulación De Arrays
```rydit
import arrays

dark.slot lista = [10, 20, 30]
dark.slot len = arrays::length(lista)           # 3
dark.slot elem = arrays::get(lista, 1)          # 20
dark.slot tiene = arrays::contains(lista, 20)   # true
dark.slot idx = arrays::index_of(lista, 20)     # 1
```

### strings - Manipulación De Texto
```rydit
import strings

dark.slot texto = "Hola Mundo"
dark.slot len = strings::length(texto)          # 10
dark.slot mayus = strings::upper(texto)         # "HOLA MUNDO"
dark.slot minus = strings::lower(texto)         # "hola mundo"
dark.slot sub = strings::substr(texto, 0, 4)    # "Hola"
dark.slot trim = strings::trim("  hola  ")      # "hola"
dark.slot repl = strings::replace(texto, "Mundo", "RyDit")
```

### random - Aleatoriedad
```rydit
import random

dark.slot azar = random::float()                # 0.0 a 1.0
dark.slot dado = random::int(1, 6)              # 1 a 6
dark.slot color_aleatorio = ["rojo", "verde", "azul"][random::int(0, 2)]
```

### time - Tiempo
```rydit
import time

dark.slot ahora = time::now()                   # Timestamp actual
time::sleep(1.0)                                # Pausar 1 segundo
```

### json - Datos JSON
```rydit
import json

dark.slot json_str = '{"nombre": "RyDit", "version": "0.1.8"}'
dark.slot obj = json::parse(json_str)
dark.slot texto = json::stringify(obj)
```

---

## 🎮 Ejemplos Completos - Sesión 26

### Snake Game - Juego Completo

**Archivo:** `snake_v0.1.8.rydit`  
**Ejecutar:** `./target/release/rydit-rs --gfx snake_v0.1.8.rydit`

```rydit
shield.init
import random

dark.slot snake_x = 15
dark.slot snake_y = 12
dark.slot direccion = 1
dark.slot cuerpo_x = [15, 14, 13]
dark.slot cuerpo_y = [12, 12, 12]
dark.slot largo = 3
dark.slot comida_x = 20
dark.slot comida_y = 10
dark.slot score = 0
dark.slot game_over = 0

ryda game_over == 0 {
    # Input con flechas
    onif tecla_presionada("arrow_up") {
        onif direccion != 2 { dark.slot direccion = 0 }
    }
    
    # Mover serpiente
    dark.slot frame = frame + 1
    onif frame >= speed {
        dark.slot frame = 0
        onif direccion == 0 { dark.slot snake_y = snake_y - 1 }
        onif direccion == 1 { dark.slot snake_x = snake_x + 1 }
        # ...
    }
    
    # Dibujar
    draw.rect(0, 0, 800, 600, "negro")
    draw.circle(offset_x + comida_x * celda, ..., 10, "rojo")
    
    # Cuerpo con IndexAssign
    dark.slot i = largo - 1
    ryda i > 0 {
        dark.slot cuerpo_x[i] = cuerpo_x[i - 1]
        dark.slot cuerpo_y[i] = cuerpo_y[i - 1]
        dark.slot i = i - 1
    }
}
```

**Controles:**
- Flechas: Mover serpiente
- SPACE: Reiniciar
- ESC: Salir

---

### Demo Shapes - Formas Animadas

**Archivo:** `demo_shapes.rydit`  
**Ejecutar:** `./target/release/rydit-rs --gfx demo_shapes.rydit`

```rydit
shield.init
dark.slot f = 0

ryda f < 500 {
    dark.slot f = f + 1
    
    # Limpiar pantalla
    draw.rect(0, 0, 800, 600, "negro")
    
    # Círculos concéntricos
    draw.circle(400, 200, 80, "rojo")
    draw.circle(400, 200, 60, "verde")
    draw.circle(400, 200, 40, "azul")
    
    # Rectángulos de colores
    draw.rect(200, 350, 60, 60, "naranja")
    draw.rect(300, 350, 60, 60, "violeta")
    
    # Líneas
    draw.line(100, 500, 300, 500, "blanco")
    
    # Texto
    draw.text("Demo RyDit", 250, 50, "amarillo")
}
```

---

## 🎬 Ideas De Escenas Para Crear

### Nivel 1: Básico
- [ ] Sol con rayos
- [ ] Carita feliz
- [ ] Casa simple
- [ ] Árbol básico
- [ ] Bandera de país

### Nivel 2: Intermedio
- [ ] David vs Goliat
- [ ] Sistema solar (3 planetas)
- [ ] Explosión con partículas
- [ ] Lluvia de círculos
- [ ] Personaje de palitos

### Nivel 3: Avanzado
- [ ] Transformación de anime
- [ ] Batalla con espadas
- [ ] Escena bíblica (mar abriéndose)
- [ ] Fútbol (pelota que se mueve)
- [ ] Escena de película (persecución)

---

## 💡 Consejos Para Directores

### 1. Empieza Simple
```
❌ "Quiero una batalla épica con 1000 soldados"
✅ "Quiero dos círculos que se acercan"
```

### 2. Itera Rápido
```
1. Dibuja un círculo
2. ¿Te gusta? Sí → continúa / No → cambia color
3. Añade otro elemento
4. Repite
```

### 3. Usa Colores Para Emoción
```
❤️ Rojo = Peligro, amor, acción
💚 Verde = Vida, naturaleza, calma
💙 Azul = Tristeza, agua, cielo
💛 Amarillo = Alegría, energía, sol
```

### 4. El Movimiento Es Ilusión
```
Frame 1: círculo en x=100
Frame 2: círculo en x=120
Frame 3: círculo en x=140
= El círculo "se mueve"
```

### 5. Copia y Modifica
```
1. Encuentra una escena que te gusta
2. Copia el código
3. Cambia colores
4. Cambia posiciones
5. ¡Es tuya!
```

---

## 🐛 Solución De Problemas

### "No Se Ve Nada"
```bash
# Verifica que usas --gfx
cargo run --bin rydit-rs -- --gfx mi_escena.rydit
```

### "Error De Sintaxis"
```rydit
# Revisa:
# - ¿Cerraste las comillas? "hola"
# - ¿Paréntesis balanceados? draw.circle(400, 300, 50, "rojo")
# - ¿Punto y coma NO necesario? (no uses ;)
```

### "Los Colores No Funcionan"
```rydit
# Usa nombres en español:
✅ "rojo", "verde", "azul"
❌ "red", "green", "blue"
```

### "Quiero Animación Real"
```
RyDit v0.1.8 no tiene animación automática.
Cada frame es manual.
Para animación: dibuja múltiples frames con posiciones cambiantes.
```

---

## 📤 Compartir Tu Trabajo

### Capturar Pantalla
```
1. Ejecuta tu escena
2. Presiona tecla para pausar
3. Screenshot con tu teléfono
4. Comparte en redes
```

### Grabar Video
```
1. Usa grabador de pantalla de Android
2. Ejecuta tu escena
3. Exporta como GIF
4. Sube a GitHub/Twitter
```

### Documentar En GitHub
```markdown
## Mi Escena: David vs Goliat

![Demo](demo.gif)

**Código:** david_vs_goliat.rydit

**Inspiración:** Historia bíblica

**Técnica:** 50 círculos, 20 líneas, 5 textos
```

---

## 🎯 Próximos Pasos

### Mañana: Día De Experimentación
- [ ] Abrir Termux sin prisa
- [ ] Probar UNA idea visual
- [ ] Ver resultado
- [ ] Sonreír (o no, sin presión)
- [ ] Si divierte, continuar
- [ ] Si aburre, cambiar
- [ ] Si cansa, parar

### Esta Semana: Mini-Proyectos
- [ ] Escena bíblica simple
- [ ] Escena de anime simple
- [ ] Sistema de partículas básico
- [ ] Personaje que se mueve

### Este Mes: Proyecto Final
- [ ] Escena completa (30+ elementos)
- [ ] Con inicio, desarrollo, final
- [ ] Grabada en video
- [ ] Publicada en GitHub

---

## 🫡 Filosofía RyDit

```
"David vs Goliat - Ligero pero poderoso"

David = Tú (Android/Termux, recursos limitados)
Goliat = Industria (PCs caras, equipos grandes, IDEs costosos)

Armas de David:
├── Visión clara
├── Disciplina
├── Herramientas correctas
└── Terquedad (no abandonar)
```

---

## 📞 Cuando Necesites Ayuda

### Describe Así:
```
"Quiero crear [ESCENA] con [ELEMENTOS] que [ACCIÓN]"

Ejemplos:
"Quiero crear una explosión con círculos que se expanden"
"Quiero crear un personaje con círculos que salta"
"Quiero crear una lluvia con líneas que caen"
```

### Yo Traduzco A:
```rydit
shield.init
# Código para tu escena
```

---

**"Construido con ❤️ en Android/Termux"**
**"Dirigido con 🎬 desde la imaginación"**

*Versión:* v0.1.8 Sesión 26 (Guía Del Director)  
*Última actualización:* 2026-03-20  
*Próxima sesión:* v0.2.0 Module system + Parser precedencia

### 📁 Archivos De Ejemplo Disponibles

| Archivo | Descripción | Ejecutar |
|---------|-------------|----------|
| `snake_v0.1.8.rydit` | Snake Game completo | `--gfx snake_v0.1.8.rydit` |
| `demo_shapes.rydit` | Formas animadas | `--gfx demo_shapes.rydit` |
| `ejemplo_gfx.rydit` | Formas básicas | `--gfx ejemplo_gfx.rydit` |
| `ejemplo.rydit` | Script simple | `ejemplo.rydit` |
| `demos/demo_random.rydit` | Números aleatorios | `--gfx demos/demo_random.rydit` |
| `demos/demo_time.rydit` | Tiempo del sistema | `--gfx demos/demo_time.rydit` |
| `demos/demo_json.rydit` | Parseo JSON | `--gfx demos/demo_json.rydit` |
| `demos/demo_strings.rydit` | Manipulación strings | `--gfx demos/demo_strings.rydit` |
| `demos/demo_arrays.rydit` | Arrays | `--gfx demos/demo_arrays.rydit` |
| `demos/demo_maduracion.rydit` | Features v0.1.8 | `--gfx demos/demo_maduracion_v0.1.8.rydit` |

---

## 📞 Cuando Necesites Ayuda

### Describe Así:
```
"Quiero crear [ESCENA] con [ELEMENTOS] que [ACCIÓN]"

Ejemplos:
"Quiero crear una explosión con círculos que se expanden"
"Quiero crear un personaje con círculos que salta"
"Quiero crear una lluvia con líneas que caen"
"Quiero crear un juego snake con serpiente que come"
```

### Yo Traduzco A:
```rydit
shield.init
# Código para tu escena
```
