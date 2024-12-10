use crate::token::Token;
use std::fmt;
use std::result;

pub type Result = result::Result<(), Box<dyn std::error::Error>>;

// Lexer Error
#[derive(Debug, Clone)]
pub struct LexError(pub Vec<(usize, &'static str)>);

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for error in &self.0 {
            write!(f, "Line: {}, Cause: {}", error.0, error.1)?;
        }
        Ok(())
    }
}

impl std::error::Error for LexError {}

// Parser Error
#[derive(Debug, Clone)]
pub struct ParseError(pub Vec<(usize, &'static str)>);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for error in &self.0 {
            write!(f, "Line: {}, Cause: {}", error.0, error.1)?;
        }
        Ok(())
    }
}

impl std::error::Error for ParseError {}

// Runtime Error
#[derive(Debug)]
pub struct RuntimeError<'a> {
    pub tok: Token<'a>,
    pub msg: &'static str,
}

impl fmt::Display for RuntimeError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n[line {}]", self.msg, self.tok.line);
        Ok(())
    }
}

impl std::error::Error for RuntimeError<'_> {}
