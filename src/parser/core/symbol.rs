use crate::parser::parser::Parser;
use crate::parser::expression::expr::VariableExpression;
use crate::parser::visitor::print_visitor::PrintVisitor;
use crate::parser::expression::expr::{DataType, Expression};
use crate::parser::core::ast_node::AstNode;
use crate::parser::visitor::visitor::Accept;
pub struct SymbolTableEntry {
    pub count: usize,
    pub variable_type: DataType
}
pub fn get_symbol(parser: &mut Parser, name: &str) {

    if let Some(variable) = parser.symbol_table.get(&name.to_string()) {
        let SymbolTableEntry { count, variable_type } = variable;
        let variable_expression = VariableExpression {
            name: name.to_string(),
            datatype: variable_type.clone(),
            count: count.clone()
        };

        let mut visitor = PrintVisitor;
        let codegen = Expression::from(variable_expression.clone()).accept(&mut visitor);
        println!("{codegen}");
        parser.emit_instruction(&codegen);

        parser.ast_stack.push(AstNode::Expression(Expression::from(variable_expression).clone()));
        parser.symbol_table.get_mut(&name.to_string()).expect("Tried incrementing count, but could not find symbol in table").count += 1;
    } else {
        parser.error_at_previous("Variable was not declared -");
    }

    
}
// be for setting it after initial assignment
pub fn set_symbol(parser: &mut Parser, name: &str, new_value: Expression) {

    if let Some(variable) = parser.symbol_table.get(&name.to_string()) {
        let a_type = &variable.variable_type;
        let b_type = &new_value.as_datatype();
    
        if types_equal(a_type, b_type) {
            let error_msg = format!("Incompatible variable assignment types - Failed to assign variable {}'s value to an item of type {}", a_type, b_type);
            parser.error_at(&parser.current.unwrap(), &error_msg)
        }
        match &variable.variable_type {
            // need to offset this to the llvm_codegen
            DataType::Integer(_) => parser.emit_instruction(
                &format!("store i32 {}, i32* %{}\n\t ; set symbol (symbol.rs)\n", 
                    new_value.resolve_operand(), 
                    name
                )),
            DataType::Boolean(_) => parser.emit_instruction(&format!("store i1 {}, i1* %{}\n\t ; set symbol (symbol.rs)\n", new_value.resolve_operand() , name)),
            DataType::String(_) => panic!("set_symbol() not impl for strings"),
        }
    } else {

        parser.error_at_previous(&format!("Variable was not declared when setting {name} to "));
    }

}

pub fn types_equal(a: &DataType, b: &DataType) -> bool {
    std::mem::discriminant(a) != std::mem::discriminant(b)
}
pub fn create_new_symbol(parser: &mut Parser, name: &str, variable_type: DataType) {
    parser.symbol_table.insert(name.to_string(), SymbolTableEntry {
        count: 0,
        variable_type
    });
}
