use crate::parser::parser::Parser;
use crate::parser::symbol::{create_new_symbol, get_symbol, set_symbol};
use crate::parser::parse_fn::{expression, convert_type_tag};
use crate::parser::expression::expr::DataType;
use crate::{common::common::PARSE_DECLARATION_MODE, scanner::token::TokenType};

// misleading title, will just 
pub fn parse_variable_name(parser: &mut Parser, err_msg: &str) -> String {
    parser.consume(TokenType::Identifier, err_msg);
    return String::from(parser.previous.unwrap().start)
    // store this in the hash table
}

pub fn parse_get_variable(parser: &mut Parser) {
    let value = parser.previous.unwrap();
    let _symbol = get_symbol(parser, String::from(value.start));
}

// evaluate an expression, then assign the expression at the location of the local variable with store
pub fn variable_assignment(parser: &mut Parser, var_name: &str) {
    expression(parser);
    if let Some(expr) = parser.constant_stack.pop() {
        let value = expr.unwrap_or_else(||panic!("Tried evaluation an expression in print_statement, but opened an empty Expr"));
        let print_val = &value.left;
        
        match &value.data_type {
            DataType::Boolean(_) => (),
            DataType::Integer(int) => {
                let codegen = format!("store i32 {}, i32* %{}", int , var_name);
                println!("{}", codegen);
                parser.emitInstruction(&codegen);
                create_new_symbol(parser, String::from(var_name), value.data_type);
            },
            DataType::String(_) => {
                let codegen1 = format!("%{} = {}", parser.expr_count, print_val);
                println!("{}", codegen1);
                parser.emitInstruction(&codegen1);
                let codegen2 = format!("store i8* %{}, i8** %{}", parser.expr_count , var_name);
                println!("{}", codegen2);
                parser.emitInstruction(&codegen2);
                create_new_symbol(parser, String::from(var_name), value.data_type);
            }
        }
    }
    

    
}
pub fn variable_declaration(parser: &mut Parser) {
    // let name: type;
    let global_name = parse_variable_name(parser, "Expected a variable name");
    parser.consume(TokenType::Colon, "Expected : when declaring variable");
    parser.consume(TokenType::Identifier, "Expected a type identifier when declaring variable");
    let type_tag = convert_type_tag(parser.previous.clone().unwrap().start);
    let codegen = format!("%{} = {}\n", global_name, type_tag);
    if PARSE_DECLARATION_MODE { println!("{}", codegen) }
    parser.compilation += &codegen;
    if parser.match_current(TokenType::Equal) {
        
        variable_assignment(parser, &global_name);
        
    } else {
        
    }


    parser.consume(TokenType::Semicolon, "Expected a semicolon after variable declaration");
}

pub fn parse_set_variable(parser: &mut Parser) {
    let identifier = parser.previous.unwrap();
    parser.consume(TokenType::Equal, "Expected assignment");
    expression(parser);
    parser.consume(TokenType::Semicolon, "");
    if let Some(expr) = parser.constant_stack.pop() {
        match expr {
            Some(new_value) => set_symbol(parser, String::from(identifier.start), new_value),
            None => parser.error_at_previous("Expected an <expression> when setting variable to a new value"),
        }
    } else {

        parser.error_at(&identifier, "Unknown variable (set_variable)");
    }



}