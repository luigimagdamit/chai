/// Traits for parser-level IR generation across different backends

pub trait ParserIR {
    /// Generate standard library/runtime definitions
    fn generate_stdlib(&self) -> String;

    /// Generate main function closing/epilogue
    fn generate_main_close(&self) -> String;

    /// Generate program preamble (if needed)
    fn generate_preamble(&self) -> String {
        String::new()
    }
}