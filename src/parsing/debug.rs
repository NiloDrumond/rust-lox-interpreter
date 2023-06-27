use super::expr::{Expr, LiteralExpr, LiteralValue, UnaryExpr, BinaryExpr, GroupingExpr, VariableExpr};

trait ExprPrint {
    fn print(&self) -> String;
}

fn parenthesize(name: String, exprs: Vec<&Expr>) -> String {
    let mut resultant = String::from("(");
    resultant.push_str(&name);
    for expr in exprs {
        resultant.push(' ');
        resultant.push_str(&expr.print());
    }
    resultant.push(')');

    return resultant;
}

impl ExprPrint for Expr {
    fn print(&self) -> String {
        match self {
            Expr::UnaryExpr(expr) => expr.print(),
            Expr::LiteralExpr(expr) => expr.print(),
            Expr::GroupingExpr(expr) => expr.print(),
            Expr::BinaryExpr(expr) => expr.print(),
            Expr::VariableExpr(expr) => expr.print(),
            Expr::AssignExpr(_) => todo!(),
            Expr::LogicalExpr(_) => todo!(),
        }
    }
}

impl ExprPrint for LiteralExpr {
    fn print(&self) -> String {
        let val = match &self.value {
            LiteralValue::String(text) => text.clone(),
            LiteralValue::Number(number) => number.to_string(),
            LiteralValue::Bool(bool) => bool.to_string(),
            LiteralValue::None => "nil".to_string(),
        };
        return val;
    }
}

impl ExprPrint for UnaryExpr {
    fn print(&self) -> String {
        return parenthesize(self.operator.lexeme.to_string(), vec![&self.right]);
    }
}

impl ExprPrint for BinaryExpr {
    fn print(&self) -> String {
        return parenthesize(
            self.operator.lexeme.to_string(),
            vec![&self.left, &self.right],
        );
    }
}

impl ExprPrint for GroupingExpr {
    fn print(&self) -> String {
        return parenthesize("group".to_string(), vec![&self.expression]);
    }
}

impl ExprPrint for VariableExpr {
    fn print(&self) -> String {
        return String::from(&self.name.lexeme);
    }
}
