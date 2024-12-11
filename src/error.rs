use crate::token::Token;
use crate::token::TokenType;
use std::fmt;
use std::result;
use std::str;

pub type Result = result::Result<(), Box<dyn std::error::Error>>;

// General Error Type
#[derive(Debug)]
pub enum LoxError<'a> {
    Lexer(Vec<LexError>),
    Parser(Vec<ParseError<'a>>),
    Runtime(RuntimeError<'a>),
}

impl fmt::Display for LoxError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxError::Lexer(errors) => {
                writeln!(f, "Lexical Errors:")?;
                for err in errors {
                    writeln!(f, "{}", err)?;
                }
            }
            LoxError::Parser(errors) => {
                writeln!(f, "Parse Errors:")?;
                for err in errors {
                    writeln!(f, "{}", err)?;
                }
            }
            LoxError::Runtime(error) => {
                write!(f, "Runtime Error: {}", error)?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for LoxError<'_> {}

// Lexer Error
#[derive(Debug)]
pub struct LexError {
    pub line: usize,
    pub msg: &'static str,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line: {}, Cause: {}", self.line, self.msg)?;
        Ok(())
    }
}

impl From<Vec<LexError>> for LoxError<'_> {
    fn from(errors: Vec<LexError>) -> Self {
        LoxError::Lexer(errors)
    }
}

// Parser Error
#[derive(Debug)]
pub struct ParseError<'a> {
    pub token: Token<'a>,
    pub msg: &'static str,
}

impl fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lexeme = if self.token.token_type == TokenType::EOF {
            "eof"
        } else {
            str::from_utf8(self.token.lexeme).unwrap()
        };

        write!(
            f,
            "Line: {}, Token: {}, Cause: {}",
            self.token.line, lexeme, self.msg
        )?;
        Ok(())
    }
}

impl<'a> From<Vec<ParseError<'a>>> for LoxError<'a> {
    fn from(errors: Vec<ParseError<'a>>) -> Self {
        LoxError::Parser(errors)
    }
}

// Runtime Error
#[derive(Debug)]
pub struct RuntimeError<'a> {
    pub token: Token<'a>,
    pub msg: &'static str,
}

impl fmt::Display for RuntimeError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n[line {}]", self.msg, self.token.line);
        Ok(())
    }
}

impl<'a> From<RuntimeError<'a>> for LoxError<'a> {
    fn from(error: RuntimeError<'a>) -> Self {
        LoxError::Runtime(error)
    }
}
