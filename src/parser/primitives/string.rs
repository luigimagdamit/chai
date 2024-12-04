use super::super::parser::{Parser, StringEntry};
use super::super::expression::expr::{DataType, Expr};
use crate::llvm::llvm_string::*;

pub fn parse_string(parser: &mut Parser) {
    let value = parser.previous.unwrap().start;
    let length = value.len() ;

    
    let codegen = llvm_new_static_string(length, parser.string_table.len(), &value[1..length - 1]);
    
    match parser.string_table.get(value) {
        Some(str) => {
            //let register_codegen = format!("%{} = ", parser.expr_count);
            let retrieve_codegen = string_expr(length - 1, str.index, value, parser.expr_count);

            parser.new_expr(retrieve_codegen.clone());
            //parser.emit_instruction(&retrieve_codegen.left);
            parser.string_table.get_mut(value).unwrap().index += 1;
        },
        None => {
            parser.string_table.insert(String::from(value), StringEntry {
                codegen: codegen,
                length: length - 1,
                index: parser.string_table.len()
            }); 

            let new_index = parser.string_table.len() - 1;
            //let register_codegen = format!("%{} = ", parser.expr_count);
            let new_str_codegen = string_expr(length, new_index, value, parser.expr_count);

            // parser.emit_instruction(&new_str_codegen.clone().left);
            parser.emit_instruction(&"\t; pushing a new string on the stack ...".to_string());
            parser.new_expr(new_str_codegen);
        }
    }
}
fn string_expr(str_length: usize, str_index: usize, str_value: &str, register: u32) -> Expr {
    let codegen = llvm_retrieve_static_string(str_length, str_index);

    Expr {
        left: String::from(&codegen),
        right: String::from(&codegen),
        data_type: str_data_type(str_value)
    }
}

fn str_data_type(value: &str) -> DataType {
    DataType::String(String::from(value))
}