mod error;
mod scanner;

use error::LoxError;
use scanner::Scanner;
use std::io::{self, Write};
use std::result;

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
