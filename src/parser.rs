use std::string::ParseError;

use crate::expr::Expr;
use crate::token::{Token, TokenType};

pub struct Parser<'a> {
    tokens: &'a Vec<Token<'a>>,
    curr: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, curr: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        Ok(self.expression())
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let expr = comparison();
    }

    fn comparison(&mut self) -> Expr {
        let expr = term();
    }

    fn term(&mut self) -> Expr {
        let expr = factor();
    }

    fn factor(&mut self) -> Expr {
        let expr = unary();
    }

    fn unary(&mut self) -> Expr {}

    fn match_token_types(&self, token_types: &Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
            }
            return true;
        }
        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.curr += 1;
        }
        self.previous()
    }

    fn check(&self, r#type: TokenType) -> bool {
        if !self.is_at_end() {
            false
        } else {
            self.peek().r#type == r#type
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().r#type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.curr]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.curr - 1]
    }
}
