use super::super::parser::{Parser, StringEntry};
use super::super::expression::expr::{DataType, Expr};
use crate::llvm::llvm_string::*;
use crate::parser::core::ast_node::AstNode;
use crate::parser::expression::expr::{Expression, ParseError, Register, StringConstant};

pub fn parse_string(parser: &mut Parser) -> Result<Expression, ParseError> {
    let value = parser.previous.unwrap().start;
    let length = value.len() ;

    
    let codegen = llvm_new_static_string(length, parser.string_table.len(), &value[1..length - 1]);
    
    match parser.string_table.get(value) {
        Some(str) => {
            let retrieve_codegen = string_expr(length - 1, str.index, value, parser.expr_count);
            
            parser.ast_stack.push(AstNode::from_expression(
                Expression::from(StringConstant{
                    name: value.to_string(),
                    length: length - 1,
                    count: 0,
                    index: str.index,
                    text: value.to_string(),
                    register: parser.expr_count as usize
                })
            ));
            parser.new_expr(retrieve_codegen.clone());
            parser.string_table.get_mut(value).unwrap().index += 1;
            parser.expr_count += 1;
        },
        None => {
            parser.string_table.insert(String::from(value), StringEntry {
                codegen: codegen,
                length: length - 1,
                index: parser.string_table.len()
            }); 

            let new_index = parser.string_table.len() - 1;
            let new_str_codegen = string_expr(length, new_index, value, parser.expr_count);
            parser.comment(&("\t; pushing a new string on the stack ...".to_string() + value));
            let str_constant = Expression::from(StringConstant{
                name: value.to_string(),
                length: length,
                count: 0,
                index: new_index,
                text: value.to_string(),
                register: parser.expr_count as usize
            });

            parser.emit_instruction(&str_constant.as_str_constant().place());
            parser.ast_stack.push(AstNode::from_expression(str_constant));
            parser.new_expr(new_str_codegen);
            parser.expr_count += 1;
            
        }
    }
    Ok(Expression::Literal(DataType::String("()".to_string())))
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