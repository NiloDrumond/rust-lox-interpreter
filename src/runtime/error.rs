use crate::{error::report_error, lexing::token::Token};

#[derive(Debug, Clone)]
pub enum RuntimeErrorMessage {
    OperandMustBeNumber,
    OperandsMustBeNumbers,
    OperandsMustBeNumberOrString,
    UndefinedVariable(String),
}

impl std::fmt::Display for RuntimeErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeErrorMessage::OperandMustBeNumber => write!(f, "Operand must be a number."),
            RuntimeErrorMessage::OperandsMustBeNumbers => write!(f, "Operands must be numbers."),
            RuntimeErrorMessage::OperandsMustBeNumberOrString => {
                write!(f, "Operands must be two numbers or two strings")
            }
            RuntimeErrorMessage::UndefinedVariable(name) => {
                write!(f, "Undefined variable '{}'.", name)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub token: Token,
    pub message: RuntimeErrorMessage,
}

pub fn report_runtime_error(error: RuntimeError) -> RuntimeError {
    report_error(error.token.line, "", &error.message.to_string());
    return error;
}
