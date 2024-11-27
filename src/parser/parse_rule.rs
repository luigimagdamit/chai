use crate::scanner::token::TokenType;
use crate::parser::precedence::Precedence;
use crate::parser::parser::Parser;
use crate::parser::parse_fn::{parse_binary, parse_number};
pub struct ParseRule<'a>{
    pub prefix: Option<ParseFn<'a>>,
    pub infix: Option<ParseFn<'a>>,
    pub precedence: Precedence,
}
type ParseFn<'a> = fn(&'a mut Parser);

pub fn get_rule<'a>(token_type: TokenType) -> ParseRule<'a> {
    // println!("GetRule: {}", token_type);
    match token_type {
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