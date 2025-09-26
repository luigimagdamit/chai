
use crate::parser::{
    parser::Parser,
    expression::expression::expression,
    parse_fn::declaration
};
use crate::scanner::token::TokenType;
use crate::parser::conditional::if_statement::LlvmWhile;

pub fn while_statement(parser: &mut Parser) {
    // if keyworld already consumed
    // parse expression
    
    let depth = parser.expr_count;
    let branch = LlvmWhile::new(depth);
    
    parser.emit_instruction(&branch.jump_to_condition());
    parser.emit_instruction(&branch.condition_label());
    expression(parser);
    
    
    parser.comment(&format!("depth: {}", depth).to_string());
    parser.depth += 1;
    
    let expr = parser.expr_pop();

    parser.emit_instruction(&branch.condition_branch(expr.1 - 1));
    parser.consume(TokenType::LeftBrace, "Expected '{' after while condition");
    parser.emit_instruction(&branch.body_label());
    // // parse block
    while !parser.match_current(TokenType::RightBrace) {
        declaration(parser);
    }
    parser.emit_instruction(&branch.jump_to_condition());
    parser.emit_instruction(&branch.exit_label());


}