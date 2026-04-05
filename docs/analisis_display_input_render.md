# 🔍 Análisis Pipeline Display + Input + Render en Termux-X11

**Fecha**: 2026-04-04
**Dispositivo**: Redmi Note 8 (Android 11)
**Terminal**: Termux + Termux-X11

---

## 📊 ARQUITECTURA ACTUAL DETECTADA

### Cadena Completa

```
┌──────────────────────────────────────────────────────────────────┐
│                    ANDROID 11 (Redmi Note 8)                     │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │           Termux-X11 (X Server en Android)               │   │
│  │  PID: 6252  |  app_process -Xnoimage-dex2oat            │   │
│  │  Display: :0                                              │   │
│  │  Window Manager: openbox (PID: 6827)                     │   │
│  │  DBus session: 6709-6710                                  │   │
│  │  gvfsd: 6730                                              │   │
│  └──────────────────────┬───────────────────────────────────┘   │
│                         │ DISPLAY=:0                             │
│  ┌──────────────────────▼───────────────────────────────────┐   │
│  │              Aplicación SDL2 / Raylib                     │   │
│  │  SDL_VIDEODRIVER=x11 (no configurado explícitamente)     │   │
│  │  SDL2 versión: 2.32.10                                    │   │
│  │  Renderer por defecto: ? (no verificado)                 │   │
│  └──────────────────────┬───────────────────────────────────┘   │
│                         │                                        │
│  ┌──────────────────────▼───────────────────────────────────┐   │
│  │              Mesa Zink (OpenGL → Vulkan)                 │   │
│  │  MESA_LOADER_DRIVER_OVERRIDE=zink                        │   │
│  │  DRI3=1                                                   │   │
│  │  Vulkan SDK: 1.4.341                                      │   │
│  │  VK_KHR_xlib_surface ✅                                   │   │
│  │  VK_KHR_xcb_surface ✅                                    │   │
│  │  VK_KHR_wayland_surface ✅                                │   │   │
│  │  /dev/dri → NO ACCESIBLE                                  │   │
│  └──────────────────────┬───────────────────────────────────┘   │
│                         │                                        │
│  ┌──────────────────────▼───────────────────────────────────┐   │
│  │              SurfaceFlinger (Android compositor)          │   │
│  │  Compone la ventana de Termux-X11                        │   │
│  │  GLES driver del dispositivo (Adreno)                    │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │              INPUT (Teclado Android)                      │   │
│  │  /dev/input → NO ACCESIBLE desde Termux                   │   │
│  │  Teclado virtual → Termux-X11 → X11 KeyEvents            │   │
│  │  NO envía repeat:true (solo primera pulsación)            │   │
│  │  Solución actual: repeat:false + acción por pulsación     │   │
│  └──────────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────────┘
```

---

## 🔑 HALLAZGOS CLAVE

### 1. Servidor X11: Termux-X11
- **NO es Xorg ni Xwayland** — es un X server custom para Android
- Corre como `app_process` (proceso Android, no daemon Linux normal)
- Display: `:0`
- Window Manager: `openbox`
- dbus-launch + dbus-daemon activos
- gvfsd (GNOME Virtual File System) corriendo

### 2. GPU: Zink sobre Vulkan
- **Zink** = driver OpenGL sobre Vulkan (Mesa project)
- `MESA_LOADER_DRIVER_OVERRIDE=zink` → fuerza uso de Zink
- `DRI3=1` → DRI3 activado
- Vulkan SDK 1.4.341 instalado
- **Problema**: `/dev/dri` NO accesible (esperado en Android)
- Extensiones VK disponibles: xlib_surface, xcb_surface, wayland_surface

### 3. SDL2: Driver NO explícito
- SDL2 versión 2.32.10 instalado
- **SDL_VIDEODRIVER NO está configurado** → SDL2 autodetecta
- Con DISPLAY=:0 presente, SDL2 debería elegir X11 automáticamente
- **No hay SDL_RENDER_DRIVER configurado**

### 4. Input: Solo primera pulsación
- `/dev/input` NO accesible desde Termux (sin root)
- Teclado virtual Android → Termux-X11 → X11 KeyEvents
- **NO envía repeat:true** (teclas mantenidas no funcionan)
- Solución actual: cada pulsación = acción individual

### 5. Raylib: nobuild feature
- raylib = "5.5.1" con feature "nobuild"
- Significa que ry-gfx compila raylib desde source
- Depende del mismo Zink + X11 que SDL2

---

## ⚠️ PUNTOS CRÍTICOS / PROBLEMAS DETECTADOS

### Problema 1: SDL_VIDEODRIVER no explícito
**Riesgo**: SDL2 puede elegir otro backend (offscreen, dummy) si no se fuerza X11.

**Solución**:
```bash
export SDL_VIDEODRIVER=x11
```

### Problema 2: SDL_RENDER_DRIVER no configurado
**Riesgo**: SDL2 puede elegir software renderer (lento) en vez de GPU.

**Solución**:
```bash
export SDL_RENDER_DRIVER=opengl
# o
export SDL_RENDER_DRIVER=opengles2
```

### Problema 3: No hay DRI3/PRESENT verificado en X11
**Riesgo**: Sin DRI3, el rendering es indirecto y lento.

**Verificación pendiente**: `xdpyinfo` no disponible (package no instalado).

### Problema 4: SDL2 no maneja TEXTINPUT para Android
**Impacto**: migui y toolkit-ry NO reciben texto del teclado Android.
- `input_map.rs` solo maneja KeyDown/KeyUp
- `input_ime.rs` es simulación (no usa JNI real)
- **Ningún módulo llama a SDL_StartTextInput() / SDL_StopTextInput()**

### Problema 5: /dev/input inaccesible
**Impacto**: No se pueden leer eventos raw del touchscreen/teclado.
Solo se reciben vía X11 events del Termux-X11 server.

---

## 🎯 CADENA DE RENDERIZADO ACTUAL

### SDL2 → X11 → Zink → Vulkan → SurfaceFlinger

```
SDL2_CreateRenderer()
    ↓
SDL chooses X11 window (DISPLAY=:0)
    ↓
SDL chooses OpenGL or OpenGLES renderer
    ↓
Mesa Zink intercepts GL calls
    ↓
Zink translates GL → Vulkan commands
    ↓
Vulkan driver (Adreno) ejecuta
    ↓
SurfaceFlinger compone en pantalla Android
```

### Raylib → X11 → Zink → Vulkan → SurfaceFlinger

```
rlInit() → Create X11 window (DISPLAY=:0)
    ↓
rlCreateTexture() → GL calls → Zink → Vulkan
    ↓
rlDraw() → GL commands → Zink → Vulkan
    ↓
SurfaceFlinger
```

**Ambos usan la misma cadena.** La diferencia es que SDL2 tiene event loop con KeyDown/KeyUp, mientras Raylib usa polling (que NO funciona en Android).

---

## 🔧 CONFIGURACIÓN ÓPTIMA DETECTADA

### Variables de entorno requeridas

```bash
# Display server
export DISPLAY=:0

# GPU: Zink sobre Vulkan
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

# SDL2: Forzar X11 + OpenGL
export SDL_VIDEODRIVER=x11
export SDL_RENDER_DRIVER=opengles2    # OpenGLES2 es más estable en Android
export SDL_HINT_VIDEO_X11_FORCE_EGL=1  # Forzar EGL sobre GLX

# Input: separar mouse/touch
export SDL_HINT_ANDROID_SEPARATE_MOUSE_AND_TOUCH=1
export SDL_HINT_TOUCH_MOUSE_EVENTS=1
export SDL_HINT_ENABLE_SCREEN_KEYBOARD=1

# Vulkan (para Zink)
export VK_ICD_FILENAMES=/data/data/com.termux/files/usr/share/vulkan/icd.d/*.json
```

### Código SDL2 faltante para input completo

```rust
// Al inicializar:
sdl2::hint::set("SDL_HINT_VIDEO_X11_FORCE_EGL", "1");
sdl2::hint::set("SDL_HINT_ANDROID_SEPARATE_MOUSE_AND_TOUCH", "1");
sdl2::hint::set("SDL_HINT_TOUCH_MOUSE_EVENTS", "1");

// Cuando un widget de texto recibe foco:
// (esto activa el teclado virtual Android)
let _ = sdl_context.set_hint("SDL_HINT_ENABLE_SCREEN_KEYBOARD", "1");

// En el event loop, manejar TextInput:
for event in event_pump.poll_iter() {
    match event {
        Event::TextInput { text, .. } => {
            // RECIBE CARACTERES DEL TECLADO ANDROID
            ime_state.push_str(&text);
        }
        Event::KeyDown { keycode, repeat, .. } => {
            if !repeat {
                // Solo primera pulsación (ya implementado)
                handle_key(keycode);
            }
        }
        _ => {}
    }
}
```

---

## 📋 VERIFICACIONES PENDIENTES

| Verificación | Comando | Estado |
|-------------|---------|--------|
| Extensiones X11 (DRI3, PRESENT) | `xdpyinfo \| grep -i dri3` | ❌ xdpyinfo no instalado |
| GPU Vulkan device | `vulkaninfo \| grep "GPU\[0\]"` | ❌ No mostró GPU |
| SDL2 renderer actual | `SDL_RENDER_DRIVER=opengl SDL2 demo` | ⏳ Pendiente |
| Raylib renderer | `cargo run --bin demo_*` + logs | ⏳ Pendiente |
| Input Text eventos | Test con SDL_StartTextInput | ❌ No implementado |
| EGL vs GLX | `glxinfo` | ❌ glxinfo no instalado |

---

## 🚀 RECOMENDACIONES INMEDIATAS

### 1. Configurar vars de entorno en launcher_sdl2.sh
Agregar las variables detectadas como óptimas.

### 2. Agregar SDL_TEXTINPUT a migui backend_sdl2.rs
Para que los widgets textbox reciban texto real del teclado Android.

### 3. Agregar SDL_StartTextInput/StopTextInput
Cuando un textbox de migui/toolkit recibe foco, llamar SDL_StartTextInput().

### 4. Forzar SDL_VIDEODRIVER=x11 explícito
En ry-gfx al inicializar SDL2.

### 5. Instalar xdpyinfo para verificar DRI3
`pkg install xdpyinfo`

---

<div align="center">

**🔍 Pipeline Display/Input/Render — Análisis Completo**

*Termux-X11 + Zink + Vulkan + SDL2/Raylib + SurfaceFlinger*

*Input: solo primera pulsación | Text: NO implementado | GPU: Zink→Vulkan*

</div>
