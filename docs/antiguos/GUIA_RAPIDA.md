# 📘 Guía Rápida de RyDit

**Versión:** v0.0.1  
**Tiempo de lectura:** 5 minutos

---

## 🚀 Empezando en 1 Minuto

### 1. Ejecutar tu Primer Script

```bash
# Script directo
cargo run -- -- "shield.init"

# Desde archivo
cargo run -- -- ejemplo.rydit
```

### 2. Crear Variables

```bash
cargo run -- -- "dark.slot vida = 100"
cargo run -- -- "dark.slot delta.flow = 0.5"
cargo run -- -- 'dark.slot nombre = "Heroe"'
```

### 3. Usar Comandos

```bash
cargo run -- -- "shield.init onda.core ryprime"
```

---

## 📖 Referencia de Comandos

### Comandos Principales

| Comando | Descripción | Ejemplo |
|---------|-------------|---------|
| `shield.init` | Inicializar sistema | `shield.init` |
| `onda.core` | Ejecutar acción central | `onda.core` |
| `ryprime` | Operación especial | `ryprime` |
| `onif` | Condicional (si) | `onif` |
| `blelse` | Alternativa (si no) | `blelse` |

### Variables

| Sintaxis | Descripción | Ejemplo |
|----------|-------------|---------|
| `dark.slot x = 100` | Variable numérica | `dark.slot vida = 100` |
| `dark.slot x = 0.5` | Variable decimal | `dark.slot delta.flow = 0.5` |
| `dark.slot x = "texto"` | Variable texto | `dark.slot nombre = "Heroe"` |

### Jerarquía con Puntos

```
dark.slot jugador.vida = 100
dark.slot jugador.nombre = "Heroe"
dark.slot enemigo.vida = 50
```

---

## 📝 Sintaxis

### Reglas Básicas

1. **Comandos en español:** `shield.init`, `onda.core`
2. **Variables con jerarquía:** `jugador.vida.max`
3. **Números:** enteros (`100`) o decimales (`0.5`)
4. **Strings:** entre comillas (`"hola"`)
5. **Comentarios:** con `#` (`# esto es un comentario`)

### Ejemplo Completo

```rydit
# Mi script RyDit
shield.init

# Variables
dark.slot jugador.vida = 100
dark.slot jugador.vida.max = 100
dark.slot jugador.nombre = "Heroe"

# Comandos
onda.core
ryprime

# Fin
```

---

## 🧪 Ejemplos

### Ejemplo 1: Script Básico

```bash
cargo run -- -- "shield.init dark.slot x = 100 onda.core"
```

**Output:**
```
--- SHIELD SYSTEM: MODO COMANDANTE ---
[RYDIT] Ejecutando: shield.init dark.slot x = 100 onda.core
[BLAST-CORE]: Executor despertado con memoria.
[SHIELD] Inicializando sistema...
[MEMORIA] x = 100
[BLAST-CORE] Ejecutando: onda.core

=== MEMORIA RYDIT ===
  x = 100
=====================
--- SISTEMA PROTEGIDO ---
```

### Ejemplo 2: Múltiples Variables

```bash
cargo run -- -- "dark.slot delta.flow = 0.5 dark.slot jugador.vida = 100"
```

**Output:**
```
[MEMORIA] delta.flow = 0.5
[MEMORIA] jugador.vida = 100

=== MEMORIA RYDIT ===
  jugador.vida = 100
  delta.flow = 0.5
=====================
```

### Ejemplo 3: Desde Archivo

```bash
# Crear archivo
echo "shield.init
dark.slot x = 100
onda.core" > mi_script.rydit

# Ejecutar
cargo run -- -- mi_script.rydit
```

---

## ⚠️ Manejo de Errores

### Error: Carácter No Reconocido

```bash
cargo run -- -- "dark.slot @#$"
```

**Output:**
```
[ERROR] Carácter '@' no reconocido (col 12)
⚠️  1 error(s) encontrado(s)
```

### Error: String Sin Cerrar

```bash
cargo run -- -- '"hola sin cerrar'
```

**Output:**
```
[ERROR] String sin cerrar en línea 1, columna 1
⚠️  1 error(s) encontrado(s)
```

### Error: Archivo No Existe

```bash
cargo run -- -- no_existe.rydit
```

**Output:**
```
[ERROR] No se pudo leer el archivo 'no_existe.rydit': No such file or directory
```

---

## 🧪 Tests

Ejecutar tests del proyecto:

```bash
cargo test
# 19 tests passed ✅
```

---

## 📁 Estructura de Archivos

```
proyecto/
├── ejemplo.rydit        # Script de ejemplo
├── Cargo.toml           # Configuración
├── docs/
│   ├── README.md        # Documentación principal
│   ├── GUIA_USO.txt     # Guía de comandos
│   └── ALERTAS.md       # Estado del desarrollo
└── crates/
    ├── lizer/           # Lexer
    ├── blast-core/      # Executor
    └── rydit-rs/        # Binario principal
```

---

## 🆘 FAQ

### ¿Cómo creo un script?

Crea un archivo `.rydit`:
```bash
echo "shield.init" > mi_script.rydit
```

### ¿Cómo ejecuto un script?

```bash
cargo run -- -- mi_script.rydit
```

### ¿Puedo usar variables?

Sí:
```bash
dark.slot x = 100
dark.slot y = 200
```

### ¿Los comentarios son necesarios?

No, pero ayudan:
```rydit
# Esto es un comentario
shield.init  # También inline
```

---

## 🎯 Próximos Pasos

1. **Prueba los ejemplos** de esta guía
2. **Crea tu propio script** `.rydit`
3. **Experimenta** con variables y comandos

---

**¿Preguntas?** Revisa `docs/ALERTAS.md` para el estado del desarrollo.
