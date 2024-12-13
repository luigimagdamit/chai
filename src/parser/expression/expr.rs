use std::fmt;
use crate::llvm::llvm_print::llvm_call_print_local;
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
    pub fn print(&self, expr_count: u32) -> String {
        match self {
            DataType::Integer(int) => {
                // place the value in a register to be used
                // ex. %0 = add i32 0;
                // call the print
                let mut cg = format!("\t%{expr_count} = add i32 {int}, 0\n");
                cg += &llvm_call_print_local(expr_count, "i32");
                println!("{cg}");
                cg
            },
            DataType::Boolean(bool) => {
                let bool_val = if *bool {1} else {0};
                let mut cg = format!("%{expr_count} = add i1 {bool_val}, 0");
                cg += &llvm_call_print_local(expr_count, "i1");
                println!("{cg}");
                cg
            }
            _ => "".to_string()
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
    operator: Operation,
    register: String,
    datatype: DataType
}
impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} {} {}>", self.left, self.right, self.operator)
    }
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



#[derive(Clone)]
pub enum Expression {
    Literal(DataType),
    Variable,
    Binary(Binary),
    Empty
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