# 📸 Guía de Capturas - RyDit v0.1.9 en Termux-X11

**Objetivo:** Capturar TODO lo que RyDit puede renderizar antes de subir a GitHub

---

## 🎯 DEMOS QUE FUNCIONAN (Confirmados)

### 1. rydit-gfx Demo (v0.0.7) - ✅ CONFIRMADO
```bash
cd /data/data/com.termux/files/home/shield-project
export DISPLAY=:0
./target/debug/examples/demo
```

**Qué renderiza:**
- ✅ Círculo rojo animado (palpita)
- ✅ Rectángulo verde
- ✅ Línea azul diagonal
- ✅ Texto "RyDit v0.0.7 - rydit-gfx"
- ✅ Texto "Rust = Arquitecto, Raylib = Pincel"
- ✅ 60 FPS, 800x600

**Duración:** Indefinida (ESC para salir)

---

### 2. demo_shapes.rydit - ✅ POR PROBAR
```bash
cd /data/data/com.termux/files/home/shield-project
export DISPLAY=:0
./target/release/rydit-rs --gfx ejemplos_gfx/demo_shapes.rydit
```

**Qué debería renderizar:**
- Círculos concéntricos animados
- Rectángulos de colores
- Líneas horizontales
- Texto informativo

---

### 3. snake_perfect.rydit - ⚠️ POR PROBAR
```bash
cd /data/data/com.termux/files/home/shield-project
export DISPLAY=:0
./target/release/rydit-rs --gfx snake_perfect.rydit
```

**Qué debería renderizar:**
- Grid retro
- Serpiente verde
- Comida roja
- UI con puntuación
- Game Over screen

---

## 📸 MÉTODOS DE CAPTURA

### Método 1: Screenshot Manual (Recomendado)
```bash
# En Termux, instalar scrot
pkg install scrot

# Capturar ventana completa
scrot ~/screenshots/rydit_demo_01.png

# Capturar con delay de 3 segundos
scrot -d 3 ~/screenshots/rydit_demo_01.png

# Capturar solo ventana seleccionada
scrot -s ~/screenshots/rydit_demo_01.png
```

### Método 2: Screenshot desde Termux:X11 App
```
# En la app Termux:X11:
# - Menú lateral
# - "Screenshot" o "Capturar pantalla"
# - Guardar en /sdcard/Pictures/
```

### Método 3: Usando adb (si está disponible)
```bash
adb shell screencap -p /sdcard/rydit_01.png
adb pull /sdcard/rydit_01.png ./screenshots/
```

---

## 📋 LISTA DE CAPTURAS RECOMENDADAS

### Demo rydit-gfx (v0.0.7)
1. `demo_gfx_01_circle.png` - Círculo rojo en el centro
2. `demo_gfx_02_full.png` - Toda la ventana con todos los elementos
3. `demo_gfx_03_text.png` - Zoom en el texto "Rust = Arquitecto"

### demo_shapes.rydit
4. `demo_shapes_01_circles.png` - Círculos concéntricos
5. `demo_shapes_02_rects.png` - Rectángulos de colores
6. `demo_shapes_03_full.png` - Vista completa

### snake_perfect.rydit
7. `snake_01_gameplay.png` - Serpiente en movimiento
8. `snake_02_eating.png` - Comiendo comida
9. `snake_03_gameover.png` - Pantalla de Game Over
10. `snake_04_ui.png` - UI con puntuación alta

### Extras
11. `terminal_build.png` - Terminal mostrando "110 tests passing"
12. `terminal_run.png` - Terminal ejecutando el demo
13. `files_structure.png` - Estructura de archivos del proyecto

---

## 🎨 COMANDOS PARA CADA DEMO

### 1. Demo rydit-gfx (Binario directo)
```bash
export DISPLAY=:0
cd /data/data/com.termux/files/home/shield-project
./target/debug/examples/demo
# Presiona ESC para salir
```

### 2. demo_shapes.rydit
```bash
export DISPLAY=:0
cd /data/data/com.termux/files/home/shield-project
./target/release/rydit-rs --gfx ejemplos_gfx/demo_shapes.rydit
# ESC para salir
```

### 3. snake_perfect.rydit
```bash
export DISPLAY=:0
cd /data/data/com.termux/files/home/shield-project
./target/release/rydit-rs --gfx snake_perfect.rydit
# Flechas: mover, SPACE: restart, ESC: salir
```

### 4. ejemplo_gfx.rydit
```bash
export DISPLAY=:0
cd /data/data/com.termux/files/home/shield-project
./target/release/rydit-rs --gfx ejemplos_gfx/ejemplo_gfx.rydit
# ESC para salir
```

---

## 🔧 SOLUCIÓN DE PROBLEMAS

### Error: "Cannot open display"
```bash
# Asegurar que Termux:X11 está corriendo
export DISPLAY=:0
```

### Error: "Segmentation fault"
```bash
# Probar con el binario debug en lugar de release
./target/debug/examples/demo
```

### Error: "Pantalla negra"
```bash
# Verificar que raylib está instalado
pkg list installed | grep raylib

# Reinstalar si es necesario
pkg reinstall raylib
```

### Los demos de RyDit no abren ventana
```bash
# Usar flag --gfx explícitamente
./target/release/rydit-rs --gfx archivo.rydit

# Verificar que rydit-gfx está linkado
ldd ./target/release/rydit-rs | grep -i raylib
```

---

## 💾 ORGANIZACIÓN DE SCREENSHOTS

```
screenshots/
├── demo_gfx_01.png
├── demo_gfx_02.png
├── demo_shapes_01.png
├── snake_01_gameplay.png
├── snake_02_gameover.png
├── terminal_build.png
└── README.md (este archivo)
```

---

## 🚀 SUBIDA A GITHUB

### Crear carpeta en GitHub
```bash
mkdir -p docs/screenshots
cp screenshots/*.png docs/screenshots/
git add docs/screenshots/
git commit -m "Agregar screenshots de demos gráficos"
git push
```

### Usar en README.md
```markdown
## 🎮 Demos Gráficas

![Demo rydit-gfx](docs/screenshots/demo_gfx_01.png)
![Snake Gameplay](docs/screenshots/snake_01_gameplay.png)
```

---

## ✅ CHECKLIST DE CAPTURA

- [ ] Termux:X11 iniciado correctamente
- [ ] `export DISPLAY=:0` ejecutado
- [ ] demo rydit-gfx probado y capturado
- [ ] demo_shapes.rydit probado y capturado
- [ ] snake_perfect.rydit probado y capturado
- [ ] Al menos 5 screenshots tomadas
- [ ] Screenshots organizadas en carpeta
- [ ] README de screenshots creado
- [ ] Listo para subir a GitHub

---

**¡Éxito con las capturas!** 📸🎮

*Construido con ❤️ en Android/Termux*  
*RyDit v0.1.9 - 110 tests passing*
