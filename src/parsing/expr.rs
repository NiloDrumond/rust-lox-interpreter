use crate::lexing::token::Token;

#[derive(Clone, Debug)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Bool(bool),
    None,
}

impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::None, Self::None) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    AssignExpr(Box<AssignExpr>),
    UnaryExpr(Box<UnaryExpr>),
    LiteralExpr(LiteralExpr),
    GroupingExpr(Box<GroupingExpr>),
    BinaryExpr(Box<BinaryExpr>),
    VariableExpr(VariableExpr),
    LogicalExpr(Box<LogicalExpr>),
}

#[derive(Clone, Debug)]
pub struct AssignExpr {
    pub name: Token,
    pub value: Expr,
}

#[derive(Clone, Debug)]
pub struct LiteralExpr {
    pub value: LiteralValue,
}

#[derive(Clone, Debug)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Expr,
}

#[derive(Clone, Debug)]
pub struct BinaryExpr {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

#[derive(Clone, Debug)]
pub struct GroupingExpr {
    pub expression: Expr,
}

#[derive(Clone, Debug)]
pub struct VariableExpr {
    pub name: Token,
}

#[derive(Clone, Debug)]
pub struct LogicalExpr {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}
