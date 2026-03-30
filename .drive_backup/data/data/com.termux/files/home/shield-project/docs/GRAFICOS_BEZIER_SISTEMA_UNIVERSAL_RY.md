# 🎨 GRÁFICOS BEZIER + SISTEMA UNIVERSAL RY

**Documento de Diseño Técnico**
**Versión**: Draft v0.1
**Fecha**: 2026-03-26
**Estado**: En planificación

---

## 📋 OBJETIVO

Implementar un **sistema de gráficos avanzado** con curvas Bezier nativas + un **sistema de contenedores universal** para mundos, actores y herramientas.

---

## 🎨 PARTE 1: GRÁFICOS BEZIER

### 1.1 draw.bezier() - Renderizado Nativo

**Funciones a implementar:**

```rust
// crates/rydit-gfx/src/draw.rs

/// Dibujar curva Bezier cúbica
pub fn draw_bezier_cubic(
    x0: f32, y0: f32,  // Punto inicial
    x1: f32, y1: f32,  // Control 1
    x2: f32, y2: f32,  // Control 2
    x3: f32, y3: f32,  // Punto final
    color: ColorRydit,
    thickness: f32,
    segments: u32,     // Suavidad (default: 20)
) {
    // Algoritmo de De Casteljau
    for i in 0..=segments {
        let t = i as f32 / segments as f32;
        let mt = 1.0 - t;
        
        // P(t) = mt³*P0 + 3*mt²*t*P1 + 3*mt*t²*P2 + t³*P3
        let x = mt*mt*mt*x0 + 3.0*mt*mt*t*x1 + 3.0*mt*t*t*x2 + t*t*t*x3;
        let y = mt*mt*mt*y0 + 3.0*mt*mt*t*y1 + 3.0*mt*t*t*y2 + t*t*t*y3;
        
        if i > 0 {
            // Dibujar línea desde punto anterior
            draw_line(prev_x, prev_y, x, y, thickness, color);
        }
        
        prev_x = x;
        prev_y = y;
    }
}

/// Dibujar curva Bezier cuadrática
pub fn draw_bezier_quadratic(
    x0: f32, y0: f32,  // Punto inicial
    x1: f32, y1: f32,  // Control
    x2: f32, y2: f32,  // Punto final
    color: ColorRydit,
    thickness: f32,
    segments: u32,
) {
    for i in 0..=segments {
        let t = i as f32 / segments as f32;
        let mt = 1.0 - t;
        
        // P(t) = mt²*P0 + 2*mt*t*P1 + t²*P2
        let x = mt*mt*x0 + 2.0*mt*t*x1 + t*t*x2;
        let y = mt*mt*y0 + 2.0*mt*t*y1 + t*t*y2;
        
        if i > 0 {
            draw_line(prev_x, prev_y, x, y, thickness, color);
        }
        
        prev_x = x;
        prev_y = y;
    }
}

/// Dibujar path (múltiples puntos conectados)
pub fn draw_path(
    points: &[(f32, f32)],
    color: ColorRydit,
    thickness: f32,
    closed: bool,      // Si es true, une último con primer punto
) {
    if points.len() < 2 {
        return;
    }
    
    for i in 1..points.len() {
        let (x0, y0) = points[i - 1];
        let (x1, y1) = points[i];
        draw_line(x0, y0, x1, y1, thickness, color);
    }
    
    if closed && points.len() > 2 {
        let (x0, y0) = points[points.len() - 1];
        let (x1, y1) = points[0];
        draw_line(x0, y0, x1, y1, thickness, color);
    }
}
```

---

### 1.2 Sintaxis en RyDit

```rydit
shield.init

ryda frame < 1000 {
    # Bezier cúbica simple
    draw.bezier(
        100, 100,   # Punto inicial
        200, 50,    # Control 1
        400, 350,   # Control 2
        500, 300,   # Punto final
        "rojo",
        3.0         # Grosor
    )
    
    # Bezier con suavidad personalizada
    draw.bezier(
        50, 50, 150, 0, 250, 0, 300, 50,
        "azul", 2.0, 50  # 50 segmentos (más suave)
    )
    
    # Path (múltiples puntos)
    dark.slot path = [
        [100, 100],
        [200, 150],
        [250, 250],
        [300, 300]
    ]
    draw.path(path, "verde", 2.0)
    
    # Path cerrado (polígono)
    dark.slot triangulo = [
        [400, 100],
        [350, 200],
        [450, 200]
    ]
    draw.path(triangulo, "amarillo", 2.0, true)
}
```

---

### 1.3 Casos de Uso

#### 1.3.1 Animación de Sprites con Bezier

```rydit
# Movimiento suave de enemigo siguiendo curva
dark.slot t = 0

ryda frame < 2000 {
    # Calcular posición en curva Bezier
    dark.slot pos = bezier.point(
        0, 0,      # Inicio
        200, 100,  # Control 1
        400, 300,  # Control 2
        600, 200,  # Fin
        t / 2000
    )
    
    # Dibujar enemigo en posición
    draw.sprite("enemy", pos[0], pos[1])
    
    dark.slot t = t + 1
}
```

#### 1.3.2 Interfaz de Usuario con Curvas

```rydit
# Botón con bordes redondeados (Bezier)
draw.rounded_rect(100, 100, 200, 50, 10, "azul")

# Barra de progreso curva
draw.bezier_bar(100, 200, 300, progreso, "verde")
```

#### 1.3.3 Trayectorias de Proyectiles

```rydit
# Proyectil con arco parabólico (Bezier cuadrática)
rytmo lanzar_proyectil(x0, y0, x1, y1, x2, y2) {
    dark.slot t = 0
    
    ryda t < 100 {
        dark.slot pos = bezier.quadratic.point(x0, y0, x1, y1, x2, y2, t / 100)
        draw.circle(pos[0], pos[1], 5, "rojo")
        dark.slot t = t + 1
    }
}
```

---

## 🌍 PARTE 2: SISTEMA UNIVERSAL RY

### 2.1 Arquitectura de Contenedores

```
┌─────────────────────────────────────────────────────────┐
│  SISTEMA UNIVERSAL RY                                   │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  CONTENEDOR (Container)                         │   │
│  │  - Nombre: "mundo_principal"                    │   │
│  │  - Tipo: mundo / nivel / escena                 │   │
│  │  - Actores: [actor1, actor2, ...]               │   │
│  │  - Herramientas: [herramienta1, ...]            │   │
│  │  - Configuración: {gravedad, fondo, ...}        │   │
│  └─────────────────────────────────────────────────┘   │
│                         │                               │
│  ┌──────────────────────▼──────────────────────────┐   │
│  │  ACTOR (Entity)                                 │   │
│  │  - ID: "jugador_1"                              │   │
│  │  - Componentes: [transform, sprite, physics]    │   │
│  │  - Scripts: [update, on_collision, ...]         │   │
│  │  - Estado: {vida, energia, ...}                 │   │
│  └─────────────────────────────────────────────────┘   │
│                         │                               │
│  ┌──────────────────────▼──────────────────────────┐   │
│  │  HERRAMIENTA (Tool/Component)                   │   │
│  │  - Tipo: transform / sprite / physics / audio   │   │
│  │  - Datos: específicos del tipo                  │   │
│  │  - Métodos: update(), render(), on_event()      │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

### 2.2 Estructura de Datos

```rust
// crates/rydit-rs/src/universal/mod.rs

use std::collections::HashMap;
use serde_json::{Value, json};

/// Contenedor universal (Mundo/Nivel/Escena)
pub struct Container {
    pub name: String,
    pub container_type: ContainerType,
    pub actors: HashMap<String, Actor>,
    pub tools: HashMap<String, Tool>,
    pub config: ContainerConfig,
}

/// Tipos de contenedor
#[derive(Clone, PartialEq)]
pub enum ContainerType {
    Mundo,      // Mundo abierto
    Nivel,      // Nivel específico
    Escena,     // Escena cinemática
    Menu,       // Menú UI
    MiniJuego,  // Minijuego dentro del mundo
}

/// Configuración del contenedor
pub struct ContainerConfig {
    pub gravedad: (f32, f32),
    pub fondo: ColorRydit,
    pub ancho: f32,
    pub alto: f32,
    pub fisica: bool,
}

/// Actor (Entidad)
pub struct Actor {
    pub id: String,
    pub name: String,
    pub components: HashMap<String, Component>,
    pub scripts: HashMap<String, Vec<Stmt>>,
    pub state: HashMap<String, Value>,
    pub parent: Option<String>,
    pub children: Vec<String>,
}

/// Componente/Herramienta
pub struct Component {
    pub component_type: ComponentType,
    pub data: ComponentData,
}

/// Tipos de componentes
#[derive(Clone, PartialEq, Hash, Eq)]
pub enum ComponentType {
    Transform,
    Sprite,
    Physics,
    Collider,
    Audio,
    Particle,
    Light,
    Camera,
    UI,
}

/// Datos de componentes
pub enum ComponentData {
    Transform(TransformData),
    Sprite(SpriteData),
    Physics(PhysicsData),
    Collider(ColliderData),
    Audio(AudioData),
    Particle(ParticleData),
    // ... más tipos
}

/// Datos de Transform
pub struct TransformData {
    pub position: (f32, f32),
    pub rotation: f32,
    pub scale: (f32, f32),
    pub visible: bool,
}

/// Datos de Sprite
pub struct SpriteData {
    pub texture: String,
    pub frame: (u32, u32),
    pub size: (f32, f32),
    pub flip_h: bool,
    pub flip_v: bool,
}

/// Datos de Physics
pub struct PhysicsData {
    pub velocity: (f32, f32),
    pub acceleration: (f32, f32),
    pub mass: f32,
    pub gravity: bool,
    pub grounded: bool,
}
```

---

### 2.3 Sintaxis en RyDit

```rydit
# =============================================================================
# SISTEMA UNIVERSAL RY - SINTAXIS
# =============================================================================

# -----------------------------------------------------------------------------
# CREACIÓN DE CONTENEDORES
# -----------------------------------------------------------------------------

# Crear un mundo
mundo.crear "mundo_principal" {
    config.gravedad = [0, 9.8]
    config.fondo = "azul_oscuro"
    config.ancho = 1920
    config.alto = 1080
}

# Crear un nivel
nivel.crear "nivel_1" {
    config.fondo = "verde_pasto"
    config.fisica = true
}

# Crear una escena
escena.crear "intro" {
    config.fondo = "negro"
}

# -----------------------------------------------------------------------------
# ACTORES
# -----------------------------------------------------------------------------

# Crear actor (jugador)
actor.crear "jugador" en "mundo_principal" {
    transform.posicion = [400, 300]
    transform.escala = [1.0, 1.0]
    
    sprite.cargar "sprites/jugador.png"
    sprite.tamano = [64, 64]
    
    fisica.activar
    fisica.masa = 1.0
    fisica.gravedad = true
    
    collider.rectangulo [0, 0, 64, 64]
    
    # Script de actualización
    ryda siempre {
        # Input
        onif key_pressed("RIGHT") {
            actor.mover "jugador" [5, 0]
        }
        onif key_pressed("LEFT") {
            actor.mover "jugador" [-5, 0]
        }
        onif key_pressed("SPACE") & actor.en_suelo("jugador") {
            actor.salto "jugador" 15
        }
    }
}

# Crear enemigo
actor.crear "enemigo_1" en "nivel_1" {
    sprite.cargar "sprites/enemigo.png"
    transform.posicion = [600, 300]
    
    # IA simple - patrulla
    ryda siempre {
        dark.slot pos = actor.obtener_posicion "enemigo_1"
        
        onif pos[0] > 700 {
            actor.mover "enemigo_1" [-2, 0]
            sprite.flip_horizontal = true
        } blelse onif pos[0] < 500 {
            actor.mover "enemigo_1" [2, 0]
            sprite.flip_horizontal = false
        }
    }
}

# Crear plataforma
actor.crear "plataforma_1" en "nivel_1" {
    transform.posicion = [300, 400]
    sprite.cargar "sprites/plataforma.png"
    transform.tamano = [200, 32]
    
    collider.rectangulo [0, 0, 200, 32]
    collider.es_suelo = true
}

# -----------------------------------------------------------------------------
# HERRAMIENTAS
# -----------------------------------------------------------------------------

# Herramienta de cámara
camara.crear "principal" {
    camara.seguir "jugador"
    camara.suavizado = 0.1
    camara.limites = [0, 0, 1920, 1080]
}

# Herramienta de partículas
particulas.crear "fuego" {
    particulas.textura = "sprites/particula_fuego.png"
    particulas.cantidad = 50
    particulas.vida = [0.5, 1.0]
    particulas.velocidad = [50, 100]
    particulas.color = ["rojo", "naranja", "amarillo"]
    particulas.emitir_en [400, 300]
}

# Herramienta de audio
audio.crear "musica_fondo" {
    audio.cargar "audio/bgm_nivel1.ogg"
    audio.repetir = true
    audio.volumen = 0.7
    audio.reproducir
}

# -----------------------------------------------------------------------------
# COLISIONES
# -----------------------------------------------------------------------------

# Detectar colisión
on colision "jugador" con "enemigo_1" {
    voz "¡Colisión con enemigo!"
    actor.dañar "jugador" 10
}

# on colision "jugador" con "moneda" {
on colision "jugador" con "moneda_1" {
    voz "¡Moneda recogida!"
    juego.sumar_puntos 100
    actor.eliminar "moneda_1"
}

# -----------------------------------------------------------------------------
# TRANSICIONES
# -----------------------------------------------------------------------------

# Cargar nivel
nivel.cargar "nivel_2" {
    transicion.desvanecer 1.0
}

# Guardar estado del mundo
mundo.guardar "mundo_principal" en "guardados/partida1.json"

# Cargar estado guardado
mundo.cargar "mundo_principal" desde "guardados/partida1.json"
```

---

### 2.4 Implementación en Rust

```rust
// crates/rydit-rs/src/universal/container.rs

use std::collections::HashMap;
use crate::universal::{Actor, Component, ContainerConfig, ContainerType};

pub struct UniversalContainer {
    pub name: String,
    pub container_type: ContainerType,
    pub actors: HashMap<String, Actor>,
    pub config: ContainerConfig,
}

impl UniversalContainer {
    pub fn new(name: &str, ctype: ContainerType) -> Self {
        Self {
            name: name.to_string(),
            container_type: ctype,
            actors: HashMap::new(),
            config: ContainerConfig::default(),
        }
    }
    
    /// Crear actor
    pub fn create_actor(&mut self, id: &str, name: &str) {
        let actor = Actor::new(id, name);
        self.actors.insert(id.to_string(), actor);
    }
    
    /// Actualizar todos los actores
    pub fn update(&mut self, delta_time: f32) {
        for actor in self.actors.values_mut() {
            actor.update(delta_time);
        }
    }
    
    /// Renderizar todos los actores
    pub fn render(&self) {
        for actor in self.actors.values() {
            actor.render();
        }
    }
    
    /// Detectar colisiones
    pub fn check_collisions(&mut self) {
        let actor_ids: Vec<String> = self.actors.keys().cloned().collect();
        
        for i in 0..actor_ids.len() {
            for j in (i + 1)..actor_ids.len() {
                let id1 = &actor_ids[i];
                let id2 = &actor_ids[j];
                
                if self.check_collision(id1, id2) {
                    self.handle_collision(id1, id2);
                }
            }
        }
    }
    
    fn check_collision(&self, id1: &str, id2: &str) -> bool {
        if let (Some(actor1), Some(actor2)) = (
            self.actors.get(id1),
            self.actors.get(id2),
        ) {
            return actor1.collides_with(actor2);
        }
        false
    }
    
    fn handle_collision(&mut self, id1: &str, id2: &str) {
        // Ejecutar scripts de colisión
        if let Some(actor) = self.actors.get_mut(id1) {
            if let Some(script) = actor.scripts.get("on_collision") {
                // Ejecutar script
            }
        }
    }
}
```

---

## 🎯 CASOS DE USO COMBINADOS

### 3.1 Juego de Plataformas con Bezier + Sistema Universal

```rydit
# =============================================================================
# JUEGO DE PLATAFORMAS - DEMO COMPLETA
# =============================================================================

shield.init

# -----------------------------------------------------------------------------
# MUNDO PRINCIPAL
# -----------------------------------------------------------------------------

mundo.crear "mundo_principal" {
    config.gravedad = [0, 9.8]
    config.fondo = [20, 20, 40]  # Azul oscuro
    config.ancho = 1920
    config.alto = 1080
}

# -----------------------------------------------------------------------------
# JUGADOR
# -----------------------------------------------------------------------------

actor.crear "jugador" en "mundo_principal" {
    transform.posicion = [200, 500]
    sprite.cargar "sprites/jugador.png"
    sprite.tamano = [64, 64]
    
    fisica.activar
    fisica.masa = 1.0
    
    collider.rectangulo [8, 8, 48, 48]
    
    # Movimiento suave con Bezier para salto
    rytmo salto() {
        dark.slot inicio = transform.posicion
        dark.slot t = 0
        
        # Arco de salto (Bezier cuadrática)
        ryda t < 50 {
            dark.slot altura = bezier.quadratic(
                inicio[0], inicio[1],     # Inicio
                inicio[0] + 50, inicio[1] - 150,  # Punto más alto
                inicio[0] + 100, inicio[1],  # Fin
                t / 50
            )
            transform.posicion = altura
            dark.slot t = t + 1
        }
    }
    
    # Input
    ryda siempre {
        onif key_pressed("RIGHT") {
            actor.mover "jugador" [5, 0]
        }
        onif key_pressed("LEFT") {
            actor.mover "jugador" [-5, 0]
        }
        onif key_pressed("SPACE") & actor.en_suelo("jugador") {
            salto()
        }
    }
}

# -----------------------------------------------------------------------------
# NIVEL CON BEZIER
# -----------------------------------------------------------------------------

# Plataforma curva (usando Bezier)
dark.slot plataforma_puntos = [
    [0, 600],
    [200, 580],
    [400, 550],
    [600, 530],
    [800, 550]
]

draw.path(plataforma_puntos, "verde", 32.0)

# Crear colisor siguiendo la curva
actor.crear "plataforma_curva" en "mundo_principal" {
    transform.posicion = [0, 600]
    collider.path(plataforma_puntos)
    collider.es_suelo = true
}

# -----------------------------------------------------------------------------
# ENEMIGOS CON TRAYECTORIA BEZIER
# -----------------------------------------------------------------------------

actor.crear "enemigo_volador" en "mundo_principal" {
    sprite.cargar "sprites/enemigo_volador.png"
    sprite.tamano = [48, 48]
    
    # Trayectoria circular/elíptica usando Bezier
    dark.slot ruta_puntos = [
        [400, 300],
        [500, 250],
        [600, 300],
        [500, 350]
    ]
    
    dark.slot t = 0
    
    ryda siempre {
        dark.slot pos = bezier.closed.path(ruta_puntos, t / 100)
        transform.posicion = pos
        dark.slot t = t + 1
        
        # Resetear para loop
        onif t >= 100 {
            dark.slot t = 0
        }
    }
}

# -----------------------------------------------------------------------------
# EFECTOS DE PARTÍCULAS CON BEZIER
# -----------------------------------------------------------------------------

# Estela del jugador (partículas siguiendo trayectoria)
dark.slot estela = []

ryda siempre {
    dark.slot pos_jugador = actor.obtener_posicion "jugador"
    
    # Agregar punto a la estela
    estela.agregar(pos_jugador)
    
    # Limitar longitud de estela
    onif estela.longitud() > 20 {
        estela.eliminar(0)
    }
    
    # Dibujar estela como path suave
    onif estela.longitud() > 2 {
        draw.path(estela, "cian", 8.0)
    }
}

# -----------------------------------------------------------------------------
# CÁMARA CON SEGUIMIENTO SUAVE
# -----------------------------------------------------------------------------

camara.crear "principal" {
    # Seguimiento suave con interpolación
    camara.seguir "jugador"
    camara.suavizado = 0.05  # Lerp factor
    camara.limites = [0, 0, 1920, 1080]
}

# -----------------------------------------------------------------------------
# UI CON BEZIER
# -----------------------------------------------------------------------------

# Barra de vida curva
rytmo dibujar_ui() {
    dark.slot vida = actor.obtener_estado "jugador" "vida"
    
    # Fondo de barra (curva)
    draw.bezier(
        50, 50, 150, 45, 250, 45, 350, 50,
        "gris", 20.0
    )
    
    # Vida actual (curva verde)
    dark.slot ancho_vida = 300 * (vida / 100.0)
    draw.bezier(
        50, 50,
        50 + ancho_vida/2, 45,
        50 + ancho_vida/2, 45,
        50 + ancho_vida, 50,
        "verde", 20.0
    )
    
    # Texto
    draw.text("VIDA: " + vida, 50, 30, "blanco")
}

# -----------------------------------------------------------------------------
# GAME LOOP PRINCIPAL
# -----------------------------------------------------------------------------

ryda frame < 100000 {
    # Limpiar
    draw.rect(0, 0, 1920, 1080, config.fondo)
    
    # Actualizar mundo
    mundo.actualizar "mundo_principal"
    
    # Renderizar
    mundo.renderizar "mundo_principal"
    
    # UI
    dibujar_ui()
    
    # Verificar game over
    dark.slot vida = actor.obtener_estado "jugador" "vida"
    onif vida <= 0 {
        voz "Game Over"
        mundo.cargar "nivel_1"
    }
}
```

---

## 📊 ARQUITECTURA TÉCNICA

### 4.1 Diagrama de Clases

```
┌────────────────────────────────────────────────────────────┐
│  UniversalSystem                                          │
├────────────────────────────────────────────────────────────┤
│  - containers: HashMap<String, Container>                 │
│  - active_container: String                               │
│  - tools: HashMap<String, Tool>                           │
├────────────────────────────────────────────────────────────┤
│  + create_container(name, type)                           │
│  + create_actor(container, id, name)                      │
│  + add_component(actor, component_type, data)             │
│  + update(delta_time)                                     │
│  + render()                                               │
│  + check_collisions()                                     │
└────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌──────────────┐   ┌──────────────┐   ┌──────────────┐
│  Container   │   │    Actor     │   │   Component  │
├──────────────┤   ├──────────────┤   ├──────────────┤
│ - name       │   │ - id         │   │ - type       │
│ - type       │   │ - components │   │ - data       │
│ - actors     │   │ - scripts    │   │ - methods    │
│ - config     │   │ - state      │   └──────────────┘
└──────────────┘   │ - parent     │
                   │ - children   │
                   └──────────────┘
```

---

### 4.2 Flujo de Actualización

```
┌─────────────────────────────────────────────────────────┐
│  GAME LOOP (60 FPS)                                     │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  1. INPUT                                               │
│     └─ Leer teclado/mouse/gamepad                       │
│                                                         │
│  2. UPDATE (delta_time)                                 │
│     ├─ Para cada Container:                             │
│     │   ├─ Para cada Actor:                             │
│     │   │   ├─ Ejecutar scripts (ryda siempre)          │
│     │   │   ├─ Actualizar componentes                   │
│     │   │   │   ├─ Transform (posición, rotación)       │
│     │   │   │   ├─ Physics (velocidad, gravedad)        │
│     │   │   │   └─ Animation (frame, estado)            │
│     │   │   └─ Actualizar estado                        │
│     │   └─ Check colisiones                             │
│     └─ Actualizar herramientas                          │
│                                                         │
│  3. RENDER                                              │
│     ├─ Limpiar pantalla                                 │
│     ├─ Para cada Container:                             │
│     │   └─ Para cada Actor:                             │
│     │       ├─ Renderizar sprite                        │
│     │       ├─ Renderizar componentes                   │
│     │       └─ Debug (colliders, etc.)                  │
│     └─ Renderizar UI                                    │
│                                                         │
│  4. SYNC                                                │
│     └─ Esperar vsync / limit FPS                        │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## 🎯 PRÓXIMOS PASOS

### Fase 1: Gráficos Bezier (v0.8.0)
- [ ] Implementar `draw.bezier_cubic()` en rydit-gfx
- [ ] Implementar `draw.bezier_quadratic()` en rydit-gfx
- [ ] Implementar `draw.path()` en rydit-gfx
- [ ] Agregar funciones a eval/mod.rs
- [ ] Crear demo de bezier
- [ ] Tests de bezier

### Fase 2: Sistema Universal - Core (v0.9.0)
- [ ] Estructuras `Container`, `Actor`, `Component`
- [ ] Sistema de creación de mundos
- [ ] Sistema de actores básicos
- [ ] Componente Transform
- [ ] Componente Sprite

### Fase 3: Sistema Universal - Física (v0.9.0)
- [ ] Componente Physics
- [ ] Componente Collider
- [ ] Detección de colisiones
- [ ] Respuesta a colisiones
- [ ] Scripts de colisión

### Fase 4: Sistema Universal - Herramientas (v0.9.0)
- [ ] Herramienta Cámara
- [ ] Herramienta Audio
- [ ] Herramienta Partículas
- [ ] Herramienta UI
- [ ] Transiciones entre niveles

---

<div align="center">

**🛡️ RyDit Engine - Gráficos Bezier + Sistema Universal**

*Documentación técnica para implementación v0.8.0 - v0.9.0*

</div>
