mod token;    // Import the Token and TokenType module
mod scanner;  // Import the Scanner module

use scanner::Scanner;
use token::TokenType;
use std::fmt;
enum ErrorCode {
    SyntaxError
}
impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::SyntaxError => write!(f, "SyntaxError")
        }
    }
}
fn main() {

    let scanner = &mut Scanner::init_scanner("++-=,.!\"hello there0");

    while !scanner.is_at_end() {
        let s = scanner.scan_token();
        println!("{} {}", s.token_type, s.start);
        match s.token_type {
            TokenType::Error => {
                println!("[ {} ] {}", ErrorCode::SyntaxError, s.start)
            }
            TokenType::EOF => { break }
            _ => {}
        }
    }
    println!("Scanner has gracefully exited: {}", scanner.is_at_end());

}
