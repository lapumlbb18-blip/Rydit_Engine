# BITACORA.md

## 2026-04-15
- **Avance v0.20.0 - Asset Pipeline**:
    - Definición del trait `AssetProvider` en `ry-loader`.
    - Implementación de `AssetServer` con caché (`RwLock`) y capa de compresión (`Compressor`).
    - Integración de `ry-loader` en `ry-gfx` (Sdl2AssetProvider base).
    - Unificación del módulo `assets.rs` en `ry-rs` para usar el nuevo `AssetServer` (soporte híbrido legado/nuevo).
    - Verificación exitosa de compilación y tests unitarios.
- **Próximos pasos**: Implementación de la carga real (linking `sdl2_image`) y/o el editor visual de tilemaps.

## 2026-04-16 (Sesión v0.21.0)
- **Modernización Action Assets**: 
    - Migración de `action_assets.rs` a tipos de datos fuertes (`SpriteAnimation`, `SpriteSheet`).
    - Implementación de wrappers legacy para mantener compatibilidad con el motor actual.
- **Higiene del Proyecto**: 
    - Resolución de colisiones de nombres renombrando `particles.rs` a `anim_particles`, `gpu_particles` y `script_particles`.
    - Actualización de referencias en todo el workspace (demos, binarios, librerías).
- **Asset Pipeline v0.20.0 (Fase 2)**: 
    - Extensión de `AssetType` con soporte para Animaciones, SpriteSheets y Config.
    - Implementación de `get_typed<T>` en `AssetServer` para deserialización automática a structs de Rust.
- **Documentación**: 
    - Creación de README.md para los 5 crates restantes (`ry-lexer`, `ry-parser`, `events-ry`, `ry-loader`, `blast-core`).
- **Estabilización**: 
    - El proyecto compila al 100% en todos sus crates principales tras las refactorizaciones.

## 2026-04-17 (Sesión v0.22.0 - ryArt & Unified Input)
- **Asset Pipeline v2**:
    - Implementación de `load_typed<T>` en `AssetServer` para deserialización automática.
    - Integración de `Sdl2AssetProvider` con carga real de bytes desde disco.
    - Validación exitosa en `demo_completo_sdl2.rs`.
- **Unificación de Input**:
    - Fusión de `ry-input` (acciones) y `events-ry` (eventos).
    - El `InputManager` ahora rastrea internamente la posición del ratón y estados de botones.
    - Implementación de `actualizar_input_unificado` en `Sdl2Backend` para sincronización automática.
- **ryArt - El Motor de Expresión**:
    - Creación del crate `ry-art` para arte generativo ("IA sin IA").
    - Implementación del trait `Brush` y `PhysicsBrush` (brocha con inercia física).
    - Creación de `demo_ryart_cyberpunk.rs` (Estilo Neon-Noir) para validar el flujo completo.
- **Estabilización**:
    - Corrección de advertencias "unexpected cfg" en todo el workspace.
    - Resolución de dependencias cruzadas entre `ry-gfx`, `events-ry` y `ry-art`.

## 2026-04-17 (Sesión v0.23.0 - Consolidación Ry-Dit)
- **Puerta Principal (ry-rs)**:
    - Rediseño de `lib.rs` para actuar como orquestador maestro.
    - Implementación de `ry_rs::prelude` para desarrollo rápido.
- **Arquitectura de Intérprete**:
    - Migración de ~5000 líneas de lógica desde `main.rs` a `interpreter.rs`.
    - `main.rs` convertido en un binario minimalista (Thin Wrapper).
- **Higiene Estructural**:
    - Unificación de rutas de importación (`crate::interpreter::...`) en todos los módulos.
    - Corrección de errores de visibilidad y tipos detectados por el compilador tras el movimiento de código.
    - Eliminación de imports inactivos en módulos core.
- **Punto de Restauración**:
    - Creación de rama `backup-v0.22.0-pre-consolidation` para seguridad.
