use crate::parser::expression::expr::{DataType, Expression, Operation};
use crate::parser::declaration::declaration::VariableDeclaration;

/// Traits for declaration IR generation across different backends

pub trait VariableIR {
    fn new_variable(&self, var_decl: &VariableDeclaration) -> String;
    fn store_variable(&self, var_decl: &VariableDeclaration) -> String;
}

pub trait PrintIR {
    fn print_i32(&self, expr: &Expression) -> String;
    fn print_i1(&self, expr: &Expression) -> String;
    fn print_str_constant(&self, expr: &Expression) -> String;
}

pub trait DeclarationIR: VariableIR + PrintIR {
    /// Generate a print statement for any expression
    fn generate_print_statement(&self, expression: &Expression) -> String {
        match expression {
            Expression::Binary(binary) => {
                match binary.operator {
                    Operation::Equal | Operation::GreaterEqual | Operation::GreaterThan |
                    Operation::LessEqual | Operation::LessThan | Operation::NotEqual => {
                        self.print_i1(&Expression::from(binary))
                    },
                    _ => self.print_i32(&Expression::from(binary))
                }
            },
            Expression::Literal(literal) => {
                match literal {
                    DataType::Integer(_) => self.print_i32(&Expression::from(literal)),
                    DataType::Boolean(_) => self.print_i1(&Expression::from(literal)),
                    _ => panic!("Unsupported literal type for printing")
                }
            },
            Expression::StringConstant(_str_constant) => {
                self.print_str_constant(expression)
            }
            Expression::Variable(variable) => {
                match variable.datatype {
                    DataType::Integer(_) => self.print_i32(&Expression::from(variable.clone())),
                    DataType::Boolean(_) => self.print_i1(&Expression::from(variable.clone())),
                    DataType::String(_) => self.print_str_constant(&Expression::from(variable.clone())),
                    DataType::Array(_, _) => panic!("Array printing not supported yet"),
                }
            },
            _ => panic!("Unsupported expression type for printing")
        }
    }

    /// Generate a variable declaration with storage
    fn generate_variable_declaration(&self, var_decl: &VariableDeclaration) -> String {
        format!("{}\n\t{}", self.new_variable(var_decl), self.store_variable(var_decl))
    }
}