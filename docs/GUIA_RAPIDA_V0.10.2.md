# 📘 Guía Rápida RyDit v0.10.2

**Tiempo de lectura:** 3 minutos  
**Estado:** ✅ Estable - Inversión de Control + AST Caching

---

## 🚀 Empezando en 1 Minuto

### 1. Configurar Entorno (CRÍTICO)

```bash
# Variables de entorno para Termux-X11
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

# Verificar X11
xset q  # Debe responder
```

### 2. Ejecutar Demo

```bash
# Opción A: Script helper (RECOMENDADO)
./run_demo.sh demos/nivel_config.rydit

# Opción B: Directo
./target/release/scene_runner demos/test_minimo.rydit

# Opción C: Demo GPU
./target/release/gpu_demo_100k
```

### 3. Verificar Diagnóstico

```bash
# Test completo de configuración
./scripts/test_x11.sh
```

---

## 📦 Binarios Disponibles

| Binario | Uso | Tamaño |
|---------|-----|--------|
| `scene_runner` | Demos normales | 326KB |
| `ecs_demo_10k` | ECS test | 272KB |
| `gpu_demo_100k` | GPU 100K partículas | 276KB |

---

## 🎮 Demos Principales

### Test Mínimo (Recomendado para empezar)

```bash
./target/release/scene_runner demos/test_minimo.rydit
```

### Configuración de Nivel (v0.10.2)

```bash
./target/release/scene_runner demos/nivel_config.rydit
```

### 2000 Partículas (Render Queue)

```bash
./target/release/scene_runner demos/test_render_queue_integrada.rydit
```

### ECS 10K Entidades

```bash
./target/release/ecs_demo_10k
```

### GPU 100K Partículas

```bash
./target/release/gpu_demo_100k
```

---

## 📝 Formato .rydit v0.10.2

### Configuración (Inversión de Control)

```rydit
# nivel_config.rydit - SOLO configuración

@nombre "Nivel 1"

mundo {
    gravedad: 9.8
    ancho: 800
    alto: 600
}

entidad "jugador" {
    sprite: "hero.png"
    x: 100
    y: 400
    script: "jugador_control.rydit"
}

entidad "enemigo" {
    sprite: "enemy.png"
    x: 400
    y: 300
    ia: "patrol"
}
```

### Renderizado (Game Loop)

```rydit
# test_render_queue.rydit

dark.slot frame = 0

ryda frame < 10000 {
    # Dibujar círculos
    dibujar.circulo(400, 300, 50, "rojo")
    dibujar.circulo(200, 200, 30, "azul")
    
    # Mostrar FPS
    texto "FPS: " + fps() en 10, 80, tamano 16, color "cyan"
    
    # Salir con ESC
    onif tecla_presionada("escape") {
        romper
    }
    
    frame = frame + 1
}
```

---

## 🛠️ Comandos de Desarrollo

```bash
# Compilar
cargo build --release

# Tests
cargo test

# Linter
cargo clippy

# Formatear
cargo fmt
```

---

## 🔧 Solución de Problemas

### Pantalla Negra

```bash
# 1. Verificar X11
xset q

# 2. Verificar variables
echo $DISPLAY              # Debe ser :0
echo $MESA_LOADER_DRIVER_OVERRIDE  # Debe ser zink
echo $DRI3                 # Debe ser 1

# 3. Ejecutar diagnóstico
./scripts/test_x11.sh
```

### Parpadeo

✅ **Fix v0.9.1**: Render Queue integrada

```bash
# Usar scene_runner (v0.10.2)
./target/release/scene_runner demos/test_minimo.rydit
```

### Stuttering

✅ **Fix v0.10.2**: AST Caching (10x speedup)

---

## 📁 Estructura

```
shield-project/
├── target/release/
│   ├── scene_runner
│   ├── ecs_demo_10k
│   └── gpu_demo_100k
├── demos/
│   ├── test_minimo.rydit
│   └── nivel_config.rydit
├── scripts/
│   └── test_x11.sh
└── run_demo.sh
```

---

## 🎯 Próximo: v0.10.3

- RyditModule trait
- DrawCommand::Texture
- Input Map acciones

---

**Más info:** `docs/COMANDOS_v0.10.2.md` | `QWEN.md`
