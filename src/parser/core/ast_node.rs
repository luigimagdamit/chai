use crate::parser::expression::expr::Expression;
use crate::parser::declaration::declaration::{Statement, Declaration};
use std::fmt;

#[derive(Clone)]
pub enum AstNode {
    Declaration(Declaration),
    Expression(Expression)
}
impl AstNode {
    pub fn from_expression(expression: Expression) -> AstNode {
        AstNode::Expression(expression)
    }
    pub fn to_expression(self) -> Expression {
        match self {
            AstNode::Expression(expr) => expr,
            _ => Expression::Empty
        }
    }

}
impl From<Declaration> for AstNode {
    fn from(value: Declaration) -> Self {
        AstNode::Declaration(value)
    }
}
impl From<Statement> for AstNode {
    fn from(value: Statement) -> Self {
        AstNode::Declaration(Declaration::Statement(value))
    }
}
impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNode::Declaration(_) => write!(f, "todo: declaration ast"),
            AstNode::Expression(e) => write!(f, "AstNode: Expression => {e}")
        }
    }
}