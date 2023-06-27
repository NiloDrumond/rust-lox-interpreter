use std::{rc::Rc, cell::RefCell};

use crate::{
    lexing::token::TokenType,
    parsing::expr::{
        AssignExpr, BinaryExpr, Expr, GroupingExpr, LiteralExpr, LiteralValue, LogicalExpr,
        UnaryExpr, VariableExpr,
    },
    runtime::error::{report_runtime_error, RuntimeErrorMessage},
};

use super::{environment::Environment, error::RuntimeError};

pub trait ExprInterpret {
    fn interpret(&self, environment: Rc<RefCell<Environment>>) -> Result<LiteralValue, RuntimeError>;
}

impl ExprInterpret for Expr {
    fn interpret(&self, environment: Rc<RefCell<Environment>>) -> Result<LiteralValue, RuntimeError> {
        match self {
            Expr::UnaryExpr(expr) => expr.interpret(environment),
            Expr::LiteralExpr(expr) => expr.interpret(environment),
            Expr::GroupingExpr(expr) => expr.interpret(environment),
            Expr::BinaryExpr(expr) => expr.interpret(environment),
            Expr::VariableExpr(expr) => expr.interpret(environment),
            Expr::AssignExpr(expr) => expr.interpret(environment),
            Expr::LogicalExpr(_) => todo!(),
        }
    }
}

impl ExprInterpret for AssignExpr {
    fn interpret(&self, environment: Rc<RefCell<Environment>>) -> Result<LiteralValue, RuntimeError> {
        let value = self.value.interpret(environment.clone())?;
        environment.borrow_mut().assign(self.name.clone(), value.clone())?;
        return Ok(value);
    }
}

impl ExprInterpret for LiteralExpr {
    fn interpret(&self, _environment: Rc<RefCell<Environment>>) -> Result<LiteralValue, RuntimeError> {
        return Ok(self.value.clone());
    }
}

impl ExprInterpret for UnaryExpr {
    fn interpret(&self, environment: Rc<RefCell<Environment>>) -> Result<LiteralValue, RuntimeError> {
        let right = self.right.interpret(environment)?;
        match self.operator.token_type {
            TokenType::Minus => {
                if let LiteralValue::Number(number) = right {
                    return Ok(LiteralValue::Number(-number));
                }
                return Err(report_runtime_error(RuntimeError {
                    token: self.operator.clone(),
                    message: RuntimeErrorMessage::OperandMustBeNumber,
                }));
            }
            TokenType::Bang => {
                return Ok(LiteralValue::Bool(!is_truthy(&right)));
            }
            _ => {
                panic!()
            }
        }
    }
}

impl ExprInterpret for GroupingExpr {
    fn interpret(&self, environment: Rc<RefCell<Environment>>) -> Result<LiteralValue, RuntimeError> {
        self.expression.interpret(environment)
    }
}

impl ExprInterpret for BinaryExpr {
    fn interpret(&self, environment: Rc<RefCell<Environment>>) -> Result<LiteralValue, RuntimeError> {
        let left = self.left.interpret(environment.clone())?;
        let right = self.right.interpret(environment)?;
        match self.operator.token_type {
            TokenType::Plus => {
                if let LiteralValue::Number(left) = left {
                    if let LiteralValue::Number(right) = right {
                        return Ok(LiteralValue::Number(left + right));
                    }
                }
                if let LiteralValue::String(left) = left {
                    if let LiteralValue::String(right) = right {
                        let mut result = String::from(&left);
                        result.push_str(&right);
                        return Ok(LiteralValue::String(result));
                    }
                }
                return Err(report_runtime_error(RuntimeError {
                    token: self.operator.clone(),
                    message: RuntimeErrorMessage::OperandsMustBeNumberOrString,
                }));
            }
            TokenType::Minus => {
                if let LiteralValue::Number(left) = left {
                    if let LiteralValue::Number(right) = right {
                        return Ok(LiteralValue::Number(left - right));
                    }
                }
                return Err(report_runtime_error(RuntimeError {
                    token: self.operator.clone(),
                    message: RuntimeErrorMessage::OperandsMustBeNumbers,
                }));
            }
            TokenType::Slash => {
                if let LiteralValue::Number(left) = left {
                    if let LiteralValue::Number(right) = right {
                        return Ok(LiteralValue::Number(left / right));
                    }
                }
                return Err(report_runtime_error(RuntimeError {
                    token: self.operator.clone(),
                    message: RuntimeErrorMessage::OperandsMustBeNumbers,
                }));
            }
            TokenType::Star => {
                if let LiteralValue::Number(left) = left {
                    if let LiteralValue::Number(right) = right {
                        return Ok(LiteralValue::Number(left * right));
                    }
                }
                return Err(report_runtime_error(RuntimeError {
                    token: self.operator.clone(),
                    message: RuntimeErrorMessage::OperandsMustBeNumbers,
                }));
            }

            TokenType::Less => {
                if let LiteralValue::Number(left) = left {
                    if let LiteralValue::Number(right) = right {
                        return Ok(LiteralValue::Bool(left < right));
                    }
                }
                return Err(report_runtime_error(RuntimeError {
                    token: self.operator.clone(),
                    message: RuntimeErrorMessage::OperandsMustBeNumbers,
                }));
            }
            TokenType::LessEqual => {
                if let LiteralValue::Number(left) = left {
                    if let LiteralValue::Number(right) = right {
                        return Ok(LiteralValue::Bool(left <= right));
                    }
                }
                return Err(report_runtime_error(RuntimeError {
                    token: self.operator.clone(),
                    message: RuntimeErrorMessage::OperandsMustBeNumbers,
                }));
            }
            TokenType::Greater => {
                if let LiteralValue::Number(left) = left {
                    if let LiteralValue::Number(right) = right {
                        return Ok(LiteralValue::Bool(left > right));
                    }
                }
                return Err(report_runtime_error(RuntimeError {
                    token: self.operator.clone(),
                    message: RuntimeErrorMessage::OperandsMustBeNumbers,
                }));
            }
            TokenType::GreaterEqual => {
                if let LiteralValue::Number(left) = left {
                    if let LiteralValue::Number(right) = right {
                        return Ok(LiteralValue::Bool(left >= right));
                    }
                }
                return Err(report_runtime_error(RuntimeError {
                    token: self.operator.clone(),
                    message: RuntimeErrorMessage::OperandsMustBeNumbers,
                }));
            }

            TokenType::EqualEqual => {
                return Ok(LiteralValue::Bool(left == right));
            }
            TokenType::BangEqual => {
                return Ok(LiteralValue::Bool(left != right));
            }
            _ => {
                panic!();
            }
        }
    }
}

impl ExprInterpret for VariableExpr {
    fn interpret(&self, environment: Rc<RefCell<Environment>>) -> Result<LiteralValue, RuntimeError> {
        environment.borrow().get(self.name.clone())
    }
}

impl ExprInterpret for LogicalExpr {
    fn interpret(&self, environment: Rc<RefCell<Environment>>) -> Result<LiteralValue, RuntimeError> {
        let left = self.left.interpret(environment.clone())?;

        if self.operator.token_type == TokenType::Or {
            if is_truthy(&left) {
                return Ok(left);
            }
        } 
        if self.operator.token_type == TokenType::And {
            if !is_truthy(&left) {
                return Ok(left);
            }
        }

        return self.right.interpret(environment);
    }
}

pub fn is_truthy(value: &LiteralValue) -> bool {
    match *value {
        LiteralValue::String(_) => return true,
        LiteralValue::Number(_) => return true,
        LiteralValue::Bool(bool) => bool,
        LiteralValue::None => return false,
    }
}
