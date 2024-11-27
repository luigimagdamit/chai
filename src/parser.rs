use crate::common::{PARSE_FN_OUTPUT, PARSE_TOKEN_OUTPUT};
use crate::token::{Token, TokenType};
use crate::scanner::Scanner;
use crate::error::ErrorCode;
use crate::parse_fn::expression;
use crate::expr::Expr;


#[allow(unused)]
pub struct Parser<'a>{
    pub current: Option<Token<'a>>,
    pub previous: Option<Token<'a>>,
    pub scanner: Scanner<'a>,
    had_error: bool,
    panic_mode: bool,
    // pub left_hand: Option<Expr>,
    // pub right_hand: Option<Expr>
    pub constant_stack: Vec<Option<Expr>>
}
impl<'a>Parser <'a>{
    pub fn error_at(&mut self, token: &Token, message: &str) {
        self.panic_mode = true;
        let stderr = format!("Line: {} - ", token.line);
        match token.token_type {
            TokenType::EOF => {
                println!("{} at end of file", token.start)
            },
            TokenType::Error(loc) => {
                println!("[{}] {} {} at `{}`", ErrorCode::SyntaxError, stderr, token.start, &self.scanner.get_lexeme(loc));
            },
            _ => {
                println!("[{}] {} {}  at `{}`", ErrorCode::CompilerError, stderr, message, token.start);
            }
        }
        //std::process::exit(1);
        // if self.panic_mode {
        //     return
        // }
    } 
    pub fn advance(&mut self) {
        self.previous = self.current.take();
        loop {
            let token = self.get_token();
            
            match token.token_type {
                TokenType::Error(_)=> {
                    self.error_at(&token, token.start);
                },
                _ => {
                    self.current = Some(token);
                    if PARSE_TOKEN_OUTPUT { println!("[CompilerSuccess] {}", token)}
                    break;
                }
            }
        }

    }
    pub fn consume(&mut self, token_type: TokenType, message: &str) {
        if let Some(token) = self.current {
            match token.token_type {
                _ if token.token_type != token_type => {
                    self.error_at(&token, message);
                },
                _ => {
                    self.advance();
                }
            }
        }
    }
    fn get_token(&mut self) -> Token<'a> {
        return self.scanner.scan_token().clone()
    }
    pub fn init_parser(source: &'a str) -> Parser<'_> {
        Parser {
            current: None,
            previous: None,
            scanner: Scanner::init_scanner(&source),
            panic_mode: false,
            had_error: false,
            // left_hand: None,
            // right_hand: None
            constant_stack: Vec::new()
        }
    }
    pub fn print_parser(&mut self) {
        if PARSE_FN_OUTPUT {
            println!("<Parser State> ");

            let stack = &mut self.constant_stack;
            println!("<Stack Top>");
            println!("{}", stack.len());
            for expr in  stack{
                expr.clone().unwrap().print_leaf();
            }
            println!("</Parser State>")
        }
        
    }
    pub fn compile(&mut self) {
        self.advance();
        expression(self);
        self.consume(TokenType::EOF, "Expect end of expression");

    }
}





