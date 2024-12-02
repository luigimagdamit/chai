
mod scanner;  // Import the Scanner module
mod parser;
mod common;
mod llvm;
mod jit;

use common::flags::{PARSE_DECLARATION_MODE, PARSE_EXPRESSION_MODE};

use parser::parser::Parser;
use std::io::{self, Write};
use jit::compile::jit_compile;

fn chai_title() -> String {
    String::from(r#"
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠿⠿⠿⢿⣿⣿⣽⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠿⣿⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁⠀⣠⣄⡀⠀⠙⢿⣿⣿⣿⣿⣿⣿⣿⡿⠋⠁⠀⠀⠀⠈⠻⣿⣻⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣟⣿⣿⡿⠁⠀⣰⣾⣿⣿⣿⣦⡀⠀⠛⠛⠛⠛⠛⠛⠁⠀⠀⣰⣿⣿⣿⣦⠀⠀⠺⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⡿⠀⠀⣼⣿⣿⣿⣿⣿⣿⣿⣤⣤⢤⢰⢠⡄⡄⣤⣴⣿⣿⣿⣿⣿⣿⣷⡄⠀⢹⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⠿⠧⠄⠸⠿⠿⠿⠿⠿⠿⢿⣿⣿⣿⣿⣿⡿⡷⢟⠿⠿⠿⠿⠿⠿⠿⠿⠿⣿⣤⣼⣿⣿⣿⣿⣿⣿
⣿⠛⠻⠿⣿⠀⢀⣀⣀⣤⣤⣄⣀⣀⣀⣠⡍⠈⣿⣿⣿⣿⣿⣿⡟⠀⣀⣀⣀⣀⣀⣀⣀⣀⣀⡀⠀⣹⣿⡿⠟⠻⣿
⣿⣦⣤⣀⣀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠿⠿⠿⠿⠻⠟⠇⠀⢹⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠋⠉⠀⣀⣴⣿
⠋⠉⠀⠀⢹⠀⢸⣿⣿⠟⠻⣿⣿⣿⣿⣿⡇⠀⣠⣤⣤⣤⣤⣤⣤⠀⢸⣿⣿⣿⣿⡿⠛⢿⣿⣿⠀⠀⠀⠙⠛⠛⢿
⣷⣾⣿⣿⣿⠀⠘⠿⠿⠤⠤⠿⠛⠛⠛⠛⠁⠀⣿⣻⣿⣿⣿⣿⣿⠀⠈⠛⠛⠛⠿⠧⠤⠼⠿⠇⠀⣼⣶⣶⣶⣤⣼
⣿⣿⣿⣿⣿⡆⠀⠀⣔⣯⡵⣠⣤⣤⣤⣤⣤⡀⠉⠉⠉⠉⡀⠀⠈⢠⣤⣤⣤⣤⣤⣤⣤⣤⣤⡀⠀⣿⣿⣿⣿⣿⣿
⣿⣿⡿⣫⣥⣲⣤⣴⣼⣿⣇⣏⠟⠻⠿⠿⢿⣿⠀⠀⠂⠀⠛⠀⢠⣿⣿⣿⣿⣿⣿⣿⡿⠿⠋⠀⣠⣿⣿⣿⣿⣿⣿
⣿⣟⠀⢿⣿⣿⣿⣿⣼⣿⣿⣿⠇⢤⣤⣀⣠⣿⣦⣤⣤⣶⣤⣤⣾⣿⣿⣿⣿⣿⡁⠀⢀⡀⠀⠀⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣇⠈⢿⣿⣿⣿⣷⣿⣿⣿⠄⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠛⠁⢀⣴⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣤⡙⢻⡿⣿⠿⢛⠁⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣽⣿⣿⣿⣿⣿⣿⣿⣿⡀⠀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣷⣶⣶⣶⣶⣿⠀⠸⡿⠛⠿⠛⠛⠛⠃⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⣿⠇⠀⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆⠀⠁⢀⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣴⣦⡀⠀⠀⣠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣯⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
"#)
}

fn repl() -> io::Result<()>{
    println!("\x1b[93m{}\nchai v0.00.1 REPL", chai_title());
    println!("\x1b[93mtype something!\x1b[0m");
    let mut body = "".to_string();
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read REPL");
        let source = input.trim();
        if source == "exit" {
            println!("\x1b[93mchai takes a nap...\x1b[0m");
            std::process::exit(0);
        }
        let jit = jit_compile(source);
        body += &jit.unwrap();
        println!("{}", body)
        
    }
}
fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        if PARSE_EXPRESSION_MODE { let _ = repl(); }
        else { panic!("[ExprModeError] PARSE_EXPRESSION_MODE flag is not enabled") }
    } else {
        match &args[1] {
            s if s.is_ascii()=> {
                if !PARSE_DECLARATION_MODE { panic!("[DeclrModeError] PARSE_DECLARATION_MODE flag is not enabled")}
                let contents = std::fs::read_to_string(s).unwrap();
                let parser = &mut Parser::init_parser(&contents);
                parser.compile();
            }
            _ => {}
        }
        
    }
}
