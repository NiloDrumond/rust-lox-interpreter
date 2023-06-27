use derive_more::Display;

#[derive(Debug, Display, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl TokenType {
    pub fn from_keyword(word: &str) -> Option<TokenType> {
        match word {
            "and" => Some(Self::And),
            "or" => Some(Self::Or),
            "class" => Some(Self::Class),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "nil" => Some(Self::Nil),
            "print" => Some(Self::Print),
            "return" => Some(Self::Return),
            "super" => Some(Self::Super),
            "this" => Some(Self::This),
            "true" => Some(Self::True),
            "false" => Some(Self::False),
            "var" => Some(Self::Var),
            "fun" => Some(Self::Fun),
            "for" => Some(Self::For),
            "while" => Some(Self::While),
            _ => None,
        }
    }
}

#[derive(Debug, Display, Clone)]
pub enum TokenLiteral {
    None,
    String(String),
    Number(f64),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: TokenLiteral,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: TokenLiteral, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{0} {1} {2}", self.token_type, self.lexeme, self.literal)
    }
}
