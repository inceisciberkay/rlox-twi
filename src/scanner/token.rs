#[derive(Debug, Copy, Clone)]
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
    r#type: TokenType,
    lexeme: &'a [u8],
    literal: Option<Literal<'a>>,
    line: usize,
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
