# 📚 ÍNDICE DE ARCHIVOS .RYDIT

**Total**: 147 archivos `.rydit`  
**Última actualización**: 2026-04-02  
**Versión**: v0.11.5  

---

## 🎯 ARCHIVOS PRINCIPALES (Top 20)

| # | Archivo | Propósito | Categoría | Estado |
|---|---------|-----------|-----------|--------|
| 1 | `demos/snake.rydit` | Snake Game clásico | Juego | ✅ Funcional |
| 2 | `demos/demo_termux_x11.rydit` | Test completo Termux-X11 | Test | ✅ Referencia |
| 3 | `demos/demo_particulas.rydit` | Sistema de partículas | Demo | ✅ Funcional |
| 4 | `demos/platformer_demo.rydit` | Platformer clásico | Juego | ✅ Funcional |
| 5 | `demos/demo_shapes.rydit` | Formas geométricas | Demo | ✅ Funcional |
| 6 | `demos/nivel1.rydit` | Test nivel 1 | Test | ✅ Funcional |
| 7 | `demos/nivel2.rydit` | Test nivel 2 | Test | ✅ Funcional |
| 8 | `demos/diagnostico_simple.rydit` | Diagnóstico motor | Test | ✅ Funcional |
| 9 | `demos/demo_migui.rydit` | UI widgets migui | UI | ✅ Funcional |
| 10 | `demos/tank_combat.rydit` | Tank combat demo | Juego | ✅ Funcional |
| 11 | `demos/demo_bezier.rydit` | Curvas de Bezier | Demo científica | ✅ Funcional |
| 12 | `demos/demo_fisica.rydit` | Simulación física | Demo científica | ✅ Funcional |
| 13 | `demos/demo_datos.rydit` | Visualización datos | Demo científica | ✅ Funcional |
| 14 | `demos/demo_strings.rydit` | Manipulación strings | Stdlib | ✅ Funcional |
| 15 | `demos/demo_math.rydit` | Funciones matemáticas | Stdlib | ✅ Funcional |
| 16 | `demos/demo_arrays.rydit` | Operaciones arrays | Stdlib | ✅ Funcional |
| 17 | `demos/demo_json.rydit` | Parsing JSON | Stdlib | ✅ Funcional |
| 18 | `demos/demo_regex.rydit` | Expresiones regulares | Stdlib | ✅ Funcional |
| 19 | `demos/demo_files.rydit` | Manejo archivos | Stdlib | ✅ Funcional |
| 20 | `demos/demo_time.rydit` | Funciones tiempo | Stdlib | ✅ Funcional |

---

## 📦 MÓDULOS STDLIB (10 módulos)

### Ubicación: `crates/modules/`

| Módulo | Funciones | Líneas | Tests |
|--------|-----------|--------|-------|
| `math.rydit` | `sumar`, `restar`, `multiplicar`, `dividir`, `potencia`, `raiz` | ~50 | ✅ |
| `arrays.rydit` | `push`, `pop`, `insertar`, `eliminar`, `length`, `filtrar` | ~80 | ✅ |
| `strings.rydit` | `upper`, `lower`, `concat`, `length`, `substring` | ~60 | ✅ |
| `random.rydit` | `int`, `float`, `range`, `choice` | ~30 | ✅ |
| `time.rydit` | `now`, `sleep`, `elapsed` | ~20 | ✅ |
| `io.rydit` | `print`, `input`, `read`, `write` | ~40 | ✅ |
| `files.rydit` | `open`, `read`, `write`, `exists` | ~50 | ✅ |
| `json.rydit` | `parse`, `stringify`, `get`, `set` | ~60 | ✅ |
| `colisiones.rydit` | `rect_rect`, `circle_circle`, `point_rect` | ~40 | ✅ |
| `regex.rydit` | `match`, `search`, `replace`, `split` | ~50 | ✅ |

**Total líneas stdlib**: ~480 líneas

---

## 🎮 JUEGOS (5 juegos)

| Juego | Archivo | Características | Complejidad |
|-------|---------|-----------------|-------------|
| **Snake** | `demos/snake.rydit` | Cuerpo, comida, puntuación, game over | ⭐⭐⭐ |
| **Platformer** | `demos/platformer_demo.rydit` | Gravedad, salto, plataformas | ⭐⭐⭐⭐ |
| **Tank Combat** | `demos/tank_combat.rydit` | Torreta rotante, balas, colisiones | ⭐⭐⭐⭐⭐ |
| **Pong** | `demos/demo_pong.rydit` | 2 jugadores, pelota, puntuación | ⭐⭐ |
| **Breakout** | `demos/demo_breakout.rydit` | Paleta, bloques, pelota | ⭐⭐⭐ |

---

## 🎨 DEMOS VISUALES (20+ demos)

### Formas y Colores

| Demo | Archivo | Elementos |
|------|---------|-----------|
| Formas básicas | `demos/demo_shapes.rydit` | Círculos, rects, líneas, texto |
| Círculos animados | `demos/demo_circulos.rydit` | 50+ círculos en movimiento |
| Líneas dinámicas | `demos/demo_lineas.rydit` | Líneas que siguen al mouse |
| Texto animado | `demos/demo_texto.rydit` | Texto con efectos |

### Partículas

| Demo | Archivo | Efecto | Partículas |
|------|---------|--------|-----------|
| Fuego | `demos/particulas_fuego.rydit` | Efecto fuego | ~200 |
| Humo | `demos/particulas_humo.rydit` | Efecto humo | ~150 |
| Explosión | `demos/particulas_explosion.rydit` | Explosión | ~500 |
| Lluvia | `demos/particulas_lluvia.rydit` | Lluvia | ~300 |
| Chispas | `demos/particulas_chispas.rydit` | Chispas | ~100 |

### Sprites

| Demo | Archivo | Sprites |
|------|---------|---------|
| Carga sprite | `demos/demo_assets.rydit` | Tanque + helicóptero |
| Animación sprite | `demos/demo_animacion.rydit` | Personaje caminando |

---

## 🔬 DEMOS CIENTÍFICAS (15+ demos)

### Matemáticas

| Demo | Archivo | Concepto |
|------|---------|----------|
| Curvas Bezier | `demos/demo_bezier.rydit` | Curvas paramétricas |
| Fractales | `demos/demo_fractal.rydit` | Mandelbrot, Julia |
| Espirales | `demos/demo_espiral.rydit` | Espiral áurea |
| Funciones | `demos/demo_funciones.rydit` | Graficador de funciones |

### Física

| Demo | Archivo | Simulación |
|------|---------|------------|
| Proyectiles | `demos/demo_proyectil.rydit` | Trayectoria parabólica |
| Gravedad | `demos/demo_gravedad.rydit` | Caída libre |
| Péndulo | `demos/demo_pendulo.rydit` | Péndulo simple |
| Ondas | `demos/demo_ondas.rydit` | Ondas senoidales |
| N-Body | `demos/demo_nbody.rydit` | Gravedad N cuerpos |

### Visualización de Datos

| Demo | Archivo | Tipo |
|------|---------|------|
| CSV reader | `demos/demo_csv.rydit` | Lectura + tablas |
| Gráficos ASCII | `demos/demo_ascii.rydit` | Gráficos en texto |
| Estadísticas | `demos/demo_stats.rydit` | Mean, median, mode |

---

## 🧪 TESTS (10+ scripts de test)

| Test | Archivo | Propósito |
|------|---------|-----------|
| Nivel 1 | `demos/nivel1.rydit` | Test lexer + parser |
| Nivel 2 | `demos/nivel2.rydit` | Test evaluator + VM |
| Nivel 3 | `demos/nivel3.rydit` | Test gráficos SDL2 |
| Diagnóstico | `demos/diagnostico_simple.rydit` | Test motor completo |
| Input | `demos/test_input.rydit` | Test teclado |
| Audio | `demos/test_audio.rydit` | Test audio SDL2 |
| Assets | `demos/test_assets.rydit` | Test carga sprites |

---

## 🛠️ HERRAMIENTAS (5+ utilidades)

| Herramienta | Archivo | Función |
|-------------|---------|---------|
| Editor escenas | `demos/editor_escenas.rydit` | Editor visual de escenas |
| RyBot CLI | `rybot_cli.rs` | Asistente de código |
| REPL | Integrado en `main.rs` | Línea interactiva |
| Calculadora | `demos/demo_calc.rydit` | Calculadora matemática |
| Conversor | `demos/demo_conversor.rydit` | Conversor unidades |

---

## 📊 DISTRIBUCIÓN POR CATEGORÍA

| Categoría | Cantidad | % del Total |
|-----------|----------|-------------|
| **Módulos stdlib** | 10 | 7% |
| **Juegos** | 5 | 3% |
| **Demos visuales** | 20 | 14% |
| **Demos científicas** | 15 | 10% |
| **Tests** | 10 | 7% |
| **Herramientas** | 5 | 3% |
| **Backups históricos** | 75 | 51% |
| **Otros** | 7 | 5% |
| **TOTAL** | **147** | **100%** |

---

## 🏆 TOP 5 MÁS COMPLEJOS

| # | Archivo | Líneas | Complejidad | Por qué |
|---|---------|--------|-------------|---------|
| 1 | `demos/tank_combat.rydit` | ~200 | ⭐⭐⭐⭐⭐ | Torreta + balas + colisiones |
| 2 | `demos/snake.rydit` | ~150 | ⭐⭐⭐⭐ | Cuerpo + comida + puntuación |
| 3 | `demos/platformer_demo.rydit` | ~140 | ⭐⭐⭐⭐ | Gravedad + plataformas |
| 4 | `demos/demo_nbody.rydit` | ~120 | ⭐⭐⭐⭐ | Física gravitacional |
| 5 | `demos/demo_bezier.rydit` | ~100 | ⭐⭐⭐ | Curvas paramétricas |

---

## 📝 SINTAXIS DE REFERENCIA RÁPIDA

### Estructura Básica

```rydit
# 1. Inicialización
shield.init

# 2. Variables
dark.slot x = 400
dark.slot y = 300

# 3. Game loop
ryda frame < 10000 {
    dark.slot frame = frame + 1
    
    # Input
    onif tecla_presionada("arrow_left") {
        dark.slot x = x - 5
    }
    
    # Render
    draw.rect(0, 0, 800, 600, "negro")
    draw.circle(x, y, 50, "rojo")
    
    # Salir
    onif tecla_presionada("escape") {
        romper
    }
}
```

### Funciones

```rydit
rytmo mi_funcion(param1, param2) {
    voz param1 + param2
    return 1
}

dark.slot resultado = mi_funcion(10, 20)
```

### Condicionales

```rydit
onif x > 5 {
    voz "Mayor"
} blelse {
    voz "Menor"
}
```

### Bucles

```rydit
# Bucle while
ryda x > 0 {
    dark.slot x = x - 1
}

# Bucle for-style
dark.slot i = 0
ryda i < 10 {
    dark.slot i = i + 1
}
```

### Módulos

```rydit
import math
import arrays
import random

dark.slot suma = math::sumar(10, 20)
dark.slot num = random::int(1, 100)
```

---

## 🚀 CÓMO USAR

### Ejecutar Script .rydit

```bash
# Con cargo run
cargo run -p rydit-rs --bin rydit-rs -- --gfx demos/snake.rydit

# Con binario directo (release)
./target/release/rydit-rs --gfx demos/snake.rydit

# Con binario directo (debug)
./target/debug/rydit-rs --gfx demos/snake.rydit
```

### Crear Nuevo Script

1. Crear archivo en `demos/mi_demo.rydit`
2. Usar templates de `GUIA_BINARIOS_GRAFICOS_TERMUX_X11.md`
3. Probar con `cargo run`
4. Agregar a documentación

---

## 📚 RECURSOS RELACIONADOS

| Recurso | Ubicación | Contenido |
|---------|-----------|-----------|
| **Guía Binarios** | `GUIA_BINARIOS_GRÁFICOS_TERMUX_X11.md` | Crear binarios desde cero |
| **Resumen Tareas** | `RESUMEN_TAREAS_V0.11.6.md` | Plan de ejecución v0.11.6 |
| **README** | `README.md` | Documentación principal |
| **Roadmap** | `ROADMAP.md` | Plan de desarrollo |
| **QWEN** | `QWEN.md` | Bitácora técnica |
| **Estado** | `ESTADO_V0.11.5.md` | Estado actual |

---

## ⚠️ NOTAS IMPORTANTES

1. **Backups**: 75 archivos están en `.drive_backup/` (históricos)
2. **Módulos**: 10 módulos stdlib en `crates/modules/`
3. **Demos activas**: ~72 scripts en uso activo
4. **Parser**: Usa `rydit-parser` con lifetimes `'a` (NO `lizer`)
5. **Regla de oro**: NUNCA usar `sed` después de refactorizar parser

---

<div align="center">

**🛡️ RyDit v0.11.5 - ÍNDICE .RYDIT**

*147 archivos | 10 módulos stdlib | 5 juegos | 20+ demos*

**Última actualización**: 2026-04-02

</div>
