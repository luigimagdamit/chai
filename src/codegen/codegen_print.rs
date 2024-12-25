use crate::parser::expression::expr::{DataType, Binary, StringConstant, Expression, Operation, VariableExpression};
use crate::parser::declaration::declaration::{VariableDeclaration, PrintStatement};

pub trait CodegenPrint {
    fn print_i1(expr: &Expression) -> String;
    fn print_i32(expr: &Expression) -> String;
    fn print_str_constant(expr: &Expression) -> String;
    fn new_variable(dec: &VariableDeclaration) -> String;
    fn store_variable(dec: &VariableDeclaration) -> String;
    fn var_expr(expr: &VariableExpression) -> String;
}