# 📸 RyDit v0.1.9 - Screenshots en Termux-X11

**Fecha:** 2026-03-20  
**Plataforma:** Android/Termux + Termux-X11  
**Renderizador:** raylib 5.5 (OpenGL 4.6, Mesa 22.0.5)  

---

## 🎮 Capturas Disponibles

### 1. Demo rydit-gfx (v0.0.7)

**Archivo:** `01_demo_rydit_gfx_menu.jpg` (39 KB)
- Menú de selección de demos
- Test_demos_x11.sh en acción

**Archivo:** `02_demo_rydit_gfx_completo.jpg` (302 KB)
- Círculo rojo animado (palpita)
- Rectángulo verde
- Línea azul diagonal
- Texto: "RyDit v0.0.7 - rydit-gfx"
- Texto: "Rust = Arquitecto, Raylib = Pincel"
- 60 FPS, 800x600

---

### 2. Demo Shapes

**Archivo:** `03_demo_shapes_circulos.jpg` (181 KB)
- Círculos concéntricos animados
- Rectángulos de colores
- Líneas horizontales
- Texto informativo
- Demo: `ejemplos_gfx/demo_shapes.rydit`

---

### 3. Snake Game

**Archivo:** `04_snake_gameplay.jpg` (286 KB)
- Snake en movimiento
- Grid retro estilo classic
- Comida roja
- Serpiente verde
- UI con puntuación

**Archivo:** `05_snake_gameover.jpg` (220 KB)
- Pantalla de Game Over
- Puntuación final
- High score
- Opciones de restart

---

## 🔧 Detalles Técnicos

### Configuración
```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1
```

### Binarios
- **demo:** 517 KB (target/debug/examples/demo)
- **rydit-rs:** 736 KB (target/release/rydit-rs)

### Demos
- `snake_perfect.rydit` - 33 statements
- `demo_shapes.rydit` - 3 statements
- `ejemplo_gfx.rydit` - 17 statements

---

## 📊 Especificaciones

| Métrica | Valor |
|---------|-------|
| **Resolución** | 800x600 |
| **FPS** | 60 (vsync) |
| **Driver** | llvmpipe (LLVM 11.1.0) |
| **OpenGL** | 4.6 Compatibility Profile |
| **Mesa** | 22.0.5 |
| **RAM Runtime** | ~11 MB |

---

## 🎨 Capturas por Demo

### rydit-gfx Demo (Binario Rust)
- **Círculo rojo:** Radio 50 + animación (frame % 20)
- **Rectángulo verde:** 100x100 en (100, 100)
- **Línea azul:** Diagonal completa (0,0) a (800,600)
- **Texto:** 3 líneas con diferentes tamaños

### demo_shapes.rydit (RyDit Script)
- **Círculos concéntricos:** Animación por frames
- **Rectángulos:** 5 colores diferentes
- **Líneas:** Pattern horizontal
- **Texto:** "Demo RyDit v0.1.8"

### snake_perfect.rydit (RyDit Script)
- **Grid:** 20x20 celdas, líneas gris oscuro
- **Serpiente:** Verde con cabeza verde claro
- **Comida:** Círculo rojo
- **UI:** Score, High Score, Speed, Length
- **Game Over:** Overlay negro + texto amarillo

```

---

## 🚀 Uso en GitHub

### En README.md
```markdown
## 🎮 Demos Gráficas

![Demo rydit-gfx](screenshots/02_demo_rydit_gfx_completo.jpg)
*Demo rydit-gfx v0.0.7 - Círculo rojo animado, rectángulo verde, línea azul*

![Snake Gameplay](screenshots/04_snake_gameplay.jpg)
*Snake Game - Grid retro, serpiente verde, comida roja*
```

### En GitHub Issues/PRs
```markdown
![Demo completo](screenshots/02_demo_rydit_gfx_completo.jpg)
```

---

## 📸 Cómo Tomar Más Capturas

```bash
# Instalar herramienta
pkg install scrot

# Capturar con delay
scrot -d 3 screenshot.png

# Capturar ventana específica
scrot -s screenshot.png
```

---

## ✅ Checklist de Capturas

- [x] Demo rydit-gfx (binario)
- [x] Demo shapes (RyDit script)
- [x] Snake gameplay
- [x] Snake game over
- [x] Nombres descriptivos
- [x] README de documentación
- [x] Backup Google Drive
- [ ] ¿Falta alguna? Agregar aquí

---

**¡Listas para GitHub!** 🚀

*Construido con ❤️ en Android/Termux*  
*RyDit v0.1.9 - 110 tests passing*
