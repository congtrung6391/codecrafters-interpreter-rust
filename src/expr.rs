use std::{
    any::{Any, TypeId},
    fmt::Display,
    process::exit,
};

use crate::token::{Token, TokenType};

/**
 * Grammer
 * expression     → literal | unary | binary | grouping ;
 * literal        → NUMBER | STRING | "true" | "false" | "nil" ;
 * grouping       → "(" expression ")" ;
 * unary          → ( "-" | "!" ) expression ;
 * binary         → expression operator expression ;
 * operator       → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;
 **/

#[derive(Clone)]
pub enum Literal {
    Bool(bool),
    Nil,
    Number(f64),
    String(String),
}

#[derive(Clone)]
pub enum Expression {
    Literal(Literal),
    Unary {
        operator: Token,
        expr: Box<Expression>,
    },
    Binary {
        operator: Token,
        left_expr: Box<Expression>,
        right_expr: Box<Expression>,
    },
    Grouping {
        expr: Box<Expression>,
    },
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(b) => f.write_fmt(format_args!("{b}")),
            Literal::Nil => f.write_str("nil"),
            Literal::Number(n) => f.write_fmt(format_args!("{n:?}")),
            Literal::Bool(s) => f.write_fmt(format_args!("{s}")),
        }
    }
}

impl Literal {
    fn to_string(&self) -> String {
        match self {
            Literal::String(s) => s.clone(),
            Literal::Nil => panic!("Something went wrong"),
            Literal::Number(n) => n.to_string(),
            Literal::Bool(b) => panic!("Something went wrong"),
        }
    }

    fn to_number(&self) -> f64 {
        match self {
            Literal::String(s) => {
                let num = s.parse();
                match num {
                    Ok(n) => return n,
                    Err(e) => panic!("Something went wrong")
                }
            },
            Literal::Nil => panic!("Something went wrong"),
            Literal::Number(n) => *n,
            Literal::Bool(b) => if *b == false { 0.0 } else { 1.1 },
        }
    }

    fn to_bool(&self) -> bool{
         match self {
            Literal::String(s) => true,
            Literal::Nil => false,
            Literal::Number(n) => true,
            Literal::Bool(b) => *b,
        }

    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Unary { operator, expr } => {
                f.write_fmt(format_args!("({} {expr})", operator.lexeme))
            }
            Expression::Binary {
                operator,
                left_expr,
                right_expr,
            } => f.write_fmt(format_args!(
                "({} {left_expr} {right_expr})",
                operator.lexeme
            )),
            Expression::Grouping { expr } => f.write_fmt(format_args!("(group {expr})")),
            Expression::Literal(lit) => f.write_fmt(format_args!("{}", lit)),
        }
    }
}

pub fn eval_unary(operator: Token, expr: &Expression) -> Literal {
    let expr_lit = expr.accept();

    match operator.token_type {
        TokenType::MINUS => {
            let num = expr_lit.to_number();
            return Literal::Number(-num);
            
        },
        TokenType::BANG => {
            let b = expr_lit.to_bool();
            return Literal::Bool(!b);
            
        },
        _ => panic!("Something went wrong!"),
    }
}

// "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;
pub fn eval_binary(operator: &Token, left_expr: &Expression, right_expr: &Expression) -> Literal {
    return Literal::Nil;
    let left = left_expr.accept();
    let right = right_expr.accept();
    match operator {
        // TokenType::EQUAL_EQUAL => {
        //     return (left == right);
        // }
        // TokenType::BANG_EQUAL => {
        //     return (left != right);
        // }
        // TokenType::LESS => {
        //     return (left < right);
        // }
        // TokenType::GREATER => {
        //     return (left > right);
        // }
        // TokenType::LESS_EQUAL => {
        //     return (left <= right);
        // }
        // TokenType::GREATER_EQUAL => {
        //     return (left >= right);
        // }
        // TokenType::PLUS => {
        //     return left + right;
        // }
        // TokenType::MINUS => {
        //     return left - right;
        // }
        // TokenType::STAR => {
        //     return left * right;
        // }
        // TokenType::SLASH => {
        //     return left / right;
        // }
        _ => panic!("Something went wrong!"),
    }
}

pub fn eval_group(expr: &Expression) -> Literal {
    return expr.accept();
}

pub fn eval_literal(lit: Literal) -> Literal {
    return lit;
}

impl Expression {
    pub fn accept(&self) -> Literal {
        match self {
            Expression::Binary {
                operator,
                left_expr,
                right_expr,
            } => eval_binary(operator, &**left_expr, &**right_expr),
            Expression::Grouping { expr } => eval_group(expr),
            Expression::Unary { operator, expr } => eval_unary(operator.clone(), &**expr),
            Expression::Literal(lit) => eval_literal(lit.clone()),
        }
    }
}

pub struct AST {
    tokens: Vec<Token>,
    exprs: Vec<Expression>,
    curr_idx: usize,
}

impl AST {
    pub fn new(tokens: Vec<Token>) -> AST {
        return AST {
            curr_idx: 0,
            tokens,
            exprs: Vec::new(),
        };
    }

    fn advance(&mut self) {
        self.curr_idx += 1;
    }

    fn is_at_end(&self) -> bool {
        if self.curr_idx >= self.tokens.len() {
            return true;
        }
        return false;
    }

    fn peek(&self) -> Token {
        return self.tokens.get(self.curr_idx).unwrap().clone();
    }

    fn check(&self, token_type: TokenType) -> bool {
        if Self::is_at_end(self) {
            return false;
        }
        return Self::peek(self).token_type == token_type;
    }

    fn match_type(&self, targets: &[TokenType]) -> bool {
        for target in targets {
            if Self::check(self, target.clone()) {
                return true;
            }
        }

        return false;
    }

    fn equality(&mut self) -> Expression {
        let mut left_expr: Expression = Self::comparision(self);

        let match_targets = [TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL];
        while Self::match_type(self, &match_targets) {
            let operator = Self::peek(self);
            Self::advance(self);
            let right_expr: Expression = Self::comparision(self);

            left_expr = Expression::Binary {
                operator,
                left_expr: Box::new(left_expr),
                right_expr: Box::new(right_expr),
            };
        }

        return left_expr;
    }

    fn comparision(&mut self) -> Expression {
        let mut left_expr: Expression = Self::term(self);

        let match_targets = [
            TokenType::LESS_EQUAL,
            TokenType::LESS,
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
        ];
        while Self::match_type(self, &match_targets) {
            let operator = Self::peek(self);
            Self::advance(self);
            let right_expr: Expression = Self::term(self);

            left_expr = Expression::Binary {
                operator,
                left_expr: Box::new(left_expr),
                right_expr: Box::new(right_expr),
            };
        }

        return left_expr;
    }

    fn term(&mut self) -> Expression {
        let mut left_expr: Expression = Self::factor(self);

        let match_targets = [TokenType::PLUS, TokenType::MINUS];
        while Self::match_type(self, &match_targets) {
            let operator = Self::peek(self);
            Self::advance(self);
            let right_expr: Expression = Self::factor(self);

            left_expr = Expression::Binary {
                operator,
                left_expr: Box::new(left_expr),
                right_expr: Box::new(right_expr),
            };
        }

        return left_expr;
    }

    fn factor(&mut self) -> Expression {
        let mut left_expr: Expression = Self::unary(self);

        let match_targets = [TokenType::STAR, TokenType::SLASH];
        while Self::match_type(self, &match_targets) {
            let operator = Self::peek(self);
            Self::advance(self);
            let right_expr: Expression = Self::unary(self);

            left_expr = Expression::Binary {
                operator,
                left_expr: Box::new(left_expr),
                right_expr: Box::new(right_expr),
            };
        }

        return left_expr;
    }

    fn unary(&mut self) -> Expression {
        let match_targets = [TokenType::MINUS, TokenType::BANG];
        if Self::match_type(self, &match_targets) {
            let operator = Self::peek(self);
            Self::advance(self);
            let expr: Expression = Self::unary(self);

            let expr = Expression::Unary {
                operator,
                expr: Box::new(expr),
            };

            return expr;
        }

        return Self::primary(self);
    }

    fn primary(&mut self) -> Expression {
        if Self::match_type(self, &[TokenType::FALSE]) {
            Self::advance(self);
            return Expression::Literal(Literal::Bool(false));
        }
        if Self::match_type(self, &[TokenType::TRUE]) {
            Self::advance(self);
            return Expression::Literal(Literal::Bool(true));
        }
        if Self::match_type(self, &[TokenType::NIL]) {
            Self::advance(self);
            return Expression::Literal(Literal::Nil);
        }
        if Self::match_type(self, &[TokenType::STRING]) {
            let lit_string = Self::peek(self).literal.unwrap_or_default();
            Self::advance(self);
            return Expression::Literal(Literal::String(lit_string));
        }
        if Self::match_type(self, &[TokenType::NUMBER]) {
            let literal_number = Self::peek(self)
                .literal
                .unwrap_or_default()
                .parse::<f64>()
                .unwrap_or_default();
            Self::advance(self);
            return Expression::Literal(Literal::Number(literal_number));
        }
        if Self::match_type(self, &[TokenType::LEFT_PAREN]) {
            Self::advance(self);
            let expr = Self::expression(self);
            if Self::match_type(self, &[TokenType::RIGHT_PAREN]) {
                Self::advance(self);
                return Expression::Grouping {
                    expr: Box::new(expr),
                };
            } else {
                eprintln!(
                    "[line 1] Error at '{}': Expect expression.",
                    self.peek().lexeme
                );
                exit(65);
            }
        }
        eprintln!(
            "[line 1] Error at '{}': Expect expression.",
            self.peek().lexeme
        );
        exit(65);
    }

    fn expression(&mut self) -> Expression {
        return Self::equality(self);
    }

    pub fn parse_tree(&mut self) {
        while !Self::is_at_end(&self) {
            let expr = Self::equality(self);
            self.exprs.push(expr);
        }

        for expr in &self.exprs {
            print!("{}", expr);
        }
    }

    pub fn export_exprs(&self) -> Vec<Expression> {
        return self.exprs.clone();
    }
}
