
use crate::parser::{
    parser::Parser,
    conditional::if_statement::if_statement,
    expression::expression::expression,
    declaration::print::print_statement,
    declaration::function::parse_fn_declare,
    declaration::variable::{variable_declaration, parse_set_variable},
    conditional::while_statement::while_statement
};
use crate::scanner::token::TokenType;

use super::declaration::variable::parse_get_variable;


pub fn expression_statement(parser: &mut Parser) {
    expression(parser);
    parser.consume(TokenType::Semicolon, "Expect ; after expression");
}

pub fn convert_type_tag(tag: &str) -> String {
    match tag {
        "int" => String::from("alloca i32"),
        "bool" => String::from("alloca i1"),
        "str" => String::from("alloca i8*"),
        _ => String::from("")
    }
}
// getters need to create a new expression since it is one

fn advance_then_declaration(parser: &mut Parser, declaration: fn(&mut Parser)) {
    parser.advance();
    declaration(parser);
}

pub fn declaration(parser: &mut Parser) {
    if let Some(curr) = parser.current {
        match curr.token_type {
            TokenType::Fun => advance_then_declaration(parser, parse_fn_declare),
            TokenType::Var => advance_then_declaration(parser, variable_declaration),
            TokenType::Identifier => advance_then_declaration(parser, parse_set_variable),
            _ => statement(parser),
        }
    }

}

pub fn statement(parser: &mut Parser) {

    if parser.match_current(TokenType::Print) {
        print_statement(parser);
    } else if parser.match_current(TokenType::If) {
        if_statement(parser);
    } else if parser.match_current(TokenType::While) {
        while_statement(parser);
    } else if parser.match_current(TokenType::Identifier) {
        parse_get_variable(parser);
    }
    else {
        expression_statement(parser);
    }
}
#[allow(unused)]
pub fn parse_block(parser: &mut Parser) {

}
// fn name() ret type





