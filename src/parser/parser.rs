use std::collections::HashMap;

use crate::common::common::{PARSE_EXPRESSION_MODE, PARSE_FN_OUTPUT, PARSE_SUPRESS_PREDEFINES, PARSE_TOKEN_OUTPUT};
use crate::scanner::{
    token::{Token, TokenType},
    scanner::Scanner
};

use crate::common::error::ErrorCode;
use crate::parser::expr::Expr;
use crate::llvm::llvm_print::{llvm_fmt_string_int, llvm_main_close, llvm_main_start, llvm_print_bool_declare, llvm_print_define, llvm_print_i32_define};
use crate::parser::parse_fn::declaration;

use super::expr::DataType;
use super::parse_fn::expression;

pub struct StringEntry {
    pub codegen: String,
    pub length: usize,
    pub index: usize
}
pub struct SymbolTableEntry {
    pub name: String,
    pub count: usize,
    pub variable_type: DataType
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
    pub symbol_table: HashMap<String, SymbolTableEntry>,
    pub expr_count: u32,
    pub compilation: String
}
impl<'a>Parser <'a>{
    pub fn new_expr(&mut self, expr: Expr) {
        self.constant_stack.push(Some(expr));
        self.expr_count += 1;
    }
    pub fn error_at(&self, token: &Token, message: &str) {

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

        panic!()

    } 
    pub fn error_at_previous(&mut self, message: &str) {
        if let Some(prev) = self.previous {
            self.error_at(&prev, message);
        } else {
            panic!("[ParserDev] - Tried generating a error message at previous but failed");
        }
    }
    pub fn advance(&mut self) {
        self.previous = self.current.take();
        loop {
            let token = self.get_token();
            
            match token.token_type {
                TokenType::Error(_) => 
                    self.error_at(&token, token.start),
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
            constant_stack: Vec::new(),
            string_table: HashMap::new(),
            symbol_table: HashMap::new(),
            expr_count: 0,
            compilation: String::from("")
        }
    }
    #[allow(unused)]
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
    // debug purposes only
    pub fn expression_mode(&mut self) {
        while !self.match_current(TokenType::EOF) {
            declaration(self);   
        }
        for (_, entry) in &self.string_table {
            println!("{}",entry.codegen);
        }
    }
    pub fn declaration_mode(&mut self) {
        while !self.match_current(TokenType::EOF) {
            declaration(self);   
        }
        for (_, entry) in &self.string_table {
            println!("{}",entry.codegen);
        }
    }
    pub fn compile(&mut self) {
        self.advance();
        if !PARSE_EXPRESSION_MODE {
            self.llvm_stdlib();
            self.declaration_mode();
        } else {

            self.expression_mode();

        }
    }
}





