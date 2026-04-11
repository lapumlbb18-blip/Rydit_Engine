# 🏗️ VISIÓN ARQUITECTÓNICA: Patrón Fusional SDL2 + Raylib

**Fecha**: 2026-04-11
**Tipo**: Propuesta arquitectónica — No es deuda técnica, es oportunidad

---

## 💡 La Idea Central

**No es que SDL2 y Raylib no puedan coexistir.**
**Es que los estamos compilando/linkando mal.**

```
PROBLEMA ACTUAL (compilación en 1 paso):
cargo build → linka SDL2 + raylib completa (con GLFW) → CONFLICTO

SOLUCIÓN (compilación en 2 pasos):
Paso 1: Compilar raylib SIN GLFW → libraylib_sdl2.so
Paso 2: Linkar binario contra SDL2 + libraylib_sdl2.so → FUNCIONA
```

---

## 🧩 Analogía Correcta del Usuario

| Componente | Motor Grande | Ry-Dit | Función |
|-----------|-------------|--------|---------|
| Gestión ventanas/input | winit | **SDL2** | Crea ventana, lee teclado/mouse/touch |
| Dibujo 2D/3D | macroquad/glium | **Raylib (rlgl)** | Cubos, esferas, partículas, modelos |
| Orquestación | Godot .gdscript | **.rydit (RyDitModule)** | Organiza escenas, assets, scripts |

**Rust no depende de ninguno.** Es el orquestador que decide cuál usar y cuándo.

---

## 📐 Estructura de Proyecto Propuesta (.rydit)

```
mi_juego.rydit/                    # Archivo de proyecto (como .godot)
├── project.rydit                  # Configuración principal
│   ├── nombre = "Mi Juego"
│   ├── version = "0.1.0"
│   ├── backend = "sdl2+raylib"    # ← El usuario elige
│   ├── ventana_ancho = 800
│   ├── ventana_alto = 600
│   └── fps_target = 60
│
├── assets/                        # Recursos (texturas, modelos, audio)
│   ├── sprites/
│   ├── models/
│   ├── audio/
│   └── fonts/
│
├── scenes/                        # Escenas (como .tscn de Godot)
│   ├── menu.ryscene
│   ├── nivel1.ryscene
│   └── boss.ryscene
│
├── scripts/                       # Lógica del juego
│   ├── jugador.rs
│   ├── enemigo.rs
│   └── ui.rs
│
├── animations/                    # Animaciones (como ry-anim)
│   ├── jugador.anim
│   └── enemigo.anim
│
├── input/                         # Mapeo de controles
│   ├── keyboard.rydit-input
│   ├── touch.rydit-input          # ← Mandos en pantalla
│   └── gamepad.rydit-input
│
└── build/                         # Output compilado
    └── mi_juego                   # Binario autocontenido
```

---

## 🔄 Flujo de Compilación en 2 Pasos

### Paso 1: Compilar Raylib sin GLFW
```bash
# Descargar raylib source
git clone https://github.com/raysan5/raylib.git
cd raylib/src

# Compilar con SDL2 como backend (sin GLFW)
make PLATFORM=DESKTOP \
     RAYLIB_LIBTYPE=SHARED \
     USE_EXTERNAL_GLFW=TRUE \
     CUSTOM_CFLAGS="-I/usr/include/SDL2" \
     CUSTOM_LDFLAGS="-lSDL2"

# Resultado: libraylib.so (sin GLFW, usa SDL2 internamente)
cp libraylib.so /usr/lib/
```

### Paso 2: Compilar Ry-Dit contra esa librería
```bash
# Cargo.toml apunta a la librería local
[dependencies]
raylib-sys = { path = "../raylib-bindings-sdl2" }  # Bindings modificados
sdl2 = "0.37"

# Build normal
cargo build --release

# Resultado: binario que usa SDL2 para todo + raylib para dibujo
```

---

## 🎯 Patrón de Funcionamiento Fusional

```
┌─────────────────────────────────────────────────┐
│                  Ry-Dit Engine                   │
├─────────────────────────────────────────────────┤
│                                                 │
│  ┌─────────────┐    ┌──────────────────────┐   │
│  │   SDL2      │    │   Raylib (rlgl)      │   │
│  │             │    │                      │   │
│  │ ✅ Ventanas  │    │ ✅ Dibujo 2D         │   │
│  │ ✅ Input     │    │ ✅ Dibujo 3D         │   │
│  │ ✅ Eventos   │    │ ✅ Partículas        │   │
│  │ ✅ Audio     │    │ ✅ Modelos GLTF/OBJ  │   │
│  │ ✅ Touch     │    │ ✅ Texturas          │   │
│  │ ✅ Mandos    │    │ ✅ Texto bonito      │   │
│  │   pantalla   │    │ ✅ Post-processing   │   │
│  └──────┬──────┘    └──────────┬───────────┘   │
│         │                      │               │
│         └──────────┬───────────┘               │
│                    │                           │
│            ┌───────▼───────┐                   │
│            │  RyDitModule  │                   │
│            │    (trait)    │                   │
│            └───────┬───────┘                   │
│                    │                           │
│            ┌───────▼───────┐                   │
│            │   .rydit      │                   │
│            │   (proyecto)  │                   │
│            └───────────────┘                   │
│                                                 │
└─────────────────────────────────────────────────┘
```

---

## 📋 RyDitModule Trait — Debe Ser Genérico

**Actual** (solo SDL2):
```rust
pub trait RyDitModule {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn register(&self) -> HashMap<&'static str, &'static str>;
    fn execute(&self, command: &str, params: Value) -> ModuleResult;
}
```

**Propuesto** (genérico):
```rust
pub trait RyDitModule {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    
    // Backend agnóstico
    fn backend_type(&self) -> BackendType;  // SDL2, Raylib, Fusion
    
    fn register(&self) -> HashMap<&'static str, &'static str>;
    fn execute(&self, command: &str, params: Value) -> ModuleResult;
    
    // Renderizado abstracto
    fn render_2d(&self, renderer: &mut dyn Renderer2D);
    fn render_3d(&self, renderer: &mut dyn Renderer3D);
}

pub enum BackendType {
    SDL2,        // Solo SDL2 (demos actuales)
    Raylib,      // Solo Raylib (standalone 3D)
    Fusion,      // SDL2 + Raylib (ideal)
}
```

---

## 🎮 Detalle de Termux + Surfaceflinger

| Problema | Causa | Solución |
|----------|-------|----------|
| GLFW no maneja input en Termux | Surfaceflinger no expone eventos de teclado/mouse a GLFW | Usar SDL2 que SÍ soporta Termux |
| Raylib con GLFW = sin input | GLFW en Termux no recibe eventos | `USE_EXTERNAL_GLFW=TRUE` → usa SDL2 |
| Touch/mandos en pantalla | GLFW no tiene API para touch virtual | SDL2 tiene `SDL_Finger` + touch events |
| Zoom con mouse en 3D | GLFW funciona pero input falla | SDL2 maneja mouse wheel correctamente |

**Conclusión**: El "problema de raylib en Termux" **NO es de raylib**, es de GLFW. Si reemplazamos GLFW con SDL2, raylib funciona perfecto.

---

## 🔧 Plan de Experimentación

### Fase 1: Validar que raylib puede compilarse con SDL2 (4-6h)
- [ ] Clonar raylib source
- [ ] Probar `make USE_EXTERNAL_GLFW=TRUE`
- [ ] Verificar que `libraylib.so` se genera sin GLFW
- [ ] Test: un programa C simple que use SDL2 + raylib para dibujar un cubo

### Fase 2: Bindings Rust para raylib-sdl2 (4-6h)
- [ ] Crear crate `raylib-sys-sdl2` (bindings modificados)
- [ ] Apuntar a `libraylib.so` compilada en Fase 1
- [ ] Test: `demo_raylib_sdl2` que dibuje un cubo con input SDL2

### Fase 3: RyDitModule genérico (6-8h)
- [ ] Refactorizar `RyDitModule` trait para ser backend-agnóstico
- [ ] Implementar `Renderer2D` y `Renderer3D` traits
- [ ] SDL2 implementa `Renderer2D`, Raylib implementa `Renderer3D`

### Fase 4: Demo fusional (8-12h)
- [ ] `demo_fusion`: SDL2 ventana + Raylib dibuja 3D + input SDL2
- [ ] Texto con calidad raylib (stb_truetype)
- [ ] Partículas 3D con raylib
- [ ] Mandos en pantalla con SDL2 touch

---

## ⚠️ Riesgos

| Riesgo | Probabilidad | Impacto | Mitigación |
|--------|-------------|---------|------------|
| `USE_EXTERNAL_GLFW=TRUE` no funciona en Android | Media | Alto | Probar primero en desktop Linux |
| Bindings Rust incompatibles | Baja | Medio | Regenerar con bindgen |
| SDL2 + raylib conflicto de símbolos | Baja | Alto | Namespaces separados |
| Binario muy grande | Media | Bajo | Feature toggle, strip |
| Mantenimiento de 2 backends | Media | Medio | Tests CI en ambos |

---

## 🎯 Conclusión

**Tu idea es correcta:**
1. ✅ El problema es de flujo de compilación, no de capacidad técnica
2. ✅ SDL2 maneja ventanas+input, raylib maneja dibujo → separación limpia
3. ✅ `USE_EXTERNAL_GLFW=TRUE` es la clave para eliminar GLFW
4. ✅ .rydit como formato de proyecto organiza todo (assets, scenes, scripts, input)
5. ✅ RyDitModule trait debe ser genérico, no atado a SDL2
6. ✅ El detalle de Termux se resuelve: SDL2 funciona ahí, GLFW no

**¿Por dónde empezamos?**
- **Opción A**: Fase 1 → validar compilación de raylib con SDL2 (más arriesgado pero resuelve todo)
- **Opción B**: Fase 3 → hacer RyDitModule genérico primero (menos riesgo, mejora arquitectura actual)

---

<div align="center">

**🏗️ Patrón Fusional SDL2+Raylib — Propuesta Arquitectónica**

*Problema: No es técnico, es de flujo de compilación*

*Solución: 2 pasos + USE_EXTERNAL_GLFW=TRUE + RyDitModule genérico*

*Formato .rydit: assets + scenes + scripts + input organizados*

</div>
