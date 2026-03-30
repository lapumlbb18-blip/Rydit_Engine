# 🛡️ RyDit v0.9.0 - GUÍA RÁPIDA TERMUX-X11

**Última actualización**: 2026-03-28
**Estado**: ✅ Listo para ejecutar

---

## 🚀 INICIO RÁPIDO

### 1. Configurar Entorno Gráfico

```bash
# En Termux (antes de abrir Termux-X11)
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

echo "✅ Entorno configurado:"
echo "   DISPLAY=$DISPLAY"
echo "   Driver: $MESA_LOADER_DRIVER_OVERRIDE"
```

### 2. Abrir Termux-X11

```bash
# En otra sesión de Termux o desde app Termux:X11
pkg install termux-x11-nightly
termux-x11 :0 -ac &

# O usar la app Termux:X11 desde el launcher
```

### 3. Ejecutar Tests

#### Opción A: Script Automático (RECOMENDADO)

```bash
cd /data/data/com.termux/files/home/shield-project
./test_gfx_v0.9.0.sh
```

#### Opción B: Comandos Directos

```bash
cd /data/data/com.termux/files/home/shield-project

# Test 1: Formas básicas
./target/release/rydit-rs --gfx ejemplos_gfx/demo_shapes.rydit

# Test 2: Ejemplo gráfico
./target/release/rydit-rs --gfx ejemplos_gfx/ejemplo_gfx.rydit

# Test 3: Test renderizado v0.9.0
./target/release/rydit-rs --gfx demos/test_renderizado_v0.9.0.rydit

# Test 4: Demo Rust (3 capas críticas)
./target/release/examples/demo_render_queue
```

---

## 📊 TESTS DISPONIBLES

### Tests RyDit (.rydit)

| Test | Archivo | Descripción |
|------|---------|-------------|
| **Formas Básicas** | `ejemplos_gfx/demo_shapes.rydit` | Círculos, rects, líneas, texto |
| **Ejemplo Gráfico** | `ejemplos_gfx/ejemplo_gfx.rydit` | Formas + ciclos |
| **Ejemplo Simple** | `ejemplos_gfx/ejemplo.rydit` | Mínimo funcional |
| **Test Renderizado** | `demos/test_renderizado_v0.9.0.rydit` | Test completo v0.9.0 |
| **Snake Game** | `ejemplos_gfx/snake_v0.1.8.rydit` | Juego completo |

### Tests Rust (render queue)

| Demo | Archivo | Descripción |
|------|---------|-------------|
| **Render Queue** | `demo_render_queue` | 100 círculos + 50 rects + 36 líneas |

---

## 🎯 QUÉ PROBAR

### Test 1: Formas Básicas (demo_shapes.rydit)

**Qué verificar**:
- ✅ 3 círculos concéntricos (rojo, verde, azul)
- ✅ 4 rectángulos de colores
- ✅ 3 líneas paralelas
- ✅ Texto visible

**Comando**:
```bash
./target/release/rydit-rs --gfx ejemplos_gfx/demo_shapes.rydit
```

### Test 2: Test Renderizado v0.9.0

**Qué verificar**:
- ✅ Grid 10x10 de círculos (100 círculos)
- ✅ 10 rectángulos animados
- ✅ 36 líneas en patrón radial
- ✅ Texto con FPS estables (~60 FPS)
- ✅ Círculo animado con sin/cos

**Comando**:
```bash
./target/release/rydit-rs --gfx demos/test_renderizado_v0.9.0.rydit
```

### Test 3: Demo Render Queue (Rust)

**Qué verificar**:
- ✅ 100 círculos en grid
- ✅ 50 rectángulos animados
- ✅ 36 líneas radiales
- ✅ Estadísticas en pantalla
- ✅ FPS estables

**Comando**:
```bash
./target/release/examples/demo_render_queue
```

---

## 🔍 DEBUGGING

### Si no se ve nada

1. **Verificar DISPLAY**:
   ```bash
   echo $DISPLAY  # Debe ser :0
   ```

2. **Verificar Termux-X11**:
   ```bash
   ps aux | grep X11  # Debe estar corriendo
   ```

3. **Reiniciar servidor X11**:
   ```bash
   pkill -f termux-x11
   termux-x11 :0 -ac &
   ```

### Si va lento

1. **Verificar FPS**:
   ```bash
   # El demo muestra FPS en pantalla
   # Debe ser ~60 FPS
   ```

2. **Reducir carga**:
   - Menos círculos/rectángulos
   - Ventana más pequeña (640x480 en vez de 800x600)

### Logs de Debug

```bash
# Ver log de rydit-gfx
cat /data/data/com.termux/files/home/shield-project/rydit_debug.log

# Ver output en terminal
./target/release/rydit-rs --gfx demo.rydit 2>&1 | tee log.txt
```

---

## 📈 MÉTRICAS ESPERADAS

### Rendimiento

| Test | FPS Esperados | Comandos/frame |
|------|---------------|----------------|
| demo_shapes.rydit | 60 FPS | ~15 |
| test_renderizado_v0.9.0.rydit | 60 FPS | ~186 |
| demo_render_queue | 60 FPS | 186 |

### Uso de Recursos

| Métrica | Valor |
|---------|-------|
| RAM | ~100 MB |
| CPU | 20-30% |
| GPU | Adreno 610 (Zink) |

---

## ✅ CHECKLIST DE VERIFICACIÓN

### Antes de Ejecutar
- [ ] Termux-X11 instalado y abierto
- [ ] DISPLAY=:0 configurado
- [ ] Driver zink activado
- [ ] Binario compilado (`target/release/rydit-rs`)

### Durante la Ejecución
- [ ] Ventana se abre (800x600)
- [ ] Formas visibles (círculos, rects, líneas)
- [ ] Texto legible
- [ ] FPS estables (~60)
- [ ] Sin crashes

### Después de la Ejecución
- [ ] Mensaje de salida visible
- [ ] Stats impresas en terminal
- [ ] Sin errores en log

---

## 🎮 CONTROLES

### Teclas Comunes

| Tecla | Acción |
|-------|--------|
| `ESC` | Salir |
| `P` | Pausa (si está implementado) |
| `SPACE` | Reiniciar (si está implementado) |

### En Snake Game

| Tecla | Acción |
|-------|--------|
| `↑` `→` `↓` `←` | Mover serpiente |
| `P` | Pausa |
| `SPACE` | Reiniciar |

---

## 📞 SOPORTE

### Logs para Debugging

```bash
# Recopilar logs
cat /data/data/com.termux/files/home/shield-project/rydit_debug.log > error.log
cargo run --gfx demo.rydit 2>&1 | tee run.log

# Adjuntar logs al reportar error
```

### Información del Sistema

```bash
# Versión de Android
getprop ro.build.version.release

# Modelo del dispositivo
getprop ro.product.model

# RAM disponible
free -h

# Driver GPU
echo $MESA_LOADER_DRIVER_OVERRIDE
```

---

<div align="center">

**🛡️ RyDit v0.9.0 - LISTO PARA EJECUTAR**

*Command Queue ✅ | Double Buffering ✅ | Platform Sync ✅*

**Prueba: `./test_gfx_v0.9.0.sh`**

</div>
