use std::{env, fs, process};

use interpreter::run;
use prompt::run_prompt;

mod error;
mod interpreter;
mod prompt;
mod lexing;
mod parsing;
mod runtime;

fn main() {
    let mut args = env::args();
    args.next();

    if args.len() > 2 {
        print!("Usage: rloxi [script]");
        process::exit(1);
    }
    if let Some(path) = args.next() {
        let script = fs::read_to_string(path).expect("Failed to read file");
        if let Err(_err) = run(script) {
            process::exit(1);
        }
    } else {
        run_prompt();
    }
}
