# 📕 Post para Reddit (r/rust, r/gamedev, r/ProgrammerHumor)

## Título del post

**Escribí un motor de juegos completo desde un celular Android con Termux, Rust y una IA. Sin PC de escritorio.**

---

## Cuerpo del post

Hola. Esto no es un tutorial. Es una historia + código abierto.

### Contexto

No tengo PC de escritorio. No tengo equipo de desarrollo. No tengo inversión.

Tengo un celular Android con Termux, Rust, y una IA colaboradora (Qwen Code).

Con eso escribí **Ry-Dit**: un motor de creación de juegos, animaciones y emuladores.

### Lo que funciona hoy (v0.15.0)

| Cosa | Detalle |
|------|---------|
| **Crates** | 25 compilando, 0 errores |
| **Publicados** | 2 en crates.io (ry-god, ry-stream) |
| **Demos** | 8 funcionales en Termux-X11 |
| **GPU Instancing** | 50K partículas, 48 FPS, Adreno 610 |
| **FSR 1.0** | Upscale 960x540 → 1280x720, 48 FPS |
| **Demos** | Torreta vs sprites, platformer, rigidbody, anime, partículas 50K, panel visual |

### La filosofía: Low-End First

Mientras todos miran hacia arriba — más GPU, más nube, más dinero, más empleados — Ry-Dit mira hacia **abajo**.

- Unity asume que tienes una PC de $2000.
- Godot asume que tienes Linux instalado.
- Unreal asume que tienes un equipo.

Ry-Dit asume que tienes **un celular y ganas de crear**.

4 mil millones de personas tienen un celular. Casi ninguna tiene herramientas de creación reales. Ry-Dit existe para cerrar esa brecha.

### Por qué desde un celular

Porque es donde está la gente.

Un estudiante en Lagos, Bogotá, Manila, Nairobi — no tiene PC de escritorio. No tiene $300 para Unity Pro. No tiene equipo.

Pero tiene un celular, internet básico, y una idea.

Ry-Dit le da la herramienta que nadie más le da.

### Cómo funciona técnicamente

- **Rust** como lenguaje (memory safety + zero-cost abstractions)
- **SDL2 + OpenGL directo** para GPU Instancing (sin Canvas, sin abstracciones que matan el contexto GL)
- **Raylib** para dibujo nativo (círculos, líneas, texto)
- **FSR 1.0** con FBO render-to-texture para upscaling
- **Zink/Vulkan** para OpenGL sobre Adreno 610 en Android
- **Termux-X11** para display gráfico

Todo corre **local**. Sin nube. Sin APIs externas. Sin vendor lock-in.

### El bug más loco que encontré

GPU Instancing mostraba "barras de colores" en vez de círculos. Después de 8 fixes encontrados:

1. `instance_vbo` no bindeado antes de atributos de instancia
2. Stride del VAO: 8 bytes (mal) → 16 bytes (bien)
3. `QUADS` no existe en OpenGL Core Profile → cambiado a TRIANGLES
4. `gl_PointCoord` solo funciona con `gl_POINTS` → cambiado a varying
5. `glViewport` no configurado cada frame
6. `glScissorTest` cortando bordes

El GPU Instancer de Unity tardó 15 años en llegar. El nuestro llegó en la versión 0.15.0. Porque somos 1 humano + 1 IA sin burocracia.

### GitHub

Todo abierto. Forkable. Mejorable.

🔗 https://github.com/lapumlbb18-blip/Ry-dit

### Mi mensaje

Hice mi parte. Un motor que cabe en un celular, escrito por un humano solo con una IA como colaboradora.

Si algún día un estudiante descarga Ry-Dit y crea su primer juego sin necesitar nada más que su teléfono... **ese es el logro.** Reconocido o no.

El mundo necesita más cosas que funcionen con poco. No más cosas que necesiten todo.

**Low-End First.** 🛡️

---

## TL;DR

Escribí un motor de juegos desde un celular Android con Termux + Rust + IA. 25 crates, GPU Instancing a 48 FPS, FSR 1.0, 8 demos. Filosofía: Low-End First. Código abierto. Para el 99% que tiene un celular, no una PC.

---

## Subreddits sugeridos

| Subreddit | Ángulo |
|-----------|--------|
| **r/rust** | Técnico: "25 crates compilando, GPU Instancing con OpenGL directo desde Termux" |
| **r/gamedev** | Story: "Escribí un motor desde un celular sin PC ni equipo" |
| **r/ProgrammerHumor** | Humor: "Mi setup de desarrollo vs el tuyo" |
| **r/android** | Práctico: "Desarrollo de juegos en Android sin PC" |
| **r/opensource** | Filosofía: "Open source low-first para el mundo emergente" |
| **r/selfhosted** | Local-first: "Todo corre sin nube, sin APIs, sin dependencias externas" |
