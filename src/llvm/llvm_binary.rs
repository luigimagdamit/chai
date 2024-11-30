use crate::{
    common::common::PARSE_CONSTANT_FOLD,
    parser::expr::{Expr, DataType}
};
pub fn llvm_binary_operands(value: i32, index: u32, type_tag: &str) -> Option<Expr>{
    if PARSE_CONSTANT_FOLD {
        Some(Expr {
        left: String::from(type_tag) + " " + &(value).to_string(),
        right: (value).to_string(),
        data_type: DataType::Integer(value) 
        })
    } else {
        match type_tag {
            "i32" => {
                Some(Expr {
                    left: String::from(type_tag) + " %" + &(index).to_string(),
                    right: String::from("%") + &(index).to_string(),
                    data_type: DataType::Integer(value) 
                })
            },
            "i1" => {
                let val_bool = if value == 1 { true }  else {false }; 
                Some(Expr {
                    left: String::from(type_tag) + " %" + &(index).to_string(),
                    right: String::from("%") + &(index).to_string(),
                    data_type: DataType::Boolean(val_bool) 
                })
            },
            _ => panic!("Unknown type for binary operand calculation")
        }
        
        
    }
}