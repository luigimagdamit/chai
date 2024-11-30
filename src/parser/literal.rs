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
        TokenType::False => {
            let false_leaf = Expr {
                left: String::from(format!("i1 {}", value)),
                right: String::from(value),
                data_type: DataType::Boolean(false)
            };
            parser.constant_stack.push(Some(false_leaf));
        }
        TokenType::True => {
            let false_leaf = Expr {
                left: String::from(format!("i1 {}", value)),
                right: String::from(value),
                data_type: DataType::Boolean(true)
            };
            parser.constant_stack.push(Some(false_leaf));
        }
        _ => parser.error_at(&parser.previous.unwrap(), "Invalid literal token"),
    }
}
pub fn parse_literal(parser: &mut Parser) {
    if let Some(prev) = parser.previous {
        match prev.token_type {
            TokenType::False => create_literal(parser, TokenType::False, "0"),
            TokenType::True => create_literal(parser, TokenType::True, "1"),
            _ => parser.error_at(&prev, "Unrecognizeed literal token type"),
        }
    }
}