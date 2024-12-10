

use crate::scanner::token::TokenType;
use crate::parser::{
    declaration::function::parse_fn_declare, declaration::variable::parse_get_variable, parser::Parser,
    primitives::{literal::parse_literal, number::parse_number, string::parse_string},
    expression::{binary::parse_binary, precedence::Precedence, expression::parse_grouping, expr::{Expression, ParseError}}
};

pub struct ParseRule<'a>{
    pub prefix: Option<ParseFn<'a>>,
    pub infix: Option<ParseFn<'a>>,
    pub precedence: Precedence,
}
type ParseFn<'a> = fn(&'a mut Parser) -> Result<Expression, ParseError>;

pub fn get_rule<'a>(token_type: TokenType) -> ParseRule<'a> {
    // println!("GetRule: {}", token_type);
    match token_type {
        // Parentheses 

        TokenType::LeftParen => ParseRule {
            prefix: Some(parse_grouping),
            infix: None,
            precedence: Precedence::PrecNone
        },
        TokenType::RightParen => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        },

        TokenType::True => ParseRule { 
            prefix: Some(parse_literal), 
            infix: None,
            precedence: Precedence::PrecNone
        },
        TokenType::False => ParseRule { 
            prefix: Some(parse_literal), 
            infix: None,
            precedence: Precedence::PrecNone
        },
        TokenType::EqualEqual => ParseRule {
            prefix: None,
            infix: Some(parse_binary),
            precedence: Precedence::PrecEquality
        },
        TokenType::BangEqual => ParseRule {
            prefix: None,
            infix: Some(parse_binary),
            precedence: Precedence::PrecEquality
        },
        TokenType::Greater => ParseRule {
            prefix: None,
            infix: Some(parse_binary),
            precedence: Precedence::PrecComparison
        },
        TokenType::Less => ParseRule {
            prefix: None,
            infix: Some(parse_binary),
            precedence: Precedence::PrecComparison
        },
        TokenType::GreaterEqual => ParseRule {
            prefix: None,
            infix: Some(parse_binary),
            precedence: Precedence::PrecComparison
        },
        TokenType::LessEqual => ParseRule {
            prefix: None,
            infix: Some(parse_binary),
            precedence: Precedence::PrecComparison
        },
        TokenType::Plus => ParseRule { 
            prefix: None, 
            infix: Some(parse_binary),
            precedence: Precedence::PrecTerm 
        },
        TokenType::Minus => ParseRule { 
            prefix: None, 
            infix: Some(parse_binary),
            precedence: Precedence::PrecTerm 
        },
        TokenType::Star => ParseRule { 
            prefix: None, 
            infix: Some(parse_binary),
            precedence: Precedence::PrecFactor
        },
        TokenType::Slash => ParseRule { 
            prefix: None, 
            infix: Some(parse_binary),
            precedence: Precedence::PrecFactor
        },
        TokenType::Number => ParseRule { 
            prefix: Some(parse_number), 
            infix: None, 
            precedence: Precedence::PrecNone 
        },
        TokenType::String => ParseRule { 
            prefix: Some(parse_string), 
            infix: None, 
            precedence: Precedence::PrecNone 
        },
        TokenType::Identifier => ParseRule {
            prefix: Some(parse_get_variable),
            infix: None,
            precedence: Precedence::PrecNone
        },
        TokenType::EOF => ParseRule {
            prefix: None, 
            infix: None, 
            precedence: Precedence::PrecNone 
        },
        _ => ParseRule { 
            prefix: None, 
            infix: None, 
            precedence: Precedence::PrecNone 
        } 
    }
}