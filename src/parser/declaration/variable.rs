use crate::parser::parser::Parser;
use crate::parser::core::symbol::{create_new_symbol, get_symbol, set_symbol};
use crate::parser::expression::expression::expression;
use crate::parser::expression::expr::{Accept, DataType, Expression, ParseError};
use crate::scanner::token::TokenType;

use super::declaration::Declaration;
use super::print::{PrintVisitor, RebuildVisitor};

// evaluate an expression, then assign the expression at the location of the local variable with store
pub fn variable_assignment(parser: &mut Parser, var_name: &str) {
    let mut visitor = PrintVisitor;
    let mut rebuild = RebuildVisitor;
    expression(parser);


    if let Some(expr_ast) = parser.ast_stack.pop() {
        let test = Declaration::new_variable(var_name, Some(expr_ast.clone().to_expression()), expr_ast.to_expression().as_datatype());
        parser.comment(&test.accept(&mut rebuild));
        parser.emit_instruction(&test.accept(&mut visitor));
        create_new_symbol(parser, var_name, test.as_variable().as_datatype());
        parser.print_symbols();
    }
}
pub fn variable_declaration(parser: &mut Parser) {
    let global_name = parse_variable_name(parser, "Expected a variable name");
    parser.consume(TokenType::Colon, "Expected : when declaring variable");
    parser.consume(TokenType::Identifier, "Expected a type identifier when declaring variable");
    let type_tag = parser.previous.expect("Expected a token when getting the type identifier");
    let type_tag = match type_tag.start {
        "int" => DataType::Integer(0),
        "bool" => DataType::Boolean(true),
        "str" => DataType::String("".to_string()),
        _ => panic!()
    };

    if parser.match_current(TokenType::Equal) { variable_assignment(parser, &global_name) } 
    else {
        let mut visitor = PrintVisitor;
        let test = Declaration::new_variable(&global_name, None, type_tag.clone());
        parser.emit_instruction(&test.accept(&mut visitor));
        create_new_symbol(parser, &global_name, type_tag);
    }
    parser.expr_count += 1;
    parser.consume(TokenType::Semicolon, "Expected a semicolon after variable declaration");
}

pub fn parse_set_variable(parser: &mut Parser) {
    let identifier = parser.previous.expect("Expected a token when parsing the identifier when setting the variable");
    if parser.match_current(TokenType::Equal) {
        expression(parser);
        let expr = parser.ast_stack
            .pop()
            .expect("Expected an AstNode on the ast_stack")
            .to_expression()
            .clone();
        set_symbol(parser, identifier.start, expr);
        parser.consume(TokenType::Semicolon, "");
    } else {
        let _ = parse_get_variable(parser);
    }
}

#[allow(unused)]


pub fn parse_variable_name(parser: &mut Parser, err_msg: &str) -> String {
    parser.consume(TokenType::Identifier, err_msg);
    String::from(parser.previous.expect("Expected a variable name token when getting variable name").start)
}

pub fn parse_get_variable(parser: &mut Parser) -> Result<Expression, ParseError>{
    let value = parser.previous.expect("Tried to get previous token, but it was empty");
    get_symbol(parser, value.start);
    Ok(Expression::Empty)
}