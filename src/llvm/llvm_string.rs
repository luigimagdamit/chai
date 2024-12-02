pub fn llvm_retrieve_static_string(str_length: usize, str_index: usize) -> String {
    let codegen = format!("getelementptr inbounds [{} x i8], [{} x i8]* @str{}, i32 0, i32 0\n", str_length , str_length, str_index);
    codegen
}
pub fn llvm_new_static_string(str_length: usize, str_index: usize, str_value: &str) -> String {
    format!("@str{} = private unnamed_addr constant [{} x i8] c\"{}\\0A\\00\", align 1", str_index, str_length , str_value)
}