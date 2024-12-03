
use crate::parser::{
    parser::Parser,
    expression::expr::{DataType, Expr},
    expression::parse_rule::get_rule,
    expression::precedence::Precedence,
    declaration::print::print_statement,
    declaration::function::parse_fn_declare,
    declaration::variable::{variable_declaration, parse_set_variable}
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
pub fn if_statement(parser: &mut Parser) {
    // if keyworld already consumed
    // parse expression
    expression(parser);
    if let Some(expr) = parser.constant_stack.pop() {
        let value = expr.unwrap_or_else(||panic!("Tried evaluation an expression in print_statement, but opened an empty Expr"));
        match &value.data_type {
            DataType::Boolean(_) => {
                let c1 = format!("\t%{} = add {}, 0", parser.expr_count , &value.left);
                println!("{c1}");
                parser.emit_instruction(&c1);
                let branch = format!("\tbr i1 %{}, label %{}, label %{}", parser.expr_count, "then0", "else0");
                parser.expr_count += 1;
                println!("{branch}");
                parser.consume(TokenType::LeftBrace, "message");
                println!("\nthen0:");
                while !parser.match_current(TokenType::RightBrace) {
                    declaration(parser);
                }
                
                println!("\tbr label %end");

                println!("\nelse0:");
                if parser.match_current(TokenType::Else) {
                    parser.consume(TokenType::LeftBrace, "message");
                    
                    while !parser.match_current(TokenType::RightBrace) {
                        statement(parser);
                    }
                    println!("\tbr label %end");
                    println!("\nend:")
                    
                } else {
                    println!("\tbr label %end");
                    println!("\nend:")
                    
                }
                

            }
            _ => ()
        }
    }
}
pub fn statement(parser: &mut Parser) {

    if parser.match_current(TokenType::Print) {
        print_statement(parser);
    } else if parser.match_current(TokenType::If) {
        if_statement(parser);
    } 
    else {
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

