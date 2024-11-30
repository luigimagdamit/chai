use std::collections::HashMap;

use crate::common::common::{PARSE_EXPRESSION_MODE, PARSE_FN_OUTPUT, PARSE_SUPRESS_PREDEFINES, PARSE_TOKEN_OUTPUT};
use crate::scanner::{
    token::{Token, TokenType},
    scanner::Scanner
};

use crate::common::error::ErrorCode;
use crate::parser::expr::Expr;
use crate::llvm::llvm_print::{llvm_fmt_string_int, llvm_print_bool_declare, llvm_print_define, llvm_print_i32_define};
use crate::parser::parse_fn::declaration;

use super::parse_fn::expression;

pub struct StringEntry {
    pub codegen: String,
    pub length: usize,
    pub index: u32
}
#[allow(unused)]
pub struct Parser<'a>{
    pub current: Option<Token<'a>>,
    pub previous: Option<Token<'a>>,
    pub scanner: Scanner<'a>,
    had_error: bool,
    panic_mode: bool,
    // pub left_hand: Option<Expr>,
    // pub right_hand: Option<Expr>
    pub constant_stack: Vec<Option<Expr>>,
    pub string_table: HashMap<String, StringEntry>,
    pub expr_count: u32
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
        std::process::exit(1);

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
    pub fn check_current(&mut self, token_type: TokenType) -> bool {
        if let Some(current) = self.current {
            return current.token_type == token_type
        } else {
            return false
        }
    }
    pub fn match_current(&mut self, token_type: TokenType) -> bool {
        if !self.check_current(token_type) {
            return false
        } else {
            self.advance();
            return true
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
            constant_stack: Vec::new(),
            string_table: HashMap::new(),
            expr_count: 0
        }
    }
    pub fn print_parser(&mut self) {
        if PARSE_FN_OUTPUT {
            println!("<Parser State> ");
            
            println!("\n\tPrevious: {}\n\tCurrent {}", self.previous.unwrap(), self.current.unwrap());
            println!("</Parser State>")
        }
        
    }
    // Should always be included
    fn llvm_stdlib(&self) {
        if !PARSE_SUPRESS_PREDEFINES {
            llvm_print_define();
            llvm_print_bool_declare();
            llvm_fmt_string_int();
            llvm_print_i32_define();
        }
        
    }
    pub fn compile(&mut self) {
        self.llvm_stdlib();
        // llvm_main_start();
        self.advance();

        if !PARSE_EXPRESSION_MODE {
            while !self.match_current(TokenType::EOF) {
                declaration(self);
            }
            for (str, entry) in &self.string_table {
                println!("{}",entry.codegen);
            }
        } else {
            expression(self);
            self.consume(TokenType::EOF, "Expect end of expression");
        }
        
        // llvm_call_print_local(self.expr_count - 1, "i32");
        // llvm_main_close();
        

    }
}





