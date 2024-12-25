use crate::parser::expression::expr::{DataType, Binary, StringConstant, Expression, Operation, VariableExpression};
use crate::parser::declaration::declaration::{VariableDeclaration, PrintStatement};
use super::codegen_print::CodegenPrint;
pub struct LlvmPrint;
impl CodegenPrint for LlvmPrint {
    fn print_i1(expr: &Expression) -> String {
        format!("call void @print_i1(i1 {}); signature from PrintVisitor\n", Expression::from(expr.clone()).resolve_operand())
    }
    fn print_i32(expr: &Expression) -> String {
        format!("call void @print_i32(i32 {}); signature from PrintVisitor\n", Expression::from(expr.clone()).resolve_operand())
    }
    fn print_str_constant(expr: &Expression) -> String {
        format!("call i32 (i8*, ...) @printf(i8* {})", expr.resolve_operand())
    }
    fn new_variable(dec: &VariableDeclaration) -> String {
        match dec.as_datatype() {
            DataType::Integer(_) => format!("%{} = alloca i32", dec.name),
            DataType::Boolean(_) => format!("%{} = alloca i1", dec.name),
            DataType::String(_) => format!("%{} = alloca i8*", dec.name)
        }
        
    }
    fn store_variable(dec: &VariableDeclaration) -> String {
        if let Some(expr) = &dec.expression {
            match expr.as_datatype() {
                DataType::Integer(_) => format!("store i32 {}, i32* %{}", expr.resolve_operand(), dec.name),
                DataType::Boolean(_) => format!("store i1 {}, i1* %{}", expr.resolve_operand(), dec.name),
                DataType::String(_) => format!("store i8* {}, i8** %{}", expr.resolve_operand(), dec.name),
                _ => panic!("Strings not supported for storing variables")
            }
            
        } else {
            "".to_string()
        }
        
        
    }
    fn var_expr(expr: &VariableExpression) -> String {
        match expr.datatype {
            DataType::Integer(_) => format!("%{}_{} = load i32, i32* %{} ; loading existing variable", expr.name, expr.count, expr.name),
            DataType::Boolean(_) => format!("%{}_{} = load i1, i1* %{} ; loading existing variable", expr.name, expr.count, expr.name),
            DataType::String(_) => format!("%{}_{} = load i8*, i8** %{} ; loading existing variable", expr.name, expr.count, expr.name),
            
            _ => panic!("not supported for strings: variable expressions")
        }
        
    }
}