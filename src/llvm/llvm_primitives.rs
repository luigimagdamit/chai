use crate::parser::expression::expr::DataType;
use super::llvm_string::llvm_new_static_string;
#[warn(unused_variables)]


pub fn llvm_top_level_expr(_value: &str, value_type: &DataType, index: u32) -> String{
    // if PARSE_EXPRESSION_MODE {println!("\nRead top-level expression:");}
    
    match value_type {
        DataType::Integer(int) => {
            let codegen = format!("define i32 @{}() {{\nentry:\n    ret i32 {}\n}}", index, int.unwrap());
            println!("{}", codegen);
            codegen
        },
        DataType::Boolean(bool) => if *bool { llvm_top_level_boolean("1", true, index) } else { llvm_top_level_boolean("0", false, index) }
        DataType::String(str) =>  {
            let codegen = llvm_new_static_string(str.len(), index as usize, &str[1..str.len() - 1]);
            println!("{}", codegen);
            codegen
        }
    }
    
}
fn llvm_top_level_boolean(chars: &str, _value: bool, index: u32) -> String{
    let codegen = 
    format!(
"define i1 @{}() {{
    entry:
    ret i1 {}
}}", index, chars);
    println!("{}", codegen);
    codegen
}