
mod scanner;  // Import the Scanner module
mod parser;
mod common;
mod llvm;

use common::common::{PARSE_DECLARATION_MODE, PARSE_EXPRESSION_MODE};
use parser::parser::Parser;
use std::io::{self, Write};
use std::fs::{self};

fn repl() {
    loop {
        let mut input = String::new();
        print!("> ");
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
fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        if PARSE_EXPRESSION_MODE { repl() }
        else { panic!("[ExprModeError] PARSE_EXPRESSION_MODE flag is not enabled") }
    } else {
        match &args[1] {

            s if s.is_ascii()=> {
                if !PARSE_DECLARATION_MODE { panic!("[DeclrModeError] PARSE_DECLARATION_MODE flag is not enabled")}
                let contents = fs::read_to_string(s).unwrap();
                let parser = &mut Parser::init_parser(&contents);
                parser.compile();
            }
            _ => {}
        }
        
    }
}
