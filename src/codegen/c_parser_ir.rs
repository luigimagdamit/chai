use super::parser_ir::ParserIR;

/// C-specific implementation for parser IR generation
#[derive(Clone, Debug)]
pub struct CParserIR;

impl ParserIR for CParserIR {
    fn generate_stdlib(&self) -> String {
        // C doesn't need runtime stdlib generation like LLVM
        String::new()
    }

    fn generate_main_close(&self) -> String {
        String::new() // C main function closing is handled by the wrapper
    }

    fn generate_preamble(&self) -> String {
        // Standard C includes
        "#include <stdio.h>\n#include <stdbool.h>\n\nint main() {\n".to_string()
    }
}