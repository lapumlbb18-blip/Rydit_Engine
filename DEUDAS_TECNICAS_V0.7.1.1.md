# 📝 DEUDAS TÉCNICAS - RyDit v0.7.1.1

**Fecha**: 2026-03-24
**Versión**: v0.7.1.1

---

## 🔴 DEUDAS CRÍTICAS

### **1. Parser - Expresiones con `+` no soportado**

**Problema**: El parser no reconoce el operador `+` para:
- Suma aritmética: `x + 1` ❌
- Concatenación strings: `"hola" + "mundo"` ❌

**Síntoma**:
```
Error: Expresión no válida: Mas
Ubicación: línea X, columna Y
```

**Workaround actual**:
```rydit
# En lugar de:
dark.slot x = frame + 100
dark.slot texto = "hola" + "mundo"

# Usar:
dark.slot x = math::sumar(frame, 100)
dark.slot texto = strings::concat("hola", "mundo")
```

**Solución requerida**:
- Agregar `+` al parser de expresiones (lizer/src/lib.rs)
- Soportar sobrecarga: suma para números, concat para strings
- Prioridad: ALTA (afecta usabilidad)

**Archivos a modificar**:
- `crates/lizer/src/lib.rs` - Parser de expresiones binarias
- `crates/rydit-rs/src/eval/mod.rs` - Manejo del operador `+`

---

### **2. Demo no se mantiene abierto sin timeout**

**Problema**: Los demos `.rydit` con `--gfx` se cierran inmediatamente después de cargar.

**Síntoma**:
```bash
# No funciona (se cierra):
./target/release/rydit-rs --gfx demo.rydit

# Funciona (con timeout):
timeout 20 ./target/release/rydit-rs --gfx demo.rydit
```

**Causa probable**:
- Game loop no se mantiene cuando no hay input
- Falta `while !gfx.should_close()` en algún lado
- Problema con detección de cierre de ventana

**Workaround actual**:
- Usar `timeout` o ejecutar en background con `&`
- Scripts `.sh` que configuran entorno y ejecutan

**Solución requerida**:
- Revisar game loop en `crates/rydit-rs/src/main.rs`
- Verificar `ejecutar_programa_gfx()`
- Asegurar que el loop se mantenga hasta ESC o cerrar ventana

**Archivos a modificar**:
- `crates/rydit-rs/src/main.rs` - Game loop
- `crates/rydit-rs/src/executor.rs` - `ejecutar_programa_gfx()`

---

### **3. Termux-X11 requiere configuración manual de ENV**

**Problema**: Cada vez que se ejecuta un demo gráfico, hay que setear:
```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1
```

**Síntoma**: Sin las variables, error:
```
GLFW: Failed to initialize GLFW
```

**Workaround actual**:
- Scripts `.sh` con las variables
- `configurar_entorno_termux()` en main.rs (pero no siempre funciona)

**Solución requerida**:
- Auto-detect y configuración en `rydit-gfx`
- Intentar configurar antes de fallar
- Mensaje de error útil si no se puede configurar

**Archivos a modificar**:
- `crates/rydit-gfx/src/lib.rs` - `RyditGfx::new()`
- `crates/rydit-rs/src/config.rs` - Mejorar `configurar_entorno_termux()`

---

## 🟡 DEUDAS MEDIAS

### **4. RyditModule trait no se usa en producción**

**Problema**: El trait está implementado (`module.rs`) pero las funciones de animación se agregaron directamente en `eval/mod.rs`.

**Estado actual**:
```rust
// module.rs existe con RyditModule trait ✅
// Pero animación está en eval/mod.rs ❌
```

**Razón**: `rydit-rs` es binario, no librería → no se puede importar desde crates externos.

**Solución futura**:
- Convertir `rydit-rs` en librería + binario
- Mover `eval/mod.rs` a crate separado (`rydit-core`?)
- Usar RyditModule para módulos externos (anim, physics, data)

**Archivos involucrados**:
- `crates/rydit-rs/Cargo.toml` - Agregar `[lib]`
- `crates/rydit-rs/src/lib.rs` - Crear librería
- `crates/rydit-rs/src/main.rs` - Solo binario

---

### **5. Funciones de animación sin tests**

**Problema**: Las 10 funciones de animación agregadas no tienen tests unitarios.

**Funciones sin tests**:
- `anim::ease_in`, `ease_out`, `ease_in_out`
- `anim::squash`, `stretch`
- `anim::anticipate`
- `illusion::muller_lyer`, `ponzo`, `phi_effect`, `fraser_spiral`

**Riesgo**: Regresiones futuras no detectadas.

**Solución**:
- Agregar tests en `crates/rydit-rs/src/eval/mod.rs` (módulo `tests`)
- O crear `crates/rydit-rs/tests/anim_tests.rs`

---

### **6. Documentación de animación incompleta**

**Problema**: No hay documentación pública de las funciones de animación.

**Falta**:
- README en `demos/` con ejemplos
- Comentarios en código explicando cada principio
- Referencia a los 12 principios de Disney

**Solución**:
- `demos/README.md` con ejemplos de cada función
- Actualizar `README.md` principal con sección de animación

---

## 🟢 DEUDAS MENORES

### **7. Whitespace en eval/mod.rs**

**Problema**: Después de agregar animación, el archivo tiene 1,063 líneas con código duplicado.

**Solución**:
- Extraer funciones de animación a `eval/anim.rs`
- Usar `cargo fmt` para consistencia

---

### **8. Demo de ilusiones muy básico**

**Problema**: `demo_ilusiones_minimo.rydit` no usa todas las funciones de animación.

**Mejoras posibles**:
- Usar `anim::ease_in_out()` para transiciones suaves
- Usar `anim::squash()` para rebote de pelota
- Agregar más ilusiones (Ebbinghaus, Hering, etc.)

---

## 📋 PRIORIDADES

| # | Deuda | Prioridad | Esfuerzo | Impacto |
|---|-------|-----------|----------|---------|
| 1 | Parser `+` | 🔴 ALTA | 2-3h | Alto |
| 2 | Game loop cierre | 🔴 ALTA | 1-2h | Alto |
| 3 | Auto-config X11 | 🟡 MEDIA | 1h | Medio |
| 4 | RyditModule producción | 🟡 MEDIA | 4-6h | Alto |
| 5 | Tests animación | 🟡 MEDIA | 2h | Bajo |
| 6 | Docs animación | 🟢 BAJA | 1h | Bajo |
| 7 | Whitespace eval | 🟢 BAJA | 30min | Mínimo |
| 8 | Demo mejorado | 🟢 BAJA | 2h | Medio |

---

## 🎯 PRÓXIMA SESIÓN

**Recomendado**: Fix #1 (Parser `+`) + #2 (Game loop)

**Beneficio**: Mejora inmediata de usabilidad para usuarios.

---

<div align="center">

**🛡️ RyDit v0.7.1.1 - Deudas Técnicas Documentadas**

*8 deudas identificadas | 3 críticas | 3 medias | 2 menores*

</div>
