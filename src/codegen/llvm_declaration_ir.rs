use super::declaration_ir::{VariableIR, PrintIR, DeclarationIR};
use crate::parser::expression::expr::Expression;
use crate::parser::declaration::declaration::VariableDeclaration;
use crate::codegen::llvm_codegen::LlvmPrint;
use crate::codegen::codegen_print::CodegenPrint;

/// LLVM-specific implementation for declaration IR generation
#[derive(Clone, Debug)]
pub struct LlvmDeclarationIR;

impl VariableIR for LlvmDeclarationIR {
    fn new_variable(&self, var_decl: &VariableDeclaration) -> String {
        LlvmPrint::new_variable(var_decl)
    }

    fn store_variable(&self, var_decl: &VariableDeclaration) -> String {
        LlvmPrint::store_variable(var_decl)
    }
}

impl PrintIR for LlvmDeclarationIR {
    fn print_i32(&self, expr: &Expression) -> String {
        LlvmPrint::print_i32(expr)
    }

    fn print_i1(&self, expr: &Expression) -> String {
        LlvmPrint::print_i1(expr)
    }

    fn print_str_constant(&self, expr: &Expression) -> String {
        LlvmPrint::print_str_constant(expr)
    }
}

impl DeclarationIR for LlvmDeclarationIR {}