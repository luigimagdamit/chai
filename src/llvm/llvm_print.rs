use crate::common::common::NO_MAIN;

pub fn llvm_main_start() -> String {
    let codegen = "define i32 @main() {\n    entry:\n";
    println!("{}", codegen);
    return String::from(codegen)
}
pub fn llvm_main_close() -> String {
    let codegen = 
    "    ret i32 0\n}";
    println!("{}", codegen);
    return String::from(codegen)
}
pub fn llvm_print_define() -> String {
    let codegen = "declare i32 @printf(i8*, ...)";
    if NO_MAIN { println!("{}", codegen) }
    return String::from(codegen)
}
pub fn llvm_print_i32_define() -> String {
    let codegen =

"\ndefine void @print_i32(i32 %value) {
    %fmt_ptr = getelementptr [4 x i8], [4 x i8]* @fmt, i32 0, i32 0
    call i32 (i8*, ...) @printf(i8* %fmt_ptr, i32 %value)
    ret void
}";
    println!("{}", codegen);
    return String::from(codegen)
}
pub fn llvm_call_print(index: u32, data_type: &str) {
    let codegen = format!(
        "    %result = call {} @{}()
    call void @print_{}({} %result)
", data_type, index, data_type, data_type);
    println!("{}", codegen);
}
pub fn llvm_call_print_local(index: u32, data_type: &str) {
    let codegen = format!(
        "call void @print_{}({} %{})
", data_type, data_type, index);
    println!("{}", codegen);
}
pub fn llvm_fmt_string_int() {
    println!("@fmt = private constant [4 x i8] c\"%d\\0A\\00\"")
}
pub fn llvm_print_no_main(index: u32) {
    let codegen = format!(
    "define i32 @main() {{\nentry:
        %result = call i32 @{}()
        call void @print_i32(i32 %result)
        ret i32 0
    }}", index);
    println!("{}", codegen)
}