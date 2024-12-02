use super::{
    llvm_print::*,
    llvm_primitives::llvm_top_level_expr
};
use crate::{
    scanner::token::TokenType,
    parser::parser::Parser,
    common::flags::{PARSE_EXPRESSION_MODE, EXPR_ONLY}
};

pub fn expr_mode(parser: &mut Parser) {
    if let Some(eof) = parser.current {
        match eof.token_type {
            TokenType::EOF if PARSE_EXPRESSION_MODE => {
                top_level_expr(parser);
                if !EXPR_ONLY {
                    llvm_main_start();
                    llvm_call_print(0, "i32");
                    llvm_main_close();
                }
                
                
            },
            _ => {}
        }
        
    }
}

#[allow(unused)]
pub fn top_level_expr(parser: &mut Parser) {
    if let Some(constant) = parser.constant_stack.pop() {
        // println!("{}", &constant.unwrap().left);
        let expr_eval = &constant.unwrap();
        llvm_top_level_expr(&expr_eval.right, &expr_eval.data_type, 0);
    }
}