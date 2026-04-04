// crates/rydit-parser/src/ast.rs
// AST (Abstract Syntax Tree) typed con validación
//
// AST robusto con tipos específicos para cada categoría de expresión.

use std::fmt;

/// Programa completo (raíz del AST)
#[derive(Debug, Clone)]
pub struct Program<'a> {
    pub statements: Vec<Stmt<'a>>,
}

impl<'a> Program<'a> {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }

    pub fn len(&self) -> usize {
        self.statements.len()
    }
}

impl<'a> Default for Program<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Expresiones en RyDit
///
/// AST typed: cada variante tiene un propósito específico.
#[derive(Debug, Clone)]
pub enum Expr<'a> {
    /// Literal numérico: 100, 0.5, -5
    Num(f64),

    /// Literal de texto: "hola", 'mundo'
    Texto(&'a str),

    /// Variable: x, jugador, delta.flow
    Var(&'a str),

    /// Booleano: true, false
    Bool(bool),

    /// Array literal: [1, 2, 3]
    Array(Vec<Expr<'a>>),

    /// Indexación: lista[0], array[i]
    Index {
        array: Box<Expr<'a>>,
        index: Box<Expr<'a>>,
    },

    /// Operación binaria: a + b, x * y
    Binary {
        left: Box<Expr<'a>>,
        op: BinaryOp,
        right: Box<Expr<'a>>,
    },

    /// Operación unaria: -x, not true
    Unary { op: UnaryOp, expr: Box<Expr<'a>> },

    /// Llamada a función: tecla_presionada("arrow_up")
    Call {
        callee: Box<Expr<'a>>,
        args: Vec<Expr<'a>>,
    },
}

/// Operadores binarios
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    // Aritméticos
    Suma,       // +
    Resta,      // -
    Mult,       // *
    Div,        // /
    MasIgual,   // +=
    MenosIgual, // -=
    PorIgual,   // *=
    DivIgual,   // /=

    // Comparación
    Mayor,      // >
    Menor,      // <
    Igual,      // ==
    MayorIgual, // >=
    MenorIgual, // <=
    Diferente,  // !=

    // Lógicos
    And,
    Or,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinaryOp::Suma => "+",
            BinaryOp::Resta => "-",
            BinaryOp::Mult => "*",
            BinaryOp::Div => "/",
            BinaryOp::MasIgual => "+=",
            BinaryOp::MenosIgual => "-=",
            BinaryOp::PorIgual => "*=",
            BinaryOp::DivIgual => "/=",
            BinaryOp::Mayor => ">",
            BinaryOp::Menor => "<",
            BinaryOp::Igual => "==",
            BinaryOp::MayorIgual => ">=",
            BinaryOp::MenorIgual => "<=",
            BinaryOp::Diferente => "!=",
            BinaryOp::And => "and",
            BinaryOp::Or => "or",
        };
        write!(f, "{}", s)
    }
}

/// Operadores unarios
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Not, // not
    Neg, // -
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UnaryOp::Not => "not",
            UnaryOp::Neg => "-",
        };
        write!(f, "{}", s)
    }
}

/// Statements (declaraciones) en RyDit
#[derive(Debug, Clone)]
pub enum Stmt<'a> {
    /// shield.init
    Init,

    /// Comandos: onda.core, ryprime
    Command(&'a str),

    /// Asignación: dark.slot x = 100
    Assign { name: &'a str, value: Expr<'a> },

    /// Index assign: lista[0] = 5
    IndexAssign {
        array: &'a str,
        index: Expr<'a>,
        value: Expr<'a>,
    },

    /// Condicional: onif condition { ... } blelse { ... }
    If {
        condition: Expr<'a>,
        then_body: Vec<Stmt<'a>>,
        else_body: Option<Vec<Stmt<'a>>>,
    },

    /// While: ryda condition { ... }
    While {
        condition: Expr<'a>,
        body: Vec<Stmt<'a>>,
    },

    /// Bloque: { ... }
    Block(Vec<Stmt<'a>>),

    /// Función: rytmo nombre(params) { ... }
    Function {
        name: &'a str,
        params: Vec<&'a str>,
        body: Vec<Stmt<'a>>,
    },

    /// Llamada a función como statement: nombre(args)
    Call {
        callee: &'a str,
        args: Vec<Expr<'a>>,
    },

    /// Return: return valor
    Return(Option<Expr<'a>>),

    /// Expresión como statement
    Expr(Expr<'a>),

    /// ForEach: cada x en lista { ... }
    ForEach {
        var: &'a str,
        iterable: Expr<'a>,
        body: Vec<Stmt<'a>>,
    },

    /// Break: salir de loop
    Break,

    /// Import: import modulo [as alias]
    Import {
        module: &'a str,
        alias: Option<&'a str>,
    },

    /// Draw commands
    DrawCircle {
        x: Expr<'a>,
        y: Expr<'a>,
        radio: Expr<'a>,
        color: &'a str,
    },

    DrawRect {
        x: Expr<'a>,
        y: Expr<'a>,
        ancho: Expr<'a>,
        alto: Expr<'a>,
        color: &'a str,
    },

    DrawLine {
        x1: Expr<'a>,
        y1: Expr<'a>,
        x2: Expr<'a>,
        y2: Expr<'a>,
        color: &'a str,
    },

    DrawText {
        texto: Expr<'a>,
        x: Expr<'a>,
        y: Expr<'a>,
        tamano: Expr<'a>,
        color: &'a str,
    },

    DrawTriangle {
        v1_x: Expr<'a>,
        v1_y: Expr<'a>,
        v2_x: Expr<'a>,
        v2_y: Expr<'a>,
        v3_x: Expr<'a>,
        v3_y: Expr<'a>,
        color: &'a str,
    },

    DrawRing {
        center_x: Expr<'a>,
        center_y: Expr<'a>,
        inner_radius: Expr<'a>,
        outer_radius: Expr<'a>,
        color: &'a str,
    },

    DrawRectangleLines {
        x: Expr<'a>,
        y: Expr<'a>,
        ancho: Expr<'a>,
        alto: Expr<'a>,
        color: &'a str,
    },

    DrawEllipse {
        center_x: Expr<'a>,
        center_y: Expr<'a>,
        radius_h: Expr<'a>,
        radius_v: Expr<'a>,
        color: &'a str,
    },

    DrawLineThick {
        x1: Expr<'a>,
        y1: Expr<'a>,
        x2: Expr<'a>,
        y2: Expr<'a>,
        thick: Expr<'a>,
        color: &'a str,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_new() {
        let program: Program = Program::new();
        assert!(program.is_empty());
        assert_eq!(program.len(), 0);
    }

    #[test]
    fn test_program_default() {
        let program: Program = Program::default();
        assert!(program.is_empty());
    }

    #[test]
    fn test_expr_num() {
        let expr = Expr::Num(100.0);
        match expr {
            Expr::Num(n) => assert_eq!(n, 100.0),
            _ => panic!("No es Num"),
        }
    }

    #[test]
    fn test_expr_bool() {
        let expr = Expr::Bool(true);
        match expr {
            Expr::Bool(b) => assert!(b),
            _ => panic!("No es Bool"),
        }
    }

    #[test]
    fn test_binary_op_display() {
        assert_eq!(format!("{}", BinaryOp::Suma), "+");
        assert_eq!(format!("{}", BinaryOp::And), "and");
    }

    #[test]
    fn test_unary_op_display() {
        assert_eq!(format!("{}", UnaryOp::Not), "not");
        assert_eq!(format!("{}", UnaryOp::Neg), "-");
    }
}
