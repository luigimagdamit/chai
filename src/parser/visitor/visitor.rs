use crate::parser::expression::expr::{DataType, Binary, StringConstant, VariableExpression, ArrayExpression};
use crate::parser::declaration::declaration::{VariableDeclaration, PrintStatement};
pub trait Visitor {
    fn visit_literal(&mut self, literal: &DataType) -> String; // 1 (base type)
    fn visit_binary(&mut self, binary: &Binary) -> String; // 1 + 2  (container type)
    fn visit_string(&mut self, str_constant: &StringConstant) -> String; // "s" (base type)
    fn visit_variable_expression(&mut self, variable_expression: &VariableExpression) -> String;
    fn visit_array(&mut self, array: &ArrayExpression) -> String;

    // Statements
    fn visit_print(&mut self, print_statement: &PrintStatement) -> String;
    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) -> String;
}
pub trait Accept {
    fn accept<V: Visitor> (&self, visitor: &mut V) -> String;
}