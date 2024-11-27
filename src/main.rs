mod token;    // Import the Token and TokenType module
mod scanner;  // Import the Scanner module
mod parser;
mod error; 
mod parse_rule;
mod precedence;
mod parse_fn;
mod expr;
mod common;
mod llvm_primitives;

use parser::Parser;
use std::io::{self, Write};
use std::fs::{self};

fn repl() {
    loop {
        let mut input = String::new();
        print!(">");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read REPL");
        let source = input.trim();
        if source == "exit" {
            break
        }



        // std::mem::replace(&mut stdout, writer);

        let parser = &mut Parser::init_parser(source);
        parser.compile(); // warmup


    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        let parser = &mut Parser::init_parser("21 * 10 + 21 * 10");
        parser.compile(); // warmup
    } else {
        match &args[1] {
            s if s == "repl" => repl(),
            s if s == "cmd" => {

                    let parser = &mut Parser::init_parser(&args[2]);
                    parser.compile();

                
            }
            s if s.is_ascii()=> {

                let contents = fs::read_to_string(s).unwrap();
                let parser = &mut Parser::init_parser(&contents);
                parser.compile();
            }
            _ => {}
        }
        
    }
}
