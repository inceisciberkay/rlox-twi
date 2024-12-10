#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens
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

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
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

    EOF,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub(crate) r#type: TokenType,
    pub(crate) lexeme: &'a [u8],
    pub(crate) literal: Option<Literal<'a>>,
    pub(crate) line: usize,
}

impl<'a> Token<'a> {
    pub fn new(
        r#type: TokenType,
        lexeme: &'a [u8],
        literal: Option<Literal<'a>>,
        line: usize,
    ) -> Self {
        Token {
            r#type,
            lexeme,
            literal,
            line,
        }
    }
}

#[derive(Debug)]
pub enum Literal<'a> {
    String(&'a str),
    Number(f64),
}

#[derive(Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl<'a> From<&Literal<'a>> for Value {
    fn from(literal: &Literal<'a>) -> Self {
        match *literal {
            Literal::String(s) => Value::String(s.to_owned()),
            Literal::Number(n) => Value::Number(n),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}
