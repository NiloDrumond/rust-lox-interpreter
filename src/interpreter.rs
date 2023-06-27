use std::{cell::RefCell, rc::Rc};

use crate::{
    lexing::lexer::Lexer,
    parsing::parser::Parser,
    runtime::{environment::Environment, evaluate::EvaluateStmt},
};

pub fn run(source: String) -> Result<(), ()> {
    let mut lexer = Lexer::new(source);
    let (tokens, lexer_error_found) = lexer.scan_tokens();
    let mut parser = Parser::new(tokens.to_vec());
    let statements = parser.parse();
    if let Ok(statements) = statements {
        let environment = Rc::new(RefCell::new(Environment::new(None)));
        for mut statement in statements {
            let res = statement.evaluate(environment.clone());
            if let Err(_) = res {
                return Err(());
            }
        }
        if lexer_error_found {
            return Err(());
        }
    }
    return Err(());
}
