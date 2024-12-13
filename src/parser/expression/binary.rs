use crate::parser::parser::AstNode;
use super::{
    super::{expression::expression::parse_precedence, parser::Parser}, expr::{DataType, Expr, ParseError}, precedence::Precedence
};
use crate::parser::expression::expr::{Expression, Operation};
use crate::parser::expression::parse_rule::get_rule;
use crate::scanner::token::TokenType;
use crate::llvm::llvm_binary::llvm_binary_operands;

pub fn parse_binary(parser: &mut Parser)  -> Result<Expression, ParseError>{
    if let Some(token) = parser.previous {
       
        let operator_type = token.token_type;
        let rule_fn = get_rule(operator_type);
        let new_prec = rule_fn.precedence as u32;
        parse_precedence(parser, Precedence::from_u32(new_prec + 1));

        match operator_type {
            TokenType::Plus => binary_op(parser, add_op, Operation::Add),
            TokenType::Minus => binary_op(parser, sub_op, Operation::Sub),
            TokenType::Star => binary_op(parser, mult_op, Operation::Mul),
            TokenType::Slash => binary_op(parser, div_op, Operation::Div),


            TokenType::EqualEqual => binary_op(parser, eq_op, Operation::Equal),
            TokenType::BangEqual => binary_op(parser, neq_op, Operation::NotEqual),
            TokenType::Greater => binary_op(parser, gt_op, Operation::GreaterThan),
            TokenType::Less => binary_op(parser, lt_op, Operation::LessThan),
            TokenType::GreaterEqual => binary_op(parser, gte_op, Operation::GreaterEqual),
            TokenType::LessEqual => binary_op(parser, lte_op, Operation::LessEqual),

            _ => Err(ParseError::Generic)
        }
    } else {
        Err(ParseError::Generic)
    }
    
}
fn match_instruction(operation: Operation) -> (String, fn(i32, i32) -> i32) {
    match operation {
        Operation::Add => (ADD.to_string(), add_op),
        Operation::Sub => (SUB.to_string(), sub_op),
        Operation::Div => (DIV.to_string(), div_op),
        Operation::Mul => (MUL.to_string(), mult_op),
        Operation::Equal => (EQL.to_string(), eq_op),
        Operation::NotEqual => (NEQ.to_string(), neq_op),
        Operation::GreaterEqual => (GTE.to_string(), gte_op),
        Operation::GreaterThan => (GT.to_string(), gt_op),
        Operation::LessEqual => (LTE.to_string(), lte_op),
        Operation::LessThan => (LT.to_string(), lt_op)
    }
}
pub fn is_boolean_op(instruction: Operation) -> bool{
    match instruction {
        Operation::Add | Operation::Div | Operation::Mul | Operation::Sub => false,
        _ => true
    }
}
fn binary_op(parser: &mut Parser, operator: fn(i32, i32) -> i32, instruction: Operation)  -> Result<Expression, ParseError>
{
    let operands = get_binary_operands(parser);
    let operation = match_instruction(instruction.clone());
    let codegen = format!("\t%{} = {} {}, {}", parser.expr_count, operation.0, operands.0.left, operands.1.right);
    parser.emit_instruction(&codegen);

    let type_tag = match instruction {
        Operation::Add | Operation::Div | Operation::Mul | Operation::Sub => { "i32" },
        _ => { "i1" }
    };
            
    let b_expr = parser.ast_stack.pop().unwrap().to_expression();
    let a_expr = parser.ast_stack.pop().unwrap().to_expression();
    match (operands.0.data_type.clone(), operands.1.data_type.clone()) {
        (DataType::Integer(a), DataType::Integer(b)) => {
            let calculation = operator(a, b);
            let ast_node = Expression::new_binary(a_expr, b_expr, instruction, &parser.expr_count.to_string(), DataType::Integer(0));
            println!("{}; resolve_binary", ast_node.resolve_binary());

            parser.new_expr(llvm_binary_operands(calculation, parser.expr_count, &type_tag).unwrap());
            parser.expr_count += 1;
            parser.ast_stack.push(AstNode::Expression(ast_node.clone()));
                    
            return Ok(ast_node)
        },
        (DataType::Boolean(a), DataType::Boolean(b)) if is_boolean_op(instruction.clone()) => {
            let a_int =  if a { 1 } else { 0 };
            let b_int =  if b { 1 } else { 0 };
            
            let bool_op = operator(a_int, b_int);
            let ast_node = Expression::new_binary(a_expr, b_expr, instruction, &parser.expr_count.to_string(), DataType::Boolean(true));
            println!("{}; resolve binary", ast_node.resolve_binary());

            parser.constant_stack.push(llvm_binary_operands(bool_op, parser.expr_count, &type_tag));
            parser.expr_count += 1;
            parser.ast_stack.push(AstNode::Expression(ast_node.clone()));
            
            return Ok(ast_node)
        },
        (_, _) => return Err(ParseError::Generic)
                // no operators found
                
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
    (left, right)
}

const ADD: &str = "add";
const SUB: &str = "sub";
const MUL: &str = "mul";
const DIV: &str = "div";

const EQL: &str = "icmp eq";
const NEQ: &str = "icmp ne";
const GT: &str = "icmp sgt";
const GTE: &str = "icmp sge";
const LT: &str = "icmp slt";
const LTE: &str = "icmp sle";
const BOOL_OPS: [&str; 6] = [EQL, NEQ, GT, GTE, LT, LTE];