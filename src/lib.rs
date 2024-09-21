use std::collections::HashMap;

fn run_prompt() -> Result<(), std::io::Error> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer)?;
    run(buffer)?;
    Ok(())
}

fn run_file(path: String) -> Result<(), std::io::Error> {
    let source = std::fs::read_to_string(path)?;
    run(source)?;
    Ok(())
}

fn run(source: String) -> Result<(), std::io::Error> {
    Ok(())
}

#[derive(Debug)]
enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or, Print, Return, Super, This, True, Var, While,

    EOF,
}

#[derive(Debug)]
struct Token {
    r#type: TokenType,
    lexeme: String,
    literal: Option<LiteralType>,
    line: usize,
}

impl Token {
    fn new(r#type: TokenType, lexeme: String, literal: Option<LiteralType>, line: i32) -> Self {
        Token {r#type, lexeme, literal, line}
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
        Self { source, tokens: Vec::new(), start: 0, current: 0, line: 1 }
    }

    fn generate_keywords_lookup_table() -> HashMap<&'static str, TokenType> {
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
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {r#type: TokenType::EOF, lexeme: String::new(), literal: Option::None, line: self.line});
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&self) {
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
            '!' => self.add_token(if self.match_char('=') {TokenType::BangEqual} else {TokenType::Bang}, Option::None),
            '=' => self.add_token(if self.match_char('=') {TokenType::EqualEqual} else {TokenType::Equal}, Option::None),
            '<' => self.add_token(if self.match_char('=') {TokenType::LessEqual} else {TokenType::Less}, Option::None),
            '>' => self.add_token(if self.match_char('=') {TokenType::GreaterEqual} else {TokenType::Greater}, Option::None),
            _ => {
                panic!("Unexpected character");
            },
        }
    }

    fn match_char(&self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn add_token(&self, r#type: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {r#type, lexeme: text.to_string(), literal, line: self.line});
    }

    fn advance(&self) -> char {
        let current_char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        current_char
    }
}

