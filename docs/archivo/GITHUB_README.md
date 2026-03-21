# 🛡️ RyDit Language - Experimental Phase

**Version:** v0.0.14 (Functions in Expressions)  
**Status:** 🧪 Experimental / Experimental  
**Platform:** Android/Termux 📱

---

## 🎯 What is RyDit? / ¿Qué es RyDit?

**EN:** A scripting language built **entirely on mobile** (Android/Termux) with native graphics integration.

**ES:** Un lenguaje de scripting construido **completamente en móvil** (Android/Termux) con integración gráfica nativa.

---

## 📊 Current Status / Estado Actual

| Metric / Métrica | Value / Valor |
|-----------------|---------------|
| **Version / Versión** | v0.0.14 |
| **Tests Passed** | ✅ 60/60 |
| **Warnings** | ✅ 0 |
| **Build Time (cached)** | ⚡ 0.3s |
| **Development Time** | 4 days / 4 días |
| **Lines of Code** | ~3,250+ |
| **Documentation** | ✅ LIBRO_RYDIT.md |

---

## 🚀 What Works Now / Qué Funciona Ahora

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

## 🎯 Purpose / Finalidad

### EN:
1. **Prove mobile-first scripting works** - You don't need a expensive laptop to create programming languages
2. **Optimize for low-RAM devices** - Built for <4GB RAM devices (common in developing countries)
3. **Learn compilers interpreters** - Educational project that became functional
4. **Android game development** - Simple games with raylib integration

### ES:
1. **Demostrar que scripting en móvil funciona** - No necesitas una laptop costosa para crear lenguajes de programación
2. **Optimizar para dispositivos con poca RAM** - Construido para dispositivos <4GB RAM (común en países en desarrollo)
3. **Aprender compiladores/intérpretes** - Proyecto educativo que se volvió funcional
4. **Desarrollo de juegos en Android** - Juegos simples con integración raylib

---

## 📈 Roadmap / Hoja de Ruta

| Version | Feature | ETA |
|---------|---------|-----|
| **v0.0.14** | Functions in expressions | ✅ DONE |
| **v0.1.0** | Snake game + Release Alpha | ✅ DONE |
| **v0.2.0** | Module system (import) | 1 month |
| **v0.3.0** | Optional type system | 2-3 months |
| **v1.0.0** | Standard library + docs | 6 months |

---

## 🎮 Snake Game Demo

### EN:
**Fully playable Snake game built with RyDit!**

![Snake Gameplay](screenshots/snake-gameplay.png)
![Game Over](screenshots/snake-gameover.png)

**Run:**
```bash
cargo run -- --gfx snake_completo.rydit
```

**Features Demonstrated:**
- ✅ Arrays and indexing
- ✅ Functions with return values
- ✅ Function composition
- ✅ Game loop with input
- ✅ Collision detection
- ✅ Scoring system
- ✅ Restart without recompiling

### ES:
**¡Juego Snake completamente jugable construido con RyDit!**

**Ejecutar:**
```bash
cargo run -- --gfx snake_completo.rydit
```

**Features Demoostradas:**
- ✅ Arrays e indexación
- ✅ Funciones con retorno
- ✅ Composición de funciones
- ✅ Game loop con input
- ✅ Detección de colisiones
- ✅ Sistema de puntuación
- ✅ Restart sin recompilar

---

## 🤝 Community / Comunidad

### EN:
**This is an experimental project.** Expect:
- 🐛 Bugs (but well-documented)
- 📝 Detailed session logs (diagnostico/)
- 🧪 60 automated tests (growing)
- 📚 Complete documentation (LIBRO_RYDIT.md)

**Contributions welcome:**
- Bug reports
- Feature suggestions
- Documentation improvements
- Examples

### ES:
**Este es un proyecto experimental.** Espera:
- 🐛 Bugs (pero bien documentados)
- 📝 Logs detallados de sesiones (diagnostico/)
- 🧪 60 tests automatizados (creciendo)
- 📚 Documentación completa (LIBRO_RYDIT.md)

**Contribuciones bienvenidas:**
- Reportes de bugs
- Sugerencias de features
- Mejoras de documentación
- Ejemplos

---

## 💭 Honest Opinion / Opinión Honesta

### EN:
**What's impressive:**
- ⚡ 14 versions in 4 days
- 🧱 Solid architecture (5 crates)
- 📖 Documentation for every session
- 🎯 Zero warnings policy
- 📱 Built entirely on mobile

**What's challenging:**
- 📚 No standard library yet
- 🔧 Limited ecosystem (no packages)
- 🎨 Graphics need more work
- 📖 English docs need improvement

**The truth:** This project proves that **constraints breed creativity**. Developing on mobile forced me to:
- Write cleaner code (less typing = more thinking)
- Automate everything (tests before features)
- Document religiously (future self will thank me)
- Optimize for performance (RAM is limited)

### ES:
**Lo que impresiona:**
- ⚡ 14 versiones en 4 días
- 🧱 Arquitectura sólida (5 crates)
- 📖 Documentación para cada sesión
- 🎯 Política de cero warnings
- 📱 Construido completamente en móvil

**Lo que es un desafío:**
- 📚 Sin librería estándar aún
- 🔧 Ecosistema limitado (sin paquetes)
- 🎨 Gráficos necesitan más trabajo
- 📖 Docs en inglés necesitan mejorar

**La verdad:** Este proyecto demuestra que **las limitaciones generan creatividad**. Desarrollar en móvil me forzó a:
- Escribir código más limpio (menos typing = más pensar)
- Automatizar todo (tests antes de features)
- Documentar religiosamente (mi yo futuro lo agradecerá)
- Optimizar para performance (la RAM es limitada)

---

## 📞 Contact / Contacto

**GitHub:** [Your GitHub]  
**Sessions:** Check `diagnostico/` folder for detailed logs  
**Documentation:** `LIBRO_RYDIT.md` (Spanish)

---

## 📜 License / Licencia

**MIT License** - Feel free to use, learn, and build upon this.

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

*v0.0.14 - Functions in Expressions / Funciones en Expresiones*
