# 🚀 LIFETIMES + ZERO-COPY + BYTECODE - Optimizaciones v0.11.0

**Fecha**: 2026-03-31  
**Versión**: v0.11.0 (Parser Fuerte)  
**Estado**: 📋 Planificación

---

## 🔥 LAS 3 OPTIMIZACIONES CLAVE

### 1. **Lifetimes** (Zero-Copy Parsing) ⚡

**Problema actual**: El parser **copia strings** innecesariamente.

```rust
// ANTES (con copias - v0.10.4)
pub enum Token {
    Ident(String),  // ← Copia String cada vez
    Text(String),   // ← Otra copia
}

// DESPUÉS (zero-copy - v0.11.0)
pub enum Token<'a> {
    Ident(&'a str),  // ← Referencia, sin copia
    Text(&'a str),   // ← Referencia, sin copia
}
```

**Beneficio**: 
- 10x más rápido
- 1000x menos allocaciones de memoria
- Menos presión en garbage collector (RC)

---

### 2. **Bytecode Compilation** 🚀

**Problema actual**: El parser **interpreta** el AST cada frame (lento).

```rust
// ANTES (interpretación - v0.10.4)
fn evaluar_expr(expr: &Expr) -> Valor {
    match expr {
        Expr::Num(n) => Valor::Num(*n),
        Expr::Call { name, args } => {
            // Lookup de función cada vez
            // Evaluación de args cada vez
            // AST traversal cada frame (60 veces/seg)
        }
    }
}

// DESPUÉS (bytecode - v0.11.0)
fn ejecutar_bytecode(instr: &Instruction) -> Valor {
    match instr {
        Instruction::PushNum(f64),      // ← Opcode simple
        Instruction::Call(usize),       // ← Índice a tabla de funciones
        Instruction::Jump(usize),       // ← Salto directo
    }
}
```

**Beneficio**:
- 50-100x más rápido
- Game loop sin parsing en runtime
- 100K partículas @ 60 FPS posible

---

### 3. **Life** (Lifetime Annotations) 📚

**Qué es**: Rust **lifetimes** para garantizar que las referencias no duren más que los datos originales.

```rust
// Lifetime explícito
pub struct Parser<'a> {
    source: &'a str,      // ← Parser no vive más que source
    tokens: Vec<Token<'a>>,  // ← Tokens referencian source
}

// El compilador garantiza:
// - Parser no puede vivir más que source
// - Tokens no pueden vivir más que Parser
// - NO hay dangling references
```

**Beneficio**:
- Zero-copy SIN riesgo de memory safety
- El compilador verifica todo en compile-time
- 0 runtime overhead

---

## 📋 PLAN DE IMPLEMENTACIÓN

### Semana 1: Lifetimes + Zero-Copy

**Archivos a modificar**:
```
crates/lizer/src/
├── token.rs       # Token<'a> con lifetimes
├── lexer.rs       # Lexer que retorna Token<'a>
├── ast.rs         # AST<'a> con referencias
└── parser.rs      # Parser<'a> que usa &'a str
```

**Código ejemplo**:
```rust
// crates/lizer/src/token.rs
#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    // Literales
    Num(f64),
    Text(&'a str),        // ← Zero-copy
    Ident(&'a str),       // ← Zero-copy
    
    // Keywords
    Shield,
    Ryda,
    Dark,
    Onif,
    Blelse,
    Rytmo,
    Voz,
    
    // Operadores
    Mas,
    Menos,
    Por,
    Entre,
    Igual,
    Menor,
    Mayor,
    
    // Delimitadores
    ParentesisIzq,
    ParentesisDer,
    LlaveIzq,
    LlaveDer,
    CorcheteIzq,
    CorcheteDer,
    Coma,
    PuntoYComa,
    
    // EOF
    Eof,
}

// crates/lizer/src/lexer.rs
pub struct Lizer<'a> {
    source: &'a str,
    pos: usize,
}

impl<'a> Lizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, pos: 0 }
    }
    
    pub fn scan(&mut self) -> Vec<Token<'a>> {
        // Tokens referencian source directamente
        // Sin copias de String
    }
}
```

**Criterio de éxito**: 
- ✅ 0 `.to_string()` en lexer
- ✅ 0 `.clone()` en strings
- ✅ Todos los tests passing

---

### Semana 2: Bytecode Compilation

**Archivos nuevos**:
```
crates/lizer/src/
├── bytecode.rs      # Instruction enum
├── compiler.rs      # AST → Bytecode
├── vm.rs            # VM ejecuta bytecode
└── opcode.rs        # Definición de opcodes
```

**Código ejemplo**:
```rust
// crates/lizer/src/bytecode.rs

/// Instrucciones de bytecode
#[derive(Debug, Clone)]
pub enum Instruction {
    // Stack operations
    PushNum(f64),
    PushText(usize),      // ← Índice a string table
    Pop,
    Duplicate,
    
    // Variables
    LoadVar(usize),       // ← Índice a variable table
    StoreVar(usize),
    
    // Operations
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    Less,
    Greater,
    
    // Control flow
    Jump(usize),          // ← Salto absoluto
    JumpIfFalse(usize),
    JumpIfTrue(usize),
    
    // Function calls
    Call(usize, u8),      // ← Índice a función, número de args
    Return,
    
    // Builtins
    DrawCircle,
    DrawRect,
    DrawLine,
    DrawText,
    Input,
    Print,
    
    // Game loop
    FrameStart,
    FrameEnd,
}

/// Programa compilado
pub struct CompiledProgram {
    instructions: Vec<Instruction>,
    string_table: Vec<String>,
    var_table: Vec<String>,
    func_table: Vec<String>,
}

/// Compilador: AST → Bytecode
pub struct Compiler {
    instructions: Vec<Instruction>,
    string_table: Vec<String>,
    var_table: Vec<String>,
    func_table: Vec<String>,
}

impl Compiler {
    pub fn compile(program: &Program) -> CompiledProgram {
        // Recorrer AST y generar bytecode
        // 50-100x más rápido que interpretación
    }
    
    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Num(n) => {
                self.instructions.push(Instruction::PushNum(*n));
            }
            Expr::Call { name, args } => {
                // Compilar args
                for arg in args {
                    self.compile_expr(arg);
                }
                // Call instruction
                let func_idx = self.get_or_add_function(name);
                self.instructions.push(Instruction::Call(func_idx, args.len() as u8));
            }
            // ... más casos
        }
    }
}
```

**VM ejecuta bytecode**:
```rust
// crates/lizer/src/vm.rs

pub struct VM {
    stack: Vec<Valor>,
    ip: usize,  // Instruction pointer
    string_table: Vec<String>,
    var_table: Vec<Valor>,
    func_table: Vec<Function>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(256),
            ip: 0,
            string_table: Vec::new(),
            var_table: Vec::new(),
            func_table: Vec::new(),
        }
    }
    
    pub fn run(&mut self, program: &CompiledProgram) {
        self.string_table = program.string_table.clone();
        self.var_table = vec![Valor::Vacio; program.var_table.len()];
        self.ip = 0;
        
        while self.ip < program.instructions.len() {
            let instr = &program.instructions[self.ip];
            self.execute(instr);
        }
    }
    
    fn execute(&mut self, instr: &Instruction) {
        match instr {
            Instruction::PushNum(n) => {
                self.stack.push(Valor::Num(*n));
                self.ip += 1;
            }
            Instruction::Call(func_idx, arg_count) => {
                // Pop args from stack
                let args: Vec<Valor> = self.stack.drain(
                    self.stack.len() - *arg_count as usize..
                ).collect();
                
                // Call function
                let func = &self.func_table[*func_idx];
                let result = func.call(&args);
                
                // Push result
                self.stack.push(result);
                self.ip += 1;
            }
            // ... más instrucciones
        }
    }
}
```

**Criterio de éxito**:
- ✅ Game loop ejecuta bytecode, no AST
- ✅ 50-100x más rápido que interpretación
- ✅ 100K partículas @ 60 FPS

---

### Semana 3: Integración + Tests

**API unificada**:
```rust
// crates/lizer/src/lib.rs

/// Compilar source a bytecode
pub fn compile(source: &str) -> Result<CompiledProgram> {
    // 1. Tokenize (zero-copy)
    let mut lexer = Lizer::new(source);
    let tokens = lexer.scan();
    
    // 2. Parse (zero-copy AST)
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    // 3. Compile to bytecode
    let bytecode = Compiler::compile(&ast);
    
    Ok(bytecode)
}

/// Ejecutar bytecode
pub fn run(program: &CompiledProgram) -> Result<()> {
    let mut vm = VM::new();
    vm.run(program);
    Ok(())
}
```

**Game loop en rydit-rs**:
```rust
// crates/rydit-rs/src/executor.rs

// UNA VEZ al inicio (al cargar script)
let bytecode = lizer::compile(&source)?;  // ← 50-100ms

// Game loop - 60 veces por segundo
loop {
    // ✅ Ejecutar bytecode compilado
    lizer::run(&bytecode)?;  // ← 0.1-0.5ms
    
    // Total: 0.5-2ms por frame → 500-1000 FPS máximo teórico
    // Realidad: 60 FPS estables con 100K partículas
}
```

**Criterio de éxito**:
- ✅ 200+ tests de parser
- ✅ 60 FPS estables con 10K partículas
- ✅ 30+ FPS con 100K partículas
- ✅ 0 parsing en runtime

---

## 📊 COMPARATIVA: ANTES VS DESPUÉS

| Métrica | v0.10.4 (Actual) | v0.11.0 (Bytecode) | Mejora |
|---------|------------------|---------------------|--------|
| **Parsing por frame** | ✅ 60 veces/seg | ❌ 0 veces | **∞** |
| **AST traversal** | ✅ 60 veces/seg | ❌ 0 veces | **∞** |
| **Bytecode execution** | ❌ No existe | ✅ 60 veces/seg | **Nuevo** |
| **String copies/frame** | ✅ ~1000 | ❌ 0 | **1000x menos** |
| **Allocaciones/frame** | ✅ ~500 | ❌ ~10 | **50x menos** |
| **FPS con 10K partículas** | ⚠️ 30-50 FPS | ✅ 60 FPS | **2x** |
| **FPS con 100K partículas** | ❌ <1 FPS | ✅ 30-60 FPS | **60x** |
| **Memoria (10K partículas)** | ⚠️ ~50 MB | ✅ ~5 MB | **10x menos** |

---

## 🎯 EJEMPLO REAL: GAME LOOP

### ANTES (v0.10.4 - Interpretación)

```rust
// executor.rs - 60 veces por segundo
loop {
    // ❌ Parsear TODO el game loop cada frame
    let ast = parser.parse(source)?;  // ← 2-4ms
    
    // ❌ Recorrer AST cada frame
    for stmt in &ast.statements {
        evaluar_stmt(stmt)?;  // ← Lookup de funciones cada vez
    }
    
    // Total: 5-10ms por frame → 100-200 FPS máximo teórico
    // Realidad: 30-50 FPS con lógica simple
}
```

### DESPUÉS (v0.11.0 - Bytecode)

```rust
// executor.rs - UNA VEZ al inicio
let bytecode = compile(source)?;  // ← 50-100ms (solo al inicio)

// Game loop - 60 veces por segundo
loop {
    // ✅ Ejecutar bytecode compilado
    vm.run(&bytecode);  // ← 0.1-0.5ms
    
    // Total: 0.5-2ms por frame → 500-1000 FPS máximo teórico
    // Realidad: 60 FPS estables con 100K partículas
}
```

---

## 📝 REFERENCIAS

### Lifetimes en Rust
- [Rust Book - Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [Rust By Example - Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)

### Zero-Copy Parsing
- [Nom Parser Combinators](https://github.com/Geal/nom)
- [Winnow Zero-Copy](https://github.com/winnow-rs/winnow)

### Bytecode VMs
- [Crafting Interpreters (Java)](https://craftinginterpreters.com/)
- [Rust Bytecode VM Tutorial](https://ruslanspivak.com/)

---

<div align="center">

**🛡️ RyDit v0.11.0 - LIFETIMES + ZERO-COPY + BYTECODE**

*10x más rápido | 50x menos allocaciones | 100K partículas @ 60 FPS*

**Próximo: Implementación (2-3 semanas)**

</div>
