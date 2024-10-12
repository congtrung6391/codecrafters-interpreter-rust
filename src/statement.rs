use crate::{expr::{Expression, AST}, token::TokenType};

pub enum Statement {
    PrintStatement(Expression),
    ExprStatement(Expression),
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

    fn statement(&mut self) -> Statement {
        if self.ast.match_type(&[TokenType::PRINT]) {
            self.ast.advance();
            let expr = self.ast.expression();
            self.ast.consume(TokenType::SEMICOLON, "expected semicolon".to_string());
            let lit = expr.accept();
            lit.print();
            return Statement::PrintStatement(expr);
        }

        let expr = self.ast.expression();
        self.ast.consume(TokenType::SEMICOLON, "expected semicolon".to_string());
        return Statement::ExprStatement(expr);
    }

    pub fn parse_tree(&mut self) {
        while !self.ast.is_at_end() {
            let stat = Self::statement(self);
            self.stats.push(stat);
        }
    }
}
