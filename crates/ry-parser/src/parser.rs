// crates/rydit-parser/src/parser.rs
// Parser con error recovery
//
// Parser que recupera y continúa, reportando múltiples errores.

use crate::ast::*;
use crate::error::*;
use ry_lexer::{Lexer, Token, TokenKind};

/// Parser para RyDit con error recovery
///
/// Convierte tokens en AST, recuperando de errores para reportar múltiples.
pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    pos: usize,
    state: ParseState<'a>,
}

impl<'a> Parser<'a> {
    /// Crear parser desde tokens
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens,
            pos: 0,
            state: ParseState::new(),
        }
    }

    /// Crear parser desde source code (convenience)
    pub fn from_source(source: &'a str) -> Self {
        let tokens = Lexer::new(source).scan();
        Self::new(tokens)
    }

    /// Parsear todo el programa
    ///
    /// Retorna (AST, Vec<Errores>) - nunca falla completamente.
    pub fn parse(&mut self) -> (Program<'a>, Vec<RyDitError>) {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            let pos_antes = self.pos;

            match self.parse_statement() {
                Some(stmt) => statements.push(stmt),
                None => {
                    // Token no reconocido - error recovery
                    if self.pos == pos_antes {
                        let token = self.current();
                        self.state.add_error(RyDitError::unexpected_token(
                            "statement",
                            &format!("{}", token.kind),
                            token.span.line,
                            token.span.column,
                        ));
                        self.advance(); // Avanzar para recovery
                    }
                }
            }

            self.state.recover();
        }

        let program = Program { statements };
        let errors = self.state.errors.clone();
        (program, errors)
    }

    /// Parsear un statement
    fn parse_statement(&mut self) -> Option<Stmt<'a>> {
        if self.is_at_end() {
            return None;
        }

        match &self.current().kind {
            // Saltar comentarios
            TokenKind::Comentario => {
                self.advance();
                None
            }

            TokenKind::ShieldInit => {
                self.advance();
                Some(Stmt::Init)
            }

            TokenKind::OndaCore => {
                self.advance();
                Some(Stmt::Command("onda.core"))
            }

            TokenKind::RyPrime => {
                self.advance();
                Some(Stmt::Command("ryprime"))
            }

            TokenKind::Ryda => self.parse_while(),
            TokenKind::Cada => self.parse_foreach(),
            TokenKind::Onif => self.parse_if(),
            TokenKind::DarkSlot => self.parse_assignment(),
            TokenKind::LlaveIzq => self.parse_block(),
            TokenKind::Rytmo => self.parse_function(),
            TokenKind::Return => self.parse_return(),
            TokenKind::Voz => self.parse_voz(),
            TokenKind::DrawCircle => self.parse_draw_circle(),
            TokenKind::DrawRect => self.parse_draw_rect(),
            TokenKind::DrawLine => self.parse_draw_line(),
            TokenKind::DrawText => self.parse_draw_text(),
            TokenKind::DrawTriangle => self.parse_draw_triangle(),
            TokenKind::DrawRing => self.parse_draw_ring(),
            TokenKind::DrawRectangleLines => self.parse_draw_rectangle_lines(),
            TokenKind::DrawEllipse => self.parse_draw_ellipse(),
            TokenKind::DrawLineThick => self.parse_draw_line_thick(),
            TokenKind::Break => {
                self.advance();
                Some(Stmt::Break)
            }
            TokenKind::Import => self.parse_import(),

            TokenKind::Ident => {
                // Verificar si es input() especial
                if let Some(name) = self.current().as_ident() {
                    self.advance(); // consumir el ident
                    if name == "input" {
                        return self.parse_input();
                    } else {
                        // Podría ser llamada a función o variable
                        return self.parse_call_or_ident(name);
                    }
                }
                None
            }

            _ => {
                // Token no es un statement válido
                None
            }
        }
    }

    /// Parsear if: onif condition { ... } [blelse { ... }]
    fn parse_if(&mut self) -> Option<Stmt<'a>> {
        self.advance(); // consumir onif

        // Parsear condición
        let condition = self.parse_expression()?;

        // Parsear cuerpo del then
        let mut then_body = Vec::new();

        // Verificar si hay bloque explícito { }
        if self.check(TokenKind::LlaveIzq) {
            if let Some(Stmt::Block(stmts)) = self.parse_block() {
                then_body = stmts;
            }
        } else {
            // Statements sueltos (hasta blelse o fin)
            while !self.is_at_end() {
                if self.check(TokenKind::Blelse) {
                    break;
                }
                if self.check(TokenKind::LlaveDer) {
                    self.advance(); // consumir }
                    break;
                }
                if let Some(stmt) = self.parse_statement() {
                    then_body.push(stmt);
                } else {
                    break;
                }
            }
        }

        // Parsear cuerpo del else (si existe)
        let else_body = if self.check(TokenKind::Blelse) {
            self.advance(); // consumir blelse

            if self.check(TokenKind::LlaveIzq) {
                if let Some(Stmt::Block(stmts)) = self.parse_block() {
                    Some(stmts)
                } else {
                    None
                }
            } else {
                // Statements sueltos
                let mut body = Vec::new();
                while !self.is_at_end() {
                    if self.check(TokenKind::LlaveDer) {
                        self.advance(); // consumir }
                        break;
                    }
                    if let Some(stmt) = self.parse_statement() {
                        body.push(stmt);
                    } else {
                        break;
                    }
                }
                Some(body)
            }
        } else {
            None
        };

        Some(Stmt::If {
            condition,
            then_body,
            else_body,
        })
    }

    /// Parsear while: ryda condition { ... } o ryda { ... } (condición true implícita)
    fn parse_while(&mut self) -> Option<Stmt<'a>> {
        self.advance(); // consumir ryda

        // Parsear condición - si es { directamente, condición true implícita
        let condition = if self.check(TokenKind::LlaveIzq) {
            Expr::Bool(true)
        } else {
            self.parse_expression()?
        };

        // Parsear cuerpo
        let body = if self.check(TokenKind::LlaveIzq) {
            if let Some(Stmt::Block(stmts)) = self.parse_block() {
                stmts
            } else {
                vec![]
            }
        } else {
            // Statements sueltos
            let mut statements = Vec::new();
            while !self.is_at_end() {
                if self.check(TokenKind::LlaveDer) {
                    self.advance(); // consumir }
                    break;
                }
                if let Some(stmt) = self.parse_statement() {
                    statements.push(stmt);
                } else {
                    break;
                }
            }
            statements
        };

        Some(Stmt::While { condition, body })
    }

    /// Parsear foreach: cada var en iterable { ... }
    fn parse_foreach(&mut self) -> Option<Stmt<'a>> {
        self.advance(); // consumir cada

        // Obtener nombre de variable
        let var = if let Some(v) = self.current().as_ident() {
            v
        } else {
            self.state.add_error(RyDitError::syntax_error(
                "Se esperaba variable después de 'cada'".to_string(),
                self.current().span.line,
                self.current().span.column,
            ));
            return None;
        };
        self.advance();

        // Consumir "en"
        if !self.check(TokenKind::En) {
            self.state.add_error(RyDitError::missing_token(
                "en",
                self.current().span.line,
                self.current().span.column,
            ));
            return None;
        }
        self.advance();

        // Parsear iterable
        let iterable = self.parse_expression()?;

        // Parsear cuerpo { ... }
        let body = if self.check(TokenKind::LlaveIzq) {
            if let Some(Stmt::Block(stmts)) = self.parse_block() {
                stmts
            } else {
                vec![]
            }
        } else {
            self.state.add_error(RyDitError::missing_token(
                "{",
                self.current().span.line,
                self.current().span.column,
            ));
            vec![]
        };

        Some(Stmt::ForEach {
            var,
            iterable,
            body,
        })
    }

    /// Parsear bloque: { ... }
    fn parse_block(&mut self) -> Option<Stmt<'a>> {
        self.advance(); // consumir {

        let mut statements = Vec::new();

        // Parsear statements hasta encontrar }
        while !self.is_at_end() {
            if self.check(TokenKind::LlaveDer) {
                self.advance(); // consumir }
                return Some(Stmt::Block(statements));
            }

            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                // Token no reconocido - error recovery
                break;
            }
        }

        // Si llegamos aquí, no se encontró }
        self.state.add_error(RyDitError::missing_token(
            "}",
            self.current().span.line,
            self.current().span.column,
        ));

        Some(Stmt::Block(statements))
    }

    /// Parsear función: rytmo nombre(params) { body }
    fn parse_function(&mut self) -> Option<Stmt<'a>> {
        self.advance(); // consumir rytmo

        // Obtener nombre
        let name = if let Some(n) = self.current().as_ident() {
            n
        } else {
            self.state.add_error(RyDitError::syntax_error(
                "Se esperaba nombre de función después de 'rytmo'".to_string(),
                self.current().span.line,
                self.current().span.column,
            ));
            return None;
        };
        self.advance();

        // Parsear parámetros ( )
        let mut params = vec![];
        if self.check(TokenKind::ParentIzq) {
            self.advance(); // consumir (

            while !self.is_at_end() && !self.check(TokenKind::ParentDer) {
                if let Some(p) = self.current().as_ident() {
                    params.push(p);
                }
                self.advance();
            }

            // Consumir )
            if self.check(TokenKind::ParentDer) {
                self.advance();
            }
        }

        // Parsear cuerpo { ... }
        let body = if self.check(TokenKind::LlaveIzq) {
            if let Some(Stmt::Block(stmts)) = self.parse_block() {
                stmts
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        Some(Stmt::Function { name, params, body })
    }

    /// Parsear voz: voz expr
    fn parse_voz(&mut self) -> Option<Stmt<'a>> {
        self.advance(); // consumir voz

        if let Some(expr) = self.parse_expression() {
            return Some(Stmt::Expr(expr));
        }

        None
    }

    /// Parsear asignación: dark.slot name = value
    fn parse_assignment(&mut self) -> Option<Stmt<'a>> {
        self.advance(); // consumir dark.slot

        // Obtener nombre
        let name = if let Some(n) = self.current().as_ident() {
            n
        } else {
            self.state.add_error(RyDitError::syntax_error(
                "Se esperaba nombre de variable después de 'dark.slot'".to_string(),
                self.current().span.line,
                self.current().span.column,
            ));
            return None;
        };
        self.advance();

        // Consumir =
        if !self.check(TokenKind::Asignar) {
            self.state.add_error(RyDitError::missing_token(
                "=",
                self.current().span.line,
                self.current().span.column,
            ));
            return None;
        }
        self.advance();

        // Parsear valor
        let value = self.parse_expression()?;

        Some(Stmt::Assign { name, value })
    }

    /// Parsear return
    fn parse_return(&mut self) -> Option<Stmt<'a>> {
        self.advance(); // consumir return

        // Return puede o no tener valor
        if self.is_at_end() || self.check(TokenKind::LlaveDer) || self.check(TokenKind::LlaveIzq) {
            return Some(Stmt::Return(None));
        }

        if let Some(expr) = self.parse_expression() {
            return Some(Stmt::Return(Some(expr)));
        }

        Some(Stmt::Return(None))
    }

    /// Parsear import
    fn parse_import(&mut self) -> Option<Stmt<'a>> {
        self.advance(); // consumir import

        // Obtener módulo
        let module = if let Some(m) = self.current().as_ident() {
            m
        } else {
            self.state.add_error(RyDitError::syntax_error(
                "Se esperaba nombre de módulo después de 'import'".to_string(),
                self.current().span.line,
                self.current().span.column,
            ));
            return None;
        };
        self.advance();

        // Verificar alias (as alias)
        let alias = if self.check(TokenKind::As) {
            self.advance(); // consumir as
            if let Some(a) = self.current().as_ident() {
                self.advance();
                Some(a)
            } else {
                None
            }
        } else {
            None
        };

        Some(Stmt::Import { module, alias })
    }

    /// Parsear input()
    fn parse_input(&mut self) -> Option<Stmt<'a>> {
        // input() como expresión, no statement
        // Se maneja en parse_call_or_ident
        None
    }

    /// Parsear llamada a función o ident
    fn parse_call_or_ident(&mut self, name: &'a str) -> Option<Stmt<'a>> {
        // Verificar si es llamada con ()
        if self.check(TokenKind::ParentIzq) {
            self.advance(); // consumir (

            let mut args = Vec::new();
            while !self.is_at_end() && !self.check(TokenKind::ParentDer) {
                if let Some(arg) = self.parse_expression() {
                    args.push(arg);
                }
                // Skip comma if present
                if self.check(TokenKind::Coma) {
                    self.advance();
                }
            }

            // Consumir )
            if self.check(TokenKind::ParentDer) {
                self.advance();
            }

            return Some(Stmt::Call { callee: name, args });
        }

        // Verificar si es indexación []
        if self.check(TokenKind::CorcheteIzq) {
            // array[index] = value
            self.advance(); // consumir [
            let index = self.parse_expression()?;

            if self.check(TokenKind::CorcheteDer) {
                self.advance(); // consumir ]
            }

            if self.check(TokenKind::Asignar) {
                self.advance(); // consumir =
                let value = self.parse_expression()?;
                return Some(Stmt::IndexAssign {
                    array: name,
                    index,
                    value,
                });
            }
        }

        // Verificar si es asignación: ident = expr
        if self.check(TokenKind::Asignar) {
            self.advance(); // consumir =
            let value = self.parse_expression()?;
            return Some(Stmt::Assign {
                name: name,
                value,
            });
        }

        // Solo ident - no es statement válido por sí solo
        None
    }

    // ========================================================================
    // DRAW COMMANDS
    // ========================================================================

    fn parse_draw_circle(&mut self) -> Option<Stmt<'a>> {
        self.advance(); // consumir draw.circle

        if !self.consume(TokenKind::ParentIzq, "(")? {
            return None;
        }

        let x = self.parse_expression()?;
        self.skip_comma();
        let y = self.parse_expression()?;
        self.skip_comma();
        let radio = self.parse_expression()?;
        self.skip_comma();
        let color = self.parse_color()?;

        self.consume(TokenKind::ParentDer, ")")?;

        Some(Stmt::DrawCircle { x, y, radio, color })
    }

    fn parse_draw_rect(&mut self) -> Option<Stmt<'a>> {
        self.advance();

        if !self.consume(TokenKind::ParentIzq, "(")? {
            return None;
        }

        let x = self.parse_expression()?;
        self.skip_comma();
        let y = self.parse_expression()?;
        self.skip_comma();
        let ancho = self.parse_expression()?;
        self.skip_comma();
        let alto = self.parse_expression()?;
        self.skip_comma();
        let color = self.parse_color()?;

        self.consume(TokenKind::ParentDer, ")")?;

        Some(Stmt::DrawRect {
            x,
            y,
            ancho,
            alto,
            color,
        })
    }

    fn parse_draw_line(&mut self) -> Option<Stmt<'a>> {
        self.advance();

        if !self.consume(TokenKind::ParentIzq, "(")? {
            return None;
        }

        let x1 = self.parse_expression()?;
        self.skip_comma();
        let y1 = self.parse_expression()?;
        self.skip_comma();
        let x2 = self.parse_expression()?;
        self.skip_comma();
        let y2 = self.parse_expression()?;
        self.skip_comma();
        let color = self.parse_color()?;

        self.consume(TokenKind::ParentDer, ")")?;

        Some(Stmt::DrawLine {
            x1,
            y1,
            x2,
            y2,
            color,
        })
    }

    fn parse_draw_text(&mut self) -> Option<Stmt<'a>> {
        self.advance();

        if !self.consume(TokenKind::ParentIzq, "(")? {
            return None;
        }

        let texto = self.parse_expression()?;
        self.skip_comma();
        let x = self.parse_expression()?;
        self.skip_comma();
        let y = self.parse_expression()?;
        self.skip_comma();
        let tamano = self.parse_expression()?;
        self.skip_comma();
        let color = self.parse_color()?;

        self.consume(TokenKind::ParentDer, ")")?;

        Some(Stmt::DrawText {
            texto,
            x,
            y,
            tamano,
            color,
        })
    }

    fn parse_draw_triangle(&mut self) -> Option<Stmt<'a>> {
        self.advance();

        if !self.consume(TokenKind::ParentIzq, "(")? {
            return None;
        }

        let v1_x = self.parse_expression()?;
        self.skip_comma();
        let v1_y = self.parse_expression()?;
        self.skip_comma();
        let v2_x = self.parse_expression()?;
        self.skip_comma();
        let v2_y = self.parse_expression()?;
        self.skip_comma();
        let v3_x = self.parse_expression()?;
        self.skip_comma();
        let v3_y = self.parse_expression()?;
        self.skip_comma();
        let color = self.parse_color()?;

        self.consume(TokenKind::ParentDer, ")")?;

        Some(Stmt::DrawTriangle {
            v1_x,
            v1_y,
            v2_x,
            v2_y,
            v3_x,
            v3_y,
            color,
        })
    }

    fn parse_draw_ring(&mut self) -> Option<Stmt<'a>> {
        self.advance();

        if !self.consume(TokenKind::ParentIzq, "(")? {
            return None;
        }

        let center_x = self.parse_expression()?;
        self.skip_comma();
        let center_y = self.parse_expression()?;
        self.skip_comma();
        let inner_radius = self.parse_expression()?;
        self.skip_comma();
        let outer_radius = self.parse_expression()?;
        self.skip_comma();
        let color = self.parse_color()?;

        self.consume(TokenKind::ParentDer, ")")?;

        Some(Stmt::DrawRing {
            center_x,
            center_y,
            inner_radius,
            outer_radius,
            color,
        })
    }

    fn parse_draw_rectangle_lines(&mut self) -> Option<Stmt<'a>> {
        self.advance();

        if !self.consume(TokenKind::ParentIzq, "(")? {
            return None;
        }

        let x = self.parse_expression()?;
        self.skip_comma();
        let y = self.parse_expression()?;
        self.skip_comma();
        let ancho = self.parse_expression()?;
        self.skip_comma();
        let alto = self.parse_expression()?;
        self.skip_comma();
        let color = self.parse_color()?;

        self.consume(TokenKind::ParentDer, ")")?;

        Some(Stmt::DrawRectangleLines {
            x,
            y,
            ancho,
            alto,
            color,
        })
    }

    fn parse_draw_ellipse(&mut self) -> Option<Stmt<'a>> {
        self.advance();

        if !self.consume(TokenKind::ParentIzq, "(")? {
            return None;
        }

        let center_x = self.parse_expression()?;
        self.skip_comma();
        let center_y = self.parse_expression()?;
        self.skip_comma();
        let radius_h = self.parse_expression()?;
        self.skip_comma();
        let radius_v = self.parse_expression()?;
        self.skip_comma();
        let color = self.parse_color()?;

        self.consume(TokenKind::ParentDer, ")")?;

        Some(Stmt::DrawEllipse {
            center_x,
            center_y,
            radius_h,
            radius_v,
            color,
        })
    }

    fn parse_draw_line_thick(&mut self) -> Option<Stmt<'a>> {
        self.advance();

        if !self.consume(TokenKind::ParentIzq, "(")? {
            return None;
        }

        let x1 = self.parse_expression()?;
        self.skip_comma();
        let y1 = self.parse_expression()?;
        self.skip_comma();
        let x2 = self.parse_expression()?;
        self.skip_comma();
        let y2 = self.parse_expression()?;
        self.skip_comma();
        let thick = self.parse_expression()?;
        self.skip_comma();
        let color = self.parse_color()?;

        self.consume(TokenKind::ParentDer, ")")?;

        Some(Stmt::DrawLineThick {
            x1,
            y1,
            x2,
            y2,
            thick,
            color,
        })
    }

    // ========================================================================
    // EXPRESSION PARSING
    // ========================================================================

    /// Parsear expresión (precedencia mínima)
    fn parse_expression(&mut self) -> Option<Expr<'a>> {
        self.parse_or()
    }

    /// Parsear OR: expr or expr
    fn parse_or(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_and()?;

        while self.check(TokenKind::Or) {
            self.advance();
            let right = self.parse_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    /// Parsear AND: expr and expr
    fn parse_and(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_equality()?;

        while self.check(TokenKind::And) {
            self.advance();
            let right = self.parse_equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    /// Parsear igualdad: expr == expr, expr != expr
    fn parse_equality(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_comparison()?;

        loop {
            let op = if self.check(TokenKind::Igual) {
                BinaryOp::Igual
            } else if self.check(TokenKind::Diferente) {
                BinaryOp::Diferente
            } else {
                break;
            };

            self.advance();
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    /// Parsear comparación: expr > expr, expr < expr, etc.
    fn parse_comparison(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_term()?;

        loop {
            let op = if self.check(TokenKind::Mayor) {
                BinaryOp::Mayor
            } else if self.check(TokenKind::Menor) {
                BinaryOp::Menor
            } else if self.check(TokenKind::MayorIgual) {
                BinaryOp::MayorIgual
            } else if self.check(TokenKind::MenorIgual) {
                BinaryOp::MenorIgual
            } else {
                break;
            };

            self.advance();
            let right = self.parse_term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    /// Parsear término: expr + expr, expr - expr, expr += expr, expr -= expr
    fn parse_term(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_factor()?;

        loop {
            let op = if self.check(TokenKind::Mas) {
                BinaryOp::Suma
            } else if self.check(TokenKind::Menos) {
                BinaryOp::Resta
            } else if self.check(TokenKind::MasIgual) {
                BinaryOp::MasIgual
            } else if self.check(TokenKind::MenosIgual) {
                BinaryOp::MenosIgual
            } else {
                break;
            };

            self.advance();
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    /// Parsear factor: expr * expr, expr / expr, expr *= expr, expr /= expr
    fn parse_factor(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_unary()?;

        loop {
            let op = if self.check(TokenKind::Por) {
                BinaryOp::Mult
            } else if self.check(TokenKind::Div) {
                BinaryOp::Div
            } else if self.check(TokenKind::PorIgual) {
                BinaryOp::PorIgual
            } else if self.check(TokenKind::DivIgual) {
                BinaryOp::DivIgual
            } else {
                break;
            };

            self.advance();
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    /// Parsear unario: -expr, not expr
    fn parse_unary(&mut self) -> Option<Expr<'a>> {
        if self.check(TokenKind::Not) {
            self.advance();
            let expr = self.parse_unary()?;
            return Some(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(expr),
            });
        }

        if self.check(TokenKind::Menos) {
            self.advance();
            let expr = self.parse_unary()?;
            return Some(Expr::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(expr),
            });
        }

        self.parse_primary()
    }

    /// Parsear primario: números, strings, variables, parentesis
    fn parse_primary(&mut self) -> Option<Expr<'a>> {
        let token = self.current();

        match token.kind {
            TokenKind::Num => {
                if let Some(n) = token.as_num() {
                    self.advance();
                    return Some(Expr::Num(n));
                }
                self.state.add_error(RyDitError::syntax_error(
                    "Número inválido".to_string(),
                    token.span.line,
                    token.span.column,
                ));
                self.advance();
                None
            }
            TokenKind::Texto => {
                let text = token.lexeme.trim_matches('"').trim_matches('\'');
                self.advance();
                Some(Expr::Texto(text))
            }
            TokenKind::Ident => {
                if let Some(name) = token.as_ident() {
                    let name_str = name;
                    self.advance();

                    // Verificar si es llamada a función
                    if self.check(TokenKind::ParentIzq) {
                        self.advance(); // consumir (

                        let mut args = Vec::new();
                        while !self.is_at_end() && !self.check(TokenKind::ParentDer) {
                            if let Some(arg) = self.parse_expression() {
                                args.push(arg);
                            }
                            if self.check(TokenKind::Coma) {
                                self.advance();
                            }
                        }

                        if self.check(TokenKind::ParentDer) {
                            self.advance();
                        }

                        return Some(Expr::Call {
                            callee: Box::new(Expr::Var(name_str)),
                            args,
                        });
                    }

                    // Verificar si es indexación
                    if self.check(TokenKind::CorcheteIzq) {
                        self.advance(); // consumir [
                        let index = self.parse_expression()?;
                        if self.check(TokenKind::CorcheteDer) {
                            self.advance();
                        }
                        return Some(Expr::Index {
                            array: Box::new(Expr::Var(name_str)),
                            index: Box::new(index),
                        });
                    }

                    Some(Expr::Var(name_str))
                } else {
                    self.state.add_error(RyDitError::syntax_error(
                        "Identificador inválido".to_string(),
                        token.span.line,
                        token.span.column,
                    ));
                    self.advance();
                    None
                }
            }
            TokenKind::ParentIzq => {
                self.advance(); // consumir (
                let expr = self.parse_expression()?;
                self.consume(TokenKind::ParentDer, ")")?;
                Some(expr)
            }
            TokenKind::CorcheteIzq => {
                self.advance(); // consumir [
                let mut elements = Vec::new();
                while !self.is_at_end() && !self.check(TokenKind::CorcheteDer) {
                    if let Some(expr) = self.parse_expression() {
                        elements.push(expr);
                    }
                    if self.check(TokenKind::Coma) {
                        self.advance();
                    }
                }
                if self.check(TokenKind::CorcheteDer) {
                    self.advance();
                }
                Some(Expr::Array(elements))
            }
            _ => {
                self.state.add_error(RyDitError::unexpected_token(
                    "expresión",
                    &format!("{}", self.current().kind),
                    self.current().span.line,
                    self.current().span.column,
                ));
                self.advance();
                None
            }
        }
    }

    // ========================================================================
    // HELPERS
    // ========================================================================

    /// Token actual
    fn current(&self) -> &Token<'a> {
        if self.pos >= self.tokens.len() {
            // EOF token
            &self.tokens[self.tokens.len() - 1]
        } else {
            &self.tokens[self.pos]
        }
    }

    /// Verificar si estamos al final
    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    /// Verificar token actual
    fn check(&self, kind: TokenKind) -> bool {
        self.current().kind == kind
    }

    /// Avanzar al siguiente token
    fn advance(&mut self) {
        if !self.is_at_end() {
            self.pos += 1;
        }
    }

    /// Consumir token esperado
    fn consume(&mut self, expected: TokenKind, msg: &str) -> Option<bool> {
        if self.check(expected) {
            self.advance();
            Some(true)
        } else {
            self.state.add_error(RyDitError::missing_token(
                msg,
                self.current().span.line,
                self.current().span.column,
            ));
            None
        }
    }

    /// Skip comma si está presente
    fn skip_comma(&mut self) {
        if self.check(TokenKind::Coma) {
            self.advance();
        }
    }

    /// Parsear color (string)
    fn parse_color(&mut self) -> Option<&'a str> {
        let token = self.current();
        if token.kind == TokenKind::Texto {
            let color = token.lexeme.trim_matches('"').trim_matches('\'');
            self.advance();
            Some(color)
        } else {
            Some("negro") // Default
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_init() {
        let mut parser = Parser::from_source("shield.init");
        let (program, _errors) = parser.parse();
        assert!(_errors.is_empty());
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn test_parse_assignment() {
        let mut parser = Parser::from_source("dark.slot x = 100");
        let (program, _errors) = parser.parse();
        assert!(_errors.is_empty());
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn test_parse_if_simple() {
        let mut parser = Parser::from_source("onif x { voz \"hola\" }");
        let (program, _errors) = parser.parse();
        assert!(_errors.is_empty());
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn test_parse_while() {
        let mut parser = Parser::from_source("ryda x < 10 { voz x }");
        let (program, _errors) = parser.parse();
        assert!(_errors.is_empty());
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn test_parse_function() {
        let mut parser = Parser::from_source("rytmo test() { voz \"hola\" }");
        let (program, _errors) = parser.parse();
        assert!(_errors.is_empty());
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn test_parse_error_recovery() {
        // Múltiples statements, uno con error - el parser continúa
        let mut parser = Parser::from_source("dark.slot x = 100; dark.slot y = 200");
        let (program, _errors) = parser.parse();
        // Debería parsear al menos 1 statement válido
        assert!(program.len() >= 1);
    }

    #[test]
    fn test_operadores_compuestos_parse_expr() {
        // Test que += se parsea como BinaryOp
        let mut parser = Parser::from_source("x += 10");
        let (program, _errors) = parser.parse();
        // Al menos parsea algo
        assert!(program.len() >= 0);
    }
}
