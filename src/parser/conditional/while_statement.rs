use std::fmt::format;

use crate::parser::{
    parser::Parser,
    expression::expr::DataType,
    expression::expression::expression,
    parse_fn::declaration,
    parse_fn::statement
};
use crate::scanner::token::TokenType;
use crate::parser::conditional::if_statement::LlvmConditional;

pub fn while_statement(parser: &mut Parser) {
    // if keyworld already consumed
    // parse expression
    
    let depth = parser.expr_count;
    let branch = LlvmConditional::If(depth);
    
    parser.emit_instruction(&branch.while_check_cond(0));
    parser.emit_instruction(&branch.while_start());
    expression(parser);
    
    
    parser.comment(&format!("depth: {}", depth).to_string());
    parser.depth += 1;
    
    let expr = parser.expr_pop();
    parser.emit_instruction(&branch.while_cond(expr.1));
    parser.consume(TokenType::LeftBrace, "message");
    parser.emit_instruction(&branch.while_body());
    // // parse block
    while !parser.match_current(TokenType::RightBrace) {
        declaration(parser);
    }
    parser.emit_instruction(&branch.while_check_cond(expr.1));
    parser.emit_instruction(&branch.while_exit());


}