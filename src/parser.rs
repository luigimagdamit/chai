use crate::token::{Token, TokenType};
use std::fmt::{self, Formatter};

pub struct ParserResult<'a> {
    pub token: Option<Token<'a>>,
    pub remainder: &'a str,
}

impl<'a> fmt::Display for ParserResult<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.token {
            Some(t) => write!(f, "{}", t),
            None => write!(f, "No Token Value"),
        }
    }
}

pub fn match_literal<'a>(expected: &'static str, t: TokenType) 
    -> impl Fn(&'a str) -> ParserResult<'a> {
    move |input| {
        match input.get(0..expected.len()) {
            Some(next) if next == expected => ParserResult {
                token: Some(Token {
                    lexeme: &input[0..expected.len()],
                    literal_type: t,
                }),
                remainder: &input[expected.len()..],
            },
            _ => ParserResult {
                token: None,
                remainder: input
            },
        }
    }
}

pub fn match_number<'a>(input: &'a str) -> ParserResult<'a> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(text) if text.is_numeric() => matched.push(text),
        _ => return ParserResult {
            token: None,
            remainder: input,
        },
    }

    while let Some(text) = chars.next() {
        if text.is_numeric() {
            matched.push(text);
        } else  {
            break;
        }
    }

    let next_index = matched.len();
    ParserResult {
        token: Some(Token {
            lexeme: &input[0..matched.len()],
            literal_type: TokenType::Number,
        }),
        remainder: &input[next_index..],
    }
}

