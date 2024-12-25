use crate::parser::expression::expr::{DataType, Binary, StringConstant, Expression, Operation, VariableExpression};
use crate::parser::declaration::declaration::{VariableDeclaration, PrintStatement};
use crate::parser::visitor::visitor::Visitor;
use crate::codegen::llvm_codegen::LlvmPrint;
use crate::codegen::codegen_print::CodegenPrint;
pub struct PrintVisitor;
impl Visitor for PrintVisitor {
    fn visit_literal(&mut self, literal: &DataType) -> String{
        literal.print()
    }
    fn visit_binary(&mut self, binary: &Binary) -> String {
        binary.print()
    }
    fn visit_string(&mut self, str_constant: &StringConstant) -> String {
        str_constant.print()
    }
    fn visit_print(&mut self, print_statement: &PrintStatement) -> String {
        match &print_statement.expression {
            Expression::Binary(binary) => {
                match binary.operator {
                    Operation::Equal | Operation::GreaterEqual | Operation::GreaterThan |Operation::LessEqual |Operation::LessThan | Operation::NotEqual => {
                        LlvmPrint::print_i1(&Expression::from(binary))
                    },
                    _ => LlvmPrint::print_i32(&Expression::from(binary))
                }
            },
            Expression::Literal(literal) => {
                match literal {
                    DataType::Integer(_) => LlvmPrint::print_i32(&Expression::from(literal)),
                    DataType::Boolean(_) => LlvmPrint::print_i1(&Expression::from(literal)), 
                    _ => panic!()
                }
            },
            Expression::StringConstant(str_constant) => {
                str_constant.print()
            }
            Expression::Variable(variable) => {
                match variable.datatype {
                    DataType::Integer(_) => LlvmPrint::print_i32(&Expression::from(variable.clone())),
                    DataType::Boolean(_) => LlvmPrint::print_i1(&Expression::from(variable.clone())),
                    DataType::String(_) => LlvmPrint::print_str_constant(&Expression::from(variable.clone())),
                }
            },
            _ => panic!("Unrecognized print statement expression input")
        }
    }
    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) -> String {
        LlvmPrint::new_variable(variable_declaration) + &"\n\t" + &LlvmPrint::store_variable(variable_declaration)
        
    }
    fn visit_variable_expression(&mut self, variable_expression: &VariableExpression) -> String {
        LlvmPrint::var_expr(variable_expression)
    }
}