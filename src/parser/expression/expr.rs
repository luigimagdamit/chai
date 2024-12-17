use std::{clone, fmt};
use crate::{llvm::llvm_print::llvm_call_print_local, parser::declaration::declaration::{PrintStatement, VariableDeclaration}};
use super::binary::is_boolean_op;

#[allow(unused)]

// DataType is a literal
#[derive(Clone, PartialEq, Eq)]
pub enum DataType {
    Integer(i32),
    String(String),
    Boolean(bool)
}
// Turn an i32 integer into a DataType::Integer
impl From<i32> for DataType {
    fn from(item: i32) -> DataType {
        DataType::Integer(item)
    } 
}
impl From<String> for DataType {
    fn from(item: String) -> DataType {
        DataType::String(item)
    }
}
impl From<bool> for DataType {
    fn from(item: bool) -> DataType {
        DataType::Boolean(item)
    }
}

impl DataType {
    pub fn print(&self) -> String {
        match self {
            DataType::Integer(int) => {
                let cg = format!("add i32 {int}, 0");
                cg
            },
            DataType::Boolean(bool) => {
                let bool_val = if *bool {1} else {0};
                let mut cg = format!("add i1 {bool_val}, 0");
                cg
            }
            _ => "".to_string()
        }
    }
    pub fn type_tag(&self) -> &str {
        match self {
            DataType::Integer(_) => "i32",
            DataType::Boolean(_) => "i1",
            _ => panic!()
        }
    }
    pub fn as_int(&self) -> i32 {
        match self {
            DataType::Integer(i) => *i,
            _ => panic!()
        }
    }
    pub fn as_bool(&self) -> bool {
        match self {
            DataType::Boolean(b) => *b,
            _ => panic!()
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
            Operation::NotEqual => write!(f, "icmp ne"),
            Operation::GreaterEqual => write!(f, "icmp sge"),
            Operation::GreaterThan => write!(f, "icmp sgt"),
            Operation::LessEqual => write!(f, "icmp sle"),
            Operation::LessThan => write!(f, "icmp slt")
        }
    }
}


#[derive(Clone)]
pub struct Binary {
    left: Box<Expression>,
    right: Box<Expression>,
    pub operator: Operation,
    register: String,
    datatype: DataType

}
impl Binary {
    pub fn new(left: Expression, right: Expression, operator: Operation, register: &str, datatype: DataType) -> Binary {
        Binary {left: Box::new(left), right: Box::new(right), operator, register: register.to_string(), datatype}
    }
    pub fn print(&self) -> String {
        if is_boolean_op(self.operator.clone()) {
            format!("{}", llvm_call_print_local(self.register.clone().parse().unwrap(), "i1"))
        } else {
            format!("{}", llvm_call_print_local(self.register.clone().parse().unwrap(), "i32"))
        }
        
    }

    pub fn get_left(&self) -> Expression {
        *self.left.clone()
    }
    pub fn get_right(&self) -> Expression {
        *self.right.clone()
    }
    pub fn as_datatype(&self) -> &DataType{
        &self.datatype
    }
}
pub fn convert_bool(b: bool) -> u32 {
    if b {1} else {0}
}


impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} {} {}>", self.left, self.right, self.operator)
    }
}

// Expressions
// These are all the default actions I want to use with Expression Types
pub trait Visitor {
    fn visit_literal(&mut self, literal: &DataType) -> String;
    fn visit_binary(&mut self, binary: &Binary) -> String;
    fn visit_variable_expression(&mut self, variable_expression: &VariableExpression) -> String;

    // Statements
    fn visit_print(&mut self, print_statement: &PrintStatement) -> String;
    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) -> String;
}
pub trait Accept {
    fn accept<V: Visitor> (&self, visitor: &mut V) -> String;
}
pub trait Register {
    fn register(&self) -> String;
}
#[derive(Clone)]
pub struct VariableExpression {
    pub name: String,
    pub datatype: DataType,
    pub count: usize
}
impl From<VariableExpression> for Expression {
    fn from(value: VariableExpression) -> Self {
        Expression::Variable(value)
    }
}
#[derive(Clone)]
pub enum Expression {
    Literal(DataType),
    Variable(VariableExpression),
    Binary(Binary),
    Empty
}
impl Register for Expression {
    fn register(&self) -> String {
        //println!("; [Register Trait] Placing expression in register for declaration use.");
        print!("\t");
        match self {
            Expression::Binary(binary) => Expression::from(binary.clone()).resolve_binary(),
            Expression::Literal(literal) => Expression::from(literal.clone()).resolve_binary(),

            _ => panic!()

        }
    }
}
impl Accept for Expression {
    fn accept<V: Visitor> (&self, visitor: &mut V) -> String{
        match self {    
            Expression::Literal(literal) => visitor.visit_literal(literal),
            Expression::Binary(binary) => visitor.visit_binary(binary),
            Expression::Variable(variable) => visitor.visit_variable_expression(variable),
            _ => panic!()
        }
    }
}
impl From<Binary> for Expression {
    fn from(binary: Binary) -> Expression {
        Expression::Binary(binary)
    }
}
impl From<DataType> for Expression {
    fn from(value: DataType) -> Self {
        Expression::Literal(value)
    }
}

impl Expression {
    pub fn new_binary(left: Expression, right: Expression, operator: Operation, register: &str, datatype: DataType) -> Expression {
        Expression::Binary(Binary::new(left, right, operator, register, datatype))
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
                let tag = match b.datatype {
                    DataType::Boolean(_) => "i1",
                    DataType::Integer(_) => "i32",
                    DataType::String(_) => panic!()
                 };
                let mut codegen = format!("{} {tag} ", b.operator);
                codegen += &b.get_left().resolve_operand();
                codegen += &", ".to_string();
                codegen += &b.get_right().resolve_operand();
                format!("%{} = ", b.register) + &codegen
            },
            _ => panic!()
        }
        
    }
    pub fn resolve_operand(&self) -> String {
        match self {
            Expression::Binary(b) => format!("%{}", b.register),
            Expression::Literal(i) => {
                match i {
                    DataType::Integer(int) => int.to_string(),
                    DataType::Boolean(bool) => convert_bool(*bool).to_string(),
                    _ => "".to_string()
                }
            },
            Expression::Variable(variable) => {
                format!("%{}_{}", variable.name, variable.count)
            }
            _ => "".to_string()
        }
    }
    pub fn from_literal(literal: DataType) -> Expression{
        Expression::Literal(literal)
    }
    pub fn as_datatype(&self) -> DataType {
        match self {
            Expression::Literal(datatype) => datatype.clone(),
            Expression::Binary(binary) => binary.datatype.clone(),
            Expression::Variable(variable) => variable.datatype.clone(),
            _ => panic!()
        }
    }
    pub fn type_tag(&self) -> &str {
        match self.as_datatype() {
            DataType::Integer(int) => "i32",
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
            DataType::Integer(int) => write!(f, "{}", int),
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