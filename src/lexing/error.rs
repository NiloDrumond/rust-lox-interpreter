use crate::error::report_error;

pub enum LexerErrorMessage {
    UnterminatedString,
    UnexpectedCharacter,
}

impl std::fmt::Display for LexerErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerErrorMessage::UnterminatedString => write!(f, "Unterminated string."),
            LexerErrorMessage::UnexpectedCharacter => write!(f, "Unexpected character."),
        }
    }
}

pub struct LexerError {
    pub line: usize,
    pub message: LexerErrorMessage,
}

pub fn report_lexer_error(error: LexerError) -> LexerError {
    report_error(error.line, "", &error.message.to_string());
    return error;
}
