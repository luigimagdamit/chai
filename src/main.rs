mod token;    // Import the Token and TokenType module
mod scanner;  // Import the Scanner module
mod parser;
mod error; 
mod parse_rule;
mod precedence;
mod parse_fn;
mod expr;
mod common;

use parser::Parser;
use std::io::{self, Write};


fn main() {
    loop {
        let mut input = String::new();
        print!(">");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read REPL");
        let source = input.trim();
        if source == "exit" {
            break
        }
        let parser = &mut Parser::init_parser(source);
        parser.compile(); // warmup
    }
    


}
