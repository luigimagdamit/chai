use crate::parser::Parser;
use crate::parse_rule::get_rule;
use crate::precedence::Precedence;
use crate::expr::{Expr, DataType};
use crate::token::TokenType;
use crate::common::PARSE_FN_OUTPUT;

const BINARY_ADD_ERROR: &str = "Expected an integer in mathematical binary operation";

pub fn parse_number(parser: &mut Parser) {
    let value = parser.previous.unwrap().start;
    let number_leaf = Expr {
        left: String::from(format!("i32 {}", value)),
        right: String::from(value),
        data_type: DataType::Integer(value.parse().unwrap())
    };
    // number_leaf.print_leaf();
    match parser.left_hand {
        None => parser.left_hand = Some(number_leaf),
        Some(_) => parser.right_hand = Some(number_leaf)
    }
    
    // println!("<number: {}>", value)
}
pub fn expression(parser: &mut Parser) {
    parse_precedence(parser, Precedence::PrecAssignment);

    // assume this is just a high level expression

    parser.print_parser();
}
pub fn parse_precedence(parser: &mut Parser, precedence: Precedence) {
    parser.advance();
    if let Some(prev) = parser.previous {
        let prefix_rule = get_rule(prev.token_type).prefix;
        match prefix_rule {
            None => { 
                let err_msg = format!("Expected a prefix rule for token <{}>", prev);
                parser.error_at(&prev, &err_msg);
            },
            Some(prefix_fn) => {
                prefix_fn(parser);
            }
        }
        if let Some(curr) = parser.current {
            while precedence.to_u32() <= get_rule(curr.token_type).precedence.to_u32() {
                parser.advance();
                if let Some(infix_rule) = get_rule(parser.previous.unwrap().token_type).infix {
                    infix_rule(parser);
                } else {
                    break
                }
            }
        }
    }
    
}
pub fn parse_binary(parser: &mut Parser) {
    if let Some(token) = parser.previous {
       
        let operator_type = token.token_type;
        let rule_fn = get_rule(operator_type);
        let new_prec = rule_fn.precedence as u32;
        parse_precedence(parser, Precedence::from_u32(new_prec + 1));

        let local_left = &mut parser.left_hand;
        let local_right = &mut parser.right_hand;

        
        match operator_type {
            TokenType::Plus => {
                let left = local_left.clone().unwrap();
                let right = local_right.clone().unwrap();

                if PARSE_FN_OUTPUT {
                    println!("\n==");
                    parser.print_parser();
                    println!("==");
                }
                
                match (left.data_type, right.data_type) {
                    (DataType::Integer(a), DataType::Integer(b)) => {
                        // println!("<add: <constant fold: {}+{}={}>>", a, b, a + b);
                        parser.left_hand = Some(Expr {
                            left: String::from("i32 ") + &(a + b).to_string(),
                            right: (a + b).to_string(),
                            data_type: DataType::Integer(a+b) 
                        });
                        parser.right_hand = None;
                    }
                    // _ => println!("<left operand: {}> <plus> <right operand: {}>", left.left, right.right)
                }
                if PARSE_FN_OUTPUT {
                    parser.print_parser();
                }
                

            },
            _ => {}
        }


    }
    
}

// pub fn parse_grouping(parser: &mut Parser) {
//     expression(parser);
//     parser.consume(TokenType::RightParen, "Expect ')' after expression");
// }