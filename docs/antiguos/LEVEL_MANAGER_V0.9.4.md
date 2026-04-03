# 🛡️ RyDit v0.9.4 - LEVEL MANAGER COMPLETADO

**Fecha**: 2026-03-30
**Versión**: v0.9.4
**Estado**: ✅ COMPLETADO

---

## 📋 RESUMEN

El **Level Manager** es el sistema de gestión de niveles para RyDit Engine. Permite cargar, descargar y hacer transiciones entre niveles, además de gestionar checkpoints.

---

## 🎯 FEATURES IMPLEMENTADAS

### 1. **Carga de Niveles**

| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `level::load(ruta)` | Cargar nivel desde archivo | `level::load("niveles/nivel1.rydit")` |
| `level::unload()` | Descargar nivel actual | `level::unload()` |
| `level::reload()` | Recargar nivel actual | `level::reload()` |

### 2. **Información del Nivel**

| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `level::get_current()` | Obtener nombre del nivel actual | `level::get_current()` |
| `level::get_name()` | Obtener nombre del nivel | `level::get_name()` |

### 3. **Transiciones**

| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `level::transition(ruta)` | Iniciar transición a otro nivel | `level::transition("niveles/nivel2.rydit")` |
| `level::transition_fade(duracion)` | Transición fade (oscuro) | `level::transition_fade(1000)` |
| `level::transition_slide(direccion, duracion)` | Transición slide | `level::transition_slide("left", 500)` |

### 4. **Checkpoints**

| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `level::set_checkpoint(nombre, x, y)` | Establecer checkpoint | `level::set_checkpoint("guard1", 400, 300)` |
| `level::load_checkpoint(nombre)` | Cargar posición de checkpoint | `level::load_checkpoint("guard1")` |
| `level::get_checkpoint(nombre)` | Obtener posición de checkpoint | `level::get_checkpoint("guard1")` |
| `level::list_checkpoints()` | Listar todos los checkpoints | `level::list_checkpoints()` |

---

## 📖 USO BÁSICO

### Cargar un nivel:

```rydit
# Cargar nivel desde archivo
level::load("demos/nivel1.rydit")

# El nivel se registra en el Level Manager
# Metadata opcional en el archivo:
# # @nombre "Nivel 1"
# # @musica "batalla.mp3"
# # @dificultad "facil"
```

### Establecer checkpoints:

```rydit
# Guardar checkpoint en posición (400, 300)
level::set_checkpoint("guard1", 400, 300)

# Múltiples checkpoints
level::set_checkpoint("inicio", 100, 100)
level::set_checkpoint("medio", 400, 300)
level::set_checkpoint("final", 700, 500)
```

### Cargar checkpoint (respawn):

```rydit
# Cuando el jugador muere, cargar último checkpoint
dark.slot pos = level::load_checkpoint("guard1")
# pos = [400, 300] (array con x, y)

# Mover jugador al checkpoint
entity::set_position("jugador", pos[0], pos[1])
```

### Transición entre niveles:

```rydit
# Cuando el jugador llega al final del nivel
onif entity::reached_goal("jugador") {
    level::transition("demos/nivel2.rydit")
}

# O con fade out
level::transition_fade(1000)  # 1 segundo
level::transition("demos/nivel2.rydit")
```

---

## 🏗️ ARQUITECTURA

### LevelManager Struct

```rust
pub struct LevelManager {
    pub current_level: Option<String>,      // Nombre del nivel actual
    pub current_path: Option<String>,       // Ruta del archivo
    pub checkpoints: HashMap<String, (f32, f32)>,  // Checkpoints
    pub level_data: HashMap<String, Valor>, // Metadata (@nombre, @musica, etc.)
    pub level_history: Vec<String>,         // Historial de niveles
    pub is_transitioning: bool,             // Estado de transición
    pub next_level: Option<String>,         // Próximo nivel
}
```

### Estado Global

El Level Manager usa un estado global thread-local:

```rust
thread_local! {
    static LEVEL_MANAGER: Rc<RefCell<LevelManager>> = ...;
}

pub fn get_level_manager() -> Rc<RefCell<LevelManager>> {
    LEVEL_MANAGER.with(|lm| lm.clone())
}
```

---

## 📁 ARCHIVOS CREADOS

| Archivo | Líneas | Descripción |
|---------|--------|-------------|
| `crates/rydit-rs/src/modules/level.rs` | 528 | Level Manager completo |
| `crates/rydit-rs/src/modules/mod.rs` | +1 | Registro del módulo |
| `crates/rydit-rs/src/eval/mod.rs` | +78 | Funciones para .rydit |
| `demos/test_level_manager.rydit` | 50 | Demo de prueba |
| `demos/nivel1.rydit` | 15 | Archivo de nivel 1 |
| `demos/nivel2.rydit` | 10 | Archivo de nivel 2 |

**Total**: ~682 líneas nuevas

---

## 🧪 TESTS

### Test básico:

```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

./target/release/rydit-rs --gfx demos/test_level_manager.rydit
```

### Verificar funciones:

1. **Carga de nivel**: Debe mostrar "Nivel actual: nivel1"
2. **Checkpoint**: Presionar 'R' para cargar checkpoint
3. **Transición**: Presionar 'N' para ir al nivel 2
4. **Lista**: Presionar 'L' para ver checkpoints

---

## 🔗 INTEGRACIÓN CON ENTITY SYSTEM

El Level Manager trabaja con Entity System:

```rydit
# Al cargar nivel, spawnear entidades
level::load("nivel1.rydit")

# Crear jugador en posición inicial
dark.slot spawn = level::get_checkpoint("inicio")
entity::create("player", spawn[0], spawn[1])

# Al morir, respawn en checkpoint
onif entity::is_dead("jugador") {
    dark.slot pos = level::load_checkpoint("ultimo_guard")
    entity::set_position("jugador", pos[0], pos[1])
    entity::respawn("jugador")
}
```

---

## 🎯 PRÓXIMOS PASOS

### v0.9.4 - FASE 2: Tilemap

- [ ] `tilemap::load(ruta, tile_size)` - Cargar tilemap
- [ ] `tilemap::set_tile(x, y, tile_id)` - Colocar tile
- [ ] `tilemap::get_tile(x, y)` - Obtener tile
- [ ] `tilemap::draw()` - Dibujar tilemap

### v0.9.4 - FASE 3: Colisiones

- [ ] `collision::check_rect_rect(...)` - Colisión AABB
- [ ] `area2d::create(id, x, y, w, h)` - Área 2D
- [ ] `collision::resolve(...)` - Respuesta a colisión

### v0.9.4 - FASE 4: Window Manager

- [ ] `window::set_title(titulo)` - Título de ventana
- [ ] `window::set_fullscreen(enabled)` - Pantalla completa
- [ ] `window::set_vsync(enabled)` - VSync

---

## 📊 MÉTRICAS v0.9.4

| Feature | Estado | Funciones |
|---------|--------|-----------|
| **Level Manager** | ✅ 100% | 13 funciones |
| **Entity System** | ✅ 95% | 50+ funciones |
| **Cámara 2D** | ✅ 100% | 15 funciones |
| **Physics 2D** | ✅ 100% | 20 funciones |
| **Render Queue** | ✅ 100% | Integrada |
| **Input Map** | ✅ 100% | 8 funciones |
| **IME** | ✅ 100% | Teclado virtual |

**Total funciones**: 100+ funciones para juegos 2D

---

## 🛡️ ESTADO v0.9.4

| Sistema | Estado |
|---------|--------|
| **Level Manager** | ✅ COMPLETADO |
| **Tilemap** | ⏳ PENDIENTE |
| **Colisiones 2D** | ⏳ PENDIENTE |
| **Window Manager** | ⏳ PENDIENTE |
| **Demo platformer** | ⏳ PENDIENTE |

**Próximo**: Tilemap + Colisiones = **Juego 2D completo posible** 🎮

---

<div align="center">

**🛡️ RyDit v0.9.4 - LEVEL MANAGER ✅**

*Level Manager ✅ | Entity System ✅ | Cámara 2D ✅ | Físicas ✅*

**100+ funciones para juegos 2D**

**Próximo: Tilemap + Colisiones**

</div>
