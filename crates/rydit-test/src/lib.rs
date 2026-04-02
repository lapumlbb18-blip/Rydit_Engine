//! RyDit Test Crate - Tests Simplificados
//!
//! **Objetivo**: Tests rápidos y aislados para Rybot + Lizer + RyditModule
//!
//! **Filosofía**:
//! - ✅ Tests rápidos (< 1 segundo cada uno)
//! - ✅ Sin dependencias gráficas pesadas
//! - ✅ Foco en lógica de negocio
//! - ✅ Fácil de ejecutar en CI/CD
//!
//! **Estructura**:
//! - `tests/nivel1_core_test.rs` - Tests de núcleo (lizer, blast, ryditmodule)
//! - `tests/nivel2_integration_test.rs` - Tests de integración (rybot, evaluator)
//! - `tests/nivel3_graphics_test.rs` - Tests gráficos (SOLO después de 1 + 2)

// Los tests están en el directorio `tests/` como integration tests
// No hay código en lib.rs para este crate
