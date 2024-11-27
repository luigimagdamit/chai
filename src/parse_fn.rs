use std::fmt::format;

use crate::parser::Parser;
use crate::parse_rule::get_rule;
use crate::precedence::Precedence;
use crate::expr::{Expr, DataType};
use crate::token::TokenType;
use crate::common::{PARSE_FN_OUTPUT, LLVM_DEBUG_OUTPUT};

const BINARY_ADD_ERROR: &str = "Expected an integer in mathematical binary operation";

pub fn llvm_top_level_expr(value: &str, value_type: &DataType, index: u32) {
    if LLVM_DEBUG_OUTPUT {println!("Read top-level expression:");}
    
    match value_type {
        DataType::Integer(int) => {
            // let codegen = format!("define i32 @{}() {{\nentry:\n    ret i32 {}\n}}", index, int);
            let codegen = format!("define i32 @main() {{\nentry:\n    ret i32 {}\n}}", int);
            println!("{}", codegen);
        }
    }
    
}
pub fn expression(parser: &mut Parser) {
    parse_precedence(parser, Precedence::PrecAssignment);

    // assume this is just a high level expression

    parser.print_parser();
    if let Some(constant) = parser.constant_stack.pop() {
        // println!("{}", &constant.unwrap().left);
        let expr_eval = &constant.unwrap();
        llvm_top_level_expr(&expr_eval.right, &expr_eval.data_type, 0);
    }
}

pub fn parse_number(parser: &mut Parser) {
    let value = parser.previous.unwrap().start;
    let number_leaf = Expr {
        left: String::from(format!("i32 {}", value)),
        right: String::from(value),
        data_type: DataType::Integer(value.parse().unwrap())
    };
    // number_leaf.print_leaf();
    // match parser.left_hand {
    //     None => parser.left_hand = Some(number_leaf),
    //     Some(_) => parser.right_hand = Some(number_leaf)
    // }
    parser.constant_stack.push(Some(number_leaf));
    
    // println!("<number: {}>", value)
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

        match operator_type {
            TokenType::Plus => {
                if PARSE_FN_OUTPUT { println!("<add>"); }
                parse_add(parser);
            },
            TokenType::Minus => {
                if PARSE_FN_OUTPUT { println!("<minus>"); }
                parse_subtract(parser);
            },
            TokenType::Star => {
                if PARSE_FN_OUTPUT { println!("<multiply>"); }
                parse_multiply(parser);
            }
            _ => {}
        }


    }
    
}

// pub fn parse_grouping(parser: &mut Parser) {
//     expression(parser);
//     parser.consume(TokenType::RightParen, "Expect ')' after expression");
// }

fn parse_add(parser: &mut Parser) {
    let local_right = &mut parser.constant_stack.pop().unwrap();
    let local_left = &mut parser.constant_stack.pop().unwrap();
    
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
            parser.constant_stack.push(Some(Expr {
                left: String::from("i32 ") + &(a + b).to_string(),
                right: (a + b).to_string(),
                data_type: DataType::Integer(a+b) 
            }))
            
        }
        // _ => println!("<left operand: {}> <plus> <right operand: {}>", left.left, right.right)
    }
    if PARSE_FN_OUTPUT {
        parser.print_parser();
    }
}

fn parse_subtract(parser: &mut Parser) {
    let local_right = &mut parser.constant_stack.pop().unwrap();
    let local_left = &mut parser.constant_stack.pop().unwrap();
    
    let left = local_left.clone().unwrap();
    let right = local_right.clone().unwrap();

    if PARSE_FN_OUTPUT {
        println!("\n==");
        parser.print_parser();
        println!("==");
    }
    
    match (left.data_type, right.data_type) {
        (DataType::Integer(a), DataType::Integer(b)) => {
            let calculation = a - b;
            // println!("<add: <constant fold: {}+{}={}>>", a, b, calculation);
            parser.constant_stack.push(Some(Expr {
                left: String::from("i32 ") + &(calculation).to_string(),
                right: (calculation).to_string(),
                data_type: DataType::Integer(calculation) 
            }))
            
        }
        // _ => println!("<left operand: {}> <plus> <right operand: {}>", left.left, right.right)
    }
    if PARSE_FN_OUTPUT {
        parser.print_parser();
    }
}

fn parse_multiply(parser: &mut Parser) {
    let local_right = &mut parser.constant_stack.pop().unwrap();
    let local_left = &mut parser.constant_stack.pop().unwrap();
    
    let left = local_left.clone().unwrap();
    let right = local_right.clone().unwrap();

    if PARSE_FN_OUTPUT {
        println!("\n==");
        parser.print_parser();
        println!("==");
    }
    
    match (left.data_type, right.data_type) {
        (DataType::Integer(a), DataType::Integer(b)) => {
            let calculation = a * b;
            // println!("<add: <constant fold: {}+{}={}>>", a, b, calculation);
            parser.constant_stack.push(Some(Expr {
                left: String::from("i32 ") + &(calculation).to_string(),
                right: (calculation).to_string(),
                data_type: DataType::Integer(calculation) 
            }))
            
        }
        // _ => println!("<left operand: {}> <plus> <right operand: {}>", left.left, right.right)
    }
    if PARSE_FN_OUTPUT {
        parser.print_parser();
    }
}