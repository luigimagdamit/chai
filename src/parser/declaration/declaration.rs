use std::fmt;
use crate::parser::expression::expr::Expression;

pub struct PrintStatement {
    pub expression: Expression
}
pub enum Statement {
    PrintStatement(PrintStatement)
}
impl Statement {
    pub fn new_print_statement(expression: Expression) -> Statement{
        Statement::PrintStatement(PrintStatement{
            expression
        })
    }
}
impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::PrintStatement(expr) => write!(f, "Print Statement => \n  | {}", expr.expression)
        }
    }
}
pub enum Declaration {
    Statement(Statement)
}
impl Declaration {
    pub fn new_statement(statement: Statement) -> Declaration {
        Declaration::Statement(statement)
    }
}