# 🎨 migui - Immediate Mode GUI para RyDit

**Versión:** v0.4.0 (Planificada)
**Estilo:** Immediate Mode (raygui-inspired)
**Implementación:** ~1000 líneas Rust en rydit-gfx
**Tiempo estimado:** 1-2 semanas

---

## 🎯 Filosofía

```
✅ Immediate mode (sin estado complejo)
✅ Una función = dibuja + retorna estado
✅ Sin layout engine complejo
✅ Sin event system complejo
✅ Fácil de usar: onif gui.button() {}
```

---

## 📋 Componentes

| Componente | Firma | Retorna | Descripción |
|------------|-------|---------|-------------|
| `button` | `(x, y, w, h, texto)` | `bool` | True si clicked este frame |
| `label` | `(x, y, texto)` | `void` | Solo dibuja texto |
| `checkbox` | `(x, y, w, h, texto, estado)` | `bool` | Estado actual del checkbox |
| `slider` | `(x, y, w, h, min, max, valor, label)` | `num` | Valor actual del slider |
| `textbox` | `(x, y, w, h, texto, placeholder)` | `string` | Texto actual del input |
| `panel` | `(x, y, w, h, titulo)` | `void` | Panel con border y título |
| `window` | `(x, y, w, h, titulo, cerrada)` | `bool` | True si ventana abierta |
| `message_box` | `(mensaje, btn1, btn2)` | `bool` | True si aceptó (btn1) |
| `dropdown` | `(x, y, w, h, opciones, seleccionado)` | `num` | Índice seleccionado |

---

## 💡 Ejemplo de Uso

```rydit
import gui

shield.init 800, 600, "Mi Juego"

dark.slot fullscreen = false
dark.slot volumen = 50
dark.slot nombre = ""

ryda !window_should_close() {
    # ===== MENÚ PRINCIPAL =====
    onif gui.button(300, 200, 200, 50, "Nuevo Juego") {
        iniciar_juego()
    }
    
    onif gui.button(300, 270, 200, 50, "Opciones") {
        mostrar_opciones = true
    }
    
    onif gui.button(300, 340, 200, 50, "Salir") {
        break
    }
    
    # ===== OPCIONES =====
    onif mostrar_opciones {
        gui.panel(200, 150, 400, 300, "Opciones")
        
        fullscreen = gui.checkbox(220, 190, 360, 30, "Fullscreen", fullscreen)
        volumen = gui.slider(220, 240, 360, 30, 0, 100, volumen, "Volumen")
        nombre = gui.textbox(220, 290, 360, 30, nombre, "Nombre")
        
        onif gui.button(350, 380, 100, 40, "Guardar") {
            guardar_opciones()
            mostrar_opciones = false
        }
    }
    
    gui.render()
}
```

---

## 🔧 Implementación Técnica

### En rydit-gfx/src/lib.rs

```rust
pub struct MiguiContext {
    hovered_id: Option<u32>,
    active_id: Option<u32>,
    mouse_pos: (i32, i32),
    mouse_pressed: bool,
}

impl MiguiContext {
    pub fn button(&mut self, x: i32, y: i32, w: i32, h: i32, text: &str) -> bool {
        let id = hash_rect(x, y, w, h);
        let hovered = self.point_in_rect(x, y, w, h);
        let clicked = hovered && self.mouse_pressed;
        
        // Draw button
        self.draw_rect(x, y, w, h, COLOR_BUTTON);
        self.draw_text(text, x + 5, y + 10);
        
        clicked
    }
    
    // ... más componentes
}
```

### Bindings en main.rs

```rust
if name == "gui::button" && args.len() == 5 {
    let x = get_num(&args[0]) as i32;
    let y = get_num(&args[1]) as i32;
    let w = get_num(&args[2]) as i32;
    let h = get_num(&args[3]) as i32;
    let text = get_str(&args[4]);
    
    let clicked = migui_ctx.button(x, y, w, h, text);
    return Valor::Bool(clicked);
}
```

---

## 📊 Métricas

| Métrica | Valor |
|---------|-------|
| Líneas Rust estimadas | ~1000 |
| Componentes | 9 |
| Tiempo implementación | 1-2 semanas |
| Tests estimados | 20+ |
| Dependencias | 0 (en rydit-gfx) |

---

## 🎯 Casos de Uso

1. **Menús de juego** (main menu, pause, options)
2. **Inventarios** (grids de items)
3. **Editores de niveles** (toolbars, properties)
4. **Consolas de debug** (logs, commands)
5. **Dialogos** (save/load, confirmations)

---

## 📝 Notas

- **No es retained mode** → sin gestión de ciclo de vida
- **No es layout engine** → posiciones manuales
- **No es event system** → retorna valores por frame
- **Sí es suficiente** para 90% de casos de uso

---

**"Construido con ❤️ para RyDit"**

*v0.4.0 - migui - Immediate Mode GUI*
