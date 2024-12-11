use crate::error::RuntimeError;
use crate::token::{Token, TokenType};
use crate::value::Value;

pub enum Expr<'token, 'lexeme> {
    Literal(Value),
    Unary(&'token Token<'lexeme>, Box<Expr<'token, 'lexeme>>),
    Binary(
        Box<Expr<'token, 'lexeme>>,
        &'token Token<'lexeme>,
        Box<Expr<'token, 'lexeme>>,
    ),
    Grouping(Box<Expr<'token, 'lexeme>>),
}

impl<'token, 'lexeme, 'err> Expr<'token, 'lexeme> {
    // print in prefix notation
    #[allow(dead_code)]
    pub fn pretty_print(&self) -> String {
        match self {
            Self::Literal(val) => val.to_string(),
            Self::Unary(token, expr) => {
                "(".to_owned() + &String::from_utf8_lossy(token.lexeme) + &expr.pretty_print() + ")"
            }
            Self::Binary(l_expr, token, r_expr) => {
                "(".to_owned()
                    + &String::from_utf8_lossy(token.lexeme)
                    + " "
                    + &l_expr.pretty_print()
                    + " "
                    + &r_expr.pretty_print()
                    + ")"
            }
            Self::Grouping(expr) => "(".to_owned() + &expr.pretty_print() + ")",
        }
    }

    pub fn interpret(self) -> Result<Value, RuntimeError<'err>> {
        match self {
            Self::Literal(val) => Ok(val),
            Self::Unary(token, expr) => {
                let right = expr.interpret()?;
                match (token.token_type, &right) {
                    (TokenType::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
                    (TokenType::Bang, Value::Boolean(_)) => {
                        Ok(Value::Boolean(!Expr::is_truthy(&right)))
                    }
                    _ => Err(RuntimeError {
                        token: token.deep_clone(),
                        msg: "Invalid unary expression",
                    }),
                }
            }
            Self::Binary(l_expr, token, r_expr) => {
                let left = l_expr.interpret()?;
                let right = r_expr.interpret()?;
                match (&left, token.token_type, &right) {
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

                    // error cases
                    (_, TokenType::Plus, _) => Err(RuntimeError {
                        token: token.deep_clone(),
                        msg:
                            "Invalid binary expression: Operands must be two numbers or two strings",
                    }),
                    (_, TokenType::Minus, _)
                    | (_, TokenType::Star, _)
                    | (_, TokenType::Slash, _)
                    | (_, TokenType::Greater, _)
                    | (_, TokenType::GreaterEqual, _)
                    | (_, TokenType::Less, _)
                    | (_, TokenType::LessEqual, _) => Err(RuntimeError {
                        token: token.deep_clone(),
                        msg: "Invalid binary expression: Operands must be two numbers",
                    }),
                    _ => Err(RuntimeError {
                        token: token.deep_clone(),
                        msg: "Invalid binary expression: reason unknown",
                    }),
                }
            }
            Self::Grouping(expr) => expr.interpret(),
        }
    }

    fn is_truthy(val: &Value) -> bool {
        match val {
            Value::Nil => false,
            Value::Boolean(val) => *val,
            _ => true,
        }
    }

    fn is_equal(val1: &Value, val2: &Value) -> bool {
        match (val1, val2) {
            (Value::Nil, Value::Nil) => true,
            (Value::Number(n1), Value::Number(n2)) => n1 == n2,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (Value::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
            _ => false,
        }
    }
}
