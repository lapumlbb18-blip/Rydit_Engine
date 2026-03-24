# 🧩 RYDITMODULE TRAIT - Diseño para v0.7.1

**Fecha**: 2026-03-24 (v0.7.0 completada)
**Estado**: Diseño / Planificación
**Próxima implementación**: v0.7.1.0

---

## 🎯 OBJETIVO

Evitar que el núcleo (`main.rs`, `eval/mod.rs`) siga creciendo indefinidamente. Permite agregar módulos independientes sin tocar el core estable.

---

## 🏗️ ARQUITECTURA PROPUESTA

```rust
// crates/rydit-rs/src/module.rs

/// Trait base para todos los módulos de RyDit
pub trait RyditModule {
    /// Nombre único del módulo (ej: "math", "physics", "anim")
    fn name(&self) -> &'static str;
    
    /// Versión semántica (ej: "0.1.0")
    fn version(&self) -> &'static str;
    
    /// Registrar funciones/constantes en el runtime
    fn register(&self, registry: &mut ModuleRegistry);
    
    /// Hook opcional para update (física, animación)
    fn update(&self, _dt: f32, _ctx: &mut ModuleContext) {}
    
    /// Hook opcional para render (gráficos 2D/3D)
    fn render(&self, _gfx: &mut RyditGfx, _ctx: &ModuleContext) {}
}

/// Registro de funciones del módulo
pub struct ModuleRegistry {
    functions: HashMap<String, ModuleFunction>,
    constants: HashMap<String, Valor>,
}

/// Función registrable
pub struct ModuleFunction {
    name: String,
    arity: usize,
    func: fn(&[Valor], &mut Executor) -> Result<Valor, String>,
}
```

---

## 📦 MÓDULOS PLANIFICADOS

### **v0.7.1.0 - Ciencia (Manim-style)**

```rust
// crates/rydit-mod-scene/

pub struct SceneModule;

impl RyditModule for SceneModule {
    fn name(&self) -> &'static str { "scene" }
    fn version(&self) -> &'static str { "0.1.0" }
    
    fn register(&self, registry: &mut ModuleRegistry) {
        registry.add_fn("create", scene_create, 1);
        registry.add_fn("add", scene_add, 2);
        registry.add_fn("play", scene_play, 2);
        registry.add_fn("wait", scene_wait, 1);
    }
}

// MObjects (Mathematical Objects)
pub struct MObject {
    id: String,
    kind: MObjectKind,  // Circle, Square, Arrow, Equation
    position: Vec3,
    color: Color,
    children: Vec<MObject>,
}
```

### **v0.7.1.0 - Física**

```rust
// crates/rydit-mod-physics/

pub struct PhysicsModule {
    gravity: Vec3,
    bodies: Vec<RigidBody>,
}

impl RyditModule for PhysicsModule {
    fn name(&self) -> &'static str { "physics" }
    
    fn register(&self, registry: &mut ModuleRegistry) {
        registry.add_fn("create_body", physics_create_body, 3);
        registry.add_fn("apply_force", physics_apply_force, 3);
        registry.add_fn("simulate", physics_simulate, 1);
    }
    
    fn update(&self, dt: f32, ctx: &mut ModuleContext) {
        // Integración de física (Verlet, RK4)
        for body in &mut self.bodies {
            body.integrate(dt);
        }
    }
}
```

### **v0.7.1.1 - Animación (12 principios)**

```rust
// crates/rydit-mod-anim/

pub struct AnimModule {
    animations: Vec<Animation>,
}

impl RyditModule for AnimModule {
    fn name(&self) -> &'static str { "anim" }
    
    fn register(&self, registry: &mut ModuleRegistry) {
        registry.add_fn("tween", anim_tween, 4);
        registry.add_fn("ease_in", anim_ease_in, 1);
        registry.add_fn("ease_out", anim_ease_out, 1);
        registry.add_fn("squash_stretch", anim_squash_stretch, 3);
        registry.add_fn("anticipation", anim_anticipation, 3);
    }
}

// 12 principios de animación
pub enum AnimPrinciple {
    SquashStretch,
    Anticipation,
    Staging,
    SlowInSlowOut,
    Arc,
    Exaggeration,
    // ... 7 más
}
```

### **v0.7.1.2 - HTTP (básico)**

```rust
// crates/rydit-mod-network/

pub struct NetworkModule {
    client: reqwest::blocking::Client,
}

impl RyditModule for NetworkModule {
    fn name(&self) -> &'static str { "network" }
    
    fn register(&self, registry: &mut ModuleRegistry) {
        registry.add_fn("http_get", http_get, 1);
        registry.add_fn("http_post", http_post, 2);
        registry.add_fn("ws_connect", ws_connect, 1);
        registry.add_fn("ws_send", ws_send, 2);
    }
}
```

---

## 🔧 MACRO DE REGISTRO

```rust
// crates/rydit-rs/src/macros.rs

#[macro_export]
macro_rules! rydit_module {
    ($name:ident, version: $version:literal, {
        $(fn $fn_name:ident = $fn_path:path,)*
    }) => {
        pub struct $name;
        
        impl RyditModule for $name {
            fn name(&self) -> &'static str { stringify!($name) }
            fn version(&self) -> &'static str { $version }
            
            fn register(&self, registry: &mut ModuleRegistry) {
                $(
                    registry.add_fn(stringify!($fn_name), $fn_path, get_arity($fn_path));
                )*
            }
        }
    };
}

// Uso:
rydit_module!(MathModule, version: "0.1.0", {
    fn sqrt = math::sqrt,
    fn sin = math::sin,
    fn cos = math::cos,
    fn tan = math::tan,
});
```

---

## 📊 MÉTRICAS ESPERADAS

| Métrica | Antes (v0.7.0) | Después (v0.7.1) |
|---------|----------------|------------------|
| **main.rs** | 4,573 líneas | ~4,500 (estable) |
| **eval/mod.rs** | 970 líneas | ~900 (-70) |
| **Módulos externos** | 0 | 4-6 crates |
| **Líneas por módulo** | N/A | ~500-1000 c/u |
| **Tests por módulo** | N/A | 20-30 c/u |
| **Build time** | ~90s | ~95s (+5%) |
| **Binario** | ~600 KB | ~650 KB (+50 KB) |

---

## 🛡️ VENTAJAS

1. **Núcleo estable**: No se toca después de v0.7.0
2. **Módulos independientes**: Publicables a crates.io
3. **Testing fácil**: Cada módulo con sus tests
4. **Comunidad**: Terceros pueden crear módulos
5. **Carga opcional`: Solo módulos usados van al binario

---

## ⚠️ DESVENTAJAS

1. **Complejidad**: Más archivos, más imports
2. **Overhead**: Trait bounds, dynamic dispatch
3. **Curva aprendizaje**: Comunidad debe aprender el trait
4. **Coordinación**: Versionamiento semántico por módulo

---

## 📋 CHECKLIST IMPLEMENTACIÓN

### **Fase 1: Infraestructura (v0.7.1.0)**
- [ ] Crear `crates/rydit-rs/src/module.rs`
- [ ] Implementar `RyditModule` trait
- [ ] Implementar `ModuleRegistry`
- [ ] Crear macro `rydit_module!`
- [ ] Migrar 1 módulo piloto (ej: math)

### **Fase 2: Módulos Ciencia (v0.7.1.0)**
- [ ] `crates/rydit-mod-scene/` (Scene, Camera, MObject)
- [ ] `crates/rydit-mod-physics/` (Projectile, NBody, Wave)
- [ ] Tests: 20+ por módulo
- [ ] Docs: README + ejemplos

### **Fase 3: Módulos Animación (v0.7.1.1)**
- [ ] `crates/rydit-mod-anim/` (12 principios, easing)
- [ ] Sprite sheets
- [ ] Timeline/keyframes

### **Fase 4: Integración (v0.8.0.0)**
- [ ] Game loop extraído a módulo
- [ ] Eval extraído completamente
- [ ] Núcleo <1000 líneas

---

## 🎯 EJEMPLO DE USO (v0.7.1.0)

```rydit
# Script RyDit con módulos

import scene
import physics
import anim

# Crear escena
scene::create("mi_escena")

# Agregar MObjects
scene::add("circulo", MObject::circle(50, "rojo"))
scene::add("cuadrado", MObject::square(40, "azul"))

# Animar con física
physics::create_body("circulo", mass: 1.0, pos: [0, 0])
physics::apply_force("circulo", [10, 0])

# Reproducir animación
scene::play("circulo", anim::tween([0, 0], [100, 100], duration: 2.0))
scene::wait(2.0)
```

---

## 🔗 REFERENCIAS

- **Manim**: https://github.com/3b1b/manim
  - `manim/scene/scene.py`
  - `manim/animation/animation.py`
  - `manim/mobject/mobject.py`

- **Bevy**: https://github.com/bevyengine/bevy
  - `crates/bevy_ecs/`
  - `crates/bevy_app/`
  - `crates/bevy_transform/`

- **Godot**: https://github.com/godotengine/godot
  - `scene/`
  - `servers/`

---

<div align="center">

**🛡️ RyDit v0.7.1 - RyditModule Trait**

*Diseñado: 2026-03-24 | Próxima implementación: v0.7.1.0*

</div>
