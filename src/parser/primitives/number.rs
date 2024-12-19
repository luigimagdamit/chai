use crate::parser::expression::expr::ParseError;
use crate::parser::core::ast_node::AstNode;
use super::super::parser::Parser;
use super::super:: expression::expr::{DataType, Expr, Expression};
pub fn parse_number(parser: &mut Parser) -> Result<Expression, ParseError> {
    let value = String::from(parser.previous.unwrap().start);
    let number_leaf = Expr {
        left: LlvmNumberTag::Integer(value.clone()).left(),
        right: LlvmNumberTag::Integer(value.clone()).right(),
        data_type: DataType::Integer(value.parse().unwrap())
    };

    parser.constant_stack.push(Some(number_leaf.clone()));
    let expr_ast = Expression::Literal(number_leaf.data_type);
    parser.ast_stack.push(AstNode::from_expression(expr_ast.clone()));
    Ok(expr_ast)
}

pub enum LlvmNumberTag {
    Integer(String)
}
impl LlvmNumberTag {
    pub fn left(&self) -> String {
        match self {
            LlvmNumberTag::Integer(int) => format!("i32 {}", int).to_string()
        }
        
    }
    pub fn right(&self) -> String {
        match self {
            LlvmNumberTag::Integer(int) => int.to_string()
        }
        
    }
}