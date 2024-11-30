use super::{parse_fn::declaration, parser::Parser};
use crate::scanner::token::TokenType;

pub fn parse_fn_declare(parser: &mut Parser) {
    print!("\ndefine ");
    parser.consume(TokenType::Identifier, "Expected function name");
    let fn_name = parser.previous.unwrap().clone();
    parser.consume(TokenType::LeftParen, "Expected ( for function arguments");
    parser.consume(TokenType::RightParen, "Expected ) for function arguments");
    parser.consume(TokenType::Identifier, "");
    let fn_type = parser.previous.unwrap_or_else(|| panic!("Expected a function return type")).clone();
    parser.consume(TokenType::LeftBrace, "Expected {");

    print!("{}", fn_type.start);
    print!(" @{}", fn_name.start);
    print!("(");
    // func args here
    print!("){{");
    println!("\nentry:");
    // func body here
    //
    
    while !parser.match_current(TokenType::RightBrace) {
        declaration(parser);
    }

    println!("ret i32 0\n}}");

    
}