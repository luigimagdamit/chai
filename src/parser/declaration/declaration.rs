use std::fmt;
use crate::parser::expression::expr::Expression;

pub struct PrintStatement {
    pub expression: Expression
}
impl From<Expression> for PrintStatement {
    fn from(expression: Expression) -> Self {
        PrintStatement {
            expression
        }
    }
}
pub enum Statement {
    PrintStatement(PrintStatement)
}
impl From<PrintStatement> for Statement {
    fn from(print_statement: PrintStatement) -> Statement {
        Statement::PrintStatement(print_statement)
    }
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
            Statement::PrintStatement(expr) => write!(f, "\nPrint Statement => \n  | {}", expr.expression)
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