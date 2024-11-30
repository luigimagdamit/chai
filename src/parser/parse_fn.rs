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
pub fn parse_variable(parser: &mut Parser, err_msg: &str) -> String {
    parser.consume(TokenType::Identifier, err_msg);
    return String::from(parser.previous.unwrap().start)
    // store this in the hash table
}
pub fn convert_type_tag(tag: &str) -> String {
    match tag {
        "int" => String::from("alloca i32"),
        "bool" => String::from("alloca i1"),
        //"str" => String::from(format!("load i8*, i8** @{}, align 8", )),
        _ => String::from("")
    }
}
pub fn variable_declaration(parser: &mut Parser) {
    // let name: type;
    let global_name = parse_variable(parser, "Expected a variable name");
    parser.consume(TokenType::Colon, "Expected : when declaring variable");
    parser.consume(TokenType::Identifier, "Expected a type identifier when declaring variable");
    let type_tag = convert_type_tag(parser.previous.clone().unwrap().start);
    println!("%{} = {}", global_name, type_tag);
    if parser.match_current(TokenType::Equal) {
        expression(parser);
    } else {
        
    }


    parser.consume(TokenType::Semicolon, "Expected a semicolon after variable declaration");
}
fn advance_then_declaration(parser: &mut Parser, declaration: fn(&mut Parser)) {
    parser.advance();
    declaration(parser);
}
pub fn declaration(parser: &mut Parser) {
    if let Some(curr) = parser.current {
        match curr.token_type {
            TokenType::Fun => advance_then_declaration(parser, parse_fn_declare),
            TokenType::Var => advance_then_declaration(parser, variable_declaration),
            _ => statement(parser),
        }
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



pub fn parse_precedence(parser: &mut Parser, precedence: Precedence) {
    parser.advance();
    if let Some(prefix_fn) = get_rule(parser.previous.unwrap().token_type).prefix {
        prefix_fn(parser);
    } else {
        let err_msg = format!("Expected a prefix rule or valid expression but found <{}>", parser.previous.unwrap());
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

