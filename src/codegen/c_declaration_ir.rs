use super::declaration_ir::{VariableIR, PrintIR, DeclarationIR};
use crate::parser::expression::expr::Expression;
use crate::parser::declaration::declaration::VariableDeclaration;

/// C-specific implementation for declaration IR generation
#[derive(Clone, Debug)]
pub struct CDeclarationIR;

impl VariableIR for CDeclarationIR {
    fn new_variable(&self, var_decl: &VariableDeclaration) -> String {
        let type_str = match var_decl.variable_type {
            crate::parser::expression::expr::DataType::Integer(_) => "int",
            crate::parser::expression::expr::DataType::Boolean(_) => "bool",
            crate::parser::expression::expr::DataType::String(_) => "char*",
            crate::parser::expression::expr::DataType::Array(_, size) => &format!("int[{}]", size), // Basic array support
        };
        format!("{} {};", type_str, var_decl.name)
    }

    fn store_variable(&self, var_decl: &VariableDeclaration) -> String {
        if let Some(expr) = &var_decl.expression {
            format!("{} = {};", var_decl.name, expr.resolve_operand())
        } else {
            String::new()
        }
    }
}

impl PrintIR for CDeclarationIR {
    fn print_i32(&self, expr: &Expression) -> String {
        format!("printf(\"%d\\n\", {});", expr.resolve_operand())
    }

    fn print_i1(&self, expr: &Expression) -> String {
        format!("printf(\"%s\\n\", {} ? \"true\" : \"false\");", expr.resolve_operand())
    }

    fn print_str_constant(&self, expr: &Expression) -> String {
        format!("printf(\"%s\\n\", {});", expr.resolve_operand())
    }
}

impl DeclarationIR for CDeclarationIR {}