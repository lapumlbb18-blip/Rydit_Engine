# 🛡️ RyDit Language

**Version:** v0.1.0 (Release Alpha)  
**Date:** 2026-03-17  
**Session:** 15 - Snake Game Complete + Release Alpha  
**Status:** ✅ **60 TESTS - 0 WARNINGS - SNAKE FUNCTIONAL**

[![Tests](https://img.shields.io/badge/tests-60%20passing-brightgreen)]()
[![Warnings](https://img.shields.io/badge/warnings-0-brightgreen)]()
[![Platform](https://img.shields.io/badge/platform-Android%2FTermux-blue)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()

---

## 🎯 What is RyDit?

**EN:** A scripting language built **entirely on mobile** (Android/Termux) with native graphics integration and a fully playable Snake game demo.

**ES:** Un lenguaje de scripting construido **completamente en móvil** (Android/Termux) con integración gráfica nativa y una demo completamente jugable del Snake game.

---

## 📊 Quick Stats / Estadísticas Rápidas

| Metric / Métrica | Value / Valor |
|-----------------|---------------|
| **Version / Versión** | v0.1.0 (Release Alpha) |
| **Tests** | ✅ 60 passing / pasando |
| **Warnings** | ✅ 0 |
| **Build Time (cached)** | ⚡ ~1.2s |
| **Lines of Code** | 📝 5,258 (4,021 Rust + 1,237 RyDit) |
| **Crates** | 🏗️ 5 (lizer, blast-core, rydit-gfx, rydit-rs, v-shield) |
| **Development Time** | ⏱️ 4 days / 4 días |
| **Platform** | 📱 Android/Termux |

---

## 🚀 Features / Características

### EN:
- ✅ Variables and scopes
- ✅ Functions with return values
- ✅ **Function composition** `f1(f2(x))`
- ✅ Control flow (if/while/for)
- ✅ Arrays and indexing
- ✅ Arithmetic operations + parentheses
- ✅ Built-in functions (sum, subtract, multiply, divide)
- ✅ Graphics (circles, rectangles, lines, text)
- ✅ Input handling (keyboard)
- ✅ **Snake Game** - Fully playable demo

### ES:
- ✅ Variables y scopes
- ✅ Funciones con retorno
- ✅ **Composición de funciones** `f1(f2(x))`
- ✅ Control de flujo (if/while/for)
- ✅ Arrays e indexación
- ✅ Operaciones aritméticas + paréntesis
- ✅ Funciones builtin (suma, resta, multiplicación, división)
- ✅ Gráficos (círculos, rectángulos, líneas, texto)
- ✅ Manejo de input (teclado)
- ✅ **Snake Game** - Demo completamente jugable

---

## 🎮 Snake Game Demo

**Fully playable Snake game built with RyDit!**

![Snake Gameplay](screenshots/snake-gameplay.png)
![Game Over](screenshots/snake-gameover.png)

**Run / Ejecutar:**
```bash
# With dedicated binary / Con binario dedicado
cargo run --bin snake

# Or with .rydit script / O con script .rydit
cargo run -- --gfx snake_limpio.rydit
```

**Features Demonstrated / Features Demostradas:**
- ✅ Arrays and indexing / Arrays e indexación
- ✅ Functions with return values / Funciones con retorno
- ✅ Function composition / Composición de funciones
- ✅ Game loop with input / Game loop con input
- ✅ Collision detection / Detección de colisiones
- ✅ Scoring system / Sistema de puntuación
- ✅ Restart without recompiling / Restart sin recompilar

**Controls / Controles:**
| Key / Tecla | Action / Acción |
|-------------|-----------------|
| `↑` | Move up / Mover arriba |
| `→` | Move right / Mover derecha |
| `↓` | Move down / Mover abajo |
| `←` | Move left / Mover izquierda |
| `SPACE` | Restart (Game Over) / Reiniciar |
| `ESC` | Exit / Salir |

---

## 📱 Mobile Development Story / Historia de Desarrollo Móvil

### EN:
**This project was built entirely on an Android device using Termux.**

No laptop. No desktop. No IDE. Just:
- 📱 Android phone
- ⌨️ Termux terminal
- 🦀 Rust + Cargo
- 🎨 Raylib (native)

**Why?** To prove that serious development is possible on mobile devices when you have:
- Clear architecture
- Automated tests
- Good documentation
- Determination

### ES:
**Este proyecto fue construido completamente en un dispositivo Android usando Termux.**

Sin laptop. Sin escritorio. Sin IDE. Solo:
- 📱 Teléfono Android
- ⌨️ Terminal Termux
- 🦀 Rust + Cargo
- 🎨 Raylib (nativo)

**¿Por qué?** Para demostrar que el desarrollo serio es posible en dispositivos móviles cuando tienes:
- Arquitectura clara
- Tests automatizados
- Buena documentación
- Determinación

---

## 🛠️ Installation / Instalación

### EN:
```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/rydit-language.git
cd rydit-language

# Build
cargo build

# Run tests
cargo test

# Run Snake game
cargo run --bin snake
```

### ES:
```bash
# Clonar el repositorio
git clone https://github.com/TU_USUARIO/rydit-language.git
cd rydit-language

# Compilar
cargo build

# Ejecutar tests
cargo test

# Ejecutar Snake game
cargo run --bin snake
```

---

## 📖 Documentation / Documentación

| Document / Documento | Description / Descripción |
|---------------------|--------------------------|
| **[LIBRO_RYDIT.md](LIBRO_RYDIT.md)** | Complete language guide / Guía completa del lenguaje (~400 líneas) |
| **[BENCHMARK_v0.1.0.md](BENCHMARK_v0.1.0.md)** | Performance metrics / Métricas de rendimiento |
| **[CONTRIBUTING.md](CONTRIBUTING.md)** | How to contribute / Cómo contribuir |
| **[FLUJO_TRABAJO.md](FLUJO_TRABAJO.md)** | Recommended workflow / Flujo de trabajo recomendado |
| **[diagnostico/](diagnostico/)** | Session logs / Logs de sesiones (24 archivos) |

---

## 📈 Roadmap / Hoja de Ruta

| Version | Feature / Feature | ETA |
|---------|------------------|-----|
| **v0.1.0** | Snake game + Release Alpha | ✅ DONE |
| **v0.1.1** | Module system (import) / Sistema de módulos | 2-3 weeks |
| **v0.1.2** | Standard library basics / Librería estándar básica | 1 month |
| **v0.1.3** | Optional type system / Sistema de tipos opcional | 2-3 months |
| **v0.2.0** | Higher-order functions / Funciones de orden superior | 3-4 months |
| **v1.0.0** | Standard library + docs / Librería estándar + docs | 6 months |

---

## 🧪 Testing / Tests

```bash
# Run all tests / Ejecutar todos los tests
cargo test

# Expected output / Resultado esperado:
# blast-core:  18 tests ✅
# lizer:       35 tests ✅
# rydit-gfx:    3 tests ✅
# rydit-rs:     2 tests ✅
# v-shield:     1 test  ✅
# doc-tests:    1 test  ✅
# ────────────────────────────
# TOTAL:       60 tests ✅
```

---

## 🧪 Usage Examples / Ejemplos de Uso

### EN: Basic Functions
```rydit
rytmo greet {
    voz "Hello World"
    return 1
}

greet()
```

### ES: Funciones Básicas
```rydit
rytmo saludar {
    voz "Hola Mundo"
    return 1
}

saludar()
```

### EN: Functions with Parameters
```rydit
rytmo greet(name) {
    voz "Hello " + name
}

greet("World")
```

### ES: Funciones con Parámetros
```rydit
rytmo saludar(nombre) {
    voz "Hola " + nombre
}

saludar("Mundo")
```

### EN: Function Composition
```rydit
rytmo sum(a, b) { return a + b }
rytmo square(x) { return x * x }

# Function composition
dark.slot x = square(sum(2, 3))
voz x  # 25 (2+3=5, 5*5=25)
```

### ES: Composición de Funciones
```rydit
rytmo sumar(a, b) { return a + b }
rytmo cuadrado(x) { return x * x }

# Composición de funciones
dark.slot x = cuadrado(sumar(2, 3))
voz x  # 25 (2+3=5, 5*5=25)
```

### EN: Arrays
```rydit
# Basic array
dark.slot lista = [1, 2, 3]

# Multidimensional (board)
dark.slot tablero = [[0, 0, 0], [0, 0, 0], [0, 0, 0]]

# With expressions
dark.slot suma = [1 + 2, 3 * 4, 10 / 2]
```

### ES: Arrays
```rydit
# Array básico
dark.slot lista = [1, 2, 3]

# Multidimensional (tablero)
dark.slot tablero = [[0, 0, 0], [0, 0, 0], [0, 0, 0]]

# Con expresiones
dark.slot suma = [1 + 2, 3 * 4, 10 / 2]
```

### EN: Graphics (Window Mode)
```rydit
shield.init

# Draw shapes
draw.circle(400, 300, 50, "red")
draw.rect(100, 100, 100, 100, "green")
draw.line(0, 0, 800, 600, "blue")
draw.text("RyDit v0.1.0", 300, 50, 30, "white")
```

### ES: Gráficos (Modo Ventana)
```rydit
shield.init

# Dibujar formas
draw.circle(400, 300, 50, "rojo")
draw.rect(100, 100, 100, 100, "verde")
draw.line(0, 0, 800, 600, "azul")
draw.text("RyDit v0.1.0", 300, 50, 30, "blanco")
```

---

## 🤝 Community / Comunidad

### EN:
**This is an experimental project built entirely on mobile.**

**Contributions welcome:**
- Bug reports
- Feature suggestions
- Documentation improvements
- Examples

### ES:
**Este es un proyecto experimental construido completamente en móvil.**

**Contribuciones bienvenidas:**
- Reportes de bugs
- Sugerencias de features
- Mejoras de documentación
- Ejemplos

---

## 💭 Honest Opinion / Opinión Honesta

### EN:
**What's impressive:**
- ⚡ 60 tests in 4 days
- 🧱 Solid architecture (5 crates)
- 📖 Complete documentation
- 🎯 Zero warnings policy
- 📱 Built entirely on mobile
- 🎮 Fully playable Snake game

**What's challenging:**
- 📚 No standard library yet
- 🔧 Limited ecosystem (no packages)
- 🎨 Graphics need more work
- 📖 English docs need improvement

**The truth:** This project proves that **constraints breed creativity**. Developing on mobile forced me to write better code, automate more, and document everything.

### ES:
**Lo que impresiona:**
- ⚡ 60 tests en 4 días
- 🧱 Arquitectura sólida (5 crates)
- 📖 Documentación completa
- 🎯 Política de cero warnings
- 📱 Construido completamente en móvil
- 🎮 Snake game completamente jugable

**Lo que es un desafío:**
- 📚 Sin librería estándar aún
- 🔧 Ecosistema limitado (sin paquetes)
- 🎨 Gráficos necesitan más trabajo
- 📖 Docs en inglés necesitan mejorar

**La verdad:** Este proyecto demuestra que **las limitaciones generan creatividad**. Desarrollar en móvil me forzó a escribir mejor código, automatizar más, y documentar todo.

---

## 📜 License / Licencia

[MIT License](LICENSE) - Feel free to use, learn, and build upon this.

---

## 🙏 Acknowledgments / Agradecimientos

### EN:
- **Rust community** - For the amazing compiler and error messages
- **Raylib** - For the simple and powerful graphics library
- **Termux** - For making Android development possible
- **You** - For reading this and considering contributing

### ES:
- **Comunidad Rust** - Por el increíble compilador y mensajes de error
- **Raylib** - Por la librería gráfica simple y poderosa
- **Termux** - Por hacer posible el desarrollo en Android
- **Tú** - Por leer esto y considerar contribuir

---

## 🚀 Final Words / Palabras Finales

### EN:
> "This project started as a learning exercise. It became proof that **mobile development is viable** for serious projects. The constraints of mobile (small screen, limited RAM, no mouse) forced me to write better code, automate more, and document everything. If you're developing on mobile too: **you're not alone, keep going**."

### ES:
> "Este proyecto comenzó como un ejercicio de aprendizaje. Se volvió prueba de que **el desarrollo en móvil es viable** para proyectos serios. Las limitaciones del móvil (pantalla pequeña, RAM limitada, sin mouse) me forzaron a escribir mejor código, automatizar más, y documentar todo. Si estás desarrollando en móvil también: **no estás solo, sigue avanzando**."

---

**Built with ❤️ on Android | Construido con ❤️ en Android**

*v0.1.0 - Release Alpha*
