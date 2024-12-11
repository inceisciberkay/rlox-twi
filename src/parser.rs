use crate::error::{LoxError, ParseError};
use crate::expr::Expr;
use crate::token::{Token, TokenType};
use crate::value::Value;
use std::cell::Cell;

pub struct Parser<'token, 'lexeme, 'err: 'token + 'lexeme> {
    tokens: &'token Vec<Token<'lexeme>>,
    curr: Cell<usize>,
    errors: Vec<ParseError<'err>>,
}

impl<'token, 'lexeme, 'err> Parser<'token, 'lexeme, 'err> {
    pub fn new(tokens: &'token Vec<Token<'lexeme>>) -> Self {
        Self {
            tokens,
            curr: Cell::new(0),
            errors: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Result<Expr<'token, 'lexeme>, LoxError<'err>> {
        let expr = self.expression();
        match expr {
            Ok(expr) => Ok(expr),
            Err(e) => {
                self.errors.push(e);
                // // TODO: implement error recovery and synchronization logic
                Err(self.errors.into())
            }
        }
    }

    fn expression(&self) -> Result<Expr<'token, 'lexeme>, ParseError<'err>> {
        self.equality()
    }

    // TODO: Create handler function for left-associative rules to simplify redundant code (while loops are nearly identical except token types and variable 'right')
    fn equality(&self) -> Result<Expr<'token, 'lexeme>, ParseError<'err>> {
        let mut expr = self.comparison()?;

        while self.match_token_types(&vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.prev();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&self) -> Result<Expr<'token, 'lexeme>, ParseError<'err>> {
        let mut expr = self.term()?;

        while self.match_token_types(&vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.prev();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn term(&self) -> Result<Expr<'token, 'lexeme>, ParseError<'err>> {
        let mut expr = self.factor()?;

        while self.match_token_types(&vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.prev();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&self) -> Result<Expr<'token, 'lexeme>, ParseError<'err>> {
        let mut expr = self.unary()?;

        while self.match_token_types(&vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.prev();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&self) -> Result<Expr<'token, 'lexeme>, ParseError<'err>> {
        if self.match_token_types(&vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.prev();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        Ok(self.primary()?)
    }

    fn primary(&self) -> Result<Expr<'token, 'lexeme>, ParseError<'err>> {
        if self.match_token_types(&vec![TokenType::False]) {
            Ok(Expr::Literal(Value::Boolean(false)))
        } else if self.match_token_types(&vec![TokenType::True]) {
            Ok(Expr::Literal(Value::Boolean(true)))
        } else if self.match_token_types(&vec![TokenType::Nil]) {
            Ok(Expr::Literal(Value::Nil))
        } else if self.match_token_types(&vec![TokenType::Number, TokenType::String]) {
            Ok(Expr::Literal(
                (self.prev().literal.as_ref().unwrap()).into(),
            ))
        } else if self.match_token_types(&vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            Ok(Expr::Grouping(Box::new(expr)))
        } else {
            Err(ParseError {
                token: self.peek().deep_clone(),
                msg: "Expect expression",
            })
        }
    }

    fn match_token_types(&self, token_types: &Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&self) -> &'token Token<'lexeme> {
        if !self.is_at_end() {
            self.curr.set(self.curr.get() + 1);
        }
        self.prev()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn consume(
        &self,
        token_type: TokenType,
        msg: &'static str,
    ) -> Result<&'token Token<'lexeme>, ParseError<'err>> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError {
                token: self.peek().deep_clone(),
                msg,
            })
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &'token Token<'lexeme> {
        &self.tokens[self.curr.get()]
    }

    fn prev(&self) -> &'token Token<'lexeme> {
        &self.tokens[self.curr.get() - 1]
    }
}
