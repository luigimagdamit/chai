use super::super::parser::Parser;
use super::super:: expression::expr::{DataType, Expr};
pub fn parse_number(parser: &mut Parser) {
    let value = String::from(parser.previous.unwrap().start);
    let number_leaf = Expr {
        left: LlvmNumberTag::Integer(value.clone()).left(),
        right: LlvmNumberTag::Integer(value.clone()).right(),
        data_type: DataType::Integer(value.parse().unwrap())
    };

    parser.constant_stack.push(Some(number_leaf));
}

pub enum LlvmNumberTag {
    Integer(String)
}
impl LlvmNumberTag {
    pub fn left(&self) -> String {
        match self {
            LlvmNumberTag::Integer(int) => format!("i32 {}", int).to_string()
        }
        
    }
    pub fn right(&self) -> String {
        match self {
            LlvmNumberTag::Integer(int) => int.to_string()
        }
        
    }
}