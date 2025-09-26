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
                DataType::Array(_, _) => {
                    // For arrays, the allocation and initialization is already handled in the array parsing
                    // The array expression should have already generated the necessary LLVM IR
                    "".to_string() // No additional store needed
                }
                _ => panic!("Unsupported data type for storing variables")
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
    let name = &dec.name;

    match datatype {
        DataType::Array(element_types, size) => {
            // Generate proper LLVM array type based on element type and size
            let element_type_str = if element_types.is_empty() {
                "i32" // Default to integer
            } else {
                match &element_types[0] {
                    DataType::Integer(_) => "i32",
                    DataType::Boolean(_) => "i1",
                    DataType::String(_) => "i8*",
                    _ => "i32" // Default fallback
                }
            };
            format!("%{name} = alloca [{size} x {element_type_str}], align 16")
        }
        _ => {
            let type_str = datatype.as_str();
            format!("%{name} = alloca {type_str}")
        }
    }
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