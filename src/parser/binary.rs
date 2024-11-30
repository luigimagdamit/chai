use crate::{common::common::PARSE_DECLARATION_MODE, parser::{
    expr::{
        DataType,
        Expr
    }, parse_fn::parse_precedence, parse_rule::get_rule, parser::Parser, precedence::Precedence
    
}};
use crate::common::common::{
    PARSE_FN_OUTPUT,
    PARSE_CONSTANT_FOLD
};
use crate::scanner::token::TokenType;

pub fn parse_binary(parser: &mut Parser) {
    if let Some(token) = parser.previous {
       
        let operator_type = token.token_type;
        let rule_fn = get_rule(operator_type);
        let new_prec = rule_fn.precedence as u32;
        parse_precedence(parser, Precedence::from_u32(new_prec + 1));

        match operator_type {
            TokenType::Plus => {
                if PARSE_FN_OUTPUT { println!("<add>"); }
                if PARSE_DECLARATION_MODE{ print!("%{} = add ", parser.expr_count) }
                
                binary_op(parser, add_op);
                parser.expr_count += 1;
                
            },
            TokenType::Minus => {
                if PARSE_FN_OUTPUT { println!("<minus>"); }
                print!("%{} = sub ", parser.expr_count);
                
                binary_op(parser, sub_op);
                parser.expr_count += 1;
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
    print!("{}, ", left.left);
    print!("{}\n", right.right);
    
    match (left.data_type, right.data_type) {
        (DataType::Integer(a), DataType::Integer(b)) => {
            let calculation = operator(a, b);
            if PARSE_CONSTANT_FOLD {
                parser.constant_stack.push(Some(Expr {
                left: String::from("i32 ") + &(calculation).to_string(),
                right: (calculation).to_string(),
                data_type: DataType::Integer(calculation) 
                }))
            } else {
                parser.constant_stack.push(Some(Expr {
                    left: String::from("i32 %") + &(parser.expr_count).to_string(),
                    right: String::from("%") + &(parser.expr_count).to_string(),
                    data_type: DataType::Integer(calculation) 
                }))
            }
            // println!("<add: <constant fold: {}+{}={}>>", a, b, a + b);


            
        }
        (_, _) => {
            if let Some(t) = parser.previous {
                parser.error_at(&t, "Invalid binary operands");
            }
        },
        // _ => println!("<left operand: {}> <plus> <right operand: {}>", left.left, right.right)
    }
    // if PARSE_FN_OUTPUT {
    //     parser.print_parser();
    // }
}
