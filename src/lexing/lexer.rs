use super::{
    error::{report_lexer_error, LexerError, LexerErrorMessage},
    token::{Token, TokenLiteral, TokenType},
};

pub struct Lexer {
    pub source: String,
    pub tokens: Vec<Token>,
    pub error_found: bool,

    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        return Self {
            source,
            tokens: vec![],
            error_found: false,
            start: 0,
            current: 0,
            line: 1,
        };
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn advance(&mut self) -> char {
        let char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        return char;
    }

    fn add_token(&mut self, token_type: TokenType, literal: TokenLiteral) {
        let lexeme = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            token_type,
            String::from(lexeme),
            literal,
            self.line,
        ));
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        };
        let Some(next) = self.source.chars().nth(self.current) else {
                return false;
            };
        if next != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        };
        return self.source.chars().nth(self.current).unwrap();
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn is_digit(c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_alpha(c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_alphanumeric(c: char) -> bool {
        return Self::is_digit(c) || Self::is_alpha(c);
    }

    fn string(&mut self) {
        loop {
            if self.peek() == '"' || self.is_at_end() {
                break;
            }
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            report_lexer_error(LexerError {
                line: self.line,
                message: LexerErrorMessage::UnterminatedString,
            });
            return;
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, TokenLiteral::String(String::from(value)));
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token(
            TokenType::Number,
            TokenLiteral::Number(self.source[self.start..self.current].parse().unwrap()),
        );
    }

    fn identifier(&mut self) {
        while Self::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let word = &self.source[self.start..self.current];
        let token_type = TokenType::from_keyword(word).unwrap_or(TokenType::Identifier);

        self.add_token(token_type, TokenLiteral::None);
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, TokenLiteral::None),
            ')' => self.add_token(TokenType::RightParen, TokenLiteral::None),
            '{' => self.add_token(TokenType::LeftBrace, TokenLiteral::None),
            '}' => self.add_token(TokenType::RightBrace, TokenLiteral::None),
            ',' => self.add_token(TokenType::Comma, TokenLiteral::None),
            '.' => self.add_token(TokenType::Dot, TokenLiteral::None),
            '-' => self.add_token(TokenType::Minus, TokenLiteral::None),
            '+' => self.add_token(TokenType::Plus, TokenLiteral::None),
            ';' => self.add_token(TokenType::Semicolon, TokenLiteral::None),
            '*' => self.add_token(TokenType::Star, TokenLiteral::None),
            '!' => {
                let token_type = if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, TokenLiteral::None)
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, TokenLiteral::None)
            }
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type, TokenLiteral::None)
            }
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type, TokenLiteral::None)
            }
            '/' => {
                if self.match_next('/') {
                    loop {
                        if self.is_at_end() || self.peek() == '\n' {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, TokenLiteral::None)
                }
            }

            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }

            '"' => self.string(),
            _ if Self::is_digit(c) => {
                self.number();
            }
            _ if Self::is_alpha(c) => {
                self.identifier();
            }
            _ => {
                report_lexer_error(LexerError {
                    line: self.line,
                    message: LexerErrorMessage::UnexpectedCharacter,
                });
            }
        }
    }

    pub fn scan_tokens(&mut self) -> (&Vec<Token>, bool) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenType::Eof, TokenLiteral::None);

        return (&self.tokens, self.error_found);
    }
}
