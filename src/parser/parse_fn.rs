use super::{
    parser::Parser,
    parse_rule::get_rule,
    precedence::Precedence,
    expr::{
        Expr,
        DataType
    },
    print::print_statement
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
pub fn parse_fn_declare(parser: &mut Parser) {
    print!("\ndefine ");
    parser.consume(TokenType::Identifier, "Expected function name");
    let fn_name = parser.previous.unwrap().clone();
    parser.consume(TokenType::LeftParen, "");
    parser.consume(TokenType::RightParen, "");
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
    if let Some(prev) = parser.previous {
        if let Some(prefix_fn) = get_rule(prev.token_type).prefix {
            prefix_fn(parser);
        } else {
            let err_msg = format!("Expected a prefix rule for token <{}>", prev);
            parser.error_at(&prev, &err_msg);
        }

        if let Some(_) = parser.current {
            while precedence.to_u32() <= get_rule(parser.current.unwrap().token_type).precedence.to_u32() {
                parser.advance();
                if let Some(infix_rule) = get_rule(parser.previous.unwrap().token_type).infix {
                    infix_rule(parser);
                } else {
                    break
                }
                parser.print_parser();
            }
        }
    }
    
}



pub fn parse_grouping(parser: &mut Parser) {
    expression(parser);
    parser.consume(TokenType::RightParen, "Expect ')' after expression, found something else at");
}

