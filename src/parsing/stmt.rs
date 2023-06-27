use crate::lexing::token::Token;

use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    BlockStmt(BlockStmt),
    ExpressionStmt(ExpressionStmt),
    PrintStmt(PrintStmt),
    VarStmt(VarStmt),
    IfStmt(Box<IfStmt>),
    WhileStmt(Box<WhileStmt>),
}

#[derive(Debug, Clone)]
pub struct ExpressionStmt {
    pub expression: Expr,
}

#[derive(Debug, Clone)]
pub struct PrintStmt {
    pub expression: Expr,
}

#[derive(Debug, Clone)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Stmt,
    pub else_branch: Option<Stmt>,
}

#[derive(Clone, Debug)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Stmt,
}
