# 🛠️ RyDit Engine - Scripts de Instalación y Utilidades

**Versión**: v0.8.0
**Estado**: ✅ Listos para usar

---

## 📦 INSTALACIÓN AUTOMÁTICA

### Android/Termux (Plataforma Primaria)

```bash
# Ejecutar instalador
./scripts/install.sh

# O directamente desde el repositorio
curl -sSf https://raw.githubusercontent.com/lapumlbb18-blip/Rydit_Engine/main/scripts/install.sh | bash
```

**Qué hace:**
- ✅ Verifica Termux
- ✅ Instala Rust y dependencias
- ✅ Compila RyDit Engine
- ✅ Configura Termux-X11 (opcional)
- ✅ Crea comandos globales (`rydit`, `rydit-demo`, `rydit-x11`)

---

### Linux (Debian/Ubuntu/Fedora/Arch)

```bash
# Ejecutar instalador
./scripts/install-linux.sh

# O directamente
curl -sSf https://raw.githubusercontent.com/lapumlbb18-blip/Rydit_Engine/main/scripts/install-linux.sh | bash
```

**Qué hace:**
- ✅ Detecta distribución
- ✅ Instala Rust vía rustup
- ✅ Instala dependencias de raylib
- ✅ Compila RyDit Engine
- ✅ Crea comando global `rydit`

---

### Windows (PowerShell)

```powershell
# Ejecutar instalador
powershell -ExecutionPolicy Bypass -File scripts/install-windows.ps1

# O directamente
iwr https://raw.githubusercontent.com/lapumlbb18-blip/Rydit_Engine/main/scripts/install-windows.ps1 -useb | iex
```

**Qué hace:**
- ✅ Instala Rust vía winget
- ✅ Instala Visual Studio Build Tools
- ✅ Instala raylib
- ✅ Compila RyDit Engine
- ✅ Agrega al PATH (opcional)

---

## 🎮 SCRIPTS DE EJECUCIÓN

### run_demo.sh - Ejecutar Demos

```bash
# Listar demos disponibles
./scripts/run_demo.sh --list

# Ejecutar un demo específico
./scripts/run_demo.sh snake

# Ejecutar con binario de release
./scripts/run_demo.sh demo_ilusiones_opticas --release

# Ejecutar sin configurar X11
./scripts/run_demo.sh demo --no-x11
```

**Opciones:**
- `-h, --help` - Mostrar ayuda
- `-l, --list` - Listar demos
- `-r, --release` - Usar binario release
- `-d, --debug` - Usar binario debug
- `--no-x11` - No configurar X11

---

### run_tests.sh - Ejecutar Tests

```bash
# Ejecutar todos los tests
./scripts/run_tests.sh

# Tests + tests visuales (requiere X11)
./scripts/run_tests.sh --visual

# Tests + benchmarks
./scripts/run_tests.sh --bench

# Output detallado
./scripts/run_tests.sh --verbose
```

**Opciones:**
- `--visual` - Ejecutar tests visuales
- `--bench` - Ejecutar benchmarks
- `-v, --verbose` - Output detallado
- `--help` - Mostrar ayuda

---

### detect_env.sh - Detectar Entorno

```bash
# Detectar configuración del sistema
./scripts/detect_env.sh
```

**Qué detecta:**
- ✅ Sistema operativo (Termux, Linux, Windows)
- ✅ Arquitectura (AArch64, x86_64, ARM)
- ✅ Rust y Cargo (versión, toolchain)
- ✅ Dependencias gráficas (X11, raylib)
- ✅ Espacio en disco y RAM
- ✅ Proyecto RyDit (crates, demos, tests)
- ✅ Recomendaciones personalizadas

**Ejemplo de output:**
```
==================================================
  🔍 RyDit Engine - Detección de Entorno
==================================================

SISTEMA OPERATIVO
----------------------------------------
  ✓ Android/Termux
    └─ Termux 1.0
    └─ Android 13

ARQUITECTURA
----------------------------------------
  Máquina: aarch64
  ✓ ARM64 (AArch64) - Soportado
    └─ Ideal para Termux

RUST Y CARGO
----------------------------------------
  ✓ Rust: rustc 1.74.0
    └─ Versión compatible (≥1.70)
  ✓ Cargo: cargo 1.74.0

...
```

---

## 📊 FLUJO DE TRABAJO RECOMENDADO

### 1. Instalación Inicial

```bash
# Termux
./scripts/install.sh

# Linux
./scripts/install-linux.sh

# Windows
powershell -ExecutionPolicy Bypass -File scripts/install-windows.ps1
```

### 2. Verificar Entorno

```bash
./scripts/detect_env.sh
```

### 3. Ejecutar Tests

```bash
# Tests básicos
./scripts/run_tests.sh

# Tests completos con visuales
./scripts/run_tests.sh --visual --bench
```

### 4. Probar Demos

```bash
# Listar demos
./scripts/run_demo.sh --list

# Ejecutar demo
./scripts/run_demo.sh snake
```

---

## 🔧 COMANDOS GLOBALES (después de instalar)

### Termux/Linux

```bash
rydit              # Ejecutar RyDit
rydit --repl       # Modo REPL
rydit --gfx file   # Ejecutar demo gráfico
rydit-demo demo    # Ejecutar demo fácilmente
rydit-x11          # Iniciar con Termux-X11
```

### Windows (PowerShell)

```powershell
rydit              # Ejecutar RyDit
rydit --repl       # Modo REPL
rydit --gfx file   # Ejecutar demo gráfico
rydit-demo demo    # Ejecutar demo
```

---

## 📁 ESTRUCTURA DE SCRIPTS

```
scripts/
├── install.sh              # Instalador para Termux
├── install-linux.sh        # Instalador para Linux
├── install-windows.ps1     # Instalador para Windows
├── run_demo.sh             # Ejecutar demos
├── run_tests.sh            # Ejecutar tests
├── detect_env.sh           # Detectar entorno
├── benchmark_v0.5.0.sh     # Benchmark (legacy)
└── setup-sccache.sh        # Configurar sccache (legacy)
```

---

## 🛠️ DESARROLLO DE SCRIPTS

### Agregar nuevo script

1. Crear archivo en `scripts/`
2. Agregar shebang: `#!/bin/bash`
3. Hacer ejecutable: `chmod +x scripts/nuevo_script.sh`
4. Documentar en este README

### Estándares

- Usar colores para output (ROJO, VERDE, AMARILLO, AZUL)
- Funciones de logging: `log_info`, `log_success`, `log_warning`, `log_error`
- Manejar errores con `set -e`
- Documentar opciones con `--help`

---

## 🐛 SOLUCIÓN DE PROBLEMAS

### Error: "Permission denied"

```bash
# Hacer scripts ejecutables
chmod +x scripts/*.sh
chmod +x scripts/*.ps1
```

### Error: "Rust no encontrado"

```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Error: "DISPLAY not set" (Termux)

```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1
```

### Error: "winget not found" (Windows)

```powershell
# Instalar winget o usar instalación manual
# Ver: https://github.com/microsoft/winget-cli
```

---

## 📞 SOPORTE

- **Documentación**: `README.md`, `docs/`
- **Issues**: https://github.com/lapumlbb18-blip/Rydit_Engine/issues
- **Discusión**: https://github.com/lapumlbb18-blip/Rydit_Engine/discussions

---

<div align="center">

**🛡️ RyDit Engine - Scripts v0.8.0**

*Instalación automática multi-plataforma*

</div>
