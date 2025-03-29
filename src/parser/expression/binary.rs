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


fn binary_op(parser: &mut Parser, inst: Operation)  -> Result<Expression, ParseError>
{
    let (b_expr, a_expr) = get_binary_operands(parser);

    match (b_expr.as_datatype(), a_expr.as_datatype()) {
        (DataType::Integer(_), DataType::Integer(_)) => {
            let ast_node = Expression::new_binary(b_expr, a_expr, inst, &parser.expr_increment().to_string(), DataType::empty_int());

            let codegen = "".to_string() + &ast_node.register();
            parser.emit_instruction(&codegen);
            parser.ast_stack.push(AstNode::Expression(ast_node.clone()));
                    
            return Ok(ast_node)
        },
        (DataType::Boolean(_), DataType::Boolean(_)) if inst.is_boolean_op() => {
            let ast_node = Expression::new_binary(b_expr, a_expr, inst, &parser.expr_increment().to_string(), DataType::empty_bool());
            let codegen = "".to_string() + &ast_node.register();
            parser.emit_instruction(&codegen);


            parser.ast_stack.push(AstNode::Expression(ast_node.clone()));
            
            return Ok(ast_node)
        },
        (_, _) => return Err(ParseError::Generic)
    }
}



fn get_binary_operands(parser: &mut Parser) -> (Expression, Expression) {
    
    let local_right = parser.ast_stack
        .pop()
        .expect("Tried to get right operand from ast_node option, but unwrapped none");
    let local_left = parser.ast_stack
        .pop()
        .expect("Tried to get left operand from ast_node option, but unwrapped none");
    
    (local_left.to_expression(), local_right.to_expression())

}
