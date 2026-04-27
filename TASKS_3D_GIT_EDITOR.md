# 🚀 TASKS — 3D Touch, GitHub CI/CD & Editor

**Fecha**: 19 de Abril, 2026
**Foco Principal**: Validación de UX 3D en Android, Infraestructura de Despliegue y Herramientas de Edición.
**Estado**: 🛠️ En Desarrollo Paralelo

---

## 🎮 PILLAR 1: 3D Touch & Camera Follow (UX/UI 3D)
*Validar el "feeling" del motor en dispositivos móviles (Android/Termux/X11).*

| ID | Tarea | Detalle Técnico | Crate | Estado |
|---|---|---|---|---|
| 3D-1 | **Controlador Touch 3D** | Mapear gestos touch a traslación/rotación de un objeto. | `ry-input` / `ry3d-gfx` | ⏳ Pendiente |
| 3D-2 | **Camera Follow Suave** | Implementar `Lerp` para que la cámara siga al jugador sin tirones. | `ry3d-gfx` | ⏳ Pendiente |
| 3D-3 | **Joystick Virtual (Opcional)** | Renderizar un joystick 2D sobre el viewport 3D para control preciso. | `migui` / `ry-gfx` | ⏳ Pendiente |
| 3D-4 | **Demo de Validación** | Crear `demo_touch_3d.rydit` integrando movimiento + cámara. | `demos/` | ⏳ Pendiente |

---

## 🤖 PILLAR 2: GitHub Runners & Binary Builds (CI/CD)
*Garantizar que el motor compile siempre y sea fácil de distribuir.*

| ID | Tarea | Detalle Técnico | Archivo | Estado |
|---|---|---|---|---|
| GIT-1 | **Matrix Build CI** | Configurar builds para Linux, Windows (cross-compile) y Android. | `.github/workflows/main.yaml` | 🟡 Parcial |
| GIT-2 | **Automated Releases** | Generar binarios (runners) automáticamente al subir tags. | `.github/workflows/main.yaml` | ⏳ Pendiente |
| GIT-3 | **Test Runner Integration** | Ejecutar todos los tests (~260) en el entorno de GitHub. | `Cargo.toml` / CI | ⏳ Pendiente |
| GIT-4 | **Build Artifacts** | Subir los ejecutables compilados como artefactos de la acción. | CI | ⏳ Pendiente |

---

## 🏗️ PILLAR 3: Editor & Viewports (Tooling)
*Transformar Rydit de un motor de código a un entorno visual.*

| ID | Tarea | Detalle Técnico | Crate | Estado |
|---|---|---|---|---|
| ED-1 | **Viewport Genérico** | Encapsular el renderizado 3D de `ry3d-gfx` en un panel de `migui`. | `ry3d-gfx` / `migui` | ✅ Completado |
| ED-2 | **Inspector de Entidades** | Panel lateral con submenús para modificar propiedades en runtime. | `rybot` / `migui` | 🟡 En Progreso |
| ED-3 | **Gizmos y Navegación** | Grid, Zoom (+/-) y Pan con botones/scroll en Viewport. | `ry-editor` / `ry-gfx` | 🟡 En Progreso |
| ED-4 | **Sistema de Layout** | Guardar y cargar la disposición de ventanas del editor. | `ry-config` | ⏳ Pendiente |

---

## 🔄 TAREAS PARALELAS Y MANTENIMIENTO
*No olvidar la estabilidad del core.*

- [ ] **Limpieza de Crates Huérfanos**: Integrar `ry-god` y `ry-script` (ver `TASKS.md`).
- [ ] **READMEs Faltantes**: `ry-lexer`, `ry-parser`, `events-ry`, `ry-loader`, `blast-core`.
- [ ] **Unificación de Input**: Resolver duplicidad entre `events-ry` y `ry-input`.

---

<div align="center">

**🛡️ Foco: Estabilidad, Potencia Visual y Facilidad de Uso**

*Hacia la v0.23.0 — El nacimiento del Editor Visual*

</div>
