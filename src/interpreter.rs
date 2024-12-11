use crate::error::Result;
use crate::expr::Expr;

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(expr: Expr) -> Result {
        let value = expr.interpret()?;
        println!("{}", value);
        Ok(())
    }
}
