use std::io::Write;

use crate::interpreter::run;

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}

pub fn run_prompt() {
    loop {
        let input = prompt("> ");
        if input == "exit" {
            break;
        }
        run(input).unwrap();
    }
}
