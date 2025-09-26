/// Generic IR abstraction traits for expression code generation
/// This allows swapping between different IR backends for expression handling

use crate::parser::expression::expr::{DataType, Operation};

/// Trait for generating type representations in different IR backends
pub trait TypeIR {
    /// Get the type representation for integers
    fn int_type(&self) -> &'static str;

    /// Get the type representation for booleans
    fn bool_type(&self) -> &'static str;

    /// Get the type representation for strings/pointers
    fn string_type(&self) -> &'static str;

    /// Get the type representation for arrays
    fn array_type(&self, element_type: &str, size: usize) -> String;

    /// Get type string for a DataType
    fn datatype_to_string(&self, datatype: &DataType) -> &'static str {
        match datatype {
            DataType::Integer(_) => self.int_type(),
            DataType::Boolean(_) => self.bool_type(),
            DataType::String(_) => self.string_type(),
            DataType::Array(_, _) => {
                // Arrays need special handling since they require dynamic type construction
                // This should not be used for arrays - use alloca function directly
                "i32*" // Pointer to array - fallback
            }
        }
    }
}

/// Trait for generating binary operation instructions
pub trait BinaryOpIR {
    /// Generate a binary operation instruction
    /// Returns the complete instruction string (e.g., "%5 = add i32 %1, %2")
    fn binary_op(&self,
                 result_reg: &str,
                 op: &Operation,
                 left_operand: &str,
                 right_operand: &str,
                 result_type: &str) -> String;
}

/// Trait for generating literal/constant instructions
pub trait LiteralIR {
    /// Generate instruction for integer literal
    fn int_literal(&self, value: i32) -> String;

    /// Generate instruction for boolean literal
    fn bool_literal(&self, value: bool) -> String;

    /// Generate instruction for string literal
    fn string_literal(&self, register: usize, length: usize, index: usize) -> String;
}

/// Trait for generating array-related instructions
pub trait ArrayIR {
    /// Generate array allocation instruction
    fn array_alloca(&self, element_type: &str, size: usize, register: usize) -> String;

    /// Generate array element access instruction
    fn array_element_ptr(&self, array_ptr: &str, element_type: &str, index: usize, result_reg: usize) -> String;

    /// Generate array element load instruction
    fn array_element_load(&self, element_ptr: &str, element_type: &str, result_reg: usize) -> String;

    /// Generate array element store instruction
    fn array_element_store(&self, value: &str, element_ptr: &str, element_type: &str) -> String;

    /// Generate array initialization instruction
    fn array_init(&self, array_ptr: &str, values: &[String], element_type: &str) -> String;
}

/// Trait for generating print/call instructions for expressions
pub trait PrintIR {
    /// Generate print call for integer values
    fn print_int(&self, register: usize) -> String;

    /// Generate print call for boolean values
    fn print_bool(&self, register: usize) -> String;

    /// Generate print call for string values
    fn print_string(&self, register: usize) -> String;
}

/// Combined trait for complete expression IR generation
pub trait ExpressionIR: TypeIR + BinaryOpIR + LiteralIR + PrintIR + ArrayIR {
    /// Generate variable load instruction
    fn load_variable(&self, var_name: &str, var_type: &str, count: usize) -> String;

    /// Generate instruction to get register reference
    fn register_ref(&self, register: &str) -> String {
        format!("%{}", register)
    }
}

/// Factory trait for creating expression IR implementations
pub trait ExprIRFactory {
    type ExprIR: ExpressionIR;

    fn create_expr_ir(&self) -> Self::ExprIR;
}