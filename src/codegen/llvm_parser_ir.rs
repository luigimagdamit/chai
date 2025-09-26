use super::parser_ir::ParserIR;
use crate::llvm::llvm_print::{llvm_fmt_string_int, llvm_main_close, llvm_print_bool_declare, llvm_print_define, llvm_print_i32_define};

/// LLVM-specific implementation for parser IR generation
#[derive(Clone, Debug)]
pub struct LlvmParserIR;

impl ParserIR for LlvmParserIR {
    fn generate_stdlib(&self) -> String {
        use crate::common::flags::{PARSE_SUPRESS_PREDEFINES, EMIT_VERBOSE};

        if !PARSE_SUPRESS_PREDEFINES && EMIT_VERBOSE {
            llvm_print_define();
            llvm_print_bool_declare();
            llvm_fmt_string_int();
            llvm_print_i32_define();
        }
        String::new()
    }

    fn generate_main_close(&self) -> String {
        llvm_main_close()
    }
}