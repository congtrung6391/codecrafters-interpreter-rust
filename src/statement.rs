use crate::{
    environment::define_env,
    expr::{Expression, Literal, AST},
    token::{Token, TokenType},
};

pub enum Statement {
    PrintStatement(Expression),
    ExprStatement(Expression),
    DeclStatement { name: Token, value: Literal },
    BlockStatement(Vec<Statement>),
}

fn eval_print_stat(expr: &Expression) {
    let lit = expr.accept();
    lit.print();
}

fn eval_decl_stat(name: String, value: &Literal) {
    define_env(name, value.clone());
}

fn eval_expr_stat(expr: &Expression) {
    expr.accept();
}

fn eval_block_stat(stats: &Vec<Statement>) {
    for stat in stats {
        stat.accept();
    }
}

impl Statement {
    pub fn accept(&self) {
        match self {
            Statement::PrintStatement(expr) => eval_print_stat(expr),
            Statement::DeclStatement { name, value } => eval_decl_stat(name.lexeme.clone(), value),
            Statement::ExprStatement(expr) => eval_expr_stat(expr),
            Statement::BlockStatement(stats) => eval_block_stat(stats),
        }
    }
}

pub struct SST {
    ast: AST,
    stats: Vec<Statement>,
}

impl SST {
    pub fn new(a: AST) -> SST {
        SST {
            ast: a,
            stats: Vec::new(),
        }
    }

    fn print_stat(&mut self) -> Statement {
        self.ast.advance();

        let expr = self.ast.expression();
        self.ast
            .consume(TokenType::SEMICOLON, "expected semicolon".to_string());
        return Statement::PrintStatement(expr);
    }

    fn declare_stat(&mut self) -> Statement {
        self.ast.advance();
        let name = self
            .ast
            .consume(TokenType::IDENTIFIER, "Expect variable name.".to_string());

        let mut value = Literal::Nil;

        if self.ast.match_type(&[TokenType::EQUAL]) {
            self.ast.advance();
            value = self.ast.expression().accept();
        }

        self.ast
            .consume(TokenType::SEMICOLON, "expected semicolon".to_string());

        return Statement::DeclStatement { name, value };
    }

    fn block_stat(&mut self) -> Statement {
        self.ast.advance();
        let mut stats = Vec::new();

        while !self.ast.check(TokenType::RIGHT_BRACE) {
            let stat = Self::statement(self);
            stats.push(stat);
        }

        self.ast
            .consume(TokenType::SEMICOLON, "Expect '}' after block.".to_string());

        self.ast
            .consume(TokenType::SEMICOLON, "expected semicolon".to_string());

        return Statement::BlockStatement(stats);
    }

    fn statement(&mut self) -> Statement {
        if self.ast.match_type(&[TokenType::PRINT]) {
            return Self::print_stat(self);
        }

        if self.ast.match_type(&[TokenType::VAR]) {
            return Self::declare_stat(self);
        }

        if self.ast.match_type(&[TokenType::LEFT_BRACE]) {
            return Self::block_stat(self);
        }

        let expr = self.ast.expression();
        self.ast
            .consume(TokenType::SEMICOLON, "expected semicolon".to_string());
        return Statement::ExprStatement(expr);
    }

    pub fn parse_tree(&mut self) {
        while !self.ast.is_at_end() {
            let stat = Self::statement(self);
            stat.accept();
            self.stats.push(stat);
        }
    }
}
