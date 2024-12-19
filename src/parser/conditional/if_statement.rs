use std::fmt::format;

use crate::parser::{
    parser::Parser,
    expression::expr::DataType,
    expression::expression::expression,
    parse_fn::declaration,
    parse_fn::statement
};
use crate::scanner::token::TokenType;

pub enum LlvmConditional {
    If(u32) // represents the depth.
}
impl LlvmConditional {
    pub fn create_branch(&self, bool_reg: u32) -> String {
        match self {
            Self::If(depth) => format!("\tbr i1 %{}, label %then{}, label %else{}", bool_reg - 2, depth, depth)
        }
    }
    pub fn then_branch(&self) -> String {
        match self {
            Self::If(depth) => format!("\nthen{}:", depth)
        }
    }
    pub fn else_branch(&self) -> String {
        match self {
            Self::If(depth) => format!("\nelse{}:", depth)
        }
    }
    pub fn end_branch(&self) -> String {
        match self {
            Self::If(depth) => format!("\nend{}:", depth)
        }
    }
    pub fn to_end(&self) -> String {
        match self {
            Self::If(depth) => format!("\tbr label %end{}", depth)
        }
    }

    pub fn while_cond(&self, bool_reg: u32) -> String {
        match self {
            Self::If(depth) => format!("\tbr i1 %{}, label %body{}, label %exit{}", bool_reg, depth, depth)
        }
    }
    pub fn while_start(&self) -> String {
        match self {
            Self::If(depth) => format!("\ncond{}:", depth)
        }
    }
    pub fn while_body(&self) -> String {
        match self {
            Self::If(depth) => format!("\nbody{}:", depth)
        }
    }
    pub fn while_exit(&self) -> String {
        match self {
            Self::If(depth) => format!("\nexit{}:", depth)
        }
    }
    pub fn while_check_cond(&self, bool_reg: u32) -> String {
        match self {
            Self::If(depth) => format!("\tbr label %cond{}", depth)
        }
    }
    
}
pub fn if_statement(parser: &mut Parser) {
    // if keyworld already consumed
    // parse expression
    parser.expr_count += 3;
    let depth = parser.depth;
    expression(parser);
    let expr = parser.expr_pop();
    let branch = LlvmConditional::If(depth);
    parser.comment(&format!("depth: {}", depth).to_string());
    parser.depth += 1;
    parser.emit_instruction(&branch.create_branch(expr.1+1));
    parser.consume(TokenType::LeftBrace, "message");
    parser.emit_instruction(&branch.then_branch());
    // parse block
    while !parser.match_current(TokenType::RightBrace) {
        declaration(parser);
    }
    parser.emit_instruction(&&branch.to_end());
    parser.emit_instruction(&branch.else_branch());
    
    if parser.match_current(TokenType::Else) {
        parser.consume(TokenType::LeftBrace, "message");
        while !parser.match_current(TokenType::RightBrace) {
            declaration(parser);
        }
    }
    parser.emit_instruction(&&branch.to_end());
    parser.emit_instruction(&branch.end_branch());
    parser.depth -= 1;

}