# 📚 COMANDOS RYDIT v0.10.2

**Estado:** ✅ Estable - Inversión de Control + AST Caching
**Fecha:** 2026-03-30
**Binarios:** scene_runner, ecs_demo_10k, gpu_demo_100k

---

## 🎯 COMANDOS PRINCIPALES (v0.10.2)

### Ejecución de Demos

```bash
# Configurar entorno (CRÍTICO para Termux-X11)
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

# Verificar X11
xset q

# Usar script helper (RECOMENDADO)
./run_demo.sh demos/nivel_config.rydit

# O ejecutar directamente
./target/release/scene_runner demos/nivel_config.rydit
./target/release/ecs_demo_10k
./target/release/gpu_demo_100k
./target/release/demo_particles
```

---

## 📦 BINARIOS DISPONIBLES

| Binario | Tamaño | Uso | Estado |
|---------|--------|-----|--------|
| **scene_runner** | 326KB | Inversión de Control (v0.10.2) | ✅ |
| **ecs_demo_10k** | 272KB | ECS test (10K entidades) | ✅ |
| **gpu_demo_100k** | 276KB | GPU 100K partículas | ✅ |
| **demo_particles** | - | Demo partículas | ✅ |
| **snake** | - | Juego Snake | ✅ |
| **rydit-rs** | - | Legacy | ❌ 64 errores |

---

## 🎮 DEMOS DISPONIBLES

### Demos de Configuración (v0.10.2)

| Demo | Descripción | Binario |
|------|-------------|---------|
| `demos/nivel_config.rydit` | Configuración de nivel | scene_runner |
| `demos/nivel1.rydit` | Nivel 1 | scene_runner |
| `demos/nivel2.rydit` | Nivel 2 | scene_runner |

### Demos de Render Queue

| Demo | Descripción | Binario |
|------|-------------|---------|
| `demos/test_render_queue_integrada.rydit` | 2000 partículas | scene_runner |
| `demos/test_renderizado_v0.9.0.rydit` | Test completo | scene_runner |
| `demos/test_render_queue_simple.rydit` | Test simple | scene_runner |

### Demos de Físicas

| Demo | Descripción | Binario |
|------|-------------|---------|
| `demos/test_fisicas.rydit` | Test de físicas | scene_runner |
| `demos/platformer_v094.rydit` | Platformer demo | scene_runner |

### Demos de Input

| Demo | Descripción | Binario |
|------|-------------|---------|
| `demos/test_teclado.rydit` | Test de teclado | scene_runner |
| `demos/test_input_map.rydit` | Test de input map | scene_runner |

### Demos de Sistema

| Demo | Descripción | Binario |
|------|-------------|---------|
| `demos/test_minimo.rydit` | Test mínimo | scene_runner |
| `demos/diagnostico_simple.rydit` | Diagnóstico | scene_runner |
| `demos/test_v0.9.4_completo.rydit` | Test completo | scene_runner |

---

## 🚀 EJECUCIÓN RÁPIDA

### 1. Configurar Entorno

```bash
# Variables de entorno (CRÍTICO)
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1
export PULSE_SERVER=127.0.0.1

# Verificar X11
xset q  # Debe responder
```

### 2. Ejecutar Demo

```bash
# Opción A: Script helper (RECOMENDADO)
./run_demo.sh demos/nivel_config.rydit

# Opción B: Directo
./target/release/scene_runner demos/test_minimo.rydit

# Opción C: Demo específico
./target/release/ecs_demo_10k
./target/release/gpu_demo_100k
```

### 3. Script de Diagnóstico

```bash
# Verificar configuración completa
./scripts/test_x11.sh
```

---

## 📝 FORMATO .RYDIT v0.10.2

### Configuración de Nivel (Inversión de Control)

```rydit
# nivel_config.rydit - SOLO configuración, NO lógica

# Metadatos
@nombre "Nivel 1"
@autor "RyDit Team"

# Mundo
mundo {
    gravedad: 9.8
    fondo: "cielo.png"
    ancho: 800
    alto: 600
}

# Cámara
camara {
    seguir: "jugador"
    zoom: 1.0
}

# Entidades
entidad "jugador" {
    tipo: "player"
    sprite: "hero.png"
    x: 100
    y: 400
    script: "jugador_control.rydit"
}

entidad "enemigo" {
    tipo: "enemy"
    sprite: "enemy.png"
    x: 400
    y: 300
    ia: "patrol"
}

# Partículas (GPU Instancing)
particulas {
    habilitado: true
    max_particulas: 10000
}
```

---

## 🎨 DEMOS GRÁFICOS (.rydit con render)

### Círculos y Formas

```rydit
# demo_shapes.rydit
dibujar.circulo(400, 300, 50, "rojo")
dibujar.rect(100, 100, 200, 150, "azul")
dibujar.linea(0, 0, 800, 600, "verde")
texto "Hola RyDit" en 300, 300, tamano 20, color "blanco"
```

### Partículas (2000+)

```rydit
# test_render_queue_integrada.rydit
dark.slot particulas = []

# Crear 2000 partículas
ryda mientras i < 2000 {
    x = matematica::random(0, 800)
    y = matematica::random(0, 600)
    radio = matematica::random(10, 30)
    arrays::empujar(particulas, [x, y, radio, "rojo"])
}

# Game loop
ryda frame < 10000 {
    # Dibujar todas (1 begin_draw por frame)
    ryda mientras idx < arrays::longitud(particulas) {
        p = arrays::obtener(particulas, idx)
        dibujar.circulo(p[0], p[1], p[2], p[3])
    }
    
    texto "FPS: " + fps() en 10, 80, tamano 16, color "cyan"
}
```

---

## 🛠️ COMANDOS DE DESARROLLO

```bash
# Compilar release
cargo build --release

# Compilar debug
cargo build

# Ejecutar tests
cargo test

# Verificar código
cargo check

# Formatear
cargo fmt

# Linter
cargo clippy

# Limpiar
cargo clean
```

---

## 🔧 SOLUCIÓN DE PROBLEMAS

### Pantalla Negra

```bash
# 1. Verificar X11
xset q

# 2. Verificar variables
echo $DISPLAY
echo $MESA_LOADER_DRIVER_OVERRIDE
echo $DRI3

# 3. Ejecutar diagnóstico
./scripts/test_x11.sh

# 4. Verificar dependencias
pkg list --installed | grep -E "termux-x11|mesa-zink"
```

### Parpadeo (Flicker)

**Causa**: Múltiples `begin_draw()` por frame (v0.9.0)
**Solución**: ✅ Fix v0.9.1 (Render Queue)

```bash
# Actualizar a v0.9.1+
cargo build --release

# Usar scene_runner (v0.10.2)
./target/release/scene_runner demos/test_minimo.rydit
```

### Stuttering

**Causa**: Parser sobrecargado
**Solución**: ✅ Fix v0.10.2 (AST Caching - 10x speedup)

```bash
# Verificar versión
./target/release/scene_runner --version

# Debe mostrar: v0.10.2 o superior
```

### Error: "No se pudo conectar a X11"

```bash
# Iniciar Termux-X11
termux-x11 :0 -xstartup xfce4-session &

# Esperar 2-3 segundos
sleep 3

# Verificar
xset q
```

---

## 📊 RENDIMIENTO

### Binarios

| Binario | Partículas | FPS | Uso |
|---------|-----------|-----|-----|
| scene_runner | 2000+ | 60 | Render Queue |
| gpu_demo_100k | 100,000+ | 60 | GPU Instancing |
| ecs_demo_10k | 10,000 | 60 | ECS |

### Mejoras v0.10.2

| Feature | Antes | Después | Mejora |
|---------|-------|---------|--------|
| **Parser** | 2-4ms | 0.2-0.4ms | 10x |
| **Partículas** | 500 @ 15 FPS | 2000 @ 60 FPS | 4x |
| **Entidades** | Limitadas | 10,000+ | ECS |
| **Stability** | Inestable | Estable | Core manda |

---

## 📁 ESTRUCTURA DE ARCHIVOS

```
shield-project/
├── target/release/
│   ├── scene_runner      # ✅ Principal (v0.10.2)
│   ├── ecs_demo_10k      # ✅ ECS
│   ├── gpu_demo_100k     # ✅ GPU
│   └── demo_particles    # ✅ Partículas
├── demos/
│   ├── nivel_config.rydit    # Configuración
│   ├── test_minimo.rydit     # Test mínimo
│   └── test_render_queue_integrada.rydit  # 2000 partículas
├── ejemplos_gfx/
│   ├── demo_shapes.rydit     # Formas básicas
│   └── snake_v0.1.8.rydit    # Snake game
├── scripts/
│   └── test_x11.sh           # Diagnóstico
├── run_demo.sh               # Script helper
└── docs/
    └── COMANDOS_v0.10.2.md   # Este archivo
```

---

## 🎯 PRÓXIMAMENTE (v0.10.3)

- [ ] RyditModule trait completo
- [ ] Fix rydit-rs legacy (64 errores)
- [ ] DrawCommand::Texture para sprites
- [ ] Input Map acciones
- [ ] Camera transform

---

## 📞 SOPORTE

| Recurso | Ubicación |
|---------|-----------|
| **Documentación** | `docs/` |
| **Diagnóstico** | `scripts/test_x11.sh` |
| **Script helper** | `run_demo.sh` |
| **Bitácora** | `QWEN.md` |
| **README** | `README.md` |

---

**Última actualización:** 2026-03-30
**Versión:** v0.10.2 - Inversión de Control + AST Caching
**Estado:** ✅ Estable
