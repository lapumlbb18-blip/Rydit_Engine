# 🛡️ TABLA DE REFERENCIA - 76 ERRORES

**Usar como checklist durante el fix manual**

---

## 📊 RESUMEN POR TIPO

| Tipo | Código | Cantidad | Fix | Fase |
|------|--------|----------|-----|------|
| Type Mismatch | E0308 | 38 | `.to_string()` o conversión | 1-2 |
| Variable no encontrada | E0425 | 6 | Cambiar `name` ↔ `func_name` | 1 |
| Trait no implementado | E0277 | 13 | `*` o `&` para dereferenciar | 1 |
| Pattern incorrecto | E0026, E0027 | 2 | Usar `callee` en vez de `name` | 3 |
| Tipo no declarado | E0433 | 4 | Agregar `use Lizer` | 1 |
| Método no encontrado | E0599 | 3 | `.expect().into_string()` | 2 |
| Feature inestable | E0658 | 3 | Quitar `.as_str()` | 1 |

---

## 📝 LISTA COMPLETA DE ERRORES

### main.rs (58 errores)

| # | Línea | Tipo | Fix | Fase | Estado |
|---|-------|------|-----|------|--------|
| 1 | 309 | E0425 | `name` → `func_name` | 1 | ⬜ |
| 2 | 1739 | E0425 | `name` → `func_name` | 1 | ⬜ |
| 3 | 2187 | E0425 | `func_name` → `name` | 1 | ⬜ |
| 4 | 3057 | E0425 | `name` → `func_name` | 1 | ⬜ |
| 5 | 3168 | E0425 | `func_name` → `name` | 1 | ⬜ |
| 6 | 4462 | E0425 | `func_name` → `name` | 3 | ⬜ |
| 7 | 244 | E0308 | `.to_string()` en name | 2 | ⬜ |
| 8 | 244 | E0308 | `.iter().map().collect()` en params | 2 | ⬜ |
| 9 | 248 | E0277 | `callee` directo (es &str) | 3 | ⬜ |
| 10 | 252 | E0308 | Consecuencia de #9 | 3 | ⬜ |
| 11 | 334 | E0308 | `.to_string()` en name | 2 | ⬜ |
| 12 | 345 | E0658 | Quitar `.as_str()` | 1 | ⬜ |
| 13 | 392 | E0308 | `.to_string()` en name | 2 | ⬜ |
| 14 | 395 | E0433 | Agregar `use Lizer` | 1 | ⬜ |
| 15 | 399 | E0308 | Tupla `(program, errors)` | 2 | ⬜ |
| 16 | 400 | E0308 | Tupla `(program, errors)` | 2 | ⬜ |
| 17 | 411 | E0308 | `.to_string()` en name | 2 | ⬜ |
| 18 | 434 | E0308 | `.to_string()` en name | 2 | ⬜ |
| 19 | 1268 | E0277 | `*name ==` en vez de `name ==` | 1 | ⬜ |
| 20 | 1269 | E0277 | `*name ==` | 1 | ⬜ |
| 21 | 1270 | E0277 | `*name ==` | 1 | ⬜ |
| 22 | 1271 | E0277 | `*name ==` | 1 | ⬜ |
| 23 | 1272 | E0277 | `*name ==` | 1 | ⬜ |
| 24 | 1273 | E0277 | `*name ==` | 1 | ⬜ |
| 25 | 1274 | E0277 | `*name ==` | 1 | ⬜ |
| 26 | 1275 | E0277 | `*name ==` | 1 | ⬜ |
| 27 | 1276 | E0277 | `*name ==` | 1 | ⬜ |
| 28 | 1277 | E0277 | `*name ==` | 1 | ⬜ |
| 29 | 1278 | E0277 | `*name ==` | 1 | ⬜ |
| 30 | 1448 | E0308 | `.to_string()` en name | 2 | ⬜ |
| 31 | 1448 | E0308 | `.iter().map().collect()` en params | 2 | ⬜ |
| 32 | 1451 | E0277 | `callee` directo | 3 | ⬜ |
| 33 | 1454 | E0308 | Consecuencia de #32 | 3 | ⬜ |
| 34 | 1619 | E0308 | Fix AST particles | 2 | ⬜ |
| 35 | 1647 | E0308 | Fix AST particles | 2 | ⬜ |
| 36 | 1647 | E0308 | Fix AST particles | 2 | ⬜ |
| 37 | 1667 | E0308 | Fix AST particles | 2 | ⬜ |
| 38 | 1778 | E0308 | `.to_string()` en module | 2 | ⬜ |
| 39 | 1789 | E0658 | Quitar `.as_str()` | 1 | ⬜ |
| 40 | 1830 | E0308 | `.to_string()` en name | 2 | ⬜ |
| 41 | 1833 | E0433 | Agregar `use Lizer` | 1 | ⬜ |
| 42 | 1837 | E0308 | Tupla `(program, errors)` | 2 | ⬜ |
| 43 | 1838 | E0308 | Tupla `(program, errors)` | 2 | ⬜ |
| 44 | 1849 | E0308 | `.to_string()` en name | 2 | ⬜ |
| 45 | 1869 | E0308 | `.to_string()` en name | 2 | ⬜ |
| 46 | 2185 | E0308 | Fix AST Texto | 3 | ⬜ |
| 47 | 3166 | E0308 | Fix AST Texto | 3 | ⬜ |
| 48 | 4008 | E0308 | Fix if/else types | 2 | ⬜ |
| 49 | 4075 | E0599 | `Binary` en vez de `BinOp` | 3 | ⬜ |
| 50 | 4457 | E0308 | `.to_string()` en name | 2 | ⬜ |
| 51 | 4457 | E0308 | `.iter().map().collect()` en params | 2 | ⬜ |
| 52 | 4459 | E0026 | `callee` en vez de `name` | 3 | ⬜ |
| 53 | 4459 | E0027 | Agregar campo `callee` | 3 | ⬜ |
| 54 | 4478 | E0308 | Consecuencia de #52-53 | 3 | ⬜ |
| 55 | 4483 | E0658 | Quitar `.as_str()` | 1 | ⬜ |
| 56 | 4508 | E0308 | `.to_string()` en name | 2 | ⬜ |
| 57 | 4510 | E0433 | Agregar `use Lizer` | 1 | ⬜ |
| 58 | 4514 | E0308 | Tupla `(program, errors)` | 2 | ⬜ |
| 59 | 4515 | E0308 | Tupla `(program, errors)` | 2 | ⬜ |
| 60 | 4525 | E0308 | `.to_string()` en name | 2 | ⬜ |
| 61 | 4544 | E0308 | `.to_string()` en name | 2 | ⬜ |

---

### eval/mod.rs (11 errores)

| # | Línea | Tipo | Fix | Fase | Estado |
|---|-------|------|-----|------|--------|
| 62 | 54 | E0308 | `s.to_string()` en vez de `s.clone()` | 3 | ⬜ |
| 63 | 57 | E0277 | `*name ==` en vez de `name ==` | 1 | ⬜ |
| 64 | 908 | E0277 | `func_name` en vez de `&func_name` | 2 | ⬜ |
| 65 | 912 | E0308 | Fix if/else types | 2 | ⬜ |
| 66 | 1399 | E0599 | `.expect().into_string()` | 2 | ⬜ |
| 67 | 1423 | E0308 | Fix HTTP POST unwrap | 2 | ⬜ |
| 68 | 1425 | E0308 | `e.to_string()` en Err | 2 | ⬜ |
| 69 | 1445 | E0308 | Fix HTTP PUT unwrap | 2 | ⬜ |
| 70 | 1447 | E0308 | `e.to_string()` en Err | 2 | ⬜ |
| 71 | 1459 | E0599 | `.expect().into_string()` | 2 | ⬜ |

---

### repl.rs (3 errores)

| # | Línea | Tipo | Fix | Fase | Estado |
|---|-------|------|-----|------|--------|
| 72 | 68 | E0433 | Agregar `use Lizer` | 1 | ⬜ |
| 73 | 72 | E0308 | Tupla `(program, errors)` | 2 | ⬜ |
| 74 | 85 | E0308 | Tupla `(program, errors)` | 2 | ⬜ |

---

### executor.rs (2 errores)

| # | Línea | Tipo | Fix | Fase | Estado |
|---|-------|------|-----|------|--------|
| 75 | 424 | E0308 | `.to_string()` en name | 2 | ⬜ |
| 76 | 424 | E0308 | `.iter().map().collect()` en params | 2 | ⬜ |

---

### cli.rs (1 warning como error)

| # | Línea | Tipo | Fix | Fase | Estado |
|---|-------|------|-----|------|--------|
| 77 | 83 | Warning | Revisar warning específico | 1 | ⬜ |

---

### modules/level.rs (3 warnings como errores)

| # | Línea | Tipo | Fix | Fase | Estado |
|---|-------|------|-----|------|--------|
| 78 | 247 | Warning | Revisar warning específico | 1 | ⬜ |
| 79 | 249 | Warning | Revisar warning específico | 1 | ⬜ |
| 80 | 250 | Warning | Revisar warning específico | 1 | ⬜ |

---

## ✅ CHECKLIST DE PROGRESO

### FASE 1 (🟢 Bajo) - 30 minutos

- [ ] Error #1 (main.rs:309)
- [ ] Error #2 (main.rs:1739)
- [ ] Error #3 (main.rs:2187)
- [ ] Error #4 (main.rs:3057)
- [ ] Error #5 (main.rs:3168)
- [ ] Error #6 (main.rs:4462)
- [ ] Error #14 (main.rs:395) - Import Lizer
- [ ] Error #19-28 (main.rs:1268-1278) - 10 comparaciones
- [ ] Error #39 (main.rs:1789) - as_str()
- [ ] Error #55 (main.rs:4483) - as_str()
- [ ] Error #63 (eval/mod.rs:57) - &str comparison
- [ ] Error #72 (repl.rs:68) - Import Lizer

**Total FASE 1**: 20 errores ✅

---

### FASE 2 (🟡 Medio) - 1.5 horas

- [ ] Error #7-8 (main.rs:244) - Function registration
- [ ] Error #11 (main.rs:334) - .to_string()
- [ ] Error #13 (main.rs:392) - .to_string()
- [ ] Error #15-16 (main.rs:399-400) - parser.parse()
- [ ] Error #17 (main.rs:411) - .to_string()
- [ ] Error #18 (main.rs:434) - .to_string()
- [ ] Error #30-31 (main.rs:1448) - Function registration
- [ ] Error #38 (main.rs:1778) - .to_string()
- [ ] Error #40 (main.rs:1830) - .to_string()
- [ ] Error #42-43 (main.rs:1837-1838) - parser.parse()
- [ ] Error #44 (main.rs:1849) - .to_string()
- [ ] Error #45 (main.rs:1869) - .to_string()
- [ ] Error #48 (main.rs:4008) - if/else types
- [ ] Error #50-51 (main.rs:4457) - Function registration
- [ ] Error #56 (main.rs:4508) - .to_string()
- [ ] Error #58-59 (main.rs:4514-4515) - parser.parse()
- [ ] Error #60 (main.rs:4525) - .to_string()
- [ ] Error #61 (main.rs:4544) - .to_string()
- [ ] Error #64-65 (eval/mod.rs:908,912) - HashMap types
- [ ] Error #66-71 (eval/mod.rs:1399-1459) - HTTP functions
- [ ] Error #73-74 (repl.rs:72,85) - parser.parse()
- [ ] Error #75-76 (executor.rs:424) - Function registration

**Total FASE 2**: 32 errores ✅

---

### FASE 3 (🔴 Alto) - 2 horas

- [ ] Error #9-10 (main.rs:248,252) - Stmt::Call pattern
- [ ] Error #32-33 (main.rs:1451,1454) - Stmt::Call pattern
- [ ] Error #34-37 (main.rs:1619-1667) - AST particles
- [ ] Error #46 (main.rs:2185) - AST Texto
- [ ] Error #47 (main.rs:3166) - AST Texto
- [ ] Error #49 (main.rs:4075) - Binary en vez de BinOp
- [ ] Error #52-54 (main.rs:4459-4478) - Stmt::Call pattern
- [ ] Error #62 (eval/mod.rs:54) - Texto type

**Total FASE 3**: 8 errores ✅

---

## 📊 PROGRESO ESPERADO

```
Inicio:        76 errores [████████████████████] 0%
Después FASE 1: 30 errores [████████▒▒▒▒▒▒▒▒▒▒▒▒] 60%
Después FASE 2: 10 errores [███▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒] 87%
Después FASE 3:  0 errores [                    ] 100% ✅
```

---

## 🎯 PRIORIDAD DE FIX

### Orden recomendado:

1. **Primero**: Errors #14, #72 (imports Lizer) - 5 min
   - Da confianza, desaparecen 4 errores de golpe

2. **Segundo**: Errors #19-28 (comparaciones &str) - 10 min
   - Mecánico, 10 errores desaparecen rápido

3. **Tercero**: Errors #1-6 (name/func_name) - 10 min
   - Simple, desaparecen 6 errores

4. **Cuarto**: Errors #39, #55 (as_str) - 5 min
   - Quitar 2 líneas de código

5. **Quinto**: Errors #11, #13, #17, #18, etc. (.to_string()) - 30 min
   - Todos los type mismatches simples

6. **Sexto**: Errors #15-16, #42-43, #58-59, #73-74 (parser.parse) - 30 min
   - Cambiar API del parser

7. **Séptimo**: Errors #66-71 (HTTP) - 30 min
   - Fixear ureq API

8. **Octavo**: Errors #7-8, #30-31, #50-51, #75-76 (function reg) - 30 min
   - HashMap types

9. **Noveno**: Errors #9-10, #32-33, #52-54 (Stmt::Call) - 1.5 horas
   - **CRÍTICO**: Entender bien el AST nuevo

10. **Décimo**: Errors #34-37, #46-47, #49, #62 (AST varios) - 30 min
    - Fixes finales de AST

---

## 🔥 FIXES CLAVE (MEMORIZAR)

### 1. Stmt::Call pattern
```rust
// CAMBIAR
Stmt::Call { callee, args } => {
    let func_name = if let Expr::Var(name) = callee.as_ref() {

// POR
Stmt::Call { callee, args } => {
    let func_name = callee;  // ¡DIRECTO!
```

### 2. parser.parse()
```rust
// CAMBIAR
match parser.parse() {
    Ok(p) => p,
    Err(e) => { ... }

// POR
let (program, errors) = parser.parse();
if !errors.is_empty() {
    for e in &errors { println!("{}", e); }
}
// usar program
```

### 3. Function registration
```rust
// CAMBIAR
funcs.insert(name.clone(), (params.clone(), body.clone()));

// POR
funcs.insert(
    name.to_string(),
    (params.iter().map(|s| s.to_string()).collect(), body.clone())
);
```

### 4. HTTP functions
```rust
// CAMBIAR
ureq::get(&url).call().into_string()

// POR
ureq::get(&url).call().expect("msg").into_string()
```

---

<div align="center">

**🛡️ RyDit v0.11.2 - TABLA DE REFERENCIA**

*76 errores | 3 fases | Checklist completo*

**Marcar ✅ después de cada fix**

</div>
