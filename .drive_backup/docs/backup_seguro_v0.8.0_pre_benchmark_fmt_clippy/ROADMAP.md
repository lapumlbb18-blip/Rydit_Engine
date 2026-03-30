# 🛣️ RyDit Engine - Roadmap

**Versión Actual:** v0.7.34 (4 CRATES PUBLICADOS EN CRATES.IO ✅)
**Próxima Versión:** v0.8.0 (Linux/Windows builds + más features)
**Última Actualización:** 2026-03-26

---

## 📍 Versión Actual: v0.7.34 - 4 CRATES PUBLICADOS ✅

### ✅ Completado en esta sesión

- [x] **rydit-core** v0.7.34 - Trait RyditModule + Registry (4 tests) ✅ PUBLICADO
- [x] **rydit-science** v0.7.34 - Bezier + Stats + Geometry (21 tests) ✅ PUBLICADO
- [x] **rydit-physics** v0.7.34 - Projectile + NBody (6 tests) ✅ PUBLICADO
- [x] **rydit-anim** v0.7.34 - Easing + Squash/Stretch (9 tests) ✅ PUBLICADO
- [x] **Geometría** - 5 ilusiones ópticas implementadas (Penrose, Cube, Spiral, Müller-Lyer, Ponzo)
- [x] **Demo visual** - Funcionando en Termux-X11 (800x600 @ 60 FPS)
- [x] **crates.io** - 4 crates publicados oficialmente
- [x] **Documentación** - READMEs + ejemplos + guía de publicación

### 📊 Métricas Actuales

| Crate | Tests | Líneas | Estado |
|-------|-------|--------|--------|
| rydit-core | 4 | ~150 | ✅ PUBLICADO v0.7.34 |
| rydit-science | 21 | ~450 | ✅ PUBLICADO v0.7.34 |
| rydit-physics | 6 | ~200 | ✅ PUBLICADO v0.7.34 |
| rydit-anim | 9 | ~280 | ✅ PUBLICADO v0.7.34 |
| rydit-rs | 53 | ~5,000 | ✅ Binario local |
| **Total** | **~93** | **~6,080** | **✅ 4 crates en crates.io** |

### ⚠️ Pendiente

- [ ] rydit-geometry (ilusiones ópticas)
- [ ] Publicación en crates.io
- [ ] Linux/Windows builds

---

## 🎯 Próxima Versión: v0.7.3.x - rydit-geometry

### Objetivos Principales

1. **Implementar geometría** - Ilusiones ópticas reales
2. **Tests visuales** - Validar con Termux-X11
3. **Integrar con LAZOS** - Comandos JSON-RPC

### Features Planeadas

```rust
// geometry::penrose - Triángulo imposible
echo '{"method":"geometry::penrose","params":[400,300,100]}' | rydit-rs --lazos

// geometry::impossible_cube - Cubo imposible
echo '{"method":"geometry::impossible_cube","params":[400,300,80]}' | rydit-rs --lazos

// geometry::spiral - Espiral de Fraser
echo '{"method":"geometry::spiral","params":[400,300,10,150,5]}' | rydit-rs --lazos
```

### Criterios de Aceptación

- [ ] 3 ilusiones implementadas (Penrose, Cube, Spiral)
- [ ] 6+ tests passing
- [ ] Demo visual funcional en Termux-X11
- [ ] Backup + Commit + Push

### Tiempo Estimado

2-3 horas de desarrollo

---

## 🚀 Versión: v0.8.0.0 - Ecosistema Ry (Crates.io + Multi-plataforma)

### Objetivos Principales

1. **Publicación crates.io** - 4 crates disponibles
2. **Linux native** - Build en Linux
3. **Windows native** - Build en Windows
4. **CI/CD** - GitHub Actions

### Features Planeadas

#### Publicación crates.io
- [ ] rydit-core (primero, sin dependencias externas)
- [ ] rydit-science (depende de rydit-core)
- [ ] rydit-physics (depende de rydit-core)
- [ ] rydit-anim (depende de rydit-core)

#### Multi-plataforma
- [ ] rydit-linux (binario Linux)
- [ ] rydit-windows (binario Windows .exe)
- [ ] GitHub Actions (build automático)

#### Documentación
- [ ] docs.rs para cada crate
- [ ] README en inglés
- [ ] Ejemplos de uso

### Criterios de Aceptación

- [ ] 4 crates en crates.io
- [ ] Binarios Linux + Windows funcionales
- [ ] CI/CD verde (tests passing)
- [ ] Documentación completa

### Tiempo Estimado

4-6 semanas de desarrollo

---

## 🔮 Versiones Futuras

### v0.9.0.0 - Expansión (2-3 meses)

- [ ] **ry-web** - WebAssembly (correr en navegador)
- [ ] **HTTP nativo** - Sin Python bridge
- [ ] **WebSocket** - Comunicación en tiempo real
- [ ] **Git integration** - Control de versiones embebido

### v1.0.0 - Release Estable (6-8 meses)

- [ ] **API estable** - Sin breaking changes
- [ ] **20+ demos** - Ejemplos reales
- [ ] **Documentación completa** - Libro/tutorial
- [ ] **Tutoriales YouTube** - Canal oficial
- [ ] **Comunidad activa** - Discord, contribuidores

### v2.0.0 - IDE RyDit (1 año)

- [ ] **Editor integrado** - Syntax highlighting
- [ ] **Debugger** - Breakpoints, step-through
- [ ] **Profiler** - Performance analysis
- [ ] **Asset manager** - Sprites, sonidos

---

## 📊 Comparativa de Versiones

| Versión | Fecha | Tests | Crates | Estado |
|---------|-------|-------|--------|--------|
| v0.7.2.0 | 2026-03-25 | 68 | 1 (monolito) | ✅ LAZOS |
| v0.7.3.0 | 2026-03-26 | 72 | 2 (core + science) | ✅ Split inicia |
| v0.7.3.1 | 2026-03-26 | 72 | 3 (+ physics) | ✅ Extracción |
| v0.7.3.2 | 2026-03-26 | 72 | 4 (+ anim) | ✅ Completado |
| v0.7.3.3 | 2026-03-26 | 81 | 4 | ✅ GitHub |
| v0.7.3.x | Próxima | 87+ | 5 | ⏳ Geometry |
| v0.8.0.0 | 4-6 semanas | 100+ | 4 + crates.io | 🔮 Multi-platform |
| v1.0.0 | 6-8 meses | 500+ | 20+ | 🔮 Production |

---

## 🎯 Métricas de Éxito

### Código
- ✅ **81 tests** passing
- ✅ **~5,930 líneas** en crates
- ✅ **4 crates** independientes
- ✅ **730 KB** binario release

### Comunidad (Objetivos v1.0.0)
- 🔮 **1,000+** stars GitHub
- 🔮 **50+** contribuidores
- 🔮 **100+** demos de comunidad
- 🔮 **10,000+** descargas crates.io

### Plataforma (Objetivos v1.0.0)
- 🔮 Android (Termux) ✅ Nativo
- 🔮 Linux ✅ Nativo
- 🔮 Windows ✅ Nativo
- 🔮 Web (WASM) 🔮 En desarrollo
- 🔮 iOS ⏳ Futuro

---

## 📝 Notas de Desarrollo

### Filosofía RyDit

1. **Mobile-First Real** - Nació en Android, no portado
2. **Ligero y Portable** - <1 MB, sin dependencias pesadas
3. **Educativo** - Código abierto, lenguaje en español
4. **David vs Goliat** - 6,000 líneas bien escritas > 500,000 líneas
5. **Rendimiento Estable** - Sin calentamiento, RAM <100 MB, 60 FPS

### Lecciones Aprendidas

✅ **Lo que funcionó:**
- Punto de restauración git antes de cambios grandes
- Backup local + Google Drive después de cada crate
- Tests primero - validar antes y después
- Extracción incremental - un crate por vez
- Commit messages descriptivos

⚠️ **Desafíos:**
- `eval/mod.rs` usa `Valor` (blast_core), no `serde_json::Value`
- `lazos.rs` tenía funciones hardcodeadas
- Termux-X11 requerido para tests visuales

🚀 **Mejoras Futuras:**
- Unificar `Valor` ↔ `serde_json::Value` conversion
- Usar módulos en lazos.rs en vez de funciones hardcodeadas
- Implementar CI/CD con GitHub Actions

---

<div align="center">

**🛡️ RyDit Engine Roadmap**

*v0.7.3.3 ✅ | 81 tests | 4 crates | GitHub actualizado*

**Próximo: v0.7.3.x → rydit-geometry | v0.8.0.0 → crates.io**

</div>
