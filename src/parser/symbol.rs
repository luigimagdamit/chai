use super::parser::{Parser, SymbolTableEntry};
use crate::parser::expression::expr::{DataType, Expr};


pub fn get_symbol(parser: &mut Parser, name: String) {
    let variable = parser.symbol_table.get(&name).unwrap();
    match variable.variable_type {
        DataType::Integer(_) => {
            let codegen = &LlvmLoad::load_i32(&variable.name, variable.count);
            parser.new_expr(Expr {
                left: format!("i32 %{}_{}", variable.name, variable.count),
                right: format!("%{}_{}", variable.name, variable.count),
                data_type: variable.variable_type.clone()
            });
            parser.emit_instruction(codegen);
        },
        DataType::String(_) => {
            let codegen = &LlvmLoad::load_string(&variable.name, variable.count);
            parser.new_expr(Expr {
                left: format!("%{}_{}", variable.name, variable.count),
                right: String::from("<__var_string__>"),
                data_type: variable.variable_type.clone()
            });
            parser.emit_instruction(&codegen);
        }
        _ => ()
    }
    parser.symbol_table.get_mut(&name).unwrap().count += 1;
    
}
// be for setting it after initial assignment
pub fn set_symbol(parser: &mut Parser, name: String, new_value: Expr) {
    let variable = parser.symbol_table.get(&name).clone().unwrap();
    
    let a_type = &variable.variable_type;
    let b_type = &new_value.data_type;

    if types_equal(&variable.variable_type, &new_value.data_type) {
        let error_msg = format!("Incompatible variable assignment types - Failed to assign variable {}'s value to an item of type {}", a_type, b_type);
        parser.error_at(&parser.current.unwrap(), &error_msg)
    }
    match &variable.variable_type {
        DataType::Integer(_) => parser.emit_instruction(&format!("\tstore {}, i32* %{}\t\t ; set symbol (symbol.rs)\n", new_value.left , name)),
        DataType::String(_) => panic!("set_symbol() not impl for strings"),
        _ => panic!("set symbol not added for this data type")
    }
}

pub fn types_equal(a: &DataType, b: &DataType) -> bool {
    std::mem::discriminant(a) != std::mem::discriminant(b)
}
pub fn create_new_symbol(parser: &mut Parser, name: String, variable_type: DataType) {
    parser.symbol_table.insert(name.clone(), SymbolTableEntry {
        name: name.clone(),
        count: 0,
        variable_type: variable_type
    });
}
pub enum LlvmLoad {
    Integer(u32),
}
impl LlvmLoad {
    pub fn load_i32(var_name: &str, var_count: usize) -> String {
        format!("\t%{var_name}_{var_count} = load i32, i32* %{var_name} \t\t\t ; LlvmLoad load_i32")
    } 
    pub fn load_string(var_name: &str, var_count: usize) -> String {
        format!("\t%{}_{} = load {}, {}* %{} ; \t\t\t Llvm Load String", var_name, var_count, "i8*", "i8*", var_name)
    }
}