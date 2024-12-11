mod error;
mod expr;
// mod interpreter;
mod parser;
mod scanner;
mod token;
mod value;

use error::Result;
// use interpreter::Interpreter;
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
    let scanner = Scanner::new(&source);
    let tokens = scanner.scan_tokens()?;

    let parser = Parser::new(&tokens);
    let expr = parser.parse()?;

    println!("{}", expr.pretty_print());

    // Interpreter::interpret(expr)?;

    Ok(())
}
