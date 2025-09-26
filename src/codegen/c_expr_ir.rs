use super::expr_ir::{TypeIR, BinaryOpIR, LiteralIR, PrintIR, ArrayIR, ExpressionIR, ExprIRFactory};
use crate::parser::expression::expr::Operation;

/// C language implementation for expression IR generation
#[derive(Clone, Debug)]
pub struct CExpressionIR;

impl TypeIR for CExpressionIR {
    fn int_type(&self) -> &'static str {
        "int"
    }

    fn bool_type(&self) -> &'static str {
        "_Bool"
    }

    fn string_type(&self) -> &'static str {
        "char*"
    }

    fn array_type(&self, element_type: &str, size: usize) -> String {
        format!("{}[{}]", element_type, size)
    }
}

impl BinaryOpIR for CExpressionIR {
    fn binary_op(&self,
                 result_reg: &str,
                 op: &Operation,
                 left_operand: &str,
                 right_operand: &str,
                 _result_type: &str) -> String {
        let c_op = match op {
            Operation::Add => "+",
            Operation::Sub => "-",
            Operation::Mul => "*",
            Operation::Div => "/",
            Operation::Equal => "==",
            Operation::NotEqual => "!=",
            Operation::GreaterThan => ">",
            Operation::GreaterEqual => ">=",
            Operation::LessThan => "<",
            Operation::LessEqual => "<=",
        };

        format!("reg{} = {} {} {};", result_reg, left_operand, c_op, right_operand)
    }
}

impl LiteralIR for CExpressionIR {
    fn int_literal(&self, value: i32) -> String {
        value.to_string()
    }

    fn bool_literal(&self, value: bool) -> String {
        if value { "1" } else { "0" }.to_string()
    }

    fn string_literal(&self, register: usize, _length: usize, _index: usize) -> String {
        format!("reg{} = str_{};", register, register)
    }
}

impl PrintIR for CExpressionIR {
    fn print_int(&self, register: usize) -> String {
        format!("printf(\"%d\\n\", reg{});", register)
    }

    fn print_bool(&self, register: usize) -> String {
        format!("printf(\"%s\\n\", reg{} ? \"true\" : \"false\");", register)
    }

    fn print_string(&self, register: usize) -> String {
        format!("printf(\"%s\", reg{});", register)
    }
}

impl ArrayIR for CExpressionIR {
    fn array_alloca(&self, element_type: &str, size: usize, register: usize) -> String {
        format!("{} arr{}[{}];", element_type, register, size)
    }

    fn array_element_ptr(&self, array_ptr: &str, _element_type: &str, index: usize, result_reg: usize) -> String {
        format!("ptr{} = &{}[{}];", result_reg, array_ptr, index)
    }

    fn array_element_load(&self, element_ptr: &str, _element_type: &str, result_reg: usize) -> String {
        format!("reg{} = *{};", result_reg, element_ptr)
    }

    fn array_element_store(&self, value: &str, element_ptr: &str, _element_type: &str) -> String {
        format!("*{} = {};", element_ptr, value)
    }

    fn array_init(&self, array_ptr: &str, values: &[String], _element_type: &str) -> String {
        let mut instructions = Vec::new();
        for (i, value) in values.iter().enumerate() {
            instructions.push(format!("{}[{}] = {};", array_ptr, i, value));
        }
        instructions.join("\n")
    }
}

impl ExpressionIR for CExpressionIR {
    fn load_variable(&self, var_name: &str, _var_type: &str, count: usize) -> String {
        format!("{}_{} = {};", var_name, count, var_name)
    }

    fn register_ref(&self, register: &str) -> String {
        format!("reg{}", register)
    }
}

/// Factory for creating C expression IR implementations
pub struct CExprIRFactory;

impl ExprIRFactory for CExprIRFactory {
    type ExprIR = CExpressionIR;

    fn create_expr_ir(&self) -> Self::ExprIR {
        CExpressionIR
    }
}