use crate::error::Result;
use crate::expr::Expr;

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret<'a>(expr: &Expr<'a>) -> Result<'a> {
        match expr.interpret() {
            Ok(value) => {
                println!("{}", value);
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}
