use crate::parser::expression::expr::{Accept, Binary, DataType, Expression, Operation, VariableExpression, Visitor};
use crate::parser::expression::expression::expression;
use crate::parser::parser::{AstNode, Parser};
use crate::{llvm::llvm_print::llvm_call_print_local, scanner::token::TokenType};
use crate::parser::declaration::declaration::{Declaration, PrintStatement};

pub struct PrintVisitor;
impl Visitor for PrintVisitor {
    fn visit_literal(&mut self, literal: &DataType) -> String{
        literal.print()
    }
    fn visit_binary(&mut self, binary: &Binary) -> String {
        binary.print()
    }
    fn visit_print(&mut self, print_statement: &PrintStatement) -> String {

        match print_statement.expression.as_datatype() {
            DataType::Integer(_) => {
                format!("\tcall void @print_i32(i32 {}); signature from PrintVisitor\n", print_statement.expression.resolve_operand())
            },
            DataType::Boolean(_) => {
                format!("\tcall void @print_i1(i1 {}); signature from PrintVisitor\n", print_statement.expression.resolve_operand())
            }
            _ => panic!()
        }
    }
    fn visit_variable_declaration(&mut self, variable_declaration: &super::declaration::VariableDeclaration) -> String {
        match variable_declaration.as_datatype() {
            DataType::Integer(_) => {
                format!("\t{}\n\t{} ; signature by visitor\n", variable_declaration.create_variable(), variable_declaration.store())
            },
            _ => panic!()
        }
        
    }
    fn visit_variable_expression(&mut self, variable_expression: &VariableExpression) -> String {
        match variable_expression.datatype {
            DataType::Integer(_) => {
                format!("\t%{}_{} = load i32, i32* %{}", variable_expression.name, variable_expression.count, variable_expression.name)
            },
            _ => panic!()
        }
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
        "".to_string()
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
        let (expr, top) = parser.expr_pop();
        match &expr.data_type {
            DataType::Boolean(_) => parser.emit_instruction(&LlvmCallPrint::Integer(top).print_i1()),
            DataType::Integer(_) => parser.emit_instruction(&LlvmCallPrint::Integer(top).print_i32()),
            DataType::String (_) => parser.emit_instruction(&LlvmCallPrint::String(top).call_print())
        }
        parser.expr_count += 1;
    }

    

    // =====================================================



    //======================================================
    parser.consume(TokenType::Semicolon, "Expect semicolon after value");
}

pub enum LlvmCallPrint {
    String(u32), //register value
    Integer(u32),
}
impl LlvmCallPrint {
    pub fn call_print(&self) -> String {
        match self {
            Self::String(register) => format!("\tcall i32 (i8*, ...) @printf(i8* %{})\t\t\t\t\t\t\t\t\t\t\t; Auto generated by LlvmCallPrint (print.rs)\n", register),
            Self::Integer(_) => panic!()
        }
    }
    pub fn print_i32(&self) -> String {
        match self {
            Self::Integer(register) => llvm_call_print_local(register.clone(), "i32"),
            _ => panic!("Not a i32")
        }
    }
    pub fn print_i1(&self) -> String {
        match self {
            Self::Integer(register) => llvm_call_print_local(register.clone(), "i1"),
            _ => panic!("Not a i32")
        }
    }
}