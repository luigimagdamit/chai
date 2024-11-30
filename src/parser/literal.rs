use super::{
    parser::Parser,
    expr::{
        DataType,
        Expr
    }
    
};
use crate::scanner::token::TokenType;
fn create_literal(parser: &mut Parser, token_type: TokenType, value: &str) {
    match token_type {
        TokenType::False => parser.new_expr(new_bool_val(value, false)),
        TokenType::True => parser.new_expr(new_bool_val(value, true)),
        _ => ()
    }
}
pub fn parse_literal(parser: &mut Parser) {
    if let Some(prev) = parser.previous {
        match prev.token_type {
            TokenType::False => create_literal(parser, TokenType::False, "0"),
            TokenType::True => create_literal(parser, TokenType::True, "1"),
            _ => parser.error_at_previous(LITERAL_ERROR),
        }
    }
}

// Helper functions
fn new_bool_val(value: &str, bool_val: bool) -> Expr {
    Expr {
        left: String::from(format!("i1 {}", value)),
        right: String::from(value),
        data_type: DataType::Boolean(bool_val)
    }
}
const LITERAL_ERROR: &str = "Tried creating a new literal, but prev.token_type is not a True or False TokenType";