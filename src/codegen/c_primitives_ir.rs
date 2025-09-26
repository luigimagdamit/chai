use super::primitives_ir::{StringIR, NumberIR, BooleanIR, PrimitivesIR};

/// C-specific implementation for primitives IR generation
#[derive(Clone, Debug)]
pub struct CPrimitivesIR;

impl StringIR for CPrimitivesIR {
    fn new_static_string(&self, _length: usize, index: usize, value: &str) -> String {
        format!("static const char str_{} = \"{}\";", index, value)
    }

    fn retrieve_static_string(&self, _length: usize, index: usize) -> String {
        format!("str_{}", index)
    }

    fn string_literal(&self, register: usize, _length: usize, index: usize) -> String {
        format!("char* reg{} = str_{};", register, index)
    }
}

impl NumberIR for CPrimitivesIR {
    fn number_left(&self, value: &str) -> String {
        // For C, both left and right are the same - just the numeric value
        value.to_string()
    }

    fn number_right(&self, value: &str) -> String {
        value.to_string()
    }
}

impl BooleanIR for CPrimitivesIR {
    fn boolean_left(&self, value: bool) -> String {
        // For C, use true/false or 1/0
        match value {
            true => "true".to_string(),
            false => "false".to_string(),
        }
    }

    fn boolean_right(&self, value: bool) -> String {
        match value {
            true => "1".to_string(),
            false => "0".to_string(),
        }
    }
}

impl PrimitivesIR for CPrimitivesIR {}