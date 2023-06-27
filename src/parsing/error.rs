use crate::{
    error::report_error,
    lexing::token::{Token, TokenType},
};

#[derive(Clone, Copy, Debug)]
pub enum ExpectAfter {
    While,
    Expression,
    If,
    Condition,
    For,
    ForClauses,
    Value,
    Declaration,
    LoopCondition,
}

impl std::fmt::Display for ExpectAfter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectAfter::While => write!(f, "'while'"),
            ExpectAfter::Expression => write!(f, "expression"),
            ExpectAfter::If => write!(f, "'if'"),
            ExpectAfter::Condition => write!(f, "condition"),
            ExpectAfter::For => write!(f, "'for'"),
            ExpectAfter::Value => write!(f, "value"),
            ExpectAfter::Declaration => write!(f, "declaration"),
            ExpectAfter::LoopCondition => write!(f, "loop condition"),
            ExpectAfter::ForClauses => write!(f, "for clauses")
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ParseErrorMessage {
    ExpectLeftParen(ExpectAfter),
    ExpectRightParen(ExpectAfter),
    ExpectExpression,
    ExpectSemicolon(ExpectAfter),
    ExpectBraceAfterBlock,
    ExpectVariableName,
    InvalidAssignmentTarget,
}

impl std::fmt::Display for ParseErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorMessage::ExpectRightParen(after) => write!(f, "Expect ')' after {}.", after),
            ParseErrorMessage::ExpectLeftParen(after) => write!(f, "Expect '(' after {}.", after),
            ParseErrorMessage::ExpectExpression => write!(f, "Expect expression."),
            ParseErrorMessage::ExpectSemicolon(after) => write!(f, "Expect ';' after {}.", after),
            ParseErrorMessage::ExpectVariableName => write!(f, "Expect variable name."),
            ParseErrorMessage::InvalidAssignmentTarget => write!(f, "Invalid assignment target."),
            ParseErrorMessage::ExpectBraceAfterBlock => write!(f, "Expect '}}' after block."),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: ParseErrorMessage,
}

pub fn report_parser_error(error: ParseError) -> ParseError {
    if error.token.token_type == TokenType::Eof {
        report_error(error.token.line, " at end", &error.message.to_string());
    } else {
        let mut location = String::from(" at '");
        location.push_str(&error.token.lexeme);
        location.push('\'');
        report_error(error.token.line, &location, &error.message.to_string());
    }
    return error;
}
