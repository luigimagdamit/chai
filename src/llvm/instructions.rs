use crate::parser::expression::expr::DataType;

// %0 = ...
pub struct LlvmTempVar {
    register_name: String,
    var_type: DataType, // holds operand values?
}
pub enum LlvmExpression {
    // Add(DataType),
    // Sub(DataType),
    // Mul(DataType),
    // Div(DataType),
    Bool(DataType),
    Number(DataType),
    String(DataType)
}