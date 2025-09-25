use super::super::parser::{Parser, StringEntry};
use super::super::expression::expr::{DataType, Expr};
use crate::llvm::llvm_string::*;
use crate::parser::core::ast_node::AstNode;
use crate::parser::expression::expr::{Expression, ParseError, StringConstant};

pub fn parse_string(parser: &mut Parser) -> Result<Expression, ParseError> {
    let value = parser.previous.unwrap().start;
    let length = value.len() ;

    
    let codegen = llvm_new_static_string(length, parser.string_table.len(), &value[1..length - 1]);
    
    match parser.string_table.get(value) {
        Some(existing_str) => {
            let retrieve_codegen = string_expr(length - 1, existing_str.index, value, parser.expr_count);
            let str_constant=  Expression::from(
                StringConstant {
                    name: value.to_string(),
                    length: length - 1,
                    index: existing_str.index,
                    register: parser.expr_count as usize,
                    datatype: DataType::String(format!("{}", parser.expr_count as usize))
                }
            );
            parser.ast_stack.push(AstNode::from_expression(str_constant.clone()));
            parser.emit_instruction(&str_constant.as_str_constant().place());
            parser.new_expr(retrieve_codegen.clone());
            parser.string_table.get_mut(value).expect("Tried to get a value from the string table, but it could not be found").index += 1;
            parser.expr_count += 1;
        },
        None => {
            parser.string_table.insert(
                String::from(value), 
                StringEntry {
                    codegen: codegen,
                    length: length - 1,
                    index: parser.string_table.len()
                }
            ); 

            let new_index = parser.string_table.len() - 1;
            let new_str_codegen = string_expr(
                length, 
                new_index, 
                value, 
                parser.expr_count
            );
            parser.comment(&("\t; pushing a new string on the stack ...".to_string() + value));
            let str_constant = Expression::from(
                StringConstant {
                    name: value.to_string(),
                    length: length,
                    index: new_index,
                    register: parser.expr_count as usize,
                    datatype: DataType::String(format!("{}", parser.expr_count as usize))
                });

            parser.emit_instruction(&str_constant.as_str_constant().place());
            parser.ast_stack.push(AstNode::from_expression(str_constant));
            parser.new_expr(new_str_codegen);
            parser.expr_count += 1;
            
        }
    }
    let res = Expression::from_literal(DataType::String("".to_string()));
    Ok(res)
}
fn string_expr(str_length: usize, str_index: usize, str_value: &str, _register: u32) -> Expr {
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