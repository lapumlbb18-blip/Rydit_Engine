# 🛡️ RyDit Language

**Scripting para Juegos y Escenas Interactivas**

[![Tests](https://img.shields.io/badge/tests-110%20passing-brightgreen)]()
[![Platform](https://img.shields.io/badge/platform-Android%2FLinux%2FWindows-blue)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()

---

## 🎯 ¿Qué es RyDit?

**RyDit** es un lenguaje de scripting diseñado para crear juegos y escenas interactivas de manera sencilla y expresiva.

```rydit
# Ejemplo simple
shield.init

dark.slot mensaje = "Hola RyDit"
voz mensaje

# Gráficos
draw.circle(400, 300, 50, "rojo")
draw.rect(100, 100, 100, 100, "verde")
```

---

## 🎮 Demos en Ejecución

### Snake Game
![Snake Gameplay](screenshots/04_snake_gameplay.jpg)
*Snake Game completo con game loop, colisiones y puntuación*

### Game Over Screen
![Snake Game Over](screenshots/05_snake_gameover.jpg)
*Pantalla de Game Over con restart*

### Demo Gráfica
![Demo rydit-gfx](screenshots/02_demo_rydit_gfx_completo.jpg)
*Círculo rojo animado, rectángulo verde, línea azul - 60 FPS*

### Demo Shapes
![Demo Shapes](screenshots/03_demo_shapes_circulos.jpg)
*Círculos concéntricos animados y formas geométricas*

---

## ✨ Features Principales

### Sintaxis Expresiva
```rydit
# Variables con símbolos
dark.slot $precio = 99.99
dark.slot @usuario = "alucard18"
dark.slot %porcentaje = 50

# Concatenación automática
voz "Precio: $" + $precio  # Precio: $99.99

# Arrays y indexación
dark.slot lista = [1, 2, 3]
dark.slot lista[0] = 10

# Módulos importables
import random
dark.slot numero = random::int(1, 100)
```

### Gráficos Integrados
```rydit
shield.init

# Game loop
ryda true {
    draw.circle(400, 300, 50, "rojo")
    draw.rect(100, 100, 100, 100, "verde")
    draw.line(0, 0, 800, 600, "azul")
    draw.text("RyDit v0.1.9", 300, 50, 30, "blanco")
}
```

### Sistema de Módulos
```rydit
import random
import arrays
import strings
import io
import time

# Uso
dark.slot numero = random::int(1, 10)
dark.slot texto = strings::upper("hola")
dark.slot longitud = arrays::length([1, 2, 3])
```

---

## 🚀 Quick Start

### Requisitos
- Rust (opcional, para compilar)
- raylib 5.5 (para gráficos)
- Android/Termux O Linux O Windows

### En Android/Termux
```bash
# Instalar dependencias
pkg install rust raylib

# Clonar repositorio
git clone https://github.com/lapumlbb18-blip/my_app.git
cd my_app

# Compilar (si tienes Rust)
cargo build --release

# Ejecutar demo
./target/release/rydit-rs --gfx demos/snake_perfect.rydit
```

### En Linux (Ubuntu/Debian)
```bash
# Instalar dependencias
sudo apt install rustc cargo libraylib-dev

# Clonar repositorio
git clone https://github.com/lapumlbb18-blip/my_app.git
cd my_app

# Compilar
cargo build --release

# Ejecutar demo
./target/release/rydit-rs --gfx demos/snake_perfect.rydit
```

### En Windows
```powershell
# Instalar Rust desde rustup.rs
# Instalar raylib desde vcpkg o build manual

# Clonar repositorio
git clone https://github.com/lapumlbb18-blip/my_app.git
cd my_app

# Compilar
cargo build --release

# Ejecutar demo
.\target\release\rydit-rs.exe --gfx demos\snake_perfect.rydit
```

---

## 📊 Métricas

| Métrica | Valor |
|---------|-------|
| **Tests** | 110 passing |
| **Warnings** | 0 |
| **Errors** | 0 |
| **Demos** | 8 funcionales |
| **FPS** | 60 (vsync) |
| **Módulos** | 6 (random, arrays, strings, io, time, json) |

---

## 📁 Estructura del Proyecto

```
my_app/
├── README.md              # Este archivo
├── LICENSE                # Licencia MIT
├── CONTRIBUTING.md        # Cómo contribuir
│
├── demos/                 # Demos y ejemplos
│   ├── snake_perfect.rydit
│   ├── demo_shapes.rydit
│   └── ejemplo_gfx.rydit
│
├── crates/modules/        # Módulos stdlib
│   ├── random.rydit
│   ├── arrays.rydit
│   ├── strings.rydit
│   ├── io.rydit
│   ├── time.rydit
│   └── json.rydit
│
├── screenshots/           # Capturas de pantalla
│   ├── 01_demo_rydit_gfx_menu.jpg
│   ├── 02_demo_rydit_gfx_completo.jpg
│   ├── 03_demo_shapes_circulos.jpg
│   ├── 04_snake_gameplay.jpg
│   └── 05_snake_gameover.jpg
│
├── scripts/               # Scripts de utilidad
│   ├── test_demos_x11.sh
│   └── jugar_snake.sh
│
└── docs/                  # Documentación
    ├── GUIA_RAPIDA.md
    └── FEATURES.md
```

---

## 🎯 Casos de Uso

### 1. Prototipado Rápido de Juegos
```rydit
shield.init

dark.slot jugador_x = 400
dark.slot jugador_y = 300

ryda true {
    # Input
    onif tecla_presionada("arrow_right") {
        dark.slot jugador_x = jugador_x + 5
    }
    
    # Draw
    draw.circle(jugador_x, jugador_y, 20, "azul")
}
```

### 2. Visualización de Datos
```rydit
shield.init

dark.slot datos = [10, 25, 15, 30, 45]
dark.slot x = 50

ryda i < arrays::length(datos) {
    dark.slot alto = datos[i] * 5
    draw.rect(x, 500 - alto, 40, alto, "verde")
    dark.slot x = x + 50
    dark.slot i = i + 1
}
```

### 3. Animaciones Simples
```rydit
shield.init

dark.slot frame = 0

ryda true {
    dark.slot radio = 50 + (frame % 20)
    draw.circle(400, 300, radio, "rojo")
    dark.slot frame = frame + 1
}
```

---

## 🛠️ Desarrollo

### Compilar desde Fuente
```bash
# Asegurar tener Rust y raylib instalados
cargo build --release

# Verificar tests
cargo test

# Ejecutar en modo gráfico
cargo run -- --gfx demos/snake_perfect.rydit
```

### Ejecutar Tests
```bash
# Todos los tests
cargo test

# Tests de un crate específico
cargo test -p lizer

# Tests con output detallado
cargo test -- --nocapture
```

---

## 📚 Documentación

- [Guía Rápida](docs/GUIA_RAPIDA.md) - Inicio rápido
- [Features](docs/FEATURES.md) - Lista completa de features
- [Contributing](CONTRIBUTING.md) - Cómo contribuir al proyecto

---

## 🤝 Contribuir

¡Las contribuciones son bienvenidas!

### Áreas donde puedes ayudar:
- 📝 Documentación
- 🧪 Tests adicionales
- 🎨 Nuevos demos y ejemplos
- 🌍 Traducciones
- 🐛 Reporte de bugs

Ver [CONTRIBUTING.md](CONTRIBUTING.md) para más detalles.

---

## 📄 Licencia

Este proyecto está bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para detalles.

### Uso
- ✅ Uso personal permitido
- ✅ Uso educativo permitido
- ✅ Modificación permitida
- ⚠️ Uso comercial requiere notificación

---

## 🙏 Agradecimientos

- [raylib](https://www.raylib.com/) - Motor gráfico
- [Rust](https://www.rust-lang.org/) - Lenguaje base
- [Termux](https://termux.dev/) - Entorno en Android

---

## 📬 Contacto

**Repositorio:** [github.com/lapumlbb18-blip/my_app](https://github.com/lapumlbb18-blip/my_app)

**Demo en vivo:**
```bash
git clone https://github.com/lapumlbb18-blip/my_app.git
cd my_app
cargo run -- --gfx demos/snake_perfect.rydit
```

---

<p align="center">
  <strong>Construido con ❤️ en Android/Termux</strong><br>
  <em>100% mobile development - No laptop used</em>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Built%20on-Android%2FTermux-blue?style=for-the-badge" alt="Built on Android/Termux">
</p>
