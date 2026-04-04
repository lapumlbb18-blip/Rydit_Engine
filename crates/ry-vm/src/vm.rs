// crates/rydit-vm/src/vm.rs
// VM Stack-Based para RyDit
//
// Ejecuta bytecode compilado.

use crate::opcodes::*;
use std::collections::HashMap;

/// Valor que puede manejar la VM
#[derive(Debug, Clone, PartialEq)]
pub enum VMValue {
    Num(f64),
    Texto(String),
    Bool(bool),
    Array(Vec<VMValue>),
    Vacio,
}

impl std::fmt::Display for VMValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VMValue::Num(n) => write!(f, "{}", n),
            VMValue::Texto(s) => write!(f, "{}", s),
            VMValue::Bool(b) => write!(f, "{}", b),
            VMValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| format!("{}", v)).collect();
                write!(f, "[{}]", items.join(", "))
            }
            VMValue::Vacio => write!(f, "vacío"),
        }
    }
}

/// Call Frame para funciones
#[derive(Debug)]
struct CallFrame {
    /// Instruction pointer
    ip: usize,

    /// Base del stack para este frame
    stack_base: usize,
}

/// VM Stack-Based
pub struct VM {
    /// Stack de valores
    stack: Vec<VMValue>,

    /// Variables globales
    globals: HashMap<String, VMValue>,

    /// Call frames
    frames: Vec<CallFrame>,

    /// Instruction pointer actual
    ip: usize,

    /// Programa actual
    program: Option<BytecodeProgram>,

    /// Callback para draw commands
    pub draw_callback: Option<Box<dyn FnMut(&str, Vec<f64>)>>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            globals: HashMap::new(),
            frames: Vec::new(),
            ip: 0,
            program: None,
            draw_callback: None,
        }
    }

    /// Cargar programa bytecode
    pub fn load(&mut self, program: BytecodeProgram) {
        self.program = Some(program);
        self.ip = 0;
        self.stack.clear();
        self.frames.clear();
    }

    /// Ejecutar programa completo
    pub fn run(&mut self) -> Result<VMValue, String> {
        if self.program.is_none() {
            return Err("No program loaded".to_string());
        }

        self.ip = 0;
        self.frames.push(CallFrame {
            ip: 0,
            stack_base: 0,
        });

        while self.ip < self.program.as_ref().unwrap().instructions.len() {
            match self.execute_instruction()? {
                ExecutionResult::Continue => {}
                ExecutionResult::Return(value) => return Ok(value),
            }
        }

        Ok(VMValue::Vacio)
    }

    /// Ejecutar una instrucción
    fn execute_instruction(&mut self) -> Result<ExecutionResult, String> {
        let program = self.program.as_ref().unwrap();
        let instruction = &program.instructions[self.ip];

        match instruction {
            OpCode::LoadConst(idx) => {
                let value = program.constants_num[*idx];
                self.stack.push(VMValue::Num(value));
                self.ip += 1;
            }

            OpCode::LoadString(idx) => {
                let value = program.constants_str[*idx].clone();
                self.stack.push(VMValue::Texto(value));
                self.ip += 1;
            }

            OpCode::LoadBool(b) => {
                self.stack.push(VMValue::Bool(*b));
                self.ip += 1;
            }

            OpCode::LoadGlobal(idx) => {
                let name = &program.global_names[*idx];
                let value = self.globals.get(name).cloned().unwrap_or(VMValue::Vacio);
                self.stack.push(value);
                self.ip += 1;
            }

            OpCode::StoreGlobal(idx) => {
                let name = &program.global_names[*idx];
                let value = self.stack.pop().ok_or("Stack underflow")?;
                self.globals.insert(name.clone(), value);
                self.ip += 1;
            }

            OpCode::LoadLocal(idx) => {
                // Buscar en stack relativo al frame actual
                if let Some(frame) = self.frames.last() {
                    let stack_idx = frame.stack_base + idx;
                    if stack_idx < self.stack.len() {
                        let value = self.stack[stack_idx].clone();
                        self.stack.push(value);
                    } else {
                        return Err(format!("Local variable index out of bounds: {}", stack_idx));
                    }
                }
                self.ip += 1;
            }

            OpCode::StoreLocal(idx) => {
                if let Some(frame) = self.frames.last() {
                    let stack_idx = frame.stack_base + idx;
                    let value = self.stack.pop().ok_or("Stack underflow")?;
                    if stack_idx < self.stack.len() {
                        self.stack[stack_idx] = value;
                    } else {
                        self.stack.push(value);
                    }
                }
                self.ip += 1;
            }

            OpCode::Add => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;

                let result = match (a, b) {
                    (VMValue::Num(x), VMValue::Num(y)) => VMValue::Num(x + y),
                    (VMValue::Texto(x), VMValue::Texto(y)) => VMValue::Texto(x + &y),
                    _ => return Err("Type mismatch for +".to_string()),
                };

                self.stack.push(result);
                self.ip += 1;
            }

            OpCode::Subtract => {
                let b = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let a = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                self.stack.push(VMValue::Num(a - b));
                self.ip += 1;
            }

            OpCode::Multiply => {
                let b = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let a = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                self.stack.push(VMValue::Num(a * b));
                self.ip += 1;
            }

            OpCode::Divide => {
                let b = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let a = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                if b == 0.0 {
                    return Err("Division by zero".to_string());
                }
                self.stack.push(VMValue::Num(a / b));
                self.ip += 1;
            }

            OpCode::Equal => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = a == b;
                self.stack.push(VMValue::Bool(result));
                self.ip += 1;
            }

            OpCode::NotEqual => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = a != b;
                self.stack.push(VMValue::Bool(result));
                self.ip += 1;
            }

            OpCode::Greater => {
                let b = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let a = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                self.stack.push(VMValue::Bool(a > b));
                self.ip += 1;
            }

            OpCode::Less => {
                let b = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let a = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                self.stack.push(VMValue::Bool(a < b));
                self.ip += 1;
            }

            OpCode::GreaterEqual => {
                let b = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let a = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                self.stack.push(VMValue::Bool(a >= b));
                self.ip += 1;
            }

            OpCode::LessEqual => {
                let b = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let a = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                self.stack.push(VMValue::Bool(a <= b));
                self.ip += 1;
            }

            OpCode::And => {
                let b = self.stack.pop().ok_or("Stack underflow")?.as_bool()?;
                let a = self.stack.pop().ok_or("Stack underflow")?.as_bool()?;
                self.stack.push(VMValue::Bool(a && b));
                self.ip += 1;
            }

            OpCode::Or => {
                let b = self.stack.pop().ok_or("Stack underflow")?.as_bool()?;
                let a = self.stack.pop().ok_or("Stack underflow")?.as_bool()?;
                self.stack.push(VMValue::Bool(a || b));
                self.ip += 1;
            }

            OpCode::Not => {
                let a = self.stack.pop().ok_or("Stack underflow")?.as_bool()?;
                self.stack.push(VMValue::Bool(!a));
                self.ip += 1;
            }

            OpCode::Jump(addr) => {
                self.ip = *addr;
            }

            OpCode::JumpIfFalse(addr) => {
                let cond = self.stack.pop().ok_or("Stack underflow")?.as_bool()?;
                if !cond {
                    self.ip = *addr;
                } else {
                    self.ip += 1;
                }
            }

            OpCode::JumpIfTrue(addr) => {
                let cond = self.stack.pop().ok_or("Stack underflow")?.as_bool()?;
                if cond {
                    self.ip = *addr;
                } else {
                    self.ip += 1;
                }
            }

            OpCode::Loop(addr) => {
                self.ip = *addr;
            }

            OpCode::Call(idx, arity) => {
                let _func_name = &program.function_names[*idx];
                let _arity = *arity;

                // Por ahora, funciones nativas simples
                // TODO: Implementar llamada a funciones definidas por usuario
                self.ip += 1;
            }

            OpCode::Return => {
                self.frames.pop();
                if self.frames.is_empty() {
                    return Ok(ExecutionResult::Return(VMValue::Vacio));
                }
                self.ip = self.frames.last().unwrap().ip;
            }

            OpCode::ReturnValue => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                self.frames.pop();
                if self.frames.is_empty() {
                    return Ok(ExecutionResult::Return(value));
                }
                self.ip = self.frames.last().unwrap().ip;
            }

            OpCode::BuildArray(size) => {
                let mut items = Vec::with_capacity(*size as usize);
                for _ in 0..*size {
                    items.push(self.stack.pop().ok_or("Stack underflow")?);
                }
                items.reverse(); // Mantener orden
                self.stack.push(VMValue::Array(items));
                self.ip += 1;
            }

            OpCode::GetIndex => {
                let index = self.stack.pop().ok_or("Stack underflow")?.as_num()? as usize;
                let array = self.stack.pop().ok_or("Stack underflow")?;

                if let VMValue::Array(arr) = array {
                    if index < arr.len() {
                        self.stack.push(arr[index].clone());
                    } else {
                        return Err(format!("Index out of bounds: {}", index));
                    }
                } else {
                    return Err("Cannot index non-array".to_string());
                }
                self.ip += 1;
            }

            OpCode::SetIndex => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                let index = self.stack.pop().ok_or("Stack underflow")?.as_num()? as usize;
                let mut array = self.stack.pop().ok_or("Stack underflow")?;

                if let VMValue::Array(ref mut arr) = array {
                    if index < arr.len() {
                        arr[index] = value;
                    } else {
                        return Err(format!("Index out of bounds: {}", index));
                    }
                } else {
                    return Err("Cannot set index on non-array".to_string());
                }

                self.stack.push(array);
                self.ip += 1;
            }

            OpCode::DrawCircle => {
                // Pop parámetros: radio, y, x
                let radio = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let y = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let x = self.stack.pop().ok_or("Stack underflow")?.as_num()?;

                if let Some(ref mut callback) = self.draw_callback {
                    callback("circle", vec![x, y, radio]);
                }

                self.ip += 1;
            }

            OpCode::DrawRect => {
                let alto = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let ancho = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let y = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let x = self.stack.pop().ok_or("Stack underflow")?.as_num()?;

                if let Some(ref mut callback) = self.draw_callback {
                    callback("rect", vec![x, y, ancho, alto]);
                }

                self.ip += 1;
            }

            OpCode::DrawLine => {
                let y2 = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let x2 = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let y1 = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let x1 = self.stack.pop().ok_or("Stack underflow")?.as_num()?;

                if let Some(ref mut callback) = self.draw_callback {
                    callback("line", vec![x1, y1, x2, y2]);
                }

                self.ip += 1;
            }

            OpCode::DrawText => {
                let tamano = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let y = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let x = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let texto = self.stack.pop().ok_or("Stack underflow")?;

                if let Some(ref mut callback) = self.draw_callback {
                    let texto_str = match texto {
                        VMValue::Texto(s) => s,
                        _ => format!("{}", texto),
                    };
                    callback("text", vec![x, y, tamano]);
                    let _ = texto_str; // Usado en callback real
                }

                self.ip += 1;
            }

            OpCode::DrawTriangle => {
                let v3_y = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let v3_x = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let v2_y = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let v2_x = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let v1_y = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let v1_x = self.stack.pop().ok_or("Stack underflow")?.as_num()?;

                if let Some(ref mut callback) = self.draw_callback {
                    callback("triangle", vec![v1_x, v1_y, v2_x, v2_y, v3_x, v3_y]);
                }

                self.ip += 1;
            }

            OpCode::DrawRing => {
                let outer = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let inner = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let y = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let x = self.stack.pop().ok_or("Stack underflow")?.as_num()?;

                if let Some(ref mut callback) = self.draw_callback {
                    callback("ring", vec![x, y, inner, outer]);
                }

                self.ip += 1;
            }

            OpCode::DrawEllipse => {
                let radius_v = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let radius_h = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let y = self.stack.pop().ok_or("Stack underflow")?.as_num()?;
                let x = self.stack.pop().ok_or("Stack underflow")?.as_num()?;

                if let Some(ref mut callback) = self.draw_callback {
                    callback("ellipse", vec![x, y, radius_h, radius_v]);
                }

                self.ip += 1;
            }

            OpCode::Print => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                println!("{}", value);
                self.ip += 1;
            }

            OpCode::Pop => {
                self.stack.pop();
                self.ip += 1;
            }

            OpCode::Duplicate => {
                if let Some(value) = self.stack.last() {
                    self.stack.push(value.clone());
                } else {
                    return Err("Cannot duplicate empty stack".to_string());
                }
                self.ip += 1;
            }

            OpCode::Nop => {
                self.ip += 1;
            }
        }

        Ok(ExecutionResult::Continue)
    }

    /// Obtener valor de variable global
    pub fn get_global(&self, name: &str) -> Option<VMValue> {
        self.globals.get(name).cloned()
    }

    /// Setear variable global
    pub fn set_global(&mut self, name: &str, value: VMValue) {
        self.globals.insert(name.to_string(), value);
    }

    /// Obtener tamaño del stack
    pub fn stack_size(&self) -> usize {
        self.stack.len()
    }

    /// Resetear VM
    pub fn reset(&mut self) {
        self.stack.clear();
        self.globals.clear();
        self.frames.clear();
        self.ip = 0;
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

/// Resultado de ejecución de instrucción
enum ExecutionResult {
    Continue,
    Return(VMValue),
}

// Extensiones para VMValue
impl VMValue {
    fn as_num(&self) -> Result<f64, String> {
        match self {
            VMValue::Num(n) => Ok(*n),
            _ => Err(format!("Expected number, got {:?}", self)),
        }
    }

    fn as_bool(&self) -> Result<bool, String> {
        match self {
            VMValue::Bool(b) => Ok(*b),
            _ => Err(format!("Expected bool, got {:?}", self)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::compile_source;

    #[test]
    fn test_vm_new() {
        let vm = VM::new();
        assert_eq!(vm.stack_size(), 0);
    }

    #[test]
    fn test_vm_run_simple() {
        let source = "dark.slot x = 100";
        let program = compile_source(source).unwrap();

        let mut vm = VM::new();
        vm.load(program);
        let result = vm.run();

        assert!(result.is_ok());
        assert_eq!(vm.get_global("x"), Some(VMValue::Num(100.0)));
    }

    #[test]
    fn test_vm_arithmetic() {
        let source = "dark.slot y = 10 + 5";
        let program = compile_source(source).unwrap();

        let mut vm = VM::new();
        vm.load(program);
        let result = vm.run();

        assert!(result.is_ok());
        assert_eq!(vm.get_global("y"), Some(VMValue::Num(15.0)));
    }

    #[test]
    fn test_vm_value_display() {
        let value = VMValue::Num(42.0);
        assert_eq!(format!("{}", value), "42");

        let value = VMValue::Texto("hola".to_string());
        assert_eq!(format!("{}", value), "hola");
    }

    #[test]
    fn test_vm_reset() {
        let mut vm = VM::new();
        vm.set_global("x", VMValue::Num(100.0));
        vm.reset();
        assert_eq!(vm.get_global("x"), None);
    }
}
