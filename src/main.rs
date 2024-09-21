use rlox_twi::*;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt().unwrap(),
        2 => run_file(args[1]).unwrap(),
        _ => {
            eprintln!("Usage: jlox [script]");
            process::exit(64);
        }
    }
}
