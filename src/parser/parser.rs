use std::collections::HashMap;
use std::process::exit;
use crate::common::flags::{EMIT_VERBOSE, PARSE_EXPRESSION_MODE, PARSE_FN_OUTPUT, PARSE_SUPRESS_PREDEFINES, PARSE_TOKEN_OUTPUT};
use crate::scanner::{
    token::{Token, TokenType},
    scanner::Scanner
};
use crate::parser::core::ast_node::AstNode;

use crate::common::error::ErrorCode;
use crate::parser::expression::expr::Expr;
use crate::codegen::parser_ir::ParserIR;
use crate::codegen::llvm_parser_ir::LlvmParserIR;
use crate::codegen::c_parser_ir::CParserIR;
use crate::codegen::backend_config::{get_current_backend, IRBackend};

/// Macro to execute parser IR-specific code based on current backend
macro_rules! with_parser_ir {
    ($method:ident($($args:expr),*)) => {{
        match get_current_backend() {
            IRBackend::LLVM => {
                let ir = LlvmParserIR;
                ir.$method($($args),*)
            }
            IRBackend::C => {
                let ir = CParserIR;
                ir.$method($($args),*)
            }
        }
    }};
}
use crate::parser::parse_fn::declaration;

use crate::parser::core::symbol::SymbolTableEntry;


#[allow(unused)]
pub struct StringEntry {
    pub codegen: String,
    pub length: usize,
    pub index: usize
}



#[allow(unused)]
pub struct Parser<'a>{
    pub current: Option<Token<'a>>,
    pub previous: Option<Token<'a>>,
    pub scanner: Scanner<'a>,
    had_error: bool,
    panic_mode: bool,
    pub constant_stack: Vec<Option<Expr>>,
    pub ast_stack: Vec<AstNode>,
    pub string_table: HashMap<String, StringEntry>,
    pub symbol_table: HashMap<String, SymbolTableEntry>,
    pub expr_count: u32,
    pub compilation: String,
    pub depth: u32,
}
impl<'a>Parser <'a>{
    pub fn init_parser(source: &'a str) -> Parser<'_> {
        Parser {
            current: None,
            previous: None,
            scanner: Scanner::init_scanner(source),
            panic_mode: false,
            had_error: false,
            constant_stack: Vec::new(),
            ast_stack: Vec::new(),
            string_table: HashMap::new(),
            symbol_table: HashMap::new(),
            expr_count: 0,
            compilation: String::from(""),
            depth: 0
        }
    }

    pub fn print_symbols(&self) {
        for (key, value) in &self.symbol_table {
            println!("{}: {}", key, value.variable_type);
        }
    }
    pub fn emit_instruction(&mut self, inst: &String) {
        if EMIT_VERBOSE { println!("{inst}") }
        self.compilation += &("\t");
        self.compilation += inst;
        self.compilation += &String::from("\n");
    }
    pub fn comment(&mut self, comment: &str) {
        self.compilation += &("\n\t;".to_string() + comment);
        self.compilation += &String::from("\n");
    }
    pub fn new_expr(&mut self, expr: Expr) {
        self.constant_stack.push(Some(expr));

    }
    pub fn expr_increment(&mut self) -> u32 {
        self.expr_count += 1;
        self.expr_count - 1
    }
    pub fn expr_pop(&mut self) -> (Expr, u32) {
        if let Some(expr) = self.constant_stack.pop() {
            let expr = expr.expect("expr_pop() expected a an Expr type");
            self.expr_count += 1;
            return (expr, self.expr_top())
        } else {
            panic!("{}", self.current.expect("Tried to get a token at current but it was empty").token_type);
        }
        
    }
    pub fn expr_top(&self) -> u32 {
        self.expr_count - 1
    }
    pub fn error_at(&self, token: &Token, message: &str) {

        let stderr = format!("Line: {} - ", token.line);
        match token.token_type {
            TokenType::EOF => {
                println!("\x1b[31m[{}] {} but found EOF (end of file)\x1b[0m", ErrorCode::CompilerError, message);
            },
            TokenType::Error(loc) => {
                println!("\x1b[31m[{}] {} {} at `{}`\x1b[0m", ErrorCode::SyntaxError, stderr, token.start, &self.scanner.get_lexeme(loc));

            },
            _ => {
                println!("\x1b[31m[{}] {} {} at `{}`\x1b[0m", ErrorCode::CompilerError, stderr, message, token.start);

            }
        }
        if self.current.unwrap().token_type == TokenType::EOF {
            eprintln!("\x1b[93mchai knocks out..\x1b[0m");
            exit(1);
        }


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
            current.token_type == token_type
        } else {
            false
        }
    }
    pub fn match_current(&mut self, token_type: TokenType) -> bool {
        if !self.check_current(token_type) {
            return false
        } else {
            self.advance();
            true
        }
    }
    fn get_token(&mut self) -> Token<'a> {
        return self.scanner.scan_token()
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
    fn generate_stdlib(&self) {
        with_parser_ir!(generate_stdlib());
    }
    // debug purposes only
    pub fn expression_mode(&mut self) {
        while !self.match_current(TokenType::EOF) {
            declaration(self);   
        }
        
        for entry in self.string_table.values() {
            println!("{}",entry.codegen);
            self.compilation += &entry.codegen;
        }

    }
    pub fn declaration_mode(&mut self) {
        while !self.match_current(TokenType::EOF) {
            declaration(self);   
        }
        
        

    }
    pub fn compile(&mut self) {
        self.advance();
        if !PARSE_EXPRESSION_MODE {
            self.generate_stdlib();
            self.declaration_mode();
            self.compilation += &with_parser_ir!(generate_main_close());
            self.comment("String Constants");
            let strings = self.string_table.values().clone();
            for entry in strings {
                self.compilation += &(entry.codegen.clone() + &"\n");
                if EMIT_VERBOSE {
                    println!("{}",entry.codegen);
                }

            }

        } else {

            self.expression_mode();

        }

    }
}





