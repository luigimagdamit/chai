mod token;      // Importing the token module
mod parser;     // Importing the parser module
mod error;      // Importing the error module

use core::panic;

use parser::{match_literal, match_number, ParserResult};
use token::{TokenType, KeywordType, Op};
use error::ErrorCode;

const ADD_OP: TokenType = TokenType::Operator(Op::Add);

fn pair<'a, P1, P2>(parser1: P1, parser2: P2) -> impl Fn(&'a str) -> (ParserResult, ParserResult) 
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
type ResultPair<'a> = (ParserResult<'a>, ParserResult<'a>);
fn print_pair_lexeme(result: &ResultPair) {
    println!("{} {}", &result.0.token.unwrap().lexeme, &result.1.token.unwrap().lexeme);
}

fn parse_binary(input: &str) {
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
                            println!("{}", l + r);
                        },
                        _ => { println!("{} : Expected alphanumeric value after mathematical operator: {}{}...", ErrorCode::SyntaxError, l, sign.lexeme) }
                    }
                },
                _ => {}
            }
        },
    _ => {println!("{}", l);}
   }
}
fn main() {

    parse_binary("69+e420))))");

}
