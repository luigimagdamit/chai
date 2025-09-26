use crate::parser::expression::expr::{DataType, Binary, StringConstant, Operation, VariableExpression, ArrayExpression};
use crate::parser::declaration::declaration::{VariableDeclaration, PrintStatement};
use crate::parser::visitor::visitor::{Visitor, Accept};

pub struct RebuildVisitor;
impl Visitor for RebuildVisitor {
    fn visit_literal(&mut self, literal: &DataType) -> String {
        literal.to_string()
    }
    fn visit_binary(&mut self, binary: &Binary) -> String {
        let left = binary.get_left().accept(self);

        let operator = match binary.operator {
            Operation::Add => "+",
            Operation::Sub => "-",
            Operation::Mul => "*",
            Operation::Div => "/",
            Operation::Equal => "==",
            Operation::NotEqual => "!=",
            Operation::GreaterEqual => ">=",
            Operation::GreaterThan => ">",
            Operation::LessEqual => "<=",
            Operation::LessThan => "<"

        };
        let right = binary.get_right().accept(self);
        format!("({} {} {})", left, operator, right)
    }
    fn visit_string(&mut self, str_constant: &StringConstant) -> String {
        format!("{}", str_constant.name)
    }
    fn visit_array(&mut self, array: &ArrayExpression) -> String {
        format!("array[{}]:{}", array.size, array.name)
    }
    fn visit_print(&mut self, print_statement: &PrintStatement) -> String {
        format!("print({});", print_statement.expression.accept(self))
    }
    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) -> String {
        match variable_declaration.as_datatype() {
            DataType::Integer(_) => {
                if let Some(expr) = &variable_declaration.expression {
                    format!("var {} : int = {}", variable_declaration.name, expr.accept(self))
                } else {
                    format!("var {} : int;", variable_declaration.name)
                }
                
            },
            DataType::Boolean(_) => {
                if let Some(expr) = &variable_declaration.expression {
                    format!("var {} : bool = {};", variable_declaration.name, expr.accept(self))
                } else {
                    format!("var {} : bool;", variable_declaration.name)
                }
            },
            DataType::String(_) => {
                if let Some(expr) = &variable_declaration.expression {
                    format!("var {} : str = {};", variable_declaration.name, expr.accept(self))
                } else {
                    format!("var {} : str;", variable_declaration.name)
                }
            },
            DataType::Array(_, size) => {
                if let Some(expr) = &variable_declaration.expression {
                    format!("var {} : array[{}] = {};", variable_declaration.name, size, expr.accept(self))
                } else {
                    format!("var {} : array[{}];", variable_declaration.name, size)
                }
            },
        }
        
    }
    fn visit_variable_expression(&mut self, variable_expression: &VariableExpression) -> String {
        format!("{}", variable_expression.name)
    }
}