use crate::error::RuntimeError;
use crate::token::{Token, TokenType, Value};

// TODO: maybe represent Token as enum so that more static checking can be performed (e.g. use for Grouping: Token::LeftParen, Box<Expr>, Token::RightParen)

pub enum Expr<'tok> {
    Literal(&'tok Token<'tok>),
    Unary(&'tok Token<'tok>, Box<Expr<'tok>>),
    Binary(Box<Expr<'tok>>, &'tok Token<'tok>, Box<Expr<'tok>>),
    Grouping(&'tok Token<'tok>, Box<Expr<'tok>>, &'tok Token<'tok>),
}

impl<'tok> Expr<'tok> {
    // print in prefix notation
    fn pretty_print(self) -> String {
        match self {
            Self::Literal(tok) => String::from_utf8_lossy(tok.lexeme).into_owned(),
            Self::Unary(tok, expr) => {
                "(".to_owned() + &String::from_utf8_lossy(tok.lexeme) + &expr.pretty_print() + ")"
            }
            Self::Binary(l_expr, tok, r_expr) => {
                "(".to_owned()
                    + &String::from_utf8_lossy(tok.lexeme)
                    + &l_expr.pretty_print()
                    + &r_expr.pretty_print()
                    + ")"
            }
            Self::Grouping(l_tok, expr, r_tok) => {
                "(".to_owned()
                    + &String::from_utf8_lossy(l_tok.lexeme)
                    + &expr.pretty_print()
                    + &String::from_utf8_lossy(r_tok.lexeme)
                    + ")"
            }
        }
    }

    pub fn interpret(&self) -> Result<Value, RuntimeError<'tok>> {
        match self {
            Self::Literal(tok) => match &tok.literal {
                Some(val) => Ok(val.into()),
                None => Err(RuntimeError {
                    tok,
                    msg: "Literal with no value",
                }),
            },
            Self::Unary(tok, expr) => {
                let right = expr.interpret()?;
                match (tok.r#type, &right) {
                    (TokenType::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
                    (TokenType::Bang, Value::Boolean(_)) => {
                        Ok(Value::Boolean(!Expr::is_truthy(&right)))
                    }
                    _ => Err(RuntimeError {
                        tok,
                        msg: "Invalid unary expression",
                    }),
                }
            }
            Self::Binary(l_expr, tok, r_expr) => {
                let left = l_expr.interpret()?;
                let right = r_expr.interpret()?;
                match (&left, tok.r#type, &right) {
                    // arithmetic operators
                    (Value::Number(a), TokenType::Minus, Value::Number(b)) => {
                        Ok(Value::Number(a - b))
                    }
                    (Value::Number(a), TokenType::Plus, Value::Number(b)) => {
                        Ok(Value::Number(a + b))
                    }
                    (Value::String(s1), TokenType::Plus, Value::String(s2)) => {
                        Ok(Value::String(s1.to_owned() + &s2))
                    }
                    (Value::Number(a), TokenType::Star, Value::Number(b)) => {
                        Ok(Value::Number(a * b))
                    }
                    (Value::Number(a), TokenType::Slash, Value::Number(b)) => {
                        Ok(Value::Number(a / b))
                    }
                    // comparison operators
                    (Value::Number(a), TokenType::Greater, Value::Number(b)) => {
                        Ok(Value::Boolean(a > b))
                    }
                    (Value::Number(a), TokenType::GreaterEqual, Value::Number(b)) => {
                        Ok(Value::Boolean(a >= b))
                    }
                    (Value::Number(a), TokenType::Less, Value::Number(b)) => {
                        Ok(Value::Boolean(a < b))
                    }
                    (Value::Number(a), TokenType::LessEqual, Value::Number(b)) => {
                        Ok(Value::Boolean(a <= b))
                    }
                    (_, TokenType::BangEqual, _) => {
                        Ok(Value::Boolean(!Expr::is_equal(&left, &right)))
                    }
                    (_, TokenType::EqualEqual, _) => {
                        Ok(Value::Boolean(Expr::is_equal(&left, &right)))
                    }
                    _ => Err(RuntimeError {
                        tok,
                        msg: "Invalid binary expression",
                    }),
                }
            }
            Self::Grouping(_, expr, _) => expr.interpret(),
        }
    }

    fn is_truthy(val: &Value) -> bool {
        match val {
            &Value::Nil => false,
            &Value::Boolean(val) => val,
            _ => true,
        }
    }

    fn is_equal(val1: &Value, val2: &Value) -> bool {
        match (val1, val2) {
            (&Value::Nil, &Value::Nil) => true,
            (&Value::Number(n1), &Value::Number(n2)) => n1 == n2,
            (&Value::String(ref s1), &Value::String(ref s2)) => s1 == s2,
            (&Value::Boolean(b1), &Value::Boolean(b2)) => b1 == b2,
            _ => false,
        }
    }
}
