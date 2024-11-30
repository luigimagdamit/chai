use super::{
    parser::Parser,
    parse_rule::get_rule,
    precedence::Precedence,
    expr::{
        Expr,
        DataType
    },
    print::print_statement,
    function::parse_fn_declare
};
use crate::{
    llvm::expr_mode::expr_mode,
    scanner::token::TokenType
};

pub fn expression(parser: &mut Parser) {
    parse_precedence(parser, Precedence::PrecAssignment);
    // assume this is just a high level expression
    expr_mode(parser);
}
pub fn expression_statement(parser: &mut Parser) {
    expression(parser);
    parser.consume(TokenType::Semicolon, "Expect ; after expression");
}

pub fn declaration(parser: &mut Parser) {
    if parser.match_current(TokenType::Fun) {
        parse_fn_declare(parser);
    } else {
        statement(parser);
    }
    
}
pub fn statement(parser: &mut Parser) {
    if parser.match_current(TokenType::Print) {

        print_statement(parser);
    } else {
        expression_statement(parser);
    }
}
#[allow(unused)]
pub fn parse_block(parser: &mut Parser) {

}
// fn name() ret type

pub fn parse_number(parser: &mut Parser) {
    let value = parser.previous.unwrap().start;
    let number_leaf = Expr {
        left: String::from(format!("i32 {}", value)),
        right: String::from(value),
        data_type: DataType::Integer(value.parse().unwrap())
    };

    parser.constant_stack.push(Some(number_leaf));
}

pub fn parse_precedence(parser: &mut Parser, precedence: Precedence) {
    parser.advance();

    if let Some(prefix_fn) = get_rule(parser.previous.unwrap().token_type).prefix {
        prefix_fn(parser);
    } else {
        let err_msg = format!("Expected a prefix rule for token <{}>", parser.previous.unwrap());
        parser.error_at(&parser.previous.unwrap(), &err_msg);
    }

    while precedence.to_u32() <= get_rule(parser.current.unwrap_or_else(||panic!("Current token not present for parse_precedence()")).token_type).precedence.to_u32() {
        parser.advance();
        if let Some(infix_rule) = get_rule(parser.previous.unwrap().token_type).infix {
            infix_rule(parser);
        } else {
            break
        }
    }
    
    
}



pub fn parse_grouping(parser: &mut Parser) {
    expression(parser);
    parser.consume(TokenType::RightParen, "Expect ')' after expression, found something else at");
}

