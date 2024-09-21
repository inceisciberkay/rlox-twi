use std::collections::HashMap;
use std::sync::OnceLock;

pub fn run_prompt() -> Result<(), std::io::Error> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer)?;
    run(buffer)?;
    Ok(())
}

pub fn run_file(path: &str) -> Result<(), std::io::Error> {
    let source = std::fs::read_to_string(path)?;
    run(source)?;
    Ok(())
}

fn run(source: String) -> Result<(), std::io::Error> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    
    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum TokenType {
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
struct Token {
    r#type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    fn new(r#type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Token {
            r#type,
            lexeme,
            literal,
            line,
        }
    }
}

#[derive(Debug)]
enum Literal {
    String(String),
    Number(f64),
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn keywords_lookup_table() -> &'static HashMap<&'static str, TokenType> {
        static HASHMAP: OnceLock<HashMap<&'static str, TokenType>> = OnceLock::new();
        HASHMAP.get_or_init(|| {
            let mut keywords = HashMap::new();
            keywords.insert("and", TokenType::And);
            keywords.insert("class", TokenType::Class);
            keywords.insert("else", TokenType::Else);
            keywords.insert("false", TokenType::False);
            keywords.insert("for", TokenType::For);
            keywords.insert("fun", TokenType::Fun);
            keywords.insert("if", TokenType::If);
            keywords.insert("nil", TokenType::Nil);
            keywords.insert("or", TokenType::Or);
            keywords.insert("print", TokenType::Print);
            keywords.insert("return", TokenType::Return);
            keywords.insert("super", TokenType::Super);
            keywords.insert("this", TokenType::This);
            keywords.insert("true", TokenType::True);
            keywords.insert("var", TokenType::Var);
            keywords.insert("while", TokenType::While);
            keywords
        })
    }

    fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            TokenType::EOF,
            String::new(),
            Option::None,
            self.line,
        ));
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, Option::None),
            ')' => self.add_token(TokenType::RightParen, Option::None),
            '{' => self.add_token(TokenType::LeftBrace, Option::None),
            '}' => self.add_token(TokenType::RightBrace, Option::None),
            ',' => self.add_token(TokenType::Comma, Option::None),
            '.' => self.add_token(TokenType::Dot, Option::None),
            '-' => self.add_token(TokenType::Minus, Option::None),
            '+' => self.add_token(TokenType::Plus, Option::None),
            ';' => self.add_token(TokenType::Semicolon, Option::None),
            '*' => self.add_token(TokenType::Star, Option::None),
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, Option::None)
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, Option::None)
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type, Option::None)
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type, Option::None)
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != None && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => {
                panic!("Unexpected character");
            }
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> Option<char> {
        // return the current char
        if self.is_at_end() {
            // TODO: this may be redundant since .nth() returns option
            None
        } else {
            Some(self.source.chars().nth(self.current).unwrap())
        }
    }

    fn peek_next(&self) -> Option<char> {
        // return the current char
        if self.current + 1 >= self.source.len() {
            // TODO: this may be redundant since .nth() returns option
            None
        } else {
            Some(self.source.chars().nth(self.current + 1).unwrap())
        }
    }

    fn add_token(&mut self, r#type: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            r#type,
            text.to_string(),
            literal,
            self.line,
        ));
    }

    fn advance(&mut self) -> char {
        let current_char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        current_char
    }

    fn string(&mut self) {
        loop {
            if self.is_at_end() {
                break;
            } // TODO: do I really need to check it? when self.peek() == None?
            match self.peek() {
                Some('"') => {
                    break;
                }
                Some('\n') => {
                    self.line += 1;
                }
                None => {
                    panic!("Peek error");
                }
                _ => (),
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string at line {}", self.line);
        }

        self.advance(); // closing "

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Some(Literal::String(value.to_string())))
    }

    fn number(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance()
            } else {
                break;
            };
        }

        if self.peek() != None
            && self.peek().unwrap() == '.'
            && self.peek_next() != None
            && self.peek_next().unwrap().is_ascii_digit()
        {
            self.advance(); // consume .
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    self.advance()
                } else {
                    break;
                };
            }
        }

        let value = &self.source[self.start..self.current];
        self.add_token(
            TokenType::Number,
            Some(Literal::Number(value.parse().unwrap())),
        );
    }

    fn identifier(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_ascii_alphanumeric() {
                break;
            }
            self.advance();
        }

        let value = &self.source[self.start..self.current];
        if let Some(keyword) = Self::keywords_lookup_table().get(value) {
            self.add_token(*keyword, None);
        } else {
            self.add_token(TokenType::Identifier, None);
        }
    }
}
