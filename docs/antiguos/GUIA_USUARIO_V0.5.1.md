# 📘 GUÍA DE USUARIO RYDIT v0.6.0

**Versión:** v0.6.0
**Fecha:** 2026-03-23
**Tiempo de lectura:** 10 minutos

---

## 🚀 EMPEZANDO EN 5 MINUTOS

### 1. Instalar y Configurar

```bash
# En Termux (Android)
pkg install rust raylib

# Clonar repositorio
git clone https://github.com/lapumlbb18-blip/Rydit_Engine.git
cd Rydit_Engine

# Compilar
cargo build --release
```

### 2. Tu Primer Script Gráfico

Crea `hola_mundo.rydit`:
```rydit
shield.init

ryda frame < 1000 {
    # Limpiar pantalla
    draw.rect(0, 0, 800, 600, "negro")

    # Dibujar círculo
    draw.circle(400, 300, 50, "rojo")

    # Dibujar texto
    draw.text("¡Hola Mundo RyDit!", 280, 50, "blanco")
}
```

**Ejecutar (v0.6.0 - AUTO-CONFIG):**
```bash
# ¡Ya no necesitas variables manuales!
./target/release/rydit-rs --gfx hola_mundo.rydit

# Output automático en Termux:
# [RYDIT] Termux detectado - Configurando entorno gráfico...
# [RYDIT] DISPLAY=:0 configurado automáticamente
# [RYDIT] zink GPU driver configurado automáticamente
# [RYDIT] DRI3=1 configurado automáticamente
# [RYDIT] ✅ Entorno gráfico listo para Termux-X11
```

---

## 📖 SINTAXIS BÁSICA

### Variables

```rydit
# Números
dark.slot vida = 100
dark.slot velocidad = 5.5

# Texto
dark.slot nombre = "Heroe"

# Arrays
dark.slot enemigos = [1, 2, 3]

# Booleanos
dark.slot vivo = verdadero
dark.slot muerto = falso
```

### Condicionales

```rydit
# If simple
onif vida > 0 {
    voz "Estás vivo"
}

# If-else
onif vida > 50 {
    voz "Salud alta"
} blelse {
    voz "Salud baja"
}
```

### Ciclos

```rydit
# While
dark.slot x = 10
ryda x > 0 {
    voz x
    dark.slot x = x - 1
}

# For each
dark.slot lista = [1, 2, 3]
cada elemento en lista {
    voz elemento
}
```

### Funciones

```rydit
# Definir función
rytmo saludar(nombre) {
    voz "Hola " + nombre
    return 1
}

# Llamar función
saludar("Mundo")
```

---

## 🎨 GRÁFICOS

### Formas Básicas

```rydit
shield.init

ryda frame < 1000 {
    # Limpiar
    draw.rect(0, 0, 800, 600, "negro")
    
    # Círculo: x, y, radio, color
    draw.circle(400, 300, 50, "rojo")
    
    # Rectángulo: x, y, ancho, alto, color
    draw.rect(100, 100, 100, 100, "verde")
    
    # Línea: x1, y1, x2, y2, color
    draw.line(0, 0, 800, 600, "azul")
    
    # Texto: texto, x, y, tamaño, color
    draw.text("RyDit v0.5.1", 300, 50, "amarillo")
}
```

### Colores Disponibles

```
rojo, verde, azul, amarillo, blanco, negro
magenta, rosa, naranja, gris, cyan
morado, cafe, lima, azul_oscuro, olivo, turquesa, vino
```

### Sprites / Texturas

```rydit
shield.init

# Cargar texturas (FUERA del game loop)
assets::load_texture("tank", "sprites/tank_16x16.png")
assets::load_texture("heli", "sprites/helicopter_16x16.png")

ryda frame < 5000 {
    # Limpiar
    draw.rect(0, 0, 800, 600, "negro")
    
    # Dibujar textura: id, x, y, color_opcional
    assets::draw("tank", 100, 200, "blanco")
    
    # Dibujar escalada: id, x, y, escala, color_opcional
    assets::draw_scaled("heli", 300, 200, 3, "blanco")  # 3x scale
}
```

**Funciones de Assets:**
```rydit
# Cargar textura
assets::load_texture("id", "ruta/al/archivo.png")

# Dibujar
assets::draw("id", x, y)
assets::draw("id", x, y, "color")

# Dibujar escalada
assets::draw_scaled("id", x, y, escala)
assets::draw_scaled("id", x, y, escala, "color")

# Verificar
si assets::has("id") {
    voz "Existe"
}

# Dimensiones
dark.slot ancho = assets::width("id")
dark.slot alto = assets::height("id")
```

---

## 🎮 INPUT

### Teclado

```rydit
shield.init

dark.slot x = 400
dark.slot y = 300

ryda frame < 1000 {
    draw.rect(0, 0, 800, 600, "negro")
    draw.circle(x, y, 20, "rojo")
    
    # Input continuo
    onif tecla_presionada("w") {
        dark.slot y = y - 2
    }
    onif tecla_presionada("s") {
        dark.slot y = y + 2
    }
    onif tecla_presionada("a") {
        dark.slot x = x - 2
    }
    onif tecla_presionada("d") {
        dark.slot x = x + 2
    }
    onif tecla_presionada("escape") {
        break  # Salir
    }
}
```

### Mouse

```rydit
# Posición del mouse
dark.slot mx = input::mouse_x()
dark.slot my = input::mouse_y()

# Click
onif input::is_mouse_button_pressed(0) {  # 0=izq, 1=der, 2=medio
    voz "Click izquierdo"
}
```

---

## 📦 MÓDULOS

### Importar Módulos

```rydit
# Importar módulo completo
import math
import arrays
import strings

# Importar con alias
import math as m
import arrays as arr

# Usar funciones
dark.slot suma = math::sumar(10, 5)
dark.slot raiz = m::sqrt(16)
dark.slot len = arrays::length([1, 2, 3])
```

### 🆕 NOVEDAD v0.6.0 - MÓDULOS EMBEBIDOS

**¡Ya no necesitas archivos externos!**

Los módulos ahora están **embebidos en el binario**:

```rydit
# Funciona sin tener modules/math.rydit
import math
import arrays
import strings
```

**Ventajas:**
- ✅ No necesitas gestionar archivos `.rydit`
- ✅ `import math` funciona siempre
- ✅ Binario auto-contenido

**Override (usuarios avanzados):**
```bash
# Si creas modules/math.rydit local, RyDit usará tu versión
mkdir -p modules
nano modules/math.rydit  # Tu versión personalizada
./rydit-rs --gfx demo.rydit  # Usa tu versión local
```

**Módulos disponibles:**
| Módulo | Alias | Funciones Principales |
|--------|-------|----------------------|
| `math` | - | `sumar()`, `restar()`, `multiplicar()`, `dividir()`, `sqrt()`, `sin()`, `cos()`, `tan()`, `atan2()`, `deg2rad()`, `rad2deg()` |
| `arrays` | `listas` | `length()`, `push()`, `pop()`, `get()`, `set()` |
| `strings` | `cadenas` | `length()`, `upper()`, `lower()`, `concat()`, `trim()`, `substr()`, `replace()`, `split()` |
| `io` | - | `print()`, `read()`, `file_exists()`, `mkdir()`, `remove()` |
| `random` | `aleatorio` | `int()`, `float()`, `choice()` |
| `time` | `tiempo` | `now()`, `sleep()` |
| `json` | - | `parse()`, `stringify()` |
| `colisiones` | - | `circulo_circulo()`, `circulo_rect()`, `rect_rect()` |

---

## 🎯 EJEMPLOS COMPLETOS

### 1. Juego Simple (Círculo que Sigue al Mouse)

```rydit
shield.init

dark.slot circulo_x = 400
dark.slot circulo_y = 300

ryda frame < 10000 {
    # Limpiar
    draw.rect(0, 0, 800, 600, "negro")
    
    # Obtener mouse
    dark.slot mx = input::mouse_x()
    dark.slot my = input::mouse_y()
    
    # Mover hacia el mouse
    onif mx > circulo_x {
        dark.slot circulo_x = circulo_x + 2
    }
    onif mx < circulo_x {
        dark.slot circulo_x = circulo_x - 2
    }
    onif my > circulo_y {
        dark.slot circulo_y = circulo_y + 2
    }
    onif my < circulo_y {
        dark.slot circulo_y = circulo_y - 2
    }
    
    # Dibujar
    draw.circle(circulo_x, circulo_y, 20, "rojo")
    draw.text("Sigue al mouse", 300, 50, "blanco")
}
```

### 2. Tanque con Sprites

```rydit
shield.init

# Cargar assets (UNA VEZ, fuera del loop)
assets::load_texture("tank", "sprites/tank_16x16.png")

dark.slot tank_x = 100
dark.slot tank_y = 300

ryda frame < 5000 {
    # Limpiar
    draw.rect(0, 0, 800, 600, "negro")
    
    # Input
    onif tecla_presionada("w") { dark.slot tank_y = tank_y - 2 }
    onif tecla_presionada("s") { dark.slot tank_y = tank_y + 2 }
    onif tecla_presionada("a") { dark.slot tank_x = tank_x - 2 }
    onif tecla_presionada("d") { dark.slot tank_x = tank_x + 2 }
    
    # Dibujar sprite escalado 4x
    assets::draw_scaled("tank", tank_x, tank_y, 4, "blanco")
    
    # Info
    draw.text("Tank: WASD para mover", 10, 10, "blanco")
    draw.text("Pos: (" + tank_x + ", " + tank_y + ")", 10, 30, "amarillo")
}
```

### 3. Sistema de Partículas Simple

```rydit
shield.init
import random

dark.slot particulas_x = []
dark.slot particulas_y = []
dark.slot particulas_vx = []
dark.slot particulas_vy = []

# Crear explosión
dark.slot i = 0
ryda i < 50 {
    arrays::push(particulas_x, 400)
    arrays::push(particulas_y, 300)
    arrays::push(particulas_vx, random::int(-5, 5))
    arrays::push(particulas_vy, random::int(-5, 5))
    dark.slot i = i + 1
}

ryda frame < 200 {
    draw.rect(0, 0, 800, 600, "negro")
    
    # Actualizar y dibujar
    dark.slot j = 0
    dark.slot total = arrays::length(particulas_x)
    ryda j < total {
        dark.slot x = arrays::get(particulas_x, j)
        dark.slot y = arrays::get(particulas_y, j)
        dark.slot vx = arrays::get(particulas_vx, j)
        dark.slot vy = arrays::get(particulas_vy, j)
        
        # Mover
        arrays::set(particulas_x, j, x + vx)
        arrays::set(particulas_y, j, y + vy)
        
        # Dibujar
        draw.circle(x, y, 3, "naranja")
        
        dark.slot j = j + 1
    }
}
```

---

## 🛡️ COMANDOS DE EJECUCIÓN

### Modo Gráfico (gfx)

```bash
# Con variables de entorno (Termux-X11)
DISPLAY=:0 MESA_LOADER_DRIVER_OVERRIDE=zink DRI3=1 \
    ./target/release/rydit-rs --gfx demo.rydit

# Sin variables (Linux desktop)
./target/release/rydit-rs --gfx demo.rydit
```

### Modo Migui (GUI)

```bash
./target/release/rydit-rs --migui demo_migui.rydit
```

### Modo Comandante (REPL)

```bash
# Modo interactivo
./target/release/rydit-rs --repl

# Script directo
./target/release/rydit-rs "dark.slot x = 100"
```

---

## 🐛 SOLUCIÓN DE PROBLEMAS

### Pantalla Negra

```bash
# Verificar variables de entorno
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

# Ejecutar de nuevo
./target/release/rydit-rs --gfx demo.rydit
```

### "No se puede abrir la ventana"

```bash
# Verificar que Termux-X11 esté corriendo
# Reiniciar Termux-X11 si es necesario
```

### "assets::load_texture() no funciona"

```rydit
# ERROR: Cargar dentro del game loop
ryda frame < 1000 {
    assets::load_texture("tank", "path.png")  # ❌
}

# CORRECTO: Cargar fuera del game loop
assets::load_texture("tank", "path.png")  # ✅

ryda frame < 1000 {
    assets::draw("tank", x, y)  # ✅
}
```

### Errores de Sintaxis

```rydit
# ERROR: String sin cerrar
dark.slot mensaje = "hola  # ❌

# CORRECTO
dark.slot mensaje = "hola"  # ✅

# ERROR: Paréntesis sin cerrar
dark.slot x = (2 + 3  # ❌

# CORRECTO
dark.slot x = (2 + 3)  # ✅
```

---

## 📁 ESTRUCTURA DE PROYECTO

```
mi_juego/
├── juego.rydit          # Script principal
├── sprites/             # Imágenes PNG
│   ├── tank_16x16.png
│   └── heli_16x16.png
└── niveles/             # Scripts de niveles
    ├── nivel1.rydit
    └── nivel2.rydit
```

---

## 🎯 RECURSOS ADICIONALES

### Documentación
- `README.md` - Documentación principal
- `CHANGELOG_v0.5.1.md` - Cambios de versión
- `SOLUCION_RENDERIZADO_TERMUX_X11_V0.5.1.md` - Fix de renderizado
- `BACKUP_INSTRUCCIONES_V0.5.1.md` - Guía de backup

### Demos de Ejemplo
- `demos/demo_assets_v0.5.1.rydit` - Sprites
- `demos/tank_combat.rydit` - Tanques
- `demos/snake.rydit` - Snake Game
- `demos/demo_migui_backend.rydit` - GUI

### Comunidad
- Discord Mouredev: https://discord.gg/mouredev
- GitHub: https://github.com/lapumlbb18-blip/Rydit_Engine

---

<div align="center">

## 🛡️ **RyDit v0.5.1 - Guía de Usuario**

**"De cero a juego en 5 minutos"**

---

*Versión:* v0.5.1 ✅
*Sprites:* ✅
*60 FPS:* ✅
*Documentación:* Completa ✅

[⬆️ Volver arriba](#-guía-de-usuario-rydit-v051)

</div>
