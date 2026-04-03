# 🛡️ RyDit v0.11.4 - ESTADO DE FIXES PENDIENTES

**Fecha**: 2026-04-02  
**Versión**: v0.11.4 ✅ 90% INTEGRACIÓN  
**Errores Pendientes**: ~96  
**Estado**: 90% completado

---

## 📊 **RESUMEN EJECUTIVO**

| Componente | Estado | Errores | Acción |
|------------|--------|---------|--------|
| **rydit-lexer** | ✅ 100% | 0 | - |
| **rydit-parser** | ✅ 100% | 0 | - |
| **rydit-vm** | ✅ 100% | 0 | - |
| **rydit-stream** | ✅ 100% | 0 | - |
| **FSR 1.0** | ✅ 100% | 0 | - |
| **Operadores +=** | ✅ 100% | 0 | - |
| **rydit-rs bin** | ⚠️ 90% | ~96 | Fix manual |

---

## 🔍 **DETALLE DE ERRORES PENDIENTES**

### **Categoría 1: name/func_name scope incorrecto** (~80 errores)

**Causa**: `sed` reemplazó `name` por `func_name` indiscriminadamente

**Archivos afectados**:
- `crates/rydit-rs/src/eval/mod.rs` (~60 errores)
- `crates/rydit-rs/src/main.rs` (~20 errores)

**Ejemplo de error**:
```rust
// ❌ INCORRECTO (sed muy agresivo)
Expr::Var(name) => {
    if func_name == "__INPUT__" {  // ← name fue cambiado a func_name
        return executor.input("> ");
    }
}

// ✅ CORRECTO
Expr::Var(name) => {
    if name == "__INPUT__" {  // ← name es correcto aquí
        return executor.input("> ");
    }
}
```

**Solución**:
1. Identificar contexto: `Expr::Var(name)` → NO cambiar
2. Identificar contexto: `if name == "func"` → CAMBIAR a `func_name`
3. Fix manual línea por línea

**Líneas afectadas (ejemplos)**:
- `eval/mod.rs:54, 57, 162, 175, 184, 193, 202...`
- `main.rs:2206, 3209...`

---

### **Categoría 2: WebSocket functions (TODO)** (~10 errores)

**Causa**: `rydit-stream` no tiene funciones HTTP/WebSocket (solo streaming LAN)

**Funciones comentadas**:
- `ws::connect(url)` - Conectar WebSocket
- `ws::disconnect()` - Desconectar
- `ws::send(msg)` - Enviar mensaje
- `ws::recv()` - Recibir mensaje
- `ws::is_connected()` - Verificar estado
- `ws::get_url()` - Obtener URL

**Solución**:
- **Opción A**: Implementar con `tungstenite` directo en `eval/mod.rs`
- **Opción B**: Mover a crate separado `rydit-websocket`
- **Opción C**: Mantener comentado (no crítico para v0.11.4)

**Recomendación**: **Opción C** (mantener comentado para v0.11.4)

---

### **Categoría 3: ureq API** (~6 errores)

**Causa**: API de `ureq` cambió (requiere `.call()` y `.send_string()`)

**Fix aplicado**:
```rust
// ❌ ANTES
ureq::get(&url)
ureq::post(&url, &data)

// ✅ DESPUÉS
ureq::get(&url).call()
ureq::post(&url).send_string(&data)
```

**Estado**: ✅ FIX APLICADO

---

## 🛠️ **PLAN DE FIX MANUAL**

### **Paso 1: Revertir cambios problemáticos (opcional)**

```bash
# Opción: Revertir eval/mod.rs a estado conocido
git checkout v0.11.4-fase2b-main -- crates/rydit-rs/src/eval/mod.rs
```

### **Paso 2: Fix manual name → func_name**

**Reglas**:
1. ✅ CAMBIAR `name` → `func_name` en:
   - `if name == "func"` → `if func_name == "func"`
   - `println!("...", name)` → `println!("...", func_name)`

2. ❌ NO CAMBIAR `name` en:
   - `Expr::Var(name)` - pattern matching
   - `fn foo(name: &str)` - parámetros de función
   - `let name = ...` - variables locales

**Herramienta recomendada**: VS Code + búsqueda manual

### **Paso 3: Verificar compilación**

```bash
cargo build -p rydit-rs --bin rydit-rs
cargo test -p rydit-lexer -p rydit-parser -p rydit-vm -p rydit-stream
```

---

## 📋 **CHECKLIST DE FIX**

- [ ] **eval/mod.rs** (~60 errores)
  - [ ] Líneas 50-70: `Expr::Var(name)` - NO cambiar
  - [ ] Líneas 160-200: `if name == "func"` → `func_name`
  - [ ] Líneas 1400-1500: WebSocket - mantener comentado
  - [ ] Líneas 1400-1460: ureq - ✅ ya fixeado

- [ ] **main.rs** (~20 errores)
  - [ ] Líneas 2200-2300: `evaluar_expr_gfx()` - fix name/func_name
  - [ ] Líneas 3200-3300: `evaluar_expr_migui()` - fix name/func_name

- [ ] **repl.rs** (~5 errores)
  - [ ] Líneas 68-85: fix name/func_name

- [ ] **modules/entity.rs** (~5 errores)
  - [ ] Líneas 13-14: imports

- [ ] **modules/level.rs** (~5 errores)
  - [ ] Líneas 12: imports

- [ ] **rybot/mod.rs** (~5 errores)
  - [ ] Líneas 13: imports

---

## ⏱️ **TIEMPO ESTIMADO**

| Tarea | Tiempo | Dificultad |
|-------|--------|------------|
| Fix eval/mod.rs | 1-2 horas | Media |
| Fix main.rs | 30 min | Baja |
| Fix otros archivos | 30 min | Baja |
| Tests + verify | 30 min | Baja |
| **TOTAL** | **2-3 horas** | **Media** |

---

## 🎯 **CRITERIOS DE ÉXITO**

- [ ] `cargo build -p rydit-rs --bin rydit-rs` ✅ 0 errores
- [ ] `cargo test --workspace` ✅ 85+ tests passing
- [ ] `cargo run --bin demo_stream` ✅ Funciona
- [ ] `cargo run --bin snake` ✅ Funciona

---

## 📝 **NOTAS IMPORTANTES**

1. **NO usar `sed` para reemplazos masivos** - Muy propenso a errores
2. **Usar VS Code o rust-analyzer** - Auto-complete ayuda
3. **Commit después de cada fix** - Puntos de reversión claros
4. **Testear después de cada archivo** - Evita errores en cascada

---

## 🚀 **PRÓXIMOS PASOS**

1. ✅ Fix manual name → func_name (2-3 horas)
2. ✅ Verificar compilación
3. ✅ Tests de integración
4. ✅ Tag v0.11.4-final
5. 🔮 Próxima feature: RyBot Cache (v0.11.5)

---

<div align="center">

**🛡️ RyDit v0.11.4 - 90% COMPLETADO**

*83 tests passing ✅ | ~96 errores pendientes | 2-3 horas para 100%*

**Próximo: Fix manual → v0.11.4 FINAL**

</div>
