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
pub struct Token<'lexeme> {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: &'lexeme [u8],
    pub(crate) literal: Option<Literal<'lexeme>>,
    pub(crate) line: usize,
}

impl<'lexeme> Token<'lexeme> {
    pub fn new(
        token_type: TokenType,
        lexeme: &'lexeme [u8],
        literal: Option<Literal<'lexeme>>,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    // used for error handling (tokens are dropped after run() function, but error handling is done in main())
    pub fn deep_clone(&self) -> Token<'static> {
        Token {
            token_type: self.token_type,
            lexeme: Box::leak(self.lexeme.to_vec().into_boxed_slice()),
            literal: self.literal.as_ref().map(|lit| lit.deep_clone()),
            line: self.line,
        }
    }
}

#[derive(Debug)]
pub enum Literal<'lexeme> {
    String(&'lexeme str),
    Number(f64),
}

impl<'lexeme> Literal<'lexeme> {
    // used for error handling (tokens are dropped after run() function, but error handling is done in main())
    pub fn deep_clone(&self) -> Literal<'static> {
        match self {
            Literal::String(s) => Literal::String(Box::leak(s.to_string().into_boxed_str())),
            Literal::Number(n) => Literal::Number(*n),
        }
    }
}
