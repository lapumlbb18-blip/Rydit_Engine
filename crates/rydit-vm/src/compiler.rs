// crates/rydit-vm/src/compiler.rs
// Compilador: AST → Bytecode
//
// Convierte AST en bytecode ejecutable por la VM.

use crate::opcodes::*;
use rydit_parser::{BinaryOp, Expr, Program, Stmt, UnaryOp};

/// Compilador de AST a Bytecode
pub struct Compiler {
    program: BytecodeProgram,
    scopes: Vec<Vec<String>>, // Scope stack para variables locales
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            program: BytecodeProgram::new(),
            scopes: Vec::new(),
        }
    }

    /// Compilar programa completo
    pub fn compile(mut self, program: &Program) -> Result<BytecodeProgram, String> {
        for stmt in &program.statements {
            self.compile_stmt(stmt)?;
        }

        // Agregar RETURN al final si no hay
        if self.program.instructions.is_empty()
            || !matches!(
                self.program.instructions.last(),
                Some(OpCode::Return | OpCode::ReturnValue)
            )
        {
            self.program.instructions.push(OpCode::Return);
        }

        Ok(self.program)
    }

    /// Compilar statement
    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Init => {
                // shield.init - no operation
                Ok(())
            }

            Stmt::Command(_cmd) => {
                // onda.core, ryprime - comandos especiales
                // Por ahora, nop
                self.program.instructions.push(OpCode::Nop);
                Ok(())
            }

            Stmt::Assign { name, value } => {
                // Compilar valor
                self.compile_expr(value)?;

                // Verificar si es variable local o global
                if let Some(idx) = self.find_local(name) {
                    self.program.instructions.push(OpCode::StoreLocal(idx));
                } else {
                    // Agregar como global
                    let idx = self.program.add_global(name.to_string());
                    self.program.instructions.push(OpCode::StoreGlobal(idx));
                }
                Ok(())
            }

            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                // Compilar condición
                self.compile_expr(condition)?;

                // Salto si falso al else
                let jump_to_else_idx = self.program.len();
                self.program.instructions.push(OpCode::JumpIfFalse(0)); // Placeholder

                // Compilar then_body
                for stmt in then_body {
                    self.compile_stmt(stmt)?;
                }

                // Si hay else, saltar sobre él
                if else_body.is_some() {
                    let jump_to_end_idx = self.program.len();
                    self.program.instructions.push(OpCode::Jump(0)); // Placeholder

                    // Fixear jump_to_else
                    let jump_target = self.program.len();
                    if let OpCode::JumpIfFalse(_) = &mut self.program.instructions[jump_to_else_idx]
                    {
                        self.program.instructions[jump_to_else_idx] =
                            OpCode::JumpIfFalse(jump_target);
                    }

                    // Compilar else_body
                    if let Some(else_stmts) = else_body {
                        for stmt in else_stmts {
                            self.compile_stmt(stmt)?;
                        }
                    }

                    // Fixear jump_to_end
                    let end_target = self.program.len();
                    if let OpCode::Jump(_) = &mut self.program.instructions[jump_to_end_idx] {
                        self.program.instructions[jump_to_end_idx] = OpCode::Jump(end_target);
                    }
                } else {
                    // No hay else, solo fixear jump_to_else
                    let jump_target = self.program.len();
                    if let OpCode::JumpIfFalse(_) = &mut self.program.instructions[jump_to_else_idx]
                    {
                        self.program.instructions[jump_to_else_idx] =
                            OpCode::JumpIfFalse(jump_target);
                    }
                }

                Ok(())
            }

            Stmt::While { condition, body } => {
                // Marca de inicio del loop
                let loop_start = self.program.len();

                // Compilar condición
                self.compile_expr(condition)?;

                // Salto si falso al final
                let jump_to_end_idx = self.program.len();
                self.program.instructions.push(OpCode::JumpIfFalse(0)); // Placeholder

                // Compilar body
                for stmt in body {
                    self.compile_stmt(stmt)?;
                }

                // Loop back al inicio
                self.program.instructions.push(OpCode::Loop(loop_start));

                // Fixear jump_to_end
                let jump_target = self.program.len();
                if let OpCode::JumpIfFalse(_) = &mut self.program.instructions[jump_to_end_idx] {
                    self.program.instructions[jump_to_end_idx] = OpCode::JumpIfFalse(jump_target);
                }

                Ok(())
            }

            Stmt::Block(stmts) => {
                // Nuevo scope
                self.scopes.push(Vec::new());

                for stmt in stmts {
                    self.compile_stmt(stmt)?;
                }

                // Pop scope
                self.scopes.pop();
                Ok(())
            }

            Stmt::Function { name, params, body } => {
                // Agregar función al pool
                let _func_idx = self.program.add_function(name.to_string());

                // Nuevo scope para parámetros
                self.scopes.push(Vec::new());

                // Agregar parámetros como locales
                for param in params {
                    if let Some(scope) = self.scopes.last_mut() {
                        scope.push(param.to_string());
                    }
                }

                // Compilar body
                for stmt in body {
                    self.compile_stmt(stmt)?;
                }

                // Return implícito
                self.program.instructions.push(OpCode::Return);

                // Pop scope
                self.scopes.pop();

                Ok(())
            }

            Stmt::Return(expr) => {
                if let Some(e) = expr {
                    self.compile_expr(e)?;
                    self.program.instructions.push(OpCode::ReturnValue);
                } else {
                    self.program.instructions.push(OpCode::Return);
                }
                Ok(())
            }

            Stmt::Expr(expr) => {
                self.compile_expr(expr)?;
                // Descartar resultado si no se usa
                self.program.instructions.push(OpCode::Pop);
                Ok(())
            }

            Stmt::Break => {
                // Break - por ahora nop (se necesita manejo de loops)
                self.program.instructions.push(OpCode::Nop);
                Ok(())
            }

            Stmt::ForEach {
                var,
                iterable,
                body,
            } => {
                // Compilar iterable (debe ser array)
                self.compile_expr(iterable)?;

                // Guardar longitud del array
                self.program.instructions.push(OpCode::Duplicate);
                // TODO: Implementar longitud de array

                // Nuevo scope para variable del loop
                self.scopes.push(Vec::new());
                if let Some(scope) = self.scopes.last_mut() {
                    scope.push(var.to_string());
                }

                // Compilar body
                for stmt in body {
                    self.compile_stmt(stmt)?;
                }

                // Pop scope
                self.scopes.pop();

                Ok(())
            }

            Stmt::Import { module, alias: _ } => {
                // Import - registrar módulo
                let _ = self.program.add_global(module.to_string());
                Ok(())
            }

            // Draw commands
            Stmt::DrawCircle {
                x,
                y,
                radio,
                color: _,
            } => {
                self.compile_expr(x)?;
                self.compile_expr(y)?;
                self.compile_expr(radio)?;
                self.program.instructions.push(OpCode::DrawCircle);
                Ok(())
            }

            Stmt::DrawRect {
                x,
                y,
                ancho,
                alto,
                color: _,
            } => {
                self.compile_expr(x)?;
                self.compile_expr(y)?;
                self.compile_expr(ancho)?;
                self.compile_expr(alto)?;
                self.program.instructions.push(OpCode::DrawRect);
                Ok(())
            }

            Stmt::DrawLine {
                x1,
                y1,
                x2,
                y2,
                color: _,
            } => {
                self.compile_expr(x1)?;
                self.compile_expr(y1)?;
                self.compile_expr(x2)?;
                self.compile_expr(y2)?;
                self.program.instructions.push(OpCode::DrawLine);
                Ok(())
            }

            Stmt::DrawText {
                texto,
                x,
                y,
                tamano,
                color: _,
            } => {
                self.compile_expr(texto)?;
                self.compile_expr(x)?;
                self.compile_expr(y)?;
                self.compile_expr(tamano)?;
                self.program.instructions.push(OpCode::DrawText);
                Ok(())
            }

            Stmt::DrawTriangle {
                v1_x,
                v1_y,
                v2_x,
                v2_y,
                v3_x,
                v3_y,
                color: _,
            } => {
                self.compile_expr(v1_x)?;
                self.compile_expr(v1_y)?;
                self.compile_expr(v2_x)?;
                self.compile_expr(v2_y)?;
                self.compile_expr(v3_x)?;
                self.compile_expr(v3_y)?;
                self.program.instructions.push(OpCode::DrawTriangle);
                Ok(())
            }

            Stmt::DrawRing {
                center_x,
                center_y,
                inner_radius,
                outer_radius,
                color: _,
            } => {
                self.compile_expr(center_x)?;
                self.compile_expr(center_y)?;
                self.compile_expr(inner_radius)?;
                self.compile_expr(outer_radius)?;
                self.program.instructions.push(OpCode::DrawRing);
                Ok(())
            }

            Stmt::DrawEllipse {
                center_x,
                center_y,
                radius_h,
                radius_v,
                color: _,
            } => {
                self.compile_expr(center_x)?;
                self.compile_expr(center_y)?;
                self.compile_expr(radius_h)?;
                self.compile_expr(radius_v)?;
                self.program.instructions.push(OpCode::DrawEllipse);
                Ok(())
            }

            Stmt::DrawRectangleLines {
                x,
                y,
                ancho,
                alto,
                color: _,
            } => {
                self.compile_expr(x)?;
                self.compile_expr(y)?;
                self.compile_expr(ancho)?;
                self.compile_expr(alto)?;
                self.program.instructions.push(OpCode::DrawRect); // Usar DrawRect por ahora
                Ok(())
            }

            Stmt::DrawLineThick {
                x1,
                y1,
                x2,
                y2,
                thick,
                color: _,
            } => {
                self.compile_expr(x1)?;
                self.compile_expr(y1)?;
                self.compile_expr(x2)?;
                self.compile_expr(y2)?;
                self.compile_expr(thick)?;
                // TODO: DrawLineThick opcode
                self.program.instructions.push(OpCode::Nop);
                Ok(())
            }

            Stmt::Call { callee, args } => {
                // Compilar argumentos
                for arg in args {
                    self.compile_expr(arg)?;
                }

                // Llamar función
                let func_idx = self.program.add_function(callee.to_string());
                self.program
                    .instructions
                    .push(OpCode::Call(func_idx, args.len() as u8));
                Ok(())
            }

            Stmt::IndexAssign {
                array,
                index,
                value,
            } => {
                // Compilar array
                if let Some(idx) = self.find_local(array) {
                    self.program.instructions.push(OpCode::LoadLocal(idx));
                } else {
                    let gidx = self.program.add_global(array.to_string());
                    self.program.instructions.push(OpCode::LoadGlobal(gidx));
                }

                // Compilar índice
                self.compile_expr(index)?;

                // Compilar valor
                self.compile_expr(value)?;

                // Setear elemento
                self.program.instructions.push(OpCode::SetIndex);
                Ok(())
            }
        }
    }

    /// Compilar expresión
    fn compile_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Num(n) => {
                let idx = self.program.add_constant_num(*n);
                self.program.instructions.push(OpCode::LoadConst(idx));
                Ok(())
            }

            Expr::Texto(s) => {
                let idx = self.program.add_constant_str(s.to_string());
                self.program.instructions.push(OpCode::LoadString(idx));
                Ok(())
            }

            Expr::Bool(b) => {
                self.program.instructions.push(OpCode::LoadBool(*b));
                Ok(())
            }

            Expr::Var(name) => {
                if let Some(idx) = self.find_local(name) {
                    self.program.instructions.push(OpCode::LoadLocal(idx));
                } else {
                    // Buscar o agregar como global
                    let idx = self
                        .program
                        .global_names
                        .iter()
                        .position(|n| n == *name)
                        .unwrap_or_else(|| self.program.add_global(name.to_string()));
                    self.program.instructions.push(OpCode::LoadGlobal(idx));
                }
                Ok(())
            }

            Expr::Binary { left, op, right } => {
                self.compile_expr(left)?;
                self.compile_expr(right)?;

                let opcode = match op {
                    BinaryOp::Suma | BinaryOp::MasIgual => OpCode::Add,
                    BinaryOp::Resta | BinaryOp::MenosIgual => OpCode::Subtract,
                    BinaryOp::Mult | BinaryOp::PorIgual => OpCode::Multiply,
                    BinaryOp::Div | BinaryOp::DivIgual => OpCode::Divide,
                    BinaryOp::Mayor => OpCode::Greater,
                    BinaryOp::Menor => OpCode::Less,
                    BinaryOp::Igual => OpCode::Equal,
                    BinaryOp::MayorIgual => OpCode::GreaterEqual,
                    BinaryOp::MenorIgual => OpCode::LessEqual,
                    BinaryOp::Diferente => OpCode::NotEqual,
                    BinaryOp::And => OpCode::And,
                    BinaryOp::Or => OpCode::Or,
                };

                self.program.instructions.push(opcode);
                Ok(())
            }

            Expr::Unary { op, expr } => {
                self.compile_expr(expr)?;

                match op {
                    UnaryOp::Not => {
                        self.program.instructions.push(OpCode::Not);
                    }
                    UnaryOp::Neg => {
                        // Negar: multiplicar por -1
                        let const_idx = self.program.add_constant_num(-1.0);
                        self.program.instructions.push(OpCode::LoadConst(const_idx));
                        self.program.instructions.push(OpCode::Multiply);
                    }
                }
                Ok(())
            }

            Expr::Call { callee, args } => {
                // Compilar argumentos
                for arg in args {
                    self.compile_expr(arg)?;
                }

                // Llamar función
                let func_idx = if let Expr::Var(name) = callee.as_ref() {
                    self.program.add_function(name.to_string())
                } else {
                    return Err("Callee must be a variable".to_string());
                };

                self.program
                    .instructions
                    .push(OpCode::Call(func_idx, args.len() as u8));
                Ok(())
            }

            Expr::Array(items) => {
                for item in items {
                    self.compile_expr(item)?;
                }
                self.program
                    .instructions
                    .push(OpCode::BuildArray(items.len() as u8));
                Ok(())
            }

            Expr::Index { array, index } => {
                self.compile_expr(array)?;
                self.compile_expr(index)?;
                self.program.instructions.push(OpCode::GetIndex);
                Ok(())
            }
        }
    }

    /// Buscar variable local en scopes
    fn find_local(&self, name: &str) -> Option<usize> {
        // Buscar desde el scope más interno hacia afuera
        for (scope_depth, scope) in self.scopes.iter().rev().enumerate() {
            if let Some(idx) = scope.iter().rposition(|n| n == name) {
                // Calcular índice local (scope_depth * max_scope_size + idx)
                // Simplificado: solo scope actual
                return Some(idx + scope_depth * 100);
            }
        }
        None
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Compilar source code a bytecode (convenience)
pub fn compile_source(source: &str) -> Result<BytecodeProgram, String> {
    use rydit_parser::Parser;

    let mut parser = Parser::from_source(source);
    let (program, errors) = parser.parse();

    if !errors.is_empty() {
        return Err(format!("Parse errors: {:?}", errors));
    }

    let compiler = Compiler::new();
    compiler.compile(&program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_simple() {
        let source = "dark.slot x = 100";
        let result = compile_source(source);
        assert!(result.is_ok());

        let program = result.unwrap();
        assert!(!program.is_empty());
    }

    #[test]
    fn test_compile_arithmetic() {
        let source = "dark.slot y = 10 + 5";
        let result = compile_source(source);
        assert!(result.is_ok());

        let program = result.unwrap();
        // Debería tener: LOAD_CONST, LOAD_CONST, ADD, STORE_GLOBAL
        assert!(program.instructions.len() >= 3);
    }

    #[test]
    fn test_compile_if() {
        let source = "onif x { voz \"hola\" }";
        let result = compile_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_while() {
        let source = "ryda x < 10 { voz x }";
        let result = compile_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compiler_new() {
        let compiler = Compiler::new();
        assert!(compiler.program.is_empty());
    }

    #[test]
    fn test_compile_empty() {
        let source = "shield.init";
        let result = compile_source(source);
        assert!(result.is_ok());
    }
}
