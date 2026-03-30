# 🤝 Contribuyendo a RyDit

¡Gracias por tu interés en contribuir a RyDit!

---

## 🎯 ¿Cómo Puedes Contribuir?

### 1. 📝 Documentación
- Mejorar el README
- Agregar ejemplos
- Corregir errores ortográficos
- Traducir a otros idiomas

### 2. 🧪 Tests
- Agregar nuevos tests
- Mejorar cobertura
- Reportar bugs encontrados

### 3. 🎨 Demos y Ejemplos
- Crear nuevos demos
- Mejorar demos existentes
- Compartir tus proyectos

### 4. 🐛 Bug Reports
- Reportar bugs en Issues
- Incluir pasos para reproducir
- Adjuntar screenshots si aplica

### 5. 💡 Features
- Sugerir nuevas features
- Discutir implementación
- Proveer casos de uso

---

## 🚀 Primeros Pasos

### 1. Fork el Repositorio
```bash
# En GitHub, click en "Fork"
# Luego clona tu fork
git clone https://github.com/TU_USUARIO/shield-project.git
cd shield-project
```

### 2. Crea una Rama
```bash
# Para features
git checkout -b feature/nueva-feature

# Para bugs
git checkout -b fix/correcion-bug

# Para documentación
git checkout -b docs/mejora-readme
```

### 3. Haz tus Cambios
```bash
# Edita los archivos
# Agrega demos en demos/
# Agrega tests en tests/
```

### 4. Commit
```bash
# Mensajes claros y descriptivos
git add .
git commit -m "feat: agregar nuevo demo de snake"
git commit -m "fix: corregir bug en concatenación"
git commit -m "docs: mejorar README con ejemplos"
```

### 5. Push
```bash
git push origin feature/nueva-feature
```

### 6. Pull Request
- Ve a tu fork en GitHub
- Click en "Pull Request"
- Describe tus cambios
- Espera review

---

## 📋 Estándares de Código

### Demos (.rydit)
```rydit
# Usar nombres descriptivos
dark.slot puntuacion = 0  # ✅
dark.slot p = 0           # ❌

# Comentarios útiles
# Calcular posición del jugador
dark.slot jugador_x = ancho / 2

# Evitar código muy largo en una línea
```

### Tests (Rust)
```rust
#[test]
fn test_nombre_descriptivo() {
    // Arrange
    let valor = 10;
    
    // Act
    let resultado = valor * 2;
    
    // Assert
    assert_eq!(resultado, 20);
}
```

### Documentación (Markdown)
```markdown
# Títulos claros
## Subsecciones
### Más específico

- Listas con guiones
1. Listas numeradas para pasos
`código inline` para comandos
```

---

## 🎨 Ideas para Contribuir

### Demos Nuevos
- [ ] Juego de plataformas simple
- [ ] Visualizador de partículas
- [ ] Reloj analógico
- [ ] Calculadora gráfica
- [ ] Juego de memoria
- [ ] Animación de rebote
- [ ] Simulación de gravedad

### Mejoras de Documentación
- [ ] Ejemplos para cada feature
- [ ] Tutorial paso a paso
- [ ] FAQ de problemas comunes
- [ ] Traducción a inglés
- [ ] Videos tutoriales

### Tests
- [ ] Tests para cada módulo stdlib
- [ ] Tests de integración
- [ ] Tests de rendimiento
- [ ] Tests en múltiples plataformas

---

## ❓ Preguntas Frecuentes

### ¿Necesito saber Rust?
**No.** Puedes contribuir con:
- Demos .rydit
- Documentación
- Tests de funcionalidad
- Reporte de bugs

### ¿Puedo compartir mis demos?
**¡Sí!** Crea un demo en `demos/` y haz un PR.

### ¿Cómo reporto un bug?
1. Ve a Issues en GitHub
2. Click en "New Issue"
3. Selecciona "Bug Report"
4. Describe el problema

### ¿Puedo sugerir features?
**¡Claro!** Crea un Issue con la etiqueta "enhancement".

---

## 📬 Contacto

¿Tienes preguntas? Abre un Issue o discute en:
- [GitHub Discussions](https://github.com/lapumlbb18-blip/shield-project/discussions)
- [MoureDev Discord](https://discord.gg/mouredev)

---

## 🙏 Gracias

¡Cada contribución cuenta! No importa si es pequeña o grande.

**Juntos hacemos RyDit mejor.** 🛡️

---

<p align="center">
  <em>Construido con ❤️ en Android/Termux</em>
</p>
