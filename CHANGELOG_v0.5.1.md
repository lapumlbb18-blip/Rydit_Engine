# 🛡️ CHANGELOG v0.5.1 - RyDit Engine

**Fecha:** 2026-03-23
**Versión:** v0.5.1
**Estado:** ✅ COMPLETADO

---

## 🎉 RESUMEN

**v0.5.1 - Funciones Assets + Renderizado Termux-X11**

Esta versión introduce las **funciones de assets en RyDit** para cargar y dibujar sprites, junto con el **fix crítico de renderizado** para Termux-X11 que permite 60 FPS estables con texturas.

---

## ✨ NUEVAS FEATURES

### 🎨 Funciones Assets en RyDit

**Funciones disponibles:**

```rydit
# Cargar textura desde archivo PNG
assets::load_texture("tank", "sprites/tank_16x16.png")

# Dibujar textura en posición (x, y)
assets::draw("tank", 100, 200, "blanco")

# Dibujar textura escalada
assets::draw_scaled("tank", 100, 200, 4, "blanco")  # 4x scale

# Verificar si existe textura
si assets::has("tank") {
    voz "Tank cargado"
}

# Obtener dimensiones
dark.slot ancho = assets::width("tank")
dark.slot alto = assets::height("tank")
```

**Sprites disponibles:**
- `tank_16x16.png` - Tanque top-down (escalable a 64x64)
- `helicopter_16x16.png` - Helicóptero con rotores (escalable a 48x48)
- `crate_8x8.png` - Caja de madera (8x8)
- `platform_16x16.png` - Plataforma/suelo (16x16)
- `cube_8x8.png` - Cubo de piedra (8x8)

### 🔧 Fix Renderizado Termux-X11

**Variables de entorno críticas:**
```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1
```

**Fixes implementados:**
1. **Variable `frame` en game loop** - Ahora `ryda frame < 5000` funciona correctamente
2. **`evaluar_expr_gfx` en statements de dibujo** - `draw.rect`, `draw.circle`, etc. ahora usan el contexto gráfico completo
3. **Carga de texturas con FFI** - `Texture2D::from_raw()` para cargar sin handle de raylib
4. **Texturas se cargan UNA vez** - No en cada frame (antes se recargaban 800+ veces)

---

## 📦 DEMOS NUEVAS

### `demos/demo_assets_v0.5.1.rydit`

**Características:**
- Tanque verde controlado con WASD
- Helicóptero controlado con flechas
- 3 cajas decorativas
- Suelo café
- Textos informativos
- 60 FPS estables

**Controles:**
- `W/A/S/D` - Mover tanque
- `Flechas` - Mover helicóptero
- `ESC` - Salir

**Código:**
```rydit
shield.init

# Cargar texturas
assets::load_texture("tank", "logo_icon_asst/sprites/tank_16x16.png")
assets::load_texture("heli", "logo_icon_asst/sprites/helicopter_16x16.png")
assets::load_texture("crate", "logo_icon_asst/sprites/crate_8x8.png")

# Game loop
ryda frame < 5000 {
    # Limpiar pantalla
    draw.rect(0, 0, 800, 600, "negro")
    
    # Input tanque
    onif tecla_presionada("w") {
        dark.slot tank_y = tank_y - 2
    }
    
    # Dibujar sprites escalados
    assets::draw_scaled("tank", tank_x, tank_y, 4, "blanco")
    assets::draw_scaled("heli", heli_x, heli_y, 3, "blanco")
}
```

---

## 🔧 CAMBIOS TÉCNICOS

### `crates/rydit-rs/src/main.rs` (+200 líneas)

**Cambios:**
1. **Variable `frame_count` en `ejecutar_programa_gfx`**
   ```rust
   let mut frame_count = 0;
   while !gfx.should_close() {
       frame_count += 1;
       executor.guardar("frame", Valor::Num(frame_count as f64));
       // ...
   }
   ```

2. **`evaluar_expr_gfx` en statements de dibujo**
   ```rust
   // Antes: evaluar_expr (sin input/assets)
   let x_val = evaluar_expr(x, executor, funcs);
   
   // Ahora: evaluar_expr_gfx (con todo)
   let x_val = evaluar_expr_gfx(x, executor, input, funcs, assets);
   ```

3. **Funciones `assets::draw` y `assets::draw_scaled`**
   ```rust
   // En ejecutar_stmt_gfx(), dentro de Stmt::Call
   else if name == "assets::draw" && args.len() >= 3 {
       // Obtener parámetros
       // Dibujar con d.draw_texture_ex()
   }
   ```

### `crates/rydit-gfx/src/lib.rs` (+30 líneas)

**Cambios:**
1. **Función `load_texture_from_path`**
   ```rust
   pub fn load_texture_from_path(path: &str) -> Result<Texture2D, String> {
       unsafe {
           let ffi_texture = raylib::ffi::LoadTexture(c_path.as_ptr());
           Ok(Texture2D::from_raw(ffi_texture))
       }
   }
   ```

2. **Método `draw_texture_ex` en `DrawHandle`**
   ```rust
   pub fn draw_texture_ex(&mut self, texture: &Texture2D, position: Vector2, rotation: f32, scale: f32, color: Color) {
       self.draw.draw_texture_ex(texture, position, rotation, scale, color);
   }
   ```

---

## 📄 DOCUMENTACIÓN NUEVA

### Archivos Creados

1. **`SOLUCION_RENDERIZADO_TERMUX_X11_V0.5.1.md`** (8 KB)
   - Estrategia completa de solución
   - Variables de entorno (DISPLAY, zink, DRI3)
   - Fix de variable `frame` en game loop
   - Fix de `evaluar_expr_gfx` en statements de dibujo
   - Funciones assets::draw y assets::draw_scaled
   - Carga de texturas con FFI

2. **`BACKUP_INSTRUCCIONES_V0.5.1.md`** (10 KB)
   - Estrategia de backup (excluir target/, incluir binarios)
   - Configuración de rclone + Google Drive
   - Comandos manuales y automáticos
   - Solución de problemas

3. **`backup_google_drive.sh`** (3 KB)
   - Backup rápido (solo código)
   - Excluye target/ automáticamente

4. **`backup_con_binarios.sh`** (3 KB)
   - Backup completo (código + binarios)
   - Sube rydit-rs y snake a carpeta `binarios/`

---

## 📊 MÉTRICAS

### Antes (v0.5.0)
```
Tests: 124 passing
Líneas: ~9,900
Binario: ~850 KB
Demos: 19
Widgets: 10
Sprites: 0 (solo Assets Manager en Rust)
FPS en X11: Variables (pantalla negra a veces)
```

### Después (v0.5.1)
```
Tests: 124 passing (sin regresiones)
Líneas: ~10,100 (+200)
Binario: ~870 KB (+20 KB)
Demos: 19 (+1 funcional)
Widgets: 10
Sprites: 3 cargados (tank, heli, crate)
FPS en X11: 60 estables ✅
```

---

## 🐛 BUG FIXES

### 1. Pantalla Negra en Termux-X11
**Problema:** Raylib no usaba la GPU en Android

**Solución:** Variables de entorno
```bash
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1
```

### 2. Game Loop Terminaba Inmediatamente
**Problema:** Variable `frame` no existía

**Solución:** Agregar `frame_count` en Rust
```rust
executor.guardar("frame", Valor::Num(frame_count as f64));
```

### 3. Texturas se Recargaban en Cada Frame
**Problema:** `assets::load_texture()` se llamaba en cada iteración

**Solución:** Texturas se cargan UNA vez al inicio

### 4. `draw.rect` No Funcionaba en Modo Gráfico
**Problema:** Usaba `evaluar_expr` en lugar de `evaluar_expr_gfx`

**Solución:** Cambiar todas las llamadas a `evaluar_expr_gfx`

---

## ✅ TESTS

### Tests Existentes (sin cambios)
```
lizer: 74 tests
blast-core: 20 tests
migui: 8 tests
v-shield: 7 tests
rydit-rs: 15 tests
─────────────────────
Total: 124 tests passing ✅
```

### Pruebas Manuales Realizadas
```
✅ Carga de texturas (3 sprites)
✅ Dibujar texturas (assets::draw)
✅ Dibujar texturas escaladas (assets::draw_scaled)
✅ Input WASD para tanque
✅ Input flechas para helicóptero
✅ 60 FPS estables en Termux-X11
✅ Variables zink funcionan
✅ Game loop ryda frame funciona
```

---

## 🚀 COMANDOS DE EJECUCIÓN

### Ejecutar Demo Assets
```bash
cd /data/data/com.termux/files/home/shield-project
DISPLAY=:0 MESA_LOADER_DRIVER_OVERRIDE=zink DRI3=1 \
    ./target/release/rydit-rs --gfx demos/demo_assets_v0.5.1.rydit
```

### Backup a Google Drive
```bash
# Solo código
./backup_google_drive.sh

# Código + Binarios
./backup_con_binarios.sh
```

---

## 🔜 PRÓXIMA VERSIÓN (v0.5.2)

### Planificado
- [ ] **Motor de Escenas** - Cambiar entre menús, niveles
- [ ] **Prefabs** - Objetos reutilizables
- [ ] **Sistema de Partículas** - Explosiones, efectos
- [ ] **Animaciones** - Sprite sheets básicos

### En Discusión
- [ ] Tilesets para mapas
- [ ] Sistema de cámaras
- [ ] Colisiones más avanzadas

---

## 📦 BACKUP

### Google Drive
- **Remote:** `gdrive:shield-project-rydit`
- **Tamaño:** ~2 MB (sin target/, con binarios)
- **Última sync:** 2026-03-23
- **Comando:** `./backup_con_binarios.sh`

### Estructura
```
shield-project-rydit/
├── crates/              ✅
├── demos/               ✅
├── docs/                ✅
├── *.md                 ✅
├── *.sh                 ✅ (backup scripts)
├── *.rydit              ✅
└── binarios/
    ├── rydit-rs         (870 KB)
    └── snake            (512 KB)
```

---

## 🙏 AGRADECIMIENTOS

- **Comunidad Mouredev** - Discord: https://discord.gg/mouredev
- **raylib** - https://www.raylib.com/
- **Rust** - https://www.rust-lang.org/
- **Termux** - Desarrollo en Android sin root
- **Termux-X11** - Renderizado gráfico en Android
- **Mesa/Zink** - OpenGL sobre Vulkan para GPU

---

<div align="center">

## 🛡️ **RyDit v0.5.1 - Funciones Assets + Renderizado X11**

**"De pantalla negra a 60 FPS estables con sprites"**

---

*Versión:* v0.5.1 ✅
*Tests:* 124 passing ✅
*FPS:* 60 estables ✅
*Sprites:* 3 cargados ✅
*Backup:* Automatizado ✅

[⬆️ Volver arriba](#-changelog-v051---rydit-engine)

</div>
