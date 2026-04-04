# 🛡️ MANIFIESTO RYDIT

## ¿Qué es RyDit?

**RyDit** es un motor de videojuegos 2D con lenguaje de scripting propio, escrito en Rust, diseñado para desarrollarse **nativamente en Android/Termux** sin necesidad de desktop, emuladores o IDEs pesados.

```rydit
# Tu primer juego en 3 líneas
shield.init
ryda frame < 1000 {
    draw.circle(400, 300, 50, "rojo")
}
```

---

## 🎯 Filosofía

### 1. **Mobile-First Real**

No "también funciona en Android". **Nació en Android**.

- Desarrollado completamente en un Redmi Note 8 con Termux
- Sin dependencia de Windows/Mac/Linux para desarrollar
- Binarios de ~550 KB (no 50 MB como Godot)
- Sin IDE: tu editor de texto favorito es suficiente

### 2. **Ligero y Portable**

| Motor | Binario | RAM mínima | Plataforma |
|-------|---------|------------|------------|
| **RyDit** | ~550 KB | 512 MB | Android nativo |
| Godot | ~50 MB | 2 GB | Desktop |
| Unity | ~200 MB | 4 GB | Desktop |
| Love2D | ~10 MB | 1 GB | Desktop |

### 3. **Educativo**

- Código 100% abierto y legible
- Lenguaje en español (accesible para hispanohablantes)
- Sin magia: ves exactamente qué hace cada línea
- Perfecto para aprender:
  - Programación de videojuegos
  - Rust desde un lenguaje sencillo
  - Gráficos 2D y game loops

### 4. **David vs Goliat**

Demostramos que **no necesitas herramientas pesadas** para crear videojuegos:

- 12,000 líneas de Rust bien escritas > 500,000 líneas de C++
- 1 persona en Android > equipos de 50 en oficinas
- 6 meses de desarrollo > 5 años de motor comercial

### 5. **Rendimiento Estable - Hardware Primero** 🔥

**Propósito Fundamental:** RyDit prioriza el rendimiento estable sobre features innecesarios.

- **Sin calentamiento:** Optimizado para ejecutarse sin sobrecargar el CPU/GPU
- **RAM consciente:** Binario ~550 KB, uso de RAM <100 MB en runtime
- **60 FPS estables:** Game loop optimizado, sin garbage collection
- **Hardware modesto:** Diseñado para Redmi Note 8 (gama baja) → funciona en cualquier lado
- **Sin sobrecargas:** Cada feature se pregunta: ¿es esencial o es bloat?

**Filosofía de Optimización:**
```
Mejor 10 features que funcionen perfecto a 100 que calienten tu dispositivo
```

**Métricas de Rendimiento:**
| Métrica | RyDit v0.6.2 | Godot | Unity |
|---------|--------------|-------|-------|
| Binario | ~580 KB | ~50 MB | ~200 MB |
| RAM runtime | <100 MB | ~500 MB | ~1 GB |
| CPU usage | ~20-30% | ~60-80% | ~80-100% |
| Calentamiento | Mínimo | Moderado | Alto |

### 6. **Portabilidad Cruzada** 🌍

**Próximamente:** RyDit no solo es Android.

- **Linux:** Nativo (x86_64, ARM64)
- **Windows:** En desarrollo (x64)
- **WebAssembly:** Futuro (v0.8.0+)
- **iOS:** Considerando (limitaciones de Apple)

**Una código, múltiples plataformas:**
```rust
// Mismo código Rust → Todos los targets
cargo build --release        # Android/Linux
cargo build --target x86_64-pc-windows-msvc  # Windows
```

---

## 🔥 ¿Por qué existe RyDit?

### El Problema

1. **Barrera de entrada alta**: Godot/Unity requieren PC potente
2. **Android es ciudadano de segunda**: "Funciona en Android" ≠ "Nació en Android"
3. **Herramientas sobredimensionadas**: 90% de features que no usas
4. **Inglés como requisito**: Documentación y sintaxis solo en inglés
5. **Dependencia de IDE**: Sin VS Code/Visual Studio estás perdido

### La Solución RyDit

✅ **Termux como plataforma primera** - No emulación, nativo
✅ **Binario <1 MB** - Cabe en cualquier lado
✅ **12 widgets, 8 módulos stdlib** - Lo justo y necesario
✅ **Español nativo** - `draw.circulo()`, `si`, `mientras`
✅ **Sin IDE** - `cargo run` y listo

---

## 🚀 Diferencias con Otros Motores

### RyDit vs Godot

| Característica | RyDit | Godot |
|---------------|-------|-------|
| Plataforma | Android nativo | Desktop |
| Lenguaje | RyDit (Español) | GDScript (Inglés) |
| Binario | 550 KB | 50 MB |
| IDE | Opcional | Requerido |
| Curva aprendizaje | 1 día | 2 semanas |
| 3D | ❌ No | ✅ Sí |
| 2D | ✅ Especializado | ✅ Sí |

**Cuándo usar RyDit**: Juegos 2D simples, aprender, prototipado rápido
**Cuándo usar Godot**: Juegos 3D, proyectos comerciales grandes

### RyDit vs Love2D

| Característica | RyDit | Love2D |
|---------------|-------|--------|
| Backend | Rust | C |
| Platforma | Android | Desktop |
| Sintaxis | Español | Lua (Inglés) |
| Type safety | ✅ Parcial | ❌ Dinámico |

### RyDit vs PICO-8

| Característica | RyDit | PICO-8 |
|---------------|-------|--------|
| Licencia | MIT (gratis) | $15 USD |
| Platforma | Android | Desktop/Switch |
| Límites | ❌ No | ✅ Sí (64x64, 4 colores) |
| Lenguaje | RyDit | Lua |

---

## 📊 Estado Actual (v0.6.0)

### ✅ Completado

- [x] Lenguaje de scripting completo
- [x] 126 tests automáticos
- [x] 12 widgets UI (migui)
- [x] 8 módulos stdlib (math, arrays, strings, io, random, time, json, colisiones)
- [x] Sistema de partículas (fuego, humo, explosión, lluvia, chispas)
- [x] Audio (sonidos + música)
- [x] REPL interactivo
- [x] Stdlib embebido (sin archivos externos)
- [x] Fix automático Termux-X11
- [x] Snake Game completo como demo
- [x] Binario optimizado ~550 KB

### 🔜 Roadmap Público

#### v0.6.1 - Git + CI/CD (Esta sesión)
- [ ] Push completo v0.5.0 → v0.6.0 a GitHub
- [ ] MANIFIESTO.md + DEUDA_TECNICA.md
- [ ] GitHub Actions básico (Linux)
- [ ] README actualizado con capturas v0.6.0

#### v0.6.2 - Regex + Módulos (Próxima)
- [ ] Módulo `regex` (match, replace, split)
- [ ] Módulo `files` (lectura/escritura archivos)
- [ ] 130+ tests

#### v0.7.0 - Animaciones 2D
- [ ] Sprite sheets (anim::load, anim::play)
- [ ] Easing functions (in_quad, out_quad, bounce, elastic)
- [ ] 12 principios de animación (4 básicos)
- [ ] Demo tanque animado
- [ ] Refactor main.rs (split en módulos)

#### v0.8.0 - Editor Visual
- [ ] Editor de escenas con migui
- [ ] Inspector de propiedades
- [ ] Preview en tiempo real
- [ ] Exportar a .rydit

#### v0.9.0 - Cross-Platform
- [ ] Builds Windows (GitHub Actions)
- [ ] Builds Linux ARM64
- [ ] Instalador automático
- [ ] Documentación completa

#### v1.0.0 - Release Estable
- [ ] 200+ tests
- [ ] 15+ demos funcionales
- [ ] Binario <600 KB
- [ ] 0 warnings
- [ ] CI/CD completo
- [ ] Comunidad activa

---

## 💬 Valores

1. **Código > Burocracia** - Preferimos código funcionando a documentación perfecta
2. **Mobile-First** - Si no funciona en Android, no es RyDit
3. **Ligero** - Cada KB cuenta. Sin dependencias innecesarias
4. **Abierto** - MIT license. 100% transparente
5. **Educativo** - Enseñamos, no solo damos herramientas
6. **Comunidad** - Hisparohablantes, mobile developers, hobbyists

---

## 🤝 Cómo Contribuir

### 1. Reporta Bugs
GitHub Issues: https://github.com/lapumlbb18-blip/Ry-Dit/issues

### 2. Propón Features
Discute primero en Issues. Priorizamos:
- Lo que USAS en tus proyectos
- Lo que MEJORA rendimiento/tamaño
- Lo que HACE más accesible el lenguaje

### 3. Envía PRs
```bash
git clone https://github.com/lapumlbb18-blip/Ry-Dit
cd Ry-Dit
# Crea tu rama
git checkout -b feature/tu-feature
# Haz cambios
cargo test  # Asegura 0 fallos
cargo build  # Asegura 0 warnings
git commit -m "feature: descripción clara"
git push origin feature/tu-feature
```

### 4. Difunde
- ⭐ Star en GitHub
- 📱 Comparte en redes
- 🎮 Muestra tus juegos
- 📝 Escribe tutoriales

---

## 📞 Comunidad

- **GitHub**: https://github.com/lapumlbb18-blip/Ry-Dit
- **Issues**: https://github.com/lapumlbb18-blip/Ry-Dit/issues
- **Discord**: (próximamente)
- **Twitter**: (próximamente)

---

## 🎮 Juegos Hechos con RyDit

### Snake Game (v0.1.8)
- 60 FPS estables
- Colisiones precisas
- Puntuación + high score
- Velocidad progresiva

### Tank Combat (v0.3.0)
- Tanque con movimiento WASD
- Cañón rotando con mouse
- Disparos con partículas
- Enemigos con IA básica

### Demo Partículas (v0.5.3)
- 4 efectos: fuego, humo, explosión, chispas
- 60 FPS con 500+ partículas
- Controles en tiempo real

---

## 📈 Métricas del Proyecto

| Métrica | Valor |
|---------|-------|
| Líneas Rust | ~12,000 |
| Tests | 126 |
| Widgets UI | 12 |
| Módulos stdlib | 8 |
| Binario release | ~550 KB |
| Binario debug | ~920 KB |
| Dependencias | 6 crates |
| Demos | 19 principales |
| FPS target | 60 |

---

## 🙏 Agradecimientos

- **raylib** - Backend gráfico (Zlib license)
- **serde/serde_json** - JSON parsing (MIT)
- **Termux** - Linux en Android
- **Rust** - El lenguaje que hace todo posible

---

## 📜 Licencia

**MIT License** - Haz lo que quieras con esto.

```
Copyright (c) 2026 Ry-Dit

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
```

---

<div align="center">

**🛡️ RyDit v0.6.0 - Construido en Android, para Android**

[GitHub](https://github.com/lapumlbb18-blip/Ry-Dit) • [Roadmap](#-roadmap-público) • [Demos](#-juegos-hechos-con-rydit)

</div>
