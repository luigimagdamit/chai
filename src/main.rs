mod token;      // Importing the token module
mod parser;     // Importing the parser module
mod error;      // Importing the error module

use parser::{match_literal, match_number, ParserResult};
use token::{TokenType, KeywordType, Op};
use error::ErrorCode;

const ADD_OP: TokenType = TokenType::Operator(Op::Add);

fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Fn(&'a str) -> (ParserResult<'a>, ParserResult<'a>) 
where 
    P1: Fn(&'a str) -> ParserResult<'a>,
    P2: Fn(&'a str) -> ParserResult<'a>
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
fn main() {
    let source = "12+20000";
    let parse_add = match_literal("+", ADD_OP);
    // let parse_add_res = parse_add(&source);
    let parse_num_res = match_number(&source);
    
    let pr = &parse_num_res.token;
    match &pr {
        Some(token) => {
            println!("{}", &token);
            let plus = parse_add(parse_num_res.remainder);
            match &plus.token {
                Some(t) => {
                    println!("{}", &t);
                    let rh = match_number(plus.remainder);
                    match rh.token {
                        Some(rht) => {
                            println!("{}", rht);
                        }
                        _ => {}
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }

    let t = pair::<_, _, ParserResult, ParserResult>(parse_add, match_number);
    let u_res = t("4000");
    let t_res = t("+23");
   
    println!("{} {} {}", u_res.0, t_res.0, t_res.1);

}
