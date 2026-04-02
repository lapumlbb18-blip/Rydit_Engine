# 🛡️ PLAN MAESTRO v0.10.4 - SEGUNDA PARTE: PLATFORM SYNC + RYDIT-INPUT

**Documento de Trabajo para Modo Agente**

**Fecha**: 2026-03-31  
**Versión**: v0.10.4  
**Estado**: ⏸️ Pendiente de completar Fase 1  
**Autorización requerida**: Modo agente con permisos completos

---

## 📋 RESUMEN EJECUTIVO

Este documento describe la **Fase 2: Platform Sync + rydit-input crate + Corrección rydit-gfx**, enfocada en resolver los problemas de comunicación con Termux-X11 y la traducción OpenGL → Vulkan.

**Problema central detectado**: 
> "rydit-gfx no conecta bien con las capas de Termux-X11. OpenGL no traduce correctamente a Vulkan. El detalle quizás sea usar Zink, OpenGL ES, OpenGL o VirGL."

**Objetivo**: Crear una capa de abstracción de plataforma que maneje correctamente:
- Termux-X11 (Zink → Vulkan → Turnip)
- OpenGL ES (Android nativo)
- OpenGL (Linux/Windows)
- VirGL (VMs/contenedores)

**Duración estimada**: 3-4 días de trabajo en modo agente  
**Archivos críticos**: 10-15 archivos .rs + 1 crate nuevo  
**Riesgo**: Alto (FFI + drivers de GPU)

---

## 🔍 ANÁLISIS DEL PROBLEMA DE PLATFORM SYNC

### Contexto: Termux-X11 Architecture

```
┌─────────────────────────────────────────────────────────┐
│  Pipeline Gráfico en Termux-X11                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  RyDit (Rust)                                           │
│     ↓ (llamadas OpenGL)                                 │
│  raylib-rs / rydit-gfx                                  │
│     ↓ (OpenGL commands)                                 │
│  Zink (traductor OpenGL → Vulkan) ⚠️ ¿ESTÁ FALLANDO?   │
│     ↓ (Vulkan commands)                                 │
│  Turnip (driver Vulkan para Qualcomm)                   │
│     ↓ (GPU instructions)                                │
│  GPU Qualcomm (Adreno)                                  │
│     ↓                                                   │
│  Pantalla Android                                       │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Problemas Detectados (Hipótesis)

| # | Hipótesis | Evidencia | Verificación |
|---|-----------|-----------|--------------|
| **1** | Zink no está activo | `llvmpipe` en logs en vez de `zink` | `glxinfo \| grep "OpenGL renderer"` |
| **2** | Variables de entorno faltan | `MESA_LOADER_DRIVER_OVERRIDE` no set | `echo $MESA_LOADER_DRIVER_OVERRIDE` |
| **3** | rydit-gfx usa OpenGL directo | Debería usar OpenGL ES para Zink | Revisar `rydit-gfx/src/lib.rs` |
| **4** | Contexto OpenGL mal creado | Versión incorrecta (3.3 vs 3.0 ES) | Revisar `raylib::init_window()` config |
| **5** | FFI directo roto | Cambios en raylib-rs API | Comparar versión actual vs cuando funcionaba |
| **6** | VirGL necesario | Termux-X11 en contenedor/VM | `echo $XDG_SESSION_TYPE` |

---

## 🎯 OBJETIVOS DE LA FASE 2

### Objetivo 1: Diagnóstico de Platform Sync

**Tareas**:
1. [ ] Verificar variables de entorno en Termux-X11
2. [ ] Ejecutar `glxinfo` o `eglinfo` para ver renderer
3. [ ] Revisar logs de rydit-gfx al iniciar
4. [ ] Identificar si usa OpenGL, OpenGL ES, o Vulkan
5. [ ] Determinar si Zink está activo o fallback a llvmpipe

**Comandos de diagnóstico**:
```bash
# Ver renderer OpenGL
echo $MESA_LOADER_DRIVER_OVERRIDE
echo $GALLIUM_DRIVER
glxinfo | grep "OpenGL renderer"

# Ver si Zink está cargado
logcat | grep -i zink

# Ver sesión X11
echo $DISPLAY
echo $XDG_SESSION_TYPE

# Probar demo simple
./target/release/rydit-rs --gfx demos/test.rydit 2>&1 | head -50
```

**Entregable**: Informe de qué capa está fallando exactamente

---

### Objetivo 2: Crear rydit-input Crate

**Justificación**: Unificar eventos de todas las plataformas (mouse, teclado, táctil, gamepad)

**Estructura del crate**:
```
crates/rydit-input/
├── Cargo.toml
└── src/
    ├── lib.rs           # RyditEvent enum + InputBackend trait
    ├── events.rs        # Enum con todos los eventos
    ├── termux.rs        # Backend para Termux-X11
    ├── desktop.rs       # Backend para Linux/Windows/Mac
    └── gamepad.rs       # Soporte opcional de gamepad
```

**RyditEvent enum**:
```rust
pub enum RyditEvent {
    // Mouse/Ratón
    PointerMotion { x: f32, y: f32 },
    PointerButton { button: MouseButton, state: ButtonState },
    Scroll { delta_x: f32, delta_y: f32 },
    
    // Teclado
    Keyboard { key: KeyCode, modifiers: Modifiers, state: KeyState },
    TextInput { text: String },  // Para IME/teclado virtual
    
    // Táctil (Android nativo)
    TouchStart { id: u64, x: f32, y: f32, pressure: f32 },
    TouchMove { id: u64, x: f32, y: f32, pressure: f32 },
    TouchEnd { id: u64 },
    
    // Gamepad (opcional)
    GamepadButton { gamepad_id: u32, button: GamepadButton, state: ButtonState },
    GamepadAxis { gamepad_id: u32, axis: GamepadAxis, value: f32 },
}
```

**InputBackend trait**:
```rust
pub trait InputBackend {
    fn poll_events(&mut self) -> Vec<RyditEvent>;
    fn is_key_pressed(&self, key: KeyCode) -> bool;
    fn get_mouse_position(&self) -> (f32, f32);
    fn set_cursor_visible(&mut self, visible: bool);
}
```

**Tareas**:
1. [ ] Crear `crates/rydit-input/Cargo.toml`
2. [ ] Definir `RyditEvent` enum
3. [ ] Definir `InputBackend` trait
4. [ ] Implementar `TermuxBackend` (usa raylib)
5. [ ] Implementar `DesktopBackend` (usa winit o raylib)
6. [ ] Tests unitarios (simular eventos)
7. [ ] Integrar con `modules/input_map.rs`

**Criterio de éxito**: Eventos unificados funcionando en Termux-X11

---

### Objetivo 3: Corregir rydit-gfx Platform Sync

**Problema**: rydit-gfx no conecta bien con Termux-X11

**Solución propuesta**: Crear `PlatformSync` struct que maneje las diferencias

**PlatformSync struct**:
```rust
// crates/rydit-gfx/src/platform_sync.rs

pub enum GraphicsAPI {
    OpenGL,      // Linux/Windows desktop
    OpenGLES,    // Android nativo, iOS
    Vulkan,      // Android nativo (vía Zink o directo)
    Zink,        // OpenGL → Vulkan (Termux-X11)
    VirGL,       // VMs/contenedores
}

pub struct PlatformSync {
    api: GraphicsAPI,
    x11_sync: Option<X11Sync>,
    egl_sync: Option<EGLSync>,
    vulkan_sync: Option<VulkanSync>,
}

pub struct X11Sync {
    display: *mut c_void,  // Display* de X11
}

impl X11Sync {
    pub fn flush(&self) {
        unsafe {
            // XFlush(display)
            // XSync(display, False)
        }
    }
}

impl PlatformSync {
    pub fn auto_detect() -> Self {
        // Detectar automáticamente:
        // 1. Verificar $DISPLAY (X11)
        // 2. Verificar $MESA_LOADER_DRIVER_OVERRIDE (Zink)
        // 3. Verificar EGL (Android nativo)
        // 4. Fallback: OpenGL desktop
    }
    
    pub fn sync(&mut self) {
        // Llamar al sync correcto para la API detectada
        match self.api {
            GraphicsAPI::Zink | GraphicsAPI::OpenGL => self.x11_sync.flush(),
            GraphicsAPI::OpenGLES => self.egl_sync.wait(),
            GraphicsAPI::Vulkan => self.vulkan_sync.present(),
        }
    }
}
```

**Tareas**:
1. [ ] Crear `platform_sync.rs` en `rydit-gfx/src/`
2. [ ] Implementar `auto_detect()` para Termux-X11
3. [ ] Implementar `X11Sync` (XFlush, XSync)
4. [ ] Implementar `EGLSync` (eglWaitClient, eglSwapBuffers)
5. [ ] Integrar en game loop (llamar `sync()` después de cada frame)
6. [ ] Tests en Termux-X11

**Criterio de éxito**: 60 FPS estables sin tearing en Termux-X11

---

### Objetivo 4: Fix OpenGL ES vs OpenGL

**Problema potencial**: rydit-gfx usa OpenGL 3.3 pero Termux-X11 necesita OpenGL ES 3.0

**Solución**: Detectar plataforma y usar versión correcta

**Tareas**:
1. [ ] Verificar qué versión usa raylib actualmente
2. [ ] Si es OpenGL 3.3 → cambiar a OpenGL ES 3.0 para Android
3. [ ] O usar `GL_ARB_ES3_compatibility` si está disponible
4. [ ] Actualizar shaders si es necesario

**Configuración raylib**:
```rust
// En rydit-gfx/src/lib.rs o main.rs

// ANTES (posiblemente roto en Termux-X11)
raylib::init_window(1280, 720, "RyDit v0.10.4");

// DESPUÉS (forzar OpenGL ES)
#[cfg(target_os = "android")]
{
    // Usar OpenGL ES 3.0
    raylib::set_config_flags(ConfigFlags::MSAA_4X_HINT);
    // O quizás: raylib::set_gl_version(3, 3); // OpenGL ES 3.0
}
```

**Tareas**:
1. [ ] Investigar raylib-rs API para versión OpenGL
2. [ ] Probar con OpenGL ES 3.0
3. [ ] Si no funciona → fallback a OpenGL 3.3 + Zink
4. [ ] Documentar qué funciona en cada plataforma

---

### Objetivo 5: VirGL vs Zink vs OpenGL ES

**Análisis de opciones**:

| Opción | Ventajas | Desventajas | ¿Cuándo usar? |
|--------|----------|-------------|---------------|
| **Zink** | OpenGL → Vulkan, funciona en Termux-X11 | Overhead ~10-15% | Termux-X11 con GPU Qualcomm |
| **OpenGL ES** | Nativo en Android, sin traducción | No funciona en X11 directo | Android nativo (sin Termux-X11) |
| **VirGL** | Funciona en VMs/contenedores | Overhead ~20-30% | Termux en contenedor/VM |
| **OpenGL desktop** | Máximo rendimiento en Linux/Windows | No disponible en Android | Linux/Windows desktop |

**Tareas**:
1. [ ] Detectar automáticamente qué usar
2. [ ] Implementar fallback chain: Vulkan → OpenGL ES → Zink → VirGL → OpenGL
3. [ ] Logs claros para debugging

**Platform detection**:
```rust
fn detect_graphics_api() -> GraphicsAPI {
    // 1. Verificar si es Android nativo
    if cfg!(target_os = "android") {
        if std::env::var("DISPLAY").is_ok() {
            // Termux-X11 → Zink
            GraphicsAPI::Zink
        } else {
            // Android nativo → OpenGL ES
            GraphicsAPI::OpenGLES
        }
    }
    // 2. Verificar si es Linux/Windows
    else if cfg!(target_os = "linux") {
        if std::env::var("XDG_SESSION_TYPE").unwrap() == "wayland" {
            // Wayland → Vulkan preferido
            GraphicsAPI::Vulkan
        } else {
            // X11 → OpenGL
            GraphicsAPI::OpenGL
        }
    }
    // 3. Fallback
    else {
        GraphicsAPI::OpenGL
    }
}
```

---

## 📁 ARCHIVOS A CREAR/MODIFICAR

### Nuevos Archivos
1. `crates/rydit-input/Cargo.toml` - **Crate nuevo**
2. `crates/rydit-input/src/lib.rs` - RyditEvent + InputBackend
3. `crates/rydit-input/src/events.rs` - Enum de eventos
4. `crates/rydit-input/src/termux.rs` - Backend Termux-X11
5. `crates/rydit-input/src/desktop.rs` - Backend desktop
6. `crates/rydit-gfx/src/platform_sync.rs` - PlatformSync struct

### Archivos a Modificar
7. `crates/rydit-gfx/src/lib.rs` - Integrar PlatformSync
8. `crates/rydit-gfx/Cargo.toml` - Dependencias (egl, x11)
9. `crates/rydit-rs/src/main.rs` - Usar rydit-input + platform_sync
10. `crates/rydit-rs/src/modules/input_map.rs` - Integrar con rydit-input
11. `Cargo.toml` (workspace) - Añadir rydit-input al workspace

---

## 🔧 COMANDOS DE DIAGNÓSTICO

### Verificar Platform Sync

```bash
# 1. Ver renderer OpenGL
export DISPLAY=:0
glxinfo | grep "OpenGL renderer"
# Debería decir: "Zink" o "Adreno" o "llvmpipe"

# 2. Ver si Zink está activo
logcat -d | grep -i zink | tail -20

# 3. Ver variables de entorno
echo "DISPLAY=$DISPLAY"
echo "MESA_LOADER_DRIVER_OVERRIDE=$MESA_LOADER_DRIVER_OVERRIDE"
echo "GALLIUM_DRIVER=$GALLIUM_DRIVER"
echo "TU_DEBUG=$TU_DEBUG"

# 4. Probar demo con logs
RUST_LOG=debug ./target/release/rydit-rs --gfx demos/test.rydit 2>&1 | grep -i "gfx\|gl\|vulkan"

# 5. Ver FPS y rendimiento
./target/release/rydit-rs --gfx demos/test.rydit
# Observar si hay tearing, stuttering, o FPS inestables
```

### Verificar rydit-input

```bash
# 1. Compilar crate
cargo build -p rydit-input --release

# 2. Tests unitarios
cargo test -p rydit-input

# 3. Demo de input
./target/release/examples/test_input_events

# 4. Ver eventos en tiempo real
./target/release/rydit-rs --gfx demos/test_input_map.rydit
# Presionar teclas y verificar que se registran
```

---

## 📊 MÉTRICAS DE ÉXITO FASE 2

| Métrica | Antes | Después | Objetivo |
|---------|-------|---------|----------|
| **Renderer** | ¿? (desconocido) | Zink/Adreno | ✅ GPU real |
| **FPS estables** | 30-50 (inestable) | 60 (estable) | ✅ Sin stuttering |
| **Tearing** | ⚠️ Presente | ❌ Ausente | ✅ Platform Sync |
| **Eventos unificados** | ❌ No existe | ✅ rydit-input | ✅ 5 tipos de eventos |
| **Input Termux** | ⚠️ 7 teclas | ✅ 100+ teclas | ✅ Todas mapeadas |
| **Multi-plataforma** | ❌ Solo Termux | ✅ Termux + Linux | ✅ Backends separados |

---

## ⚠️ RIESGOS Y MITIGACIÓN

### Riesgo 1: FFI OpenGL inestable
**Mitigación**: Usar `gl-rs` crate (oficial, mantenido)

### Riesgo 2: Zink no disponible en Termux
**Mitigación**: Fallback a llvmpipe (CPU) o OpenGL ES nativo

### Riesgo 3: raylib-rs no soporta OpenGL ES
**Mitigación**: FFI directo a EGL/GLES2 o cambiar a sokol-rs

### Riesgo 4: rydit-input muy complejo
**Mitigación**: Empezar solo con teclado+mouse, añadir táctil después

### Riesgo 5: VirGL necesario pero no documentado
**Mitigación**: Investigar en foros de Termux-X11, preguntar en Discord

---

## 📝 ENTREGABLES FASE 2

Al finalizar esta fase, entregaré:

1. ✅ **rydit-input crate** - Eventos unificados (teclado, mouse, táctil)
2. ✅ **PlatformSync struct** - Detección automática de plataforma
3. ✅ **X11Sync + EGLSync** - Sync correcto para Termux-X11
4. ✅ **Diagnóstico completo** - Qué API gráfica usa cada plataforma
5. ✅ **Demo funcional** - 60 FPS estables en Termux-X11
6. ✅ **Documentación** - Cómo configurar variables de entorno

---

## 🔗 INTEGRACIÓN CON FASE 1

**Dependencias**:
- Fase 2 requiere Fase 1 completada (Input Map integrado)
- rydit-input usa `modules/input_map.rs` para mapeo de acciones
- PlatformSync usa `rydit-gfx` que se arregló en Fase 1

**Secuencia**:
```
Fase 1: Reconexión (2-3 días)
   ↓
Fase 2: Platform Sync + rydit-input (3-4 días)
   ↓
Fase 3: Demo completo + validación (1-2 días)
```

---

## 🚀 SIGUIENTES FASES

Después de completar Fase 1 + Fase 2:

**Fase 3: Demo Completo .rydit**
- Integrar todo: Input + Assets + Physics + Platform Sync
- Crear `test_completo.rydit`
- Validar en Termux-X11

**Fase 4: GPU Instancing + ECS**
- FFI OpenGL con `gl-rs`
- Shaders GLSL
- bevy_ecs integrado
- 100K partículas @ 60 FPS

---

## ✋ AUTORIZACIÓN REQUERIDA

**Solicito autorización para**:

- [ ] Crear crate nuevo `rydit-input` (6 archivos nuevos)
- [ ] Modificar `rydit-gfx` (platform_sync.rs + lib.rs)
- [ ] Ejecutar comandos de diagnóstico (glxinfo, logcat, etc.)
- [ ] Modificar variables de entorno en scripts
- [ ] FFI directo a OpenGL ES / EGL si es necesario
- [ ] Refactorizar game loop para usar PlatformSync

**Modo de trabajo**: Agente autónomo con reportes cada 2-3 horas

**Señal de inicio**: Usuario responde "autorizado Fase 2" o "procede"

---

<div align="center">

**🛡️ RyDit v0.10.4 - FASE 2: PLATFORM SYNC + RYDIT-INPUT**

*Zink + OpenGL ES + rydit-input | 3-4 días | 10-15 archivos + 1 crate*

**Próximo: Completar Fase 1 primero**

</div>

---

## 📎 APÉNDICE: Variables de Entorno para Termux-X11

### Configuración recomendada (ejecutar_termux.sh)

```bash
#!/bin/bash
# ejecutar_termux.sh - Configuración óptima para Termux-X11

# Display de X11
export DISPLAY=:0

# Zink (OpenGL → Vulkan)
export MESA_LOADER_DRIVER_OVERRIDE=zink
export GALLIUM_DRIVER=zink

# Turnip (driver Vulkan para Qualcomm)
export TU_DEBUG=noconform

# Para debugging
export RUST_LOG=debug
export VK_INSTANCE_LAYERS=VK_LAYER_KHRONOS_validation

# Ejecutar RyDit
./target/release/rydit-rs --gfx "$@"
```

### Verificación

```bash
# Después de ejecutar el script, verificar:
glxinfo | grep "OpenGL renderer"
# Debería decir: "Zink [Adreno XXX]"

# Ver si Vulkan está disponible
vulkaninfo | head -20
```

---

**Notas para el agente**:
- Este documento depende de la Fase 1 completada
- Prioriza: Platform Sync primero, rydit-input después
- Si Zink no funciona → investigar VirGL o OpenGL ES nativo
- Tests en Termux-X11 después de cada cambio
- Documenta TODO (variables de entorno, comandos, errores)
