use crate::parser::parser::Parser;
use crate::parser::parse_rule::get_rule;
use crate::parser::precedence::Precedence;
use crate::parser::expr::{Expr, DataType};
use crate::scanner::token::TokenType;
use crate::common::common::PARSE_FN_OUTPUT;
use crate::llvm::llvm_primitives::llvm_top_level_expr;
use crate::llvm::llvm_print::{llvm_print_define, llvm_print_no_main, llvm_fmt_string_int, llvm_print_i32_define};

pub fn top_level_expr(parser: &mut Parser) {
    if let Some(constant) = parser.constant_stack.pop() {
        // println!("{}", &constant.unwrap().left);
        let expr_eval = &constant.unwrap();
        llvm_top_level_expr(&expr_eval.right, &expr_eval.data_type, 0);
    }
}
pub fn expression(parser: &mut Parser) {
    parse_precedence(parser, Precedence::PrecAssignment);

    if let Some(eof) = parser.current {
        match eof.token_type {
            TokenType::EOF => {
                parser.print_parser();
    
                println!("{}", llvm_print_define());
                llvm_fmt_string_int();
                llvm_print_i32_define();
                
                top_level_expr(parser);
                llvm_print_no_main(0);
            },
            _ => {}
        }
        
    }
    // assume this is just a high level expression


    
    
    
}

pub fn parse_number(parser: &mut Parser) {
    let value = parser.previous.unwrap().start;
    let number_leaf = Expr {
        left: String::from(format!("i32 {}", value)),
        right: String::from(value),
        data_type: DataType::Integer(value.parse().unwrap())
    };
    parser.constant_stack.push(Some(number_leaf));
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
            while precedence.to_u32() <= get_rule(parser.current.unwrap().token_type).precedence.to_u32() {
                parser.advance();
                if let Some(infix_rule) = get_rule(parser.previous.unwrap().token_type).infix {
                    infix_rule(parser);
                } else {
                    break
                }
                parser.print_parser();
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
                binary_op(parser, add_op);
            },
            TokenType::Minus => {
                if PARSE_FN_OUTPUT { println!("<minus>"); }
                binary_op(parser, sub_op);
            },
            TokenType::Star => {
                if PARSE_FN_OUTPUT { println!("<multiply>"); }
                binary_op(parser, mult_op);
            }
            TokenType::Slash => {
                if PARSE_FN_OUTPUT { println!("<divide>"); }
                binary_op(parser, div_op);
            }
            _ => {}
        }
        


    }
    
}

pub fn parse_grouping(parser: &mut Parser) {

    expression(parser);
    //parser.constant_stack.pop().unwrap().unwrap().print_leaf();
    parser.consume(TokenType::RightParen, "Expect ')' after expression, found something else at");
}

fn add_op(a: i32, b: i32) -> i32 {
    a + b
}
fn sub_op(a: i32, b: i32) -> i32 {
    a - b
}
fn mult_op(a: i32, b: i32) -> i32 {
    a * b
}
fn div_op(a: i32, b: i32) -> i32 {
    a / b
}
fn binary_op(parser: &mut Parser, operator: fn(i32, i32) -> i32) 
where

{
    let local_right = &mut parser.constant_stack.pop().unwrap_or_else(|| panic!());
    let local_left = &mut parser.constant_stack.pop().unwrap_or_else(|| panic!());
    
    let left = local_left.clone().unwrap();
    let right = local_right.clone().unwrap();

    
    match (left.data_type, right.data_type) {
        (DataType::Integer(a), DataType::Integer(b)) => {
            let calculation = operator(a, b);
            // println!("<add: <constant fold: {}+{}={}>>", a, b, a + b);
            parser.constant_stack.push(Some(Expr {
                left: String::from("i32 ") + &(calculation).to_string(),
                right: (calculation).to_string(),
                data_type: DataType::Integer(calculation) 
            }))
            
        }
        // _ => println!("<left operand: {}> <plus> <right operand: {}>", left.left, right.right)
    }
    // if PARSE_FN_OUTPUT {
    //     parser.print_parser();
    // }
}
