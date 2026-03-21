# 🤝 Contributing to RyDit / Contribuir a RyDit

**EN:** Contributions are welcome! This is an experimental project built entirely on mobile (Android/Termux).

**ES:** ¡Las contribuciones son bienvenidas! Este es un proyecto experimental construido completamente en móvil (Android/Termux).

---

## 📋 How to Contribute / Cómo Contribuir

### EN:
1. **Report bugs** - Open an issue with steps to reproduce
2. **Suggest features** - Open an issue describing the feature
3. **Improve docs** - PRs for documentation improvements welcome
4. **Add examples** - Share your RyDit scripts

### ES:
1. **Reportar bugs** - Abre un issue con pasos para reproducir
2. **Sugerir features** - Abre un issue describiendo el feature
3. **Mejorar docs** - PRs para mejoras de documentación bienvenidas
4. **Agregar ejemplos** - Comparte tus scripts de RyDit

---

## 🧪 Testing / Tests

```bash
# Run all tests / Ejecutar todos los tests
cargo test

# Run specific test / Ejecutar test específico
cargo test test_sumar
```

---

## 📁 Project Structure / Estructura del Proyecto

```
shield-project/
├── crates/
│   ├── lizer/          # Lexer + Parser
│   ├── blast-core/     # Executor + Memory
│   ├── rydit-gfx/      # Graphics (raylib)
│   ├── rydit-rs/       # Main binary
│   └── v-shield/       # Raylib wrapper
├── diagnostico/        # Session logs
├── LIBRO_RYDIT.md      # Language guide
└── *.rydit             # Example scripts
```

---

## 💬 Communication / Comunicación

- **Issues:** Bug reports, feature requests
- **PRs:** Code improvements, docs, examples
- **Discussions:** General questions, ideas

---

## 📜 Code of Conduct / Código de Conducta

**EN:** Be respectful. This is a learning project built by someone developing entirely on mobile.

**ES:** Sé respetuoso. Este es un proyecto de aprendizaje construido por alguien que desarrolla completamente en móvil.

---

## 🙏 Acknowledgments / Agradecimientos

Every contribution helps! This project proves that mobile development is viable.

¡Cada contribución ayuda! Este proyecto demuestra que el desarrollo en móvil es viable.
