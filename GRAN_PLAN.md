# 🛡️ Ry-Dit: Gran Plan Estratégico v0.22.0+

**Fecha**: 2026-04-17
**Estado**: En curso (Fases 1 y 2 completadas)
**Visión**: Pasar de un "conjunto de crates" a un "ecosistema de desarrollo creativo".

---

## 1. Arquitectura de "Puerta Principal" (ry-rs)
Actualmente, el usuario tiene que navegar por muchos crates. Debemos consolidar `ry-rs` como la única puerta de entrada necesaria para el 90% de los proyectos.

- **Re-exports**: `ry-rs/lib.rs` debe exportar todo lo necesario (`rybot`, `ry-gfx`, `ry-input`, `ry-anim`).
- **Simplificación**: Un usuario debería poder hacer `use ry_rs::prelude::*;` y tener el motor listo.

## 2. ryArt: El Motor de Expresión
Inspirado en Processing y arte generativo, `ryArt` será un crate que simplifique la creación de visuales complejos.

- **Firma**: Funciones simples como `circle(x, y, r)`, `noise(t)`, `lerp_color(a, b, f)`.
- **Integración**: Usará `gpu_instancing` por debajo para manejar miles de formas sin lag.
- **Uso**: Ideal para prototipado rápido y visualización de datos científicos (`ry-science`).

## 3. Editor Visual (Prototipo v1)
Necesitamos dejar de programar "a ciegas". El editor no será un programa aparte, sino un modo dentro del mismo motor.

- **Inspector**: Panel lateral para modificar variables en tiempo real (vía `serde` + `migui`).
- **SceneTree**: Visualizar la jerarquía de entidades.
- **Tilemap Editor**: Herramienta integrada para pintar niveles y exportarlos a `.ryscene`.

## 4. Evolución del Asset Pipeline
Tras la integración del `AssetServer` en `ry-rs`, los siguientes pasos son:

- **Hot Reload**: Implementar un `FileWatcher` que recargue assets automáticamente al guardar en el disco.
- **Typed Deserialization**: Cargar niveles enteros (`.ryscene`) directamente a structs de Rust usando `load_typed<T>`.
- **SAZ (Shield Archive)**: Formato de empaquetado para distribución de juegos.

## 5. Evaluación del Código "Durmiente"
Antes de borrar, evaluaremos cada módulo marcado como muerto:

- **Módulos de Quest/Diálogos**: ¿Son placeholders para un RPG futuro? Se mantendrán si hay un plan de integrarlos en `rybot`.
- **Físicas Experimentales**: Si no se usan en demos, se moverán a una carpeta `experimental/` en lugar de borrarlos.
- **Demos Obsoletas**: Se consolidarán en una "Galería de Ejemplos" única.

---

## 📅 Roadmap de Ejecución (Próximos Pasos)

1.  **[PROX]** Re-exports masivos en `ry-rs/lib.rs`.
2.  **[PROX]** Creación de `crates/ry-art` (Estructura base).
3.  **[PROX]** Implementación de `FileWatcher` en `ry-loader`.
4.  **[PLAN]** Diseño del sistema de Inspector para `migui`.

---
*Este plan es una hoja de ruta viva. Ry-Dit está listo para la madurez industrial.*
