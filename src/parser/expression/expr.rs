use std::fmt;
#[derive(Clone, PartialEq, Eq)]
#[allow(unused)]

// DataType is a literal
pub enum DataType {
    Integer(i32),
    String(String),
    Boolean(bool)
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
            Operation::Add => write!(f, "ADD"),
            Operation::Sub => write!(f, "SUB"),
            Operation::Mul => write!(f, "MUL"),
            Operation::Div => write!(f, "DIV"),
            Operation::Equal => write!(f, "EQ"),
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
    left: DataType,
    right: DataType,
    operator: Operation

}
impl Binary {
    pub fn new(left: DataType, right: DataType, operator: Operation) -> Binary {
        Binary {left, right, operator}
    }
}
impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "left: {}, right: {}, operation: {}", self.left, self.right, self.operator)
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
    pub fn new_binary(left: DataType, right: DataType, operator: Operation) -> Expression {
        Expression::Binary(Binary::new(left, right, operator))
    }
    pub fn new_literal(literal: DataType) -> Expression{
        Expression::Literal(literal)
    }
}
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Binary(b) => write!(f, "Binary AST: {b}"),
            Expression::Literal(l) => write!(f, "Literal AST: {l}"),
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