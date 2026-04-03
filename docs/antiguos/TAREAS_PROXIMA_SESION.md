# 📋 TAREAS PRÓXIMA SESIÓN - v0.7.2.0

**Fecha estimada**: 2026-03-26
**Versión objetivo**: v0.7.2.0
**Enfoque**: Sistema de Nodos + Scene Graph

---

## 🔴 PRIORIDAD ALTA

### 1. Verificar Tests Completos ⚠️
**Estado**: PENDIENTE (requiere Termux-X11)

**Comandos**:
```bash
# Abrir Termux-X11 primero
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

# Ejecutar todos los tests
cargo test --all 2>&1 | grep "test result:"

# Contar total
cargo test --all 2>&1 | grep "test result:" | awk '{sum += $3} END {print "Total: " sum " tests"}'
```

**Objetivo**: Confirmar 163+ tests passing (incluyendo rydit-gfx)

**Posibles fixes**:
- Si rydit-gfx falla: revisar inicialización GLFW
- Si hay errores de memoria: revisar game loop

---

### 2. Sistema de Nodos - Diseño 📐
**Estado**: PLANIFICADO

**Tareas**:
1. Diseñar estructura `Node` (transform, children, parent)
2. Diseñar `SceneGraph` (gestión de nodos)
3. Diseñar `Transform` (posición, rotación, escala)
4. Crear archivo de diseño: `NODES_DISENO.md`

**Estructura propuesta**:
```rust
struct Node {
    id: String,
    transform: Transform,
    children: Vec<Node>,
    parent: Option<Box<Node>>,
}

struct Transform {
    position: Vec2,
    rotation: f32,
    scale: Vec2,
}

struct SceneGraph {
    root: Node,
    nodes: HashMap<String, Node>,
}
```

---

### 3. RyditModule Trait - Implementación 🔌
**Estado**: PLANIFICADO

**Tareas**:
1. Mover `RyditModule` trait a crate separado
2. Convertir `rydit-rs` en lib + bin
3. Migrar funciones stdlib a módulos
4. Crear `crates/rydit-mod-scene/`

**Estructura**:
```
crates/
  rydit-core/      (núcleo, eval, executor)
  rydit-mod-scene/ (Scene, Camera, MObject)
  rydit-mod-physics/ (Physics module)
  rydit-mod-data/  (Data science module)
  rydit-rs/        (binario + stdlib)
```

---

## 🟡 PRIORIDAD MEDIA

### 4. Fix: physics::wave_1d 🔧
**Estado**: BUG CONOCIDO

**Problema**: Retorna 0 en ciertos casos
**Causa posible**: Parámetros fuera de rango
**Fix**: Revisar fórmula y rangos válidos

**Código actual**:
```rust
if name == "physics::wave_1d" && args.len() == 4 {
    // ...
    let amplitude = (k * x - omega * t).sin();
    return Valor::Num(amplitude);
}
```

**Fix propuesto**:
- Validar lambda > 0
- Validar freq > 0
- Clamp x, t a rangos razonables

---

### 5. Documentación: RyDit Science 📖
**Estado**: MEJORA OPCIONAL

**Tareas**:
1. Agregar ejemplos visuales (ASCII art)
2. Agregar casos de uso reales
3. Crear tutorial: "Curvas de Bezier en RyDit"
4. Actualizar README_SCIENCE.md con más ejemplos

---

## 🟢 PRIORIDAD BAJA

### 6. Limpieza de Código 🧹
**Estado**: MEJORA CONTINUA

**Tareas**:
1. `cargo fmt --all` - Formato consistente
2. `cargo clippy --fix` - Fix warnings
3. Eliminar código duplicado
4. Comentar funciones complejas

---

### 7. Demos Extendidas 🎮
**Estado**: OPCIONAL

**Demos propuestas**:
1. `demo_bezier_completa.rydit` - Visualización ASCII
2. `demo_fisica_completa.rydit` - Simulación orbital
3. `demo_datos_completa.rydit` - Análisis CSV real

---

## 📊 ROADMAP COMPLETO

### v0.7.1.x - Ciencia y Matemáticas ✅
- [x] v0.7.1.0: Física 2D (parcial)
- [x] v0.7.1.1: Animación 2D + Ilusiones
- [x] v0.7.1.2/3: Física + Datos
- [x] v0.7.1.4: Bezier + RyDit Science

### v0.7.2.x - Sistema de Nodos 🔜
- [ ] v0.7.2.0: Transform Trees + Scene Graph
- [ ] v0.7.2.1: RyditModule Trait
- [ ] v0.7.2.2: Módulos externos (crates)

### v0.7.3.x - Gráficos Avanzados
- [ ] v0.7.3.0: draw.bezier(), draw.path()
- [ ] v0.7.3.1: Sprite animations
- [ ] v0.7.3.2: Particle system 2.0

### v0.8.0.0 - Integración Completa
- [ ] v0.8.0.0: rydit-science crate
- [ ] v0.8.0.1: Plugin system
- [ ] v0.8.0.2: Documentación completa API

---

## 📝 NOTAS DE SESIÓN

### Pendientes de v0.7.1.4
- Tests de rydit-gfx requieren Termux-X11
- physics::wave_1d bug menor (no crítico)

### Ideas Futuras
- Unificar todos los módulos en `rydit::science::`
- Agregar `draw.bezier()` para renderizado directo
- Sistema de easing para animaciones (ya existe en anim::)

### Referencias
- Manim (3Blue1Brown) - Scene, Camera, MObject
- Bevy ECS - Transform, Node hierarchy
- De Casteljau algorithm - Bezier curves

---

## ✅ CHECKLIST PRE-SESIÓN

Antes de empezar v0.7.2.0:

- [ ] Termux-X11 instalado y configurado
- [ ] Backup completado (Google Drive)
- [ ] Repositorio limpio (sin archivos privados)
- [ ] QWEN.md actualizado
- [ ] Tests actuales pasando (163+)
- [ ] Documentación al día

---

<div align="center">

**🛡️ Próxima Sesión: v0.7.2.0 - Sistema de Nodos**

*Objetivo: Transform Trees + Scene Graph + RyditModule Trait*

</div>
