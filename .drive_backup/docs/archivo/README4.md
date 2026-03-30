# 🛡️ Shield Project - README4.md

## Resumen de Desarrollo v0.0.2

**Fecha:** 2026-03-14  
**Versión:** v0.0.2 (Parser AST + Condicionales + Operadores + Ciclos)  
**Estado:** ✅ Funcional

---

## 🎯 **Lo que se Logró en v0.0.2**

### **1. Parser con AST** ✅
- Tokens → AST (Árbol de Sintaxis Abstracta)
- Statements: `Init`, `Command`, `Assign`, `If`, `While`, `Expr`
- Expresiones: `Num`, `Texto`, `Var`, `Bool`, `BinOp`, `Unary`

### **2. Condicionales (onif/blelse)** ✅
```rydit
onif x onda.core blelse ryprime
```
- Condición evalúa números (0 = false, != 0 = true)
- Soporta `else` opcional

### **3. Operadores Lógicos** ✅
| Operador | Token | Ejemplo |
|----------|-------|---------|
| `and` | `And` | `a and b` |
| `or` | `Or` | `a or b` |
| `not` | `Not` | `not x` |

### **4. Ciclos (ryda)** ⚠️
```rydit
ryda x onda.core
```
- Loop while simple
- **Limitación:** 1 statement en el body
- **Futuro:** Bloques `{}` para múltiples statements

### **5. Tests Automáticos** ✅
- **22 tests pasando** (15 lizer + 7 blast-core)
- Tests de lexer, parser, memoria, operadores

---

## 📊 **Estado de Features**

| Feature | Estado | Tests | Notas |
|---------|--------|-------|-------|
| Parser AST | ✅ | 3/3 | Funcional |
| Variables | ✅ | 5/5 | Memoria persistente |
| Condicionales | ✅ | 4/4 | onif/blelse funciona |
| AND/OR | ✅ | 4/4 | Conversión num→bool |
| NOT | ✅ | 1/1 | Corregido en v0.0.2 |
| ryda | ⚠️ | 1/2 | 1 statement body |
| Archivos .rydit | ✅ | 2/2 | Lectura funciona |
| REPL | ✅ | 5/5 | Parser integrado |
| Errores | ✅ | 2/2 | Tokens inválidos se ignoran |

---

## 🧪 **Tests Comprobados**

### Test 1: Parser + Variables
```bash
cargo run -- -- "shield.init dark.slot x = 100 onda.core"
# ✅ 3 statements en AST
```

### Test 2: Condicionales
```bash
cargo run -- -- "dark.slot x = 10 onif x onda.core blelse ryprime"
# ✅ ejecuta onda.core
```

### Test 3: AND (falso)
```bash
cargo run -- -- "dark.slot a = 1 dark.slot b = 0 onif a and b onda.core blelse ryprime"
# ✅ ejecuta ryprime
```

### Test 4: OR (verdadero)
```bash
cargo run -- -- "dark.slot a = 1 dark.slot b = 0 onif a or b onda.core blelse ryprime"
# ✅ ejecuta onda.core
```

### Test 5: NOT (corregido)
```bash
cargo run -- -- "dark.slot x = 0 onif not x onda.core blelse ryprime"
# ✅ ejecuta onda.core (not 0 = true)
```

### Test 6: While
```bash
cargo run -- -- "dark.slot x = 1 ryda x onda.core"
# ✅ ejecuta onda.core 100 veces (límite)
```

### Test 7: REPL
```bash
echo -e "dark.slot x = 100\nmem\nexit" | cargo run -- --repl
# ✅ Memoria persistente
```

### Test 8: Archivos .rydit
```bash
cargo run -- -- ejemplo.rydit
# ✅ 6 statements, 3 variables
```

---

## 📁 **Estructura del Proyecto**

```
shield-project/
├── Cargo.toml                    # Workspace
├── crates/
│   ├── lizer/                    # Lexer + Parser + AST
│   │   └── src/lib.rs            # 760 líneas
│   ├── blast-core/               # Executor + Memoria
│   │   └── src/lib.rs            # 170 líneas
│   ├── rydit-rs/                 # Binario principal
│   │   └── src/main.rs           # 280 líneas
│   └── v-shield/                 # Wrapper raylib
│       └── src/lib.rs            # 20 líneas
├── docs/
│   ├── ALERTAS.md                # Tracking 8/8 completas
│   ├── TESTS.md                  # 26 tests documentados
│   ├── GUIA_RAPIDA.md            # Guía de usuario
│   └── sessions/                 # Historial de sesiones
├── ejemplo.rydit                 # Script de ejemplo
└── scripts/
    └── setup-sccache.sh          # Build rápido (17x)
```

---

## 🔧 **Comandos Útiles**

```bash
# Desarrollo
cargo check          # Verificar (1-5s)
cargo build          # Compilar (3-60s con sccache)
cargo test           # 22 tests pasando
cargo run -- -- "script"

# REPL
cargo run -- --repl

# Archivos
cargo run -- -- ejemplo.rydit

# Backup
rclone sync ./ alucard18:/shield-project-rydit
```

---

## 📈 **Métricas**

| Métrica | Valor |
|---------|-------|
| Líneas de código | ~1230 |
| Tests automáticos | 22 |
| Tests manuales | 26 |
| Features completas | 8/9 (89%) |
| Build time (con caché) | 3-5s |
| RAM durante build | ~2 GB |

---

## 🚀 **Próximos Pasos (v0.0.3)**

1. **Bloques `{}` para ryda** - Múltiples statements en body
2. **Funciones (rytmo)** - Definición y llamada
3. **Más operadores** - Comparación (`>`, `<`, `==`)
4. **Mejores errores** - Línea/columna en mensajes
5. **Tests adicionales** - Cobertura > 90%

---

## 📝 **Versiones**

| Versión | Estado | Features |
|---------|--------|----------|
| v0.0.1 | ✅ | CLI, Lexer, Memoria, REPL, Archivos |
| v0.0.2 | ✅ | Parser AST, Condicionales, Operadores, Ciclos |
| v0.0.3 | 📅 | Bloques {}, Funciones, Más operadores |

---

**Última actualización:** 2026-03-14  
**Backup:** Google Drive (`alucard18:/shield-project-rydit`)
