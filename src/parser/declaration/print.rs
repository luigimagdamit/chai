use crate::parser::expression::expr::{Accept, Binary, DataType, Expression, Operation, VariableExpression, Visitor};
use crate::parser::expression::expression::expression;
use crate::parser::parser::Parser;
use crate::scanner::token::TokenType;
use crate::parser::declaration::declaration::PrintStatement;

use super::declaration::VariableDeclaration;

pub trait CodegenPrint {
    fn print_i1(expr: &Expression) -> String;
    fn print_i32(expr: &Expression) -> String;
    fn new_variable(dec: &VariableDeclaration) -> String;
    fn store_variable(dec: &VariableDeclaration) -> String;
    fn var_expr(expr: &VariableExpression) -> String;
}
pub struct LlvmPrint;
impl CodegenPrint for LlvmPrint {
    fn print_i1(expr: &Expression) -> String {
        format!("\tcall void @print_i1(i1 {}); signature from PrintVisitor\n", Expression::from(expr.clone()).resolve_operand())
    }
    fn print_i32(expr: &Expression) -> String {
        format!("\tcall void @print_i32(i32 {}); signature from PrintVisitor\n", Expression::from(expr.clone()).resolve_operand())
    }
    fn new_variable(dec: &VariableDeclaration) -> String {
        format!("\t%{} = alloca i32", dec.name)
    }
    fn store_variable(dec: &VariableDeclaration) -> String {
        if let Some(expr) = &dec.expression {
            match expr.as_datatype() {
                DataType::Integer(_) => format!("store i32 {}, i32* %{}", expr.resolve_operand(), dec.name),
                DataType::Boolean(_) => format!("store i1 {}, i1* %{}", expr.resolve_operand(), dec.name),
                _ => panic!("Strings not supported for storing variables")
            }
            
        } else {
            "".to_string()
        }
        
        
    }
    fn var_expr(expr: &VariableExpression) -> String {
        match expr.datatype {
            DataType::Integer(_) => format!("\n\t%{}_{} = load i32, i32* %{}", expr.name, expr.count, expr.name),
            DataType::Boolean(_) => format!("\n\t%{}_{} = load i1, i1* %{}", expr.name, expr.count, expr.name),
            _ => panic!("not supported for strings: variable expressions")
        }
        
    }
}
pub struct PrintVisitor;
impl Visitor for PrintVisitor {
    fn visit_literal(&mut self, literal: &DataType) -> String{
        literal.print()
    }
    fn visit_binary(&mut self, binary: &Binary) -> String {
        binary.print()
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
            Expression::Variable(variable) => {
                match variable.datatype {
                    DataType::Integer(_) => LlvmPrint::print_i32(&Expression::from(variable.clone())),
                    DataType::Boolean(_) => LlvmPrint::print_i1(&Expression::from(variable.clone())),
                    _ => panic!()
                }
            },
            _ => panic!("Unrecognized print statement expression input")
        }
    }
    fn visit_variable_declaration(&mut self, variable_declaration: &super::declaration::VariableDeclaration) -> String {
        LlvmPrint::new_variable(variable_declaration) + &LlvmPrint::store_variable(variable_declaration)
        
    }
    fn visit_variable_expression(&mut self, variable_expression: &VariableExpression) -> String {
        LlvmPrint::var_expr(variable_expression)
    }
}

pub struct RebuildVisitor;
impl Visitor for RebuildVisitor {
    fn visit_literal(&mut self, literal: &DataType) -> String {
        literal.to_string()
    }
    fn visit_binary(&mut self, binary: &Binary) -> String {
        let left = binary.get_left().accept(self);

        let operator = match binary.operator {
            Operation::Add => "+",
            Operation::Sub => "-",
            Operation::Mul => "*",
            Operation::Div => "/",
            Operation::Equal => "==",
            Operation::NotEqual => "!=",
            Operation::GreaterEqual => ">=",
            Operation::GreaterThan => ">",
            Operation::LessEqual => "<=",
            Operation::LessThan => "<"

        };
        let right = binary.get_right().accept(self);
        format!("({} {} {})", left, operator, right)
    }
    fn visit_print(&mut self, print_statement: &PrintStatement) -> String {
        format!("print {}", print_statement.expression.accept(self))
    }
    fn visit_variable_declaration(&mut self, variable_declaration: &super::declaration::VariableDeclaration) -> String {
        match variable_declaration.as_datatype() {
            DataType::Integer(_) => {
                format!("var {} : int", variable_declaration.name)
            },
            _ => panic!()
        }
        
    }
    fn visit_variable_expression(&mut self, variable_expression: &VariableExpression) -> String {
        format!("{}", variable_expression.name)
    }
}
pub fn print_statement(parser: &mut Parser) {
    expression(parser);
    let ast_mode = true;
    let mut visitor = PrintVisitor;
    let mut rebuild = RebuildVisitor;

    let mut print_statement = PrintStatement{ expression: Expression::Empty };
    // =====================================================
    if ast_mode {
        parser.comment("\tast mode");
        let expr_ast = parser.ast_stack.pop();

        if let Some(ast_node) = expr_ast { 
            match ast_node.to_expression() {
                Expression::Binary(b) => {
                    let expr = Expression::from(b);
                    parser.comment(&format!("\t; {};", expr.clone().accept(&mut rebuild)));
                    print_statement.expression = expr;
                    parser.emit_instruction(&visitor.visit_print(&print_statement));
                },
                Expression::Literal(l) => {
                    let expr = Expression::from(l);
                    parser.comment(&format!("\t; {};", expr.clone().accept(&mut rebuild)));
                    print_statement.expression = expr;

                    parser.emit_instruction(&visitor.visit_print(&print_statement));
                }
                Expression::Variable(variable) => {
                    let expr = Expression::from(variable);
                    parser.comment(&format!("\t; {};", expr.clone().accept(&mut rebuild)));
                    print_statement.expression = expr;

                    parser.emit_instruction(&visitor.visit_print(&print_statement));
                }
                _ => ()
            }
    
        }
    } else {
    }
    parser.consume(TokenType::Semicolon, "Expect semicolon after value");
}

