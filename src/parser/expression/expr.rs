use std::fmt;
use crate::llvm::llvm_print::llvm_call_print_local;
use super::binary::is_boolean_op;
#[derive(Clone, PartialEq, Eq)]
#[allow(unused)]

// DataType is a literal
pub enum DataType {
    Integer(i32),
    String(String),
    Boolean(bool)
}
impl DataType {
    pub fn llvm_print(&self, expr_count: u32) {
        match self {
            DataType::Integer(int) => {
                println!("%{expr_count} = add i32 {int}, 0");
                let cg = format!("call i32 (i8*, ...) @printf(i8* %{})\t\t\t\t\t\t\t\t\t\t\t; Auto generated by LlvmCallPrint (print.rs)\n", expr_count);
                println!("{cg}")
            },
            DataType::Boolean(bool) => {
                let bool_val = if *bool {1} else {0};
                println!("%{expr_count} = add i1 {bool_val}, 0");
                let cg = format!("call i1 (i8*, ...) @printf(i8* %{})\t\t\t\t\t\t\t\t\t\t\t; Auto generated by LlvmCallPrint (print.rs)\n", expr_count);
                println!("{cg}")
            }
            _ => ()
        }
    }
}
#[derive(Clone)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterEqual,
    LessThan,
    LessEqual
}
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Add => write!(f, "add"),
            Operation::Sub => write!(f, "sub"),
            Operation::Mul => write!(f, "mul"),
            Operation::Div => write!(f, "div"),
            Operation::Equal => write!(f, "icmp eq"),
            Operation::NotEqual => write!(f, "N.EQ"),
            Operation::GreaterEqual => write!(f, "GR.EQ"),
            Operation::GreaterThan => write!(f, "GR.TH"),
            Operation::LessEqual => write!(f, "LE.EQ"),
            Operation::LessThan => write!(f, "LE.TH")
        }
    }
}
#[derive(Clone)]
pub struct Binary {
    left: Box<Expression>,
    right: Box<Expression>,
    operator: Operation,
    register: String

}
impl Binary {
    pub fn new(left: Expression, right: Expression, operator: Operation, register: &str) -> Binary {
        Binary {left: Box::new(left), right: Box::new(right), operator, register: register.to_string()}
    }
    pub fn llvm_print(&self) {
        if is_boolean_op(self.operator.clone()) {
            println!("{}", llvm_call_print_local(self.register.clone().parse().unwrap(), "i1"))
        } else {
            let cg = format!("call i32 (i8*, ...) @printf(i8* %{})\t\t\t\t\t\t\t\t\t\t\t; Auto generated by LlvmCallPrint (print.rs)\n", self.register);
            println!("{cg}")
        }
        
    }
}
pub fn convert_bool(b: bool) -> u32 {
    if b {1} else {0}
}
pub fn resolve_operand(expression: Expression) -> String {
    match expression {
        Expression::Binary(b) => format!("%{}", b.register),
        Expression::Literal(i) => {
            match i {
                DataType::Integer(int) => int.to_string(),
                DataType::Boolean(bool) => convert_bool(bool).to_string(),
                _ => "".to_string()
            }
        }
        _ => "".to_string()
    }
}
pub fn resolve_binary(binary: &Binary) -> String {
    let tag = if is_boolean_op(binary.clone().operator) {"i1" } else {"i32"};
    let mut codegen = format!("{} {tag} ", binary.operator);
    codegen += &resolve_operand(*binary.clone().left);
    codegen += &(", ".to_string() + &resolve_operand(*binary.clone().right));
    return codegen

}
impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} {} {}>", self.left, self.right, self.operator)
    }
}
#[derive(Clone)]
pub enum Expression {
    Literal(DataType),
    Variable,
    Binary(Binary),
    Empty
}

impl Expression {
    pub fn new_binary(left: Expression, right: Expression, operator: Operation, register: &str) -> Expression {
        Expression::Binary(Binary::new(left, right, operator, register))
    }
    pub fn get_register(&self) -> String {
        match self {
            Expression::Binary(b) => b.register.clone(),
            _ => "".to_string()
        }
    }
    pub fn as_binary(&self) -> Binary {
        match self {
            Expression::Binary(b) => b.clone(),
            _ => panic!()
        }
    }
    pub fn resolve_binary(&self) -> String {
        match self {
            Expression::Binary(b) => {
                format!("%{} = ", b.register) + &resolve_binary(&b)
            },
            _ => panic!()
        }
        
    }
    pub fn new_literal(literal: DataType) -> Expression{
        Expression::Literal(literal)
    }
    pub fn unwrap_literal(&self) -> &DataType {
        match self {
            Expression::Literal(datatype) => datatype,
            _ => panic!()
        }
    }
}
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Binary(b) => write!(f, "\n{b}"),
            Expression::Literal(l) => write!(f, "{l}"),
            _ => write!(f, "")
        }
    }
}
pub enum ParseError {
    Generic
}
impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Integer(int) => write!(f, "int: <{}>", int),
            DataType::Boolean(bool) => write!(f, "bool <{}>", bool),
            DataType::String(str) => write!(f, "str:<{}>", str)
        }
    }
}
// enum Operator {
//     Add
// }
#[derive(Clone)]
pub struct Expr {
    pub left: String,
    pub right: String,
    pub data_type: DataType
}
#[allow(unused)]
impl Expr {
    pub fn print_leaf(&self) {
        println!("<leaf> <left: {}> <right: {}> <data_type: {}>", self.left, self.right, self.data_type);
    }
}