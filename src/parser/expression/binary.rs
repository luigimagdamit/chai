use crate::parser::expression::expr::Register;
use crate::parser::core::ast_node::AstNode;
use super::{
    super::{expression::expression::parse_precedence, parser::Parser}, expr::{DataType, ParseError}, precedence::Precedence
};
use crate::parser::expression::expr::{Expression, Operation};
use crate::parser::expression::parse_rule::get_rule;
use crate::scanner::token::TokenType;

pub fn parse_binary(parser: &mut Parser)  -> Result<Expression, ParseError>{
    if let Some(token) = parser.previous {
       
        let operator_type = token.token_type;
        let rule_fn = get_rule(operator_type);
        let new_prec = rule_fn.precedence as u32;
        parse_precedence(parser, Precedence::from_u32(new_prec + 1));

        match operator_type {
            TokenType::Plus => binary_op(parser, Operation::Add),
            TokenType::Minus => binary_op(parser, Operation::Sub),
            TokenType::Star => binary_op(parser, Operation::Mul),
            TokenType::Slash => binary_op(parser, Operation::Div),


            TokenType::EqualEqual => binary_op(parser, Operation::Equal),
            TokenType::BangEqual => binary_op(parser, Operation::NotEqual),
            TokenType::Greater => binary_op(parser, Operation::GreaterThan),
            TokenType::Less => binary_op(parser, Operation::LessThan),
            TokenType::GreaterEqual => binary_op(parser, Operation::GreaterEqual),
            TokenType::LessEqual => binary_op(parser, Operation::LessEqual),

            _ => Err(ParseError::Generic)
        }
    } else {
        Err(ParseError::Generic)
    }
    
}

pub fn is_boolean_op(instruction: Operation) -> bool{
    match instruction {
        Operation::Add | Operation::Div | Operation::Mul | Operation::Sub => false,
        _ => true
    }
}
fn binary_op(parser: &mut Parser, instruction: Operation)  -> Result<Expression, ParseError>
{
    let operands = get_binary_operands(parser);

            
    let b_expr = operands.0;
    let a_expr = operands.1;
    
    match (b_expr.as_datatype(), a_expr.as_datatype()) {
        (DataType::Integer(_), DataType::Integer(_)) => {
            let datatype = DataType::Integer(0);
            let ast_node = Expression::new_binary(b_expr, a_expr, instruction, &parser.expr_increment().to_string(), datatype);

            let codegen = "\t".to_string() + &ast_node.register();
            parser.emit_instruction(&codegen);
            parser.ast_stack.push(AstNode::Expression(ast_node.clone()));
                    
            return Ok(ast_node)
        },
        (DataType::Boolean(_), DataType::Boolean(_)) if is_boolean_op(instruction.clone()) => {
            let register= parser.expr_increment();
            let ast_node = Expression::new_binary(b_expr, a_expr, instruction, &register.to_string(), DataType::Boolean(true));
            let codegen = "\t".to_string() + &ast_node.register();
            parser.emit_instruction(&codegen);


            parser.ast_stack.push(AstNode::Expression(ast_node.clone()));
            
            return Ok(ast_node)
        },
        (_, _) => return Err(ParseError::Generic)
    }
}



fn get_binary_operands(parser: &mut Parser) -> (Expression, Expression) {
    
    let local_right = parser.ast_stack.pop().unwrap_or_else(|| panic!());
    let local_left = parser.ast_stack.pop().unwrap_or_else(|| panic!());
    
    (local_left.to_expression(), local_right.to_expression())

}
