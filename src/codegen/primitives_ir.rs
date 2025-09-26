/// Traits for primitive type IR generation across different backends

pub trait StringIR {
    /// Generate a new static string definition
    fn new_static_string(&self, length: usize, index: usize, value: &str) -> String;

    /// Generate code to retrieve/reference a static string
    fn retrieve_static_string(&self, length: usize, index: usize) -> String;

    /// Generate a string literal placement instruction
    fn string_literal(&self, register: usize, length: usize, index: usize) -> String;
}

pub trait NumberIR {
    /// Generate the left operand form of a number (with type info)
    fn number_left(&self, value: &str) -> String;

    /// Generate the right operand form of a number (raw value)
    fn number_right(&self, value: &str) -> String;
}

pub trait BooleanIR {
    /// Generate the left operand form of a boolean (with type info)
    fn boolean_left(&self, value: bool) -> String;

    /// Generate the right operand form of a boolean (raw value)
    fn boolean_right(&self, value: bool) -> String;
}

pub trait PrimitivesIR: StringIR + NumberIR + BooleanIR {}