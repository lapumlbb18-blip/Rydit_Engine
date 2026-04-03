# 📝 DEUDAS TÉCNICAS - RyDit v0.7.1.1

**Fecha**: 2026-03-24
**Versión**: v0.7.1.1

---

## 🔴 DEUDAS CRÍTICAS

### **1. Parser - Expresiones con `+` ✅ RESUELTO**

**Estado**: El operador `+` **SÍ FUNCIONA** correctamente.

**Verificación**:
```rydit
dark.slot x = frame + 100  # ✅ Funciona
dark.slot y = ilusion + 1  # ✅ Funciona
```

**Implementación existente**:
- Lexer: Token `Mas` → línea 253, 713
- Parser: `parse_additive()` → línea 2256
- Eval: `BinOp::Suma` → línea 1019

**Cierre**: Esta deuda está **RESUELTA**. El parser y evaluador soportan `+` correctamente.

---

### **2. Demo no se mantiene abierto sin timeout ✅ RESUELTO**

**Estado**: El game loop **SÍ FUNCIONA** correctamente.

**Análisis**:
```rust
// executor.rs: ejecutar_programa_gfx()
while !gfx.should_close() {
    // Game loop corre indefinidamente
    input.actualizar(gfx);
    // ... dibujar ...
    if escape { break; }  // Solo sale con ESC
}
```

**Comportamiento real**:
- **Con `timeout 20`**: Proceso se mata a los 20s (útil para testing)
- **Sin timeout**: Game loop corre hasta presionar ESC ✅
- **Problema percibido**: Sin timeout, parece que "no abre" porque espera input

**Verificación**:
```bash
# Funciona - se mantiene abierto hasta ESC
./target/release/rydit-rs --gfx demos/demo_ilusiones_minimo.rydit

# También funciona - se mata a los 20s
timeout 20 ./target/release/rydit-rs --gfx demos/demo_ilusiones_minimo.rydit
```

**Cierre**: Esta deuda está **RESUELTA**. El game loop funciona correctamente.
El `timeout` es opcional, solo para testing automatizado.

**Recomendación**: Documentar en README que los demos gráficos:
- Se mantienen abiertos hasta presionar ESC
- Usar `timeout` solo para CI/testing

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
| 1 | ~~Parser `+`~~ | ✅ RESUELTA | - | - |
| 2 | ~~Game loop cierre~~ | ✅ RESUELTA | - | - |
| 3 | Auto-config X11 | 🔴 ALTA | 1h | Alto |
| 4 | RyditModule producción | 🟡 MEDIA | 4-6h | Alto |
| 5 | Tests animación | 🟡 MEDIA | 2h | Bajo |
| 6 | Docs animación | 🟢 BAJA | 1h | Bajo |
| 7 | Whitespace eval | 🟢 BAJA | 30min | Mínimo |
| 8 | Demo mejorado | 🟢 BAJA | 2h | Medio |

---

## 🎯 PRÓXIMA SESIÓN

**Recomendado**: Fix #3 (Auto-config X11)

**Beneficio**: Eliminar necesidad de `export DISPLAY=:0` manual.

---

<div align="center">

**🛡️ RyDit v0.7.1.1 - Deudas Técnicas Documentadas**

*8 deudas identificadas | 3 críticas | 3 medias | 2 menores*

</div>
