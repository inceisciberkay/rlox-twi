mod scanner;

use scanner::Scanner;
use std::fmt;
use std::result;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct LoxError(Vec<(usize, &'static str)>);

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for error in &self.0 {
            write!(f, "Line: {}, Cause: {}", error.0, error.1)?;
        }
        Ok(())
    }
}

impl std::error::Error for LoxError {}

type Result = result::Result<(), Box<dyn std::error::Error>>;
type LoxResult = result::Result<(), LoxError>;

pub fn run_prompt() -> Result {
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    loop {
        print!("> ");
        io::stdout().flush()?;
        if let Ok(_) = stdin.read_line(&mut buffer) {
            run(&buffer)?;
        } else {
            break;
        }
    }

    Ok(())
}

pub fn run_file(path: &str) -> Result {
    let source = std::fs::read_to_string(path)?;
    run(&source)?;

    Ok(())
}

fn run(source: &str) -> LoxResult {
    let mut scanner = Scanner::new(&source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
