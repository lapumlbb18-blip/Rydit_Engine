# 🔍 Análisis Completo: ry-ecs (Entity Component System)

**Fecha**: 2026-04-05
**Versión**: ry-ecs v0.10.0
**Estado**: ✅ Compila | ✅ 5/5 tests pasando

---

## 📊 RESUMEN EJECUTIVO

ry-ecs es un **ECS híbrido** — usa `bevy_ecs` como base pero tiene una capa simplificada propia
(`EcsWorld` + `Entity` structs con `Option<Component>`) para evitar complejidad de Bevy puro.

**Lo que TIENE**:
- ✅ Componentes: Position, Velocity, Sprite, Body, Collider, Particle
- ✅ Marcadores: Player, Enemy, DespawnMarker
- ✅ Recursos: Gravity, DeltaTime, RenderConfig
- ✅ EcsWorld con create/destroy/entity management
- ✅ Sistemas: movimiento, gravedad, N-Body, partículas
- ✅ EcsRenderer con rlgl (rectángulos, colores por tipo, N-Body visualization)
- ✅ Demo world 10K entidades
- ✅ N-Body demo con órbitas
- ✅ Scene Runner (inversión de control: .rydit = config, Rust = game loop)

**Lo que NO tiene**:
- ❌ Colisiones reales (Collider existe pero no hay collision detection)
- ❌ Integración con módulos/rydit scripts (.rydit → ECS)
- ❌ Renderizado de texturas reales (solo rectángulos de color)
- ❌ Sistemas de combate, vida, daño
- ❌ AI, pathfinding
- ❌ Animación/sprites animados
- ❌ ECS queries reales de bevy_ecs (usa HashMap propio)

---

## 🏗️ ARQUITECTURA

### Dos Capas

```
┌─────────────────────────────────────────┐
│         ry-ecs (capa simplificada)      │
│  EcsWorld (HashMap<EntityId, Entity>)   │
│  Entity { position: Option<Position> }  │
│  - Fácil de usar desde .rydit           │
│  - Sin complejidad de Bevy              │
├─────────────────────────────────────────┤
│         bevy_ecs (base)                 │
│  Components: #[derive(Component)]       │
│  Resources:  #[derive(Resource)]        │
│  - Usado para compatibilidad futura     │
│  - Actualmente NO se usa directamente   │
└─────────────────────────────────────────┘
```

### Problema de Diseño Detectado

**ry-ecs tiene DOS sistemas ECS paralelos**:

1. **bevy_ecs**: Components, Resources definidos con derive (Position, Velocity, etc.)
2. **EcsWorld propio**: HashMap con Entity structs que tienen `Option<Position>`

**Los componentes de bevy_ecs están definidos pero NUNCA se usan en sistemas de bevy_ecs.**
Se usan SOLO como tipos de datos dentro de `EcsWorld` propio.

**Consecuencia**: `bevy_ecs = "0.15"` es una dependencia de ~150 crates que no se usa realmente.

---

## 📋 COMPONENTES IMPLEMENTADOS

| Componente | Campos | Uso |
|------------|--------|-----|
| **Position** | x, y | Movimiento, render |
| **Velocity** | vx, vy | Movimiento, gravedad |
| **Sprite** | texture_id, w, h, color, flip_x/y | Render (solo placeholder) |
| **Body** | mass, is_static | N-Body gravity |
| **Collider** | w, h, is_trigger | ⚠️ Existe pero SIN detección |
| **Particle** | lifetime, max_lifetime, size, rotation | Sistema de partículas |
| **Player** | marker | Identificar jugador |
| **Enemy** | marker | Identificar enemigo |
| **DespawnMarker** | marker | Marcar para eliminar |

---

## ⚙️ SISTEMAS IMPLEMENTADOS

| Sistema | Qué hace | Funciona |
|---------|----------|----------|
| **update_movement()** | pos += vel * dt | ✅ |
| **update_gravity()** | vel += gravity * dt | ✅ (afecta todas las entidades) |
| **update_nbody()** | F = G*m1*m2/r² entre pares | ✅ (simplificado) |
| **update_particles()** | Decrementa lifetime, elimina muertas | ✅ |
| **update()** | Corre todos los sistemas en orden | ✅ |

---

## 🎨 RENDER

| Función | Qué hace | Estado |
|---------|----------|--------|
| `render()` | Rectángulos rojos con rlgl | ✅ Funcional |
| `render_colored()` | Colores por tipo (player=verde, enemy=rojo) | ✅ Funcional |
| `render_nbody()` | Cruces amarillas para cuerpos N-Body | ✅ Funcional |
| **Texturas reales** | ❌ NO implementado | Solo rectángulos de color |

**Rlgl** se usa directamente (FFI `rlBegin`, `rlVertex2f`, `rlEnd`) — funciona pero es 2D puro.

---

## 🔧 INTEGRACIÓN ACTUAL

### scene_runner.rs (binario funcional)

```rust
// 1. Parsear config .rydit (SOLO datos, sin evaluator)
let config = ConfigParser::parse("demos/nivel.rydit")?;

// 2. Crear ECS World
let mut ecs_world = EcsWorld::new();
ecs_world.set_gravity(0.0, config.gravedad);

// 3. Spawnear entidades desde config
for ent in &config.entidades {
    ecs_world.create_player(ent.x, ent.y);
    // o
    ecs_world.create_sprite_entity(ent.x, ent.y, ...);
}

// 4. Game loop nativo (Rust puro, sin evaluator)
while !gfx.should_close() {
    ecs_world.update(0.016);         // UPDATE
    renderer.render_colored(&ecs_world); // RENDER
    // INPUT directo desde gfx
}
```

**Esto FUNCIONA** — es la "inversión de control" mencionada en docs anteriores.

### cli.rs

Tiene un comando `ry run-ecs <escena>` que hace lo mismo que scene_runner.

---

## ❌ LO QUE FALTA

### Crítico (para que sea usable)

| Feature | Descripción | Esfuerzo |
|---------|-------------|----------|
| **Colisiones reales** | Collider existe pero no hay detección ni respuesta | 10-15h |
| **Render texturas** | Usar Assets de ry-gfx en vez de rectángulos de color | 4-6h |
| **Integración .rydit** | Scripts .rydit que creen/manipulen entidades ECS | 8-12h |
| **Eliminar bevy_ecs** | No se usa realmente, solo añade peso | 2-4h |

### Deseable

| Feature | Descripción | Esfuerzo |
|---------|-------------|----------|
| **Vida/daño** | Componente Health + sistema de daño | 4-6h |
| **AI básica** | Perseguir jugador, patrullar | 8-12h |
| **Animación sprites** | Frames, estados (idle, run, jump) | 6-8h |
| **Camera follow** | Cámara sigue al jugador automáticamente | 2-4h |
| **Query system** | world.query::<(&Position, &Velocity)>() estilo Bevy | 4-6h |
| **Events system** | Mensajes entre sistemas | 6-8h |

### Futuro

| Feature | Esfuerzo |
|---------|----------|
| 3D ECS | 20-30h |
| Networking ECS | 15-20h |
| ECS editor visual | 30-40h |
| Hot reload ECS | 8-12h |

---

## 📊 COMPARATIVA

| | ry-ecs actual | bevy_ecs puro | Godot ECS |
|---|---|---|---|
| **Complejidad** | Baja | Alta | Media |
| **Rendimiento** | Medio (HashMap) | Alto (archetypes) | Alto |
| **Facilidad de uso** | ✅ Fácil | ❌ Complejo | ✅ Fácil |
| **Sistemas** | 4 | Ilimitados | Ilimitados |
| **Colisiones** | ❌ | ❌ (necesita physics) | ✅ |
| **Dependencias** | bevy_ecs (150+ crates) | 150+ crates | N/A (built-in) |
| **Tamaño binario** | +5MB por bevy_ecs | +5MB | N/A |

---

## 🎯 RECOMENDACIONES

### Opción A: Simplificar (recomendada)
1. **Eliminar bevy_ecs** como dependencia — no se usa
2. Los componentes Position, Velocity, etc. son structs simples, no necesitan derive(Component)
3. **Agregar colisiones AABB** entre entidades con Collider
4. **Conectar con Assets** para renderizar texturas reales

### Opción B: Usar bevy_ecs completo
1. Migrar EcsWorld a `bevy_ecs::World`
2. Usar `Query`, `System`, `Commands` de Bevy
3. Mayor poder, mayor complejidad

### Opción C: Híbrido (actual)
- Mantener como está
- Agregar features faltantes sobre la base actual

---

<div align="center">

**🔍 ry-ecs v0.10.0 — ECS híbrido funcional pero incompleto**

*4 sistemas | 5 tests ✅ | Sin colisiones | Sin texturas reales*

*Recomendación: eliminar bevy_ecs + agregar colisiones + conectar texturas*

</div>
