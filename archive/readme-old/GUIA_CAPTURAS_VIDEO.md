# 📸 Guía para Capturas y Video - RyDit v0.1.0

## 🎬 OBJETIVO

Crear contenido visual para demostrar que RyDit **REALMENTE funciona en Android**.

---

## 📱 CAPTURAS DE PANTALLA (Termux:X11)

### Configuración Recomendada:

```bash
# 1. Iniciar Termux:X11
# 2. Ejecutar Snake:
cd ~/shield-project
cargo run -- --gfx snake_completo.rydit

# 3. Capturar momentos clave:
#    - Juego iniciando
#    - Serpiente moviéndose
#    - Comida siendo consumida
#    - Game Over screen
#    - Restart exitoso
```

### Momentos para Capturar:

1. **Inicio del Juego**
   - Serpiente en centro
   - Score en 0
   - UI visible

2. **Jugando (Score ~50)**
   - Serpiente con cuerpo visible (~5 segmentos)
   - Comida en pantalla
   - Score mostrando progreso

3. **Game Over**
   - Pantalla de Game Over
   - Score Final visible
   - High Score visible

4. **Código (Opcional)**
   - Abrir snake_completo.rydit en editor
   - Mostrar funciones: `esta_en_cuerpo()`, `nueva_comida()`

---

## 🎥 VIDEO CORTO (1-2 minutos)

### Guión Sugerido:

**0:00 - 0:10 | INTRO**
```
Texto en pantalla: "RyDit v0.1.0 - Snake Game"
"Construido 100% en Android/Termux"
"Sin laptop. Sin IDE. Solo código."
```

**0:10 - 0:30 | DEMO DEL JUEGO**
```
- Mostrar Snake ejecutándose
- Mover serpiente con flechas
- Comer comida (score aumenta)
- Mostrar cuerpo de serpiente creciendo
```

**0:30 - 0:45 | GAME OVER + RESTART**
```
- Chocar con pared (Game Over)
- Presionar SPACE para restart
- Juego reinicia correctamente
```

**0:45 - 1:00 | CÓDIGO FUENTE**
```
- Mostrar archivo snake_completo.rydit
- Resaltar funciones clave:
  * rytmo esta_en_cuerpo(x, y)
  * rytmo nueva_comida()
  * Composición: score + cuerpo + colisiones
```

**1:00 - 1:15 | ESTADÍSTICAS**
```
Texto en pantalla:
"60 tests automáticos"
"0 warnings"
"3,250+ líneas de Rust"
"5 crates"
"4 días de desarrollo"
```

**1:15 - 1:30 | CIERRE**
```
"GitHub: [tu-usuario]/rydit-language"
"Documentación completa: LIBRO_RYDIT.md"
"¡Contribuciones bienvenidas!"
```

---

## 🛠️ HERRAMIENTAS RECOMENDADAS

### Para Capturas:
- **Termux:X11** - Para ventana gráfica
- **Screenshot nativo de Android** - Botón físico
- **O:** `screenshot` command en Termux (si está disponible)

### Para Video:
- **Grabador de pantalla de Android** - Nativo
- **O:** OBS si estás en PC con emulador
- **Duración:** 1-2 minutos máximo

### Para Edición:
- **CapCut** - Gratis, Android
- **Kinemaster** - Android
- **O:** Sin edición, solo grabación continua

---

## 📤 DÓNDE PUBLICAR

### 1. YouTube (Principal)
```
Título: "Snake Game en RyDit - Lenguaje de Scripting Construido en Android"
Descripción:
  - Link a GitHub
  - Link a LIBRO_RYDIT.md
  - Hashtags: #rust #android #termux #gamedev
```

### 2. Reddit
```
Subreddits:
- r/rust (principal)
- r/termux
- r/androiddev
- r/programming
- r/gamedev

Título: "Built a scripting language entirely on Android/Termux - Snake game demo"
```

### 3. X (Twitter)
```
Tweet 1: Video + "Just built Snake game in my language RyDit"
Tweet 2: "100% developed on Android/Termux, no laptop"
Tweet 3: Link a GitHub + #rust #android

Tag: @rustlang @termux @androiddev
```

### 4. LinkedIn (Opcional)
```
Post profesional:
"Desarrollé un lenguaje de scripting completamente en móvil..."
- Enfoque en logro técnico
- Link a GitHub
```

---

## 📝 TEXTO PARA README DE GITHUB

```markdown
## 🎮 Snake Game Demo

**¡Totalmente funcional en Android/Termux!**

![Snake Game](screenshots/snake-gameplay.png)
![Game Over](screenshots/snake-gameover.png)

### Ejecutar:
```bash
cargo run -- --gfx snake_completo.rydit
```

### Features Demostradas:
- ✅ Arrays y indexación
- ✅ Funciones con retorno
- ✅ Composición de funciones
- ✅ Game loop con input
- ✅ Colisiones y scoring
- ✅ Restart sin recompilar

### Código (150 líneas):
- `rytmo esta_en_cuerpo(x, y)` - Verifica colisión
- `rytmo nueva_comida()` - Posición aleatoria válida
- `rytmo random_pos(max)` - Pseudo-random simple
```

---

## ✅ CHECKLIST ANTES DE PUBLICAR

- [ ] Snake game compilado y funcionando
- [ ] 5-10 capturas de pantalla tomadas
- [ ] Video de 1-2 minutos grabado
- [ ] Video subido a YouTube
- [ ] Screenshots en carpeta `screenshots/`
- [ ] README de GitHub actualizado con imágenes
- [ ] Posts preparados para Reddit/X

---

## 🚀 TIMELINE SUGERIDO

| Hora | Tarea |
|------|-------|
| **0:00 - 0:45** | Jugar Snake, tomar capturas |
| **0:45 - 1:15** | Grabar video demo |
| **1:15 - 1:30** | Subir video a YouTube |
| **1:30 - 2:00** | Actualizar README con imágenes |
| **2:00 - 2:30** | Preparar posts para Reddit/X |
| **2:30** | **PUBLICAR** |

---

## 💡 CONSEJOS FINALES

1. **Autenticidad > Perfección**
   - No necesitas edición profesional
   - Mostrar Termux:X11 real es más auténtico

2. **Muestra el código**
   - La gente quiere VER que es código real
   - 10 segundos de código > 1 minuto de gameplay

3. **Enfatiza el logro**
   - "100% Android" es tu diferenciador
   - Repítelo en cada plataforma

4. **Sé transparente**
   - "v0.1.0 - Experimental"
   - "Feedback bienvenido"
   - "Proyecto de aprendizaje"

5. **Responde comentarios**
   - Los primeros comentarios son ORO
   - Responde TODOS en las primeras 24h

---

**¡Tu historia merece ser vista! 🚀**

*RyDit v0.1.0 - Construido con ❤️ en Android*
