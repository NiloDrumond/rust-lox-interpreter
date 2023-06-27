use crate::{
    lexing::token::{Token, TokenLiteral, TokenType},
    parsing::expr::{GroupingExpr, LiteralExpr},
};

use super::{
    error::{report_parser_error, ExpectAfter, ParseError, ParseErrorMessage},
    expr::{AssignExpr, BinaryExpr, Expr, LiteralValue, LogicalExpr, UnaryExpr, VariableExpr},
    stmt::{BlockStmt, ExpressionStmt, IfStmt, PrintStmt, Stmt, VarStmt, WhileStmt},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        return Ok(statements);
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.type_match(vec![TokenType::Var]) {
            let result = self.var_declaration();
            match result {
                Ok(statement) => {
                    return Ok(statement);
                }
                Err(err) => {
                    self.synchronize();
                    return Err(err);
                }
            }
        }
        let result = self.statement();
        if let Err(_) = result {
            self.synchronize();
        }
        return result;
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, ParseErrorMessage::ExpectVariableName)?;

        let mut initializer: Option<Expr> = None;
        if self.type_match(vec![TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenType::Semicolon,
            ParseErrorMessage::ExpectSemicolon(ExpectAfter::Declaration),
        )?;
        return Ok(Stmt::VarStmt(VarStmt { name, initializer }));
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.type_match(vec![TokenType::For]) {
            return self.for_statement();
        }
        if self.type_match(vec![TokenType::If]) {
            return self.if_statement();
        }
        if self.type_match(vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.type_match(vec![TokenType::While]) {
            return self.while_statement();
        }
        if self.type_match(vec![TokenType::LeftBrace]) {
            let statements = self.block()?;
            return Ok(Stmt::BlockStmt(BlockStmt { statements }));
        }

        return self.expression_statement();
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(
            TokenType::LeftParen,
            ParseErrorMessage::ExpectLeftParen(ExpectAfter::For),
        )?;

        let initializer;
        if self.type_match(vec![TokenType::Semicolon]) {
            initializer = None;
        } else if self.type_match(vec![TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition = None;

        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(
            TokenType::Semicolon,
            ParseErrorMessage::ExpectSemicolon(ExpectAfter::LoopCondition),
        )?;

        let mut increment = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(
            TokenType::RightParen,
            ParseErrorMessage::ExpectRightParen(ExpectAfter::ForClauses),
        )?;
        let mut body = self.statement()?;
        if let Some(increment) = increment {
            let increment = Stmt::ExpressionStmt(ExpressionStmt {
                expression: increment,
            });
            body = Stmt::BlockStmt(BlockStmt {
                statements: vec![body, increment],
            });
        }

        let condition = condition.unwrap_or(Expr::LiteralExpr(LiteralExpr {
            value: LiteralValue::Bool(true),
        }));
        body = Stmt::WhileStmt(Box::new(WhileStmt { condition, body }));

        if let Some(initializer) = initializer {
            body = Stmt::BlockStmt(BlockStmt { statements: vec![initializer, body] });
        }

        return Ok(body);
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(
            TokenType::LeftParen,
            ParseErrorMessage::ExpectLeftParen(ExpectAfter::While),
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            ParseErrorMessage::ExpectRightParen(ExpectAfter::Condition),
        )?;
        let body = self.statement()?;

        return Ok(Stmt::WhileStmt(Box::new(WhileStmt { condition, body })));
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(
            TokenType::LeftParen,
            ParseErrorMessage::ExpectLeftParen(ExpectAfter::If),
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            ParseErrorMessage::ExpectRightParen(ExpectAfter::Condition),
        )?;

        let then_branch = self.statement()?;
        let mut else_branch = None;
        if self.type_match(vec![TokenType::Else]) {
            else_branch = Some(self.statement()?);
        }

        return Ok(Stmt::IfStmt(Box::new(IfStmt {
            condition,
            then_branch,
            else_branch,
        })));
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(
            TokenType::RightBrace,
            ParseErrorMessage::ExpectBraceAfterBlock,
        )?;
        return Ok(statements);
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            ParseErrorMessage::ExpectSemicolon(ExpectAfter::Value),
        )?;
        return Ok(Stmt::PrintStmt(PrintStmt { expression: value }));
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            ParseErrorMessage::ExpectSemicolon(ExpectAfter::Expression),
        )?;
        return Ok(Stmt::ExpressionStmt(ExpressionStmt { expression: expr }));
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        return self.assignment();
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.type_match(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::VariableExpr(expr) = expr {
                let name = expr.name;
                return Ok(Expr::AssignExpr(Box::new(AssignExpr { name, value })));
            }

            report_parser_error(ParseError {
                token: equals,
                message: ParseErrorMessage::InvalidAssignmentTarget,
            });
        }

        return Ok(expr);
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.type_match(vec![TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::LogicalExpr(Box::new(LogicalExpr {
                left: expr,
                operator,
                right,
            }))
        }

        return Ok(expr);
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.type_match(vec![TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::LogicalExpr(Box::new(LogicalExpr {
                left: expr,
                operator,
                right,
            }))
        }

        return Ok(expr);
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.type_match(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::BinaryExpr(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }));
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.type_match(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::BinaryExpr(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }))
        }
        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.type_match(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::BinaryExpr(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }))
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.type_match(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::BinaryExpr(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }))
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.type_match(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::UnaryExpr(Box::new(UnaryExpr { operator, right })));
        }

        return Ok(self.primary()?);
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.type_match(vec![TokenType::False]) {
            return Ok(Expr::LiteralExpr(LiteralExpr {
                value: LiteralValue::Bool(false),
            }));
        }
        if self.type_match(vec![TokenType::True]) {
            return Ok(Expr::LiteralExpr(LiteralExpr {
                value: LiteralValue::Bool(true),
            }));
        }
        if self.type_match(vec![TokenType::Nil]) {
            return Ok(Expr::LiteralExpr(LiteralExpr {
                value: LiteralValue::None,
            }));
        }

        if self.type_match(vec![TokenType::Number, TokenType::String]) {
            let token = self.previous();
            match token.literal {
                TokenLiteral::None => {
                    panic!();
                }
                TokenLiteral::String(text) => {
                    return Ok(Expr::LiteralExpr(LiteralExpr {
                        value: LiteralValue::String(text),
                    }))
                }
                TokenLiteral::Number(number) => {
                    return Ok(Expr::LiteralExpr(LiteralExpr {
                        value: LiteralValue::Number(number),
                    }))
                }
            }
        }

        if self.type_match(vec![TokenType::Identifier]) {
            return Ok(Expr::VariableExpr(VariableExpr {
                name: self.previous(),
            }));
        }

        if self.type_match(vec![TokenType::LeftParen]) {
            let expression = self.expression()?;
            self.consume(
                TokenType::RightParen,
                ParseErrorMessage::ExpectRightParen(ExpectAfter::Expression),
            )
            .unwrap();
            return Ok(Expr::GroupingExpr(Box::new(GroupingExpr { expression })));
        }

        let error = ParseError {
            token: self.peek().clone(),
            message: ParseErrorMessage::ExpectExpression,
        };
        return Err(report_parser_error(error));
    }

    fn type_match(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn consume(
        &mut self,
        token_type: TokenType,
        message: ParseErrorMessage,
    ) -> Result<Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        let error = ParseError {
            message,
            token: self.peek().clone(),
        };
        return Err(report_parser_error(error));
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn peek(&self) -> &Token {
        return self.tokens.get(self.current).unwrap();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::Eof;
    }

    fn previous(&self) -> Token {
        return self.tokens.get(self.current - 1).unwrap().clone();
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == token_type;
    }
}
