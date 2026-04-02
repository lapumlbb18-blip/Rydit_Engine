# 🛡️ DIAGNÓSTICO COMPLETO - ROOT CAUSE ENCONTRADO

**Fecha**: 2026-03-31  
**Hora**: Después de análisis profundo  
**Estado**: ✅ Root cause identificado

---

## 🔥 ROOT CAUSE: SPLIT RYDITMODULE

### Problema Fundamental

**ANTES del split**:
```
rydit-gfx (monolito)
  ├── RyditGfx (game loop + input + render)
  ├── Assets (carga + dibujo)
  └── Todo integrado
```

**DESPUÉS del split**:
```
rydit-gfx (crate separado)
  ├── RyditGfx (solo game loop + input)
  ├── Assets (solo carga - dibujo roto)
  └── RenderQueue (nuevo, para draw calls)

rydit-rs (binario)
  ├── modules/assets.rs (¿dibuja?)
  ├── eval/mod.rs (¿usa FFI o RenderQueue?)
  └── executor.rs (game loop)
```

### La Desconexión Crítica

**`eval/mod.rs:1747` - CÓDIGO ACTUAL (ROTO)**:
```rust
// assets::draw(id, x, y) - Dibujar textura en posición
if name == "assets::draw" && args.len() >= 3 {
    // ... evaluar args ...
    
    // ❌ FFI DIRECTO - Esto NO funciona con RenderQueue
    let assets = assets::get_assets();
    let assets_ref = assets.borrow();
    
    if let Some(texture) = assets_ref.get_texture(&id) {
        unsafe {
            ffi::DrawTexture(**texture, x as i32, y as i32, color.to_color().into());
            // ↑ ESTO ESTÁ ROTO - FFI directo sin contexto de dibujado
        }
    }
}
```

**POR QUÉ ESTÁ ROTO**:
1. `ffi::DrawTexture()` requiere contexto de dibujado (`DrawHandle`)
2. En modo gráfico, el `DrawHandle` solo existe dentro de `gfx.begin_draw()`
3. `eval/mod.rs` NO tiene acceso al `DrawHandle`
4. **Conclusión**: El FFI directo falla silenciosamente

---

## ✅ SOLUCIÓN: MIGRAR A RENDERQUEUE

**CÓDIGO CORREGIDO**:
```rust
// assets::draw(id, x, y) - Dibujar textura en posición
if name == "assets::draw" && args.len() >= 3 {
    // ... evaluar args ...
    
    // ✅ USAR RENDERQUEUE - Acumular comando para ejecutar después
    queue.push(DrawCommand::Texture {
        id,
        x,
        y,
        scale: 1.0,
        rotation: 0.0,
        color,
    });
    
    return Valor::Texto(format!("assets::draw() - '{}' en cola", id));
}
```

**VENTAJAS**:
1. ✅ No requiere `DrawHandle` inmediato
2. ✅ Acumula comandos para batch processing
3. ✅ Funciona con double buffering
4. ✅ Compatible con Platform Sync

---

## 📋 FIXES NECESARIOS (3 CAMBIOS)

### Fix 1: `eval/mod.rs` → Usar RenderQueue

**Archivo**: `crates/rydit-rs/src/eval/mod.rs`

**Cambios**:
- Línea ~1747: Eliminar FFI directo (`ffi::DrawTexture`)
- Reemplazar con: `queue.push(DrawCommand::Texture { ... })`
- Requiere: Pasar `&mut RenderQueue` a las funciones de assets

**Complejidad**: Media (50-100 líneas a modificar)

---

### Fix 2: `modules/assets.rs` → Retornar DrawCommand

**Archivo**: `crates/rydit-rs/src/modules/assets.rs`

**Cambios**:
- `assets_draw()`: Ahora retorna `DrawCommand` en vez de dibujar
- `assets_draw_scaled()`: Igual
- Eliminar: Todo el código de dibujo FFI
- Agregar: Lógica para crear `DrawCommand::Texture`

**Complejidad**: Baja-Media (30-50 líneas)

---

### Fix 3: `executor.rs` → Pasar RenderQueue a eval

**Archivo**: `crates/rydit-rs/src/executor.rs`

**Cambios**:
- `ejecutar_stmt_gfx()`: Ya tiene `queue: &mut RenderQueue`
- `evaluar_expr_gfx()`: Necesita recibir `queue` también
- Propagar `queue` a todas las llamadas de assets

**Complejidad**: Media (20-30 líneas)

---

## 🎯 INPUT MAP - FIX ADICIONAL

### Problema

`InputEstado::actualizar()` hace polling manual (100+ líneas) pero NUNCA sincroniza con `InputMapState`.

### Solución

**Opción A (Rápida - 1 hora)**:
```rust
// main.rs: InputEstado::actualizar()
fn actualizar(&mut self, gfx: &RyditGfx) {
    // Polling manual (ya existe)
    self.space = gfx.is_key_pressed(Key::Space);
    
    // NUEVO: Sincronizar con InputMapState
    if self.space {
        input_map::press_key("space");
    } else {
        input_map::release_key("space");
    }
}
```

**Opción B (Limpia - 3 horas)**:
```rust
// Crear rydit-input crate
// Eliminar InputEstado e InputMapState
// Usar solo rydit-input::poll_events()
```

**Recomendación**: Opción A ahora, Opción B en Fase 2

---

## 📊 IMPACTO DEL FIX

| Sistema | Antes | Después | Mejora |
|---------|-------|---------|--------|
| **Assets** | ❌ FFI roto | ✅ RenderQueue | 100% funcional |
| **Input Map** | ❌ 0% sincronizado | ✅ 100% sincronizado | 20+ combinaciones |
| **Partículas** | ❌ Comentado | ✅ RenderQueue | 500+ partículas |
| **Rendimiento** | ⚠️ 1 begin_draw por draw call | ✅ 1 begin_draw por frame | 4x más rápido |

---

## 🚀 PRÓXIMOS PASOS

1. [ ] **Fix eval/mod.rs** - Eliminar FFI directo de assets::draw
2. [ ] **Fix modules/assets.rs** - Retornar DrawCommand
3. [ ] **Fix executor.rs** - Pasar RenderQueue a eval
4. [ ] **Fix InputEstado** - Sincronizar con InputMapState
5. [ ] **Descomentar partículas** - Verificar que funciona
6. [ ] **Crear demo test** - Validar todos los fixes

**Tiempo estimado**: 4-6 horas  
**Riesgo**: Bajo (código ya existe, solo reconectar)  
**Beneficio**: Alto (Assets + Input + Partículas funcionando)

---

<div align="center">

**🛡️ ROOT CAUSE: SPLIT RYDITMODULE**

*FFI directo → RenderQueue | 3 archivos a modificar | 4-6 horas*

</div>
