use super::{declaration::print::PrintVisitor, expression::expr::{Accept, VariableExpression}, parser::{AstNode, Parser, SymbolTableEntry}};
use crate::parser::expression::expr::{DataType, Expr, Expression};

pub enum LlvmGetVariable {
    Integer((String, usize)) // name of string
}
impl LlvmGetVariable {
    pub fn create_tags(&self) -> (String, String) {
        match self {
            Self::Integer(name) => (format!("i32 %{}_{}", name.0, name.1), format!("%{}_{}", name.0, name.1))
        }
    }
}

pub fn get_symbol(parser: &mut Parser, name: String) {

    if let Some(variable) = parser.symbol_table.get(&name) {
        let SymbolTableEntry { name: _, count, variable_type } = variable;
        let variable_expression = VariableExpression {
            name: name.clone(),
            datatype: variable_type.clone(),
            count: count.clone()
        };

        let mut visitor = PrintVisitor;
        let codegen = Expression::from(variable_expression.clone()).accept(&mut visitor);
        println!("{codegen}");
        parser.emit_instruction(&codegen);

        parser.ast_stack.push(AstNode::Expression(Expression::from(variable_expression).clone()));
        parser.symbol_table.get_mut(&name).unwrap().count += 1;
    } else {
        parser.error_at_previous("Variable was not declared -");
    }

    
}
// be for setting it after initial assignment
pub fn set_symbol(parser: &mut Parser, name: String, new_value: Expr) {

    if let Some(variable) = parser.symbol_table.get(&name).clone() {
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
    } else {

        parser.error_at_previous(&format!("Variable was not declared when setting {name} to "));
    }

}

pub fn types_equal(a: &DataType, b: &DataType) -> bool {
    std::mem::discriminant(a) != std::mem::discriminant(b)
}
pub fn create_new_symbol(parser: &mut Parser, name: String, variable_type: DataType) {
    parser.symbol_table.insert(name.clone(), SymbolTableEntry {
        name,
        count: 0,
        variable_type
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