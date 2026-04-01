# 🛡️ RyDit v0.11.0 - GUÍA RÁPIDA PARA EL USUARIO

**Última actualización**: 2026-03-31  
**Versión**: v0.11.0  
**Estado**: ✅ **95% Sistema Ry + SDL2 conectado**

---

## 🚀 **INICIO RÁPIDO**

### **Requisitos**
- ✅ Termux + Termux-X11 instalados
- ✅ Rust instalado (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- ✅ SDL2 instalado (`pkg install sdl2`)

---

## 🎮 **DEMOS DISPONIBLES**

### **1. Demo Platformer Completo** ⭐ RECOMENDADO
```bash
cargo run --bin demo_platformer_completo --release
```

**Características**:
- ✅ Movimiento lateral (A, D, ←, →)
- ✅ Salto con gravedad (W, ↑, SPACE)
- ✅ Plataformas múltiples
- ✅ Colisiones detectadas
- ✅ 60 FPS estables

**Controles**:
| Tecla | Acción |
|-------|--------|
| **A / ←** | Mover izquierda (mantener) |
| **D / →** | Mover derecha (mantener) |
| **W / ↑ / SPACE** | Saltar |
| **ESC** | Salir |

---

### **2. Demo Movimiento Básico**
```bash
cargo run --bin demo_movimiento --release
```

**Características**:
- ✅ Movimiento en 4 direcciones
- ✅ 60 FPS estables
- ✅ Ideal para probar input

---

### **3. Demo MiGUI + SDL2**
```bash
cargo run --bin demo_migui_sdl2 --release
```

**Características**:
- ✅ Botones interactivos
- ✅ Slider ajustable
- ✅ Checkbox
- ✅ Panel dinámico

**Nota**: El texto se muestra como rects (pendiente SDL2_ttf)

---

### **4. Demo Partículas SDL2**
```bash
cargo run --bin demo_particulas_sdl2 --release
```

**Características**:
- ✅ 100+ partículas
- ✅ Emisor controlado por teclado
- ✅ 60 FPS estables

---

## 📚 **COMANDOS ÚTILES**

### **Compilar todo el proyecto**
```bash
cargo build --release
```

### **Compilar un demo específico**
```bash
cargo build --bin demo_platformer_completo --release
```

### **Ver todos los demos disponibles**
```bash
ls crates/rydit-rs/src/bin/demo_*.rs
```

### **Limpiar build anterior**
```bash
cargo clean
```

### **Actualizar dependencias**
```bash
cargo update
```

---

## 🎯 **CLAVE DEL MOVIMIENTO SDL2**

**Importante**: Para que el movimiento funcione en SDL2/Termux-X11, el código debe usar **DOS eventos**:

```rust
// ✅ PRIMERA PULSACIÓN
Event::KeyDown { keycode: Some(key), repeat: false, .. }

// ✅ TECLA MANTENIDA (movimiento continuo)
Event::KeyDown { keycode: Some(key), repeat: true, .. }
```

**Sin `repeat: true` → NO hay movimiento lateral**

Ver documentación completa: `CLAVE_MOVIMIENTO_SDL2.md`

---

## 📁 **ESTRUCTURA DEL PROYECTO**

```
rydit-engine/
├── crates/
│   ├── lizer/              # Parser RyDit
│   ├── rydit-core/         # Core del engine
│   ├── rydit-gfx/          # Gráficos (SDL2 + Raylib)
│   ├── rydit-anim/         # 12 principios de animación
│   ├── rydit-science/      # Geometría + matemáticas
│   ├── rydit-rs/           # Binario principal
│   │   └── src/bin/        # Demos
│   │       ├── demo_platformer_completo.rs
│   │       ├── demo_movimiento.rs
│   │       ├── demo_migui_sdl2.rs
│   │       └── demo_particulas_sdl2.rs
│   └── migui/              # Immediate Mode GUI
├── docs/                   # Documentación técnica
├── README_EN.md            # Este archivo
└── QWEN.md                 # Bitácora técnica
```

---

## 🛠️ **SOLUCIÓN DE PROBLEMAS**

### **Error: "no SDL2 video device found"**
```bash
# Iniciar Termux-X11 primero
xinit
# Luego en otra sesión:
cargo run --bin demo_platformer_completo --release
```

### **Error: "failed to create window"**
```bash
# Verificar que DISPLAY está configurado
echo $DISPLAY
# Debe mostrar: :0

# Si no, configurar:
export DISPLAY=:0
```

### **Error: "undefined symbol: IMG_Init"**
```bash
# Reinstalar SDL2_image
pkg uninstall sdl2_image
pkg install sdl2_image
cargo clean
cargo build --release
```

### **El movimiento lateral no funciona**
- Verificar que el código usa `repeat: false` Y `repeat: true`
- Ver documentación: `CLAVE_MOVIMIENTO_SDL2.md`

---

## 📖 **DOCUMENTACIÓN ADICIONAL**

| Archivo | Descripción |
|---------|-------------|
| **README_EN.md** | Documentación principal (inglés) |
| **QWEN.md** | Bitácora técnica completa |
| **ESTRUCTURA.md** | Estructura del proyecto |
| **CLAVE_MOVIMIENTO_SDL2.md** | Clave del movimiento SDL2 |
| **SISTEMA_RY_ESTADO_REAL.md** | Estado del Sistema Ry |
| **ESTADO_FINAL_V0.10.6.md** | Estado SDL2 Backend |

---

## 🎨 **PRÓXIMO: v0.12.0**

### **Pendiente de Fix**
- ⚠️ **MiGUI texto** - SDL2_ttf o ab_glyph
- ⚠️ **assets::load_texture_sdl2** - API correcta
- ⚠️ **Parser lizer** - Zero-copy + bytecode

### **Nuevas Features**
- 🎨 **12 Principios de Animación 2D**
  - Implementados en `crates/rydit-anim/`
  - Squash & Stretch, Anticipation, Slow In/Out, etc.
- 🎨 **Geometrías de Ilusiones Ópticas**
- 🎨 **Efectos especiales de animación**

---

## 💬 **COMUNIDAD**

- **GitHub**: https://github.com/lapumlbb18-blip/Rydit_Engine
- **Discord**: https://discord.gg/mouredev (#mostrar-proyecto)
- **Issues**: Reportar bugs en GitHub Issues

---

## 📄 **LICENCIA**

MIT License - Ver [LICENSE](LICENSE) para detalles.

---

<div align="center">

**🛡️ RyDit v0.11.0 - GUÍA RÁPIDA**

*95% Sistema Ry + SDL2 ✅ | Platformer Demo ✅ | 60 FPS ✅*

**Próximo: v0.12.0 - 12 Principios de Animación 2D**

</div>
