use super::parser::{Parser, StringEntry};
use super::expr::{DataType, Expr};
use crate::llvm::llvm_string::*;

pub fn parse_string(parser: &mut Parser) {
    let value = parser.previous.unwrap().start;
    let length = value.len() ;

    
    let codegen = llvm_new_static_string(length, parser.string_table.len(), &value[1..length - 1]);

    match parser.string_table.get(value) {
        Some(str) => {
            parser.new_expr(string_expr(length - 1, str.index, value));
        },
        None => {
            parser.string_table.insert(String::from(value), StringEntry {
                codegen: codegen,
                length: length - 1,
                index: parser.expr_count as usize
            }); 

            let new_index = parser.string_table.len() - 1;
            parser.new_expr(string_expr(length, new_index, value));
        }
    }
}
fn string_expr(str_length: usize, str_index: usize, str_value: &str) -> Expr {
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