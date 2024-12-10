mod error;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
mod token;

use error::Result;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

use std::io::{self, Write};

pub fn run_prompt() -> Result {
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    loop {
        print!("> ");
        io::stdout().flush()?;
        if let Ok(_) = stdin.read_line(&mut buffer) {
            run(&buffer)?;
            buffer.clear();
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

fn run(source: &str) -> Result {
    let mut scanner = Scanner::new(&source);
    let tokens = scanner.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let expr = parser.parse()?;

    Interpreter::interpret(&expr)?;

    // for token in tokens {
    //     println!("{:?}", token);
    // }

    Ok(())
}
