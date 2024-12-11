use crate::token::Literal;

#[derive(Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl From<&Literal<'_>> for Value {
    fn from(literal: &Literal<'_>) -> Self {
        match *literal {
            Literal::String(s) => Value::String(s.to_owned()),
            Literal::Number(n) => Value::Number(n),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}
