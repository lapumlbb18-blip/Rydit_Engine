// crates/rydit-vm/src/opcodes.rs
// Bytecode OpCodes para RyDit VM
//
// Instrucciones de bytecode para la VM stack-based.

use std::fmt;

/// Instrucciones de bytecode (OpCodes)
#[derive(Debug, Clone)]
pub enum OpCode {
    // === CONSTANTES ===
    /// Cargar constante numérica del pool
    LoadConst(usize),

    /// Cargar constante de texto del pool
    LoadString(usize),

    /// Cargar booleano
    LoadBool(bool),

    // === VARIABLES ===
    /// Cargar variable global
    LoadGlobal(usize),

    /// Guardar variable global
    StoreGlobal(usize),

    /// Cargar variable local
    LoadLocal(usize),

    /// Guardar variable local
    StoreLocal(usize),

    // === OPERACIONES ARITMÉTICAS ===
    /// Suma: a + b
    Add,

    /// Resta: a - b
    Subtract,

    /// Multiplicación: a * b
    Multiply,

    /// División: a / b
    Divide,

    // === OPERACIONES DE COMPARACIÓN ===
    /// Igualdad: a == b
    Equal,

    /// Diferente: a != b
    NotEqual,

    /// Mayor que: a > b
    Greater,

    /// Menor que: a < b
    Less,

    /// Mayor o igual: a >= b
    GreaterEqual,

    /// Menor o igual: a <= b
    LessEqual,

    // === OPERACIONES LÓGICAS ===
    /// AND lógico: a and b
    And,

    /// OR lógico: a or b
    Or,

    /// NOT lógico: not a
    Not,

    // === CONTROL DE FLUJO ===
    /// Salto incondicional
    Jump(usize),

    /// Salto si falso
    JumpIfFalse(usize),

    /// Salto si verdadero
    JumpIfTrue(usize),

    /// Fin de loop (volver al inicio)
    Loop(usize),

    // === FUNCIONES ===
    /// Llamar función
    Call(usize, u8), // (índice nombre, arity)

    /// Retorno de función
    Return,

    /// Retorno con valor
    ReturnValue,

    // === ESTRUCTURAS ===
    /// Crear array
    BuildArray(u8), // número de elementos

    /// Indexar array
    GetIndex,

    /// Setear elemento de array
    SetIndex,

    // === DRAW COMMANDS ===
    /// Dibujar círculo
    DrawCircle,

    /// Dibujar rectángulo
    DrawRect,

    /// Dibujar línea
    DrawLine,

    /// Dibujar texto
    DrawText,

    /// Dibujar triángulo
    DrawTriangle,

    /// Dibujar anillo
    DrawRing,

    /// Dibujar elipse
    DrawEllipse,

    // === ESPECIALES ===
    /// Nop (no operation)
    Nop,

    /// Pop (descartar tope del stack)
    Pop,

    /// Duplicar tope del stack
    Duplicate,

    /// Print/voz
    Print,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::LoadConst(idx) => write!(f, "LOAD_CONST {}", idx),
            OpCode::LoadString(idx) => write!(f, "LOAD_STRING {}", idx),
            OpCode::LoadBool(b) => write!(f, "LOAD_BOOL {}", b),
            OpCode::LoadGlobal(idx) => write!(f, "LOAD_GLOBAL {}", idx),
            OpCode::StoreGlobal(idx) => write!(f, "STORE_GLOBAL {}", idx),
            OpCode::LoadLocal(idx) => write!(f, "LOAD_LOCAL {}", idx),
            OpCode::StoreLocal(idx) => write!(f, "STORE_LOCAL {}", idx),
            OpCode::Add => write!(f, "ADD"),
            OpCode::Subtract => write!(f, "SUBTRACT"),
            OpCode::Multiply => write!(f, "MULTIPLY"),
            OpCode::Divide => write!(f, "DIVIDE"),
            OpCode::Equal => write!(f, "EQUAL"),
            OpCode::NotEqual => write!(f, "NOT_EQUAL"),
            OpCode::Greater => write!(f, "GREATER"),
            OpCode::Less => write!(f, "LESS"),
            OpCode::GreaterEqual => write!(f, "GREATER_EQUAL"),
            OpCode::LessEqual => write!(f, "LESS_EQUAL"),
            OpCode::And => write!(f, "AND"),
            OpCode::Or => write!(f, "OR"),
            OpCode::Not => write!(f, "NOT"),
            OpCode::Jump(addr) => write!(f, "JUMP {}", addr),
            OpCode::JumpIfFalse(addr) => write!(f, "JUMP_IF_FALSE {}", addr),
            OpCode::JumpIfTrue(addr) => write!(f, "JUMP_IF_TRUE {}", addr),
            OpCode::Loop(addr) => write!(f, "LOOP {}", addr),
            OpCode::Call(idx, arity) => write!(f, "CALL {} {}", idx, arity),
            OpCode::Return => write!(f, "RETURN"),
            OpCode::ReturnValue => write!(f, "RETURN_VALUE"),
            OpCode::BuildArray(size) => write!(f, "BUILD_ARRAY {}", size),
            OpCode::GetIndex => write!(f, "GET_INDEX"),
            OpCode::SetIndex => write!(f, "SET_INDEX"),
            OpCode::DrawCircle => write!(f, "DRAW_CIRCLE"),
            OpCode::DrawRect => write!(f, "DRAW_RECT"),
            OpCode::DrawLine => write!(f, "DRAW_LINE"),
            OpCode::DrawText => write!(f, "DRAW_TEXT"),
            OpCode::DrawTriangle => write!(f, "DRAW_TRIANGLE"),
            OpCode::DrawRing => write!(f, "DRAW_RING"),
            OpCode::DrawEllipse => write!(f, "DRAW_ELLIPSE"),
            OpCode::Nop => write!(f, "NOP"),
            OpCode::Pop => write!(f, "POP"),
            OpCode::Duplicate => write!(f, "DUPLICATE"),
            OpCode::Print => write!(f, "PRINT"),
        }
    }
}

/// Programa bytecode compilado
#[derive(Debug, Clone)]
pub struct BytecodeProgram {
    /// Instrucciones
    pub instructions: Vec<OpCode>,

    /// Pool de constantes numéricas
    pub constants_num: Vec<f64>,

    /// Pool de constantes de texto
    pub constants_str: Vec<String>,

    /// Nombres de variables globales
    pub global_names: Vec<String>,

    /// Nombres de funciones
    pub function_names: Vec<String>,
}

impl BytecodeProgram {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants_num: Vec::new(),
            constants_str: Vec::new(),
            global_names: Vec::new(),
            function_names: Vec::new(),
        }
    }

    /// Agregar constante numérica y retornar índice
    pub fn add_constant_num(&mut self, value: f64) -> usize {
        let idx = self.constants_num.len();
        self.constants_num.push(value);
        idx
    }

    /// Agregar constante de texto y retornar índice
    pub fn add_constant_str(&mut self, value: String) -> usize {
        let idx = self.constants_str.len();
        self.constants_str.push(value);
        idx
    }

    /// Agregar variable global y retornar índice
    pub fn add_global(&mut self, name: String) -> usize {
        let idx = self.global_names.len();
        self.global_names.push(name);
        idx
    }

    /// Agregar función y retornar índice
    pub fn add_function(&mut self, name: String) -> usize {
        let idx = self.function_names.len();
        self.function_names.push(name);
        idx
    }

    /// Obtener longitud de instrucciones
    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    /// Verificar si está vacío
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    /// Disassemblar bytecode (para debug)
    pub fn disassemble(&self) -> String {
        let mut output = String::new();
        output.push_str("=== BYTECODE PROGRAM ===\n");
        output.push_str(&format!("Instructions: {}\n", self.len()));
        output.push_str(&format!("Constants (num): {}\n", self.constants_num.len()));
        output.push_str(&format!("Constants (str): {}\n", self.constants_str.len()));
        output.push_str(&format!("Globals: {}\n", self.global_names.len()));
        output.push_str(&format!("Functions: {}\n\n", self.function_names.len()));

        for (addr, opcode) in self.instructions.iter().enumerate() {
            output.push_str(&format!("{:04}: {}\n", addr, opcode));
        }

        output
    }
}

impl Default for BytecodeProgram {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_display() {
        assert_eq!(format!("{}", OpCode::Add), "ADD");
        assert!(format!("{}", OpCode::LoadConst(5)).contains("5"));
    }

    #[test]
    fn test_bytecode_program_new() {
        let program = BytecodeProgram::new();
        assert!(program.is_empty());
        assert_eq!(program.len(), 0);
    }

    #[test]
    fn test_add_constant_num() {
        let mut program = BytecodeProgram::new();
        let idx1 = program.add_constant_num(100.0);
        let idx2 = program.add_constant_num(200.0);
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
        assert_eq!(program.constants_num, vec![100.0, 200.0]);
    }

    #[test]
    fn test_add_constant_str() {
        let mut program = BytecodeProgram::new();
        let idx = program.add_constant_str("hola".to_string());
        assert_eq!(idx, 0);
        assert_eq!(program.constants_str, vec!["hola"]);
    }

    #[test]
    fn test_add_global() {
        let mut program = BytecodeProgram::new();
        let idx = program.add_global("x".to_string());
        assert_eq!(idx, 0);
        assert_eq!(program.global_names, vec!["x"]);
    }

    #[test]
    fn test_disassemble() {
        let mut program = BytecodeProgram::new();
        program.instructions.push(OpCode::LoadConst(0));
        program.instructions.push(OpCode::Add);
        program.add_constant_num(100.0);

        let disasm = program.disassemble();
        assert!(disasm.contains("BYTECODE PROGRAM"));
        assert!(disasm.contains("LOAD_CONST"));
        assert!(disasm.contains("ADD"));
    }
}
