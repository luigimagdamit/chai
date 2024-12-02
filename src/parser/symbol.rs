use crate::common::flags::PARSE_DECLARATION_MODE;

use super::parser::{Parser, SymbolTableEntry};
use crate::parser::expression::expr::{DataType, Expr};

pub fn create_new_symbol(parser: &mut Parser, name: String, variable_type: DataType) {
    parser.symbol_table.insert(name.clone(), SymbolTableEntry {
        name: name.clone(),
        count: 0,
        variable_type: variable_type
    });
}
pub fn get_symbol(parser: &mut Parser, name: String) {
    let variable = parser.symbol_table.get(&name).unwrap();
    
    match variable.variable_type {
        DataType::Integer(_) => {
            println!("%{}_{} = load {}, {}* %{}", variable.name, variable.count, "i32", "i32", variable.name);
            parser.new_expr(Expr {
                left: format!("i32 %{}_{}", variable.name, variable.count),
                right: format!("%{}_{}", variable.name, variable.count),
                data_type: variable.variable_type.clone()
            });
            // decrement since we don't use a name / tmp variable register name
            
        },
        DataType::String(_) => {
            let codegen = format!("\n%{}_{} = load {}, {}* %{}\n", variable.name, variable.count, "i8*", "i8*", variable.name);
            if PARSE_DECLARATION_MODE { println!("{}", codegen) }

            
            parser.new_expr(Expr {
                left: format!("%{}_{}", variable.name, variable.count),
                right: String::from("<__var_string__>"),
                data_type: variable.variable_type.clone()
            });
            // decrement since we don't use a name / tmp variable register name
            parser.expr_count -= 1;
            parser.emit_instruction(&codegen);
        }
        _ => ()
    }
    parser.symbol_table.get_mut(&name).unwrap().count += 1;
    
}
// be for setting it after initial assignment
pub fn set_symbol(parser: &mut Parser, name: String, value: Expr) {
    let variable = parser.symbol_table.get(&name).unwrap();

    if let DataType::Integer(_) = variable.variable_type {
        println!("store {}, i32* %{}\n", value.left , name);
    }
    if std::mem::discriminant(&variable.variable_type) != std::mem::discriminant(&value.data_type) {
        parser.error_at(&parser.current.unwrap(), "incompatibe variable assignment types")
    }
    // add more
}