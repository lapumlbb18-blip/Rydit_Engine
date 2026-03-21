## Qwen Added Memories

### v0.1.9 SESIÓN 26 COMPLETADA (2026-03-20) - CHECKPOINT 100 TESTS
- **110 Tests Pasando (Meta 100 Superada)**
- 30 tests nuevos agregados (80 → 110):
  1. lizer: +10 tests (precedencia, símbolos, expresiones)
  2. rydit-rs: +10 tests (concatenación, símbolos, executor)
  3. blast-core: +2 tests (scopes, memoria)
  4. rydit-gfx: +2 tests (draw, colores)
  5. v-shield: +2 tests (init, colores)
  6. doctests: +4 tests (documentación viva)
- Bug #1: Precedencia YA funcionaba correctamente (5 tests documentan)
- Bug #2: Concatenación string+número FIXEADA (coerción automática)
- 4 doctests en documentación (Lizer, Parser)
- 0 warnings, 0 errors, 110 tests passing
- Backup Google Drive sincronizado
- Archivos diagnóstico: RESUMEN_FIX_BUGS_v0.1.9.md, CHECKPOINT_100_TESTS_v0.1.9.md
- Próxima: v0.2.0 Module system + Parte gráfica

### v0.1.8 SESIÓN 26 COMPLETADA (2026-03-20)
- **Pruebas Gráficas y Parser Fix**
- 2 bugs críticos fixeados:
  1. IndexAssign: asignación por índice de array (`arr[i] = x`)
  2. Parser: saltar comentarios después de `dark.slot`
- Snake Game funcional con game loop (33 statements)
- Demo shapes visual creada (3 statements)
- 8/8 demos funcionales (100%)
- Binarios compilados: rydit-rs (735 KB), snake (494 KB)
- Diagnóstico completo documentado (12 KB)
- Próxima: v0.2.0 Module system + Parser precedencia

### v0.1.8 COMPLETADA (2026-03-19)
- Sesión 25: Maduración del lenguaje
- 4 features principales implementadas:
  1. Escapes en strings: \\n, \\t, \\\\, \\r
  2. Comillas simples '...' además de "..."
  3. UTF-8 completo (is_numeric, is_alphabetic en lugar de ascii)
  4. Símbolos en identificadores: @, $, %, &, |, ^, ~, `
- 10 tests nuevos añadidos (75 tests totales)
- Demo demo_maduracion_v0.1.8.rydit creada
- 0 warnings, 75 tests pasando
- Próxima: v0.2.0 Module system avanzado

### v0.1.7 COMPLETADA (2026-03-19)
- Sesión 24: Test de Demos - 5 demos creados (random, time, json, strings, arrays)
- 4 bugs críticos fixeados: comentarios //→#, random::int() enteros, demo_time sintaxis, json::parse() objetos
- Lexer mejorado: soporte de escapes \" en strings
- 65 tests Rust pasando, 5/5 demos funcionales
- Backup Google Drive: alucard18:/shield-project-rydit

### v0.1.6 (2026-03-18)
- Random + Time ligeros sin dependencias externas
- PRNG xorshift propio (~50 líneas Rust, +10 KB binario)
- std::time básico (sin chrono, +0 KB)
- 65 tests pasando, 0 warnings

### v0.1.5 (2026-03-18)
- Soporte JSON con serde_json
- json::parse() y json::stringify()
- 63 tests pasando

### v0.1.4 (2026-03-18)
- Strings + IO + Arrays maduros
- 12 funciones strings, 10 funciones io, 6 funciones arrays
- 63 tests pasando

### v0.1.3 (2026-03-17)
- Bug fixes críticos
- 63 tests pasando, 0 warnings

### v0.1.1 (2026-03-17)
- Sistema de módulos (import)
- Imports con alias (import math as m)
- Imports cíclicos detectados
- Cache de módulos (evita re-ejecución)

### v0.1.0 (2026-03-17)
- Snake Game completo
- Parser con AST
- Funciones con retorno
- 60 tests pasando

### v0.0.1-v0.0.14 (2026-03-14 a 2026-03-16)
- CLI básica → Snake Game
- Scopes y argumentos
- Aritmética completa
- Funciones en expresiones

### Proyecto Shield Project (RyDit)
- Lenguaje de scripting en Rust con raylib para Termux
- Arquitectura: blast-core (audio), lizer (lexer), v-shield (gráficos), rydit-rs (binario)
- Optimizado con sccache (17x más rápido) y codegen-units=1 para poca RAM
- Usuario desarrolla en Android/Termux con raylib nativo instalado

### migui (GUI futura v0.4.0)
- El sistema GUI de RyDit se llamará "migui" (no rygui)
- Immediate mode GUI estilo raygui
- API: gui.button(), gui.label(), gui.checkbox(), gui.slider(), gui.textbox(), gui.panel(), gui.window(), gui.message_box()
- Immediate mode: retorna bool/valor por frame, sin estado complejo

### Roadmap RyDit
- v0.1.7: ✅ Test de Demos COMPLETADA
- v0.1.8 Sesión 25: ✅ Maduración (comillas, símbolos, UTF-8, escapes)
- v0.1.8 Sesión 26: ✅ Gráficos + IndexAssign + Snake
- v0.1.9 Sesión 26: ✅ CHECKPOINT 100 TESTS (110 tests)
- v0.2.0: Module system avanzado + Parte gráfica
- v0.3.0: regex + random + time maduros
- v0.4.0: migui (immediate mode GUI ~1000 líneas)
- v0.5.0: Madurez del lenguaje
