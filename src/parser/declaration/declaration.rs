use std::fmt;
use crate::parser::expression::expr::{DataType, Expression};
#[derive(Clone)]
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
#[derive(Clone)]
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
#[derive(Clone)]
pub struct VariableDeclaration {
    name: String,
    variable_type: DataType,
    expression: Option<Expression> // sometimes will have no value right?

}
impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(expr) = &self.expression {
            write!(f, "Variable Name: {}, Type: {},  Expresion: {}", self.name, expr.as_datatype(), expr)
        } else {
            write!(f, "Variable Name: {}", self.name)
        }
    }
}
impl VariableDeclaration {
    pub fn create_variable(&self) {
        match self.variable_type {
            DataType::Integer(_) => {
                println!("%{} = alloca i32", self.name)
            },
            _ => panic!()
        }
    }
    pub fn store(&self) {
        match &self.variable_type {
            DataType::Integer(_) => {
                if let Some(expr) = &self.expression {
                    println!("store i32 {}, i32* %{}", expr.resolve_operand(), self.name)
                }

            },
            _ => panic!()
        }
    }
    pub fn as_datatype(&self) -> DataType {
        if let Some(expr) = &self.expression {
            expr.as_datatype()
        } else {
            panic!()
        }
    }
}

#[derive(Clone)]
pub enum Declaration {
    Statement(Statement),
    Variable(VariableDeclaration)
}
impl Declaration {
    pub fn new_statement(statement: Statement) -> Declaration {
        Declaration::Statement(statement)
    }
    pub fn new_variable(name: String, expression: Option<Expression>, variable_type: DataType) -> Declaration {
        Declaration::Variable(VariableDeclaration {
            name,
            variable_type,
            expression
        })
    }
    pub fn as_variable(&self) -> &VariableDeclaration {
        match self {
            Declaration::Variable(inner) => {
                inner
            },
            _ => panic!()
        }
    }
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Declaration::Variable(var_struct) => {
                write!(f, "{var_struct}")
            },
            _ => panic!()
        }
    }
}