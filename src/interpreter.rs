use crate::error::LoxError;
use crate::expr::Expr;
use crate::value::Value;

pub struct Interpreter {}

impl<'token, 'lexeme, 'err> Interpreter {
    pub fn interpret(expr: Expr<'token, 'lexeme>) -> Result<Value, LoxError<'err>> {
        Ok(expr.interpret()?)
    }
}
