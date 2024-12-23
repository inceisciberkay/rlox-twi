use rlox_twi::*;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt().unwrap(),
        2 => {
            if let Err(e) = run_file(&args[1]) {
                println!("{}", e);
            }
        }
        _ => {
            eprintln!("Usage: ./rlox or ./rlox [file]");
            process::exit(64);
        }
    }
}
