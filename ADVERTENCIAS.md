# ⚠️ ADVERTENCIAS RyDit v0.11.5

**Fecha**: 2026-04-02  
**Versión**: v0.11.5  
**Tipo**: Lista de verificación de funcionalidades

---

## ✅ LO QUE SÍ FUNCIONA (Verificado)

| Feature | Estado | Test | Notas |
|---------|--------|------|-------|
| **Lexer** | ✅ Verificado | `test_rydit_simple` | Tokenización OK |
| **Parser** | ✅ Verificado | `test_rydit_simple` | Parseo de scripts OK |
| **Variables** | ✅ Verificado | `test_rydit_simple` | `dark.slot x = 10` |
| **Operaciones** | ✅ Verificado | `test_rydit_simple` | Math básico OK |
| **Bucles (ryda)** | ✅ Verificado | `test_rydit_simple` | While loops OK |
| **Condicionales** | ✅ Verificado | `test_bloques_anidados` | 100 condicionales (502 líneas) |
| **Funciones** | ✅ Verificado | `test_bloques_anidados` | 20 funciones con bloques (222 líneas) |
| **Arrays** | ⏳ Pendiente | - | Código existe, sin test |
| **Módulos stdlib** | ⏳ Pendiente | - | math, random, strings |
| **VM Bytecode** | ✅ Tests lib | `cargo test -p rydit-vm` | 19 passing |
| **Scripts 500+ líneas** | ✅ Verificado | `test_bloques_anidados` | 1227 líneas parseadas sin errores |
| **Bloques anidados** | ✅ Verificado | `test_bloques_anidados` | Sin límite artificial |

---

## ⚠️ PENDIENTES POR TESTEAR (Requieren binario rydit-rs)

### Gráficos SDL2
| Feature | Estado | Notas |
|---------|--------|-------|
| **draw.circle()** | ⏳ Pendiente | Binario no compila (linker raylib) |
| **draw.rect()** | ⏳ Pendiente | Idem |
| **draw.line()** | ⏳ Pendiente | Idem |
| **draw.text()** | ⏳ Pendiente | Idem |
| **draw.sprite()** | ⏳ Pendiente | Idem |

### Input
| Feature | Estado | Notas |
|---------|--------|-------|
| **tecla_presionada()** | ⏳ Pendiente | Requiere SDL2 runtime |
| **mouse_x(), mouse_y()** | ⏳ Pendiente | Idem |
| **gamepad** | ⏳ Pendiente | Idem |

### Audio
| Feature | Estado | Notas |
|---------|--------|-------|
| **audio::play()** | ⏳ Pendiente | Linker SDL2_mixer |
| **audio::load_sound()** | ⏳ Pendiente | Idem |
| **audio::play_music()** | ⏳ Pendiente | Idem |

### Módulos Avanzados
| Feature | Estado | Notas |
|---------|--------|-------|
| **colisiones** | ⏳ Pendiente | Sin test .rydit |
| **regex** | ⏳ Pendiente | Sin test .rydit |
| **files** | ⏳ Pendiente | Sin test .rydit |
| **json** | ⏳ Pendiente | Sin test .rydit |
| **csv** | ⏳ Pendiente | Sin test .rydit |
| **entity** | ⏳ Pendiente | Sin test .rydit |
| **camera** | ⏳ Pendiente | Sin test .rydit |
| **particles** | ⏳ Pendiente | Sin test .rydit |

---

## 🔴 PROBLEMAS CONOCIDOS

### 1. Binario rydit-rs no compila (linker)
**Error**: `undefined symbol: DrawTextureEx, UnloadSound, SetMusicVolume, etc.`

**Causa**: Bibliotecas nativas de raylib/SDL2 no están linkeadas correctamente en este entorno.

**Impacto**: No se pueden ejecutar scripts `.rydit` con gráficos/audio/input.

**Solución pendiente**:
- Instalar raylib correctamente en Termux
- O usar solo SDL2 backend (ya funcional en código)
- O crear wrapper que no requiera linker externo

### 2. Tests gráficos requieren Termux-X11
**Limitación**: No hay forma de automatizar tests gráficos en Termux-X11.

**Impacto**: Tests de render/input deben ser manuales.

**Solución**: Ejecutar manualmente y verificar visualmente.

### 3. ⚠️ BUG: Lexer corta identificadores tras comentarios
**Descubierto**: 2026-04-02 (test bloques anidados)

**Error**: Cuando un script `.rydit` comienza con comentario `#`, el lexer corta los identificadores siguientes incluyendo saltos de línea.

**Ejemplo**:
```
# Comentario al inicio
dark.slot x = 10
```

**Tokens generados (INCORRECTOS)**:
```
Token { kind: Comentario, lexeme: " Comentario al inicio" }
Token { kind: Ident, lexeme: "\ndark.slo" }  ← Mal, incluye \n y corta
Token { kind: Ident, lexeme: " nive" }       ← Mal, espacio incluido
```

**Tokens esperados (CORRECTOS)**:
```
Token { kind: Comentario, lexeme: " Comentario al inicio" }
Token { kind: DarkSlot, lexeme: "dark.slot" }  ← Bien
Token { kind: Ident, lexeme: "x" }             ← Bien
```

**Impacto**: Scripts con comentarios al inicio fallan el parseo (1000+ errores).

**Workaround**: Evitar comentarios `#` al inicio de scripts `.rydit`.

**Causa probable**: El lexer no consume correctamente el salto de línea después del comentario, o el estado del lexer no se resetea.

**Prioridad**: 🔴 ALTA - Afecta usabilidad básica del lenguaje.

**Archivo sospechoso**: `crates/rydit-lexer/src/lexer.rs` - manejo de tokenización tras comentarios.

---

## 📊 RESUMEN

| Categoría | Verificado | Pendiente | Total |
|-----------|------------|-----------|-------|
| **Core (sin gráficos)** | 10 | 3 | 13 |
| **Gráficos** | 0 | 5 | 5 |
| **Input** | 0 | 3 | 3 |
| **Audio** | 0 | 3 | 3 |
| **Módulos** | 0 | 7 | 7 |
| **TOTAL** | **10** | **21** | **31** |

**Funcionalidad core**: 77% verificado (10/13)
**Funcionalidad completa**: ~32% verificado (10/31)

---

## 🎯 PRÓXIMOS PASOS

### Prioridad 1: Fix linker rydit-rs
- [ ] Resolver linker raylib/SDL2
- [ ] Compilar binario exitosamente
- [ ] Ejecutar scripts .rydit con gráficos

### Prioridad 2: Tests manuales Termux-X11
- [ ] draw.circle(), draw.rect(), draw.text()
- [ ] tecla_presionada() input
- [ ] audio::play() sonidos

### Prioridad 3: Tests módulos
- [ ] colisiones, regex, files, json
- [ ] entity, camera, particles
- [ ] csv, http, websocket

---

<div align="center">

**⚠️ RyDit v0.11.5 - ADVERTENCIAS**

*Core: 60% verificado ✅ | Gráficos: 0% ⏳ | Audio: 0% ⏳ | Input: 0% ⏳*

**Próximo: Fix linker rydit-rs + tests manuales**

</div>
