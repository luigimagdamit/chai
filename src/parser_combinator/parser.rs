use crate::token::{Token, TokenType, Op};
use crate::error::{ErrorCode};
use std::fmt::{self, Formatter};

const ADD_OP: TokenType = TokenType::Operator(Op::Add);

type ResultPair<'a> = (ParserResult<'a>, ParserResult<'a>);
fn print_pair_lexeme(result: &ResultPair) {
    println!("{} {}", &result.0.token.unwrap().lexeme, &result.1.token.unwrap().lexeme);
}
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

pub fn pair<'a, P1, P2>(parser1: P1, parser2: P2) -> impl Fn(&'a str) -> (ParserResult, ParserResult) 
where 
    P1: Fn(&'a str) -> ParserResult,
    P2: Fn(&'a str) -> ParserResult,
{
    move |input| {
        let result1 = parser1(&input);
        match &result1.token {
            Some(_) => {
                let result2 = parser2(&result1.remainder);
                match &result2.token {
                    Some(_) => { (result1, result2) }
                    _ => { (result1, ParserResult{token: None, remainder: input})}
                }
            },
            _ => { (ParserResult{token: None, remainder: input}, ParserResult{token: None, remainder: input} )}
        }
    }
}

pub fn parse_binary(input: &str) -> Result<i32, String> {
    let parse_add = &match_literal("+", ADD_OP);
    
    let parse_number = pair(match_number, parse_add); // 1 +
    let parse_expr_tail = pair(parse_add, match_number);

   let left_hand_number = parse_number(input); // this is a number


    // parsing a math expression
   let l: i32  = left_hand_number.0.token.unwrap().lexeme.parse().unwrap();
   let expr_tail = left_hand_number.0.remainder;
   match &left_hand_number.1.token {
        Some(sign) =>{
            match sign.literal_type {
                TokenType::Operator(Op::Add) => {
                    let plus_and_number = parse_expr_tail(&expr_tail);
                    let right_operand = plus_and_number.1.token;
                    match right_operand {
                        Some(operand) if operand.is_numeric()=> {
                            let r: i32  = operand.lexeme.parse().unwrap();
                            Ok(l + r)
                        },
                        _ => { Err(format!("{} : Expected alphanumeric value after mathematical operator: {}{}...", ErrorCode::SyntaxError, l, sign.lexeme)) }
                    }
                },
                _ => {Ok(l)}
            }
        },
    _ => {Ok(l)}
   }
}