# 🧊 INFORME: Patrón Correcto para 3D con ry3d-gfx

**Versión**: v0.18.0
**Fecha**: 2026-04-11
**Crate**: ry3d-gfx v0.1.0
**Estado**: ✅ Funcional — 15 primitivas 3D + draw_model/ex

---

## El Patrón Correcto

Este es el patrón oficial para compilar y dibujar correctamente en 3D con ry3d-gfx.

```rust
// imports
use raylib::prelude::*;
use ry3d_gfx::DrawHandle3D;
use ry3d_gfx::touch_controls::TouchControls;
use ry_gfx::ColorRydit;

fn main() -> Result<(), String> {
    // 1. Iniciar raylib (ventana + contexto GL)
    let (mut rl, thread) = raylib::init()
        .size(900, 600)
        .title("Mi Demo 3D")
        .build();

    // 2. Crear cámara con Camera3D::perspective()
    let camera = Camera3D::perspective(
        Vector3::new(cam_x, cam_y, cam_z),
        target,
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    // 3. Game loop
    while !rl.window_should_close() {
        // Input (mouse, touch, teclado)
        let touching = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        let touch_pos = rl.get_mouse_position();

        // 4. Render
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // 5. Modo 3D con DrawHandle3D
        let mut h3d = DrawHandle3D::new(&camera);
        h3d.clear_3d(ColorRydit::Negro);
        h3d.draw_grid_3d(20, 1.0);
        h3d.draw_cube_3d((0.0, 1.0, 0.0), (2.0, 2.0, 2.0), ColorRydit::Rojo);
        // ... más primitivas 3D
        drop(h3d); // Fin modo 3D

        // 6. Touch controls overlay (2D encima de 3D)
        // touch.draw();

        // 7. HUD 2D
        d.draw_text("HUD", 10, 10, 20, Color::WHITE);
    }
    Ok(())
}
```

---

## Lecciones Críticas

| # | Regla | Explicación |
|---|-------|-------------|
| 1 | **NO usar `gl::Clear()` directamente** | Causa `gl function was not loaded` — usar `h3d.clear_3d()` |
| 2 | **NO usar `d.begin_mode_3d()`** | API incorrecta — usar `DrawHandle3D::new(&camera)` |
| 3 | **NO importar `raylib::prelude::*` en ry3d-gfx** | Causa conflicto con re-exports |
| 4 | **TouchControls DESPUÉS de `drop(h3d)`** | 2D sobre 3D, no al revés |
| 5 | **`MOUSE_BUTTON_LEFT` no `MOUSE_LEFT_BUTTON`** | Nombre correcto del enum en raylib |

---

## Errores Comunes y Soluciones

| Error | Causa | Solución |
|-------|-------|----------|
| `gl function was not loaded` | `gl::Clear()` sin contexto GL activo | Usar `h3d.clear_3d()` |
| `no method named end_mode_3d` | API incorrecta para modo 3D | `DrawHandle3D::new()` hace Begin/End automático |
| `Camera3D reimported` | Wildcard imports en ry3d-gfx | Importar tipos específicos |
| `MOUSE_LEFT_BUTTON not found` | Nombre incorrecto del enum | Usar `MOUSE_BUTTON_LEFT` |

---

## Flujo de Renderizado 3D

```
┌─────────────────────────────────────────┐
│ 1. raylib::init() → ventana + contexto  │
│ 2. Camera3D::perspective() → cámara     │
│ 3. Game loop                            │
│    ├─ Input (mouse, touch, teclado)     │
│    ├─ rl.begin_drawing() → clear 2D     │
│    ├─ DrawHandle3D::new(&camera)        │
│    │   ├─ clear_3d()                    │
│    │   ├─ draw_grid_3d()                │
│    │   ├─ draw_cube_3d()                │
│    │   ├─ draw_sphere_3d()              │
│    │   └─ ... más primitivas            │
│    ├─ drop(h3d) → fin modo 3D           │
│    ├─ TouchControls.draw() → 2D overlay │
│    └─ HUD 2D (texto, barras, etc.)      │
└─────────────────────────────────────────┘
```

---

## Primitivas 3D Disponibles

| Función | Descripción |
|---------|-------------|
| `clear_3d()` | Limpiar buffer 3D con color |
| `draw_grid_3d()` | Grid de referencia (suelo) |
| `draw_cube_3d()` | Cubo con posición, tamaño, color |
| `draw_sphere_3d()` | Esfera con posición, radio, color |
| `draw_cylinder_3d()` | Cilindro con posición, radio, altura, color |
| `draw_line_3d()` | Línea 3D entre dos puntos |
| `draw_axes_3d()` | Ejes X, Y, Z con colores |
| `draw_bbox_3d()` | Bounding box con posición y tamaño |
| `draw_model()` | Modelo 3D (FFI real) |
| `draw_model_ex()` | Modelo con rotación y escala (FFI real) |

---

## TouchControls (Joysticks Virtuales)

Los TouchControls son como los de RayGunz — joysticks virtuales + botones táctiles.

```rust
let mut touch = TouchControls::new(900, 600);
// ... después de drop(h3d)
touch.draw(&mut d);
```

Se dibuja **DESPUÉS** de `drop(h3d)` porque es 2D overlay sobre la escena 3D.

---

## Cargo.toml

```toml
[dependencies]
ry3d-gfx = { path = "crates/ry3d-gfx" }
ry-gfx = { path = "crates/ry-gfx" }
raylib = "5.0"
```

---

## Demos de Referencia

| Demo | Descripción |
|------|-------------|
| `demo_3d_primitives` | Escena 3D con cubos, esferas, cilindros, grid, ejes |
| `demo_3d_touch` | Escena 3D con TouchControls (joysticks virtuales) |
| `demo_militar` | Soldado procedural + partículas + granadas en arco |

---

<div align="center">

**ry3d-gfx v0.1.0 — Patrón Correcto para 3D**

*15 primitivas 3D · draw_model/ex FFI · DrawHandle3D RAII · TouchControls*

**Regla de oro: DrawHandle3D::new() para 3D, drop() para terminar, 2D después**

</div>
