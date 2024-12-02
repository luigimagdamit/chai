use super::super::parser::Parser;
use crate::parser::expression::expr::{Expr, DataType};


use crate::scanner::token::TokenType;
fn create_boolean(parser: &mut Parser, token_type: TokenType) {
    match token_type {
        TokenType::False => parser.new_expr(new_bool_val(false)),
        TokenType::True => parser.new_expr(new_bool_val(true)),
        _ => ()
    }
}
pub fn parse_literal(parser: &mut Parser) {
    if let Some(prev) = parser.previous {
        match prev.token_type {
            TokenType::False => create_boolean(parser, TokenType::False),
            TokenType::True => create_boolean(parser, TokenType::True),
            _ => parser.error_at_previous(LITERAL_ERROR),
        }
    }
}

// Helper functions
fn new_bool_val(bool_val: bool) -> Expr {
    match bool_val {
        true => Expr {
            left: LlvmBooleanTag::True.left(),
            right: LlvmBooleanTag::True.right(),
            data_type: DataType::Boolean(bool_val)
        },
        false => Expr {
            left: LlvmBooleanTag::False.left(),
            right: LlvmBooleanTag::False.right(),
            data_type: DataType::Boolean(bool_val)
        }
    }
    
}
pub enum LlvmBooleanTag {
    True,
    False
}
impl LlvmBooleanTag {
    fn left(&self) -> String{
        match self {
            LlvmBooleanTag::True => String::from("i1 1"),
            LlvmBooleanTag::False => String::from("i1 0")
        }
    }
    fn right(&self) -> String {
        match self {
            LlvmBooleanTag::True => String::from("1"),
            LlvmBooleanTag::False => String::from("0")
        }
    }
}
const LITERAL_ERROR: &str = "Tried creating a new literal, but prev.token_type is not a True or False TokenType";