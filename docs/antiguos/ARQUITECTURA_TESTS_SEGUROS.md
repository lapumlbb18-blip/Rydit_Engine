# 🛡️ RyDit v0.11.1 - ARQUITECTURA DE TESTS SEGURA

**Fecha**: 2026-04-01  
**Problema**: 54 binarios, tests gráficos fallan al inicio, implementación insegura  
**Solución**: Tests en 3 niveles desde el núcleo

---

## 🔍 **DIAGNÓSTICO ACTUAL**

### **Problemas Detectados**

| Problema | Impacto | Causa Raíz |
|----------|---------|------------|
| **54 binarios en src/bin/** | 🔴 ALTO | Sin organización, duplicación |
| **Tests gráficos fallan al inicio** | 🔴 ALTO | Dependencias gráficas pesadas |
| **Lizer con límites (200 líneas)** | 🟡 MEDIO | Parser no optimizado |
| **Imports no auto-cargados** | 🟡 MEDIO | Puentes manuales |
| **Binarios tardan en compilar** | 🟡 MEDIO | Sobrecarga de imports |

---

## 🎯 **SOLUCIÓN: TESTS EN 3 NIVELES**

```
┌─────────────────────────────────────────────────┐
│  NIVEL 3: Tests Gráficos (Termux-X11)           │
│  - Binarios reales (.rs)                        │
│  - Solo después de Nivel 1 + 2 ✅               │
│  - 5-10 binarios MÁXIMO (no 54)                 │
└─────────────────────────────────────────────────┘
                      ↑
┌─────────────────────────────────────────────────┐
│  NIVEL 2: Tests de Integración (sin gráficos)   │
│  - Rybot + RyditModule + Modules                │
│  - Ejecutor + Blast-core                        │
│  - Rápidos (< 5 segundos)                       │
└─────────────────────────────────────────────────┘
                      ↑
┌─────────────────────────────────────────────────┐
│  NIVEL 1: Tests de Núcleo (unitarios)           │
│  - Lizer (parser)                               │
│  - Blast-core (executor)                        │
│  - RyditModule (trait)                          │
│  - Ultrarrápidos (< 1 segundo)                  │
└─────────────────────────────────────────────────┘
```

---

## 📋 **IMPLEMENTACIÓN PROPUESTA**

### **Nivel 1: Núcleo (crates/rydit-test/tests/)**

```rust
// tests/core_lizer_test.rs
// ✅ Sin gráficos, < 1 segundo

#[test]
fn test_parse_numero() {
    let mut lizer = Lizer::new("5");
    let tokens = lizer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
}

#[test]
fn test_parse_expresion() {
    let mut lizer = Lizer::new("2 + 3 * 4");
    let tokens = lizer.tokenize().unwrap();
    assert!(tokens.len() >= 5);
}
```

```rust
// tests/core_blast_test.rs
// ✅ Sin gráficos, < 1 segundo

#[test]
fn test_executor_guardar_leer() {
    let mut executor = Executor::new();
    executor.guardar("x", Valor::Num(5.0));
    assert_eq!(executor.leer("x"), Some(Valor::Num(5.0)));
}
```

```rust
// tests/core_ryditmodule_test.rs
// ✅ Sin gráficos, < 1 segundo

#[test]
fn test_registro_modulos() {
    let mut registry = ModuleRegistry::new();
    registry.register(PhysicsModule);
    assert!(registry.contains("physics"));
}
```

---

### **Nivel 2: Integración (crates/rydit-test/tests/)**

```rust
// tests/integration_rybot_test.rs
// ✅ Sin gráficos, < 5 segundos

#[test]
fn test_rybot_detecta_modulos() {
    let mut rybot = RyBot::new();
    rybot.registrar_modulo("physics");
    assert!(rybot.modulo_registrado("physics"));
}

#[test]
fn test_rybot_alerta_no_usados() {
    let mut rybot = RyBot::new();
    rybot.check_unused_modules();
    // Rybot detecta módulos no usados
}
```

```rust
// tests/integration_evaluator_test.rs
// ✅ Sin gráficos, < 5 segundos

#[test]
fn test_evaluar_script_basico() {
    let source = "x = 5\ny = x + 3";
    let result = evaluar_script(source);
    assert!(result.is_ok());
}
```

---

### **Nivel 3: Gráficos (SOLO después de Nivel 1 + 2)**

```rust
// bins/test_audio_sdl2.rs
// ✅ Gráficos, > 5 segundos
// SOLO si Nivel 1 + 2 pasan

fn main() {
    let mut audio = AudioSystemSDL2::new().unwrap();
    audio.load_sound("click", "test.wav");
    audio.play_sound("click");
}
```

```rust
// bins/test_render_sdl2.rs
// ✅ Gráficos, > 5 segundos
// SOLO si Nivel 1 + 2 pasan

fn main() {
    let mut gfx = RyditGfx::new("Test", 800, 600);
    gfx.draw_circle(400, 300, 50, ColorRydit::Rojo);
}
```

---

## 🎯 **REORGANIZACIÓN DE BINARIOS**

### **Actuales (54 binarios) → Propuestos (12 binarios)**

| Categoría | Actuales | Propuestos | Acción |
|-----------|----------|------------|--------|
| **Tests Núcleo** | 0 | 3 | ✅ Crear |
| **Tests Integración** | 0 | 3 | ✅ Crear |
| **Tests Gráficos** | 54 | 6 | 🔴 Eliminar 48 |
| **Demos Reales** | 0 | 3 | ✅ Crear |

### **Binarios Propuestos**

```
crates/rydit-rs/src/bin/
├── core_tests.rs          # Nivel 1 (test runner)
├── integration_tests.rs   # Nivel 2 (test runner)
├── test_audio_sdl2.rs     # Nivel 3 (gráfico)
├── test_render_sdl2.rs    # Nivel 3 (gráfico)
├── test_input_sdl2.rs     # Nivel 3 (gráfico)
├── demo_snake.rs          # Demo real
├── demo_platformer.rs     # Demo real
└── demo_particles.rs      # Demo real
```

---

## 🔧 **FLUJO DE TRABAJO SEGURO**

### **Flujo Actual (INSEGURO)**
```
1. Implementar feature
2. Crear binario gráfico
3. Compilar (tarda)
4. Falla en Termux-X11 ❌
5. Debuggear (horas)
6. Fixear (sin entender causa)
7. Funciona... por ahora
```

### **Flujo Propuesto (SEGURO)**
```
1. Implementar feature en núcleo
2. Test Nivel 1 (< 1 segundo)
   ✅ Pasa → Continuar
   ❌ Falla → Fixear (fácil, sin gráficos)
3. Test Nivel 2 (< 5 segundos)
   ✅ Pasa → Continuar
   ❌ Falla → Fixear (integración)
4. Test Nivel 3 (gráfico)
   ✅ Pasa → Feature completa
   ❌ Falla → Debuggear (ya hay base sólida)
```

---

## 📊 **BENEFICIOS ESPERADOS**

| Métrica | Actual | Propuesto | Mejora |
|---------|--------|-----------|--------|
| **Binarios** | 54 | 12 | -78% ✅ |
| **Tiempo test (falla)** | 10+ min | < 1 seg | -99% ✅ |
| **Debugging** | Horas | Minutos | -90% ✅ |
| **Compilación** | Lenta | Rápida | 10x ✅ |
| **Entendimiento** | Bajo | Alto | 5x ✅ |

---

## 🛠️ **IMPLEMENTACIÓN PASO A PASO**

### **Semana 1: Núcleo**
1. ✅ Crear `crates/rydit-test/`
2. ✅ Tests de Lizer (10 tests)
3. ✅ Tests de Blast-core (10 tests)
4. ✅ Tests de RyditModule (10 tests)

### **Semana 2: Integración**
1. 🔮 Tests de Rybot (5 tests)
2. 🔮 Tests de Evaluator (5 tests)
3. 🔮 Tests de Modules (5 tests)

### **Semana 3: Gráficos (SOLO si 1 + 2 pasan)**
1. 🔮 Test Audio SDL2 (1 binario)
2. 🔮 Test Render SDL2 (1 binario)
3. 🔮 Test Input SDL2 (1 binario)

### **Semana 4: Demos Reales**
1. 🔮 Demo Snake (1 binario)
2. 🔮 Demo Platformer (1 binario)
3. 🔮 Demo Particles (1 binario)

---

## 💡 **CLAVES DEL ÉXITO**

### **1. Rybot es el Guardián**
```rust
// Rybot monitorea qué módulos se cargan
// Rybot detecta imports no usados
// Rybot alerta antes de que fallen tests gráficos
```

### **2. Lizer Solo Organiza**
```rust
// Lizer NO ejecuta
// Lizer NO carga imports pesados
// Lizer solo tokeniza y parsea (ligero)
```

### **3. Blast-Core Ejecuta**
```rust
// Blast-core ES el executor
// Blast-core carga módulos bajo demanda
// Blast-core reporta a Rybot
```

### **4. RyditModule es el Puente**
```rust
// RyditModule conecta Lizer + Blast-core
// RyditModule auto-carga módulos
// RyditModule es ligero (trait + registry)
```

---

## 🎯 **CRITERIOS DE ÉXITO**

### **Nivel 1 (Núcleo)**
- ✅ 30 tests passing
- ✅ < 1 segundo total
- ✅ 0 dependencias gráficas
- ✅ CI/CD compatible

### **Nivel 2 (Integración)**
- ✅ 15 tests passing
- ✅ < 5 segundos total
- ✅ 0 ventanas gráficas
- ✅ Rybot reporta estado

### **Nivel 3 (Gráficos)**
- ✅ 3 tests passing
- ✅ < 10 segundos total
- ✅ Termux-X11 funciona
- ✅ 0 fallos "misteriosos"

---

## 📝 **PRÓXIMOS PASOS INMEDIATOS**

### **HOY**
1. ✅ Eliminar 48 binarios rotos/legacy
2. ✅ Crear `crates/rydit-test/tests/`
3. ✅ Tests Nivel 1 (lizer, blast, ryditmodule)
4. ✅ Verificar que pasan en < 3 segundos

### **MAÑANA**
1. 🔮 Tests Nivel 2 (rybot, evaluator, modules)
2. 🔮 Rybot monitorea carga de módulos
3. 🔮 Auto-detección de imports no usados

### **ESTA SEMANA**
1. 🔮 Solo 3-6 binarios gráficos
2. 🔮 Demos reales (snake, platformer, particles)
3. 🔮 Documentar flujo seguro

---

<div align="center">

**🛡️ RyDit v0.11.1 - Tests en 3 Niveles**

*54 → 12 binarios ✅ | Tests seguros ✅ | Rybot guardián ✅*

**Próximo: Implementar Nivel 1 (Núcleo)**

</div>
