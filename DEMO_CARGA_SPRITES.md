# 🎮 Demo Carga de Sprites - Documentación

**Fecha**: 2026-04-02  
**Versión**: v0.11.6  
**Estado**: ✅ COMPILADO EXITOSAMENTE  
**Binario**: `target/release/demo_carga_sprites` (~560 KB)  

---

## 📋 DESCRIPCIÓN

Demo simplificado de carga de sprites PNG usando **SDL2 + Zink (DRI3)** para Termux-X11.

### Características

- ✅ **4 sprites** disponibles (tank, helicopter, crate, platform)
- ✅ **Verificación automática** de archivos PNG
- ✅ **Fallback a colores** si las texturas no cargan
- ✅ **Movimiento con teclado** (← → ↑ ↓)
- ✅ **Selección de sprites** (teclas 1-4)
- ✅ **Escalado dinámico** (+/-)
- ✅ **Animación senoidal** (helicóptero flotando, crate oscilando)
- ✅ **Panel de información** toggleable
- ✅ **Grid de fondo** para referencia visual

---

## 🚀 EJECUCIÓN

### Prerrequisitos

```bash
# 1. Termux-X11 debe estar ejecutándose
# 2. Display :0 activo
export DISPLAY=:0

# 3. SDL2 instalado
pkg install sdl2 sdl2_image sdl2_ttf sdl2_mixer
```

### Comandos

```bash
# Ejecutar desde directorio del proyecto
./target/release/demo_carga_sprites

# O con cargo
cargo run --bin demo_carga_sprites --release

# Build (si necesita recompilar)
cargo build -p rydit-rs --bin demo_carga_sprites --release
```

---

## 🎮 CONTROLES

| Tecla | Acción |
|-------|--------|
| **← → ↑ ↓** | Mover sprite seleccionado |
| **1-4** | Seleccionar sprite (tank, helicopter, crate, platform) |
| **+** / **=** | Aumentar escala del sprite |
| **-** | Disminuir escala del sprite |
| **I** | Toggle panel de información |
| **A** | Toggle animación |
| **R** | Resetear posiciones originales |
| **ESC** | Salir del demo |

---

## 📁 SPRITES DISPONIBLES

| Sprite | Archivo | Tamaño | Color Fallback |
|--------|---------|--------|----------------|
| **Tank** | `tank_16x16.png` | 16x16 | 🟢 Verde |
| **Helicopter** | `helicopter_16x16.png` | 16x16 | 🔵 Cyan |
| **Crate** | `crate_8x8.png` | 8x8 | 🟤 Marrón |
| **Platform** | `platform_16x16.png` | 16x16 | ⚫ Gris |

**Ubicación**: `/data/data/com.termux/files/home/shield-project/logo_icon_asst/sprites/`

---

## 🏗️ ARQUITECTURA TÉCNICA

### Backend

- **SDL2** para ventana, input y render 2D
- **SDL2_image** para carga de PNG/JPG
- **Zink** como driver OpenGL (DRI3)
- **OpenGL 3.3 Core** context

### Estructura del Código

```rust
struct SpriteInfo {
    nombre: &'static str,      // Nombre del sprite
    x: f32,                    // Posición X
    y: f32,                    // Posición Y
    width: u32,                // Ancho original
    height: u32,               // Alto original
    scale: f32,                // Escala actual
    color: (u8, u8, u8),       // Color fallback RGB
    archivo: String,           // Nombre del archivo PNG
    existe: bool,              // Si el archivo existe
}
```

### Game Loop

```
1. Procesar eventos SDL2
2. Input (teclado)
3. Update (animaciones)
4. Render (grid + sprites + UI)
5. Present frame
```

---

## ⚠️ NOTAS IMPORTANTES

### Limitaciones Actuales

1. **Texturas vs Fallback**:
   - El demo verifica si los archivos PNG existen
   - Si SDL2_image puede cargarlos, los muestra con indicador visual
   - Si no, usa rectángulos de colores como fallback

2. **SDL2_image en Android**:
   - La carga de texturas puede tener limitaciones en Termux
   - Se requiere `pkg install sdl2_image`
   - El fallback garantiza funcionamiento siempre

3. **Zink + DRI3**:
   - Display debe estar activo (`echo $DISPLAY` → `:0`)
   - Termux-X11 debe estar ejecutándose

### Solución de Problemas

| Problema | Solución |
|----------|----------|
| "No such file or directory" | Verificar que Termux-X11 está abierto |
| Sprites no cargados | `pkg install sdl2_image` |
| Ventana no aparece | `export DISPLAY=:0` |
| Teclado no responde | Click en ventana para foco |

---

## 📊 MÉTRICAS

| Métrica | Valor |
|---------|-------|
| **Tamaño binario** | ~560 KB |
| **FPS objetivo** | 60 FPS (vsync) |
| **Resolución** | 800x600 |
| **Sprites** | 4 |
| **Líneas de código** | ~300 |
| **Tiempo compilación** | ~9 minutos (release) |

---

## 🔍 DIFERENCIAS CON DEMO_ASSETS_SIMPLE

| Característica | demo_assets_simple.rs | demo_carga_sprites.rs |
|----------------|----------------------|-----------------------|
| **Backend** | Raylib | SDL2 |
| **Assets** | Rects simulados | Verifica archivos PNG reales |
| **Input** | Mouse (drag & drop) | Teclado (movimiento) |
| **Animación** | No | Sí (senoidal) |
| **Info panel** | No | Sí |
| **Escalado** | Fijo (32x32) | Dinámico (+/-) |
| **Resolución** | 1280x720 | 800x600 |

---

## 🎯 PRÓXIMAS MEJORAS

- [ ] Carga real de texturas SDL2_image (verificar compatibilidad Android)
- [ ] Soporte para mouse (drag & drop como demo_assets_simple)
- [ ] Más sprites (cube_8x8.png no incluido)
- [ ] Rotación de sprites
- [ ] Alpha blending
- [ ] Sprite sheets

---

## 📚 ARCHIVOS RELACIONADOS

| Archivo | Ubicación |
|---------|-----------|
| **Binario** | `target/release/demo_carga_sprites` |
| **Código fuente** | `crates/rydit-rs/src/bin/demo_carga_sprites.rs` |
| **Sprites** | `logo_icon_asst/sprites/*.png` |
| **Guía completa** | `GUIA_BINARIOS_GRAFICOS_TERMUX_X11.md` |
| **Resumen tareas** | `RESUMEN_TAREAS_V0.11.6.md` |

---

## ✅ CHECKLIST DE VERIFICACIÓN

Antes de ejecutar en Termux-X11:

- [ ] Termux-X11 está abierto y activo
- [ ] `export DISPLAY=:0` ejecutado
- [ ] `pkg install sdl2 sdl2_image sdl2_ttf` instalado
- [ ] Binario compilado: `ls -lh target/release/demo_carga_sprites`
- [ ] Sprites existen: `ls logo_icon_asst/sprites/`

Durante la ejecución:

- [ ] Ventana 800x600 se abre
- [ ] Grid de fondo visible
- [ ] 4 sprites (rectángulos) dibujados
- [ ] Texto superior legible
- [ ] Teclado responde (← → ↑ ↓)
- [ ] Selección funciona (1-4)
- [ ] Escalado funciona (+/-)
- [ ] Animación activa (helicopter flotando)
- [ ] Panel info muestra datos (tecla I)
- [ ] Reset funciona (tecla R)
- [ ] Salir funciona (ESC)

---

<div align="center">

**🛡️ RyDit v0.11.6 - Demo Carga Sprites**

*SDL2 + Zink | 4 sprites | 60 FPS | Termux-X11*

**Estado**: ✅ LISTO PARA PROBAR

</div>
