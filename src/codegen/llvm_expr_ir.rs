use super::expr_ir::{TypeIR, BinaryOpIR, LiteralIR, PrintIR, ArrayIR, ExpressionIR, ExprIRFactory};
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

    fn array_type(&self, element_type: &str, size: usize) -> String {
        format!("[{} x {}]*", size, element_type)
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

impl ArrayIR for LlvmExpressionIR {
    fn array_alloca(&self, element_type: &str, size: usize, register: usize) -> String {
        format!("%{} = alloca [{} x {}], align 16", register, size, element_type)
    }

    fn array_element_ptr(&self, array_ptr: &str, element_type: &str, index: usize, result_reg: usize) -> String {
        format!("%{} = getelementptr inbounds [{} x {}], [{} x {}]* {}, i64 0, i64 {}",
                result_reg, element_type, element_type, element_type, element_type, array_ptr, index)
    }

    fn array_element_load(&self, element_ptr: &str, element_type: &str, result_reg: usize) -> String {
        format!("%{} = load {}, {}* {}", result_reg, element_type, element_type, element_ptr)
    }

    fn array_element_store(&self, value: &str, element_ptr: &str, element_type: &str) -> String {
        format!("store {} {}, {}* {}", element_type, value, element_type, element_ptr)
    }

    fn array_init(&self, array_ptr: &str, values: &[String], element_type: &str) -> String {
        let mut instructions = Vec::new();
        for (i, value) in values.iter().enumerate() {
            let ptr_reg = format!("ptr_{}", i);
            instructions.push(self.array_element_ptr(array_ptr, element_type, i, i * 1000)); // Use distinct register numbers
            instructions.push(self.array_element_store(value, &format!("%{}", i * 1000), element_type));
        }
        instructions.join("\n\t")
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