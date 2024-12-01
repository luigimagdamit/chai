use std::fmt::format;

use super::{
    parser::Parser,
    parse_rule::get_rule,
    precedence::Precedence,
    expr::{
        Expr,
        DataType
    },
    print::print_statement,
    function::parse_fn_declare,
    symbol::{create_new_symbol, get_symbol, set_symbol}
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
        "str" => String::from("alloca i8*"),
        _ => String::from("")
    }
}
// getters need to create a new expression since it is one
pub fn parse_get_variable(parser: &mut Parser) {
    let value = parser.previous.unwrap();
    let symbol = get_symbol(parser, String::from(value.start));
}
pub fn variable_assignment(parser: &mut Parser, var_name: &str) {

    expression(parser);
    if let Some(expr) = parser.constant_stack.pop() {
        let value = expr.unwrap_or_else(||panic!("Tried evaluation an expression in print_statement, but opened an empty Expr"));
        let print_val = &value.left;
        
        match &value.data_type {
            DataType::Boolean(_) => (),
            DataType::Integer(int) => {
                println!("store i32 {}, i32* %{}", int , var_name);
                create_new_symbol(parser, String::from(var_name), value.data_type);
            },
            DataType::String(_) => {
                println!("%{} = {}", parser.expr_count, print_val);
                println!("store i8* %{}, i8** %{}", parser.expr_count , var_name);
                create_new_symbol(parser, String::from(var_name), value.data_type);
            }
        }
    }
    
    parser.expr_count += 1;
    
}
pub fn variable_declaration(parser: &mut Parser) {
    // let name: type;
    let global_name = parse_variable(parser, "Expected a variable name");
    parser.consume(TokenType::Colon, "Expected : when declaring variable");
    parser.consume(TokenType::Identifier, "Expected a type identifier when declaring variable");
    let type_tag = convert_type_tag(parser.previous.clone().unwrap().start);
    println!("%{} = {}", global_name, type_tag);
    if parser.match_current(TokenType::Equal) {
        
        variable_assignment(parser, &global_name);
        
    } else {
        
    }


    parser.consume(TokenType::Semicolon, "Expected a semicolon after variable declaration");
}
fn advance_then_declaration(parser: &mut Parser, declaration: fn(&mut Parser)) {
    parser.advance();
    declaration(parser);
}
pub fn parse_set_variable(parser: &mut Parser) {
    let identifier = parser.previous.unwrap();
    parser.consume(TokenType::Equal, "Expected assignment");
    expression(parser);
    parser.consume(TokenType::Semicolon, "");
    let expr = parser.constant_stack.pop().unwrap().unwrap();

    set_symbol(parser, String::from(identifier.start), expr);
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

