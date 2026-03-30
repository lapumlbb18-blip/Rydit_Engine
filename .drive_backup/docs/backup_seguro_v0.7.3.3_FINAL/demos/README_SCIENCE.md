# 🧪 Módulo RyDit Science - v0.7.1.4

**Módulo unificado de ciencia y matemáticas aplicadas**

Este módulo contiene todas las funciones para:
- 📈 **Curvas de Bezier** (diseño gráfico, animaciones)
- ⚛️ **Física 2D** (simulaciones, juegos, visualización científica)
- 📊 **Ciencia de Datos** (análisis estadístico, CSV, gráficos)

---

## 📡 Curvas de Bezier

### Funciones Disponibles

#### `bezier::linear(p0_x, p0_y, p1_x, p1_y, t)`
Interpolación lineal entre 2 puntos.

**Retorna:** `[x, y]` - Punto en la curva en el parámetro t

**Ejemplo:**
```rydit
dark.slot p = bezier::linear(0, 0, 100, 100, 0.5)
# Retorna: [50, 50] - punto medio
```

---

#### `bezier::quadratic(p0_x, p0_y, p1_x, p1_y, p2_x, p2_y, t)`
Curva cuadrática con 1 punto de control.

**Retorna:** `[x, y]`

**Ejemplo:**
```rydit
# P0=(0,0), P1=(50,100) [control], P2=(100,0)
dark.slot q = bezier::quadratic(0, 0, 50, 100, 100, 0, 0.5)
# Retorna: [50, 50]
```

---

#### `bezier::cubic(p0_x, p0_y, p1_x, p1_y, p2_x, p2_y, p3_x, p3_y, t)`
Curva cúbica con 2 puntos de control.

**Retorna:** `[x, y]`

**Ejemplo:**
```rydit
# Curva en S: P0=(0,0), P1=(30,100), P2=(70,100), P3=(100,0)
dark.slot c = bezier::cubic(0, 0, 30, 100, 70, 100, 100, 0, 0.5)
# Retorna: [50, 75]
```

---

#### `bezier::cubic_derivative(p0_x, p0_y, p1_x, p1_y, p2_x, p2_y, p3_x, p3_y, t)`
Calcula la tangente (derivada) de una curva cúbica.

**Retorna:** `[dx, dy]` - Vector tangente

**Ejemplo:**
```rydit
dark.slot tang = bezier::cubic_derivative(0, 0, 30, 100, 70, 100, 100, 0, 0.5)
# Retorna: [105, 0] - tangente horizontal
```

---

#### `bezier::generate_points([puntos_control], steps)`
Genera múltiples puntos a lo largo de la curva usando el algoritmo de De Casteljau.

**Parámetros:**
- `puntos_control`: Array de arrays `[[x0,y0], [x1,y1], ...]`
- `steps`: Número de puntos a generar

**Retorna:** Array de puntos `[[x0,y0], [x1,y1], ...]`

**Ejemplo:**
```rydit
dark.slot ctrl = [[0, 0], [30, 100], [70, 100], [100, 0]]
dark.slot pts = bezier::generate_points(ctrl, 10)
# Retorna: 10 puntos a lo largo de la curva
```

---

## ⚛️ Física 2D

### Proyectiles

#### `physics::projectile(x0, y0, v0, angle)`
Calcula la trayectoria completa de un proyectil.

**Retorna:** `[x_final, y_final, tiempo_vuelo, altura_max, alcance]`

**Ejemplo:**
```rydit
dark.slot t = physics::projectile(0, 0, 50, 45)
# Retorna: [254.84, 0, 7.21, 63.71, 254.84]
```

---

#### `physics::projectile_at(x0, y0, v0, angle, t)`
Posición y velocidad en un tiempo específico.

**Retorna:** `[x, y, vx, vy]`

**Ejemplo:**
```rydit
dark.slot p = physics::projectile_at(0, 0, 50, 45, 2)
# Retorna: [70.71, 51.09, 35.36, 15.74]
```

---

### Gravedad

#### `physics::nbody_2(m1, m2, x1, y1, x2, y2, G)`
Fuerza gravitacional entre 2 cuerpos.

**Retorna:** `[fx1, fy1, fx2, fy2, distancia]`

**Ejemplo:**
```rydit
dark.slot G = 6.674e-11
dark.slot f = physics::nbody_2(5.97e24, 7.35e22, 0, 0, 3.844e8, 0, G)
# Retorna: [3.34e22, 0, -3.34e22, 0, 3.844e8]
```

---

### Ondas

#### `physics::wave_1d(x, t, lambda, freq)`
Amplitud de onda 1D.

**Retorna:** Amplitud (-1 a 1)

---

#### `physics::wave_2d(x, y, t, lambda, freq)`
Amplitud de onda circular 2D.

**Retorna:** Amplitud (decae con 1/√r)

---

### Péndulo

#### `physics::pendulum(length, angle0, t)`
Ángulo y velocidad de péndulo simple.

**Retorna:** `[angulo, velocidad_angular, periodo]`

---

## 📊 Ciencia de Datos

### CSV

#### `csv::parse(csv_text)`
Parsea CSV con headers.

**Retorna:** Array de filas (sin headers)

**Ejemplo:**
```rydit
dark.slot csv = "nombre,edad
Juan,25
Maria,30"
dark.slot filas = csv::parse(csv)
# Retorna: [[Juan, 25], [Maria, 30]]
```

---

#### `csv::parse_no_headers(csv_text)`
Parsea CSV sin headers.

---

### Estadísticas

#### `stats::mean(array)`
Media aritmética.

**Ejemplo:**
```rydit
dark.slot m = stats::mean([1, 2, 3, 4, 5])
# Retorna: 3.0
```

---

#### `stats::median(array)`
Mediana (valor central).

**Ejemplo:**
```rydit
dark.slot m = stats::median([1, 2, 3, 4, 5])
# Retorna: 3.0
dark.slot m2 = stats::median([1, 2, 3, 4])
# Retorna: 2.5
```

---

#### `stats::std_dev(array)`
Desviación estándar muestral.

---

#### `stats::min(array)` / `stats::max(array)`
Valor mínimo / máximo.

---

### Gráficos

#### `plot::ascii_chart(data, width)`
Genera gráfico ASCII.

**Ejemplo:**
```rydit
dark.slot g = plot::ascii_chart([10, 50, 30, 80, 60], 30)
print(g)
```

**Salida:**
```
          *                   
                            
       *     *              
                            
    *           *           
                            
 *                          
                            
                            
```

---

#### `plot::svg_chart(data, width, height)`
Genera código SVG para gráfico de líneas.

**Retorna:** String con código SVG

**Ejemplo:**
```rydit
dark.slot svg = plot::svg_chart([10, 50, 30, 80, 60], 300, 200)
# svg puede insertarse directamente en HTML
```

---

## 📁 Archivos del Módulo

- `demos/demo_bezier_simple.rydit` - Demo de curvas de Bezier
- `demos/demo_fisica_simple.rydit` - Demo de física 2D
- `demos/demo_datos_simple.rydit` - Demo de ciencia de datos

---

## 🧪 Tests

**Total:** 163 tests passing
- Bezier: 5 tests
- Física: 5 tests
- Datos: 6 tests

---

## 📊 Métricas

| Métrica | Valor |
|---------|-------|
| **Funciones** | 26 |
| **Tests** | 163 passing |
| **Binario** | ~700 KB |
| **Dependencias** | 9 (csv) |

---

<div align="center">

**🛡️ RyDit Science v0.7.1.4**

*Bezier + Física + Datos en un solo módulo*

</div>
