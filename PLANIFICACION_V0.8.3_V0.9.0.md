# 📋 PLANIFICACIÓN v0.8.3 → v0.9.0

**Fecha**: 2026-03-26  
**Estado**: v0.8.2 ✅ COMPLETADO  
**Próximo**: v0.8.3 - MADURACIÓN

---

## 🎯 PRIORIDADES (EN ORDEN)

### 1️⃣ **Fix Warnings** (2-3 días) ⭐⭐⭐
**Objetivo**: 0 deuda técnica

```bash
# Actual: 26 warnings
cargo clippy --release | grep "warning:" | wc -l

# Meta: 0 warnings
```

**Tasks**:
- [ ] `&Vec` → `&[_]` (15 warnings)
- [ ] `manual_clamp` (3 warnings)
- [ ] `single_match` (2 warnings)
- [ ] Evaluar `too_many_arguments` (2 warnings)

---

### 2️⃣ **Optimizar Runtime** (3-4 días) ⭐⭐⭐
**Objetivo**: Máximo rendimiento

| Métrica | Actual | Meta |
|---------|--------|------|
| Startup | ~200ms | <100ms |
| RAM | ~100 MB | <80 MB |
| Build | 1m 10s | <1m |

**Acciones**:
- [ ] Profile con `cargo build --timings`
- [ ] Ajustar `[profile.release]`
- [ ] Benchmark pre/post

---

### 3️⃣ **Tests Gráficos Termux-X11** (4-5 días) ⭐⭐⭐⭐
**Objetivo**: Material para publicación

**Demos a testear**:
1. bezier_demo.rydit
2. bezier_completo.rydit
3. snake.rydit
4. demo_particulas.rydit
5. migui_demo.rydit

**Productos**:
- 📸 15-25 screenshots
- 🎥 3-5 videos (30-60s)
- 🎬 Video principal (2-3 min)

---

### 4️⃣ **Madurar Módulos** (3-4 días) ⭐⭐
**Objetivo**: Evaluar y decidir

**A evaluar**:
- rydit-script: ¿Mejorar o descartar?
- rydit-loader: ¿Soporte Android?
- modules/*.rydit: ¿Auto-detección?

**Decisiones**:
- ¿Qué crates publicar a crates.io?
- ¿Qué features priorizar?

---

### 5️⃣ **v0.9.0 - GitHub Actions** (7-10 días) ⭐⭐⭐⭐
**Objetivo**: CI/CD + Multi-plataforma

**Fases**:
1. GitHub Actions workflow
2. 3 plataformas (Linux, Windows, macOS)
3. 2 métodos instalación (cargo + install.sh)

---

## 📅 CRONOGRAMA

| Semana | Versión | Foco | Días |
|--------|---------|------|------|
| **1** | v0.8.3 | Fix + Optimize | 3-4 |
| **2** | v0.8.4 | Tests Gráficos | 4-5 |
| **3** | v0.8.5 | Madurar Módulos | 3-4 |
| **4** | v0.8.6 | Final Polish | 2-3 |
| **5-6** | v0.9.0 | GitHub Actions | 7-10 |

**Total**: 5-6 semanas

---

## ✅ CRITERIOS v0.9.0

- [ ] 0 warnings clippy
- [ ] 300+ tests passing
- [ ] <100ms startup
- [ ] <80 MB RAM
- [ ] 15+ screenshots
- [ ] 3-5 videos
- [ ] GitHub Actions activo
- [ ] 3 plataformas soportadas

---

## 🚀 COMENZAMOS CON v0.8.3

**Primera tarea**: Fix warnings

```bash
cd shield-project
cargo clippy --release 2>&1 | tee /tmp/clippy.txt
```

¿Comenzamos?

---

<div align="center">

**Planificación v0.8.3 → v0.9.0**

*5-6 semanas | 5 sesiones | Objetivo: Producción*

</div>
