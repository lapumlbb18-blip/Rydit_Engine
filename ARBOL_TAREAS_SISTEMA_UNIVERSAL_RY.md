# 🛡️ SISTEMA UNIVERSAL RY - ÁRBOL DE TAREAS v0.9.0

**Objetivo**: Implementar features tipo RPG/Platformer/Roguelike
**Tiempo estimado**: 5-7 días (ritmo rápido demostrado)
**Prioridad**: Implementar → Testear → Documentar

---

## 🌳 ÁRBOL DE TAREAS

```
SISTEMA UNIVERSAL RY v0.9.0
│
├── 📸 FASE 1: CÁMARA 2D (1-2 días)
│   ├── 1.1 Camera2D struct
│   │   ├── x, y (posición)
│   │   ├── zoom (escala)
│   │   ├── rotation (rotación)
│   │   ├── target_x, target_y (seguimiento)
│   │   └── smooth (suavizado)
│   │
│   ├── 1.2 Funciones básicas
│   │   ├── camera::set_position(x, y)
│   │   ├── camera::get_position() → (x, y)
│   │   ├── camera::set_zoom(level)
│   │   ├── camera::get_zoom() → level
│   │   ├── camera::set_rotation(angle)
│   │   └── camera::get_rotation() → angle
│   │
│   ├── 1.3 Scroll
│   │   ├── camera::scroll(dx, dy)
│   │   ├── camera::scroll_to(x, y)
│   │   └── camera::limit_bounds(min_x, min_y, max_x, max_y)
│   │
│   ├── 1.4 Seguimiento del jugador
│   │   ├── camera::follow(target_x, target_y)
│   │   ├── camera::follow_smooth(target_x, target_y, smooth)
│   │   ├── camera::follow_lerp(target_x, target_y, speed)
│   │   └── camera::set_follow_offset(offset_x, offset_y)
│   │
│   └── 1.5 Integración con draw
│       ├── camera::apply() (transforma coordenadas)
│       ├── camera::world_to_screen(wx, wy) → (sx, sy)
│       └── camera::screen_to_world(sx, sy) → (wx, wy)
│
├── 🎮 FASE 2: JUGADOR/CARÁCTER (1 día)
│   ├── 2.1 Player struct
│   │   ├── x, y (posición)
│   │   ├── vx, vy (velocidad)
│   │   ├── width, height (dimensiones)
│   │   ├── speed (velocidad movimiento)
│   │   ├── jump_force (fuerza salto)
│   │   ├── is_grounded (en suelo)
│   │   └── is_jumping (saltando)
│   │
│   ├── 2.2 Movimiento platformer
│   │   ├── player::move_left()
│   │   ├── player::move_right()
│   │   ├── player::move_up()
│   │   ├── player::move_down()
│   │   ├── player::jump()
│   │   ├── player::apply_gravity()
│   │   └── player::update(dt)
│   │
│   ├── 2.3 Estados del jugador
│   │   ├── player::set_state("idle", "run", "jump", "fall")
│   │   ├── player::get_state() → "state"
│   │   ├── player::is_idle() → bool
│   │   ├── player::is_running() → bool
│   │   ├── player::is_jumping() → bool
│   │   └── player::is_falling() → bool
│   │
│   └── 2.4 Animación básica
│       ├── player::set_sprite(id)
│       ├── player::set_flip_x(flip)
│       └── player::set_flip_y(flip)
│
├── 🗺️ FASE 3: GESTIÓN DE NIVELES (1 día)
│   ├── 3.1 Level manager
│   │   ├── level::load("nivel.rydit")
│   │   ├── level::unload()
│   │   ├── level::transition("nivel2.rydit")
│   │   ├── level::get_current() → "nivel"
│   │   ├── level::reload()
│   │   └── level::get_name() → "nombre"
│   │
│   ├── 3.2 Tilemap básico
│   │   ├── tilemap::load("tiles.png", tile_size)
│   │   ├── tilemap::set_tile(x, y, tile_id)
│   │   ├── tilemap::get_tile(x, y) → tile_id
│   │   ├── tilemap::draw()
│   │   └── tilemap::get_size() → (width, height)
│   │
│   ├── 3.3 Checkpoints
│   │   ├── level::set_checkpoint(name, x, y)
│   │   ├── level::load_checkpoint(name)
│   │   ├── level::get_checkpoint(name) → (x, y)
│   │   └── level::list_checkpoints() → [names]
│   │
│   └── 3.4 Transiciones
│       ├── level::transition_fade(duration)
│       ├── level::transition_slide(direction)
│       └── level::set_transition_callback(callback)
│
├── 🪟 FASE 4: GESTIÓN DE VENTANA (0.5 días)
│   ├── 4.1 Configuración
│   │   ├── window::set_title(title)
│   │   ├── window::set_size(width, height)
│   │   ├── window::get_size() → (w, h)
│   │   ├── window::get_width() → w
│   │   └── window::get_height() → h
│   │
│   ├── 4.2 Modos de pantalla
│   │   ├── window::set_fullscreen(enabled)
│   │   ├── window::is_fullscreen() → bool
│   │   ├── window::toggle_fullscreen()
│   │   └── window::set_windowed()
│   │
│   ├── 4.3 Comportamiento
│   │   ├── window::set_vsync(enabled)
│   │   ├── window::set_resizable(enabled)
│   │   ├── window::minimize()
│   │   ├── window::maximize()
│   │   └── window::restore()
│   │
│   └── 4.4 FPS
│       ├── window::set_fps_limit(fps)
│       ├── window::get_fps() → fps
│       └── window::get_delta_time() → dt
│
├── ⬡ FASE 5: SISTEMA HEXAGONAL (1 día)
│   ├── 5.1 Grid hexagonal
│   │   ├── hex::grid_create(rows, cols, size)
│   │   ├── hex::get_hex(x, y) → hex_id
│   │   ├── hex::set_hex(x, y, data)
│   │   └── hex::draw()
│   │
│   ├── 5.2 Coordenadas hex
│   │   ├── hex::axial_to_pixel(q, r) → (x, y)
│   │   ├── hex::pixel_to_axial(x, y) → (q, r)
│   │   ├── hex::get_neighbors(q, r) → [(q,r)]
│   │   └── hex::distance(q1, r1, q2, r2) → dist
│   │
│   └── 5.3 Pathfinding hex
│       ├── hex::find_path(start, end) → [(q,r)]
│       └── hex::get_reachable(start, range) → [(q,r)]
│
├── ⚔️ FASE 6: COLISIONES 2D (1 día)
│   ├── 6.1 Detección básica
│   │   ├── collision::check_rect_rect(x1,y1,w1,h1, x2,y2,w2,h2) → bool
│   │   ├── collision::check_circle_circle(x1,y1,r1, x2,y2,r2) → bool
│   │   ├── collision::check_rect_circle(rx,ry,rw,rh, cx,cy,cr) → bool
│   │   └── collision::check_point_rect(px,py, rx,ry,rw,rh) → bool
│   │
│   ├── 6.2 Área2D (Godot-style)
│   │   ├── area2d::create(id, x, y, w, h)
│   │   ├── area2d::set_position(id, x, y)
│   │   ├── area2d::get_position(id) → (x, y)
│   │   ├── area2d::check(id, other_id) → bool
│   │   ├── area2d::get_overlapping(id) → [ids]
│   │   └── area2d::destroy(id)
│   │
│   ├── 6.3 Respuesta a colisiones
│   │   ├── collision::resolve_rect_rect(...) → (overlap_x, overlap_y)
│   │   ├── collision::bounce(obj, normal, force)
│   │   └── collision::slide(obj, normal)
│   │
│   └── 6.4 Spatial hash (optimización)
│       ├── spatial::insert(id, x, y, w, h)
│       ├── spatial::remove(id)
│       ├── spatial::query(x, y, w, h) → [ids]
│       └── spatial::clear()
│
└── 📚 FASE 7: DOCUMENTACIÓN (0.5 días)
    ├── docs/SISTEMA_UNIVERSAL_RY.md
    ├── docs/CAMARA_2D.md
    ├── docs/JUGADOR.md
    ├── docs/NIVELES.md
    ├── docs/VENTANA.md
    ├── docs/HEXAGONAL.md
    ├── docs/COLISIONES.md
    ├── ejemplos/camera_platformer.rydit
    ├── ejemplos/rpg_hexagonal.rydit
    └── ejemplos/roguelike_dungeon.rydit
```

---

## 📊 TIEMPOS ESTIMADOS (Ritmo Rápido)

| Fase | Feature | Tiempo | Prioridad | Dependencias |
|------|---------|--------|-----------|--------------|
| **1** | Cámara 2D | 1-2 días | 🔴 CRÍTICA | Ninguna |
| **2** | Jugador/Carácter | 1 día | 🔴 ALTA | Cámara 2D |
| **3** | Niveles | 1 día | 🟡 ALTA | Ninguna |
| **4** | Ventana | 0.5 días | 🟢 MEDIA | Ninguna |
| **5** | Hexagonal | 1 día | 🟢 MEDIA | Ninguna |
| **6** | Colisiones 2D | 1 día | 🔴 ALTA | Jugador |
| **7** | Documentación | 0.5 días | 🟢 BAJA | Todas |
| **TOTAL** | **6 features** | **5-7 días** | | |

---

## 🎯 ORDEN DE IMPLEMENTACIÓN RECOMENDADO

### **Día 1-2: Cámara 2D** (Cimientos)
```
✅ Camera2D struct
✅ camera::set_position, get_position
✅ camera::set_zoom, get_zoom
✅ camera::follow, follow_smooth
✅ camera::apply (integración con draw)
```

### **Día 3: Jugador + Colisiones Básicas**
```
✅ Player struct (movimiento platformer)
✅ player::move_*, jump, apply_gravity
✅ collision::check_rect_rect
✅ collision::check_point_rect
```

### **Día 4: Gestión de Niveles + Ventana**
```
✅ level::load, unload, transition
✅ tilemap::load, set_tile, draw
✅ window::set_title, set_size, set_fullscreen
```

### **Día 5: Sistema Hexagonal + Colisiones Área2D**
```
✅ hex::grid_create, axial_to_pixel
✅ area2d::create, check, get_overlapping
✅ spatial::insert, query (optimización)
```

### **Día 6: Documentación + Demos**
```
✅ docs/*.md (todas las guías)
✅ ejemplos/camera_platformer.rydit
✅ ejemplos/rpg_hexagonal.rydit
```

---

## 🛡️ FEATURES CLAVE POR TIPO DE JUEGO

### **Platformer 2D**
- ✅ Cámara 2D (seguimiento + smooth)
- ✅ Jugador (movimiento + salto + gravedad)
- ✅ Colisiones rectángulo (tilemap + jugador)
- ✅ Niveles (carga + checkpoints)

### **RPG Hexagonal**
- ✅ Grid hexagonal
- ✅ Pathfinding hex
- ✅ Cámara 2D (zoom + scroll)
- ✅ Área2D (detección unidades)

### **Roguelike Dungeon**
- ✅ Tilemap (dungeon tiles)
- ✅ Niveles (transición entre pisos)
- ✅ Colisiones (paredes + items)
- ✅ Cámara 2D (follow jugador)

---

## 📝 NOTAS DE IMPLEMENTACIÓN

### **Cámara 2D - Detalles Técnicos**
- Usar `raylib::Camera2D` de FFI o implementar propio
- Transformaciones: traslación → rotación → escala
- `apply()` debe llamarse ANTES de cualquier draw call
- `world_to_screen()` útil para UI sobre juego

### **Jugador - Física Simplificada**
- Gravedad constante: `vy += gravity * dt`
- Salto: `vy = -jump_force` (solo si `is_grounded`)
- Colisión con suelo: `is_grounded = (y >= ground_y)`

### **Hexagonal - Coordinate Systems**
- Usar coordenadas axiales (q, r) internamente
- Conversión a pixel: `x = size * (q + r/2)`, `y = size * r * sqrt(3)/2`
- Neighbors: 6 direcciones precalculadas

### **Colisiones - Optimización**
- Spatial hash para muchos objetos
- Broad phase (spatial) → Narrow phase (check preciso)
- Área2D con signals/callbacks cuando hay overlap

---

## ✅ CRITERIOS DE ACEPTACIÓN

### **Cámara 2D**
- [ ] `camera::follow()` sigue al jugador suavemente
- [ ] Zoom funciona sin romper coordenadas
- [ ] Límites de scroll respetados
- [ ] Demo platformer funcionando

### **Jugador**
- [ ] Movimiento izquierda/derecha fluido
- [ ] Salto con gravedad natural
- [ ] Detección de suelo precisa
- [ ] Estados (idle/run/jump) correctos

### **Niveles**
- [ ] Carga de tilemap desde archivo
- [ ] Transición entre niveles sin crash
- [ ] Checkpoints guardan posición
- [ ] Demo con 2+ niveles conectados

### **Hexagonal**
- [ ] Grid se dibuja correctamente
- [ ] Pathfinding encuentra ruta más corta
- [ ] Conversión pixel↔axial precisa
- [ ] Demo RPG hexagonal funcionando

### **Colisiones**
- [ ] Detección rect-rect 100% precisa
- [ ] Área2D detecta overlaps
- [ ] Spatial hash reduce chequeos
- [ ] Demo con 50+ objetos colisionando

---

<div align="center">

**🛡️ SISTEMA UNIVERSAL RY v0.9.0**

*6 features principales | 5-7 días | RPG/Platformer/Roguelike ready*

**Inicio: Día 1 → Cámara 2D**

</div>
