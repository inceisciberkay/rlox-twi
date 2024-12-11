use crate::error::{LexError, LoxError};
use crate::token::{Literal, Token, TokenType};

use std::collections::HashMap;
use std::str;
use std::sync::OnceLock;

pub struct Scanner<'lexeme> {
    source: &'lexeme [u8],
    tokens: Vec<Token<'lexeme>>,
    start: usize,
    current: usize,
    line: usize,
    errors: Vec<LexError>,
}

impl<'lexeme, 'err> Scanner<'lexeme> {
    pub fn new(source: &'lexeme str) -> Self {
        Self {
            source: source.as_bytes(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
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

    pub fn scan_tokens(mut self) -> Result<Vec<Token<'lexeme>>, LoxError<'err>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, b"", Option::None, self.line));

        if self.errors.is_empty() {
            Ok(self.tokens)
        } else {
            Err(self.errors.into())
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            b'(' => self.add_token(TokenType::LeftParen, Option::None),
            b')' => self.add_token(TokenType::RightParen, Option::None),
            b'{' => self.add_token(TokenType::LeftBrace, Option::None),
            b'}' => self.add_token(TokenType::RightBrace, Option::None),
            b',' => self.add_token(TokenType::Comma, Option::None),
            b'.' => self.add_token(TokenType::Dot, Option::None),
            b'-' => self.add_token(TokenType::Minus, Option::None),
            b'+' => self.add_token(TokenType::Plus, Option::None),
            b';' => self.add_token(TokenType::Semicolon, Option::None),
            b'*' => self.add_token(TokenType::Star, Option::None),
            b'!' => {
                let token_type = if self.match_char(b'=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, Option::None)
            }
            b'=' => {
                let token_type = if self.match_char(b'=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, Option::None)
            }
            b'<' => {
                let token_type = if self.match_char(b'=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type, Option::None)
            }
            b'>' => {
                let token_type = if self.match_char(b'=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type, Option::None)
            }
            b'/' => {
                if self.match_char(b'/') {
                    while self.peek() != Some(b'\n') {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            b' ' | b'\r' | b'\t' => (),
            b'\n' => self.line += 1,
            b'"' => self.string(),
            b'0'..=b'9' => self.number(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.identifier(),
            _ => self.errors.push(LexError {
                line: self.line,
                msg: "Unexpected character",
            }),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal<'lexeme>>) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn match_char(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> Option<u8> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source[self.current])
        }
    }

    fn peek_next(&self) -> Option<u8> {
        if self.current + 1 >= self.source.len() {
            None
        } else {
            Some(self.source[self.current + 1])
        }
    }

    fn string(&mut self) {
        while self.peek() != Some(b'"') {
            if self.peek() == Some(b'\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.push(LexError {
                line: self.line,
                msg: "Unterminated string",
            });
        }

        self.advance(); // closing "

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(
            TokenType::String,
            Some(Literal::String(str::from_utf8(value).unwrap())),
        )
    }

    fn is_digit(c: Option<u8>) -> bool {
        if let Some(c) = c {
            c.is_ascii_digit()
        } else {
            false
        }
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == Some(b'.') && Self::is_digit(self.peek_next()) {
            self.advance(); // consume .
            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = &self.source[self.start..self.current];
        self.add_token(
            TokenType::Number,
            Some(Literal::Number(
                str::from_utf8(value).unwrap().parse().unwrap(),
            )),
        );
    }

    fn is_alpha_numeric(c: Option<u8>) -> bool {
        if let Some(c) = c {
            c.is_ascii_alphanumeric()
        } else {
            false
        }
    }

    fn identifier(&mut self) {
        while Self::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let value = &self.source[self.start..self.current];
        if let Some(keyword) = Self::keywords_lookup_table().get(str::from_utf8(value).unwrap()) {
            self.add_token(*keyword, None);
        } else {
            self.add_token(TokenType::Identifier, None);
        }
    }
}
