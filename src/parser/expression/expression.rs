use crate::parser::parser::Parser;
use crate::parser::expression::parse_rule::get_rule;
use crate::parser::expression::precedence::Precedence;
use crate::llvm::expr_mode::expr_mode;
use crate::scanner::token::TokenType;
use super::expr::{Expression, ParseError};

pub fn expression(parser: &mut Parser) {
    parse_precedence(parser, Precedence::PrecAssignment);
    expr_mode(parser);
}

pub fn parse_precedence(parser: &mut Parser, precedence: Precedence) {
    parser.advance();
    if let Some(prefix_fn) = get_rule(parser.previous.unwrap().token_type).prefix {
        let _ = prefix_fn(parser);
    } else {
        let err_msg = format!("Expected a prefix rule or valid expression but found <{}>", parser.previous.unwrap());
        parser.error_at(&parser.previous.unwrap(), &err_msg);
    }
    while precedence.to_u32() <= get_rule(parser.current.unwrap_or_else(||panic!("Current token not present for parse_precedence()")).token_type).precedence.to_u32() {
        parser.advance();
        if let Some(infix_rule) = get_rule(parser.previous.unwrap().token_type).infix {
            let _ = infix_rule(parser);
        } else {
            break
        }
    }
}

pub fn parse_grouping(parser: &mut Parser) -> Result<Expression, ParseError> {
    expression(parser);
    parser.consume(TokenType::RightParen, "Expect ')' after expression, found something else at");
    Ok(Expression::Empty)
}