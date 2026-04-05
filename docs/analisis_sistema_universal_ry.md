# 🎮 Sistema Universal Ry — Análisis Completo del Motor de Juegos

**Fecha**: 2026-04-04
**Versión**: v0.13.0
**Estado**: Análisis del sistema de videojuegos completo

---

## 🎯 VISIÓN GENERAL

El **Sistema Universal Ry** es el conjunto de módulos, crates y sistemas que conforman el motor de juegos Ry-Dit. No es un solo crate — es la **integración de todos los componentes** trabajando juntos.

```
Sistema Universal Ry = Módulos ry-rs + Crates especializados + UI (migui/toolkit) + Streaming
```

---

## 📊 ARQUITECTURA DE MÓDULOS (ry-rs/src/modules/)

### Módulos Activos (13 módulos)

| Módulo | Líneas | Funcionalidad | Estado |
|--------|--------|---------------|--------|
| **assets.rs** | ~520 | Carga/dibujo de sprites PNG | ✅ Funcional |
| **audio.rs** | ~430 | SDL2_mixer (sonidos, música) | ✅ Funcional |
| **camera.rs** | ~530 | Camera2D (posición, zoom, rotación, bounds) | ✅ Funcional |
| **collision.rs** | ~460 | Area2D, rect/circle collision, raycast | ✅ Funcional |
| **csv.rs** | ~930 | CSV parser + queries | ✅ Funcional |
| **entity.rs** | ~2730 | Sistema de entidades (posición, vida, stats) | ✅ Funcional |
| **input_ime.rs** | ~270 | Input texto IME (teclado virtual) | ✅ Funcional |
| **input_map.rs** | ~475 | Mapeo de inputs (teclado → acciones) | ✅ Funcional |
| **level.rs** | ~593 | Gestión de niveles + checkpoints + transiciones | ✅ Funcional |
| **particles.rs** | ~190 | Bindings .rydit → ry-gfx particles | ✅ Funcional |
| **physics.rs** | ~520 | PhysicsBody + PhysicsWorld (simulación real-time) | ✅ Funcional |
| **tilemap.rs** | ~582 | Tilemap system (multi-capa, tilesets) | ✅ Funcional |
| **window.rs** | ~290 | Creación/configuración ventana SDL2 | ✅ Funcional |

**Total líneas módulos**: ~8,520 líneas de código funcional.

---

## 🗺️ ANÁLISIS POR SISTEMA

### 1️⃣ GESTIÓN DE NIVELES (level.rs — 593 líneas)

| Función | Implementado | Descripción |
|---------|-------------|-------------|
| `level::load(name, path)` | ✅ | Cargar nivel desde archivo .rydit |
| `level::unload()` | ✅ | Descargar nivel actual |
| `level::transition(next)` | ✅ | Transición a siguiente nivel |
| `level::reload()` | ✅ | Recargar nivel actual |
| `level::get_current()` | ✅ | Obtener nombre nivel actual |
| `level::get_name()` | ✅ | Obtener metadata del nivel |
| `level::set_checkpoint(name, x, y)` | ✅ | Registrar checkpoint |
| `level::load_checkpoint(name)` | ✅ | Ir a checkpoint |
| `level::get_checkpoint(name)` | ✅ | Obtener posición checkpoint |
| `level::list_checkpoints()` | ✅ | Listar todos los checkpoints |
| `level::transition_fade(duration)` | ✅ | Transición fade in/out |
| `level::transition_slide(dir, duration)` | ✅ | Transición slide (directional) |

**Metadata de niveles soportada:**
```rydit
# @nombre "Nivel 1"
# @musica "tema1.mp3"
# @fondo "cielo.png"
```

**❌ LO QUE FALTA:**

| Feature | Descripción | Prioridad |
|---------|-------------|-----------|
| **Quests/Misiones** | Sistema de objetivos, progreso, recompensas | 🔴 Alta |
| **Mundos múltiples** | Hub world → sub-worlds (como Zelda) | 🟡 Media |
| **Niveles hexagonales** | Tilemap hexagonal (no solo rectangular) | 🟡 Media |
| **Niveles 1v1** | Arena PvP con 2 jugadores | 🟡 Media |
| **Área3D** | Saltar de nivel a mapa 3D | 🔮 Futuro |
| **Nivel procedural** | Generación aleatoria de niveles | 🔮 Futuro |
| **Guardado/carga** | Save game completo (no solo checkpoints) | 🔴 Alta |

---

### 2️⃣ SEGUIMIENTO DE CÁMARA (camera.rs — 530 líneas)

| Función | Implementado | Descripción |
|---------|-------------|-------------|
| `camera::set_position(x, y)` | ✅ | Mover cámara |
| `camera::follow(entity_id)` | ✅ | Seguir entidad |
| `camera::set_zoom(zoom)` | ✅ | Zoom in/out |
| `camera::set_rotation(angle)` | ✅ | Rotar cámara |
| `camera::set_bounds(x, y, w, h)` | ✅ | Límites de cámara |
| `camera::get_screen_pos(wx, wy)` | ✅ | World → Screen |
| `camera::get_world_pos(sx, sy)` | ✅ | Screen → World |
| `camera::shake(intensity, duration)` | ✅ | Screen shake |
| `camera::lerp_to(x, y, speed)` | ✅ | Movimiento suave |
| `camera::apply()` | ✅ | Aplicar al render |

**Funciones internas (Rust):**
- `get_transform_matrix()` → Matriz de transformación 2D
- `world_to_screen()` → Conversión coordenadas
- `screen_to_world()` → Conversión inversa

**❌ LO QUE FALTA:**

| Feature | Descripción | Prioridad |
|---------|-------------|-----------|
| **Camera path** | Ruta predefinida (cinemáticas) | 🟡 Media |
| **Multi-camera** | Split-screen para 1v1 | 🟡 Media |
| **Camera3D** | Seguimiento en 3D | 🔮 v0.14.0 |
| **Dead zone** | Zona muerta (no seguir exactamente) | 🟢 Baja |
| **Look ahead** | Predecir dirección del jugador | 🟢 Baja |

---

### 3️⃣ MUNDOS PLATAFORMA (physics.rs + tilemap.rs + collision.rs)

#### Physics (520 líneas)
| Función | Estado | Descripción |
|---------|--------|-------------|
| PhysicsBody (x, y, vx, vy, w, h) | ✅ | Cuerpo físico básico |
| PhysicsWorld (gravedad, fricción) | ✅ | Mundo con parámetros globales |
| AABB collision | ✅ | Detección de colisiones |
| Bounds checking | ✅ | Límites del mundo |
| Gravity simulation | ✅ | Gravedad aplicada |
| `physics::create_body()` | ✅ | Crear cuerpo desde .rydit |
| `physics::update(dt)` | ✅ | Actualizar simulación |
| `physics::get_overlap()` | ✅ | Obtener solapamiento |

#### Tilemap (582 líneas)
| Función | Estado | Descripción |
|---------|--------|-------------|
| `tilemap::create(w, h, tile_size)` | ✅ | Crear tilemap vacío |
| `tilemap::set_tile(x, y, id)` | ✅ | Colocar tile |
| `tilemap::get_tile(x, y)` | ✅ | Obtener tile |
| `tilemap::draw()` | ✅ | Dibujar tilemap completo |
| `tilemap::draw_layer(layer)` | ✅ | Dibujar capa específica |
| `tilemap::set_tileset(path)` | ✅ | Cambiar tileset (imagen) |
| `tilemap::get_size()` | ✅ | Obtener dimensiones |
| `tilemap::fill_rect(x, y, w, h, id)` | ✅ | Llenar rectángulo |
| `tilemap::set_layer_count(n)` | ✅ | Múltiples capas |
| `tilemap::clear()` | ✅ | Limpiar tilemap |

#### Collision (460 líneas)
| Función | Estado | Descripción |
|---------|--------|-------------|
| Area2D (rect, circle) | ✅ | Tipos de área |
| `collision::check(a, b)` | ✅ | Detectar colisión |
| `collision::resolve(a, b)` | ✅ | Resolver solapamiento |
| `collision::raycast(origin, dir, max_dist)` | ✅ | Raycast 2D |
| AABB overlap | ✅ | Rectángulo vs rectángulo |
| Circle overlap | ✅ | Círculo vs círculo |
| Rect vs Circle | ✅ | Mixto |

**❌ LO QUE FALTA para plataformas completo:**

| Feature | Descripción | Prioridad |
|---------|-------------|-----------|
| **One-way platforms** | Plataformas que solo se suben desde abajo | 🔴 Alta |
| **Moving platforms** | Plataformas que se mueven | 🟡 Media |
| **Slopes** | Pendientes con ángulo | 🟡 Media |
| **Tilemap hexagonal** | Grid hexagonal (no rectangular) | 🟡 Media |
| **Parallax backgrounds** | Fondos con múltiples capas | 🟢 Baja |
| **Lava/spikes** | Tiles dañinos | 🟢 Baja |

---

### 4️⃣ SISTEMA DE ENTIDADES (entity.rs — 2,730 líneas)

El módulo más grande del proyecto.

| Función | Estado | Descripción |
|---------|--------|-------------|
| `entity::create(name)` | ✅ | Crear entidad |
| `entity::set_pos(id, x, y)` | ✅ | Posición |
| `entity::get_pos(id)` | ✅ | Obtener posición |
| `entity::set_vel(id, vx, vy)` | ✅ | Velocidad |
| `entity::set_sprite(id, sprite)` | ✅ | Asignar sprite |
| `entity::set_color(id, color)` | ✅ | Color (sin sprite) |
| `entity::set_size(id, w, h)` | ✅ | Tamaño |
| `entity::set_health(id, hp)` | ✅ | Vida/HP |
| `entity::damage(id, amount)` | ✅ | Recibir daño |
| `entity::heal(id, amount)` | ✅ | Curar |
| `entity::destroy(id)` | ✅ | Eliminar entidad |
| `entity::get_all()` | ✅ | Listar entidades |
| `entity::get_nearby(x, y, radius)` | ✅ | Entidades cercanas |
| `entity::query(condition)` | ✅ | Query con condiciones |
| `entity::set_gravity(id, g)` | ✅ | Gravedad individual |
| `entity::set_friction(id, f)` | ✅ | Fricción individual |
| `entity::set_bounce(id, b)` | ✅ | Rebote |
| `entity::set_solid(id)` | ✅ | Sólido (colisiona) |
| `entity::set_trigger(id)` | ✅ | Trigger (no colisiona, detecta) |
| `entity::on_collision(id, callback)` | ⚠️ Parcial | Callback de colisión |
| `entity::on_destroy(id, callback)` | ⚠️ Parcial | Callback de destrucción |

**❌ LO QUE FALTA:**

| Feature | Descripción | Prioridad |
|---------|-------------|-----------|
| **Quests/Objetivos** | Sistema de misiones por entidad | 🔴 Alta |
| **Inventario** | Items por entidad | 🔴 Alta |
| **Skills/Abilities** | Habilidades especiales | 🟡 Media |
| **AI básica** | Comportamiento automático | 🟡 Media |
| **Equipamiento** | Armas, armaduras | 🟡 Media |
| **Stats avanzados** | STR, DEX, INT, etc. | 🟡 Media |
| **Niveles/XP** | Sistema de progresión | 🟡 Media |

---

### 5️⃣ INPUT SYSTEM (input_map.rs + input_ime.rs)

#### Input Map (475 líneas)
| Función | Estado | Descripción |
|---------|--------|-------------|
| `input_map::register(action, key)` | ✅ | Mapear tecla → acción |
| `input_map::is_pressed(action)` | ✅ | Verificar acción |
| `input_map::unregister(action)` | ✅ | Desmapear |
| `input_map::list()` | ✅ | Listar mapeos |

#### Input IME (270 líneas)
| Función | Estado | Descripción |
|---------|--------|-------------|
| IME text input | ✅ | Teclado virtual Android |
| Text composition | ✅ | Composición de texto |

**❌ LO QUE FALTA:**

| Feature | Prioridad |
|---------|-----------|
| **Gamepad support** | 🟡 Media |
| **Touch gestures** | 🟡 Media |
| **Input buffering** | 🟢 Baja |
| **Combo detection** | 🟢 Baja |

---

### 6️⃣ AUDIO (audio.rs — 430 líneas)

| Función | Estado | Descripción |
|---------|--------|-------------|
| `audio::load(name, path)` | ✅ | Cargar sonido WAV |
| `audio::play(name)` | ✅ | Reproducir |
| `audio::stop(name)` | ✅ | Detener |
| `audio::set_volume(name, vol)` | ✅ | Volumen |
| `audio::play_music(path)` | ✅ | Música de fondo |
| `audio::stop_music()` | ✅ | Detener música |
| `audio::list()` | ✅ | Listar sonidos |

**❌ LO QUE FALTA:**

| Feature | Prioridad |
|---------|-----------|
| **3D spatial audio** | 🟡 Media |
| **Music looping** | 🟢 Baja |
| **Audio filters** | 🔮 Futuro |
| **Procedural audio** | 🔮 Futuro |

---

## 🎨 SISTEMA UI — migui + toolkit-ry

### migui (Immediate Mode GUI) — ~1,230 líneas

| Widget | Estado | Descripción |
|--------|--------|-------------|
| **Button** | ✅ | Botón clickeable |
| **Label** | ✅ | Texto estático |
| **Checkbox** | ✅ | Toggle on/off |
| **Slider** | ✅ | Valor numérico rango |
| **Textbox** | ✅ | Input de texto |
| **Window** | ✅ | Ventana arrastrable |
| **Dropdown** | ✅ | Menú desplegable |
| **Progress Bar** | ✅ | Barra de progreso |
| **Message Box** | ✅ | Diálogo con botones |
| **Listbox** | ✅ | Lista de items |
| **Panel** | ✅ | Panel de color |
| **Layout (vertical/horizontal)** | ✅ | Auto-layout |

**Backend**: SDL2 (`backend_sdl2.rs`)
**Font**: Native font manager (`font_native.rs`)

### toolkit-ry — ⚠️ ESQUELETO VACÍO

```
crates/toolkit-ry/
├── Cargo.toml     ✅ (SDL2 + ry-gfx deps)
└── src/
    └── lib.rs     ⚠️ Solo re-exports de módulos inexistentes
        ├── widgets  ❌ No existe el archivo
        └── theme    ❌ No existe el archivo
```

**toolkit-ry está declarado pero SIN IMPLEMENTAR.** Los archivos `widgets.rs` y `theme.rs` no existen.

**Visión original:** toolkit-ry sería la capa de alto nivel sobre migui para:
- Menús de juego (main menu, pause, options)
- HUD (health bar, score, minimap)
- Inventarios
- Diálogos de NPCs

**Estado real**: Solo el Cargo.toml existe. Todo el código de UI está en migui directamente.

---

## 🎮 TIPOS DE JUEGOS SOPORTADOS

### ✅ Lo que YA se puede hacer

| Tipo de Juego | Soporte | Ejemplo |
|--------------|---------|---------|
| **Snake** | ✅ Completo | Ya implementado |
| **Platformer 2D** | ✅ 90% | Tilemap + physics + camera |
| **Tank Combat** | ✅ Completo | Ya implementado |
| **Top-down shooter** | ✅ 80% | Entities + collision + input |
| **Puzzle** | ✅ 90% | Tilemap + entity + level |
| **Particle showcase** | ✅ Completo | Ya implementado |
| **Physics sandbox** | ✅ 80% | Physics world + entities |

### ⚠️ Parcialmente soportado

| Tipo | Qué falta | Esfuerzo |
|------|-----------|----------|
| **RPG** | Quests, inventario, diálogos, AI | Alto (40-60h) |
| **Fighter 1v1** | Split-screen, combos, hitboxes | Alto (30-40h) |
| **Hex grid game** | Tilemap hexagonal | Medio (15-20h) |
| **Multiplayer LAN** | ry-stream integration | Medio (10-15h) |
| **Visual novel** | Sistema de diálogos avanzado | Medio (15-20h) |

### ❌ No soportado aún

| Tipo | Qué se necesita | Esfuerzo |
|------|-----------------|----------|
| **3D game** | Camera3D + meshes + lighting | Muy Alto (60-80h) |
| **MMO** | Server dedicado + netcode | Muy Alto (100+h) |
| **Racing** | Physics de vehículos | Alto (40-60h) |
| **Strategy** | Pathfinding + AI avanzada | Alto (40-60h) |

---

## 📊 MATRIZ DE IMPLEMENTACIÓN

| Sistema | Implementado | Pendiente | % Completo |
|---------|-------------|-----------|------------|
| **Niveles** | 12 funciones | Quests, save, mundos | 60% |
| **Cámara** | 10 funciones | Path, multi-camera | 75% |
| **Plataforma** | Physics + tilemap + collision | One-way, slopes, hex | 70% |
| **Entidades** | 20 funciones | Quests, inventario, AI | 55% |
| **Input** | Map + IME | Gamepad, touch | 65% |
| **Audio** | 7 funciones | Spatial, filters | 50% |
| **UI (migui)** | 12 widgets | Theme system | 80% |
| **UI (toolkit)** | 0% | TODO | 0% |
| **Particles** | Completo | — | 95% |
| **Streaming** | Server + client + portal | mDNS, async | 40% |

---

## 🎯 PRIORIDADES PARA COMPLETAR EL SISTEMA

### P0 — Críticos (para juegos completos)

| Feature | Módulos afectados | Esfuerzo | Impacto |
|---------|-------------------|----------|---------|
| **Save/Load game** | level.rs, entity.rs | 8-12h | Permite juegos con progreso |
| **Quest system** | entity.rs nuevo módulo | 15-20h | RPGs, aventuras |
| **One-way platforms** | physics.rs, collision.rs | 4-6h | Platformers proper |
| **toolkit-ry** (HUD/menus) | toolkit-ry (nuevo) | 10-15h | UI de juegos completa |

### P1 — Importantes (mejora significativa)

| Feature | Esfuerzo | Impacto |
|---------|----------|---------|
| Tilemap hexagonal | 15-20h | Juegos de estrategia |
| 1v1 arena | 10-15h | Fighting games |
| Camera path | 6-8h | Cinemáticas |
| Inventory system | 12-16h | RPGs |
| AI básica | 15-20h | NPCs, enemigos |

### P2 — Deseables (premium feel)

| Feature | Esfuerzo | Impacto |
|---------|----------|---------|
| Parallax backgrounds | 4-6h | Visual polish |
| 3D spatial audio | 8-12h | Inmersión |
| Gamepad support | 6-8h | Controllers |
| Procedural levels | 10-15h | Replay value |

---

## 🏗️ ARQUITECTURA PROPUESTA (Sistema Universal Completo)

```
┌──────────────────────────────────────────────────────────────┐
│                    SISTEMA UNIVERSAL RY                      │
│                                                              │
│  ┌─────────────────┐    ┌─────────────────┐                 │
│  │   ry-anim       │    │   ry-science    │                 │
│  │   (Disney FX)   │    │   (Análisis)    │                 │
│  └────────┬────────┘    └────────┬────────┘                 │
│           │                      │                          │
│  ┌────────▼──────────────────────▼────────┐                 │
│  │            ry-rs (CORE)                │                 │
│  │                                        │                 │
│  │  ┌──────────┐  ┌──────────┐           │                 │
│  │  │ levels   │  │ entities │           │                 │
│  │  │ +quests  │  │ +inventory            │                 │
│  │  └──────────┘  └──────────┘           │                 │
│  │  ┌──────────┐  ┌──────────┐           │                 │
│  │  │ camera   │  │ physics  │           │                 │
│  │  │ +path    │  │ +slopes  │           │                 │
│  │  └──────────┘  └──────────┘           │                 │
│  │  ┌──────────┐  ┌──────────┐           │                 │
│  │  │ tilemap  │  │collision │           │                 │
│  │  │ +hex     │  │ +3D      │           │                 │
│  │  └──────────┘  └──────────┘           │                 │
│  │  ┌──────────┐  ┌──────────┐           │                 │
│  │  │ migui    │  │ toolkit  │           │                 │
│  │  │ (widgets)│  │ (menus)  │           │                 │
│  │  └──────────┘  └──────────┘           │                 │
│  │  ┌──────────┐  ┌──────────┐           │                 │
│  │  │ audio    │  │particles │           │                 │
│  │  └──────────┘  └──────────┘           │                 │
│  └───────────────────┬───────────────────┘                 │
│                      │                                     │
│  ┌───────────────────▼───────────────────┐                 │
│  │            ry-stream                  │                 │
│  │   (Portal web + Discord + YouTube)    │                 │
│  └───────────────────────────────────────┘                 │
│                                                              │
│  ┌───────────────────┬───────────────────┐                  │
│  │   v-shield        │   ry-physics      │                  │
│  │   (multiplatform) │   (math puras)    │                  │
│  └───────────────────┴───────────────────┘                  │
└──────────────────────────────────────────────────────────────┘
```

---

<div align="center">

**🎮 Sistema Universal Ry — Análisis Completo**

*13 módulos | ~8,520 líneas | 60% completado*

*De motor invisible a plataforma de juegos visible*

</div>
