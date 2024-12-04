use crate::parser::expression::expr::{DataType, Expr};
use crate::parser::parse_fn::expression;
use crate::parser::parser::Parser;
use crate::{common::flags::PARSE_DECLARATION_MODE, llvm::llvm_print::llvm_call_print_local, scanner::token::TokenType};

use super::variable::LlvmTempRegister;


// all printed items should pass through this!
pub enum LlvmCallPrint {
    String(u32), //register value
    Integer(u32),
}
impl LlvmCallPrint {
    pub fn call_print(&self) -> String {
        match self {
            Self::String(register) => format!("\tcall i32 (i8*, ...) @printf(i8* %{})\t\t\t\t\t\t\t\t\t\t\t; Auto generated by LlvmCallPrint (print.rs)\n", register),
            Self::Integer(register) => panic!()
        }
    }
    pub fn print_i32(&self) -> String {
        match self {
            Self::Integer(register) => llvm_call_print_local(register.clone(), "i32"),
            _ => panic!("Not a i32")
        }
    }
    pub fn print_i1(&self) -> String {
        match self {
            Self::Integer(register) => {
                let c2 = llvm_call_print_local(register.clone(), "i1");

                c2
            }
            _ => panic!("Not a i32")
        }
    }
}
pub fn print_statement(parser: &mut Parser) {
    expression(parser);
    let expr = parser.expr_pop();
    let value = expr;
    match &value.data_type {
        DataType::Boolean(_) => {
            let codegen = LlvmCallPrint::Integer(parser.expr_count).print_i1();
            parser.emit_instruction(&codegen);
        }
        DataType::Integer(_) => {
            // pass tmp register value and the Expr item itself.
            let codegen = LlvmCallPrint::Integer(parser.expr_top()).print_i32() ;
            parser.emit_instruction(&codegen);
        },
        DataType::String(_) => {
                let print_inst = LlvmCallPrint::String(parser.expr_top()).call_print();
                parser.emit_instruction(&print_inst);
                parser.expr_count += 1;
        }
    }
    
    parser.expr_count += 1;
    parser.consume(TokenType::Semicolon, "Expect semicolon after value");
}

