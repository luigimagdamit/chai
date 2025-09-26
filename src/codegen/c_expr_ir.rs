use super::expr_ir::{TypeIR, BinaryOpIR, LiteralIR, PrintIR, ExpressionIR, ExprIRFactory};
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