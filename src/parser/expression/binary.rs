use crate::common::common::PARSE_DECLARATION_MODE;
use super::{
    expr::{Expr, DataType},
    precedence::Precedence,
    super::{parse_fn::parse_precedence, parser::Parser}
};
use crate::parser::expression::parse_rule::get_rule;
use crate::scanner::token::TokenType;
use crate::llvm::llvm_binary::llvm_binary_operands;

pub fn parse_binary(parser: &mut Parser) {
    if let Some(token) = parser.previous {
       
        let operator_type = token.token_type;
        let rule_fn = get_rule(operator_type);
        let new_prec = rule_fn.precedence as u32;
        parse_precedence(parser, Precedence::from_u32(new_prec + 1));

        match operator_type {
            TokenType::Plus => binary_op(parser, add_op, ADD),
            TokenType::Minus => binary_op(parser, sub_op, SUB),
            TokenType::Star => binary_op(parser, mult_op, MUL),
            TokenType::Slash => binary_op(parser, div_op, DIV),

            // Boolean
            TokenType::EqualEqual => binary_op(parser, eq_op, EQL),
            TokenType::BangEqual => binary_op(parser, neq_op, NEQ),
            TokenType::Greater => binary_op(parser, gt_op, GT),
            TokenType::Less => binary_op(parser, lt_op, LT),
            TokenType::GreaterEqual => binary_op(parser, gte_op, GTE),
            TokenType::LessEqual => binary_op(parser, lte_op, LTE),
            _ => parser.error_at(&token, "Unsupported binary instruction: check parse_binary()")
        }
    }
    
}
fn binary_op(parser: &mut Parser, operator: fn(i32, i32) -> i32, instruction: &str) 
{
    let operands = get_binary_operands(parser);
    let codegen = format!("%{} = {} {}, {}", parser.expr_count, instruction, operands.0.left, operands.1.right);
    if PARSE_DECLARATION_MODE{ println! ("{}", codegen)}
    parser.emitInstruction(&codegen);


    
    // (left, right)
    match (operands.0.data_type, operands.1.data_type) {
        (DataType::Integer(a), DataType::Integer(b)) => {
            let calculation = operator(a, b);
            if BOOL_OPS.contains(&instruction) {
                parser.constant_stack.push(llvm_binary_operands(calculation, parser.expr_count, "i1"));
            } else {
                parser.constant_stack.push(llvm_binary_operands(calculation, parser.expr_count, "i32"));
            }
        },
        (DataType::Boolean(a), DataType::Boolean(b)) => {
            let a_int = match a {
                true => 1,
                false=> 0
            };
            let b_int = match b {
                true => 1,
                false=> 0
            };

            let bool_op = operator(a_int, b_int);
            parser.constant_stack.push(llvm_binary_operands(bool_op, parser.expr_count, "i1"));
        },
        (_, _) => parser.error_at(&parser.current.unwrap(), "Invalid binary operands while trying to parse in binary_op()")

    }
    parser.expr_count += 1;
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

fn eq_op(a: i32, b: i32) -> i32 {
    let res = a == b;
    if res {1} else {0}
}
fn neq_op(a: i32, b: i32) -> i32 {
    let res = a != b;
    if res {1} else {0}
}
fn gt_op(a: i32, b: i32) -> i32 {
    let res = a > b;
    if res {1} else {0}
}
fn gte_op(a: i32, b: i32) -> i32 {
    let res = a >= b;
    if res {1} else {0}
}
fn lt_op(a: i32, b: i32) -> i32 {
    let res = a < b;
    if res {1} else {0}
}
fn lte_op(a: i32, b: i32) -> i32 {
    let res = a <= b;
    if res {1} else {0}
}

fn get_binary_operands(parser: &mut Parser) -> (Expr, Expr) {
    let local_right = &mut parser.constant_stack.pop().unwrap_or_else(|| panic!());
    let local_left = &mut parser.constant_stack.pop().unwrap_or_else(|| panic!());
    
    let left = local_left.clone().unwrap();
    let right = local_right.clone().unwrap();
    return (left, right)
}

const ADD: &str = "add";
const SUB: &str = "sub";
const MUL: &str = "mul";
const DIV: &str = "div";

const EQL: &str = "icmp eq";
const NEQ: &str = "icmp ne";
const GT: &str = "icmp ugt";
const GTE: &str = "icmp uge";
const LT: &str = "icmp ult";
const LTE: &str = "icmp ule";
const BOOL_OPS: [&'static str; 6] = [EQL, NEQ, GT, GTE, LT, LTE];