use std::{
    fmt::Display,
    process::{exit, ExitCode},
};

use crate::{
    environment::{define_env, get_env},
    token::{Token, TokenType},
};

fn evaluation_error(msg: &str) -> Literal {
    eprintln!("{}", msg);
    exit(70)
}

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
    Variable {
        variable: Token,
    },
    Assignment {
        name: Token,
        value: Box<Expression>,
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
    fn get_type(&self) -> String {
        match self {
            Literal::String(_s) => "string".to_string(),
            Literal::Nil => "nil".to_string(),
            Literal::Number(_n) => "number".to_string(),
            Literal::Bool(_b) => "bool".to_string(),
        }
    }

    fn to_string(&self) -> Result<String, String> {
        match self {
            Literal::String(s) => Ok(s.clone()),
            Literal::Nil => Err("Error type".to_string()),
            Literal::Number(n) => Ok(n.to_string()),
            Literal::Bool(b) => Ok(if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }),
        }
    }

    fn to_number(&self) -> Result<f64, String> {
        match self {
            Literal::String(s) => {
                let num = s.parse();
                match num {
                    Ok(n) => Ok(n),
                    Err(_e) => Err("Error type".to_string()),
                }
            }
            Literal::Nil => Err("Error type".to_string()),
            Literal::Bool(_b) => Err("Error type".to_string()),
            Literal::Number(n) => Ok(*n),
        }
    }

    fn to_bool(&self) -> Result<bool, String> {
        match self {
            Literal::String(_s) => Ok(true),
            Literal::Nil => Ok(false),
            Literal::Number(_n) => Ok(true),
            Literal::Bool(b) => Ok(*b),
        }
    }

    pub fn print(&self) {
        match self {
            Literal::String(s) => println!("{}", s),
            Literal::Nil => println!("nil"),
            Literal::Bool(s) => println!("{}", s),
            Literal::Number(n) => {
                let _formatted = if n.fract() == 0.0 {
                    // If there is no fractional part, show one decimal place
                    println!("{:.0}", n)
                } else {
                    // Otherwise, show the full precision
                    println!("{}", n)
                };
            }
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
            Expression::Variable { variable } => f.write_fmt(format_args!("{}", variable.lexeme)),
            Expression::Assignment { name, value } => f.write_fmt(format_args!("{}", name.lexeme)),
        }
    }
}

pub fn eval_unary(operator: Token, expr: &Expression) -> Literal {
    let expr_lit_raw = expr.accept();

    match operator.token_type {
        TokenType::MINUS => match expr_lit_raw.to_number() {
            Ok(num) => {
                return Literal::Number(-num);
            }
            Err(_) => evaluation_error("Operand must be a number."),
        },
        TokenType::BANG => match expr_lit_raw.to_bool() {
            Ok(b) => {
                return Literal::Bool(!b);
            }
            Err(_) => panic!("Something went wrong!"),
        },
        _ => panic!("Something went wrong!"),
    }
}

// "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;
pub fn eval_binary(operator: &Token, left_expr: &Expression, right_expr: &Expression) -> Literal {
    let left_raw = left_expr.accept();
    let right_raw = right_expr.accept();
    let left = left_raw.to_number();
    let right = right_raw.to_number();
    let left_str = left_raw.to_string();
    let right_str = right_raw.to_string();
    let left_type = left_raw.get_type();
    let right_type = right_raw.get_type();

    match operator.token_type {
        TokenType::EQUAL_EQUAL => {
            if let Ok(l) = left_str {
                if let Ok(r) = right_str {
                    return Literal::Bool(l == r && left_type == right_type);
                }
            }
            return evaluation_error("Operands must be numbers.");
        }
        TokenType::BANG_EQUAL => {
            if let Ok(l) = left_str {
                if let Ok(r) = right_str {
                    return Literal::Bool(l != r || left_type != right_type);
                }
            }
            return evaluation_error("Operands must be numbers.");
        }
        TokenType::LESS => {
            if let Ok(l) = left {
                if let Ok(r) = right {
                    return Literal::Bool(l < r);
                }
            }
            return evaluation_error("Operands must be numbers.");
        }
        TokenType::GREATER => {
            if let Ok(l) = left {
                if let Ok(r) = right {
                    return Literal::Bool(l > r);
                }
            }
            return evaluation_error("Operands must be numbers.");
        }
        TokenType::LESS_EQUAL => {
            if let Ok(l) = left {
                if let Ok(r) = right {
                    return Literal::Bool(l <= r);
                }
            }
            return evaluation_error("Operands must be numbers.");
        }
        TokenType::GREATER_EQUAL => {
            if let Ok(l) = left {
                if let Ok(r) = right {
                    return Literal::Bool(l >= r);
                }
            }
            return evaluation_error("Operands must be numbers.");
        }
        TokenType::PLUS => {
            if left_type != right_type {
                return evaluation_error("Operands must be two numbers or two strings.");
            }

            if let Ok(l) = left {
                if let Ok(r) = right {
                    return Literal::Number(l + r);
                }
            }

            if let Ok(l_str) = left_str {
                if let Ok(r_str) = right_str {
                    return Literal::String(format!("{}{}", l_str, r_str));
                }
            }

            return evaluation_error("Operands must be two numbers or two strings.");
        }
        TokenType::MINUS => {
            if let Ok(l) = left {
                if let Ok(r) = right {
                    return Literal::Number(l - r);
                }
            }
            return evaluation_error("Operands must be numbers.");
        }
        TokenType::STAR => {
            if let Ok(l) = left {
                if let Ok(r) = right {
                    return Literal::Number(l * r);
                }
            }
            return evaluation_error("Operands must be numbers.");
        }
        TokenType::SLASH => {
            if let Ok(l) = left {
                if let Ok(r) = right {
                    if r == 0.0 {
                        panic!("Something went wrong!");
                    }
                    return Literal::Number(l / r);
                }
            }
            return evaluation_error("Operands must be numbers.");
        }
        _ => panic!("Something went wrong!"),
    }
}

pub fn eval_group(expr: &Expression) -> Literal {
    return expr.accept();
}

pub fn eval_literal(lit: Literal) -> Literal {
    return lit;
}

pub fn eval_variable(variable: &Token) -> Literal {
    return get_env(variable.lexeme.clone());
}

pub fn eval_assignment(name: String, value: &Expression) -> Literal {
    let val = value.accept();
    define_env(name, val.clone());
    return val;
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
            Expression::Variable { variable } => eval_variable(variable),
            Expression::Assignment { name, value } => eval_assignment(name.lexeme.clone(), &**value),
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

    pub fn advance(&mut self) {
        self.curr_idx += 1;
    }

    pub fn is_at_end(&self) -> bool {
        if self.curr_idx >= self.tokens.len() {
            return true;
        }
        return false;
    }

    pub fn peek(&self) -> Token {
        return self.tokens.get(self.curr_idx).unwrap().clone();
    }

    fn check(&self, token_type: TokenType) -> bool {
        if Self::is_at_end(self) {
            return false;
        }
        return Self::peek(self).token_type == token_type;
    }

    pub fn match_type(&self, targets: &[TokenType]) -> bool {
        for target in targets {
            if Self::check(self, target.clone()) {
                return true;
            }
        }

        return false;
    }

    pub fn consume(&mut self, expected_token: TokenType, error_msg: String) -> Token {
        if !Self::match_type(self, &[expected_token]) {
            eprintln!("{}", error_msg);
            exit(65);
        }

        let token = Self::peek(self);

        Self::advance(self);

        return token;
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
            Self::consume(
                self,
                TokenType::RIGHT_PAREN,
                "[line 1] Error at '{}': Expect expression.".to_string(),
            );
            return Expression::Grouping {
                expr: Box::new(expr),
            };
        }
        if Self::match_type(self, &[TokenType::IDENTIFIER]) {
            let variable = Expression::Variable {
                variable: Self::peek(self),
            };
            Self::advance(self);
            return variable;
        }
        eprintln!(
            "[line 1] Error at '{}': Expect expression.",
            self.peek().lexeme
        );
        exit(65);
    }

    pub fn assignment(&mut self) -> Expression {
        let expr = self.equality();

        if Self::match_type(self, &[TokenType::EQUAL]) {
            let token = Self::peek(self);
            Self::advance(self);

            let value = Self::expression(self);

            match expr {
                Expression::Variable { variable } => {
                    return Expression::Assignment {
                        name: variable,
                        value: Box::new(value),
                    };
                }

                _ => {
                    eprintln!("Invalid assignment target.");
                    exit(65);
                }
            }
        }

        return expr;
    }

    pub fn expression(&mut self) -> Expression {
        return Self::assignment(self);
    }

    pub fn parse_tree(&mut self, debug: bool) {
        while !Self::is_at_end(&self) {
            let expr = Self::equality(self);
            self.exprs.push(expr);
        }

        if debug {
            for expr in &self.exprs {
                print!("{}", expr);
            }
        }
    }

    pub fn export_exprs(&self) -> Vec<Expression> {
        return self.exprs.clone();
    }
}
