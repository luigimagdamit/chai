use super::expr_ir::{TypeIR, BinaryOpIR, LiteralIR, PrintIR, ExpressionIR, ExprIRFactory};
use crate::parser::expression::expr::Operation;
use crate::llvm::llvm_print::llvm_call_print_local;
use crate::llvm::llvm_string::llvm_retrieve_static_string;

/// LLVM-specific implementation for expression IR generation
#[derive(Clone, Debug)]
pub struct LlvmExpressionIR;

impl TypeIR for LlvmExpressionIR {
    fn int_type(&self) -> &'static str {
        "i32"
    }

    fn bool_type(&self) -> &'static str {
        "i1"
    }

    fn string_type(&self) -> &'static str {
        "i8*"
    }
}

impl BinaryOpIR for LlvmExpressionIR {
    fn binary_op(&self,
                 result_reg: &str,
                 op: &Operation,
                 left_operand: &str,
                 right_operand: &str,
                 result_type: &str) -> String {
        format!("%{} = {} {} {}, {}",
                result_reg,
                op.to_string(),
                result_type,
                left_operand,
                right_operand)
    }
}

impl LiteralIR for LlvmExpressionIR {
    fn int_literal(&self, value: i32) -> String {
        format!("add i32 {}, 0", value)
    }

    fn bool_literal(&self, value: bool) -> String {
        let bool_val = if value { 1 } else { 0 };
        format!("add i1 {}, 0", bool_val)
    }

    fn string_literal(&self, register: usize, length: usize, index: usize) -> String {
        format!("%{} = {}", register, llvm_retrieve_static_string(length, index))
    }
}

impl PrintIR for LlvmExpressionIR {
    fn print_int(&self, register: usize) -> String {
        llvm_call_print_local(register as u32, "i32")
    }

    fn print_bool(&self, register: usize) -> String {
        llvm_call_print_local(register as u32, "i1")
    }

    fn print_string(&self, register: usize) -> String {
        format!("call i32 (i8*, ...) @printf(i8* %{})", register)
    }
}

impl ExpressionIR for LlvmExpressionIR {
    fn load_variable(&self, var_name: &str, var_type: &str, count: usize) -> String {
        format!("%{}_{} = load {}, {}* %{} ; loading existing variable",
                var_name, count, var_type, var_type, var_name)
    }
}

/// Factory for creating LLVM expression IR implementations
pub struct LlvmExprIRFactory;

impl ExprIRFactory for LlvmExprIRFactory {
    type ExprIR = LlvmExpressionIR;

    fn create_expr_ir(&self) -> Self::ExprIR {
        LlvmExpressionIR
    }
}