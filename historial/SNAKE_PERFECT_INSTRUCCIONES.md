# 🎮 Snake Perfect - Instrucciones de Ejecución

**Versión:** RyDit v0.1.9  
**Binario:** `target/release/rydit-rs` (736 KB)  
**Archivo:** `snake_perfect.rydit` (33 statements)  

---

## 📋 REQUISITOS

### 1. Termux-X11 Instalado
```bash
# Instalar Termux-X11 desde F-Droid o GitHub
# https://github.com/termux/termux-x11
```

### 2. Raylib Nativo Instalado
```bash
pkg install raylib
```

### 3. Variables de Entorno Configuradas
```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1
```

---

## 🚀 EJECUCIÓN

### Opción 1: Desde el Directorio del Proyecto
```bash
cd /data/data/com.termux/files/home/shield-project
export DISPLAY=:0
./target/release/rydit-rs --gfx snake_perfect.rydit
```

### Opción 2: Usando el Script
```bash
./jugar_snake.sh
```

### Opción 3: Binario Directo
```bash
./target/release/rydit-rs --gfx snake_perfect.rydit
```

---

## 🎮 CONTROLES

| Tecla | Acción |
|-------|--------|
| **↑** (Flecha Arriba) | Mover hacia arriba |
| **↓** (Flecha Abajo) | Mover hacia abajo |
| **←** (Flecha Izquierda) | Mover hacia izquierda |
| **→** (Flecha Derecha) | Mover hacia derecha |
| **SPACE** | Reiniciar después de Game Over |
| **ESC** | Salir del juego |

---

## 🎯 CARACTERÍSTICAS

- ✅ **Game Loop Completo** - 60 FPS
- ✅ **Input Continuo** - `tecla_presionada()`
- ✅ **Colisiones** - Paredes y cuerpo
- ✅ **Comida Aleatoria** - `random::int()`
- ✅ **Puntuación** - Score y High Score
- ✅ **Velocidad Ajustable** - Control de frames
- ✅ **Grid Visual** - Estilo retro
- ✅ **Game Over** - Pantalla de restart

---

## 📊 ESPECIFICACIONES TÉCNICAS

```
Statements:     33
Módulos:        random (importado)
Funciones:      random::int(), random::float(), random::choice()
Variables:      20+
Arrays:         serpiente_x[], serpiente_y[]
Colisiones:     Paredes + Cuerpo
Puntuación:     Score + High Score
Velocidad:      8 frames/movimiento
```

---

## 🖼️ SCREENSHOTS

Para capturar screenshots en Termux-X11:

### Método 1: Screenshot Manual
```bash
# En Termux-X11, usar combinación de teclas
# (depende de la configuración de Termux-X11)
```

### Método 2: Usando scrot
```bash
pkg install scrot
scrot screenshot_snake.png
```

---

## 🐛 SOLUCIÓN DE PROBLEMAS

### Error: "DISPLAY no establecido"
```bash
export DISPLAY=:0
```

### Error: "No se puede abrir la ventana"
```bash
# Asegurar que Termux-X11 está corriendo
# Reiniciar Termux-X11 si es necesario
```

### Error: "raylib no encontrado"
```bash
pkg install raylib
```

### Error de Sintaxis en Línea X
```bash
# Verificar que el archivo no tenga emojis o caracteres UTF-8 raros
# Usar solo ASCII en comentarios
```

---

## 🎨 PERSONALIZACIÓN

Editar `snake_perfect.rydit`:

### Cambiar Colores
```rydit
dark.slot $color_serpiente = "azul"
dark.slot $color_comida = "amarillo"
```

### Cambiar Velocidad
```rydit
dark.slot velocidad = 5  # Menor = más rápido
```

### Cambiar Tamaño de Celda
```rydit
dark.slot CELDA = 30  # Más grande = más fácil
```

---

## 📈 RENDIMIENTO

```
Binario:        736 KB (release optimizado)
RAM:            ~11 MB
FPS:            60 (vsync)
Build Time:     ~47s (release)
```

---

## 💾 BACKUP

El archivo `snake_perfect.rydit` está sincronizado con Google Drive:
```
alucard18:/shield-project-rydit/snake_perfect.rydit
```

---

**¡Disfruta del juego!** 🐍🎮

*Construido con ❤️ en Android/Termux*  
*RyDit v0.1.9 - 110 tests passing*
