use super::primitives_ir::{StringIR, NumberIR, BooleanIR, PrimitivesIR};
use crate::llvm::llvm_string::{llvm_new_static_string, llvm_retrieve_static_string};

/// LLVM-specific implementation for primitives IR generation
#[derive(Clone, Debug)]
pub struct LlvmPrimitivesIR;

impl StringIR for LlvmPrimitivesIR {
    fn new_static_string(&self, length: usize, index: usize, value: &str) -> String {
        llvm_new_static_string(length, index, value)
    }

    fn retrieve_static_string(&self, length: usize, index: usize) -> String {
        llvm_retrieve_static_string(length, index)
    }

    fn string_literal(&self, register: usize, length: usize, index: usize) -> String {
        format!("%{} = getelementptr inbounds [{}x i8], [{}x i8]* @str.{}, i32 0, i32 0",
                register, length, length, index)
    }
}

impl NumberIR for LlvmPrimitivesIR {
    fn number_left(&self, value: &str) -> String {
        format!("i32 {}", value)
    }

    fn number_right(&self, value: &str) -> String {
        value.to_string()
    }
}

impl BooleanIR for LlvmPrimitivesIR {
    fn boolean_left(&self, value: bool) -> String {
        match value {
            true => "i1 1".to_string(),
            false => "i1 0".to_string(),
        }
    }

    fn boolean_right(&self, value: bool) -> String {
        match value {
            true => "1".to_string(),
            false => "0".to_string(),
        }
    }
}

impl PrimitivesIR for LlvmPrimitivesIR {}