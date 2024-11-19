use std::fmt;

#[derive(Debug, Clone)]
pub struct LoxError(pub Vec<(usize, &'static str)>);

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for error in &self.0 {
            write!(f, "Line: {}, Cause: {}", error.0, error.1)?;
        }
        Ok(())
    }
}

impl std::error::Error for LoxError {}
