use std::fmt;
use crate::parser::expression::expr::{DataType, Expression};
use crate::parser::visitor::visitor::{Accept, Visitor};
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

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::PrintStatement(expr) => write!(f, "\nPrint Statement => \n  | {}", expr.expression)
        }
    }
}
#[derive(Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub variable_type: DataType,
    pub expression: Option<Expression> // sometimes will have no value right?

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

    pub fn as_datatype(&self) -> DataType {
        if let Some(expr) = &self.expression {
            expr.as_datatype()
        } else {
            self.variable_type.clone()
        }
    }
}

#[derive(Clone)]
pub enum Declaration {
    Statement(Statement),
    Variable(VariableDeclaration)
}
impl Accept for Declaration {
    fn accept<V: Visitor> (&self, visitor: &mut V) -> String{
        match self {    
            Declaration::Statement(statement) => {
                match statement {
                    Statement::PrintStatement(print_statement) => visitor.visit_print(print_statement)
                }
            },
            Declaration::Variable(var_declaration) => visitor.visit_variable_declaration(var_declaration)
        }
    }
}
impl From<PrintStatement> for Declaration {
    fn from(value: PrintStatement) -> Self {
        Declaration::Statement(Statement::PrintStatement(value))
    }
}
impl Declaration {
    pub fn new_variable(name: &str, expression: Option<Expression>, variable_type: DataType) -> Declaration {
        Declaration::Variable(VariableDeclaration {
            name: name.to_string(),
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