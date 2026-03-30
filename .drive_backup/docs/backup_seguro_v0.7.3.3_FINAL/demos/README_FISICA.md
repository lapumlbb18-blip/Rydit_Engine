# ⚛️ Módulo de Física 2D - RyDit v0.7.1.2

## Descripción

El módulo de física proporciona funciones para simulación de movimiento de proyectiles, gravedad n-body, ondas y péndulos.

## Funciones Disponibles

### Movimiento de Proyectiles

#### `physics::projectile(x0, y0, v0, angle)`

Calcula la trayectoria completa de un proyectil.

**Parámetros:**
- `x0` (número): Posición inicial X (metros)
- `y0` (número): Posición inicial Y (metros)
- `v0` (número): Velocidad inicial (m/s)
- `angle` (número): Ángulo en grados

**Retorna:**
- Array `[x_final, y_final, tiempo_vuelo, altura_maxima, alcance]`

**Fórmulas:**
```
vx = v0 * cos(θ)
vy = v0 * sin(θ)
t_vuelo = 2 * vy / g
h_max = vy² / (2 * g)
alcance = vx * t_vuelo
```

**Ejemplo:**
```rydit
# Lanzar desde origen a 50 m/s, 45°
dark.slot resultado = physics::projectile(0, 0, 50, 45)
print("Tiempo de vuelo: " + resultado[2] + " s")
print("Altura máxima: " + resultado[3] + " m")
print("Alcance: " + resultado[4] + " m")
```

---

#### `physics::projectile_at(x0, y0, v0, angle, t)`

Calcula la posición y velocidad en un tiempo específico.

**Parámetros:**
- `x0, y0, v0, angle`: Igual que `physics::projectile`
- `t` (número): Tiempo en segundos

**Retorna:**
- Array `[x, y, vx, vy]`

**Ejemplo:**
```rydit
# ¿Dónde está el proyectil a los 2 segundos?
dark.slot estado = physics::projectile_at(0, 0, 50, 45, 2)
print("Posición: (" + estado[0] + ", " + estado[1] + ")")
print("Velocidad: (" + estado[2] + ", " + estado[3] + ")")
```

---

### Gravedad N-Body

#### `physics::nbody_2(m1, m2, x1, y1, x2, y2, G)`

Calcula la fuerza gravitacional entre dos cuerpos.

**Parámetros:**
- `m1` (número): Masa del cuerpo 1 (kg)
- `m2` (número): Masa del cuerpo 2 (kg)
- `x1, y1` (números): Posición del cuerpo 1
- `x2, y2` (números): Posición del cuerpo 2
- `G` (número): Constante gravitacional (6.674×10⁻¹¹ m³/kg·s²)

**Retorna:**
- Array `[fx1, fy1, fx2, fy2, distancia]`
  - `fx1, fy1`: Fuerza en cuerpo 1
  - `fx2, fy2`: Fuerza en cuerpo 2 (igual magnitud, dirección opuesta)
  - `distancia`: Distancia entre cuerpos

**Fórmula:**
```
F = G * m1 * m2 / r²
```

**Ejemplo:**
```rydit
# Sistema Tierra-Luna
dark.slot G = 6.674e-11
dark.slot fuerzas = physics::nbody_2(
    5.97e24,    # Tierra (kg)
    7.35e22,    # Luna (kg)
    0, 0,       # Tierra en origen
    3.844e8, 0, # Luna a 384,400 km
    G
)
print("Fuerza: " + fuerzas[0] + " N")
print("Distancia: " + fuerzas[4] + " m")
```

---

### Ondas

#### `physics::wave_1d(x, t, lambda, freq)`

Calcula la amplitud de una onda 1D en un punto y tiempo dados.

**Parámetros:**
- `x` (número): Posición (metros)
- `t` (número): Tiempo (segundos)
- `lambda` (número): Longitud de onda (metros)
- `freq` (número): Frecuencia (Hz)

**Retorna:**
- Número: Amplitud (-1 a 1)

**Fórmula:**
```
y(x,t) = sin(kx - ωt)
k = 2π / λ
ω = 2π * f
```

**Ejemplo:**
```rydit
dark.slot amplitud = physics::wave_1d(1, 0.5, 2, 1)
print("Amplitud en x=1m, t=0.5s: " + amplitud)
```

---

#### `physics::wave_2d(x, y, t, lambda, freq)`

Calcula la amplitud de una onda circular 2D.

**Parámetros:**
- `x, y` (números): Posición (metros)
- `t, lambda, freq`: Igual que `wave_1d`

**Retorna:**
- Número: Amplitud (decae con 1/√r)

**Fórmula:**
```
y(r,t) = sin(kr - ωt) / √r
r = √(x² + y²)
```

**Ejemplo:**
```rydit
dark.slot amp = physics::wave_2d(3, 4, 0.5, 2, 1)
print("Amplitud en (3,4): " + amp)
```

---

### Péndulo

#### `physics::pendulum(length, angle0, t)`

Calcula el ángulo y velocidad angular de un péndulo simple.

**Parámetros:**
- `length` (número): Longitud del péndulo (metros)
- `angle0` (número): Ángulo inicial (grados)
- `t` (número): Tiempo (segundos)

**Retorna:**
- Array `[angulo, velocidad_angular, periodo]`

**Fórmulas:**
```
ω = √(g / L)
θ(t) = θ0 * cos(ωt)
ω(t) = -θ0 * ω * sin(ωt)
T = 2π / ω
```

**Ejemplo:**
```rydit
# Péndulo de 1m, 10° inicial, t=1s
dark.slot pendulo = physics::pendulum(1, 10, 1)
print("Ángulo: " + pendulo[0] + "°")
print("Velocidad angular: " + pendulo[1] + "°/s")
print("Período: " + pendulo[2] + " s")
```

---

## Casos de Uso

### 1. Simulación de Proyectil

```rydit
# Simular lanzamiento de pelota
dark.slot v0 = 30
dark.slot angle = 45
dark.slot g = 9.81

# Calcular trayectoria completa
dark.slot trayectora = physics::projectile(0, 0, v0, angle)

# Imprimir en intervalos
dark.slot t = 0
mientras t <= trayectora[2] {
    dark.slot estado = physics::projectile_at(0, 0, v0, angle, t)
    print("t=" + t + "s: (" + estado[0] + "m, " + estado[1] + "m)")
    t = t + 0.5
}
```

---

### 2. Órbita Simple

```rydit
# Simular fuerza gravitacional Tierra-Satélite
dark.slot G = 6.674e-11
dark.slot m_tierra = 5.97e24
dark.slot m_satelite = 1000  # 1000 kg

# Satélite a 400 km de altura
dark.slot r = 6.371e6 + 400e3  # Radio terrestre + altura

dark.slot fuerzas = physics::nbody_2(
    m_tierra, m_satelite,
    0, 0,
    r, 0,
    G
)

print("Fuerza gravitacional: " + fuerzas[0] + " N")
```

---

### 3. Visualización de Ondas

```rydit
# Dibujar onda 1D
dark.slot lambda = 2
dark.slot freq = 1
dark.slot x = 0

mientras x < 10 {
    dark.slot y = physics::wave_1d(x, 0, lambda, freq)
    # Dibujar barra (escalar y de -1..1 a 0..20)
    dark.slot barras = (y + 1) * 10
    dark.slot i = 0
    mientras i < barras {
        print(" ")
        i = i + 1
    }
    print("*")
    x = x + 0.5
}
```

---

## Constantes Físicas

```rydit
# Gravedad terrestre
dark.slot g = 9.81  # m/s²

# Constante gravitacional
dark.slot G = 6.674e-11  # m³/kg·s²

# Velocidad de la luz (para referencia)
dark.slot c = 299792458  # m/s

# Masa de la Tierra
dark.slot m_tierra = 5.97e24  # kg

# Radio de la Tierra
dark.slot r_tierra = 6.371e6  # m
```

---

## Limitaciones

1. **Sin resistencia del aire**: Los proyectiles no consideran drag
2. **Gravedad constante**: g = 9.81 m/s² siempre
3. **Ángulos pequeños**: Péndulo asume sin(θ) ≈ θ
4. **2 cuerpos**: nbody_2 solo soporta 2 cuerpos (no n-body general)
5. **Ondas ideales**: Sin amortiguamiento ni dispersión

---

## Ejemplos en el Repositorio

- `demos/demo_fisica_v0.7.1.2.rydit` - Demo completo

---

<div align="center">

**🛡️ RyDit v0.7.1.2 - Módulo de Física**

*Simulación física 2D desde RyDit*

</div>
