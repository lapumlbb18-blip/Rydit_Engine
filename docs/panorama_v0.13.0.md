# 🛡️ Ry-Dit - PANORAMA v0.13.0 → v1.0.0

**Fecha**: 2026-04-04
**Versión actual**: v0.12.1 ✅ Parser infalible + ry-god en crates.io
**Próxima versión**: v0.13.0 - Arrays completos + Math avanzado + Crates maduros
**Meta final**: v1.0.0 - Motor completo + Editor visual + Multiplataforma

---

## 📊 ESTADO ACTUAL v0.12.1

| Métrica | Valor |
|---------|-------|
| **Versión** | v0.12.1 |
| **Commit** | `22252bc` |
| **Errores** | 0 |
| **Warnings** | ~30 |
| **Crates** | 22 en workspace |
| **crates.io** | ry-god v0.1.0 ✅ publicado |
| **Parser** | ✅ Infalible (6 bugs raíz resueltos) |
| **Tests** | 15/15 parser revelación |
| **Plataforma** | Android/Termux (nativa) |

---

## 1️⃣ CONCATENACIÓN DE STRINGS

| Capa | Estado | Detalle |
|------|--------|---------|
| **Lexer** | ✅ | `+` → `TokenKind::Mas`, `+=` → `TokenKind::MasIgual` |
| **Parser** | ✅ | `parse_term()` → `Expr::Binary { Suma }` |
| **Evaluador** | ✅ | `Texto + Texto` → concat, `Texto + Num` → coerción |
| **Tests** | ⚠️ | Funcional sin tests edge-case dedicados |
| **Código** | ⚠️ | Duplicado entre `eval/mod.rs` y `main.rs` |

**Veredicto**: ✅ **FUNCIONAL**. Pendiente: tests dedicados + deduplicación.

---

## 2️⃣ FUNCIONES MATEMÁTICAS

### ✅ Implementadas

| Función | Sintaxis | Ubicación |
|---------|----------|-----------|
| Seno | `math::sin(x)` / `matematica::seno(x)` | `eval/mod.rs:180` |
| Coseno | `math::cos(x)` / `matematica::coseno(x)` | `eval/mod.rs:189` |
| Tangente | `math::tan(x)` / `matematica::tangente(x)` | `eval/mod.rs:198` |
| Raíz cuadrada | `math::sqrt(x)` / `matematica::raiz(x)` | `eval/mod.rs:167` |
| Piso | `math::floor(x)` / `matematica::piso(x)` | `eval/mod.rs:324` |
| Techo | `math::ceil(x)` / `matematica::techo(x)` | `eval/mod.rs:333` |
| Valor absoluto | `math::abs(x)` / `matematica::valor_absoluto(x)` | `eval/mod.rs:342` |
| Arcotangente 2 | `math::atan2(y, x)` | `eval/mod.rs:207` |
| Grados → radianes | `math::deg2rad(x)` | `eval/mod.rs:218` |
| Radianes → grados | `math::rad2deg(x)` | `eval/mod.rs:227` |
| FPS | `fps()` → 60.0 | Builtin |

### ❌ Pendientes v0.13.0

| Función | Fórmula | Prioridad |
|---------|---------|-----------|
| *(Todas las funciones math están implementadas ✅)* | — | — |

### ✅ IMPLEMENTADAS v0.13.0 (nuevas)

| Función | Descripción | Archivos |
|---------|-------------|----------|
| `math::pow(base, exp)` | x^n | `eval/mod.rs` + `main.rs` |
| `math::log(x)` | ln(x) | `eval/mod.rs` + `main.rs` |
| `math::log10(x)` | log₁₀(x) | `eval/mod.rs` + `main.rs` |
| `math::exp(x)` | e^x | `eval/mod.rs` + `main.rs` |
| `math::min(a, b)` | mín(a, b) | `eval/mod.rs` + `main.rs` |
| `math::max(a, b)` | máx(a, b) | `eval/mod.rs` + `main.rs` |
| `math::clamp(v, a, b)` | máx(a, mín(b, v)) | `eval/mod.rs` + `main.rs` |
| `math::lerp(a, b, t)` | a + (b - a) · t | `eval/mod.rs` + `main.rs` |
| `math::sign(x)` | signo(x) | `eval/mod.rs` + `main.rs` |
| `math::mod(a, b)` | a mod b | `eval/mod.rs` + `main.rs` |
| `math::round(x)` | Redondeo | `eval/mod.rs` + `main.rs` |
| `math::trunc(x)` | Truncar | `eval/mod.rs` + `main.rs` |
| `math::fract(x)` | Parte fraccional | `eval/mod.rs` + `main.rs` |
| `math::hypot(x, y)` | √(x² + y²) | `eval/mod.rs` + `main.rs` |
| `math::cbrt(x)` | Raíz cúbica | `eval/mod.rs` + `main.rs` |
| `math::PI` | π | `eval/mod.rs` + `main.rs` |
| `math::E` | e | `eval/mod.rs` + `main.rs` |
| `math::TAU` | 2π | `eval/mod.rs` + `main.rs` |
| `math::INF` | ∞ | `eval/mod.rs` + `main.rs` |
| `calc::derivada(f, x, h)` | [f(x+h) - f(x-h)] / 2h | `eval/mod.rs` |
| `calc::derivada2(f, x, h)` | [f(x+h) - 2f(x) + f(x-h)] / h² | `eval/mod.rs` |
| `calc::integral(f, a, b, n)` | Regla de Simpson | `eval/mod.rs` |
| `calc::integral_trapezio(f, a, b, n)` | Regla del trapecio | `eval/mod.rs` |

---

## 3️⃣ FÍSICA AVANZADA

### ✅ Implementadas

| Feature | Estado | Detalle |
|---------|--------|---------|
| Proyectiles 2D | ✅ | `ry-physics`: trayectoria, altura máxima, alcance |
| Gravedad 2 cuerpos | ✅ | `ry-physics`: N-Body (2 cuerpos) |
| Rigid Body 2D | ✅ | AABB, gravedad, colisiones, transferencia velocidad |
| Curvas Bezier | ✅ | `ry-science`: linear, quadratic, cubic (De Casteljau) |

### 🔬 Fórmulas Físicas PENDIENTES v0.13.0

#### Cinemática

| Fórmula | Nombre | Uso |
|---------|--------|-----|
| `v = v₀ + a·t` | Velocidad final | Movimiento acelerado |
| `x = x₀ + v₀·t + ½·a·t²` | Posición | Caída libre, proyectiles |
| `v² = v₀² + 2·a·Δx` | Velocidad sin tiempo | Frenado, aceleración |
| `h_max = v₀² / (2·g)` | Altura máxima | Proyectil vertical |
| `R = v₀²·sin(2θ) / g` | Alcance horizontal | Proyectil oblicuo |
| `t_vuelo = 2·v₀·sin(θ) / g` | Tiempo de vuelo | Proyectil oblicuo |

#### Dinámica

| Fórmula | Nombre | Uso |
|---------|--------|-----|
| `F = m·a` | 2da Ley Newton | Fuerza neta |
| `p = m·v` | Momentum lineal | Colisiones |
| `K = ½·m·v²` | Energía cinética | Conservación energía |
| `U = m·g·h` | Energía potencial grav. | Campos gravitatorios |
| `W = F·d·cos(θ)` | Trabajo mecánico | Fuerzas aplicadas |
| `P = W / t` | Potencia | Tasa de trabajo |
| `F_fricción = μ·N` | Fricción cinética/estática | Superficies |
| `F_elástica = -k·x` | Ley de Hooke | Resortes |

#### Gravitación

| Fórmula | Nombre | Uso |
|---------|--------|-----|
| `F = G·m₁·m₂ / r²` | Ley gravitación universal | N-Body >2 cuerpos |
| `U_grav = -G·m₁·m₂ / r` | Energía potencial grav. | Órbitas |
| `v_escape = √(2·G·M / r)` | Velocidad de escape | Simulaciones espaciales |

#### Ondas y Oscilaciones

| Fórmula | Nombre | Uso |
|---------|--------|-----|
| `x(t) = A·cos(ω·t + φ)` | Oscilador armónico | Péndulo, resortes |
| `ω = √(k / m)` | Frecuencia angular | Resorte masa |
| `T = 2π·√(L / g)` | Período péndulo simple | Péndulo |
| `v_onda = λ·f` | Velocidad de onda | Sonido, luz |

#### Fluidos

| Fórmula | Nombre | Uso |
|---------|--------|-----|
| `F_arrastre = ½·ρ·v²·Cd·A` | Resistencia del aire | Paracaídas, caída real |
| `P = ρ·g·h` | Presión hidrostática | Líquidos |
| `F_empuje = ρ_fluido·V·g` | Principio de Arquímedes | Flotación |

#### Termodinámica

| Fórmula | Nombre | Uso |
|---------|--------|-----|
| `Q = m·c·ΔT` | Calor específico | Transferencia térmica |
| `PV = nRT` | Gas ideal | Simulaciones gas |
| `ΔS ≥ Q / T` | 2da Ley termodinámica | Entropía |

### 🧮 Cálculo: DERIVADAS e INTEGRALES

#### Derivadas Numéricas

| Función | Fórmula | Uso |
|---------|---------|-----|
| `calc::derivada(f, x, h)` | f'(x) ≈ [f(x+h) - f(x-h)] / 2h | Diferencias centradas |
| `calc::derivada2(f, x, h)` | f''(x) ≈ [f(x+h) - 2f(x) + f(x-h)] / h² | Segunda derivada |
| `calc::gradiente(f, [x,y], h)` | ∇f = [∂f/∂x, ∂f/∂y] | Campos escalares 2D |
| `calc::jacobiana(f, x, h)` | J = [∂fi/∂xj] | Sistemas de ecuaciones |

**Implementación**: Método de diferencias finitas con paso h configurable (default: 1e-8).

```rydit
# Ejemplo: derivada de x² en x=3 → debería dar 6
rytmo f(x) {
    return x * x
}

resultado = calc::derivada(f, 3, 0.0001)
# resultado ≈ 6.0
```

#### Integrales Numéricas

| Función | Fórmula | Uso |
|---------|---------|-----|
| `calc::integral(f, a, b, n)` | Σ f(xi)·Δx (Riemann) | Área bajo curva |
| `calc::integral_simpson(f, a, b, n)` | (h/3)·[f₀ + 4f₁ + 2f₂ + ... + fn] | Mayor precisión |
| `calc::integral_trapezio(f, a, b, n)` | (h/2)·[f₀ + 2f₁ + 2f₂ + ... + fn] | Compromiso |

**Implementación**: Regla de Simpson (mayor precisión) como default, Riemann como fallback.

```rydit
# Ejemplo: integral de x² de 0 a 1 → debería dar 1/3
rytmo f(x) {
    return x * x
}

area = calc::integral_simpson(f, 0, 1, 100)
# area ≈ 0.3333...
```

#### Aplicaciones Físicas del Cálculo

| Aplicación | Fórmula | Uso en Ry-Dit |
|------------|---------|---------------|
| **Velocidad desde posición** | v(t) = dx/dt | Derivada numérica de trayectoria |
| **Aceleración desde velocidad** | a(t) = dv/dt | Derivada segunda |
| **Trabajo desde fuerza** | W = ∫F·dx | Integral de fuerza variable |
| **Centro de masa** | x_cm = ∫x·dm / ∫dm | Objetos no uniformes |
| **Momento de inercia** | I = ∫r²·dm | Rotación de cuerpos |
| **Flujo de campo** | Φ = ∫∫F·n·dA | Fluidos, electromagnetismo |
| **Energía total** | E = ∫P(t)dt | Potencia integrada |

---

## 4️⃣ ARRAYS

| Feature | Estado | Detalle |
|---------|--------|---------|
| Literales `[1, 2, 3]` | ✅ | `parse_primary()` |
| Declaración `dark.slot[]` | ✅ | Parser soporta |
| Strings en arrays | ✅ | `["rojo", "verde"]` |
| **Acceso `arr[i]`** | ⚠️ | Verificar |
| **Asignación `arr[i] = v`** | ⚠️ | Verificar |
| **`push(elem)`** | ❌ | No implementado |
| **`pop()`** | ❌ | No implementado |
| **`len(arr)`** | ❌ | No implementado |
| **`slice(arr, a, b)`** | ❌ | No implementado |
| **`insert(arr, i, elem)`** | ❌ | No implementado |
| **`remove(arr, i)`** | ❌ | No implementado |
| **Arrays multidimensionales** | ❌ | `[[1,2],[3,4]]` |
| **`for x en arr`** | ⚠️ | Verificar |

**Veredicto**: ⚠️ **PARCIAL**. Creación funciona, operaciones faltan.

---

## 5️⃣ TIPO Vec2 NATIVO

| Feature | Estado |
|---------|--------|
| `Vec2` como tipo del lenguaje | ❌ No existe |
| `vec2(x, y)` constructor | ❌ |
| `v.x`, `v.y` acceso | ❌ |
| `v1 + v2` suma | ❌ |
| `v1 - v2` resta | ❌ |
| `v * scalar` escalado | ❌ |
| `v.magnitud()` | ❌ |
| `v.normalizar()` | ❌ |
| `v1 · v2` producto punto | ❌ |
| `v1 × v2` producto cruz (2D→scalar) | ❌ |
| `v.ángulo()` | ❌ |
| `v.rotar(θ)` | ❌ |
| `vec2::lerp(a, b, t)` | ❌ |
| `vec2::dist(a, b)` | ❌ |

**Veredicto**: ❌ **NO EXISTE**. Solo `Vector2` de raylib en FFI interno.

---

## 6️⃣ SALTO A 3D

### Infraestructura existente

| Componente | Estado | Reutilizable para 3D |
|------------|--------|---------------------|
| OpenGL FFI (`gl 0.14.0`) | ✅ | Sí (vertex arrays, shaders) |
| GPU Instancing | ✅ | Sí (mat4 projection, instanced draw) |
| Shaders GLSL | ✅ | Sí (vertex.glsl, fragment.glsl) |
| raylib `5.5.1` | ✅ | Sí (tiene API 3D completa) |
| Camera2D | ✅ | Patrón reutilizable |
| Render Queue | ✅ | Adaptable a 3D |

### ❌ Lo que NO existe

| Componente | Esfuerzo | Prioridad |
|------------|----------|-----------|
| **ry-geometry crate** | Alto (12-16h) | 🔴 |
| `Vec3(x, y, z)` | Medio | 🔴 |
| `Mat4` (4x4 matrix) | Alto | 🔴 |
| **Quaternions** | Alto | 🔴 |
| **Euler angles** | Medio | 🟡 |
| `Camera3D` | Alto | 🔴 |
| `DrawCube`, `DrawSphere` | Medio | 🔴 |
| `DrawCylinder`, `DrawMesh` | Medio | 🟡 |
| Iluminación 3D | Alto | 🟡 |
| **Física 3D (AABB, OBB)** | Alto (16-24h) | 🟡 |
| Raycasting 3D | Alto | 🟢 |

### 🗺️ Ruta mínima al 3D (3 pasos)

```
Paso 1: ry-geometry crate (v0.14.0)
  ├── Vec3 (x, y, z) + ops
  ├── Mat4 identidad, traslación, rotación, escala
  ├── Quaternions (from_euler, to_matrix, slerp)
  └── Tests unitarios

Paso 2: ry-gfx expone 3D (v0.14.0)
  ├── DrawCube(pos, size, color)
  ├── DrawSphere(center, radius, color)
  ├── DrawCylinder(pos, r_top, r_bottom, height, color)
  ├── Camera3D (position, target, up, fov, projection)
  └── Demo: cubos rotando

Paso 3: Física 3D básica (v0.15.0)
  ├── AABB 3D
  ├── Colisión esfera-caja
  ├── Gravedad 3D
  └── Rigid Body 3D
```

---

## 7️⃣ CRATES PARA CRATES.IO

### ✅ Publicables YA (con fixes menores)

| Crate | Versión | Estado | Pendiente |
|-------|---------|--------|-----------|
| **ry-god** | 0.1.0 | ✅ **PUBLICADO** | Nada |
| **ry-core** | 0.8.2 | ✅ | Tests completos |
| **ry-lexer** | 0.1.0 | ✅ | Tests unitarios |
| **ry-parser** | 0.1.0 | ✅ | Tests + docs |
| **ry-physics** | 0.7.34 | ✅ | Tests 3D futuro |
| **ry-anim** | 0.7.34 | ✅ | Tests adicionales |
| **ry-ecs** | 0.10.0 | ✅ | Docs |
| **ry-script** | 0.8.2 | ✅ | Nada grave |
| **ry-stream** | 0.1.0 | ✅ | Tests |
| **lizer** | 0.11.2 | ✅ | Legacy wrapper |
| **toolkit-ry** | 0.1.0 | ⚠️ | Agregar license |
| **ry-system-ry** | 0.11.0 | ⚠️ | Agregar license |

### ⚠️ Requieren trabajo

| Crate | Pendiente |
|-------|-----------|
| **ry-vm** | Metadata (version, description, license) |
| **ry-gfx** | `links = "raylib"` → problema portabilidad |
| **ry-science** | Metadata + Bezier tests |
| **ry-loader** | Metadata completa |
| **blast-core** | description + license |
| **migui** | Metadata completa |
| **ry-test** | Definir si es publicable |

**Total**: ~12 crates listos (fixes menores), ~7 requieren trabajo.

---

## 8️⃣ MULTIPLATAFORMA

| Plataforma | Estado | Pendiente |
|------------|--------|-----------|
| **Android/Termux** | ✅ **NATIVA** | Nada (desarrollo aquí) |
| **Linux** | ⚠️ Parcial | Build verificado, SDL2/raylib libs |
| **Windows** | ❌ No probado | Toolchain cross-compile |
| **WASM** | ❌ No iniciado | `wasm32-unknown-unknown` |

### 📦 Crate `ry-platform` (pendiente)

| Feature | Descripción |
|---------|-------------|
| Abstracción de inputs | Unificar teclado Android vs Linux vs Windows |
| Abstracción de paths | Rutas de assets por plataforma |
| Conditional compilation | `#[cfg(target_os = "android")]` centralizado |
| System info | RAM, CPU, GPU, OS detect |
| Window creation | Crear ventana nativa por plataforma |

---

## 9️⃣ EDITOR VISUAL + CLI COMUNIDAD

| Herramienta | Estado | Prioridad |
|-------------|--------|-----------|
| **migui** (immediate GUI) | ✅ 12 widgets | Base existente |
| **toolkit-ry** | ⚠️ Parcial | Completar |
| **Editor visual** | ❌ No existe | v0.16.0 |
| **CLI `ry`** | ⚠️ Parcial | v0.13.0 |
| **Hot reload** | ❌ No implementado | v0.16.0 |
| **Debug mode** | ❌ No implementado | v0.16.0 |
| **rybot CLI** | ⚠️ Básico | Mejorar |

### CLI Planeado

```bash
ry init mi_juego          # Crear proyecto desde template
ry run                    # Ejecutar proyecto
ry build --release        # Compilar release
ry build --target wasm32  # Compilar WASM
ry debug                  # Modo debug con logs
ry test                   # Correr tests .rydit
ry format                 # Formatear código .rydit
ry lint                   # Analizar estilo
ry package                # Empaquetar para distribución
```

---

## 🗺️ ROADMAP COMPLETO

```
v0.12.1 ✅ ACTUAL — Parser infalible + ry-god crates.io
   │
   ├── v0.13.0 ✅ Math + Cálculo + Arrays + Vec2 + Crates
   │     ✅ math::pow, log, log10, exp, PI, E, TAU, INF
   │     ✅ math::min, max, clamp, lerp, sign, mod
   │     ✅ math::round, trunc, fract, hypot, cbrt
   │     ✅ calc::derivada, derivada2 (diferencias centradas)
   │     ✅ calc::integral (Simpson + Trapecio)
   │     ⏳ Array push/pop/len/slice/insert/remove
   │     ⏳ Vec2 tipo nativo + ops
   │     ⏳ 12 crates listos para crates.io
   │     ⏳ CLI básico (init, run, build, format)
   │
   ├── v0.14.0 🔮 3D Básico
   │     ├── ry-geometry crate (Vec3, Mat4, Quaternions)
   │     ├── ry-gfx: DrawCube, DrawSphere, Camera3D
   │     ├── Euler angles + transformaciones
   │     └── Demo 3D: cubos rotando
   │
   ├── v0.15.0 🔮 Física 3D + Multiplataforma
   │     ├── AABB 3D, colisiones esfera-caja
   │     ├── N-Body >2 cuerpos (gravitación universal)
   │     ├── ry-platform crate (abstracción OS)
   │     ├── Builds Linux + Windows verificados
   │     └── CI/CD GitHub Actions
   │
   ├── v0.16.0 🔮 Editor + Herramientas
   │     ├── Editor visual básico (migui + toolkit-ry)
   │     ├── Hot reload de scripts .rydit
   │     ├── Debug mode step-by-step
   │     └── rybot debug avanzado
   │
   └── v1.0.0 🔮 Motor Completo
         ├── 5+ demos funcionales multiplataforma
         ├── 15+ crates publicados en crates.io
         ├── Documentación completa (usuario + dev)
         ├── Videos tutoriales
         ├── Editor visual funcional
         └── Comunidad activa
```

---

## 🎯 PRIORIDADES INMEDIATAS

| # | Tarea | Impacto | Esfuerzo | Versión |
|---|-------|---------|----------|---------|
| **1** | ✅ `math::pow, log, PI, min, max, lerp, clamp` + derivadas + integrales | Alto | ✅ Hecho | v0.13.0 |
| **2** | ⏳ Array `push/pop/len/slice` builtins | Alto | 3-4h | v0.13.0 |
| **3** | ⏳ `Vec2` tipo nativo + ops | Alto | 4-6h | v0.13.0 |
| **4** | ⏳ Tests + metadata 12 crates | Alto | 6-8h | v0.13.0 |
| **5** | ⏳ Publicar 5+ crates crates.io | Alto | 1h | v0.13.0 |
| **6** | ⏳ CLI básico (`ry init/run/build`) | Medio | 4-6h | v0.13.0 |
| **7** | ⏳ `ry-geometry` (Vec3, Mat4, Quat) | Alto | 12-16h | v0.14.0 |
| **8** | ⏳ Camera3D + DrawCube en ry-gfx | Alto | 12-16h | v0.14.0 |
| **9** | ⏳ Física 3D (AABB, colisiones) | Alto | 16-24h | v0.15.0 |
| **10** | ⏳ `ry-platform` multiplataforma | Alto | 16-24h | v0.15.0 |
| **11** | ⏳ Editor visual + hot reload | Alto | 24-32h | v0.16.0 |

---

## 📐 REFERENCIA: FÓRMULAS FÍSICAS COMPLETAS

### Cinemática 1D

```
v = v₀ + a·t
x = x₀ + v₀·t + ½·a·t²
v² = v₀² + 2·a·(x - x₀)
```

### Proyectiles 2D

```
x(t) = v₀·cos(θ)·t
y(t) = v₀·sin(θ)·t - ½·g·t²
h_max = v₀²·sin²(θ) / (2·g)
R = v₀²·sin(2θ) / g
t_vuelo = 2·v₀·sin(θ) / g
```

### Dinámica

```
ΣF = m·a
p = m·v          (momentum)
K = ½·m·v²       (energía cinética)
W = ∫F·dx        (trabajo)
P = dW/dt        (potencia)
F_gravedad = G·m₁·m₂ / r²
F_resorte = -k·x (Ley de Hooke)
F_fricción = μ·N
```

### Oscilaciones

```
x(t) = A·cos(ω·t + φ)
ω = √(k/m)
T = 2π/ω = 2π·√(m/k)
Péndulo: T ≈ 2π·√(L/g)  (θ pequeño)
```

### Cálculo Numérico

```
Derivada (diferencias centradas):
  f'(x) ≈ [f(x+h) - f(x-h)] / 2h

Segunda derivada:
  f''(x) ≈ [f(x+h) - 2f(x) + f(x-h)] / h²

Integral (Riemann):
  ∫f(x)dx ≈ Σ f(xi)·Δx

Integral (Simpson):
  ∫f(x)dx ≈ (h/3)·[f(x₀) + 4f(x₁) + 2f(x₂) + 4f(x₃) + ... + f(xn)]

Regla del trapecio:
  ∫f(x)dx ≈ (h/2)·[f(x₀) + 2·f(x₁) + 2·f(x₂) + ... + f(xn)]
```

### Gradiente y Campos

```
Gradiente 2D:
  ∇f(x,y) = [∂f/∂x, ∂f/∂y]

Laplaciano 2D:
  ∇²f = ∂²f/∂x² + ∂²f/∂y²

Campo gravitatorio:
  g(r) = -G·M·r̂ / r²

Potencial gravitatorio:
  Φ(r) = -G·M / r
```

---

## 🏗️ ESTRUCTURA DE CRATES ACTUAL

```
shield-project/
├── crates/
│   ├── ry-core/          ✅ 0.8.2  Traits, types, module system
│   ├── ry-lexer/         ✅ 0.1.0  Zero-copy lexer
│   ├── ry-parser/        ✅ 0.1.0  Parser AST + zero-copy
│   ├── ry-vm/            ⚠️       VM opcodes + compiler
│   ├── ry-gfx/           ⚠️ 0.10.7 Graphics (raylib + SDL2 + OpenGL)
│   ├── ry-physics/       ✅ 0.7.34 2D projectile + N-body
│   ├── ry-anim/          ✅ 0.7.34 Easing + Disney principles
│   ├── ry-ecs/           ✅ 0.10.0 ECS (bevy_ecs)
│   ├── ry-science/       ⚠️       Geometry 2D + stats
│   ├── ry-script/        ✅ 0.8.2  Script loading
│   ├── ry-stream/        ✅ 0.1.0  LAN streaming
│   ├── ry-god/           ✅ 0.1.0  Security & efficiency (crates.io)
│   ├── ry-loader/        ⚠️       Module loader
│   ├── ry-rs/            —        Main binary
│   ├── ry-system-ry/     ⚠️ 0.11.0 Universal system (SDL2)
│   ├── ry-test/          ⚠️       Test utilities
│   ├── toolkit-ry/       ⚠️ 0.1.0  UI toolkit (SDL2)
│   ├── migui/            ⚠️       Immediate mode GUI
│   ├── blast-core/       ⚠️ 0.1.0  Value executor
│   ├── lizer/            ✅ 0.11.2 Legacy lexer wrapper
│   └── v-shield/         ⚠️       (por definir)
│
├── docs/
│   ├── actuales/
│   ├── antiguos/
│   │   ├── sdl2/
│   │   ├── demos/
│   │   └── guias/
│   ├── sessions/
│   ├── tests/
│   └── tests_referencia/
│
├── demos/                Scripts .rydit
├── screenshots/          Capturas y videos
└── scripts/              Utilidades
```

---

<div align="center">

**🛡️ Ry-Dit v0.12.1 — PANORAMA v0.13.0 → v1.0.0**

*Math avanzado + Arrays completos → 3D → Multiplataforma → Editor → v1.0.0*

*Última actualización: 2026-04-04*

</div>
