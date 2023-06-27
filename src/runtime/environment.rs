use std::{collections::HashMap, cell::RefCell, rc::Rc};
use crate::{lexing::token::Token, parsing::expr::LiteralValue};
use super::error::{RuntimeError, RuntimeErrorMessage};

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn assign(&mut self, token: Token, value: LiteralValue) -> Result<(), RuntimeError> {
        if let Some(_) = self.values.get(&token.lexeme) {
            self.values.entry(token.lexeme).and_modify(|v| {
                *v = value;
            });
            return Ok(());
        };
        if let Some(enclosing) = &mut self.enclosing {
            return enclosing.borrow_mut().assign(token, value);
        };
        return Err(RuntimeError {
            token: token.clone(),
            message: RuntimeErrorMessage::UndefinedVariable(token.lexeme),
        });
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, token: Token) -> Result<LiteralValue, RuntimeError> {
        if let Some(value) = self.values.get(&token.lexeme) {
            return Ok(value.clone());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(token);
        }

        return Err(RuntimeError {
            token: token.clone(),
            message: RuntimeErrorMessage::UndefinedVariable(token.lexeme),
        });
    }
}
