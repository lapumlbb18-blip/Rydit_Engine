# 🎨 RyDit - Animación 2D y Ilusiones Ópticas

**Versión**: v0.7.1.1
**Estado**: ✅ Funcional

---

## 📚 12 PRINCIPIOS DE ANIMACIÓN (Disney)

RyDit implementa **3 principios fundamentales** con funciones listas para usar:

### **Principio #1: Squash & Stretch**

Objetos se deforman al moverse (manteniendo volumen).

```rydit
# Squash: aplastar (ancho aumenta, alto disminuye)
dark.slot escala = anim::squash(2.0)
# Retorna: [2.0, 0.5] - escala X y Y

# Stretch: estirar (alto aumenta, ancho disminuye)
dark.slot escala = anim::stretch(2.0)
# Retorna: [0.5, 2.0] - escala X y Y

# Ejemplo: Pelota que rebota
dark.slot rebote = 0
ryda frame < 500 {
    draw.rect(400, 300, 50, 50, "rojo")
    
    # En el impacto: squash
    onif rebote > 0 && rebote < 10 {
        dark.slot s = anim::squash(1.5)
        # Usar s[0] para ancho, s[1] para alto
    }
    
    dark.slot rebote = rebote + 1
}
```

---

### **Principio #2: Anticipation**

Movimiento contrario antes de la acción principal.

```rydit
# Anticipar salto: retrocede antes de avanzar
dark.slot pos = anim::anticipate(100, 200, 20)
# Retorna: 80 (retrocede 20 antes de ir a 200)

# Ejemplo: Personaje que salta
dark.slot x = 100
dark.slot saltando = false

ryda frame < 1000 {
    onif tecla_presionada(" ") && not saltando {
        dark.slot x = anim::anticipate(x, x + 100, 15)
        dark.slot saltando = true
    }
    
    draw.circle(x, 300, 20, "azul")
}
```

---

### **Principio #6: Slow In & Slow Out (Easing)**

Movimiento suave que acelera y frena gradualmente.

```rydit
# Ease In: comienza lento, acelera al final
dark.slot t = 0.5
dark.slot ease = anim::ease_in(t)
# Retorna: 0.25 (más lento al inicio)

# Ease Out: comienza rápido, frena al final
dark.slot ease = anim::ease_out(t)
# Retorna: 0.75 (más rápido al inicio)

# Ease In-Out: combina ambos
dark.slot ease = anim::ease_in_out(t)
# Retorna: 0.5 (suave en ambos extremos)

# Ejemplo: Movimiento suave de izquierda a derecha
dark.slot t = 0
ryda frame < 200 {
    dark.slot t = t + 0.005
    onif t > 1 { dark.slot t = 0 }
    
    dark.slot ease = anim::ease_in_out(t)
    dark.slot x = 100 + ease * 600
    
    draw.circle(x, 300, 30, "verde")
}
```

---

## 🔮 ILUSIONES ÓPTICAS

### **1. Müller-Lyer (Flechas)**

Dos líneas iguales que parecen diferentes.

```rydit
shield.init

ryda frame < 1000 {
    draw.rect(0, 0, 800, 600, "negro")
    
    # Línea 1: Flechas hacia adentro (parece MÁS larga)
    draw.line(200, 150, 600, 150, "blanco")
    draw.line(200, 150, 230, 130, "rojo")
    draw.line(200, 150, 230, 170, "rojo")
    draw.line(600, 150, 570, 130, "rojo")
    draw.line(600, 150, 570, 170, "rojo")
    
    # Línea 2: Flechas hacia afuera (parece MÁS corta)
    draw.line(200, 300, 600, 300, "blanco")
    draw.line(200, 300, 170, 280, "verde")
    draw.line(200, 300, 170, 320, "verde")
    draw.line(600, 300, 630, 280, "verde")
    draw.line(600, 300, 630, 320, "verde")
    
    draw.text("¡Ambas miden 400px!", 280, 450, "amarillo")
}
```

---

### **2. Ponzo (Perspectiva)**

Líneas paralelas que parecen converger.

```rydit
shield.init

ryda frame < 1000 {
    draw.rect(0, 0, 800, 600, "negro")
    
    # Líneas de perspectiva (vías de tren)
    draw.line(200, 100, 300, 500, "gris")
    draw.line(600, 100, 500, 500, "gris")
    
    # Barra superior (parece MÁS pequeña)
    draw.line(280, 200, 520, 200, "rojo")
    
    # Barra inferior (parece MÁS grande)
    draw.line(330, 400, 470, 400, "verde")
    
    draw.text("¡Ambas miden 240px!", 280, 500, "amarillo")
}
```

---

### **3. Phi Effect (Movimiento Aparente)**

Base del cine y la animación.

```rydit
shield.init

dark.slot frame = 0

ryda frame < 1000 {
    draw.rect(0, 0, 800, 600, "negro")
    
    # Trayectoria
    draw.line(100, 300, 700, 300, "gris")
    
    # Círculo en movimiento
    dark.slot x = 100 + frame * 2
    onif x > 700 { dark.slot x = 100 }
    
    draw.circle(x, 300, 30, "rojo")
    draw.circle(x, 300, 15, "blanco")
    
    draw.text("Movimiento aparente - Cine", 260, 450, "amarillo")
}
```

---

### **4. Fraser Spiral (Falsa Espiral)**

Círculos concéntricos que parecen espiral.

```rydit
shield.init

ryda frame < 1000 {
    draw.rect(0, 0, 800, 600, "negro")
    
    # Círculos concéntricos
    draw.circle(400, 300, 30, "blanco")
    draw.circle(400, 300, 50, "blanco")
    draw.circle(400, 300, 70, "blanco")
    draw.circle(400, 300, 90, "blanco")
    draw.circle(400, 300, 110, "blanco")
    
    # Líneas diagonales (crean ilusión de espiral)
    draw.line(250, 200, 550, 400, "negro")
    draw.line(250, 400, 550, 200, "negro")
    
    draw.text("¡Son círculos, NO espiral!", 260, 500, "amarillo")
}
```

---

## 🎮 DEMOS DISPONIBLES

| Demo | Descripción | Comando |
|------|-------------|---------|
| `demo_ilusiones_minimo.rydit` | 4 ilusiones interactivas | `cargo run --bin rydit-rs -- --gfx demos/demo_ilusiones_minimo.rydit` |
| `demo_ilusiones_simple.rydit` | Versión simplificada | `cargo run --bin rydit-rs -- --gfx demos/demo_ilusiones_simple.rydit` |

---

## 📊 FUNCIONES DISPONIBLES

### **Easing (Suavizado)**

| Función | Parámetros | Retorna | Descripción |
|---------|------------|---------|-------------|
| `anim::ease_in(t)` | t: 0.0-1.0 | número | Comienza lento, acelera |
| `anim::ease_out(t)` | t: 0.0-1.0 | número | Comienza rápido, frena |
| `anim::ease_in_out(t)` | t: 0.0-1.0 | número | Combina ambos |

### **Squash & Stretch (Deformación)**

| Función | Parámetros | Retorna | Descripción |
|---------|------------|---------|-------------|
| `anim::squash(factor)` | factor: 0.5-2.0 | [x, y] | Aplasta (x↑, y↓) |
| `anim::stretch(factor)` | factor: 0.5-2.0 | [x, y] | Estira (x↓, y↑) |

### **Anticipation (Anticipación)**

| Función | Parámetros | Retorna | Descripción |
|---------|------------|---------|-------------|
| `anim::anticipate(pos, target, amount)` | 3 números | número | Retrocede antes de avanzar |

### **Ilusiones Ópticas**

| Función | Parámetros | Retorna | Descripción |
|---------|------------|---------|-------------|
| `illusion::muller_lyer(x, y, len, arrow)` | 4 valores | [x, y, len, arrow] | Datos para dibujar |
| `illusion::ponzo(x, y, w, h)` | 4 números | [x, y, w, h] | Datos para dibujar |
| `illusion::phi_effect(x1, y1, x2, y2, speed, frame)` | 6 números | [x, y, dir] | Posición actual |
| `illusion::fraser_spiral(cx, cy, min, max, step)` | 5 números | [cx, cy, circles] | Círculos concéntricos |

---

## 🧪 TESTS

```bash
# Ejecutar tests de animación
cargo test -p rydit-rs anim

# Resultado esperado:
# running 8 tests
# test_anim_ease_in ... ok
# test_anim_ease_out ... ok
# test_anim_ease_in_out ... ok
# test_anim_squash ... ok
# test_anim_stretch ... ok
# test_anim_anticipate ... ok
# test_illusion_muller_lyer ... ok
# test_illusion_phi_effect ... ok
```

---

<div align="center">

**🛡️ RyDit v0.7.1.1 - Animación 2D + Ilusiones Ópticas**

*147 tests passing | 10 funciones de animación | 4 ilusiones*

</div>
