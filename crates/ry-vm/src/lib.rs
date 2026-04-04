// crates/rydit-vm/src/lib.rs
// RyDit VM - Bytecode Compiler + Stack-Based VM
//
// Compilador de AST a bytecode y VM para ejecutarlo.

pub mod compiler;
pub mod opcodes;
pub mod vm;

pub use compiler::{compile_source, Compiler};
pub use opcodes::{BytecodeProgram, OpCode};
pub use vm::{VMValue, VM};

/// Versión del crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Compilar y ejecutar source code (convenience)
pub fn compile_and_run(source: &str) -> Result<VMValue, String> {
    let program = compile_source(source)?;
    let mut vm = VM::new();
    vm.load(program);
    vm.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_and_run() {
        let source = "dark.slot x = 100";
        let result = compile_and_run(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
