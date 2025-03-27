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
    fn new_variable(dec: &VariableDeclaration) -> String { alloca(dec) }

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
        load(expr)
        
    }
}

// Load Codegen Function
// Loads a value at the pointer location
// TODO: Modify load so that it just takes a any keyword like load. can share with load, store
fn load(var_expr: &VariableExpression) -> String {
    let type_str = (&var_expr.datatype).as_str();
    format!("%{}_{} = load {type_str}, {type_str}* %{} ; loading existing variable", var_expr.name, var_expr.count, var_expr.name)
}
fn store() {

}
fn alloca(dec: &VariableDeclaration) -> String {
    let datatype = &dec.as_datatype();
    let type_str = datatype.as_str();
    let name = &dec.name;
    format!("%{name} = alloca {type_str}")
    // match dec.as_datatype() {
    //     DataType::Integer(_) => format!("%{} = alloca i32", dec.name),
    //     DataType::Boolean(_) => format!("%{} = alloca i1", dec.name),
    //     DataType::String(_) => format!("%{} = alloca i8*", dec.name)
    // }
}
enum InstructionType {
    Integer,
    Boolean,
    StringPtr
}
impl InstructionType {
    fn as_str(&self) -> &str {
        match self {
            InstructionType::Integer => "i32",
            InstructionType::Boolean => "i1",
            InstructionType::StringPtr => "i8*"
        }
    }
}