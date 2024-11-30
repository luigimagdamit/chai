use crate::scanner::token::TokenType;
use super::{
    parser::Parser,
    parse_fn::parse_number,
    literal::parse_literal,
    binary::parse_binary,
    parse_fn::{parse_fn_declare, parse_grouping},
    precedence::Precedence
};
pub struct ParseRule<'a>{
    pub prefix: Option<ParseFn<'a>>,
    pub infix: Option<ParseFn<'a>>,
    pub precedence: Precedence,
}
type ParseFn<'a> = fn(&'a mut Parser);

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
        TokenType::Fun => ParseRule {
            prefix: Some(parse_fn_declare),
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