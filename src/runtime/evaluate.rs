use std::{cell::RefCell, rc::Rc};

use crate::parsing::{
    expr::LiteralValue,
    stmt::{BlockStmt, ExpressionStmt, IfStmt, PrintStmt, Stmt, VarStmt, WhileStmt},
};

use super::{
    environment::Environment,
    error::RuntimeError,
    interpret::{is_truthy, ExprInterpret},
};

pub trait EvaluateStmt {
    fn evaluate(&mut self, environment: Rc<RefCell<Environment>>) -> Result<(), RuntimeError>;
}

impl EvaluateStmt for Stmt {
    fn evaluate(&mut self, environment: Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {
        match self {
            Stmt::ExpressionStmt(stmt) => stmt.evaluate(environment),
            Stmt::PrintStmt(stmt) => stmt.evaluate(environment),
            Stmt::VarStmt(stmt) => stmt.evaluate(environment),
            Stmt::BlockStmt(stmt) => stmt.evaluate(environment),
            Stmt::IfStmt(stmt) => stmt.evaluate(environment),
            Stmt::WhileStmt(stmt) => stmt.evaluate(environment),
        }
    }
}

impl EvaluateStmt for ExpressionStmt {
    fn evaluate(&mut self, environment: Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {
        self.expression.interpret(environment)?;
        Ok(())
    }
}

impl EvaluateStmt for PrintStmt {
    fn evaluate(&mut self, environment: Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {
        let value = self.expression.interpret(environment)?;
        println!("{:?}", value);
        Ok(())
    }
}

impl EvaluateStmt for VarStmt {
    fn evaluate(&mut self, environment: Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {
        let mut value = LiteralValue::None;
        if let Some(expr) = &self.initializer {
            value = expr.interpret(environment.clone())?;
        }

        environment.borrow_mut().define(self.name.lexeme.clone(), value);

        Ok(())
    }
}

impl EvaluateStmt for BlockStmt {
    fn evaluate(&mut self, environment: Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {
        let current_environment = Rc::new(RefCell::new(Environment::new(Some(environment))));
        for statement in &mut self.statements {
            statement.evaluate(current_environment.clone())?;
        }
        return Ok(());
    }
}

impl EvaluateStmt for IfStmt {
    fn evaluate(&mut self, environment: Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {
        let result = self.condition.interpret(environment.clone())?;
        if is_truthy(&result) {
            self.then_branch.evaluate(environment)?;
            return Ok(());
        }
        if let Some(else_branch) = &mut self.else_branch {
            else_branch.evaluate(environment)?;
        }
        return Ok(());
    }
}

impl EvaluateStmt for WhileStmt {
    fn evaluate(&mut self, environment: Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {
        while is_truthy(&self.condition.interpret(environment.clone())?) {
            self.body.evaluate(environment.clone())?;
        }
        return Ok(());
    }
}
