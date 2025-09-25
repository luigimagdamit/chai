use crate::parser::expression::expr::{DataType, Binary, StringConstant, Expression, Operation, VariableExpression};
use crate::parser::declaration::declaration::{VariableDeclaration, PrintStatement};
use crate::parser::visitor::visitor::Visitor;
use crate::parser::expression::expr::ExprNode;
pub struct PrintVisitor;

// Visited nodes should have some method print
impl Visitor for PrintVisitor {
    fn visit_literal(&mut self, literal: &DataType) -> String{
        literal.print()
    }
    fn visit_binary(&mut self, binary: &Binary) -> String {
        binary.print()
    }
    fn visit_string(&mut self, str_constant: &StringConstant) -> String {
        str_constant.print()
    }
    fn visit_print(&mut self, print_statement: &PrintStatement) -> String {
        print_statement.print()
    }
    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) -> String {
        variable_declaration.print()
    }
    fn visit_variable_expression(&mut self, variable_expression: &VariableExpression) -> String {
        variable_expression.print()
    }
}