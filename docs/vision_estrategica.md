# 🎯 Visión Estratégica v0.13.0+ — Ecosistema Ry-Dit

**Fecha**: 2026-04-04
**Versión**: v0.13.0

---

## 🗺️ VISIÓN GENERAL

Ry-Dit no es solo un motor de juegos. Es un **ecosistema de crates interconectados** donde cada uno tiene un rol estratégico en crates.io y en la arquitectura.

---

## 🛡️ v-shield → Crate de Compatibilidad Multiplataforma

### Rol Estratégico
**Wrapper/Adapter** que une el ecosistema Rust (raylib, SDL2, nativo) con Ry-Dit.

### Funcionalidades Planificadas

| Módulo | Descripción | Estado |
|--------|-------------|--------|
| **platform_sync** | Sync eventos entre raylib/SDL2/Rust nativo | ⏳ En ry-gfx (inactivo) |
| **config_defaults** | Configuraciones predeterminadas por plataforma | ⏳ Pendiente |
| **event_registry** | Registro unificado de eventos (teclado, mouse, gamepad) | ⏳ Pendiente |
| **security_layer** | Capa segura alternativa para ejecución de scripts | ⏳ Pendiente |
| **dependency_check** | Tests de verificación de dependencias | ⏳ Pendiente |
| **connection_test** | Conexión directa con backends gráficos | ⏳ Pendiente |

### Arquitectura Propuesta

```
┌─────────────────────────────────────────────────────┐
│                    v-shield                         │
│                                                     │
│  ┌─────────────┐  ┌─────────────┐  ┌────────────┐ │
│  │   raylib    │  │    SDL2     │  │  Rust N    │ │
│  │  backend    │  │  backend    │  │  backend   │ │
│  └──────┬──────┘  └──────┬──────┘  └─────┬──────┘ │
│         │                │                │         │
│  ┌──────▼────────────────▼────────────────▼──────┐ │
│  │          Platform Abstraction Layer           │ │
│  │  - Event normalization                        │ │
│  │  - Config defaults por OS                     │ │
│  │  - Security sandbox                           │ │
│  └───────────────────────┬───────────────────────┘ │
│                          │                         │
│  ┌───────────────────────▼───────────────────────┐ │
│  │           Ry-Dit Core (ry-rs)                 │ │
│  │  - Parser, VM, eval, modules                  │ │
│  └───────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────┘
```

### Antes de GitHub Actions

v-shield dará un **panorama de lo que falta**:
```bash
# Test de verificación de dependencias
cargo test -p v-shield --test verify_deps

# Test de conexión directa
cargo run --bin v-shield -- --test-connection

# Reporte de compatibilidad
cargo run --bin v-shield -- --platform-report
```

Esto nos dice **qué falta** antes de automatizar CI/CD.

---

## 🎬 ry-anim → Crate Showcase para crates.io

### Rol Estratégico
**Llamar la atención** en crates.io. Es el crate "vitrina" que demuestra que Ry-Dit es serio.

### Lo que YA tiene

| Módulo | Implementado | Descripción |
|--------|-------------|-------------|
| **12 principios Disney** | 3/13 | Squash&Stretch, Anticipation, Easing |
| **particles.rs** | ✅ | Fire, smoke, spark, explosion presets |

### Lo que DEBE tener (efectos visuales)

| Efecto | Descripción | Prioridad |
|--------|-------------|-----------|
| **neon.rs** | Efectos neón (glow, bloom simulado) | 🔴 Alta |
| **fx.rs** | Efectos especiales (screen shake, flash, fade) | 🔴 Alta |
| **bw.rs** | Filtros blanco y negro, sepia, invert | 🟡 Media |
| **dlss.rs** | Upscaling simulado (FSR alternative) | 🟡 Media |
| **trails.rs** | Estelas de movimiento | 🟡 Media |
| **ripple.rs** | Ondas de distorsión | 🟢 Baja |

### Principios Disney Pendientes (9 restantes)

| # | Principio | Implementación Planificada |
|---|-----------|---------------------------|
| 4 | Staging | Puesta en escena con cámaras |
| 5 | Straight Ahead / Pose to Pose | Dos modos de animación |
| 6 | Follow Through | Inercia de partes del cuerpo |
| 8 | Arcs | Trayectorias curvas naturales |
| 9 | Secondary Action | Acción secundaria automática |
| 10 | Timing | Velocidad y ritmo variable |
| 11 | Exaggeration | Exageración controlada |
| 12 | Solid Drawing | Volumen 3D en sprites 2D |
| 13 | Appeal | Atractivo visual automático |

### Por qué ry-anim es estratégico

1. **Diferenciador único**: Ningún crate en crates.io tiene "12 principios Disney"
2. **Visualmente impresionante**: Efectos neon + partículas + animación = demo atractivo
3. **Sin dependencias pesadas**: Funciona con math pura (sin raylib obligatorio)
4. **Cross-platform**: Al no depender de GPU directa, funciona en cualquier lado

---

## 📊 MAPA DE CRATES — Roles Estratégicos

```
┌──────────────────────────────────────────────────────────────┐
│                    ECOSISTEMA RY-DIT                         │
│                                                              │
│  ┌─────────────────┐    ┌─────────────────┐                 │
│  │   ry-core       │    │   ry-god        │                 │
│  │   (corazón)     │    │   (seguridad)   │                 │
│  │   ✅ pub crates │    │   ✅ pub crates │                 │
│  └────────┬────────┘    └────────┬────────┘                 │
│           │                      │                          │
│  ┌────────▼────────┐    ┌───────▼─────────┐                 │
│  │   ry-lexer      │    │   v-shield      │                 │
│  │   ry-parser     │    │   (compat)      │                 │
│  │   ry-vm         │    │   ⏳ planificado │                 │
│  └────────┬────────┘    └───────┬─────────┘                 │
│           │                      │                          │
│  ┌────────▼──────────────────────▼────────┐                 │
│  │            ry-rs (binary)              │                 │
│  │   Parser → VM → Eval → Modules → GFX   │                 │
│  └────────┬───────────────────────┬───────┘                 │
│           │                       │                         │
│  ┌────────▼────────┐    ┌────────▼─────────┐                │
│  │   ry-gfx        │    │   ry-physics     │                │
│  │   (render)      │    │   (math puras)   │                │
│  │   raylib+SDL2   │    │   proyectiles    │                │
│  └────────┬────────┘    └──────────────────┘                │
│           │                                                  │
│  ┌────────▼────────┐    ┌──────────────────┐                │
│  │   ry-anim ⭐    │    │   ry-science     │                │
│  │   (showcase)    │    │   (geometría)    │                │
│  │   Disney+FX     │    │   imposible      │                │
│  └─────────────────┘    └──────────────────┘                │
│                                                              │
│  ⭐ = Crate "vitrina" para llamar atención en crates.io     │
│  🔧 = Crate infraestructura (no visible pero esencial)      │
│  🛡️ = Crate seguridad/compatibilidad                        │
└──────────────────────────────────────────────────────────────┘
```

### Clasificación por Rol

| Rol | Crates | Prioridad |
|-----|--------|-----------|
| **⭐ Showcase** (vitrina) | ry-anim, ry-science | Alta — llaman atención |
| **🔧 Core** (esenciales) | ry-core, ry-lexer, ry-parser, ry-vm | Alta — sin ellos no hay motor |
| **🎨 Render** (gráficos) | ry-gfx, ry-ecs | Media — dependen de platform |
| **🧮 Math** (cálculo) | ry-physics | Media — fórmulas puras |
| **🛡️ Compat** (puente) | v-shield | Alta — multiplataforma |
| **🔒 Security** | ry-god | Media — publicado |
| **🌐 Network** | ry-stream | Baja — streaming LAN |

---

## 🎯 ESTRATEGIA DE PUBLICACIÓN

### Orden de publicación en crates.io

| Orden | Crate | Razón |
|-------|-------|-------|
| 1 | ✅ ry-god | Ya publicado, security |
| 2 | ry-core | Base de todos los demás |
| 3 | ry-lexer | Independiente, zero-copy |
| 4 | ry-parser | Depende de lexer |
| 5 | ⭐ ry-anim | **Showcase** — llama atención |
| 6 | ry-physics | Math puras, fácil publish |
| 7 | ry-science | Geometría única |
| 8 | 🛡️ v-shield | Compatibilidad — clave |
| 9 | ry-gfx | Depende de v-shield |
| 10 | ry-ecs | ECS genérico |
| 11 | ry-stream | Network |
| 12 | ry-script | Script loading |

### ry-anim como "Hook"

**Título en crates.io**: "12 Principles of Disney Animation in Rust"
**Descripción**: "Create beautiful animations with proven Disney principles. Includes particle effects, easing functions, squash & stretch, and more."

Esto atrae a:
- Developers de juegos
- Artistas técnicos
- Estudiantes de animación
- Comunidades de Rust

---

## 📋 PRÓXIMOS PASOS

### Inmediatos (v0.13.0)
1. ✅ Limpieza de basura (-17,604 líneas)
2. ⏳ Consolidar collision AABB
3. ⏳ Agregar easing a `math::` (sin tocar ry-anim)
4. ⏳ Probar demos clave

### Corto plazo (v0.13.1)
1. ⏳ ry-anim: Agregar neon.rs, fx.rs, bw.rs
2. ⏳ v-shield: Empezar platform abstraction
3. ⏳ v-shield: Tests de verificación de dependencias

### Mediano plazo (v0.14.0)
1. ⏳ v-shield completo con config_defaults
2. ⏳ ry-anim: 9 principios Disney restantes
3. ⏳ ry-anim: dlss.rs upscaling
4. ⏳ Publicar ry-core, ry-lexer, ry-parser

---

<div align="center">

**🛡️ Ry-Dit — Ecosistema, no solo motor**

*ry-anim = vitrina | v-shield = puente | ry-core = corazón*

</div>
