# 🛡️ RyDit Engine - ROADMAP ACTUALIZADO 2026

**Última actualización**: 2026-03-26  
**Versión actual**: v0.8.2 - SISTEMA UNIVERSAL RY ✅ COMPLETADO  
**Próxima versión**: v0.8.3 - MADURACIÓN Y OPTIMIZACIÓN

---

## 📊 ESTADO ACTUAL (v0.8.2)

### ✅ Completado
- [x] **4 crates publicados** en crates.io (core, science, physics, anim)
- [x] **Sistema Universal Ry** - Carga dinámica de módulos
- [x] **Hot reload** - Hooks y loader global
- [x] **Scripts como módulos** - Parser de .rydit
- [x] **Demo módulo dinámico** - modulo_ejemplo.so
- [x] **273 tests passing** ✅
- [x] **Push a GitHub** completado

### ⚠️ Deuda Técnica
| Ítem | Cantidad | Prioridad | Impacto |
|------|----------|-----------|---------|
| **Warnings clippy** | ~26 | Media | Código limpio |
| **&Vec en vez de &[_]** | 15 | Baja | Performance menor |
| **too_many_arguments** | 2 | Baja | Mantenibilidad |
| **single_match** | 2 | Baja | Legibilidad |
| **manual_clamp** | 3 | Baja | Consistencia |

---

## 🎯 PRIORIDADES ANTES DE v0.9.0

### **FASE 0: MADURACIÓN v0.8.3** (1-2 semanas)

**Objetivo**: Consolidar v0.8.2, eliminar deuda técnica, preparar para publicación

#### Tarea 1: Fix Warnings ⭐⭐⭐
**Duración**: 2-3 días

```bash
# Warnings a fixear (26 totales)
cargo clippy --release 2>&1 | grep "warning:" | wc -l
```

**Prioridad**:
1. ✅ `&Vec` → `&[_]` (15 warnings) - 1-2 horas
2. ✅ `manual_clamp` (3 warnings) - 30 min
3. ✅ `single_match` (2 warnings) - 30 min
4. ⏳ `too_many_arguments` (2 warnings) - Requiere refactor (2-3 horas)
5. ⏳ Otros (4 warnings) - Evaluar caso por caso

**Meta**: <10 warnings (idealmente 0)

---

#### Tarea 2: Optimizar Runtime ⭐⭐⭐
**Duración**: 3-4 días

**Objetivos**:
- [ ] **Startup time**: <100ms (actual ~200ms)
- [ ] **RAM en reposo**: <80 MB (actual ~100 MB)
- [ ] **Build time**: <1m (actual 1m 10s)

**Acciones**:
```toml
# Cargo.toml - Optimizaciones
[profile.release]
opt-level = 3        # Máxima optimización
lto = "thin"         # Link-time optimización
codegen-units = 1    # Mejor optimización, más build time
strip = true         # Ya está
panic = "abort"      # Ya está

# Perfils específicos
[profile.release-small]
inherits = "release"
opt-level = "z"      # Optimizar tamaño
```

**Benchmarks**:
```bash
# Medir startup time
hyperfine --warmup 3 './target/release/rydit-rs --repl --help'

# Medir RAM
/usr/bin/time -v ./target/release/rydit-rs --repl

# Build time
cargo build --release --timings
```

---

#### Tarea 3: Tests de Demos Gráficos ⭐⭐⭐⭐
**Duración**: 4-5 días

**Objetivo**: Grabar videos y capturas para publicación

**Demos a probar**:
1. ✅ **bezier_demo.rydit** - Curva Bezier animada
2. ✅ **bezier_completo.rydit** - 3 tipos de curvas
3. ⏳ **snake.rydit** - Juego completo
4. ⏳ **demo_particulas.rydit** - Sistema de partículas
5. ⏳ **migui_demo.rydit** - 12 widgets UI

**Checklist por demo**:
- [ ] Probar en Termux-X11 @ 60 FPS
- [ ] Capturar screenshots (3-5 por demo)
- [ ] Grabar video (30-60 segundos)
- [ ] Verificar sin crashes
- [ ] Medir FPS (debe ser estable)
- [ ] Documentar controles

**Equipo necesario**:
```bash
# Grabar pantalla en Termux-X11
pkg install termux-x11-nightly

# O usar screenrecord de Android
screenrecord --time-limit 60 /sdcard/rydit_demo.mp4

# Screenshots
screencap -p /sdcard/rydit_screenshot.png
```

**Productos finales**:
- 📸 15-25 screenshots (organizados en `screenshots/`)
- 🎥 3-5 videos cortos (30-60s cada uno)
- 📝 README actualizado con imágenes reales
- 🎬 Video principal "RyDit en Acción" (2-3 min)

---

#### Tarea 4: Madurar Módulos ⭐⭐
**Duración**: 3-4 días

**Evaluación del panorama**:

**Crates actuales**:
```
rydit-core      ✅ Estable, bien diseñado
rydit-loader    ✅ Funcional, pero limitado a desktop
rydit-script    ⚠️  Parser básico, falta runtime completo
rydit-science   ✅ Bezier + Stats + Geometry completos
rydit-physics   ✅ Projectile + NBody funcionales
rydit-anim      ✅ Easing + Squash/Stretch
```

**A mejorar**:
1. **rydit-script**:
   - [ ] Integrar con evaluator de rydit-rs
   - [ ] Ejecutar scripts reales (no solo placeholder)
   - [ ] Soporte para imports entre scripts

2. **rydit-loader**:
   - [ ] Soporte para Android (si es posible)
   - [ ] Hot reload real (recargar .so sin reiniciar)
   - [ ] Dependency tracking

3. **Módulos stdlib** (`modules/*.rydit`):
   - [ ] Auto-detección al iniciar
   - [ ] Carga automática
   - [ ] Documentación de cada módulo

**Decisiones a tomar**:
- ¿Vale la pena mejorar rydit-script o enfocarse en módulos Rust?
- ¿Android soporta dlopen() sin root?
- ¿Cuántos módulos stdlib son necesarios?

---

#### Tarea 5: Mejorar Features/Crates ⭐⭐
**Duración**: 2-3 días

**Evaluación**:

**Crates publicados (crates.io)**:
- ✅ rydit-core v0.8.2
- ✅ rydit-science v0.8.2
- ✅ rydit-physics v0.8.2
- ✅ rydit-anim v0.8.2

**Crates locales (publicables)**:
- ⏳ rydit-gfx → ¿Publicar? (depende de raylib)
- ⏳ rydit-loader → ✅ Publicar (útil para otros)
- ⏳ rydit-script → ⚠️ Mejorar antes de publicar
- ⏳ migui → ✅ Publicar (immediate mode GUI)
- ⏳ lizer → ⚠️ Evaluar (lexer/parser genérico)
- ⏳ blast-core → ⚠️ Muy acoplado a RyDit

**Recomendaciones**:
1. **Publicar rydit-loader** - Útil para sistemas de plugins
2. **Publicar migui** - GUI ligera, independiente
3. **Mejorar rydit-script** - Antes de publicar
4. **No publicar blast-core** - Muy específico de RyDit

---

## 📋 PLAN DE SESIONES

### Sesión v0.8.3 - Fix & Optimize (3-4 días)
- [ ] Fix 26 warnings clippy
- [ ] Optimizar startup time (<100ms)
- [ ] Optimizar RAM (<80 MB)
- [ ] Benchmark pre/post optimización

### Sesión v0.8.4 - Graphics Testing (4-5 días)
- [ ] Probar 5 demos en Termux-X11
- [ ] Capturar 15-25 screenshots
- [ ] Grabar 3-5 videos cortos
- [ ] Crear video principal "RyDit en Acción"

### Sesión v0.8.5 - Module Maturation (3-4 días)
- [ ] Evaluar rydit-script (mejorar o descartar)
- [ ] Auto-detección modules/*.rydit
- [ ] Decidir crates a publicar
- [ ] Documentar módulos stdlib

### Sesión v0.8.6 - Final Polish (2-3 días)
- [ ] Fix bugs encontrados en testing
- [ ] Actualizar README con screenshots/videos
- [ ] Actualizar documentación crates.io
- [ ] Preparar release notes v0.9.0

---

## 🚀 v0.9.0 - PRODUCCIÓN (2-3 semanas)

### Fase 1: GitHub Actions ⭐⭐⭐⭐
**Duración**: 3-4 días

**Workflow principal** (`.github/workflows/ci.yml`):
```yaml
name: CI/CD

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-action@stable
      - run: cargo test --release
  
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-action@stable
      - run: cargo build --release
  
  clippy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-action@clippy
      - run: cargo clippy -- -D warnings
```

**Runners**:
- ✅ ubuntu-latest (gratis)
- ✅ windows-latest (gratis)
- ✅ macos-latest (gratis, limitado a 5 horas)

---

### Fase 2: Multi-Plataforma ⭐⭐⭐⭐
**Duración**: 4-5 días

**Plataformas objetivo**:
1. ✅ **Linux** (x86_64, aarch64) - Ya funciona
2. ⏳ **Windows** (x86_64) - Probar build
3. ⏳ **macOS** (Intel + Apple Silicon) - Probar build
4. ⚠️ **Android** (Termux) - Ya funciona, pero limitado

**Binarios precompilados**:
```bash
# Estructura de release
rydit-v0.9.0/
├── rydit-v0.9.0-x86_64-unknown-linux-gnu.tar.gz
├── rydit-v0.9.0-x86_64-pc-windows-msvc.zip
├── rydit-v0.9.0-x86_64-apple-darwin.tar.gz
├── rydit-v0.9.0-aarch64-apple-darwin.tar.gz
└── rydit-v0.9.0-aarch64-linux-android.tar.gz
```

---

### Fase 3: Métodos de Instalación ⭐⭐⭐
**Duración**: 3-4 días

**Método 1: Cargo (ya funciona)**
```bash
cargo install rydit-rs
```

**Método 2: Install Script (recomendado)**
```bash
# Linux/macOS
curl -fsSL https://rydit.dev/install.sh | sh

# Windows (PowerShell)
irm https://rydit.dev/install.ps1 | iex
```

**Install Script** (`install.sh`):
```bash
#!/bin/bash
set -e

# Detectar plataforma
OS=$(uname -s)
ARCH=$(uname -m)

# Descargar binario precompilado
VERSION="v0.9.0"
URL="https://github.com/lapumlbb18-blip/Rydit_Engine/releases/download/${VERSION}/rydit-${VERSION}-${ARCH}-${OS}.tar.gz"

# Instalar
curl -L "$URL" | tar xz
sudo mv rydit-rs /usr/local/bin/

echo "✅ RyDit Engine ${VERSION} instalado"
```

**Método 3: Git Clone + Build**
```bash
git clone https://github.com/lapumlbb18-blip/Rydit_Engine
cd Rydit_Engine
cargo build --release
sudo cp target/release/rydit-rs /usr/local/bin/
```

**Método 4: Package Managers (futuro)**
- [ ] Homebrew (macOS)
- [ ] AUR (Arch Linux)
- [ ] Scoop (Windows)

---

## 📅 TIMELINE ESTIMADO

| Semana | Versión | Foco | Duración |
|--------|---------|------|----------|
| **Semana 1** | v0.8.3 | Fix warnings + Optimizar | 3-4 días |
| **Semana 2** | v0.8.4 | Tests gráficos + Videos | 4-5 días |
| **Semana 3** | v0.8.5 | Madurar módulos | 3-4 días |
| **Semana 4** | v0.8.6 | Final polish | 2-3 días |
| **Semana 5-6** | v0.9.0 | GitHub Actions + Release | 7-10 días |

**Total estimado**: 5-6 semanas (1-1.5 meses)

---

## 🎯 CRITERIOS DE ÉXITO v0.9.0

### Calidad de Código
- [ ] **0 warnings** clippy
- [ ] **300+ tests** passing
- [ ] **<100ms** startup time
- [ ] **<80 MB** RAM en reposo

### Documentación
- [ ] README con 15+ screenshots
- [ ] 3-5 videos demostrativos
- [ ] Documentación crates.io actualizada
- [ ] Tutorial "Getting Started"

### Infraestructura
- [ ] GitHub Actions funcionando
- [ ] 3 plataformas soportadas (Linux, Windows, macOS)
- [ ] 2 métodos de instalación (cargo + install.sh)
- [ ] CI/CD automático en push

### Comunidad
- [ ] 1-2 módulos de comunidad
- [ ] Issues/PRs respondidos
- [ ] Discord/Telegram grupo (opcional)

---

## 🔗 REFERENCIAS

### Sesiones Completadas
- ✅ v0.8.1 - Gráficos Bezier + Fix Warnings
- ✅ v0.8.2 - Sistema Universal Ry (4 fases + demo)

### Próximas Sesiones
- ⏳ v0.8.3 - Fix & Optimize
- ⏳ v0.8.4 - Graphics Testing
- ⏳ v0.8.5 - Module Maturation
- ⏳ v0.8.6 - Final Polish
- ⏳ v0.9.0 - Producción

---

<div align="center">

**🛡️ RyDit Engine - ROADMAP 2026**

*v0.8.2 ✅ | v0.8.3-v0.8.6 🔄 | v0.9.0 ⏳*

**Próxima sesión: v0.8.3 - Fix Warnings + Optimizar**

</div>
