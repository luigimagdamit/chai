use crate::expr::DataType;
use crate::common::{LLVM_DEBUG_OUTPUT, NO_MAIN};
#[warn(unused_variables)]

pub fn llvm_print_declare() -> String {
    let codegen = "declare i32 @printf(i8*, ...)";
    if NO_MAIN { println!("{}", codegen) }
    return String::from(codegen)
}
pub fn llvm_fmt_string_int() {
    println!("@fmt = private constant [4 x i8] c\"%d\\0A\\00\"")
}
pub fn llvm_print_no_main(index: u32) {
    let codegen = format!(
"define i32 @main() {{\nentry:
    %result = call i32 @{}()

    %fmt_ptr = getelementptr [4 x i8], [4 x i8]* @fmt, i32 0, i32 0
    call i32 @printf(i8* %fmt_ptr, i32 %result)
    ret i32 0
}}", index);
    println!("{}", codegen)
}
pub fn llvm_top_level_expr(_value: &str, value_type: &DataType, index: u32) -> String{
    if LLVM_DEBUG_OUTPUT {println!("Read top-level expression:");}
    
    match value_type {
        DataType::Integer(int) => {
            if NO_MAIN {
                let codegen = format!("define i32 @main() {{\nentry:\n    ret i32 {}\n}}", int);
                println!("{}", codegen);
                return codegen
            } else {
                let codegen = format!("define i32 @{}() {{\nentry:\n    ret i32 {}\n}}", index, int);
                println!("{}", codegen);
                return codegen;
            }
        }
    }
    
}