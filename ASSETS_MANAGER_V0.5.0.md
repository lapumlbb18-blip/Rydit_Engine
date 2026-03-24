# 🎨 ASSETS MANAGER v0.5.0 - Documentación Técnica

**Fecha:** 2026-03-23  
**Estado:** ✅ Implementado (Rust)  
**Próximo:** Funciones RyDit (v0.5.1)

---

## 📊 ARQUITECTURA ACTUAL

### **Assets Manager en `rydit-gfx`**

```rust
// crates/rydit-gfx/src/lib.rs

pub struct Assets {
    textures: HashMap<String, Texture2D>,
}

impl Assets {
    pub fn new() -> Self { ... }
    pub fn insert_texture(&mut self, id: String, texture: Texture2D) { ... }
    pub fn get_texture(&self, id: &str) -> Option<&Texture2D> { ... }
    pub fn unload_texture(&mut self, id: &str) -> bool { ... }
    pub fn draw_texture(&self, d: &mut RaylibDrawHandle, id: &str, x: f32, y: f32, scale: f32, color: Color) { ... }
    pub fn draw_texture_rec(&self, d: &mut RaylibDrawHandle, id: &str, source: Rectangle, dest: Rectangle, color: Color) { ... }
}
```

---

## 🔧 IMPLEMENTACIÓN PENDIENTE (v0.5.1)

### **Funciones RyDit Necesarias**

```rydit
# Cargar textura
assets::load_texture("tank", "logo_icon_asst/sprites/tank_16x16.png")

# Dibujar textura
assets::draw("tank", x, y, 1.0)  # x, y, escala

# Dibujar textura escalada
assets::draw_scaled("tank", x, y, 2.0)  # 2x scale

# Verificar si existe
si assets::has("tank") {
    voz "Tank cargado"
}

# Descargar
assets::unload("tank")
```

### **Implementación en Rust (pendiente)**

```rust
// crates/rydit-rs/src/main.rs
// En evaluar_expr_migui() o funciones globales:

// assets::load_texture(id, path)
if name == "assets::load_texture" && args.len() == 2 {
    if let (Valor::Texto(id), Valor::Texto(path)) = (...) {
        // Cargar textura con raylib
        let image = raylib::load_image(&path)?;
        let texture = image.load_texture_from_image();
        assets.insert_texture(id, texture);
        return Valor::Vacio;
    }
}

// assets::draw(id, x, y, scale)
if name == "assets::draw" && args.len() == 4 {
    if let (Valor::Texto(id), Valor::Num(x), Valor::Num(y), Valor::Num(scale)) = (...) {
        // Dibujar en el game loop
        // Necesitamos acceso a Assets y DrawHandle
        return Valor::Vacio;
    }
}
```

---

## 📁 SPRITES DISPONIBLES

En `logo_icon_asst/sprites/`:

| Sprite | Tamaño | Descripción |
|--------|--------|-------------|
| `tank_16x16.png` | 16x16 | Tanque top-down |
| `helicopter_16x16.png` | 16x16 | Helicóptero con rotores |
| `platform_16x16.png` | 16x16 | Plataforma/suelo |
| `crate_8x8.png` | 8x8 | Caja de madera |
| `cube_8x8.png` | 8x8 | Cubo de piedra |

---

## 🎮 EJEMPLO DE USO FUTURO (v0.5.1)

```rydit
shield.init

# Cargar sprites al inicio
assets::load_texture("tank", "logo_icon_asst/sprites/tank_16x16.png")
assets::load_texture("helicopter", "logo_icon_asst/sprites/helicopter_16x16.png")
assets::load_texture("crate", "logo_icon_asst/sprites/crate_8x8.png")

dark.slot tank_x = 400
dark.slot tank_y = 300

cada frame en frames {
    gfx::clear("negro")
    
    # Input
    si input::tecla_presionada("derecha") {
        dark.slot tank_x = tank_x + 2
    }
    si input::tecla_presionada("izquierda") {
        dark.slot tank_x = tank_x - 2
    }
    
    # Dibujar sprite del tank
    assets::draw("tank", tank_x, tank_y, 2.0)  # 2x scale
    
    # Dibujar cajas decorativas
    assets::draw("crate", 100, 200, 1.0)
    assets::draw("crate", 150, 200, 1.0)
    
    migui::end_frame()
}
```

---

## ⚠️ LIMITACIONES ACTUALES

1. **Audio no disponible** - raylib nobuild no tiene funciones de audio
2. **Carga desde RyDit pendiente** - Assets está en Rust, falta exponer a RyDit
3. **Tilesets no implementados** - Solo texturas completas

---

## 🔜 PRÓXIMOS PASOS (v0.5.1)

1. **Funciones RyDit** - `assets::load_texture()`, `assets::draw()`
2. **Demo Tank con Sprites** - Reemplazar círculos por sprites
3. **Tilesets** - Soporte para spritesheets
4. **Audio** - Cuando raylib nobuild tenga audio

---

<div align="center">

## 🛡️ **Assets Manager v0.5.0 - Base Lista**

**"Rust implementado, RyDit pendiente"**

---

*Assets struct:* ✅ Rust  
*Funciones RyDit:* 🔜 v0.5.1  
*Sprites disponibles:* 5 ✅  

[⬆️ Volver arriba](#-assets-manager-v050---documentación-técnica)

</div>
