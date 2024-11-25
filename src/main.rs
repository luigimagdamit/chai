mod token;    // Import the Token and TokenType module
mod scanner;  // Import the Scanner module
use std::fmt;
use scanner::Scanner;
use token::{TokenType, ErrorCode, Token};

struct Parser<'a>{
    current: Option<token::Token<'a>>,
    previous: Option<token::Token<'a>>,
    scanner: Scanner<'a>
}
impl<'a>Parser <'a>{
    fn errorAt(&self, token: &Token, message: &str) {
        let stderr = format!("Line: {} - ", token.line);
        match token.token_type {
            TokenType::EOF => {
                println!("{} at end of file", token.start)
            },
            TokenType::Error(loc) => {
                println!("{} {} at `{}...`", stderr, token.start, &self.scanner.get_lexeme(loc));
            },
            _ => {
                println!("{} at `{}`", stderr, token.start);
            }
        }
    } 
    fn advance(&mut self) {
        self.previous = self.current.take();
        
        println!("Scanner State");
        while let token = self.get_token() {
            match token.token_type {
                TokenType::Error(_)=> {
                    self.errorAt(&token, token.start);
                },
                _ => {
                    self.current = Some(token);
                    match self.previous {
                        Some(t) => println!("\t{}", t),
                        _ => println!("\t<no token>")
                    }
                    match self.current {
                        Some(t) => println!("\t{}", t),
                        _ => println!("<\tno token>")
                    }

                    break;
                }
            }
        }

    }
    fn get_token(&mut self) -> Token<'a> {
        return self.scanner.scan_token().clone()
    }
    fn init_parser(source: &'static str) -> Parser<'_> {
        Parser {
            current: None,
            previous: None,
            scanner: Scanner::init_scanner(&source)
        }
    }
    fn compile(&mut self) {
        self.advance();
    }
}
impl <'a> fmt::Display for Parser<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.previous, self.current) {
            (Some(t1), Some(t2)) => {
                write!(f, "{} {}", t1, t2);
            }
            (Some(t1), None) => {
                write!(f, "{} {}", t1, "<no token>");
            },
            (None, Some(t2)) => {
                write!(f, "{} {}", "<no token>", t2);
            }
            (None, None) => {
                write!(f, "{} {}", "<no token>", "<no token>");
            }
        }
        match self.current {
            Some(token) => write!(f, "{}", token),
            _ => write!(f, "<no token>")
        }
        // write!(f, "Previous: {} Current: {}", self.previous.unwrap(), self.current)
    }
}

fn main() {
    let source = "+=!===";
    //let scanner = &mut Scanner::init_scanner();

    // while !scanner.is_at_end() {
    //     let s = scanner.scan_token();
    //     println!("{} {}", s.token_type, s.start);
    //     match s.token_type {
    //         TokenType::Error(location) => {
    //             println!("[ {} ] {} ", ErrorCode::SyntaxError, &scanner.get_lexeme(location))
    //         }
    //         TokenType::EOF => { break }
    //         _ => {}
    //     }
    // }
    // println!("Scanner has gracefully exited: {}", scanner.is_at_end());
    let parser = &mut Parser::init_parser(source);
    
    while !parser.scanner.is_at_end() {
        parser.compile();
    }

}
