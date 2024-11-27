use crate::parser::expr::DataType;
use crate::common::common::{LLVM_DEBUG_OUTPUT, NO_MAIN};
#[warn(unused_variables)]


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